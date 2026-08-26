# F-57 Expansion and Highest-Security Release Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand the proven Windows CTC-01 slice into the complete four-platform, customizable, provider-enabled F-57 product and certify one immutable Windows/P340 release candidate against the highest-security evidence set.

**Architecture:** G5 expands independent client, provider, platform, model, package, portal, and business branches behind the same CapabilityGraph and authority spine, then joins them at `INTEGRATION_GREEN`. G6 builds and installs the final native Windows services and backup/recovery components, advances them through one final-installed `OBSERVED_COMMITTED` generation, freezes the signed client/server candidate, proves streaming backup and ransomware recovery plus exactly the current physical P340/UPS infrastructure-power pair, reruns L2 on those exact bytes, and only then aggregates L3 as `RELEASE_CERTIFIED`. Production remains quarantined until the separate two-person, live-readback-bound admission reaches `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED`. Future `IAAS_WINDOWS_SERVER_HDD_STRICT` is an independent graph-versioned seam, not a current G5/G6 recipe, schema, handler, carrier result, certificate input or activation terminal；当前 selector 必须在 STARTED 前返回 `PROFILE_NOT_IMPLEMENTED`，并把存储状态投影为 `STORAGE_MEDIA_UNVERIFIED`，但不得伪造已经存在的 IaaS evaluator 或 carrier result。

**Tech Stack:** Rust 2021/MSVC, PostgreSQL 16, Tauri 2 or the frozen Flutter + Rust fallback, React/TypeScript or Dart, Wasmtime, Windows Job Objects and conditional Hyper-V containers, MCP, Excel/CSV/document adapters, native Windows SCM/Service SID/named pipe/DACL/Job Object/CNG/BitLocker/W32Time, streaming encrypted backup, PowerShell evidence carriers.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`

**Status:** `READY_NOT_AUTHORIZED` / `BLOCKED_BY_DEV_SLICE`; planning is complete, but no task may start before separate development authorization and its prerequisite gate receipt.

## Global Constraints

- G5 requires a verified `DEV_SLICE_GREEN` receipt before its first change; G6 similarly requires `INTEGRATION_GREEN`. Those receipts authorize dependency entry only. After the tree changes, `gate g5` and `gate g6` obtain a fresh complete same-run result set by typed-joining L2/client/physical terminal records under the durable `(candidate_identity_sha256,gate_run_id,TestID)` journal key and starting only `ABSENT` predecessor conformance; an old receipt is never re-signed as current and a started TestID is never rerun.
- Every Fresh-PG rehearsal and gate first applies the unchanged digest-locked 69-file pre-F57 baseline, then the contiguous `CREATED` F57 suffix. G5 ends at 43 F57 files (`112` total); G6 ends at 47 F57 files (`116` total). None of the 310 legacy or nine absent baseline paths may appear or execute.
- All user-facing clients consume one generated Employee API and UI schema. No platform may fork business states, permissions, amounts, error codes, or close/reopen semantics.
- Exactly one Workbench technology branch is selected by `ClientStackDecisionV1`; a rejected stack cannot be shipped or silently mixed into the accepted branch.
- Local AI/model/OCR/RAG/KG implementation remains disabled. Only provider contracts, null implementation, containment, and negative product claims are release-due.
- Provider effective permission is the four-way intersection of package ceiling, provider manifest, deployment grant, and invocation grant. Default is empty.
- P340 container activation returns `HOST_CAPABILITY_UNAVAILABLE`; implementation of the conditional carrier ABI never implies this host is certified to run it.
- Customer relational customization accepts typed model declarations only; no SQL, trigger, function, physical name, or arbitrary expression enters the compiler.
- Module license, capability-package trust, and provider resource grant are separate decisions and separate machine records.
- `DROP_SHIP` and `STANDARD` use mutually exclusive primary fulfilment families and never cross-consume facts or objectives.
- All release-due business, client, provider, security, and deferred-boundary Requirement rows must pass; a disabled deferred feature passes only its seam and negative activation/claim tests.
- Final Windows/macOS/iOS/Android signed bytes are frozen before final L2. Any byte, graph, migration, installer, or manifest change invalidates L2/L3 and starts a new candidate.
- Every final-candidate, L2, L3, release-gate, migration, schema, workflow, installer, and evidence-carrier source is committed before final freeze. Tasks 10–13 may produce engineering rehearsal evidence only. After Task 14's tooling commit, Task 15 starts from clean `HEAD`, builds/fixes no source, freezes the final bytes, and runs every release-bearing physical measurement against that one digest.
- Production remains `SINGLE_DISK_DEGRADED_PRODUCTION`; certification never describes the single internal HDD as redundant, hot-swappable, or ransomware-safe by itself.
- Normative Files-list expansion: every task below that creates one or more `db/migrations/**` files also modifies `docs/f57-migration-reservations.v2.tsv`, changes exactly those rows to `CREATED`, and includes that registry in the same task-stage commit as the SQL. A pre-commit Fresh PG run is engineering rehearsal only; the clean-HEAD G5/G6 gate reruns the complete due prefix and issues candidate-bound evidence. No aggregate task may postpone row transitions.
- Every task runs in its own clean F-57 worktree, begins with `cargo xtask f57 task begin --task <exact-id>`, and commits only after `task stage` plus `task verify-staged`. Raw `git add`, directory/glob staging, pre-task dirty paths, and cached-set drift are forbidden. G5-02A/02B staging condition must exact-match the signed stack-decision receipt.
- G0 owns the 22 canonical Requirement facade files. Expansion tasks create/modify only concrete modules under `testkit/src/f57_cases/g5/` or `testkit/src/f57_cases/g6/`, register exact RequirementIDs in the CapabilityGraph, and regenerate `docs/generated/f57/requirement-test-facades.v1.json`; they never hand-edit a canonical `testkit/tests/f57_*.rs` facade. A mixed-profile target is invoked by exact due symbol, never target-wide with a future row treated as success.
- `scripts/windows/run-l2-candidate.ps1` accepts `-TargetGate` from the closed set `DEV_SLICE_GREEN|INTEGRATION_GREEN|RELEASE_CERTIFIED` and mandatory `-BundleRoot` plus `-RunJournal`; `scripts/windows/run-l3-release.ps1` likewise requires and forwards those same exact paths. `DeliveryProfileV1` remains the separate closed set `G0_BOOTSTRAP|G1_AUTHORITY_SPINE|G2_CTC_DATA|G3_CLIENT_SHELL|G4_CTC01|G5_INTEGRATION|G6_RELEASE`; neither enum is accepted where the other is required.

---

## 1. G5/G6 migration reservations

| Version | Exact path | Owner |
|---:|---|---|
| `20261025091700` | `db/migrations/platform_ops/V20261025091700__platform_ops_create_remote_support_sessions.sql` | remote-support security lifecycle |
| `20261025091710` | `db/migrations/platform_meta/V20261025091710__platform_meta_create_offline_intents.sql` | shared four-platform/offline contract |
| `20261025091800` | `db/migrations/platform_meta/V20261025091800__platform_meta_extend_provider_and_mcp_registry.sql` | provider/MCP containment |
| `20261025091900` | `db/migrations/platform_flow/V20261025091900__platform_flow_create_approval_cases.sql` | approvals |
| `20261025091910` | `db/migrations/platform_meta/V20261025091910__platform_meta_create_search_definitions.sql` | governed search |
| `20261025091920` | `db/migrations/platform_file/V20261025091920__platform_file_extend_governed_lifecycle.sql` | file lifecycle |
| `20261025091930` | `db/migrations/platform_core/V20261025091930__platform_core_create_external_identity_links.sql` | external identity |
| `20261025092000` | `db/migrations/platform_meta/V20261025092000__platform_meta_create_customer_model_specs.sql` | customer relational model compiler |
| `20261025092100` | `db/migrations/platform_meta/V20261025092100__platform_meta_create_capability_packages.sql` | package/license/hotplug |
| `20261025092200` | `db/migrations/portal/V20261025092200__portal_create_identity_and_customization.sql` | portal |
| `20261025092210` | `db/migrations/mdm/V20261025092210__mdm_extend_release_breadth.sql` | complete master-data breadth |
| `20261025092220` | `db/migrations/crm/V20261025092220__crm_create_release_breadth.sql` | CRM authority and projections |
| `20261025092230` | `db/migrations/cpq/V20261025092230__cpq_create_release_breadth.sql` | CPQ quote/version lifecycle |
| `20261025092240` | `db/migrations/clm/V20261025092240__clm_extend_release_breadth.sql` | complete contract lifecycle |
| `20261025092300` | `db/migrations/sales/V20261025092300__sales_extend_release_breadth.sql` | sales breadth |
| `20261025092310` | `db/migrations/procure/V20261025092310__procure_extend_release_breadth.sql` | procurement breadth |
| `20261025092320` | `db/migrations/inventory/V20261025092320__inventory_extend_release_breadth.sql` | inventory breadth |
| `20261025092330` | `db/migrations/invoice/V20261025092330__invoice_extend_release_breadth.sql` | invoice breadth |
| `20261025092340` | `db/migrations/finance/V20261025092340__finance_extend_release_breadth.sql` | finance breadth |
| `20261025092350` | `db/migrations/ledger/V20261025092350__ledger_create_operating_ledger_release_breadth.sql` | independent operating-ledger owner |
| `20261025092400` | `db/migrations/service/V20261025092400__service_create_release_breadth.sql` | service breadth |
| `20261025092410` | `db/migrations/project/V20261025092410__project_create_release_breadth.sql` | project breadth |
| `20261025092420` | `db/migrations/reporting/V20261025092420__reporting_create_release_breadth.sql` | reporting breadth |
| `20261025092500` | `db/migrations/platform_ops/V20261025092500__platform_ops_create_authority_service_fences.sql` | native Windows service lease/fencing readback; references canonical authz epoch |
| `20261025092510` | `db/migrations/platform_ops/V20261025092510__platform_ops_create_backup_sets_media_and_certification.sql` | backup/offline media |
| `20261025092520` | `db/migrations/platform_ops/V20261025092520__platform_ops_create_security_incidents_and_recovery_cuts.sql` | ransomware/clean restore |
| `20261025092530` | `db/migrations/platform_core/V20261025092530__platform_core_create_production_activation_and_admission.sql` | production activation, generation admission/hold and final RLS/unpoliced closure |

No other G5/G6 migration may occupy this block. Each task proves its exact prefix on Fresh PostgreSQL 16 before its gate.

### Task 1: Run the four-platform Tauri 2 technology gate

**Files:**
- Create: `crates/platform/client-common/Cargo.toml`
- Create: `crates/platform/client-common/src/lib.rs`
- Create: `crates/platform/client-common/src/model.rs`
- Create: `crates/platform/client-common/tests/wire.rs`
- Create: `crates/platform/client-common/tests/fixtures/f57-client-common-v1-golden.json`
- Create: `docs/evidence/f57-client-common.v1.schema.json`
- Create: `clients/technology-gate/tauri2/README.md`
- Create: `clients/technology-gate/tauri2/package.json`
- Create: `clients/technology-gate/tauri2/package-lock.json`
- Create: `clients/technology-gate/tauri2/tsconfig.json`
- Create: `clients/technology-gate/tauri2/vitest.config.ts`
- Create: `clients/technology-gate/tauri2/probe-manifest.v1.json`
- Create: `xtask/src/f57/client_gate.rs`
- Create: `xtask/src/f57/client_build.rs`
- Modify: `xtask/src/f57/client_conformance.rs`
- Create: `xtask/tests/f57_client_gate.rs`
- Create: `xtask/tests/f57_client_build.rs`
- Create: `xtask/tests/f57_client_conformance.rs`
- Create: `xtask/tests/fixtures/f57-client-architecture-decision-slot-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-stack-decision-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-stack-decision-trust-closure-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-stack-decision-archive-manifest-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-stack-decision-attempt-v1-golden.jcs.jsonl`
- Create: `xtask/tests/fixtures/f57-client-stack-decision-attempt-checkpoint-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-stack-validation-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-artifact-set-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-client-platform-evidence-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-client-platform-lifecycle-evidence-v1-goldens.json`
- Read: `docs/schemas/f57-client-lifecycle-fixture-corpus.v1.schema.json`
- Read: `testkit/fixtures/client-lifecycle/fixture-corpus.v1.json`
- Read: `testkit/fixtures/client-lifecycle/trust/android-fixture-root.der`
- Read: `testkit/fixtures/client-lifecycle/trust/ios-fixture-root.der`
- Read: `testkit/fixtures/client-lifecycle/trust/macos-fixture-root.der`
- Read: `testkit/fixtures/client-lifecycle/trust/windows-fixture-root.der`
- Read: `testkit/fixtures/client-lifecycle/android/{baseline,revoked,downgrade,failed-update}.apk`
- Read: `testkit/fixtures/client-lifecycle/ios/{baseline,revoked,downgrade,failed-update}.ipa`
- Read: `testkit/fixtures/client-lifecycle/macos/{baseline,revoked,downgrade,failed-update}.pkg`
- Read: `testkit/fixtures/client-lifecycle/windows/{baseline,revoked,downgrade,failed-update}.msi`
- Regenerate: `docs/generated/f57/client-platform-lifecycle-policy.v1.json`
- Read: `xtask/tests/fixtures/f57-client-conformance-result-v1-golden.json`
- Create: `clients/technology-gate/tauri2/tests/handlers/cli_007.ts`
- Create: `docs/evidence/f57-client-stack-decision.v1.schema.json`
- Create: `docs/evidence/f57-client-stack-decision-archive.v1.schema.json`
- Create: `docs/evidence/f57-client-stack-decision-attempt.v1.schema.json`
- Create: `docs/evidence/f57-client-stack-validation.v1.schema.json`
- Create: `docs/evidence/f57-client-artifact-set.v1.schema.json`
- Create: `docs/evidence/f57-client-platform-evidence.v1.schema.json`
- Read: `docs/evidence/f57-client-conformance-result.v1.schema.json`
- Modify: `xtask/src/f57/cli.rs`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `xtask/src/f57/g3.rs`
- Modify: `xtask/src/f57/l2.rs`
- Modify: `scripts/windows/run-l2-candidate.ps1`
- Modify/regenerate after final edit: `scripts/windows/trust/F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after descriptor verification: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Modify: `clients/workbench/src-tauri/tauri.conf.json`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/client-conformance-manifest.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: the G3 Windows client, generated Employee protocol vectors, controlled macOS/Xcode, iOS, Android SDK/NDK, Windows MSVC runners, the CapabilityGraph's exact four typed lifecycle-fixture trust-root rows and exact 16 typed `CLIENT_LIFECYCLE_FIXTURE` source rows, and the reviewed 20-file fixture corpus listed above.
- Shared owner: `ep-platform-client-common` and `docs/evidence/f57-client-common.v1.schema.json` are the sole production Rust/schema owners of `ClientPackageArtifactIdV1` and strict shared client artifact-reference shapes. The closed ID wires are exactly `ANDROID_CLIENT|IOS_CLIENT|MACOS_CLIENT|WINDOWS_CLIENT`; checked constructors bind one platform, package media and artifact ref, and no architecture branch or release crate redeclares them. The schema imports foundation exactly once. Task 14 adds a direct `ep-platform-release -> ep-platform-client-common` Cargo edge and only imports these types; workspace metadata, all-target compilation and schema-DAG goldens reject a production-to-xtask edge, copied enum, fifth lane or reverse client-common-to-release edge.
- Produces: exactly one global `ArchitectureDecisionSlotV1`, one durable `ArchitectureDecisionAttemptV1` header/record/checkpoint stream, one `ClientStackDecisionV1 = SignedBusinessArtifactV1<ClientStackDecisionPayloadV1>`, one immutable `ClientStackDecisionTrustClosureV1`, and one deterministic `ClientStackDecisionArchiveManifestV1`, plus the single `client-gate` and `client-build` dispatch implementations used by both branch tasks. `ClientStackDecisionPayloadV1` has exactly `schema_version=1`, typed `purpose`, `finalization_attempt_id`, `decision: TAURI2_CERTIFIED|FLUTTER_RUST_REQUIRED`, `candidate_identity`, `architecture_decision_slot_ref`, `architecture_attempt_header_ref`, `architecture_attempt_checkpoint_ref`, the exact four `platform_results`, and `decided_at_unix_ms`. `ArchitectureDecisionAttemptHeaderRefV1` is `{attempt_id,journal_prefix: ArchitectureDecisionAttemptPrefixRefV1}` and authenticates the complete signed header frame, not the growing journal as a whole-file artifact. The trust closure uses purpose `EP-F57-CLIENT-STACK-DECISION-TRUST-CLOSURE-V1` and media `application/vnd.ep.f57-client-stack-decision-trust-closure-v1+json`; the archive manifest uses purpose `EP-F57-CLIENT-STACK-DECISION-ARCHIVE-MANIFEST-V1` and media `application/vnd.ep.f57-client-stack-decision-archive-manifest-v1+json`. Both are unsigned deterministic exact-closure locators and gain no signer-registry row; the former is frozen and wholly offline-verified before `DECISION_BOUND`, while the latter is derived only after BOUND from its contained signed journal/evidence plus the already durable trust closure and later committed graph digest. The fixed global slot, authoritative archive root, initial materialized archive, attempt journal, platform, checkpoint, object, trust, decision, trust-closure and archive-manifest paths, exact-once finalization/recovery and purpose/media wires are the master contract; this task cannot introduce another attempt index.

Before any bundle output, the Windows evidence authority calls only the G1-01-owned `verify_authority_data_root_v1(storage_manifest_path,trust_input_paths)` constructor. It typed-verifies the signed authority-storage manifest through the complete deployment bootstrap, requires `DATA_HDD`, performs the final-handle non-reparse/volume/anti-rollback checks, and returns private `ValidatedDataRootV1`; no raw-path overload or caller-built value exists. The authority then create-new creates or adopts exactly `<validated-data-root>\evidence-authority\architecture-decision-slots\<candidate_identity_sha256>\slot.v1.json`; a drive letter and `C:\ProgramData` are never part of the wire. Its `slot_id_sha256` is exactly `sha256(JCS({"contract":"EP-F57-CLIENT-ARCHITECTURE-DECISION-V1","candidate_identity_sha256":<64-lowerhex>,"stack_under_test":"TAURI2"}))`, its URI is `evidence-authority://architecture-decision-slots/<candidate_identity_sha256>/slot.v1.json`, its aggregator is `f57-architecture-decision-authority`, and its signer subject is the registry SPKI token subject under the additional leaf-DN constraint `CN=EP F57 Architecture Decision Authority,O=Enterprise Platform`. The slot fixes one unpredictable `attempt_id`; another authority root, slot, or physical attempt for the same committed candidate fails closed. `ArchitectureDecisionStoreV1` uses that global slot's create-new `archive-root` as the authoritative persistence root and the explicit initial bundle root only as an exact materialized view. Before its attempt header it create-new copies/fsyncs/reloads exactly eight immutable inputs—the signed 89-row signer registry, signed storage manifest, deployment manifest/signature/trust bundle, storage trust root/revocation/checkpoint—and binds their exact refs, DATA_HDD volume identity and canonical root digest. Canonical bytes are durably stored below the global archive first, then create-new byte-copied without resigning to the same relative path below the initial materialization; any conflict fails. It exclusively owns `g5/client-stack-decision-slot.v1.json`, `g5/client-stack-decision-attempt.jcs.jsonl`, `g5/client-stack-decision-platform-results/<windows|macos|ios|android>.v1.json`, `g5/client-stack-decision-attempt-checkpoints/<20-digit-sequence>.v1.json`, `g5/client-stack-decision.v1.json`, the eight fixed archived inputs, content-addressed subordinate objects, `trust/timestamps/*`, `trust/revocation/*`, exact `trust/trust-closure.v1.json`, and root `archive-manifest.v1.json` in both roots.

G5-01 exclusively owns the trust-closure and archive-manifest Rust types, both canonical schemas and byte-goldens, global authority archive and initial materialization protocol. Exactly one selected `G5-02A|G5-02B` task owns the immutable committed `docs/decisions/f57-client-stack-decision-archive/` tree, its convenience decision copy and the graph `architecture_inputs` row; the unselected task performs no write, and no later task may reconstruct, resign or mutate that historical archive.

`ClientStackDecisionTrustClosureV1` has exactly `schema_version`, typed `purpose`, `finalization_attempt_id`, `timestamp_request_nonce_lowerhex`, `decision_ref`, `archive_trust_anchor_policy_sha256`, and canonical tagged `trust_proofs`. `ClientStackDecisionArchiveManifestV1` has exactly `schema_version`, typed `purpose`, `decision_ref`, `trust_closure_ref`, `decision_bound_checkpoint_ref`, `decision_bound_attempt_prefix`, the same `archive_trust_anchor_policy_sha256`, the same canonical tagged `trust_proofs`, and canonical `entries`. Each `ClientStackDecisionArchiveEntryV1` has exactly `relative_path,media_type,size_bytes,sha256`. `ClientStackDecisionArchiveTrustProofV1` is the closed `proof_kind`-tagged set `RFC3161_TIMESTAMP_TOKEN|CERTIFICATE_CHAIN|X509_CRL|OCSP_RESPONSE`; the last three carry the closed `chain_role=DECISION_SIGNER|TSA` where applicable, and `CERTIFICATE_CHAIN` carries the authenticated source-order `certificate_sha256s_leaf_to_root`. The manifest's entry vector is the complete transitive historical closure of the slot, authenticated attempt prefix through bound/reconciled, every checkpoint, four platform aggregates with all embedded lifecycle evidence, every subordinate object/package readback, the decision envelope, the exact trust closure, trusted CMS/RFC3161 proof, and archived signed CRL/OCSP proof. The manifest excludes itself; entries sort by `relative_path`, both proof vectors and policy digests are byte-identical and use the master canonical tagged ordering, every referenced artifact is an exact entry, all sets are unique, and missing, extra, path-escaping, digest/media/size-drifting or ambient/network trust material fails closed. The decision is validated at finalization/signing against then-current input expiry and archived revocation state; later historical verification uses the trusted signing time, immutable trust closure and archived chains/CRL/OCSP rather than incorrectly requiring the original 90-day platform evidence to still be current.

`trust_proofs` contains exactly one `RFC3161_TIMESTAMP_TOKEN`, exactly two `CERTIFICATE_CHAIN` rows—one `DECISION_SIGNER` and one `TSA`—and, for every non-root certificate at zero-based `covered_chain_index` in each exact role-specific chain, exactly one same-role `X509_CRL` plus one same-role `OCSP_RESPONSE`. Rows sort by `(proof_kind ordinal,chain_role null-first,covered_chain_index null-first,artifact_ref.uri,artifact_ref.sha256)`; the timestamp has both nullable keys absent, and duplicates, a role/index collision or an uncovered non-root certificate fail. Exact media are respectively `application/timestamp-reply`, `application/pkcs7-mime`, `application/pkix-crl`, and `application/ocsp-response`. The RFC-3161 token has a valid TSA signature and request nonce, and its `message_imprint_sha256` hashes the actual decision CMS `SignerInfo.signature` OCTET STRING bytes, never the payload or envelope digest; `gen_time_unix_ms` is the actual trusted signing time inside the frozen finalization window. The `DECISION_SIGNER` chain is exactly the decision CMS signer's ordered leaf-through-root chain; the `TSA` chain is exactly the timestamp token signer's ordered leaf-through-root chain. Each role's leaf identity, serial and count exact-match its tagged scalars, every certificate is valid at the trusted time, and neither chain may be substituted or merged with the other. Each CRL is signature-valid for its role/indexed certificate's issuer, exact-matches issuer and serial, satisfies `this_update_unix_ms <= signing_time < next_update_unix_ms`, and reports `GOOD`. Each OCSP response is signature-valid from the issuer or an authorized responder, exact-matches role, issuer name/key hashes and serial, satisfies `produced_at_unix_ms <= signing_time` and `this_update_unix_ms <= signing_time < next_update_unix_ms`, and reports `GOOD`. Verification is wholly offline and never substitutes a currently fetched response. Wrong imprint/nonce/TSA, incomplete or cross-signed alternate chain, role/index drift, future/stale/expired proof, unauthorized responder, missing indexed pair, `UNKNOWN|REVOKED`, media alias or bytes disagreeing with tagged fields fails closed. The archive and trust-closure schemas own this one enum and all path/cardinality goldens; no second trust-proof shape is legal.

```rust
#[serde(deny_unknown_fields)]
pub struct ArchitectureInputBindingV1 {
    pub input_id: ArchitectureInputIdV1,
    pub artifact_sha256: Sha256Digest,
    pub archive_manifest_sha256: Sha256Digest,
    pub media_type: String,
    pub selected_stack: ClientStackKindV1,
}

#[serde(tag = "event_kind", content = "event", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArchitectureDecisionAttemptEventV1 {
    PlatformStarted { platform: ClientPlatformV1, execution_attempt_id: UuidV1 },
    PlatformCompleted { platform: ClientPlatformV1, execution_attempt_id: UuidV1, result_ref: ArtifactRefV1 },
    PlatformUnknown { platform: ClientPlatformV1, execution_attempt_id: UuidV1, error_code: ErrorCodeV1 },
    PlatformReconciled { platform: ClientPlatformV1, execution_attempt_id: UuidV1, result_ref: ArtifactRefV1 },
    DecisionFinalizationStarted {
        finalization_attempt_id: UuidV1,
        frozen_input_checkpoint_ref: ArtifactRefV1,
        decided_at_unix_ms: i64,
        signing_not_after_unix_ms: i64,
        timestamp_request_nonce_lowerhex: String,
    },
    DecisionBound {
        finalization_attempt_id: UuidV1,
        decision_ref: ArtifactRefV1,
        trust_closure_ref: ArtifactRefV1,
    },
    DecisionReconciled {
        finalization_attempt_id: UuidV1,
        decision_ref: ArtifactRefV1,
        trust_closure_ref: ArtifactRefV1,
    },
}

pub enum ClientStackDecisionArchiveManifestPurposeV1 {
    ClientStackDecisionArchiveManifest,
}

pub enum ClientStackDecisionTrustClosurePurposeV1 {
    ClientStackDecisionTrustClosure,
}

pub struct ClientStackDecisionArchiveEntryV1 {
    pub relative_path: String,
    pub media_type: String,
    pub size_bytes: u64,
    pub sha256: Sha256Digest,
}

pub enum ClientStackDecisionArchivedCertificateStatusV1 { Good, Revoked, Unknown }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientStackDecisionArchiveCertificateChainRoleV1 { DecisionSigner, Tsa }

#[serde(tag = "proof_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientStackDecisionArchiveTrustProofV1 {
    Rfc3161TimestampToken {
        artifact_ref: ArtifactRefV1,
        tsa_subject_sha256: Sha256Digest,
        request_nonce_sha256: Sha256Digest,
        message_imprint_sha256: Sha256Digest,
        gen_time_unix_ms: i64,
    },
    CertificateChain {
        chain_role: ClientStackDecisionArchiveCertificateChainRoleV1,
        artifact_ref: ArtifactRefV1,
        certificate_sha256s_leaf_to_root: Vec<Sha256Digest>,
        leaf_subject_sha256: Sha256Digest,
        leaf_serial_lowerhex: String,
        certificate_count: u16,
    },
    X509Crl {
        chain_role: ClientStackDecisionArchiveCertificateChainRoleV1,
        artifact_ref: ArtifactRefV1,
        covered_chain_index: u16,
        issuer_name_sha256: Sha256Digest,
        covered_serial_lowerhex: String,
        this_update_unix_ms: i64,
        next_update_unix_ms: i64,
        status: ClientStackDecisionArchivedCertificateStatusV1,
    },
    OcspResponse {
        chain_role: ClientStackDecisionArchiveCertificateChainRoleV1,
        artifact_ref: ArtifactRefV1,
        covered_chain_index: u16,
        responder_subject_sha256: Sha256Digest,
        issuer_name_hash_sha256: Sha256Digest,
        issuer_key_hash_sha256: Sha256Digest,
        covered_serial_lowerhex: String,
        produced_at_unix_ms: i64,
        this_update_unix_ms: i64,
        next_update_unix_ms: i64,
        status: ClientStackDecisionArchivedCertificateStatusV1,
    },
}

pub struct ClientStackDecisionTrustClosureV1 {
    pub schema_version: u32,
    pub purpose: ClientStackDecisionTrustClosurePurposeV1,
    pub finalization_attempt_id: UuidV1,
    pub timestamp_request_nonce_lowerhex: String,
    pub decision_ref: ArtifactRefV1,
    pub archive_trust_anchor_policy_sha256: Sha256Digest,
    pub trust_proofs: Vec<ClientStackDecisionArchiveTrustProofV1>,
}

pub struct ClientStackDecisionArchiveManifestV1 {
    pub schema_version: u32,
    pub purpose: ClientStackDecisionArchiveManifestPurposeV1,
    pub decision_ref: ArtifactRefV1,
    pub trust_closure_ref: ArtifactRefV1,
    pub decision_bound_checkpoint_ref: ArtifactRefV1,
    pub decision_bound_attempt_prefix: ArchitectureDecisionAttemptPrefixRefV1,
    pub archive_trust_anchor_policy_sha256: Sha256Digest,
    pub trust_proofs: Vec<ClientStackDecisionArchiveTrustProofV1>,
    pub entries: Vec<ClientStackDecisionArchiveEntryV1>,
}
```

`ClientStackValidationV1` is exactly the master `SignedBusinessArtifactV1<ClientStackValidationPayloadV1>`: `selection_receipt_ref` must originate from the exact committed `docs/decisions/f57-client-stack-decision.v1.json`, whose bytes equal the manifest-listed decision under fixed `docs/decisions/f57-client-stack-decision-archive/`. `validate-selected` first replays the complete archive offline, typed-verifies the decision and materializes its required proof closure in-bundle; it never consults the deleted initial target, global authority path, network or ambient trust cache. `candidate_run` recomputes from `current_candidate_identity` plus the newly generated unpredictable journal `gate_run_id`, `mode` is selected once by `--integration|--release`, and the exact four `platform_results` are the final package bytes for that mode. Every platform aggregate contains the exact eight typed lifecycle categories, including revocation, DLP and accessibility, over those same bytes; only all-PASS permits validation PASS. The payload binds its precursor journal checkpoint and trusted issue/expiry times. Digest-only, archive-less, incomplete or uncommitted selection is forbidden.

`ClientArtifactSetV1` is exactly the master `SignedBusinessArtifactV1<ClientArtifactSetPayloadV1>` and the sole `client-build` output: `schema_version=1`, typed purpose, the validation's exact `mode: INTEGRATION|RELEASE`, exact signed `validation_ref`, matching `candidate_run` and `selected_stack`, the exact four signed client lanes whose inner package refs byte-match the four validation refs, one canonical same-run `g5_four_platform_conformance_ref`, a strictly extending checkpoint, and trusted issue/expiry times. `client-build` only aggregates the already validated package bytes and dispatches conformance; it cannot rebuild, repackage, resign or replace them. Candidate construction typed-consumes that one envelope, adopts its run ID and never accepts an untyped list, digest-only validation, or replacement value. G5-01 owns the attempt, decision, trust-closure, archive-manifest, validation, artifact-set, platform aggregate and lifecycle schema/goldens; the trust/archive schemas are the sole canonical owners for their purpose/proof/manifest/entry families, and all closed schemas reject free-form purpose, unknown fields and the typed expectation for another artifact. The shared conformance type/schema remains G0-owned.

The lifecycle policy is the sole plain JCS projection `docs/generated/f57/client-platform-lifecycle-policy.v1.json`, purpose/media `EP-F57-CLIENT-PLATFORM-LIFECYCLE-POLICY-V1` / `application/vnd.ep.f57-client-platform-lifecycle-policy-v1+json`, and the sole member of the existing `client-conformance-manifest.v1.json` projection family. It has exactly four rows in UTF-8 order `android,ios,macos,windows`; each row carries one platform trust root and exactly four fixtures in role order `UPGRADE_BASELINE|REVOKED_PACKAGE|DOWNGRADE_PACKAGE|FAILED_UPDATE_PACKAGE`, for exactly four roots and 16 fixtures. Those 20 descriptors originate only from the CapabilityGraph root's typed vectors and survive byte-for-byte in `CompiledCapabilityGraphV1`. `project_all(&CompiledCapabilityGraphV1)` performs a pure total transformation: it never reads a package, filesystem, network, trust store or environment. A distinct source-tree verifier fixed-loads the reviewed corpus manifest plus all four DER roots and 16 native packages, rejects symlink/reparse or path drift, constructs an isolated trust store from the one platform root, and proves actual bytes, native package ID/version, leaf SPKI, signature chain, role, expected outcome and all graph digests before the projection manifest is accepted. No directory scan, caller-selected fixture or ambient root is legal.

`docs/evidence/f57-client-platform-evidence.v1.schema.json` is the canonical owner for both signed platform/lifecycle envelopes, the four-row policy wire, all eight strict plain readback families and exactly 32 descriptor parsers in canonical `(platform UTF-8,evidence-kind ordinal)` order: the eight kinds `PACKAGE_SIGNATURE,INSTALL_START,UPGRADE,REVOCATION,CAPABILITY,RESOURCE,DLP,ACCESSIBILITY` for `android`, then the same eight for `ios`, `macos`, and `windows`. Each descriptor selects one exact purpose/media/`$defs` parser from outer platform plus tagged evidence kind; caller context and generic JSON/octet-stream never select a parser. The aggregate embeds the eight independently signed lifecycle envelopes directly, not refs to guessed lifecycle result filenames. `docs/evidence/f57-client-stack-decision-archive.v1.schema.json` is the sole combined owner for trust-closure, archive-manifest, entry and archive-proof wires; the former split trust-closure/archive-manifest schema paths are forbidden aliases. Foundation is sole owner of `UuidV1`, digests, repository paths, artifact/test/run IDs, `ClientPlatformV1`, `ClientStackKindV1`, `ClientPackageIdV1`, fixture-role/outcome enums, principals, `RunnerIdV1`, `CandidateRunIdentityV1`, `CandidateIdentityV1` and the signed-envelope field set. Every signed client root imports foundation directly and composes that envelope once; client-common owns only client-local package/artifact wrappers, and conformance imports foundation plus requirement-evidence binding, never client-common.

`ClientInstallStartReadbackV1` is one before/after collector and contains the non-optional fields `pre_install_observed_at_unix_ms`, `pre_install_registered_package_count`, `pre_install_matching_application_count`, `pre_install_matching_process_count`, `pre_install_business_cache_entry_count`, and `pre_install_authoritative_business_database_count`. The pre-install time is after durable STARTED and strictly before the common observed time, every pre-install count is exactly zero, and the post-start evidence proves exactly one first launch, authenticated authority readiness and zero local authoritative business databases. `ClientAccessibilityReadbackV1` has its single-value `ClientAccessibilityReadbackPurposeV1`, exact standard wire `WCAG_2_2_AA_PLUS_NATIVE`, the exact eight policy cases, per-case outcomes, and explicit `screen_reader_passed`, `keyboard_or_switch_passed`, `text_scaling_200_percent_passed`, `contrast_passed`, and `failed_case_count`; PASS requires all four booleans true, the exact case set and zero failures. Every one-field mutation of the six pre-install fields, accessibility purpose/standard/cases/booleans/count, fixture source, or any of the exact `4/16/32` cardinalities has a failing golden.

This task changes the inherited `run-l2-candidate.ps1`, so after its last edit it must re-sign/timestamp the final bytes and regenerate `F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json` before any platform-gate use. A stale G4 descriptor cannot authorize the changed file.

- [ ] **Step 1: Write the failing exact-platform decision tests.**

```rust
#[test]
fn decision_requires_four_distinct_real_platform_results() {
    let err = decide(vec![windows_pass(), macos_pass(), ios_pass()]).unwrap_err();
    assert_eq!(err.code(), "CLIENT_GATE_PLATFORM_SET_INCOMPLETE");
}

#[test]
fn one_failed_hard_requirement_selects_fallback() {
    let decision = decide(vec![windows_pass(), macos_pass(), ios_camera_fail(), android_pass()]).unwrap();
    assert_eq!(decision.decision, ClientStackDecisionKindV1::FlutterRustRequired);
}

#[test]
fn architecture_attempt_is_durable_exact_once_and_not_best_of_many() {
    assert_global_slot_formula_and_authority_path_exact();
    assert_header_ref_hashes_only_the_complete_authenticated_header_prefix();
    assert!(platform_output_before_terminal_is_adopted_without_rerun());
    assert_code(second_platform_start_same_attempt(), "CLIENT_DECISION_PLATFORM_ALREADY_STARTED");
    assert_code(decision_with_platform_unknown(), "CLIENT_DECISION_ATTEMPT_INCONCLUSIVE");
    assert_code(second_attempt_without_signed_supersession(), "CLIENT_DECISION_ATTEMPT_ALREADY_EXISTS");
    assert_code(second_authority_root_for_same_candidate(), "CLIENT_DECISION_AUTHORITY_SLOT_CONFLICT");
    assert_code(decision_at_noncanonical_output_path(), "CLIENT_DECISION_OUTPUT_NONCANONICAL");
    assert_slot_attempt_checkpoint_and_decision_goldens_exact();
}

#[test]
fn decision_binds_execution_ids_checkpoint_and_frozen_finalization_window() {
    assert_code(platform_result_with_changed_execution_attempt_id(), "CLIENT_DECISION_EXECUTION_ATTEMPT_MISMATCH");
    assert_code(decision_with_nonextending_attempt_checkpoint(), "CLIENT_DECISION_CHECKPOINT_NOT_EXTENSION");
    assert_code(decision_time_outside_evidence_intersection(), "CLIENT_DECISION_SIGNING_TIME_INVALID");
    assert_decision_finalization_started_has_exact_five_fields_unpredictable_id_and_32_byte_nonce();
    assert_payload_copies_id_checkpoint_and_decided_at_only_from_frozen_record();
    assert_actual_cms_tsa_time_is_inside_inclusive_frozen_window_and_all_input_expiries();
    assert!(decision_requiring_payload_time_equal_later_tsa_time().is_err());
    assert_decision_output_and_trust_closure_are_create_new_fsynced_before_bound();
    assert_bound_and_reconciled_repeat_same_finalization_decision_and_trust_closure_refs();
    assert_decision_output_crash_is_signed_once_or_adopted_with_same_finalization_id_and_nonce();
    assert_crash_resends_only_byte_identical_timestamp_request_before_first_durable_valid_token();
    assert_durable_token_or_proof_is_never_replaced_or_best_of_many_selected();
}

#[test]
fn decision_archive_is_the_exact_permanent_offline_historical_closure() {
    assert_client_stack_decision_trust_closure_schema_and_golden_exact();
    assert_client_stack_decision_archive_manifest_schema_and_golden_exact();
    assert_archive_entries_cover_slot_bound_prefix_all_checkpoints_platform_lifecycle_and_objects();
    assert_archive_manifest_exact_matches_durable_trust_closure_ref_and_proof_vector();
    assert_exactly_one_timestamp_two_role_chains_and_one_crl_ocsp_pair_per_role_nonroot_certificate();
    assert_trust_proofs_sort_by_kind_role_index_uri_and_sha256_with_nulls_first();
    assert_decision_signer_and_tsa_chains_cannot_merge_or_substitute();
    assert_global_and_initial_archive_materializations_are_byte_identical();
    assert_code(archive_with_missing_or_extra_entry(), "CLIENT_DECISION_ARCHIVE_SET_MISMATCH");
    assert_code(archive_with_path_digest_media_or_size_drift(), "CLIENT_DECISION_ARCHIVE_ENTRY_MISMATCH");
    assert_code(archive_that_treats_unsigned_manifest_as_authority(), "CLIENT_DECISION_ARCHIVE_AUTHORITY_INVALID");
    assert_code(archive_requiring_network_or_ambient_trust_cache(), "CLIENT_DECISION_ARCHIVE_NOT_OFFLINE");
    assert!(archive_with_role_index_collision_or_missing_role_pair().is_err());
    assert!(bound_record_that_names_non_durable_trust_closure().is_err());
}

#[test]
fn each_platform_requires_all_eight_lifecycle_categories() {
    assert_eq!(client_platform_evidence_kinds(), [
        "ACCESSIBILITY", "CAPABILITY", "DLP", "INSTALL_START",
        "PACKAGE_SIGNATURE", "RESOURCE", "REVOCATION", "UPGRADE",
    ]);
    assert_code(platform_without_revocation(), "CLIENT_LIFECYCLE_SET_INCOMPLETE");
    assert_code(platform_with_category_details_mismatch(), "CLIENT_LIFECYCLE_KIND_MISMATCH");
    assert_code(platform_with_cross_package_lifecycle_evidence(), "CLIENT_LIFECYCLE_PACKAGE_MISMATCH");
    assert_platform_aggregate_and_lifecycle_goldens_exact();
}

#[test]
fn lifecycle_policy_is_pure_and_closes_four_roots_sixteen_fixtures_and_thirty_two_readbacks() {
    assert_eq!(compiled_graph().client_lifecycle_fixture_trust_roots.len(), 4);
    assert_eq!(compiled_graph().client_lifecycle_fixture_sources.len(), 16);
    assert_eq!(offline_client_lifecycle_readback_bindings().len(), 32);
    assert_project_all_does_not_open_fixture_paths_or_read_ambient_state();
    assert_source_tree_verifier_loads_exact_manifest_four_roots_and_sixteen_native_packages();
    assert_code(policy_with_ambient_or_alternate_fixture_root(), "CLIENT_FIXTURE_TRUST_ROOT_MISMATCH");
    assert_code(policy_with_fixture_role_or_digest_drift(), "CLIENT_FIXTURE_DESCRIPTOR_MISMATCH");
    assert_code(offline_manifest_with_31_or_33_readback_bindings(), "OFFLINE_DESCRIPTOR_BINDING_SET_MISMATCH");
}

#[test]
fn install_start_and_accessibility_wires_are_total() {
    assert_install_start_has_exact_six_preinstall_fields_and_all_counts_zero();
    assert_pre_install_observed_after_started_and_before_common_observed_time();
    assert_accessibility_purpose_standard_eight_cases_and_four_explicit_booleans_exact();
    assert_eq!(accessibility_standard_wire(), "WCAG_2_2_AA_PLUS_NATIVE");
    assert_code(accessibility_with_alias_standard_or_failed_case(), "CLIENT_ACCESSIBILITY_READBACK_INVALID");
    assert_all_preinstall_and_accessibility_one_field_mutation_goldens_fail();
}

#[test]
fn client_build_requires_selected_stack_and_four_runner_manifests() {
    assert_code(build_with_unselected_stack(), "CLIENT_BUILD_STACK_NOT_SELECTED");
    assert_code(build_with_three_runner_results(), "CLIENT_BUILD_PLATFORM_SET_INCOMPLETE");
}

#[test]
fn validation_and_artifact_set_are_closed_signed_chains() {
    assert_code(validation_with_digest_only_selection(), "CLIENT_VALIDATION_SELECTION_REF_REQUIRED");
    assert_code(validation_with_cross_run_checkpoint(), "CLIENT_VALIDATION_RUN_MISMATCH");
    assert_code(artifact_set_with_authority_lane(), "CLIENT_ARTIFACT_ROLE_UNKNOWN");
    assert_code(artifact_set_with_mixed_signature_classes(), "CLIENT_ARTIFACT_MODE_SIGNATURE_MISMATCH");
    assert_code(artifact_set_with_rebuilt_package(), "CLIENT_ARTIFACT_VALIDATED_BYTES_MISMATCH");
    assert_code(artifact_set_with_pre_candidate_cli_007(), "CLIENT_ARTIFACT_REQUIREMENT_RESULT_FORBIDDEN");
    assert_code(artifact_set_with_three_platform_conformance_refs(), "CLIENT_CONFORMANCE_EVIDENCE_CARDINALITY");
    assert_all_g5_client_schema_and_golden_pairs_exact();
}

#[test]
fn current_candidate_conformance_uses_only_the_selected_stack() {
    assert_eq!(
        selected_recipe_ids(flutter_selection()),
        ["FlutterRustG3Shell", "FlutterRustG4CtcUiApi", "FlutterRustG5FourPlatform"]
    );
    assert_code(
        satisfy_with_rejected_tauri_fixture(flutter_selection()),
        "CLIENT_CONFORMANCE_NON_SELECTED_STACK",
    );
}

#[test]
fn architecture_inputs_are_empty_before_selection_and_exact_singleton_after_copy() {
    assert!(bootstrap_and_preselection_graphs().architecture_inputs.is_empty());
    assert_eq!(postselection_graph().architecture_inputs, [
        architecture_input_from_exact_committed_decision_and_archive(),
    ]);
    assert_code(postselection_graph_with_digest_only_input(), "CAPABILITY_GRAPH_ARCHITECTURE_INPUT_INVALID");
    assert_code(postselection_graph_without_archive_manifest_digest(), "CAPABILITY_GRAPH_ARCHITECTURE_INPUT_INVALID");
    assert_code(postselection_graph_with_archive_manifest_digest_drift(), "CAPABILITY_GRAPH_ARCHITECTURE_INPUT_MISMATCH");
    assert_code(postselection_graph_with_stack_value_drift(), "CAPABILITY_GRAPH_ARCHITECTURE_INPUT_MISMATCH");
    assert_require_compile_and_validate_selected_pass_after_initial_bundle_deleted_with_network_and_global_store_disabled();
}
```

- [ ] **Step 2: Run tests and verify RED.**

Run: `npm --prefix clients/technology-gate/tauri2 ci`

Run: `npm --prefix clients/technology-gate/tauri2 test -- --run`

Run: `cargo test -p ep-xtask --test f57_client_gate --test f57_client_build --test f57_client_conformance -- --nocapture`

Expected: FAIL because the client-gate evaluator does not exist.

- [ ] **Step 3: Implement a non-negotiable platform matrix.**

```rust
pub const CLIENT_GATE_PLATFORMS: [ClientPlatformV1; 4] = [
    ClientPlatformV1::Windows,
    ClientPlatformV1::Macos,
    ClientPlatformV1::Ios,
    ClientPlatformV1::Android,
];

pub const CLIENT_GATE_CAPABILITIES: [&str; 8] = [
    "SECURE_STORAGE", "FILE_PICKER", "CAMERA", "BARCODE_SCAN",
    "NOTIFICATION", "ENTERPRISE_SIGNING", "INSTALL_UPGRADE", "PERFORMANCE_ACCESSIBILITY",
];
```

Each platform result binds real runner/device, generated protocol digest, package ID/bytes, signature identity, candidate/attempt or candidate-run binding, and embeds the exact eight independently signed lifecycle rows `{PACKAGE_SIGNATURE,INSTALL_START,UPGRADE,REVOCATION,CAPABILITY,RESOURCE,DLP,ACCESSIBILITY}`. Every lifecycle row exact-repeats that context and uses its tagged named readback; missing, simulated, generic, cross-package or mismatched result is failure. The aggregate is the only durable platform output; no lifecycle filename or side store may be introduced. One honest hard FAIL selects the Flutter branch; UNKNOWN is inconclusive and selects nothing. No committee may override either result with prose.

`decide` returns an unsigned `ClientStackDecisionPayloadV1` for evaluator tests. Only the clean-HEAD evidence command may create/adopt the global slot, byte-copy it into the fixed store, open/adopt the signed attempt header and prefix-authenticated journal, durably start and reconcile the four platform operations, create-new the latest checkpoint, and wrap the complete attempt as `ClientStackDecisionV1`. Each platform's unpredictable `PLATFORM_STARTED.execution_attempt_id` must exact-match its aggregate and all eight embedded lifecycle bindings. After four distinct terminal rows, it create-new writes the latest checkpoint and durably appends exactly one `DECISION_FINALIZATION_STARTED{finalization_attempt_id,frozen_input_checkpoint_ref,decided_at_unix_ms,signing_not_after_unix_ms,timestamp_request_nonce_lowerhex}`. The nonce is exactly 32 unpredictable lowercase-hex bytes and immutable for this finalization. The record's decided time equals its own trusted journal `recorded_at_unix_ms`; `signing_not_after_unix_ms` is the checked minimum expiry across slot/header/checkpoint/platform/lifecycle inputs and is strictly later. The decision payload copies its finalization ID, checkpoint and decided time only from that frozen record. Its later trusted CMS/RFC-3161 signing time must lie inside the inclusive `[decided_at_unix_ms,signing_not_after_unix_ms]` window, within certificate validity and no later than every consumed input expiry; payload time is never rewritten to or required to equal that after-the-fact signing time.

Before any `DECISION_BOUND`, Rust creates or adopts the decision envelope and complete immutable trust closure. It first create-new writes the one decision at the fixed path from the frozen record. From the exact CMS `SignerInfo.signature` bytes and frozen nonce it deterministically reconstructs the RFC-3161 request, then create-new/fsyncs the first valid response, both role-specific certificate chains and every required role/index CRL/OCSP object. If a request/response is lost before durable creation, recovery may resend only that byte-identical request while the signing window remains open; once a valid token path exists it may only adopt exact bytes and may not choose among responses. It then create-new/fsyncs `ClientStackDecisionTrustClosureV1` at exact `trust/trust-closure.v1.json`, repeating the finalization ID, nonce, decision ref and canonical typed proof vector. Only after reloading and wholly offline-verifying that closure may it append `DECISION_BOUND{finalization_attempt_id,decision_ref,trust_closure_ref}`. Output/trust-closure-before-bound recovery appends `DECISION_RECONCILED` with the same refs, but never re-signs the decision, changes nonce/window or replaces a durable proof. A bound record cannot name trust bytes not already durable. Already-bound exact bytes return idempotently.

After bound/reconciled is durable, the store create-new writes the final checkpoint through that record, walks only typed refs from the fixed roots, and deterministically derives `ClientStackDecisionArchiveManifestV1` from the already frozen trust closure. It writes the manifest last at `archive-manifest.v1.json` in the authoritative global `archive-root`, then create-new copies the exact full closure and manifest to the initial materialization root; conflicts, partial closure or an unlisted file fail rather than returning a decision. Archive recovery adopts only byte-identical verified objects/manifest and never re-signs, re-measures, fetches a network proof or silently changes trust state. A payload with a string/free-form purpose, missing or generic slot/header/checkpoint/trust-closure ref, changed execution-attempt/finalization ID/nonce, incomplete/UNKNOWN platform, second attempt, invalid finalization/signing window, or a signed envelope whose typed purpose expectation differs fails. `client-gate` implements this exact attempt/archive store and the `require` branch guard. After selection is committed, `require` and `validate-selected` fixed-load only `docs/decisions/f57-client-stack-decision.v1.json` plus `docs/decisions/f57-client-stack-decision-archive/`, replay the decision, trust closure and archive wholly offline, and reject an initial-target/global/network dependency. `validate-selected` receives the explicit current bundle root plus exactly one `--integration|--release` mode, copies the verified committed selection proof closure into that current bundle, durably journals four build/sign/eight-category lifecycle attempts, and emits the one bound `ClientStackValidationV1`. Its four platform evidence envelopes name the final package bytes for that mode. `client-build` typed-loads that validation, never invokes a compiler, packager or signer, wraps exactly those four package refs as the four signed client lanes, dispatches the selected compiled `G5_FOUR_PLATFORM` conformance recipe over those same bytes, and emits the one bound `ClientArtifactSetV1`. It contains no business behavior and returns 70 for a stack recipe not yet delivered.

The same task extends the G0-owned stack-neutral conformance dispatcher shared by `g3`, `l2`, `gate g5`, and `gate g6`. It reads only the graph-generated `client-conformance-manifest.v1.json`, accepts the closed selectors `G3_SHELL|G4_CTC_UI_API|G5_FOUR_PLATFORM`, and maps each typed recipe ID to compiled runner code; the manifest never supplies a command line. On pre-decision G3/G4 candidates it permits the initial Tauri G3/G4 recipes only. On every post-decision G5/G6 candidate it requires a verified current stack validation, resolves all three selectors to that stack, and requires every row `DELIVERED`. `client-build` may emit only the same-run auxiliary `G5_FOUR_PLATFORM` conformance result over the exact four validation refs; G3/G4 execute once against the current authority. After candidate binding, L2 dispatches the canonical CLI-007 Requirement handler over the frozen candidate, artifact set and that auxiliary result. Missing, duplicate, stale, non-selected, `NOT_DELIVERED`, `REJECTED_FIXTURE`, pre-candidate CLI-007, or validated/shipped byte drift fails closed.

At the start of exactly one selected Task 2 branch, the sole allowed pre-copy `client-gate require` verifies the complete initial materialization and authoritative global archive, including slot, bound attempt prefix/checkpoints, four aggregates with embedded lifecycle evidence, subordinate objects, decision, pre-BOUND trust closure, timestamp, both role-specific certificate chains and every role/index signed CRL/OCSP pair. It then create-new copies the exact whole root to fixed `docs/decisions/f57-client-stack-decision-archive/` and byte-copies the manifest-listed `g5/client-stack-decision.v1.json` without resigning to `docs/decisions/f57-client-stack-decision.v1.json`; those two decision bytes must be identical. The committed archive manifest is fixed at `docs/decisions/f57-client-stack-decision-archive/archive-manifest.v1.json`; its `trust_closure_ref` exact-loads the immutable proof vector, every manifest entry resolves below that root with exact digest/media/size, and no extra file exists. Both paths and digests are committed and graph/projection-bound in the same branch commit.

Bootstrap through every pre-selection G5 graph has `architecture_inputs=[]`; the post-selection G5/G6 authored and compiled graphs have exactly one row `{input_id=CLIENT_STACK_DECISION,artifact_sha256=<exact committed envelope sha256>,archive_manifest_sha256=<exact committed archive manifest sha256>,media_type=application/vnd.ep.f57-client-stack-decision-v1+json,selected_stack=<typed decision value>}`, with `TAURI2_CERTIFIED -> tauri2` and `FLUTTER_RUST_REQUIRED -> flutter-rust`. Compilation fixed-loads both committed paths, verifies the manifest and complete offline signed/TSA/revocation closure, checks decision-byte identity, and rejects digest/value drift, missing/extra archive content, another decision, unsigned locator-as-authority, or an uncommitted copy. Once committed, every `client-gate require|validate-selected` uses only those fixed repository paths and selected stack, materializing the needed exact proof closure in the current bundle. It never reads the deleted initial target, global Windows archive, network, sibling path or ambient certificate cache. A golden deletes the initial target and disables global/network access before proving `require`, graph compile and `validate-selected` still PASS. The original candidate identity and attempt history remain permanent architecture evidence. A later candidate must run `client-gate validate-selected --selection-receipt docs/decisions/f57-client-stack-decision.v1.json --candidate <current> <--integration|--release>` and obtain a new `ClientStackValidationV1`; it may PASS or FAIL but may not switch stacks. Failure blocks the candidate and requires an explicitly approved architecture change, signed attempt supersession and plan revision, never an automatic release-time fallback.

The real four-runner conformance result is the sole raw input to the G5 `CLI-007` handler; the generated `clients/technology-gate/tauri2/tests/gate.spec.ts` wrapper delegates to `handlers/cli_007.ts`. The handler is first started by candidate-bound L2, not by pre-candidate `client-build`. A simulated, missing, byte-mismatched or prose-overridden platform result cannot mark that Requirement delivered.

- [ ] **Step 4: Run evaluator tests before commit.**

Run: `cargo test -p ep-xtask --test f57_client_gate --test f57_client_build --test f57_client_conformance -- --nocapture`

Expected: PASS, including executable discovery of canonical symbol `t_f57_cli_007`, exact platform set, fallback selection, signature/candidate mismatch, and unknown recipe negatives. No signed client decision is issued from the dirty worktree.

- [ ] **Step 5: Commit the technology decision mechanism.**

```bash
cargo xtask f57 task stage --task G5-01
cargo xtask f57 task verify-staged --task G5-01
git commit -m "test: add four platform client technology gate"
```

- [ ] **Step 6: Run the real decision only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run on the evidence aggregator before an attempt header exists: `cargo xtask f57 client-gate --stack tauri2 --candidate HEAD --storage-manifest <absolute-path> --deployment-manifest <absolute-path> --deployment-manifest-signature <absolute-path> --deployment-trust-bundle <absolute-path> --storage-trust-root <absolute-path> --storage-revocation <absolute-path> --storage-checkpoint <absolute-path> --bundle-root target/f57/evidence --out target/f57/evidence/g5/client-stack-decision.v1.json`

Expected: the absent-header grammar requires all seven storage/bootstrap paths exactly once, obtains private `ValidatedDataRootV1`, creates/adopts the one fixed global DATA_HDD authority slot and its unpredictable attempt ID, archives the signer registry plus those seven verified inputs before the header, writes canonical bytes first below its authoritative create-new `archive-root`, and materializes them byte-for-byte in the explicit initial bundle. After the header exists, recovery uses the same command with all seven flags absent; any one of them is then forbidden and only authenticated archive refs may be resolved. It records one durable physical operation and exact eight-category evidence set per platform, freezes the exact checkpoint/time/signing window plus one 32-byte unpredictable lowercase-hex timestamp nonce in `DECISION_FINALIZATION_STARTED`, and produces exactly `TAURI2_CERTIFIED` or `FLUTTER_RUST_REQUIRED`. The signed decision contains the same finalization ID, slot ref, authenticated header-prefix ref, frozen create-new checkpoint, four real terminal platform results with exact execution-attempt IDs, and frozen `decided_at_unix_ms`; the actual trusted signing time lies within the frozen window and all input expiries. Before BOUND it create-new/fsyncs the decision, exact timestamp response, both `DECISION_SIGNER|TSA` chains, every role/index CRL/OCSP pair and `trust/trust-closure.v1.json`, then wholly offline-verifies that closure. `DECISION_BOUND|DECISION_RECONCILED` repeat the same finalization, decision and trust-closure refs. Only afterward does it add the final checkpoint and derive matching deterministic `archive-manifest.v1.json` bytes last in both roots. It binds the new committed `HEAD` and writes no repository file. UNKNOWN returns nonzero with no decision; a finalization/trust/archive crash signs once, resends only the byte-identical frozen timestamp request before a durable valid token, or adopts exact bytes under the same finalization ID and nonce. Retry never starts a second measurement/signature, replaces durable trust proof or fetches a different proof after BOUND.

Run: `cargo xtask f57 evidence verify --receipt target/f57/evidence/g5/client-stack-decision.v1.json --bundle-root target/f57/evidence --expect-type CLIENT_STACK_DECISION_V1`

Expected: PASS only when the initial materialization and authoritative archive root are exact byte-matching, the manifest exact-loads its pre-BOUND trust closure and closes every signed/journal/TSA/revocation object with no extras, the proof vector has one timestamp, both role chains and all role/index CRL/OCSP pairs, and both the decision signer and TSA were valid at the trusted signing time. This selects exactly one of Task 2A/2B; Task 9 rebuilds that same selected branch on the later complete G5 candidate.

### Task 2A: Complete the Tauri Workbench branch when certified

**Files:**
- Create by exact create-new archive copy (`NEW_TREE`): `docs/decisions/f57-client-stack-decision-archive/`
- Create by exact byte-copy: `docs/decisions/f57-client-stack-decision.v1.json`
- Modify: `clients/workbench/package.json`
- Modify: `clients/workbench/package-lock.json`
- Modify: `clients/workbench/src/app/App.tsx`
- Modify: `clients/workbench/src/security/endpoint.ts`
- Modify: `clients/workbench/src-tauri/Cargo.toml`
- Modify: `clients/workbench/src-tauri/src/main.rs`
- Modify: `clients/workbench/src-tauri/tauri.conf.json`
- Create: `clients/workbench/src/platform/device.ts`
- Create: `clients/workbench/src/platform/secure-storage.ts`
- Create: `clients/workbench/src/platform/files-camera-scan.ts`
- Create: `clients/workbench/tests/four-platform-contract.test.ts`
- Create: `testkit/tests/f57_four_platform_contract.rs`
- Create: `testkit/src/f57_cases/g5/workbench_client.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/client-conformance-manifest.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: a verified `TAURI2_CERTIFIED` decision only.
- Produces: canonical DAG product `client-branch:selected` from the verified signed `TAURI2_CERTIFIED` decision, its create-new complete immutable offline archive at `docs/decisions/f57-client-stack-decision-archive/`, the byte-identical convenience decision at `docs/decisions/f57-client-stack-decision.v1.json`, the singleton graph binding carrying both decision and archive-manifest digests, one Tauri codebase, and four signed platform artifacts. This selected task exclusively owns the committed archive tree; it copies but never reconstructs or re-signs Task 1 evidence.

- [ ] **Step 1: Write the failing shared-vector test.**

```rust
#[test]
fn all_tauri_packages_return_the_same_protocol_results() {
    let vectors = load_employee_positive_and_negative_vectors();
    let results = run_signed_client_artifacts(vectors);
    assert_eq!(results.windows, results.macos);
    assert_eq!(results.windows, results.ios);
    assert_eq!(results.windows, results.android);
}
```

- [ ] **Step 2: Verify branch authorization and RED.**

Run: `cargo xtask f57 client-gate require --decision TAURI2_CERTIFIED --receipt target/f57/evidence/g5/client-stack-decision.v1.json --bundle-root target/f57/evidence`

Expected: PASS only for the certified decision after verifying the initial and global archive roots byte-for-byte, replaying the complete historical slot/bound-prefix/checkpoint/platform/lifecycle/object closure, exact pre-BOUND trust closure, one timestamp, both `DECISION_SIGNER|TSA` chains and every role/index CRL/OCSP pair, and proving the archive manifest's exact no-extra set; otherwise this task is not selected and Task 2B is selected. The guarded create-new operation copies the complete root to `docs/decisions/f57-client-stack-decision-archive/`, copies its manifest-listed decision bytes to `docs/decisions/f57-client-stack-decision.v1.json`, proves their identity, and writes the exact singleton `architecture_inputs` binding with both `artifact_sha256` and `archive_manifest_sha256` before graph/projection regeneration. Manual reconstruction, resigning, partial copy, hash-only binding or selected-stack drift is forbidden. The branch test deletes the initial target and disables the global store/network, then requires `client-gate require`, graph compile and `validate-selected` to PASS solely from the fixed committed archive.

Run: `cargo test -p ep-testkit --test f57_four_platform_contract -- --nocapture`

Expected: FAIL until four signed artifacts and adapters exist.

- [ ] **Step 3: Implement adapters without business branching.**

```typescript
export interface DeviceCapabilityPortV1 {
  secureStore(key: string, value: Uint8Array): Promise<void>;
  secureLoad(key: string): Promise<Uint8Array | null>;
  pickFile(policy: FilePolicyV1): Promise<PickedFileV1>;
  capture(policy: CapturePolicyV1): Promise<CapturedMediaV1>;
  scanBarcode(policy: ScanPolicyV1): Promise<BarcodeResultV1>;
}
```

Only adapters vary by OS. Generated commands, query results, error handling, authorization display, and closure logic stay shared. Jailbreak/Root/unmanaged state blocks sensitive cache and high-risk actions. At this task `workbench_client.rs` implements only `CLI-001`, `CLI-002`, `CLI-006`, `CLI-008`, and `CLI-009`. Offline draft/cache/revocation/reconnect Requirements `CLI-003`, `CLI-004`, and `CLI-005` remain `NOT_DELIVERED` until Task 3; `SEC-015` remains fail-closed until Task 2C.

The graph update exact-checks that the Tauri `G3_SHELL` and `G4_CTC_UI_API` rows were already `DELIVERED`, changes only Tauri `G5_FOUR_PLATFORM` from `NOT_DELIVERED` to `DELIVERED`, and leaves every Flutter row `NOT_DELIVERED`. The three Tauri rows retain the exact source paths and closed recipe IDs frozen in master §3. Regeneration must fail if the selected stack does not have exactly one delivered carrier for each conformance ID or if this task rewrites an earlier row.

- [ ] **Step 4: Run pre-commit shared-code tests.**

Run: `npm --prefix clients/workbench test -- --run`

Run: `npm --prefix clients/workbench run build`

Expected: PASS for generated protocol vectors and platform adapter contracts. No signed artifact manifest is issued from the dirty worktree.

- [ ] **Step 5: Commit the certified Tauri branch.**

```bash
cargo xtask f57 task stage --task G5-02A
cargo xtask f57 task verify-staged --task G5-02A
git commit -m "feat: complete four platform tauri workbench"
```

- [ ] **Step 6: Build branch evidence only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 client-build engineering --stack tauri2 --candidate HEAD`

Expected: builds/tests Windows, macOS, iOS, and Android on their required runners, then runs only the five delivered Workbench Requirement handlers and `f57_four_platform_contract`. This engineering branch emits no signed artifact manifest, output path, journal record or consumable evidence; its build-directory bytes are never accepted by candidate construction. The repository remains clean and `CLI-003..005` cannot be selected or passed here.

### Task 2B: Build the Flutter + Rust Workbench branch when required

**Files:**
- Create by exact create-new archive copy (`NEW_TREE`): `docs/decisions/f57-client-stack-decision-archive/`
- Create by exact byte-copy: `docs/decisions/f57-client-stack-decision.v1.json`
- Move: `clients/workbench/package.json` to `clients/technology-gate/tauri2-ctc01/package.json`
- Move: `clients/workbench/package-lock.json` to `clients/technology-gate/tauri2-ctc01/package-lock.json`
- Move: `clients/workbench/tsconfig.json` to `clients/technology-gate/tauri2-ctc01/tsconfig.json`
- Move: `clients/workbench/vite.config.ts` to `clients/technology-gate/tauri2-ctc01/vite.config.ts`
- Move: `clients/workbench/src/app/App.tsx` to `clients/technology-gate/tauri2-ctc01/src/app/App.tsx`
- Move: `clients/workbench/src/api/authority.ts` to `clients/technology-gate/tauri2-ctc01/src/api/authority.ts`
- Move: `clients/workbench/src/tasks/TaskHome.tsx` to `clients/technology-gate/tauri2-ctc01/src/tasks/TaskHome.tsx`
- Move: `clients/workbench/src/features/ctc01/Ctc01Flow.tsx` to `clients/technology-gate/tauri2-ctc01/src/features/ctc01/Ctc01Flow.tsx`
- Move: `clients/workbench/src/schema/renderer.tsx` to `clients/technology-gate/tauri2-ctc01/src/schema/renderer.tsx`
- Move: `clients/workbench/src/security/endpoint.ts` to `clients/technology-gate/tauri2-ctc01/src/security/endpoint.ts`
- Move: `clients/workbench/src-tauri/Cargo.toml` to `clients/technology-gate/tauri2-ctc01/src-tauri/Cargo.toml`
- Move: `clients/workbench/src-tauri/src/main.rs` to `clients/technology-gate/tauri2-ctc01/src-tauri/src/main.rs`
- Move: `clients/workbench/src-tauri/tauri.conf.json` to `clients/technology-gate/tauri2-ctc01/src-tauri/tauri.conf.json`
- Move: `clients/workbench/e2e/ctc01.spec.ts` to `clients/technology-gate/tauri2-ctc01/e2e/ctc01.spec.ts`
- Move: `clients/workbench/tests/g3-shell.conformance.test.ts` to `clients/technology-gate/tauri2-ctc01/tests/g3-shell.conformance.test.ts`
- Create: `clients/technology-gate/tauri2-ctc01/README.md`
- Create: `clients/technology-gate/tauri2-ctc01/fixture-manifest.v1.json`
- Create: `clients/workbench/pubspec.yaml`
- Create: `clients/workbench/pubspec.lock`
- Create: `clients/workbench/lib/main.dart`
- Create: `clients/workbench/lib/app/app.dart`
- Create: `clients/workbench/lib/app/task_home.dart`
- Create: `clients/workbench/lib/features/ctc01/ctc01_flow.dart`
- Create: `clients/workbench/lib/schema/renderer.dart`
- Create: `clients/workbench/lib/api/authority_bridge.dart`
- Create: `clients/workbench/lib/platform/device_capability.dart`
- Create: `clients/workbench/lib/security/endpoint.dart`
- Create: `clients/workbench/rust/Cargo.toml`
- Create: `clients/workbench/rust/src/lib.rs`
- Create (`NEW_TREE`): `clients/workbench/android`
- Create (`NEW_TREE`): `clients/workbench/ios`
- Create (`NEW_TREE`): `clients/workbench/macos`
- Create (`NEW_TREE`): `clients/workbench/windows`
- Create: `clients/workbench/test/g3_shell_conformance_test.dart`
- Create: `clients/workbench/test/four_platform_contract_test.dart`
- Create: `clients/workbench/integration_test/ctc01_ui_api_test.dart`
- Create: `testkit/tests/f57_four_platform_contract.rs`
- Create: `testkit/src/f57_cases/g5/workbench_client.rs`
- Modify: `xtask/tests/f57_client_conformance.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/client-conformance-manifest.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: a verified `FLUTTER_RUST_REQUIRED` decision only, the same generated Employee vectors, and graph-generated Rust protocol DTOs.
- Produces: canonical DAG product `client-branch:selected` from the verified signed `FLUTTER_RUST_REQUIRED` decision, its create-new complete immutable offline archive at `docs/decisions/f57-client-stack-decision-archive/`, the byte-identical convenience decision at `docs/decisions/f57-client-stack-decision.v1.json`, the singleton graph binding carrying both decision and archive-manifest digests, Flutter UI plus the one Rust protocol/security core, and selected-stack replacements for the G3 shell and G4 CTC UI/API conformance carriers. This selected task exclusively owns the committed archive tree; it copies but never reconstructs or re-signs Task 1 evidence.

- [ ] **Step 1: Write the failing Dart/Rust golden-vector test.**

```dart
test('generated command bytes and errors match the authority vectors', () async {
  final vectors = await EmployeeGoldenVectors.load();
  for (final vector in vectors.all) {
    expect(await RustAuthorityCore.evaluate(vector.request), vector.expected);
  }
});
```

- [ ] **Step 2: Verify branch authorization and RED.**

Run: `cargo xtask f57 client-gate require --decision FLUTTER_RUST_REQUIRED --receipt target/f57/evidence/g5/client-stack-decision.v1.json --bundle-root target/f57/evidence`

Expected: PASS only for the fallback decision after verifying the initial and global archive roots byte-for-byte, replaying the complete historical slot/bound-prefix/checkpoint/platform/lifecycle/object closure, exact pre-BOUND trust closure, one timestamp, both `DECISION_SIGNER|TSA` chains and every role/index CRL/OCSP pair, and proving the archive manifest's exact no-extra set; otherwise Task 2A is selected. The guarded create-new operation copies the complete root to `docs/decisions/f57-client-stack-decision-archive/`, copies its manifest-listed decision bytes to `docs/decisions/f57-client-stack-decision.v1.json`, proves their identity, and writes the exact singleton `architecture_inputs` binding with both `artifact_sha256` and `archive_manifest_sha256` before graph/projection regeneration. Manual reconstruction, resigning, partial copy, hash-only binding or selected-stack drift is forbidden. The branch test deletes the initial target and disables the global store/network, then requires `client-gate require`, graph compile and `validate-selected` to PASS solely from the fixed committed archive.

Run: `flutter test clients/workbench/test/g3_shell_conformance_test.dart clients/workbench/test/four_platform_contract_test.dart`

Expected: FAIL because the Flutter/Rust bridge, G3 shell, G4 integration carrier, and Rust protocol core do not exist.

- [ ] **Step 3: Implement the fallback with identical public behavior.**

```rust
pub fn evaluate_employee_vector(bytes: &[u8]) -> Result<Vec<u8>, ClientCoreErrorV1> {
    let request: GeneratedEmployeeRequestV1 = serde_json::from_slice(bytes)?;
    let result = validate_and_prepare_non_authoritative_request(request)?;
    Ok(serde_json::to_vec(&result)?)
}
```

Recreate the complete G3 Windows shell and G4 Workbench CTC UI/API behavior first, then add the four platform device adapters. Dart never receives a hand-authored or independently generated Employee DTO: `authority_bridge.dart` passes canonical bytes and opaque, versioned view-model handles through FFI to `clients/workbench/rust`, whose types are graph-generated members of `docs/generated/f57/rust/manifest.v1.json`. The Rust core alone parses generated protocol bytes and returns presentation-safe values; Flutter cannot branch business state, authorization, amount, or error semantics.

Only after the Flutter G3 and G4 parity tests pass may the exact leaf-file moves above occur. `fixture-manifest.v1.json` records every moved path/digest and `release_eligible=false`; the fixture is excluded from Cargo/npm/package discovery and exists only for negative regression tests. The graph update marks all three Flutter conformance rows `DELIVERED` and every Tauri row `REJECTED_FIXTURE`, then regenerates the client conformance manifest. At this task `workbench_client.rs` implements only `CLI-001`, `CLI-002`, `CLI-006`, `CLI-008`, and `CLI-009`. Offline draft/cache/revocation/reconnect Requirements `CLI-003`, `CLI-004`, and `CLI-005` remain `NOT_DELIVERED` until Task 3; `SEC-015` remains fail-closed until Task 2C.

- [ ] **Step 4: Run pre-commit Flutter/Rust tests.**

Run: `flutter test clients/workbench/test/g3_shell_conformance_test.dart clients/workbench/test/four_platform_contract_test.dart`

Expected: PASS.

Run on the controlled Windows runner: `flutter test clients/workbench/integration_test/ctc01_ui_api_test.dart -d windows`

Expected: PASS through the same Employee HTTPS surface and CTC-01 fixture used by G4; no handler/repository shortcut.

Run: `cargo test -p ep-xtask --test f57_client_conformance -- --nocapture`

Expected: PASS; the Flutter selection resolves all three conformance IDs to Flutter, and the preserved Tauri fixture resolves to none.

Expected: PASS with the same generated vectors as the prior Windows slice. No signed artifact manifest is issued from the dirty worktree.

- [ ] **Step 5: Commit the fallback branch.**

```bash
cargo xtask f57 task stage --task G5-02B
cargo xtask f57 task verify-staged --task G5-02B
git commit -m "feat: adopt flutter rust workbench fallback"
```

- [ ] **Step 6: Build branch evidence only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 client-build engineering --stack flutter-rust --candidate HEAD`

Expected: four-platform engineering build/test plus only the five delivered Workbench Requirement handlers and `G5_FOUR_PLATFORM`; no signed artifact manifest, output path, journal record or consumable evidence is emitted. The repository remains clean. The current-candidate G5 validation-based build later produces the only `ClientArtifactSetV1` and runs selected Flutter `G3_SHELL`/`G4_CTC_UI_API` carriers against the current authority. `CLI-003..005` cannot be selected or passed here.

Exactly one of Task 2A or Task 2B is executed. The selected branch creates `testkit/tests/f57_four_platform_contract.rs` and `testkit/src/f57_cases/g5/workbench_client.rs` exactly once; the non-selected task performs no file operation. Both branches deliver the same three conformance IDs through the same generated manifest and public interface; no due result is marked skipped. G5/G6 never reuse the earlier Tauri G3/G4 receipt after a Flutter selection.

### Task 2C: Complete bounded remote support and four-platform DLP

**Files:**
- Create: `crates/platform/support/Cargo.toml`
- Create: `crates/platform/support/src/lib.rs`
- Create: `crates/platform/support/src/session.rs`
- Create: `crates/platform/support/src/policy.rs`
- Create: `crates/platform/support/src/cleanup.rs`
- Create: `crates/platform/endpoint-policy/Cargo.toml`
- Create: `crates/platform/endpoint-policy/src/lib.rs`
- Create: `crates/platform/endpoint-policy/src/dlp.rs`
- Create: `apps/support-provisioner/Cargo.toml`
- Create: `apps/support-provisioner/src/main.rs`
- Create: `apps/support-provisioner/src/session.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/remote_support_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/platform/remote_support.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Create: `apps/core-server/src/wiring/support.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `clients/control-center/src/features/support/SupportSessionDesk.tsx`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src/security/dlp.ts`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/security/dlp.dart`
- Create: `db/migrations/platform_ops/V20261025091700__platform_ops_create_remote_support_sessions.sql`
- Create: `testkit/tests/f57_remote_support_lifecycle.rs`
- Create: `testkit/tests/f57_endpoint_dlp.rs`
- Modify: `testkit/Cargo.toml`
- Create: `testkit/src/f57_cases/g5/control_center_contract.rs`
- Create: `testkit/src/f57_cases/g5/workbench_security.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: selected four-platform stack, current authorization/SoD/MFA/device posture, support work order, zero-outbound policy, generated Control/Employee protocols.
- Produces: `SupportSessionV1`, `SupportScopeV1`, `EndpointDlpDecisionV1`, one-time provisioner lease, and concrete G5 handlers for `SEC-006` and `SEC-015`. `OPS-001` remains `NOT_DELIVERED` until Task 15 verifies the final Control Center against all release carriers.

- [ ] **Step 1: Write failing lifecycle, escape, and honesty tests.**

```rust
#[test]
fn support_session_is_bounded_and_has_no_privileged_backdoor() {
    assert_exact_states(["REQUESTED", "APPROVED", "READY", "ACTIVE", "CLOSED", "REVOKED", "EXPIRED", "FAILED_CONTAINED"]);
    assert_eq!(default_support_ttl(), Duration::from_secs(3600));
    assert_eq!(maximum_support_ttl(), Duration::from_secs(4 * 3600));
    assert_zero_credentials(["DATABASE", "KMS", "BACKUP_CONTROL", "PERMANENT_TUNNEL"]);
}

#[test]
fn unmanaged_or_compromised_endpoint_cannot_receive_high_classification_bytes() {
    for posture in ["UNMANAGED", "ROOTED", "JAILBROKEN", "ATTESTATION_STALE"] {
        assert_high_classification_denied(posture);
    }
}
```

- [ ] **Step 2: Run focused tests and verify RED.**

Run: `cargo test -p ep-testkit --test f57_remote_support_lifecycle --test f57_endpoint_dlp -- --nocapture`

Expected: FAIL because session authority, controlled provisioner, and endpoint policy do not exist.

- [ ] **Step 3: Implement fail-closed support and DLP.**

Every support session binds requesting person, distinct approver, work order, authenticated origin, customer-controlled VPN or one-time reverse-session endpoint, legal entity, objects, fields, actions, device, authority epoch, generation, start, expiry, and audit chain. `REQUESTED -> APPROVED -> READY -> ACTIVE` is the only opening path. MFA, reauthentication, SoD, current authorization, and customer-side approval are checked again at activation. Expiry, revocation, network change, service restart, grant narrowing, or cleanup uncertainty terminates access and destroys ephemeral credentials; failure lands in `FAILED_CONTAINED`. The provisioner has no business SQL, database admin, KMS unwrap, backup, restore, package-signing, or permanent inbound/outbound credential.

The shared Rust endpoint policy performs server-side field/row projection first, then carrier-specific decisions for watermark, print, clipboard, share, screenshot/recording control where the OS exposes an enforceable API, managed download, export approval, offline cache, wipe, Root/Jailbreak, and stale device posture. Windows/macOS/iOS/Android return a signed enforcement readback. Browser or OS limitations are displayed as `BEST_EFFORT_BOUNDARY`; the product never claims to block an external camera. A missing carrier result denies high-classification delivery. `remote_support_store` is the only SQL implementation, reached by `support.rs` through `AuthorizedPgTx`; support-provisioner receives only a time-bounded signed provisioning request and never a database credential. `apps/core-server/src/platform/mod.rs` registers the support HTTPS surface in this same task. The same staged commit adds explicit `core-server -> ep-platform-support + ep-platform-endpoint-policy` and typed `ep-testkit` dev dependencies; support wiring and its tests must be reachable under all-target compilation rather than deferred as unlinked source.

Register only `SEC-006` in `control_center_contract.rs` and `SEC-015` in `workbench_security.rs`; regenerate the manifest without changing the canonical facade symbols.

- [ ] **Step 4: Prove Fresh PG and the full negative matrix.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025091700`

Expected: engineering rehearsal PASS with reservation `91700` marked `CREATED` and zero support credential outside its bounded tables/secret lease.

Run: `cargo test -p ep-testkit --test f57_remote_support_lifecycle --test f57_endpoint_dlp -- --nocapture`

Expected: PASS for approval/MFA/SoD, scope escape, expiry, revoke, network change, crash cleanup, diagnostic redaction, watermark/export/print/clipboard/share/download, compromised/offline device, and honest unenforceable-carrier cases.

Run: `cargo test -p ep-platform-support -p ep-platform-endpoint-policy -p support-provisioner -p core-server -p ep-testkit --all-targets --locked`

Expected: PASS with the support/platform wiring and focused-test imports reachable in this task's own Cargo graph; no missing/path/git/deferred dependency or lockfile rewrite.

- [ ] **Step 5: Commit support and DLP atomically.**

```bash
cargo xtask f57 task stage --task G5-02C
cargo xtask f57 task verify-staged --task G5-02C
```

Add exactly the selected-stack DLP adapter path to the same staging set, then commit: `feat(security): add bounded remote support and endpoint dlp`.

### Task 3: Add device lifecycle and bounded offline intents

**Files:**
- Create: `crates/platform/sync/Cargo.toml`
- Create: `crates/platform/sync/src/lib.rs`
- Create: `crates/platform/sync/src/intent.rs`
- Create: `crates/platform/sync/src/conflict.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/offline_intent_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/sync.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025091710__platform_meta_create_offline_intents.sql`
- Create: `testkit/tests/f57_offline_conflicts.rs`
- Create: `testkit/tests/f57_device_lifecycle.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `apps/core-server/src/platform/client_sessions.rs`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src/offline/intent.ts`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src/offline/cache.ts`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src/offline/reconnect.ts`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src/platform/device-lifecycle.ts`
- Create (`TAURI2_CERTIFIED`): `clients/workbench/src-tauri/src/offline.rs`
- Modify (`TAURI2_CERTIFIED`): `clients/workbench/src/app/App.tsx`
- Modify (`TAURI2_CERTIFIED`): `clients/workbench/src/api/authority.ts`
- Modify (`TAURI2_CERTIFIED`): `clients/workbench/src/platform/device.ts`
- Modify (`TAURI2_CERTIFIED`): `clients/workbench/src-tauri/src/main.rs`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/offline/intent.dart`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/offline/cache.dart`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/offline/reconnect.dart`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/platform/device_lifecycle.dart`
- Create (`FLUTTER_RUST_REQUIRED`): `clients/workbench/rust/src/offline.rs`
- Modify (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/main.dart`
- Modify (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/api/authority_bridge.dart`
- Modify (`FLUTTER_RUST_REQUIRED`): `clients/workbench/lib/platform/device_capability.dart`
- Modify (`FLUTTER_RUST_REQUIRED`): `clients/workbench/rust/src/lib.rs`
- Modify: `testkit/src/f57_cases/g5/workbench_client.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: canonical DAG product `client-branch:selected` with its verified signed stack decision and selected-branch artifact set, `ClientIntentV1`, device/session contracts, dynamic authorization.
- Produces: encrypted bounded offline projection, signed intents, server replay, conflict classification, re-attestation/wipe lifecycle.

- [ ] **Step 1: Write failing high-risk conflict and revocation tests.**

```rust
#[test]
fn money_quantity_state_permission_and_contract_conflicts_require_human_review() {
    for field in ["amount_minor", "quantity", "state", "grant", "contract_term"] {
        assert_eq!(classify_conflict(field), ConflictResolutionV1::HumanReview);
    }
}

#[tokio::test]
async fn revoked_device_intent_cannot_replay_after_reconnect() {
    assert_eq!(replay_from_revoked_device().await.unwrap_err().code(), "DEVICE_REVOKED");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_offline_conflicts --test f57_device_lifecycle -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement bounded, non-authoritative offline behavior.**

```rust
pub enum ConflictResolutionV1 { AutoMergeTextNote, ServerWinsDisplayOnly, HumanReview }

pub async fn replay_intent(
    employee_ingress: &dyn EmployeeCommandIngressV1,
    reconnected_context: &VerifiedIngressContextV1,
    intent: ClientIntentV1,
) -> Result<CommandReceiptV1, IntentReplayErrorV1> {
    validate_intent_carrier_without_reserializing(&intent)?;
    employee_ingress
        .submit_exact(reconnected_context, &intent.envelope_jcs)
        .await
        .map_err(Into::into)
}
```

`EmployeeCommandIngressV1::submit_exact` is the same parser/authentication/`AuthorityCommandGatewayV1` path used by `/employee/v1/commands`; it strict-parses the original canonical device-signed envelope bytes and never calls `CommandPipeline` directly or constructs a second command. The authenticated reconnect derives current principal/device context, and the authority rechecks signature, generation, current grants, revocation, SoD, risk, and idempotency. Cache copy/rollback, Root/Jailbreak, stale generation, permission narrowing, expired device, wipe receipt replay, stolen signing key, byte mutation, and reserialization are negative tests.

`offline_intent_store` is the only SQL adapter for the new table and is composed through `sync.rs`; `ep-platform-sync` contains no SQL. This task declares `ep-platform-sync` directly in `apps/core-server/Cargo.toml` and `testkit/Cargo.toml`, registers reachable wiring in the same commit, and compiles both consumers under the locked workspace; a source-only module or later dependency repair is forbidden. This task changes the existing handler module from five to all eight Workbench rows by adding exactly `CLI-003`, `CLI-004`, and `CLI-005`. Those three handlers require the offline/device tests in this task and cannot reuse Task 2 platform-only evidence.

- [ ] **Step 4: Run Fresh PG and four-client tests.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025091710`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_offline_conflicts --test f57_device_lifecycle --test f57_four_platform_contract -- --nocapture`

Expected: PASS, including exact execution of newly delivered `t_f57_cli_003`, `t_f57_cli_004`, and `t_f57_cli_005`; the other five Workbench handlers retain their prior evidence and are not falsely reissued from a dirty task run.

Run: `cargo test -p ep-platform-sync -p core-server -p ep-testkit --all-targets --locked`

Expected: PASS with the sync store/wiring and typed tests linked in this same task and no lockfile drift.

Run only the branch selected by the same signed decision used by `task stage`:

- `TAURI2_CERTIFIED`: `npm --prefix clients/workbench test -- --run`
- `FLUTTER_RUST_REQUIRED`: `flutter test clients/workbench/test/g3_shell_conformance_test.dart clients/workbench/test/four_platform_contract_test.dart`

Expected: PASS for the selected client only. Running or staging the other branch is a hard failure; the offline change cannot divert G3/G4 conformance to a different stack.

- [ ] **Step 5: Commit sync/device lifecycle.**

```bash
cargo xtask f57 task stage --task G5-03
cargo xtask f57 task verify-staged --task G5-03
git commit -m "feat: add bounded offline intents and device lifecycle"
```

### Task 4: Complete provider, MCP, WASM, worker, and conditional container containment

**Files:**
- Modify: `crates/platform/provider/src/manifest.rs`
- Create: `crates/platform/provider/src/permission.rs`
- Modify: `crates/platform/provider/src/invocation.rs`
- Create: `docs/schemas/f57-provider-permission.v1.schema.json`
- Create: `crates/platform/provider/tests/fixtures/f57-provider-permission-v1-golden.json`
- Modify: `crates/platform/mcp/src/lib.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/provider_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `apps/core-server/src/wiring/provider.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `crates/adapter/windows-worker/Cargo.toml`
- Create: `crates/adapter/windows-worker/src/lib.rs`
- Create: `crates/adapter/windows-container/Cargo.toml`
- Create: `crates/adapter/windows-container/src/lib.rs`
- Modify: `crates/adapter/wasm/Cargo.toml`
- Modify: `apps/integration-gateway/src/main.rs`
- Modify: `apps/integration-gateway/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025091800__platform_meta_extend_provider_and_mcp_registry.sql`
- Read generated: `testkit/tests/f57_provider_containment.rs`
- Read generated: `testkit/tests/f57_ai_optional.rs`
- Create: `testkit/tests/f57_mcp_containment.rs`
- Create: `testkit/tests/f57_hyperv_container.rs`
- Modify: `testkit/Cargo.toml`
- Create: `testkit/src/f57_cases/g5/provider_containment.rs`
- Create: `testkit/src/f57_cases/g5/ai_optional.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: ADR-0023 manifest/grant/invocation contracts and pin/drain mechanism.
- Produces: complete governed carrier registry plus the unique ADR-0023 `PermissionCeilingV1` Rust/schema owner; local AI remains `NullAiProviderV1` and disabled. `crates/platform/provider/src/permission.rs` and `docs/schemas/f57-provider-permission.v1.schema.json` own the exact permission/resource vocabulary once, and both provider and later package schemas import it without copying or widening any field.

- [ ] **Step 1: Write failing deny-by-default and unavailable-host tests.**

```rust
#[test]
fn p340_profile_refuses_container_without_fallback() {
    let err = WindowsContainerCarrierV1::activate(p340_profile(), signed_container_manifest()).unwrap_err();
    assert_eq!(err.code(), "HOST_CAPABILITY_UNAVAILABLE");
    assert_eq!(err.fallback_selected(), None);
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_provider_containment --test f57_ai_optional --test f57_mcp_containment --test f57_hyperv_container -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement resource-bounded carriers.**

```rust
pub trait GovernedCarrierV1 {
    fn probe(&self, host: &HostCapabilityEvidenceV1) -> CarrierAvailabilityV1;
    async fn invoke(&self, invocation: VerifiedProviderInvocationV1) -> Result<ProviderOutcomeV1, ProviderErrorV1>;
    async fn drain(&self, deadline: std::time::Instant) -> Result<DrainReceiptV1, ProviderErrorV1>;
}
```

Cover network/domain/port, object/field, secret/file, CPU/memory/time, concurrency, residency, Unknown, revocation, drift, kill, orphan cleanup, and no arbitrary native DLL. MCP write tools call typed commands only. The expanded registry is persisted only through the existing authority-owned `provider_store` and `AuthorizedPgTx`; the gateway still has zero SQL/KMS/authority-file capability. This task explicitly adds the new worker/container dependencies to the core/gateway consumer manifests and their canonical typed dependencies to `testkit`; both apps and the tests must link under `--all-targets --locked` in this commit, while the gateway dependency graph is mechanically rejected if it gains DB/KMS/authority-file crates. Fresh-PG must write/read each carrier/manifest discriminator through that store, not merely observe columns. `provider_containment.rs` implements exactly `INT-001`, `INT-004`, `PKG-003`, `MCP-001..003`, and `DEF-008`. `ai_optional.rs` implements `AI-001..005` and `DEF-001` with a real `NullAiProviderV1`, signed disabled seams, no local-model bytes, no hidden cloud call, deterministic unavailable/degraded responses, and negative product-claim evidence.

- [ ] **Step 4: Verify containment and migration.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025091800`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_provider_containment --test f57_ai_optional --test f57_mcp_containment --test f57_hyperv_container -- --nocapture`

Expected: PASS.

Run: `cargo test -p ep-adapter-windows-worker -p ep-adapter-windows-container -p core-server -p integration-gateway -p ep-testkit --all-targets --locked`

Expected: PASS with both carrier adapters reachable from their registered consumers, typed tests importing the canonical owners, zero gateway DB/KMS/authority-file dependency and no deferred Cargo repair.

- [ ] **Step 5: Commit providers.**

```bash
cargo xtask f57 task stage --task G5-04
cargo xtask f57 task verify-staged --task G5-04
git commit -m "feat: complete governed provider carriers"
```

### Task 5: Add governed office, file, approval, search, and identity services

**Files:**
- Create: `crates/platform/approval/Cargo.toml`
- Create: `crates/platform/approval/src/lib.rs`
- Create: `crates/platform/search/Cargo.toml`
- Create: `crates/platform/search/src/lib.rs`
- Modify: `crates/platform/import-export/src/lib.rs`
- Modify: `crates/platform/file/src/lib.rs`
- Modify: `crates/platform/identity/src/lib.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/approval_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_flow/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/search_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/platform_file/file_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_file/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_core/external_identity_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/office_identity.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/integration-gateway/src/providers/smtp.rs`
- Create: `apps/integration-gateway/src/providers/webhook.rs`
- Create: `apps/integration-gateway/src/providers/ad_ldap.rs`
- Modify: `apps/integration-gateway/src/providers/mod.rs`
- Modify: `apps/integration-gateway/Cargo.toml`
- Create: `db/migrations/platform_flow/V20261025091900__platform_flow_create_approval_cases.sql`
- Create: `db/migrations/platform_meta/V20261025091910__platform_meta_create_search_definitions.sql`
- Create: `db/migrations/platform_file/V20261025091920__platform_file_extend_governed_lifecycle.sql`
- Create: `db/migrations/platform_core/V20261025091930__platform_core_create_external_identity_links.sql`
- Read generated: `testkit/tests/f57_import_export.rs`
- Read generated: `testkit/tests/f57_platform_connectors_lifecycle.rs`
- Create: `testkit/tests/f57_identity_provider.rs`
- Create: `testkit/tests/f57_approval_search.rs`
- Modify: `testkit/Cargo.toml`
- Create: `testkit/src/f57_cases/g5/import_export.rs`
- Create: `testkit/src/f57_cases/g5/platform_connectors_lifecycle.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: query authorization, file quarantine/lifecycle, provider grants, dynamic SoD.
- Produces: Excel/CSV/Word/PDF actions, approval cases, authorized search, SMTP/webhook, local identity, AD/LDAP provider, and broken-glass path.

- [ ] **Step 1: Write failing adversarial office/identity tests.**

```rust
#[test]
fn spreadsheet_formula_and_external_link_are_never_executed() {
    let proposal = inspect_workbook(fixture_formula_and_external_link()).unwrap();
    assert_eq!(proposal.cells[0].classification, CellClassificationV1::UntrustedFormulaText);
    assert!(proposal.external_links_blocked);
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_import_export --test f57_platform_connectors_lifecycle --test f57_identity_provider --test f57_approval_search -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement typed, authorized services.**

```rust
pub struct AuthorizedSearchRequestV1 {
    pub capability_id: CapabilityIdV1,
    pub query_use: QueryUseV1,
    pub fields: Vec<FieldIdV1>,
    pub predicate: TypedPredicateV1,
}
```

`CapabilityIdV1` and every `FieldIdV1` must resolve in the active generated graph; the request parser rejects a free-form object/table/field string before query planning. Approval, search, file-lifecycle, and external-identity SQL live only in their four db-pg stores and are composed by `office_identity.rs` through `AuthorizedPgTx`; their platform crates contain no SQL. `providers/mod.rs` explicitly registers SMTP, webhook, and AD/LDAP, and integration-gateway still has no authority database credential. This task declares the approval/search/import-export/file/identity dependencies in the exact app manifests that consume them and in `testkit` for typed tests, then all-target compiles both apps in the same locked commit; a registered module may not rely on Task 8 to become linkable. Fresh-PG tests perform command-path write/readback through each concrete store rather than checking table existence only. Tests cover formula injection, macros, external links, row errors, field permissions, scan TOCTOU, archive bomb, hidden-field search/facet inference, AD outage, session revocation, and local two-person break-glass. XML is disabled except individually signed codec profiles. `import_export.rs` implements `MDM-006`. `platform_connectors_lifecycle.rs` implements exactly `GOV-007`, `INT-002`, `INT-003`, `PLT-001..004`, `IDP-001..003`, `SEC-007`, `SEC-008`, `DEF-004`, and `DEF-011`, consuming Task 4 carrier evidence where applicable and proving the mandatory first-stage provider exact set rather than claiming every optional vendor preinstalled.

- [ ] **Step 4: Run migration and negative suites.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025091930`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_import_export --test f57_platform_connectors_lifecycle --test f57_identity_provider --test f57_approval_search -- --nocapture`

Expected: PASS.

Run: `cargo test -p ep-platform-approval -p ep-platform-search -p core-server -p integration-gateway -p ep-testkit --all-targets --locked`

Expected: PASS with office/identity wiring and provider registration reachable now, typed tests linked to canonical owners, gateway zero-authority credentials and no lockfile drift.

- [ ] **Step 5: Commit platform services.**

```bash
cargo xtask f57 task stage --task G5-05
cargo xtask f57 task verify-staged --task G5-05
git commit -m "feat: add governed office and identity services"
```

### Task 6: Compile deployment-scoped customer relational models

**Files:**
- Create: `crates/platform/meta/src/model.rs`
- Create: `crates/platform/meta/src/compiler.rs`
- Create: `crates/platform/meta/src/plan.rs`
- Modify: `crates/platform/meta/Cargo.toml`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/model_store.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/plan_executor.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `apps/core-server/src/wiring/meta.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Read/import unchanged: `crates/platform/release/src/participant.rs`
- Read/import unchanged: `crates/platform/release/src/generation.rs`
- Create: `db/migrations/platform_meta/V20261025092000__platform_meta_create_customer_model_specs.sql`
- Create: `testkit/tests/f57_model_migration_faults.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `crates/platform/meta/src/lib.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: only a release-owned private `VerifiedGenerationParticipantApplyRequestV1` received over the authenticated endpoint registered for the meta participant, the exact model-spec artifacts in its canonical required-item subset, migration journal, storage/capacity policy and active field/capability resolver. It does not import `ep-platform-generation-activation`, accept `CompiledCapabilityGraphV1`, inspect a generation ACK/OBSERVED pointer or construct any generation/approval wire. The upper coordinator already verified manifest/graph/declaration under its transition lock and remains the only caller of the release-owned request minting boundary.
- Produces: `CustomerModelSpecV1`, `CustomerModelBindingV1`, `CompiledObjectSecurityBundleV1`, `ModelCompiler::compile_for_participant`, and one `GenerationParticipantApplyPortV1` implementation that returns exact measured application/readiness state but never an ACK or OBSERVED verdict.

- [ ] **Step 1: Write failing protected-zone and incomplete-bundle tests.**

```rust
#[test]
fn customer_sql_and_partial_security_bundle_are_rejected() {
    assert_eq!(compile_spec(spec_with_sql()).unwrap_err().code(), "CUSTOM_MODEL_SQL_FORBIDDEN");
    assert_eq!(activate_bundle(bundle_without_export_policy()).unwrap_err().code(), "OBJECT_SECURITY_BUNDLE_INCOMPLETE");
    assert_code(meta_participant_with_raw_compiled_graph(), "GENERATION_PARTICIPANT_REQUEST_REQUIRED");
    assert!(!meta_participant_api_can_construct_ack_or_commit_observed());
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_model_migration_faults -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement all-or-nothing compilation.**

```rust
pub fn compile_for_participant(
    spec: &CustomerModelSpecV1,
    request: &VerifiedGenerationParticipantApplyRequestV1,
) -> Result<CompiledObjectSecurityBundleV1, ModelCompileErrorV1> {
    require_meta_participant_and_exact_model_item_subset(request)?;
    validate_deployment_namespace(spec)?;
    let bundle = compile_physical_rls_authz_crypto_lifecycle_projection(
        spec,
        request.required_items(),
    )?;
    bundle.validate_complete()?;
    Ok(bundle)
}
```

First implementation is additive/expand-contract only. Every ext row has legal entity, security level, scope tags, row version, audit columns, FORCE RLS, same-entity compound FKs, encryption/query-use/export/retention rules, HDD quota, and rollback/checkpoint impact. `meta.rs` composes `model_store`, `plan_executor` and the stable participant port behind the command path; no compiler or customer input receives SQL, a raw connection, a raw compiled graph or an ACK. The handler exact-loads only model artifacts named by `request.required_items()`, applies or reconciles that fixed subset, and returns measured state for the coordinator's independent fresh readback. It never advances OBSERVED. The same commit declares `ep-platform-meta` in `core-server` and `testkit`, updates the locked package dependency graph and compiles both consumers; `ep-platform-meta -> ep-platform-release` is legal, while every edge/path to `ep-platform-generation-activation` is forbidden. No unlinked wiring or test-local duplicate compiler is accepted. The fault suite must compile and execute one additive model through `CommandPipeline` + `AuthorizedPgTx`, restart every journal phase, and read back the generated security bundle.

- [ ] **Step 4: Run Fresh PG and fault injection.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025092000`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_model_migration_faults -- --nocapture`

Expected: PASS for restart at each migration journal phase, namespace conflict, quota, RLS, digest, and rollback negatives.

Run: `cargo test -p ep-platform-meta -p core-server -p ep-testkit --all-targets --locked`

Expected: PASS with the canonical compiler/store/wiring reachable in this task, the request/item-subset exact binding, measured-readback/UNKNOWN reconciliation behavior, no ACK or OBSERVED write, no raw compiled-graph admission, no meta -> generation-activation edge, and no test-local nominal or deferred dependency.

- [ ] **Step 5: Commit model compiler.**

```bash
cargo xtask f57 task stage --task G5-06
cargo xtask f57 task verify-staged --task G5-06
git commit -m "feat: compile governed customer relational models"
```

### Task 7: Add signed package, license, and hotplug lifecycle

**Files:**
- Create: `crates/platform/package/Cargo.toml`
- Create: `crates/platform/package/src/lib.rs`
- Create: `crates/platform/package/src/manifest.rs`
- Create: `crates/platform/package/src/trust.rs`
- Create: `crates/platform/package/src/trust_provider.rs`
- Create: `crates/platform/package/src/maintenance.rs`
- Create: `crates/platform/package/src/finalization.rs`
- Create: `crates/platform/package/src/lifecycle.rs`
- Create: `crates/platform/package/src/participant.rs`
- Modify: `crates/platform/capability-graph/src/model.rs`
- Modify: `crates/platform/capability-graph/src/compiler.rs`
- Modify: `docs/schemas/f57-capability-graph.v1.schema.json`
- Read/import unchanged: `crates/platform/provider/src/permission.rs`
- Read/import unchanged: `docs/schemas/f57-provider-permission.v1.schema.json`
- Create: `docs/evidence/f57-capability-package.v1.schema.json`
- Create: `docs/schemas/f57-capability-package-trust-registry.v1.schema.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-implementation-manifest-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-generation-item-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-generation-transition-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-operation-request-result-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-installed-state-readback-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-maintenance-reservation-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-maintenance-authorization-scope-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-maintenance-authorization-decision-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-execution-trust-snapshot-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-maintenance-execution-authorization-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-maintenance-plan-v1-golden.json`
- Create: `crates/platform/package/tests/fixtures/f57-capability-package-trust-registry-v1-golden.json`
- Modify: `crates/platform/license/src/lib.rs`
- Read/import unchanged: `crates/platform/release/src/participant.rs`
- Read: `docs/evidence/f57-generation.v1.schema.json`
- Read: `docs/evidence/f57-foundation.v1.schema.json`
- Create: `crates/adapter/file/src/package_maintenance_plan_store.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Read/import unchanged: `crates/platform/runtime/src/storage/manifest_rotation.rs`
- Read/import unchanged: `crates/adapter/file/src/storage_manifest_rotation_store.rs`
- Create: `crates/adapter/kms/src/package_trust.rs`
- Create: `crates/adapter/kms/tests/package_trust_provider.rs`
- Modify: `crates/adapter/kms/src/lib.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/package_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/core-server/src/wiring/package.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/recovery-tool/src/package_trust.rs`
- Modify: `apps/recovery-tool/src/storage_manifest_rotation.rs`
- Modify: `apps/recovery-tool/src/main.rs`
- Modify: `apps/recovery-tool/Cargo.toml`
- Create: `db/migrations/platform_meta/V20261025092100__platform_meta_create_capability_packages.sql`
- Read generated: `testkit/tests/f57_package_hotplug.rs`
- Modify: `testkit/Cargo.toml`
- Create: `testkit/src/f57_cases/g5/package_hotplug.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Regenerate descriptor/type-ID registry only: `crates/foundation/src/generated_cms_artifacts.rs`
- Generate package-owned trait implementations: `crates/platform/package/src/generated_cms_artifacts.rs`
- Regenerate: `docs/generated/f57/cms-artifact-descriptors.v1.json`
- Regenerate: `crates/foundation/tests/fixtures/cms-artifact-descriptors-v1-golden.json`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: the Task-4 sole `PermissionCeilingV1`, CapabilityGraph compiler, license projection, foundation signing/object-store contracts, trusted clock, pin lease, provider drain, and both release-owned private `VerifiedGenerationParticipantApplyRequestV1` and `VerifiedGenerationParticipantRollbackRequestV1`/ports for atomic/drain operations. The strict package payload has exactly the master's thirteen fields, including graph-owned `component_class` and package-owned `implementation_manifest_ref`; graph owns the closed component-class/host-capability vocabulary and canonical ten-field slot-template vector, while package owns compatibility/retention/hotplug/implementation/scope wires. This task does **not** consume the not-yet-created `VerifiedBackupCheckpointV1` or expose a production maintenance-authoring/execution entrypoint. It lands maintenance reservation/plan/execution/transition/operation/store contracts dormant. Task 11 later supplies backup cut/checkpoint; Task 14 alone composes live tenancy scope, admission hold, privileged executors and maintenance execution. The upper coordinator remains the sole ACK/OBSERVED authority.
- Produces: the reusable signed thirteen-field package root, eight-field implementation-manifest closure, signed 30-field structural maintenance-plan root, signed package-trust registry, nine-field pure desired-state item, sixteen-field decision, reservation/authorization-scope/execution-trust/execution-authorization/transition/operation/readback roots; schema/goldens/private verifiers; package-owned generated CMS implementations plus foundation-owned descriptors; a package-domain typed adapter to the already G1-owned global storage-manifest rotation coordinator; crash-safe plan finalization; atomic/drain forward **and rollback** participant execution; dormant reservation/maintenance projections; and the exact package execution-store protocol. The maintenance pure validator compiles now, but no code path can create execution authorization or privileged maintenance binding until Task 14 supplies Task-11 checkpoint/cut, live hold/barrier and upper-only dependencies.

- [ ] **Step 1: Write failing license/trust and pin tests.**

```rust
#[test]
fn valid_license_cannot_activate_untrusted_package() {
    let err = activate(valid_license(), package_with_revoked_signature()).unwrap_err();
    assert_eq!(err.code(), "PACKAGE_TRUST_REVOKED");
}

#[test]
fn unknown_effect_prevents_old_package_reclaim() {
    assert_eq!(reclaim(package_with_unknown_effect()).unwrap_err().code(), "PACKAGE_DURABLE_REFERENCE_ACTIVE");
}

#[test]
fn package_is_a_measured_participant_not_an_activation_authority() {
    assert_code(package_apply_with_ack_or_raw_generation(), "GENERATION_PARTICIPANT_REQUEST_REQUIRED");
    assert!(package_apply_response_contains_measurement_not_ack());
    assert!(!locked_cargo_metadata().has_path(
        "ep-platform-package", "ep-platform-generation-activation"));
    assert!(!locked_cargo_metadata().has_cycle());
}

#[test]
fn hotplug_grade_is_signed_graph_checked_and_crash_recoverable() {
    assert_atomic_grade_pins_inflight_requests_to_one_generation_then_pointer_swaps();
    assert_drain_grade_blocks_new_work_and_completes_or_durably_hands_off_every_old_work_item();
    assert_maintenance_contract_is_dormant_until_task14_privileged_composition();
    assert_crash_at_every_atomic_and_drain_intent_commit_and_adopt_cut_reconciles_one_attempt();
    assert_code(package_with_carrier_incompatible_hotplug_grade(), "PACKAGE_HOTPLUG_GRADE_INVALID");
    assert_code(drain_timeout_with_unresolved_work_declared_enabled(), "PACKAGE_DRAIN_INCOMPLETE");
}

#[test]
fn maintenance_plan_is_typed_dual_control_and_generation_exact() {
    assert_maintenance_plan_schema_and_rust_golden_are_byte_identical();
    assert_three_signed_roots_have_generated_stable_cms_descriptors();
    assert_plan_signer_is_customer_maintenance_authority_not_vendor();
    assert_desired_state_item_and_closed_action_matrix_are_exact();
    assert_typed_decision_role_order_scope_and_media_are_exact();
    assert_self_contained_finalization_store_recovers_each_decision_provider_spool_and_bind_cut();
    assert_no_public_api_can_construct_maintenance_binding_before_task14();
    assert_code(raw_or_cross_generation_plan(), "PACKAGE_MAINTENANCE_PLAN_INVALID");
    assert_code(second_attempt_for_same_plan(), "PACKAGE_MAINTENANCE_PLAN_ALREADY_BOUND");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_package_hotplug -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement three hotplug grades.**

```rust
pub struct CapabilityPackageIdV1(UuidV1);
pub struct CapabilityPackageVersionV1(String);
pub struct PlatformContractVersionV1(String);
pub enum CapabilityPackagePurposeV1 {
    #[serde(rename = "EP-F57-CAPABILITY-PACKAGE-V1")]
    CapabilityPackage,
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageCompatibilityV1 {
    pub minimum_platform_api_version: PlatformContractVersionV1,
    pub maximum_platform_api_version_exclusive: PlatformContractVersionV1,
    pub required_runtime_abi_sha256: Sha256Digest,
}
pub enum CapabilityPackageDataRetentionModeV1 {
    #[serde(rename = "RETAIN_ON_DISABLE_ROLLBACK_AND_LICENSE_LOSS")]
    RetainOnDisableRollbackAndLicenseLoss,
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageDataRetentionContractV1 {
    pub mode: CapabilityPackageDataRetentionModeV1,
    pub data_namespace_ids: Vec<String>,
    pub minimum_retention_days: u32,
    pub legal_hold_precedence: bool,
}
pub enum CapabilityPackageImplementationManifestPurposeV1 {
    #[serde(rename = "EP-F57-CAPABILITY-PACKAGE-IMPLEMENTATION-MANIFEST-V1")]
    CapabilityPackageImplementationManifest,
}
#[serde(tag = "artifact_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum CapabilityPackageImplementationArtifactV1 {
    DeclarativeBundle { artifact_ref: ArtifactRefV1, schema_ref: ArtifactRefV1, entrypoint_id: String },
    WasmModule { module_ref: ArtifactRefV1, wit_contract_ref: ArtifactRefV1, sbom_ref: ArtifactRefV1 },
    WindowsNativeBinary { binary_ref: ArtifactRefV1, authenticode_readback_ref: ArtifactRefV1, sbom_ref: ArtifactRefV1, abi_readback_ref: Option<ArtifactRefV1>, entrypoint_id: String },
    HyperVContainerImage { oci_archive_ref: ArtifactRefV1, image_manifest_ref: ArtifactRefV1, signature_bundle_ref: ArtifactRefV1, sbom_ref: ArtifactRefV1 },
    DatabaseMigrationBundle { migration_plan_ref: ArtifactRefV1, migration_bundle_ref: ArtifactRefV1, sbom_ref: ArtifactRefV1 },
    FoundationArtifactSet { artifact_manifest_ref: ArtifactRefV1, artifact_refs: Vec<ArtifactRefV1>, sbom_ref: ArtifactRefV1 },
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageImplementationManifestV1 {
    pub schema_version: u32,
    pub purpose: CapabilityPackageImplementationManifestPurposeV1,
    pub package_id: CapabilityPackageIdV1,
    pub package_version: CapabilityPackageVersionV1,
    pub component_class: CapabilityPackageComponentClassV1,
    pub artifacts: Vec<CapabilityPackageImplementationArtifactV1>,
    pub sbom_ref: ArtifactRefV1,
    pub implementation_set_sha256: Sha256Digest,
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackagePayloadV1 {
    pub schema_version: u32,
    pub purpose: CapabilityPackagePurposeV1,
    pub package_id: CapabilityPackageIdV1,
    pub package_version: CapabilityPackageVersionV1,
    pub component_class: CapabilityPackageComponentClassV1,
    pub capability_subgraph_ref: ArtifactRefV1,
    pub implementation_manifest_ref: ArtifactRefV1,
    pub permission_ceiling: PermissionCeilingV1,
    pub required_host_capabilities: Vec<CapabilityPackageHostCapabilityIdV1>,
    pub migration_plan_ref: Option<ArtifactRefV1>,
    pub compatibility: CapabilityPackageCompatibilityV1,
    pub data_retention_contract: CapabilityPackageDataRetentionContractV1,
    pub hotplug_contract: CapabilityPackageHotplugContractV1,
}
pub type CapabilityPackageArtifactV1 =
    SignedBusinessArtifactV1<CapabilityPackagePayloadV1>;

#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageDrainTransferStrategyV1 { CompleteInPlace, DurableHandoff }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageMaintenanceBackupPreconditionV1 { VerifiedRecoveryCheckpointRequired }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageLifecycleStateV1 { Absent, InstalledDisabled, Enabled }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageScopeModeV1 { LegalEntity, Deployment }
#[serde(tag = "scope_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum CapabilityPackageScopeV1 {
    LegalEntity {
        legal_entity_id: UuidV1,
        tenancy_generation: u64,
        key_domain_generation: u64,
        tenancy_authority_snapshot_ref: ArtifactRefV1,
    },
    Deployment {
        tenancy_generation: u64,
        impacted_legal_entity_ids: Vec<UuidV1>,
        impacted_legal_entity_set_sha256: Sha256Digest,
        tenancy_authority_snapshot_ref: ArtifactRefV1,
    },
}
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageMaintenanceExecutorKindV1 { WindowsControlBroker, RecoveryTool }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageRollbackStrategyV1 { ArtifactStateRestore, RecoveryCheckpointRestore }
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackagePrivilegedOperationKindV1 {
    PackageArtifactStateSwitch,
    PackageArtifactStateRestore,
    PackageRuntimeSlotSwitch,
    PackageRuntimeSlotRestore,
    PackageFoundationMaintenanceApply,
    PackageRecoveryCheckpointRestore,
}
pub enum PackageMaintenancePlanRequirementV1 {
    #[serde(rename = "SIGNED_MAINTENANCE_PLAN_REQUIRED")]
    SignedMaintenancePlanRequired,
}
#[serde(deny_unknown_fields)]
pub struct PackageMaintenanceProbeSetsV1 {
    pub install: Vec<String>,
    pub enable: Vec<String>,
    pub disable: Vec<String>,
    pub upgrade: Vec<String>,
    pub rollback: Vec<String>,
}
pub enum CapabilityPackageMaintenancePlanPurposeV1 {
    #[serde(rename = "EP-F57-CAPABILITY-PACKAGE-MAINTENANCE-PLAN-V1")]
    CapabilityPackageMaintenancePlan,
}
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageMaintenanceActionV1 { Install, Enable, Disable, Upgrade, Rollback }

#[serde(deny_unknown_fields)]
pub struct CapabilityPackageMaintenancePlanV1 {
    pub schema_version: u32,
    pub purpose: CapabilityPackageMaintenancePlanPurposeV1,
    pub plan_id: UuidV1,
    pub reservation_id: UuidV1,
    pub deployment_id: UuidV1,
    pub scope: CapabilityPackageScopeV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub participant_id: String,
    pub item_id: GenerationItemIdV1,
    pub action: CapabilityPackageMaintenanceActionV1,
    pub executor_kind: CapabilityPackageMaintenanceExecutorKindV1,
    pub rollback_strategy: CapabilityPackageRollbackStrategyV1,
    pub package_id: CapabilityPackageIdV1,
    pub source_package_ref: Option<ArtifactRefV1>,
    pub source_package_version: Option<CapabilityPackageVersionV1>,
    pub source_lifecycle_state: CapabilityPackageLifecycleStateV1,
    pub target_package_ref: Option<ArtifactRefV1>,
    pub target_package_version: Option<CapabilityPackageVersionV1>,
    pub target_lifecycle_state: CapabilityPackageLifecycleStateV1,
    pub approved_window_not_before_unix_ms: i64,
    pub approved_window_not_after_unix_ms: i64,
    pub recovery_checkpoint_policy_ref: ArtifactRefV1,
    pub required_probe_ids: Vec<String>,
    pub initiator_principal: PrincipalRefV1,
    pub approver_principal: PrincipalRefV1,
    pub authorization_decision_refs: Vec<ArtifactRefV1>,
    pub authorization_not_after_unix_ms: i64,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type SignedCapabilityPackageMaintenancePlanV1 =
    SignedBusinessArtifactV1<CapabilityPackageMaintenancePlanV1>;

pub enum CapabilityPackageGenerationItemPurposeV1 {
    #[serde(rename = "EP-F57-CAPABILITY-PACKAGE-GENERATION-ITEM-V1")]
    CapabilityPackageGenerationItem,
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageGenerationItemV1 {
    pub schema_version: u32,
    pub purpose: CapabilityPackageGenerationItemPurposeV1,
    pub item_id: GenerationItemIdV1,
    pub package_id: CapabilityPackageIdV1,
    pub desired_package_ref: ArtifactRefV1,
    pub desired_package_version: CapabilityPackageVersionV1,
    pub desired_lifecycle_state: CapabilityPackageLifecycleStateV1,
    pub package_trust_registry_ref: ArtifactRefV1,
    pub scope: CapabilityPackageScopeV1,
}

pub enum CapabilityPackageMaintenanceAuthorizationDecisionPurposeV1 {
    #[serde(rename = "EP-F57-CAPABILITY-PACKAGE-MAINTENANCE-AUTHORIZATION-DECISION-V1")]
    CapabilityPackageMaintenanceAuthorizationDecision,
}
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityPackageMaintenanceAuthorizationRoleV1 { Initiator, Approver }
pub enum CapabilityPackageMaintenanceAuthorizationOutcomeV1 {
    #[serde(rename = "ALLOW")]
    Allow,
}
#[serde(deny_unknown_fields)]
pub struct CapabilityPackageMaintenanceAuthorizationDecisionV1 {
    pub schema_version: u32,
    pub purpose: CapabilityPackageMaintenanceAuthorizationDecisionPurposeV1,
    pub decision_id: UuidV1,
    pub role: CapabilityPackageMaintenanceAuthorizationRoleV1,
    pub outcome: CapabilityPackageMaintenanceAuthorizationOutcomeV1,
    pub principal: PrincipalRefV1,
    pub deployment_id: UuidV1,
    pub scope: CapabilityPackageScopeV1,
    pub authority_epoch: u64,
    pub capability_id: CapabilityIdV1,
    pub decision_scope_sha256: Sha256Digest,
    pub policy_sha256: Sha256Digest,
    pub mfa_verified_at_unix_ms: i64,
    pub reauthenticated_at_unix_ms: i64,
    pub decided_at_unix_ms: i64,
    pub not_after_unix_ms: i64,
}

#[serde(tag = "hotplug_grade", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum CapabilityPackageHotplugContractV1 {
    AtomicGenerationSwap {
        post_switch_probe_ids: Vec<String>,
    },
    DrainAndReplace {
        drain_timeout_ms: u64,
        transfer_strategy: PackageDrainTransferStrategyV1,
        post_switch_probe_ids: Vec<String>,
    },
    MaintenanceUpgrade {
        maintenance_plan_requirement: PackageMaintenancePlanRequirementV1,
        maximum_approved_window_ms: u64,
        backup_precondition: PackageMaintenanceBackupPreconditionV1,
        post_action_probe_sets: PackageMaintenanceProbeSetsV1,
    },
}
```

The exact enum/purpose/state/action/media wires are the master values. `docs/evidence/f57-capability-package.v1.schema.json` direct-imports only foundation, generation, capability-graph and the Task-4 provider-permission schema. It solely owns the thirteen-field signed package payload, eight-field plain implementation manifest, 30-field signed structural plan, nine-field pure desired-state item, sixteen-field decision and the strict reservation, authorization-scope, execution-trust, execution-authorization, per-attempt transition, operation request/result and installed-state readback roots. Backup cut/checkpoint, tenancy snapshot and production hold are media-constrained refs only; the lower package crate does not import their later owners. The envelope is composed exactly once for each signed root and never around an operational readback. The implementation manifest's tagged artifact exact-set closes every executable/configuration byte, SBOM, schema/WIT, Authenticode/OCI proof, migration or foundation artifact; component class selects the sole legal artifact variant. Package/plan/registry `CmsSignableArtifactV1` implementations live only in package-generated code, while foundation generates stable descriptor/type-ID rows; handwritten registration, foreign impl, missing artifact or implementation bytes outside the manifest fail.

`docs/schemas/f57-capability-package-trust-registry.v1.schema.json` owns the strict signed seven-field registry and six-field rows under exact media `application/vnd.ep.f57-capability-package-trust-registry-v1+json`. Rows are canonical unique, wildcard-free and `CURRENT_AT_VERIFICATION`; package accepts product/approved-customer package authorities and a plan accepts only customer maintenance authority. The storage manifest pins the registry envelope digest and bootstrap signer SPKI; verification uses the authenticated deployment bootstrap, exact registry-authority DN, current chain/revocation/checkpoint and fixed source path. `CapabilityPackageTrustProviderV1` exposes four pairwise-disjoint roles—registry bootstrap, package, customer maintenance and recovery bootstrap—through exactly self-hosted nonexportable Windows CNG/TPM/PIV or approved-existing-enterprise adapters. Fixed container/SPKI/DN/ACL identities, approved-digest-only signing and durable `(signing_operation_id,authorization_digest)` query/adopt are mandatory. `package_trust.rs` is only the package-domain adapter to G1's already sole global `AuthorityStorageManifestRotationCoordinatorV1`; Task 7 modifies that composition to add the package tagged branch and four-event sequence, but creates no second lock, journal, manifest writer or revision allocator. Cross-domain generation/package crash/CAS tests prove one global monotonic storage-manifest history. The recovery-tool ceremony creates/imports, proves and installs credentials plus the mutually verified registry/manifest pair, then copies the registry to the portable authority-generation input. None enters the 89-row evidence registry.

License answers entitlement only; graph plus package trust answer compatibility, permission ceiling and hotplug. CapabilityGraph owns the closed 15-class vocabulary, host-capability registry and ten-field static slot templates with `scope_mode`; the upper authority later joins the current tenancy snapshot to create exact tagged `LEGAL_ENTITY|DEPLOYMENT` runtime scope. The minimum-grade table is `CONFIGURATION|UI|REPORT|RULE|WORKFLOW|MCP_CONFIGURATION -> ATOMIC_GENERATION_SWAP`, `WASM_EXTENSION|JOB_OBJECT_WORKER|HYPER_V_CONTAINER|CONNECTOR|AI_OCR_PROVIDER -> DRAIN_AND_REPLACE`, and `RUST_KERNEL|POSTGRESQL_DATABASE_MIGRATION|CRYPTOGRAPHY_FOUNDATION|STORAGE_FOUNDATION -> MAINTENANCE_UPGRADE`. An ordinary class may elect a stronger grade but receives only the master's executor/rollback/operation pair. All four global classes require `DEPLOYMENT` scope and allow production runtime only `UPGRADE|ROLLBACK`; their initial install is the signed release/recovery bootstrap. The eleven ordinary classes allow all five mutations. Exact mutation edges are `INSTALL ABSENT/null -> INSTALLED_DISABLED/target`, `ENABLE INSTALLED_DISABLED/same -> ENABLED/same`, `DISABLE ENABLED/same -> INSTALLED_DISABLED/same`, and same-lifecycle `UPGRADE|ROLLBACK` for either disabled or enabled state to a distinct higher/lower package. Deletion is not representable. Atomic pins each request to one generation; drain durably closes admission and accounts every item as complete-in-place or exact-once handed off. Maintenance remains compiled and dormant at G5.

Every signed-generation package ref points to the strict nine-field pure desired-state item `{schema_version,purpose,item_id,package_id,desired_package_ref,desired_package_version,desired_lifecycle_state,package_trust_registry_ref,scope}`. It contains no plan, decision, checkpoint, window or attempt. `VERIFY_UNCHANGED` and every mutation are represented by a separate create-new `CapabilityPackageGenerationTransitionV1` keyed by the full `(deployment,epoch,generation,manifest,activation_attempt,participant,item)` tuple. The transition binds source/target item, action, reverse plan and execution-trust snapshot; only maintenance binds reservation, historical plan, live execution authorization, actual checkpoint and admission hold. A transition becomes reachable only through the participant apply readback and fourteen-field ACK. Reverse plans restore the prior pure item or use `DEACTIVATE_RETAIN_DATA` for a newly introduced item, never revive expired authority.

`finalization.rs` and the file adapter implement the exact self-contained `PackageMaintenancePlanFinalizationRecordV1` under the DATA_HDD root and success grammar `INPUTS_FROZEN -> DECISIONS_STORED -> SIGNING_AUTHORIZATION_COMMITTED -> PROVIDER_COMMITTED -> PLAN_STORED -> PLAN_BOUND`. Sequence zero freezes the complete reservation, source readback, registry/source/target refs, both ordinal-1 decisions and unsigned 30-field plan containing `recovery_checkpoint_policy_ref`—never an actual checkpoint. `PLAN_BOUND` binds historical plan to reservation, not to the reusable desired item. Valid committed CMS bytes are adopted/stored even after the live window expires; `EXPIRED_UNBOUND` is legal only when provider query proves uncommitted, and UNKNOWN is never resent. Task 7 tests the journal with sealed fixtures but exposes no production maintenance authoring/execution constructor. Task 14 supplies current tenancy/security/hold/barrier and asks Task 11 for the actual full-cut checkpoint after drain.

Migration `V20261025092100` creates both dormant `platform_meta.package_maintenance_reservations` and the authoritative `platform_meta.package_activation_attempts` exactly as specified by the master. Execution rows use primary/CAS identity `(activation_attempt_id,participant_id,item_id,cas_version,row_sha256)`, participant-scoped nonnull plan uniqueness and `(rollback_execution_attempt_id,participant_id,item_id)` rollback uniqueness. A many-to-many item is therefore independently authorized, executed, measured and ACKed for each participant. Atomic/drain freeze the common execution-trust snapshot in the same CAS as first external intent; it exact-repeats deployment/epoch/generation/manifest/attempt/participant/item/scope/action and verified trust inputs. `VERIFY_UNCHANGED` alone uses a fresh readback with no external operation. Every mutation first create-new stores a strict operation request whose binding covers transition, trust, optional live authorization, implementation target and rollback identity; the broker accepts only private verified request, exposes `begin_or_adopt|query_exact`, and returns a strict result repeating the full participant tuple. Response loss never allocates another ID.

The forward state graph is `TUPLE_BOUND -> [DRAIN_INTENT_COMMITTED -> DRAIN_COMPLETED ->] SWITCH_INTENT_COMMITTED -> SWITCH_COMMITTED -> PROBING -> APPLIED_VERIFIED`, with measured `FORWARD_FAILED|UNKNOWN`; only the upper release-owned rollback request may continue any eligible state through `ROLLBACK_BOUND -> ROLLBACK_INTENT_COMMITTED -> ROLLBACK_COMMITTED -> PREDECESSOR_VERIFIED`. Installed-state readback nullability is exact. Generation apply readback uses only tagged `DESIRED_ITEM` rows. Rollback readback uses `DESIRED_ITEM` for a real predecessor or `DEACTIVATED_RETAIN_DATA` with an `ABSENT`/retained-data proof for a newly installed item; initial-generation rollback uses `NO_OBSERVED_GENERATION`, never a fabricated manifest ref. Set hashes include variant tags and all fields. `APPLIED_VERIFIED` is package-local measurement, never generation OBSERVED or production admission. Data, attachments, audit, recovery and rollback bytes remain pinned across disable, expiry, failure and UNKNOWN.

`package_store` is the sole SQL adapter and is composed through `AuthorizedPgTx`; package code owns no SQL. This task declares package in core-server/testkit, registers only atomic/drain participant execution and rejects package -> generation-activation, package -> backup and every dependency cycle. The handler receives only the release-owned private apply/rollback request, exact-loads desired item, registry, package, implementation, transition and trust snapshot, executes/reconciles one participant tuple, and returns typed operation plus installed-state readbacks. It never accepts a raw generation/plan/checkpoint, constructs an ACK, advances desired/OBSERVED, opens production admission or exposes privileged maintenance. Task 14 alone adds the acyclic upper-coordinator -> package/backup/tenancy composition. The handler implements exactly `GOV-004`, `GOV-005`, `PKG-001`, `PKG-002`, and `PKG-004`; `PKG-003` remains Task 4.

- [ ] **Step 4: Run Fresh PG and lifecycle matrix.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025092100`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_package_hotplug -- --nocapture`

Expected: PASS.

Run: `cargo test -p ep-foundation -p ep-platform-capability-graph -p ep-platform-license -p ep-platform-package -p ep-adapter-file -p ep-adapter-kms -p ep-adapter-db-pg -p recovery-tool -p core-server -p ep-testkit --all-targets --locked`

Expected: PASS with package store/wiring and atomic/drain forward/rollback execution reachable now; exact 13/8/30/9/16 schemas plus reservation/scope/trust/authorization/transition/operation/readback goldens; all three generated CMS descriptors; package-provider role separation and durable query/adopt; one G1-owned global trust-rotation lock/journal with cross-domain crash/CAS proof; closed graph class/grade/action/scope/implementation mapping; per-participant tuple isolation; exhaustive plan-finalization and operation crash cuts; `DEACTIVATED_RETAIN_DATA` plus generation-1 `NO_OBSERVED_GENERATION` readback; retained data and predecessor pins; no production maintenance binding/executor before Tasks 11/14; ACK/OBSERVED/admission writes unavailable; no package -> activation/backup edge or dependency cycle; and no lockfile drift.

- [ ] **Step 5: Commit package lifecycle.**

```bash
cargo xtask f57 task stage --task G5-07
cargo xtask f57 task verify-staged --task G5-07
git commit -m "feat: add signed capability package lifecycle"
```

### Task 8: Complete portals and release business breadth

**Files:**
- Create: `crates/features/portal-experience/Cargo.toml`
- Create: `crates/features/portal-experience/src/lib.rs`
- Create: `crates/features/portal-experience/src/public/mod.rs`
- Generate: `crates/features/portal-experience/src/public/generated.rs`
- Create: `crates/features/portal-experience/src/public/spi.rs`
- Create: `crates/features/portal-experience/src/domain/mod.rs`
- Create: `crates/features/portal-experience/src/domain/portal.rs`
- Create: `crates/features/portal-experience/src/application/mod.rs`
- Create: `crates/features/portal-experience/src/application/commands.rs`
- Create: `crates/features/portal-experience/tests/feature_boundary.rs`
- Create: `crates/features/portal-identity/Cargo.toml`
- Create: `crates/features/portal-identity/src/lib.rs`
- Create: `crates/features/portal-identity/src/public/mod.rs`
- Generate: `crates/features/portal-identity/src/public/generated.rs`
- Create: `crates/features/portal-identity/src/public/spi.rs`
- Create: `crates/features/portal-identity/src/domain/mod.rs`
- Create: `crates/features/portal-identity/src/domain/binding.rs`
- Create: `crates/features/portal-identity/src/application/mod.rs`
- Create: `crates/features/portal-identity/src/application/commands.rs`
- Create: `crates/features/portal-identity/tests/feature_boundary.rs`
- Create: `clients/portal/package.json`
- Create: `clients/portal/package-lock.json`
- Create: `clients/portal/tsconfig.json`
- Create: `clients/portal/vite.config.ts`
- Create: `clients/portal/src/App.tsx`
- Create: `clients/portal/src/api/portal.ts`
- Create: `clients/portal/src/generated/manifest-link.ts`
- Create: `clients/portal/tests/portal-contract.test.ts`
- Create: `crates/features/service-cycle/Cargo.toml`
- Create: `crates/features/service-cycle/src/lib.rs`
- Create: `crates/features/service-cycle/src/public/mod.rs`
- Generate: `crates/features/service-cycle/src/public/generated.rs`
- Create: `crates/features/service-cycle/src/public/spi.rs`
- Create: `crates/features/service-cycle/src/domain/mod.rs`
- Create: `crates/features/service-cycle/src/domain/service.rs`
- Create: `crates/features/service-cycle/src/application/mod.rs`
- Create: `crates/features/service-cycle/src/application/commands.rs`
- Create: `crates/features/service-cycle/tests/feature_boundary.rs`
- Create: `crates/features/project-cycle/Cargo.toml`
- Create: `crates/features/project-cycle/src/lib.rs`
- Create: `crates/features/project-cycle/src/public/mod.rs`
- Generate: `crates/features/project-cycle/src/public/generated.rs`
- Create: `crates/features/project-cycle/src/public/spi.rs`
- Create: `crates/features/project-cycle/src/domain/mod.rs`
- Create: `crates/features/project-cycle/src/domain/project.rs`
- Create: `crates/features/project-cycle/src/application/mod.rs`
- Create: `crates/features/project-cycle/src/application/commands.rs`
- Create: `crates/features/project-cycle/tests/feature_boundary.rs`
- Modify: `crates/features/customer-master/src/public/spi.rs`
- Regenerate: `crates/features/customer-master/src/public/generated.rs`
- Modify: `crates/features/customer-master/src/domain/customer.rs`
- Modify: `crates/features/customer-master/src/application/commands.rs`
- Create: `crates/features/crm/Cargo.toml`
- Create: `crates/features/crm/src/lib.rs`
- Create: `crates/features/crm/src/public/mod.rs`
- Generate: `crates/features/crm/src/public/generated.rs`
- Create: `crates/features/crm/src/public/spi.rs`
- Create: `crates/features/crm/src/domain/mod.rs`
- Create: `crates/features/crm/src/domain/customer_relationship.rs`
- Create: `crates/features/crm/src/application/mod.rs`
- Create: `crates/features/crm/src/application/commands.rs`
- Create: `crates/features/crm/tests/feature_boundary.rs`
- Create: `crates/features/cpq/Cargo.toml`
- Create: `crates/features/cpq/src/lib.rs`
- Create: `crates/features/cpq/src/public/mod.rs`
- Generate: `crates/features/cpq/src/public/generated.rs`
- Create: `crates/features/cpq/src/public/spi.rs`
- Create: `crates/features/cpq/src/domain/mod.rs`
- Create: `crates/features/cpq/src/domain/quote.rs`
- Create: `crates/features/cpq/src/application/mod.rs`
- Create: `crates/features/cpq/src/application/commands.rs`
- Create: `crates/features/cpq/tests/feature_boundary.rs`
- Modify: `crates/features/contracting/src/public/spi.rs`
- Modify: `crates/features/contracting/src/domain/contract.rs`
- Modify: `crates/features/contracting/src/application/commands.rs`
- Regenerate: `crates/features/contracting/src/public/generated.rs`
- Modify: `crates/features/sales-order/src/public/spi.rs`
- Modify: `crates/features/sales-order/src/domain/order.rs`
- Modify: `crates/features/sales-order/src/application/commands.rs`
- Regenerate: `crates/features/sales-order/src/public/generated.rs`
- Modify: `crates/features/procurement/src/public/spi.rs`
- Modify: `crates/features/procurement/src/domain/procurement.rs`
- Modify: `crates/features/procurement/src/application/commands.rs`
- Regenerate: `crates/features/procurement/src/public/generated.rs`
- Modify: `crates/features/inventory-fulfilment/src/public/spi.rs`
- Modify: `crates/features/inventory-fulfilment/src/domain/fulfilment.rs`
- Modify: `crates/features/inventory-fulfilment/src/application/commands.rs`
- Regenerate: `crates/features/inventory-fulfilment/src/public/generated.rs`
- Modify: `crates/features/sales-invoicing/src/public/spi.rs`
- Modify: `crates/features/sales-invoicing/src/domain/invoice.rs`
- Modify: `crates/features/sales-invoicing/src/application/commands.rs`
- Regenerate: `crates/features/sales-invoicing/src/public/generated.rs`
- Modify: `crates/features/receivable-cash/src/public/spi.rs`
- Modify: `crates/features/receivable-cash/src/domain/receivable.rs`
- Modify: `crates/features/receivable-cash/src/application/commands.rs`
- Regenerate: `crates/features/receivable-cash/src/public/generated.rs`
- Create: `crates/features/purchase-invoicing/Cargo.toml`
- Create: `crates/features/purchase-invoicing/src/lib.rs`
- Create: `crates/features/purchase-invoicing/src/public/mod.rs`
- Generate: `crates/features/purchase-invoicing/src/public/generated.rs`
- Create: `crates/features/purchase-invoicing/src/public/spi.rs`
- Create: `crates/features/purchase-invoicing/src/domain/mod.rs`
- Create: `crates/features/purchase-invoicing/src/domain/purchase_invoice.rs`
- Create: `crates/features/purchase-invoicing/src/application/mod.rs`
- Create: `crates/features/purchase-invoicing/src/application/commands.rs`
- Create: `crates/features/purchase-invoicing/tests/feature_boundary.rs`
- Create: `crates/features/payable-cash/Cargo.toml`
- Create: `crates/features/payable-cash/src/lib.rs`
- Create: `crates/features/payable-cash/src/public/mod.rs`
- Generate: `crates/features/payable-cash/src/public/generated.rs`
- Create: `crates/features/payable-cash/src/public/spi.rs`
- Create: `crates/features/payable-cash/src/domain/mod.rs`
- Create: `crates/features/payable-cash/src/domain/payable.rs`
- Create: `crates/features/payable-cash/src/application/mod.rs`
- Create: `crates/features/payable-cash/src/application/commands.rs`
- Create: `crates/features/payable-cash/tests/feature_boundary.rs`
- Create: `crates/features/operating-ledger/Cargo.toml`
- Create: `crates/features/operating-ledger/src/lib.rs`
- Create: `crates/features/operating-ledger/src/public/mod.rs`
- Generate: `crates/features/operating-ledger/src/public/generated.rs`
- Create: `crates/features/operating-ledger/src/public/spi.rs`
- Create: `crates/features/operating-ledger/src/domain/mod.rs`
- Create: `crates/features/operating-ledger/src/domain/ledger.rs`
- Create: `crates/features/operating-ledger/src/application/mod.rs`
- Create: `crates/features/operating-ledger/src/application/commands.rs`
- Create: `crates/features/operating-ledger/tests/feature_boundary.rs`
- Create: `crates/features/reporting/Cargo.toml`
- Create: `crates/features/reporting/src/lib.rs`
- Create: `crates/features/reporting/src/public/mod.rs`
- Generate: `crates/features/reporting/src/public/generated.rs`
- Create: `crates/features/reporting/src/public/spi.rs`
- Create: `crates/features/reporting/src/domain/mod.rs`
- Create: `crates/features/reporting/src/domain/report.rs`
- Create: `crates/features/reporting/src/application/mod.rs`
- Create: `crates/features/reporting/src/application/commands.rs`
- Create: `crates/features/reporting/tests/feature_boundary.rs`
- Create: `crates/adapter/db-pg/src/portal/mod.rs`
- Create: `crates/adapter/db-pg/src/portal/identity_repository.rs`
- Create: `crates/adapter/db-pg/src/portal/projection_repository.rs`
- Modify: `crates/adapter/db-pg/src/mdm/customer_repository.rs`
- Create: `crates/adapter/db-pg/src/crm/mod.rs`
- Create: `crates/adapter/db-pg/src/crm/crm_repository.rs`
- Create: `crates/adapter/db-pg/src/cpq/mod.rs`
- Create: `crates/adapter/db-pg/src/cpq/quote_repository.rs`
- Modify: `crates/adapter/db-pg/src/clm/contract_repository.rs`
- Modify: `crates/adapter/db-pg/src/sales/order_repository.rs`
- Modify: `crates/adapter/db-pg/src/procure/procurement_repository.rs`
- Modify: `crates/adapter/db-pg/src/inventory/fulfilment_repository.rs`
- Modify: `crates/adapter/db-pg/src/invoice/sales_invoice_repository.rs`
- Create: `crates/adapter/db-pg/src/invoice/purchase_invoice_repository.rs`
- Modify: `crates/adapter/db-pg/src/invoice/mod.rs`
- Modify: `crates/adapter/db-pg/src/finance/receivable_cash_repository.rs`
- Create: `crates/adapter/db-pg/src/finance/payable_cash_repository.rs`
- Modify: `crates/adapter/db-pg/src/finance/mod.rs`
- Create: `crates/adapter/db-pg/src/ledger/mod.rs`
- Create: `crates/adapter/db-pg/src/ledger/operating_ledger_repository.rs`
- Create: `crates/adapter/db-pg/src/service/mod.rs`
- Create: `crates/adapter/db-pg/src/service/service_repository.rs`
- Create: `crates/adapter/db-pg/src/project/mod.rs`
- Create: `crates/adapter/db-pg/src/project/project_repository.rs`
- Create: `crates/adapter/db-pg/src/reporting/mod.rs`
- Create: `crates/adapter/db-pg/src/reporting/report_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `apps/core-server/src/wiring/features.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/job-worker/src/wiring/features.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`
- Modify: `apps/job-worker/src/main.rs`
- Modify: `apps/job-worker/Cargo.toml`
- Create: `apps/portal-gateway/src/wiring/features.rs`
- Modify: `apps/portal-gateway/src/wiring/mod.rs`
- Create: `apps/portal-gateway/src/routes.rs`
- Modify: `apps/portal-gateway/src/main.rs`
- Modify: `apps/portal-gateway/Cargo.toml`
- Create: `db/migrations/portal/V20261025092200__portal_create_identity_and_customization.sql`
- Create: `db/migrations/mdm/V20261025092210__mdm_extend_release_breadth.sql`
- Create: `db/migrations/crm/V20261025092220__crm_create_release_breadth.sql`
- Create: `db/migrations/cpq/V20261025092230__cpq_create_release_breadth.sql`
- Create: `db/migrations/clm/V20261025092240__clm_extend_release_breadth.sql`
- Create: `db/migrations/sales/V20261025092300__sales_extend_release_breadth.sql`
- Create: `db/migrations/procure/V20261025092310__procure_extend_release_breadth.sql`
- Create: `db/migrations/inventory/V20261025092320__inventory_extend_release_breadth.sql`
- Create: `db/migrations/invoice/V20261025092330__invoice_extend_release_breadth.sql`
- Create: `db/migrations/finance/V20261025092340__finance_extend_release_breadth.sql`
- Create: `db/migrations/ledger/V20261025092350__ledger_create_operating_ledger_release_breadth.sql`
- Create: `db/migrations/service/V20261025092400__service_create_release_breadth.sql`
- Create: `db/migrations/project/V20261025092410__project_create_release_breadth.sql`
- Create: `db/migrations/reporting/V20261025092420__reporting_create_release_breadth.sql`
- Read generated: `testkit/tests/f57_customer_contract_order.rs`
- Read generated: `testkit/tests/f57_portal_customization.rs`
- Read generated: `testkit/tests/f57_procure_inventory_cash.rs`
- Read generated: `testkit/tests/f57_service_project_reporting.rs`
- Create: `testkit/src/f57_cases/g5/customer_contract_order.rs`
- Create: `testkit/src/f57_cases/g5/portal_customization.rs`
- Create: `testkit/src/f57_cases/g5/procure_inventory_cash.rs`
- Create: `testkit/src/f57_cases/g5/service_project_reporting.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `testkit/Cargo.toml`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: G1/G2 primitives, complete providers/packages/models, generated Portal API.
- Produces: complete master-data/CRM/CPQ/contract breadth; DROP_SHIP; six procurement sources; purchase invoice/AP/supplier payment; returns/corrections; service/project/reporting cycles; customer/supplier portal; and complete customization surfaces.

- [ ] **Step 1: Write failing business closure and XOR tests.**

```rust
#[tokio::test]
async fn procurement_waiting_gap_closes_only_after_all_three_owner_facts() {
    let run = g4_procurement_waiting_fixture().await;
    let run = record_purchase_invoice(run).await;
    let run = recognize_payable(run).await;
    let run = settle_supplier_payment(run).await;
    assert_eq!(run.procurement_gap(), ProcurementSettlementGapV1 {
        purchase_invoice_recorded: true,
        payable_recognized: true,
        supplier_payment_settled: true,
    });
    assert_eq!(approve_closure_by_distinct_reviewer(run).await.state, "CLOSED");
}

#[test]
fn standard_and_drop_ship_primary_families_are_xor() {
    assert_rejected(order_with_both_primary_families(), "SALES_PRIMARY_FULFILMENT_XOR");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_customer_contract_order --test f57_portal_customization --test f57_procure_inventory_cash --test f57_service_project_reporting -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement feature-owned breadth.**

Each feature exposes only `public`; every new owner has generator-owned `public/generated.rs`, authored `public/spi.rs`, private domain/application modules, and a db-pg repository registered in `crates/adapter/db-pg/src/lib.rs`. `identity` remains the sole credential/authenticator/device/session owner; `portal-identity` owns invitation, external-subject binding, legal-entity fence and revocation orchestration through `identity_repository`; `portal-experience` owns only allowlists and authorization-trimmed projections through `projection_repository`. They share the portal schema but never commands, facts, ports, or repository ownership. Complete `MDM-001..005` (versioned organization/customer/supplier/product/unit/warehouse/price authority, approval, merge, immutable historical snapshots), `CRM-001..004` (owner-safe 360 projection and opportunity/follow-up lifecycle; complaints are only registered as an intake channel/authorized projection and are handed by typed command to the service owner), `CPQ-001` (immutable quote versions and exact-once accepted conversion), `CLM-002..007` (templates/clauses/comments/signature evidence, approval/SoD, downstream obligations, amendment, renewal/expiry/merge, termination impact closure), and `REP-001..004` (registered formulas, lineage, authorization-safe drill-down and custom report/dashboard/print generations). No CRM or reporting table copies another owner's authoritative fact.

Also complete STANDARD/DROP_SHIP XOR, order/contract/project/inventory/manual/external-production procurement sources, purchase settlement, partial/return/reversal invariants, service complaint→work order→parts/time→root cause→follow-up→maintenance cycles, projects, metrics with drill-down evidence, and portal allowlists. `purchase-invoicing` is the sole owner of `PurchaseInvoiceFact`; `payable-cash` is the sole owner of payable recognition, supplier-payment allocation, and their reversals; `operating-ledger` is the sole owner of balanced internal operating entries, mappings, trial balance, subledger reconciliation, and permanently locked operating periods. `receivable-cash` additionally owns the enterprise cash/bank settlement-account master: the full identifier is represented only by an encrypted non-exportable handle plus blind reference, ordinary/list projections expose only `last4`, enable/disable and version changes are append-only high-risk dual-control commands, and a permitted full-value read requires a current scoped field grant plus reauthentication and is never logged. `payable-cash` references that account ID but cannot copy or write its master. `procurement` may request/observe settlement facts but cannot write them, `receivable-cash` remains customer-side for transaction facts, and neither invoice/cash owner may write ledger tables. `PROCUREMENT_FULFILMENT` closes only after the three distinct owner facts are present and a different authorized reviewer approves closure. Reversing invoice/payment/supplier payment creates a new cycle and reopens; it never edits the closed cycle. Core-server composes command repositories; job-worker composes only durable objective/effect consumers; portal-gateway remains a zero-database upstream and exposes the generated Portal API only. Every breadth test enters through Control/Employee/Portal HTTPS, reaches `CommandPipeline` + `AuthorizedPgTx`, and reads back through a public query/projection—direct handler/repository calls cannot satisfy a Requirement.

Handler exact sets are frozen: `customer_contract_order.rs` owns the 19 G5 rows in its canonical target (all except G4 `CLM-001`/`SAL-002`); `portal_customization.rs` owns its 8 rows; `procure_inventory_cash.rs` owns its 36 rows; and `service_project_reporting.rs` owns its 19 rows. Every handler asserts the named Requirement semantics and its evidence schema; none is a target-wide umbrella pass.

Within `procure_inventory_cash.rs`, `FIN-011` and `FIN-013` must enter through public commands and the independent `operating-ledger` repository and prove: every operating entry is balanced; posted facts are immutable and corrected only by linked reversal/correction entries; trial balance exact-equals the entry set; every subledger reconciliation carries source-fact lineage; locking an operating period is permanent and no command/admin path can reopen it; a late fact posts only to the next open period while retaining original business date, deferral basis, source fact, and correction chain. A finance/invoice table, direct repository call, unbalanced fixture, reopened period, or copied subledger fact cannot satisfy either handler.

The same handler's `FIN-001` case exact-asserts that account plaintext is absent from PostgreSQL clear columns, audit/Outbox/log/error/telemetry payloads and ordinary exports; list/query responses return only `last4`; unauthorized, stale-reauth, wrong-entity, wrong-purpose, or single-actor high-risk changes are rejected with zero writes; only a current reauthenticated field grant can request the full-value decrypt projection. A UI mask over a stored plaintext value is a hard failure.

- [ ] **Step 4: Generate, Fresh PG, and execute breadth suites.**

Run: `cargo xtask f57 graph generate`

Expected: all touched nodes become activation-ready and generated projections share one digest.

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025092420`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_customer_contract_order --test f57_portal_customization --test f57_procure_inventory_cash --test f57_service_project_reporting -- --nocapture`

Run: `cargo test -p core-server -p job-worker -p portal-gateway -p ep-adapter-db-pg --all-targets`

Run: `npm --prefix clients/portal ci && npm --prefix clients/portal test -- --run && npm --prefix clients/portal run build`

Expected: PASS with all 14 migration families exercised through their registered stores, including the independent ledger owner, all new modules compiled into their composition roots, portal-gateway holding zero database credentials, and no direct feature SQL.

- [ ] **Step 5: Commit release business breadth.**

```bash
cargo xtask f57 task stage --task G5-08
cargo xtask f57 task verify-staged --task G5-08
git commit -m "feat: complete portal and business release breadth"
```

### Task 9: Aggregate INTEGRATION_GREEN

**Files:**
- Create: `xtask/src/f57/g5.rs`
- Create: `testkit/tests/f57_integration_profile.rs`
- Create: `testkit/src/f57_cases/probes/g5_integration.rs`
- Create: `testkit/tests/f57_slice_probes_g5_integration.rs`
- Read: `docs/evidence/f57-integration-candidate.schema.json`
- Read: `docs/evidence/f57-l2-candidate-evidence.schema.json`
- Read: `docs/evidence/f57-gate-receipt.v1.schema.json`
- Read: `xtask/tests/fixtures/f57-gate-receipt-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-integration-candidate-integration-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-l2-integration-v1-golden.json`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `scripts/windows/run-l2-candidate.ps1`
- Modify/regenerate after final edit: `scripts/windows/trust/F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after descriptor verification: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Modify: `.github/workflows/ci.yml`

**Interfaces:**
- Consumes: every G5 branch receipt and all `first_due_profile<=G5_INTEGRATION` rows.
- Produces: a new integration candidate and `INTEGRATION_GREEN`; this is not the final signed release candidate.

Because this task edits `run-l2-candidate.ps1`, its final implementation step re-signs/timestamps the new bytes and regenerates the same-ID descriptor before the shown Windows run. The fixed trust executor rejects the earlier G4/Task-1 descriptor and any direct PowerShell invocation.

- [ ] **Step 1: Write failing branch-join tests.**

```rust
#[test]
fn integration_gate_rejects_missing_selected_client_branch_or_due_row() {
    assert_eq!(join(g5_receipts_without_client()).unwrap_err().code(), "G5_CLIENT_BRANCH_MISSING");
    assert_eq!(join(g5_receipts_with_missing_due_row()).unwrap_err().code(), "F57_DUE_REQUIREMENT_MISSING");
}

#[test]
fn integration_gate_rejects_client_packages_from_the_earlier_task2_tree() {
    let input = g5_input_with_task2_client_artifacts_and_current_server_head();
    assert_eq!(join(input).unwrap_err().code(), "G5_CLIENT_ARTIFACT_CANDIDATE_MISMATCH");
}

#[test]
fn integration_candidate_owns_one_bound_g5_fresh_pg_receipt() {
    let candidate = build_integration_candidate(current_g5_input()).unwrap();
    assert!(candidate.fresh_pg_receipt().typed_loads_profile_through(
        DeliveryProfileV1::G5Integration,
        20261025092420,
    ));
    assert!(all_g0_through_g5_receipts_reference_exact_same_fresh_pg_bytes(&candidate));
    assert_code(candidate_with_second_fresh_pg_receipt(), "CANDIDATE_FRESH_PG_REF_MISMATCH");
}

#[test]
fn fallback_candidate_cannot_reuse_tauri_g3_or_g4_conformance() {
    let input = flutter_selected_input_with_tauri_g3_g4_results();
    assert_eq!(join(input).unwrap_err().code(), "CLIENT_CONFORMANCE_NON_SELECTED_STACK");
    assert_code(
        flutter_selected_input_missing_g4_ui_api(),
        "CLIENT_CONFORMANCE_SET_INCOMPLETE",
    );
}

#[test]
fn g5_handler_exact_set_is_126() {
    let due = real_registry().first_due(DeliveryProfileV1::G5Integration);
    assert_eq!(due.len(), 126);
    assert_eq!(concrete_handler_ids_for(&due).len(), 126);
}

#[test]
fn integration_candidate_and_l2_match_the_g4_owned_schemas() {
    assert_byte_golden::<SignedIntegrationCandidateV1>(
        "xtask/tests/fixtures/f57-integration-candidate-integration-v1-golden.json",
    );
    assert_byte_golden::<L2CandidateEvidenceV1>(
        "xtask/tests/fixtures/f57-l2-integration-v1-golden.json",
    );
    assert_no_local_schema_copy_for_integration_or_l2();
}

#[test]
fn g5_candidate_and_receipt_use_frozen_finalization_and_materialized_registry() {
    let candidate = build_integration_candidate(current_g5_input()).unwrap();
    assert_eq!(candidate.payload.data_classification, CandidateDataClassificationV1::Deidentified);
    assert_candidate_registry_ref_exact_same_run_materialized_registry(&candidate);
    assert_candidate_id_and_checkpoint_equal_candidate_manifest_finalization_record(&candidate);
    assert_code(candidate_with_synthetic_classification(), "INTEGRATION_CANDIDATE_DATA_CLASSIFICATION_MISMATCH");
    assert_code(candidate_with_live_finalization_id(), "CANDIDATE_FINALIZATION_MISMATCH");
    assert_code(candidate_with_latest_checkpoint_instead_of_frozen(), "CANDIDATE_FINALIZATION_CHECKPOINT_MISMATCH");
    let receipt = issue_integration_green(current_g5_aggregate()).unwrap();
    assert_gate_receipt_registry_ref_exact_same_run_materialized_registry(&receipt);
    assert_gate_receipt_checkpoint_and_times_equal_frozen_finalization_record(&receipt);
    assert_code(g5_with_spoofed_caller_candidate_manifest_ref(), "G5_CANDIDATE_MANIFEST_REF_MISMATCH");
    assert_code(g5_with_spoofed_caller_identity_hash(), "G5_CANDIDATE_IDENTITY_MISMATCH");
    assert_code(g5_with_spoofed_caller_gate_run_id(), "G5_CANDIDATE_RUN_MISMATCH");
    assert_code(g5_with_spoofed_caller_fresh_pg_ref(), "G5_CANDIDATE_FRESH_PG_REF_MISMATCH");
}

#[test]
fn g5_closes_exact_four_objectives_with_governed_procurement_facts() {
    let l2 = integration_l2();
    let receipt = integration_green_receipt();
    assert_eq!(l2.payload.objective_closures, receipt.payload.objective_closures);
    assert_exact_four_objective_kinds_all_closed_and_no_open_obligations(&receipt.payload.objective_closures);
    assert_procurement_fact_kinds_and_owners_exact(
        &receipt.payload.objective_closures,
        [
            (ProcurementClosureFactKindV1::PurchaseInvoice, FeatureOwnerIdV1::PurchaseInvoicing),
            (ProcurementClosureFactKindV1::AccountsPayableRecognition, FeatureOwnerIdV1::PayableCash),
            (ProcurementClosureFactKindV1::SupplierPaymentAllocation, FeatureOwnerIdV1::PayableCash),
        ],
    );
    assert_procurement_requester_and_reviewer_nonzero_distinct_and_same_generation();
    assert_all_closure_refs_are_same_run_authorized_results_and_frozen_for_checkpoint_and_expiry();
    assert_code(g5_with_waiting_objective(), "G5_OBJECTIVE_NOT_CLOSED");
    assert_code(g5_with_reopened_objective(), "G5_OBJECTIVE_REOPENED");
    assert_code(g5_with_stale_objective_generation(), "G5_OBJECTIVE_GENERATION_STALE");
    assert_code(g5_with_fact_owner_permutation(), "G5_PROCUREMENT_FACT_OWNER_MISMATCH");
    assert_code(g5_with_missing_or_reversed_procurement_fact(), "G5_PROCUREMENT_FACT_SET_INVALID");
    assert_code(g5_with_same_requester_and_reviewer(), "G5_OBJECTIVE_REVIEW_NOT_DISTINCT");
    assert_code(g5_with_closure_result_outside_authorized_result_set(), "G5_OBJECTIVE_RESULT_UNAUTHORIZED");
}

#[test]
fn g4_objective_snapshot_is_selected_by_verified_candidate_purpose() {
    assert_development_slice_g4_has_three_closed_plus_procurement_waiting_gap();
    let integration_l2 = integration_l2();
    let fresh_g4 = fresh_g4_receipt_for_integration_candidate();
    assert_eq!(fresh_g4.payload.objective_closures, integration_l2.payload.objective_closures);
    assert_exact_four_objective_kinds_all_closed_and_no_open_obligations(&fresh_g4.payload.objective_closures);
    assert_g4_due_and_probe_sets_unchanged_by_closure_context(&fresh_g4);
    assert_gate_receipt_golden_covers_development_waiting_and_integration_closed_contexts();
    assert_code(reused_development_g4_receipt_in_g5(), "G5_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(integration_g4_with_waiting_procurement(), "G5_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(development_g4_with_closed_procurement(), "G4_OBJECTIVE_CONTEXT_MISMATCH");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_integration_profile -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement exact branch/due aggregation.**

```rust
pub fn issue_integration_green(input: G5AggregateInputV1) -> Result<GateReceiptV1, GateErrorV1> {
    let candidate = input.typed_load_verified_signed_candidate_for_g5_context()?;
    require_g5_candidate_context_exact(&candidate, input.aggregate_context)?;
    let candidate_manifest_ref = candidate.exact_signed_envelope_ref();
    let candidate_identity_sha256 = sha256_jcs(candidate.identity());
    let gate_run_id = candidate.gate_run_id();
    let fresh_pg_receipt = candidate.fresh_pg_receipt().clone();
    require_candidate_matches_journal_receipts_and_l2(
        &candidate,
        &input.verified_run_journal,
        &input.receipts,
        &input.verified_l2_evidence,
    )?;
    require_one_client_branch(&input)?;
    require_same_candidate_and_graph(&input.receipts)?;
    require_selected_client_conformance_exact_set(&input, [
        ClientConformanceIdV1::G3Shell,
        ClientConformanceIdV1::G4CtcUiApi,
        ClientConformanceIdV1::G5FourPlatform,
    ])?;
    require_all_first_due_rows_pass(&input.delivery_registry, DeliveryProfileV1::G5Integration)?;
    require_all_deferred_capabilities_disabled(&input)?;
    let due_results = exact_current_profile_due_results(&input, DeliveryProfileV1::G5Integration)?;
    let probe_results = exact_current_profile_probe_results(&input, DeliveryProfileV1::G5Integration)?;
    let objective_closures = current_authorized_objective_closures(&candidate, &input)?;
    require_exact_g5_four_closed_objectives(&objective_closures)?;
    require_procurement_fact_owner_and_distinct_reviewer_closure(&objective_closures)?;
    require_objective_closures_byte_equal_integration_l2(&objective_closures, &input.verified_l2_evidence)?;
    require_fresh_g4_receipt_closure_matches_verified_candidate_purpose(
        &input.fresh_g4_receipt,
        &candidate,
        &objective_closures,
    )?;
    let finalization = input.verified_evidence_finalization(EvidenceEnvelopeKindV1::GateReceiptG5)?;
    require_exact_evidence_finalization_kind(finalization, "GATE_RECEIPT_G5")?;
    require_finalization_freezes_objective_inputs_and_expiry(finalization, &objective_closures)?;
    let artifact_signer_registry_ref = input.verified_same_run_materialized_signer_registry.artifact_ref().clone();
    require_registry_selected_signer_row(
        &input.verified_same_run_materialized_signer_registry,
        "GATE_RECEIPT_V1",
        "INTEGRATION_GREEN",
        input.gate_signer.identity(),
    )?;
    let payload = GateReceiptPayloadV1 {
        schema_version: 1,
        purpose: GateReceiptPurposeV1::GateReceipt,
        gate: ProgramGateV1::IntegrationGreen,
        evidence_class: GateEvidenceClassV1::Integration,
        candidate_binding: GateCandidateBindingV1::SignedCandidate {
            candidate_manifest_ref,
            candidate_identity_sha256,
        },
        gate_run_id,
        prerequisite_receipts: canonical_prerequisite_refs(&input)?,
        delivery_registry_sha256: input.delivery_registry.sha256(),
        artifact_signer_registry_ref,
        first_due_map_sha256: input.delivery_registry.first_due_map_sha256(),
        due_result_set_sha256: canonical_due_result_set_sha256(&due_results)?,
        probe_result_set_sha256: canonical_probe_result_set_sha256(&probe_results)?,
        test_results: canonical_test_result_refs(&due_results)?,
        probe_results: canonical_probe_result_refs(&probe_results)?,
        objective_closures,
        run_journal_checkpoint_ref: finalization.frozen_input_checkpoint_ref.clone(),
        fresh_pg_receipt,
        issued_at_unix_ms: finalization.issued_at_unix_ms,
        expires_at_unix_ms: finalization.expires_at_unix_ms,
    };
    Ok(sign_business_artifact_v1(input.gate_signer, payload)?)
}
```

The join also proves the per-task handler partition totals exactly 126: client gate 1, selected Workbench 8, support/DLP 2, provider/AI 13, office/identity/platform 15, package 5, and business/portal 82. It recomputes both the delivery-registry digest and the frozen `first_due_map_sha256=a9547557f95a3a9892efa9f6751a0dd03accac65da344aa559a3203488fee086` from the same typed registry and exact-matches both to every current-run prerequisite receipt. Separately, it exact-joins the three stack-neutral conformance selectors from the generated manifest. The auxiliary `G5_FOUR_PLATFORM` result may come from `client-build` only when it was produced earlier in this run over the exact four validated package refs; after candidate binding, L2 starts the canonical `T-F57-CLI-007` handler once and consumes that auxiliary result plus the frozen artifact set. `G3_SHELL` and `G4_CTC_UI_API` dispatch their auxiliary TestIDs once against the just-built current authority and selected current client stack. This receipt contains exactly 126 current-profile `test_results` and exactly three current-profile `probe_results`; both vectors and their digests exact-match terminal journal records. Its `artifact_signer_registry_ref` comes from the one same-run materialized registry and the independent signer lookup `GATE_RECEIPT_V1/INTEGRATION_GREEN`; checkpoint, issue time and expiry come only from `verified_evidence_finalization(EvidenceEnvelopeKindV1::GateReceiptG5)` whose event wire is `GATE_RECEIPT_G5`, never from that signer key, a live clock or latest mutable checkpoint. Duplicate ownership, a target-wide umbrella result, a pre-candidate CLI-007 result, an earlier-tree result, a mismatched registry/map digest, an expiry extension, event/signer naming-domain confusion, finalization/registry drift, or a result from the preserved rejected stack fails even if the count happens to match.

`issue_integration_green` never accepts candidate identity as trusted caller scalars. It first typed-loads the explicitly supplied signed candidate as either standalone `SignedIntegrationCandidateV1{purpose=INTEGRATION}` or, only inside the final aggregate context, `ReleaseCandidateV1`; from those verified bytes it derives the exact manifest ref, `sha256(JCS(identity))`, run ID, Fresh-PG ref and signer-registry ref. Those values must exact-match the journal manifest-bound event, prerequisite receipts, L2 evidence and materialized registry before the G5 receipt is constructed. A caller-spoofed manifest ref, identity hash, run ID, Fresh-PG ref, candidate envelope type/purpose or final/standalone context fails before result aggregation.

Integration L2 and the G5 receipt carry the same canonical `Vec<ObjectiveClosureBindingV1>` sorted uniquely by `objective_kind`: exactly `CONTRACT_FULFILMENT`, `SALES_ORDER_FULFILMENT`, `PROCUREMENT_FULFILMENT`, and `RECEIVABLE_COLLECTION`, all `CLOSED`, positive current generation and `open_obligation_ids=[]`. State result and every `(evidence_id,result)` pair come from current authorized public queries and same-candidate signed `RequirementEvidenceBindingV1` rows in the aggregate's authorized result closure; raw database rows, caller status or an outside result ref cannot construct a binding.

Closed procurement has exactly the sorted fact set `{PURCHASE_INVOICE -> purchase-invoicing, ACCOUNTS_PAYABLE_RECOGNITION -> payable-cash, SUPPLIER_PAYMENT_ALLOCATION -> payable-cash}` with three distinct signed result refs. Its review is `DISTINCT_REVIEWER`; requester/reviewer digests are nonzero and unequal, and the authorized review result accepts the same objective generation after all facts. Other objective rows follow the graph's exact review rule. `WAITING`, `REOPENED`, stale generation, missing/reversed/owner-swapped fact, same reviewer, spoofed state/result or expired closure input fails G5. The receipt finalization checkpoint and expiry minimum include this complete typed closure and all reachable time-bearing inputs.

Candidate purpose controls the G4 snapshot without changing G4's due/probe ownership. Only a standalone verified `SignedIntegrationCandidateV1{purpose=DEVELOPMENT_SLICE}` permits the original three-CLOSED plus `PROCUREMENT_FULFILMENT=WAITING`, `open_obligation_ids=[PURCHASE_AP_CLOSED]`, zero procurement facts/review and typed false `ProcurementSettlementGapV1`. A freshly issued G4 receipt inside the current `INTEGRATION` aggregate must instead byte-equal Integration L2's four-CLOSED snapshot, including procurement facts/reviewer. Reusing the Development G4 receipt, carrying WAITING into G5, or marking procurement CLOSED in the Development context fails the canonical gate-receipt schema/golden branch.

- [ ] **Step 4: Run pre-commit Fresh PG, integration tests, and L1.**

Run: `cargo xtask f57 fresh-pg --profile G5_INTEGRATION --through 20261025092420`

Expected: PASS through the unchanged 69-file baseline plus the complete 43-file G5 F57 suffix (`112` total) on a clean PostgreSQL 16 database.

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Expected: PASS.

Run: `cargo test -p ep-testkit --test f57_integration_profile -- --nocapture`

Expected: PASS. No candidate or gate receipt is issued from the dirty worktree.

- [ ] **Step 5: Commit G5 aggregation.**

```bash
cargo xtask f57 task stage --task G5-09
cargo xtask f57 task verify-staged --task G5-09
git commit -m "test: certify f57 integration profile"
```

- [ ] **Step 6: Build L2 and issue G5 only from clean committed HEAD.**

Run: `git status --porcelain=v1`

Expected: no output.

Run on Windows through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_RUN_L2_CANDIDATE_V1 -- -TargetGate INTEGRATION_GREEN -BundleRoot target/f57/evidence -RunJournal target/f57/evidence/g5/gate-run.jcs.jsonl`.

Before that script builds the candidate it must run these two clean-HEAD operations:

Run: `cargo xtask f57 client-gate validate-selected --selection-receipt docs/decisions/f57-client-stack-decision.v1.json --candidate HEAD --integration --bundle-root target/f57/evidence --run-journal target/f57/evidence/g5/gate-run.jcs.jsonl --out target/f57/evidence/g5/client-stack-validation.v1.json`

Run: `cargo xtask f57 client-build --validation target/f57/evidence/g5/client-stack-validation.v1.json --candidate HEAD --bundle-root target/f57/evidence --run-journal target/f57/evidence/g5/gate-run.jcs.jsonl --out target/f57/evidence/g5/client-artifacts.v1.json`

Expected: before current validation, `validate-selected` fixed-loads the committed decision and `docs/decisions/f57-client-stack-decision-archive/`, exact-matches both graph-bound digests and selected value, replays the signed journal plus pre-BOUND immutable trust closure with one timestamp, both role chains and every role/index archived revocation pair wholly offline, and materializes the required selection proof into the new bundle; the deleted original target, global archive, network and ambient trust cache are never read. Validation may only PASS/FAIL for that already selected stack and `INTEGRATION` mode; it atomically creates or resumes the signed journal header before exposing one unpredictable `gate_run_id`, and PASS names the exact four built, integration-signed, installed/upgraded and measured package byte refs. `client-build` does not rebuild or resign them: it exact-wraps those refs as four client lanes, runs the same-run auxiliary `G5_FOUR_PLATFORM` conformance over them, and emits the artifact set. The L2 script then calls `candidate build --candidate HEAD --client-artifacts target/f57/evidence/g5/client-artifacts.v1.json --bundle-root target/f57/evidence --run-journal target/f57/evidence/g5/gate-run.jcs.jsonl --out target/f57/evidence/g5/integration-candidate.v1.json`; `candidate build` adopts that run, materializes the canonical signer registry for the same run, independently builds/hashes the current-HEAD authority, invokes the G0-owned internal CandidateBound Fresh-PG once for `G5_INTEGRATION` through `20261025092420`, and create-new binds that envelope. It freezes the complete precursor prefix in `CANDIDATE_MANIFEST_FINALIZATION_STARTED`, then constructs `SignedIntegrationCandidateV1` only from that record's unpredictable `finalization_attempt_id` and frozen checkpoint, the same-run `artifact_signer_registry_ref`, fixed `data_classification=DEIDENTIFIED`, and canonical five-artifact set `{android-client,ios-client,macos-client,windows-authority,windows-client}` before appending its manifest-bound event. No caller or live clock supplies classification, registry, ID, checkpoint or time. Every freshly emitted G0…G5 receipt reuses the candidate's byte-identical Fresh-PG ref; gate does not rerun it. It hashes those exact signed manifest bytes and runs `cargo xtask f57 verify --level l2 --candidate <candidate-manifest-sha256> --candidate-manifest target/f57/evidence/g5/integration-candidate.v1.json --bundle-root target/f57/evidence --run-journal target/f57/evidence/g5/gate-run.jcs.jsonl --out target/f57/evidence/g5/l2-evidence.v1.json`; L2 dispatches auxiliary conformance TestIDs for selectors `G3_SHELL` and `G4_CTC_UI_API` only when absent, exact-joins the artifact set's terminal auxiliary `G5_FOUR_PLATFORM`, and then first starts candidate-bound `T-F57-CLI-007`. Then `gate g5 --candidate-manifest target/f57/evidence/g5/integration-candidate.v1.json --l2-evidence target/f57/evidence/g5/l2-evidence.v1.json --bundle-root target/f57/evidence --run-journal target/f57/evidence/g5/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g5` typed-joins terminal results, starts only absent G0…G5 TestIDs, freezes its receipt finalization, and writes `target/f57/evidence/g5/integration-receipt.v1.json` with that record's checkpoint/times plus the same materialized registry ref, without modifying the repository. Every checkpoint is a strict prefix extension. Crash recovery adopts exact header/Fresh-PG/output bytes without new IDs, database rerun or re-signing. Empty/duplicate/missing-authority artifacts, wrong classification, validation/artifact byte drift, alternate registry/Fresh-PG ref, live/latest finalization fields, pre-candidate CLI-007, duplicate TestID execution, Task 2 artifacts, a G4-tree receipt, non-selected/rejected-stack conformance, changed/missing run ID, conflicting output, path escape, archive/digest/trust drift, or any mismatched graph/toolchain/client manifest are rejected rather than relabeled.

### Task 10: Freeze the native Windows authority carrier, manifest, and fencing contract

**Files:**
- Create: `installer/windows/Product.wxs`
- Create: `installer/windows/Services.wxs`
- Regenerate: `installer/windows/generated/AuthorityServices.wxi`
- Create: `docs/evidence/f57-release-carrier-common.v1.schema.json`
- Create: `docs/evidence/f57-windows-authority-manifest.v1.schema.json`
- Create: `docs/evidence/f57-windows-runtime-deployment.v1.schema.json`
- Create: `xtask/tests/fixtures/f57-release-carrier-common-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-windows-authority-manifest-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-runtime-deployment-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-authority-static-build-receipt-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-service-install-plan-v1-golden.json`
- Create: `scripts/windows/build-msi.ps1`
- Create: `scripts/windows/install-services.ps1`
- Create: `scripts/windows/verify-service-acls.ps1`
- Create: `scripts/windows/verify-ipc.ps1`
- Create: `scripts/windows/verify-hdd-routing.ps1`
- Create: `scripts/windows/verify-time.ps1`
- Create after final signing: `scripts/windows/trust/F57_PS_BUILD_MSI_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_INSTALL_SERVICES_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_SERVICE_ACLS_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_IPC_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_HDD_ROUTING_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_TIME_V1.authenticode.json`
- Regenerate after all six descriptor verifications: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after all six descriptor verifications: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after all six descriptor verifications: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Create: `crates/platform/runtime/src/release_carrier_common.rs`
- Create: `crates/platform/runtime/src/windows/authority_manifest.rs`
- Create: `crates/platform/runtime/src/windows/deployment.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Read: `crates/platform/runtime/src/topology.rs`
- Read: `crates/platform/runtime/src/evidence/object_store.rs`
- Read: `crates/adapter/file/src/evidence_object_store.rs`
- Read: `crates/platform/runtime/src/storage/manifest.rs`
- Read: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Read: `docs/schemas/f57-authority-storage-manifest.v1.schema.json`
- Read: `docs/generated/f57/policy/p340-certification-policy.v1.json`
- Create: `crates/adapter/ipc/src/windows_pipe.rs`
- Modify: `crates/adapter/ipc/src/lib.rs`
- Modify: `crates/adapter/ipc/Cargo.toml`
- Modify: `crates/platform/runtime/Cargo.toml`
- Modify: `crates/platform/runtime/src/windows/mod.rs`
- Create: `crates/platform/runtime/src/windows/job.rs`
- Create: `crates/platform/runtime/src/windows/service.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/authority_service_fence_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `apps/core-server/src/wiring/authority_fence.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `db/migrations/platform_ops/V20261025092500__platform_ops_create_authority_service_fences.sql`
- Create: `testkit/tests/f57_windows_ipc.rs`
- Create: `testkit/tests/f57_windows_time.rs`
- Create: `testkit/tests/f57_authority_fencing.rs`
- Create: `testkit/tests/f57_windows_runtime_deployment.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `xtask/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: the G0 CapabilityGraph and generated projection manifest, the runtime topology/participant wire contract, the verified signed authority-storage manifest, the non-expiring P340 policy-definition ref, and Windows credentials. A caller-authored process list, signed/monolithic topology manifest or pre-P340 runtime-topology certification is not an input.
- Produces: one strict `WindowsAuthorityManifestV1`, one deterministic MSI/static-service-table projection for the manifest's exact authority binary and exact five eventual permanent `AUTO_START` services, the runtime-owned carrier-polymorphic `WindowsRuntimeDeploymentClosureV1` contract, trusted-time and fencing primitives, and compile-time/readback test fixtures. Task 10 deliberately does **not** build deployment participants, materialize a final closure, install, start, restart, or dynamically inspect services: Task 14 lands the final builders/collectors and every Authority ServiceMain/broker mode, while Task 15's ordered `WINDOWS_AUTHORITY_BUILD -> WINDOWS_SERVICE_INSTALL` carriers perform the first production build, exact-set installation/start and dynamic readback. `crates/platform/runtime/src/release_carrier_common.rs` is the sole production-linkable Rust owner of shared carrier binding/context/outcome nominal types; `crates/platform/runtime/src/windows/authority_manifest.rs` solely owns the static Authority manifest/install-row/preshutdown-policy family; `crates/platform/runtime/src/windows/deployment.rs` solely owns the variable runtime-participant deployment closure. Their schemas mirror those owners, direct-import foundation/runtime topology as applicable, and never import release. `crates/platform/release` may consume these runtime types but may not redeclare them.

After the final byte of all six Task-10 scripts, the G0 trust tool Authenticode/RFC-3161-signs each one and emits its same-ID descriptor before any rehearsal. Every top-level or nested invocation goes through `PowerShellExecutionPortV1`; direct `powershell`, PATH lookup and `& <child.ps1>` are forbidden. Task 14 later re-signs the two scripts it changes and proves the other four descriptors remain exact members of the final 18-row call-graph closure.

The manifest and extracted MSI service table have exactly five role-tagged rows and byte-equal static projections: `AUTHORITY_SERVER=EPAuthorityServer`, `CONTINUATION=EPF57PowerShutdownContinuation`, `CONTROL_BROKER=EPAuthorityControl`, `RAW_EVIDENCE_SIGNER=EPF57PowerRawSigner`, and `GATE_RUN_JOURNAL_SIGNER=EPF57GateJournalSigner`. All five have the same fixed bare launcher path, while each raw SCM `ImagePath/BINARY_PATH_NAME` is the Windows-quoted launcher followed by that role's exact argv and must round-trip through the frozen argv parser. Package maintenance never changes executable path, argv, start/configuration or calls `ChangeServiceConfig`. All five are `AUTO_START`; the continuation uses `NO_AUTOMATIC_RESTART`, while other roles use the frozen bounded policy. At Task 14 the launcher dispatches each fixed argv to the verified versioned kernel ABI; the Authority role inside that DLL solely owns ordinary recovery/API startup. `EPAuthorityControl` statically reserves disjoint `POWER|PACKAGE|RESILIENCE|POSTGRES_LOG_RETENTION` typed protocol subroots on the same pipe and exact 18-object SDDL model; Task 10 cannot invoke package, resilience or log-retention operations, while Task 14 supplies those handlers. The continuation remains dormant when `ActiveRecordPath` is absent. Only control has preshutdown `ENABLED{timeout_ms=600000}` and `SeShutdownPrivilege`; the other four use the distinct `DISABLED` preshutdown enum.

The runtime owner's following five rows are the sole canonical static service table; every row uses installed path `C:\Program Files\EnterprisePlatform\Authority\ep-core-server.exe`, the manifest's exact `authority_binary_ref`, virtual account shown below and SID type `UNRESTRICTED`. `RESTART_ON_FAILURE_MAX_THREE` is exactly reset period `86400` seconds, non-crash failures enabled and actions `RESTART/5000ms,RESTART/15000ms,RESTART/60000ms,NONE`; `NO_AUTOMATIC_RESTART` has reset period zero and no actions.

| Role | Service / display / account | Source entrypoint | Exact source-order argv | Start / recovery | Preshutdown | Exact dependencies | Exact privileges | Exact endpoint/key |
|---|---|---|---|---|---|---|---|---|
| `AUTHORITY_SERVER` | `EPAuthorityServer` / `Enterprise Platform Authority Server` / `NT SERVICE\EPAuthorityServer` | `crates/platform/authority-kernel/src/dispatch.rs#authority_server` | `["authority-server","--service-mode","windows-scm"]` | `AUTO_START` / `RESTART_ON_FAILURE_MAX_THREE` | `DISABLED` | `[CryptSvc,EPF57DataVolumeUnlockBroker,EPF57EvidenceSignerBroker,EventLog,RpcSs]` | `[SeChangeNotifyPrivilege]` | recovery/readiness pipe `\\.\pipe\EnterprisePlatform\AuthorityRecoveryProofV1`; configured HTTPS/API opens only after ordinary recovery |
| `CONTINUATION` | `EPF57PowerShutdownContinuation` / `Enterprise Platform F57 Power Shutdown Continuation` / `NT SERVICE\EPF57PowerShutdownContinuation` | `crates/platform/authority-kernel/src/dispatch.rs#power_shutdown_continuation` | `["power-shutdown-continuation","--activation-source","scm-parameter"]` | `AUTO_START` / `NO_AUTOMATIC_RESTART` | `DISABLED` | `[CryptSvc,EPAuthorityControl,EPAuthorityServer,EventLog]` | `[SeChangeNotifyPrivilege]` | activation child key `HKLM\SYSTEM\CurrentControlSet\Services\EPF57PowerShutdownContinuation\Parameters\F57ActivationV1` containing value `ActiveRecordPath`; read-only recovery pipe client |
| `CONTROL_BROKER` | `EPAuthorityControl` / `Enterprise Platform Authority Control Broker` / `NT SERVICE\EPAuthorityControl` | `crates/platform/authority-kernel/src/dispatch.rs#authority_control` | `["power-shutdown-control-broker"]` | `AUTO_START` / `RESTART_ON_FAILURE_MAX_THREE` | `ENABLED{timeout_ms=600000}` | `[CryptSvc,EventLog,RpcSs]` | `[SeChangeNotifyPrivilege,SeShutdownPrivilege]` | pipe `\\.\pipe\EnterprisePlatform\EPAuthorityControlV1`; disjoint `POWER\|PACKAGE\|RESILIENCE\|POSTGRES_LOG_RETENTION` protocol capabilities |
| `RAW_EVIDENCE_SIGNER` | `EPF57PowerRawSigner` / `Enterprise Platform F57 Power Raw Evidence Signer Facade` / `NT SERVICE\EPF57PowerRawSigner` | `crates/platform/authority-kernel/src/dispatch.rs#power_raw_signer` | `["power-shutdown-signing-broker","--role","raw-evidence"]` | `AUTO_START` / `RESTART_ON_FAILURE_MAX_THREE` | `DISABLED` | `[CryptSvc,EPF57EvidenceSignerBroker,EventLog,RpcSs]` | `[SeChangeNotifyPrivilege]` | keyless facade pipe `\\.\pipe\EnterprisePlatform\EPF57PowerRawSignerV1`; authenticated forward only to G0 `F57EvidenceSignerV1` operation API |
| `GATE_RUN_JOURNAL_SIGNER` | `EPF57GateJournalSigner` / `Enterprise Platform F57 Gate Run Journal Signer Facade` / `NT SERVICE\EPF57GateJournalSigner` | `crates/platform/authority-kernel/src/dispatch.rs#gate_journal_signer` | `["power-shutdown-signing-broker","--role","gate-run-journal"]` | `AUTO_START` / `RESTART_ON_FAILURE_MAX_THREE` | `DISABLED` | `[CryptSvc,EPF57EvidenceSignerBroker,EventLog,RpcSs]` | `[SeChangeNotifyPrivilege]` | keyless facade pipe `\\.\pipe\EnterprisePlatform\EPF57GateJournalSignerV1`; authenticated forward only to G0 `F57EvidenceSignerV1` operation API |

The five-row Authority table is infrastructure, not the complete product-process list. Product process count remains deployment-variable under ADR-0019. For every CapabilityGraph participant the generated projection contains exactly one delivery row: `ACTIVE` with one carrier-specific artifact binding, or `DEFERRED_DISABLED` with a closed reason and mandatory absence probes. The F-57 first release projects local model execution as `DEFERRED_DISABLED{reason=LOCAL_AI_IMPLEMENTATION_DEFERRED}`; `NullAiProviderV1` remains the explicit business behavior, and no `ai-inferer` process, endpoint, model package or resource reservation may appear. Every other graph-active in-process host, native Windows service/worker, WASM component or Hyper-V-isolated container participant must enter `WindowsRuntimeDeploymentClosureV1`. Fixed historical process counts, directory discovery and an installer-maintained side list are forbidden.

Task 15 materializes that exact projection only from clean `HEAD`: native rows bind built PE/package/SBOM/AuthentiCode and planned service-install rows; in-process rows bind the exact host PE and compiled registration digest; WASM rows bind signed package/module manifests; Hyper-V rows bind the approved image/package and isolation manifest. The install carrier then returns one carrier-specific dynamic readback per `ACTIVE` row and one negative absence readback per `DEFERRED_DISABLED` row. The five Authority service roles and Task-11 backup/recovery component set are separate typed subclosures, with explicit equality joins wherever they host or implement the same runtime participant. The complete graph participant-ID set is bijective with the complete runtime-deployment-closure participant-ID set and complete install-readback participant-ID set. Separately, only the `ACTIVE` participant-ID set is bijective with positive-readback participant IDs, `RuntimeTopologyDeclarationV1.participants[].participant_id`, `GenerationManifestV1.required_participants[].participant_id`, and ACK participant IDs. `database_consumers` independently exact-equals the graph-derived consumer projection for active service identities, so an active participant may own zero or multiple consumers. Generation `items` independently exact-equals the canonical active graph plus installed-fact item projection, while each required participant's `required_item_ids` exact-equals its canonical subset; the relation may be many-to-many, but every item is referenced and no out-of-set item edge is allowed. Every `DEFERRED_DISABLED` row must instead have no artifact, declaration participant, database consumer, required participant, participant-item edge or ACK and exactly one negative absence readback. Missing/extra/duplicate participants, consumers, items or relation edges; an orphan item; two artifacts for one participant; active-but-uninstalled; deferred-but-present; wrong host binary; carrier drift; stale config generation; or a service/package that is installed but not declared fails before candidate freeze.

Static manifest rows contain the Windows virtual-service account and `UNRESTRICTED` SID type but no numeric SID, installed-file readback or attempt-specific descriptor. Task 10 verifies only manifest -> WiX/MSI-table byte equality and explicitly rejects any engineering install/start request. Task 15's service-install carrier resolves every account to canonical numeric `WindowsSidV1`, proves exact manifest -> MSI table -> installed SCM/file equality, and authenticates the same installed `ep-core-server.exe`. The later POWER attempt constructs exactly 18 canonical runtime descriptor rows—five SCM service objects, installed executable, bundle root, run journal, staging root, control-capsule root, phase state, the dedicated activation child registry key, three control/raw/journal broker-facade pipes, the distinct Authority-recovery-proof pipe, and the two G0 evidence-broker key containers. The activation row protects the child registry key as a unit; `ActiveRecordPath` is a value inside it and is never modeled as an independently securable object. `EPAuthorityControl` has only `SC_MANAGER_CONNECT` globally and exactly `QUERY_STATUS|START` on the continuation object, never `STOP|CHANGE_CONFIG|DELETE`; every service/binary/argv/dependency/privilege/preshutdown/numeric-SID/MSI-table/SDDL mutation is a failing golden.

`WindowsAuthorityStaticBuildReceiptV1` is the Task-10-owned unsigned engineering receipt under purpose/media `EP-F57-WINDOWS-AUTHORITY-STATIC-BUILD-RECEIPT-V1` / `application/vnd.ep.f57-windows-authority-static-build-receipt-v1+json`. Its exact fields are `{schema_version,purpose,source_tree_sha256,authority_binary_ref,authority_manifest_ref,wix_include_sha256,msi_ref,extracted_service_rows_sha256,toolchain_ref,build_log_ref}`; the Windows-manifest schema owns it alongside the static manifest. It has no candidate/run/attempt/signer/finalization field and cannot substitute for the Task-14-owned signed `WindowsAuthorityArtifactSetV1`.

`WindowsAuthorityManifestProjectionAuthorityV1` in the runtime owner is the only constructor of the table above. That one constant projection drives both `WindowsAuthorityManifestV1` JCS and `installer/windows/generated/AuthorityServices.wxi`; handwritten WiX files only include that generated fragment and cannot restate a service row. After every build, the MSI-table extractor maps the built package back into the same neutral Rust rows and requires `JCS(manifest.service_rows) == JCS(extracted_msi.service_rows)` before any artifact set can be signed. The exact comparison and byte goldens cover every displayed role/name/display/account/source/binary/SID/start/recovery/action/preshutdown/argv/dependency/privilege/endpoint/key/facade cell; a one-field mutation fails. Task 10 ACL scripts cover baseline/static identities only; the attempt-specific 18-row SDDL authority remains exclusively in Task 14.

The normalized WiX/MSI projection is also closed. Role maps exactly to service IDs `AUTHORITY_SERVER=EPF57SvcAuthority`, `CONTINUATION=EPF57SvcContinuation`, `CONTROL_BROKER=EPF57SvcControl`, `RAW_EVIDENCE_SIGNER=EPF57SvcRawSigner`, and `GATE_RUN_JOURNAL_SIGNER=EPF57SvcJournalSigner`. All five `ServiceInstall.Component_` values equal `EPAuthorityBinaryComponent`, whose key-path file is `EPAuthorityBinaryFile`; the embedded byte-identical manifest is `EPAuthorityManifestFile`. Every row maps `AUTO_START` to `StartType=2`, own-process/normal-error control, frozen argv through the no-shell Windows quoting parser, normalized dependency multi-string, required privileges/SID type, exact recovery actions and only the control broker's `MsiServiceConfig` preshutdown value `600000`. Each role has exactly one `ServiceControl{start_on_install=true,stop_on_uninstall=true,delete_on_uninstall=true,wait=true}`; the installer therefore starts the five already compiled modes from these rows and has no manual service-start branch. Goldens reject a missing/extra ServiceInstall or ServiceControl row, alternate component/file/manifest ID, custom-action-created service, quoting/config mismatch, omitted uninstall event or install-script attempt to create/start a service outside this projection.

- [ ] **Step 1: Write failing Windows security readback tests.**

```rust
#[test]
fn stale_authority_epoch_cannot_write_after_old_service_returns() {
    assert_eq!(write_with_epoch(41, current_epoch(42)).unwrap_err().code(), "AUTHORITY_EPOCH_STALE");
}

#[test]
fn runtime_deployment_closure_is_a_bijection_with_the_graph_projection() {
    assert_runtime_participant_ids_states_carriers_and_capability_digests_exact();
    assert_active_rows_have_exactly_one_legal_carrier_artifact_binding();
    assert_deferred_rows_have_no_artifact_process_endpoint_or_resource_reservation();
    assert_local_ai_is_exactly_deferred_disabled_for_f57();
    assert_code(closure_omitting_an_active_participant(), "F57_RUNTIME_DEPLOYMENT_SET_MISMATCH");
    assert_code(closure_with_deferred_participant_installed(), "F57_RUNTIME_DEFERRED_PARTICIPANT_PRESENT");
}
```

- [ ] **Step 2: Run and verify RED on Windows.**

Run: `cargo test -p ep-testkit --test f57_windows_ipc --test f57_windows_time --test f57_authority_fencing --test f57_windows_runtime_deployment -- --nocapture`

Expected: FAIL.

- [ ] **Step 3: Implement native-only service and fencing controls.**

No active installer/service path invokes WSL, systemd, Podman, Compose, Linux path, TCP localhost admin bypass, or shared administrator account. High-risk time drift fails closed. SSD/HDD path readback must match the signed storage manifest. `platform_authz` remains the sole monotonic AuthorityEpoch owner created in G1; this migration never creates or increments a second epoch. `authority_service_fence_store` records Windows service instance/lease/readback plus the canonical epoch it observed, and `wiring/authority_fence.rs` rejects a returning old service when that reference differs. Tests read `current_epoch` only from the G1 authz store.

`crates/platform/runtime/src/release_carrier_common.rs` implements the complete common nominal family exactly once:

```rust
#[serde(tag = "binding_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseCarrierBindingV1 {
    PreFreeze {
        candidate_identity_sha256: Sha256Digest,
    },
    SignedCandidate {
        candidate_manifest_ref: ArtifactRefV1,
        candidate_identity_sha256: Sha256Digest,
    },
}

#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseCarrierOutcomeV1 {
    Pass,
    Fail,
}

pub struct ReleasePhysicalEvidenceContextV1 {
    pub candidate_run: CandidateRunIdentityV1,
    pub binding: ReleaseCarrierBindingV1,
    pub execution_attempt_id: UuidV1,
    pub runner_id: RunnerIdV1,
    pub host_fingerprint_sha256: Sha256Digest,
    pub started_at_unix_ms: i64,
    pub finished_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}
```

`crates/platform/runtime/src/windows/deployment.rs` owns the following strict family; every vector is sorted by UTF-8 participant ID and byte-unique, and every digest is recomputed over canonical JCS:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeParticipantDeferredReasonV1 {
    LocalAiImplementationDeferred,
    CapabilityNotLicensed,
    HostCapabilityUnavailable,
    CustomerPolicyDisabled,
}

#[serde(tag = "artifact_class", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WindowsRuntimeArtifactBindingV1 {
    WindowsServicePe {
        binary_ref: ArtifactRefV1,
        package_ref: ArtifactRefV1,
        sbom_ref: ArtifactRefV1,
        authenticode_readback_ref: ArtifactRefV1,
        service_install_row_ref: ArtifactRefV1,
    },
    JobObjectWorkerPe {
        binary_ref: ArtifactRefV1,
        package_ref: ArtifactRefV1,
        sbom_ref: ArtifactRefV1,
        authenticode_readback_ref: ArtifactRefV1,
        supervisor_participant_id: String,
        job_policy_sha256: Sha256Digest,
    },
    InProcessHost {
        host_participant_id: String,
        host_binary_ref: ArtifactRefV1,
        compiled_registration_sha256: Sha256Digest,
    },
    WasmComponent {
        host_participant_id: String,
        signed_package_ref: ArtifactRefV1,
        component_manifest_ref: ArtifactRefV1,
        compiled_grant_ceiling_sha256: Sha256Digest,
    },
    HyperVContainer {
        supervisor_participant_id: String,
        signed_package_ref: ArtifactRefV1,
        image_manifest_ref: ArtifactRefV1,
        isolation_policy_sha256: Sha256Digest,
    },
}

#[serde(tag = "delivery_state", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WindowsRuntimeParticipantDeploymentV1 {
    Active {
        participant_id: String,
        carrier: RuntimeCarrierV1,
        capability_set_sha256: Sha256Digest,
        resource_policy_sha256: Sha256Digest,
        artifact: WindowsRuntimeArtifactBindingV1,
    },
    DeferredDisabled {
        participant_id: String,
        reason: RuntimeParticipantDeferredReasonV1,
        required_absence_probe_ids: Vec<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct WindowsRuntimeDeploymentClosureV1 {
    pub schema_version: u32,
    pub capability_graph_digest_sha256: Sha256Digest,
    pub projection_manifest_ref: ArtifactRefV1,
    pub configuration_generation_sha256: Sha256Digest,
    pub participants: Vec<WindowsRuntimeParticipantDeploymentV1>,
    pub active_participant_set_sha256: Sha256Digest,
    pub complete_delivery_set_sha256: Sha256Digest,
}

#[serde(tag = "readback_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WindowsRuntimeParticipantInstallReadbackV1 {
    WindowsServiceRunning {
        participant_id: String,
        service_name: String,
        artifact_ref: ArtifactRefV1,
        installed_file_identity_sha256: Sha256Digest,
        token_service_sid: WindowsSidV1,
        windows_boot_id: String,
        process_id: u32,
        process_start_key: u64,
        held_process_image_identity_sha256: Sha256Digest,
        authenticated_readiness_session_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    JobObjectWorkerRunning {
        participant_id: String,
        supervisor_participant_id: String,
        artifact_ref: ArtifactRefV1,
        installed_file_identity_sha256: Sha256Digest,
        job_object_identity_sha256: Sha256Digest,
        job_policy_readback_sha256: Sha256Digest,
        token_principal_sid: WindowsSidV1,
        windows_boot_id: String,
        process_id: u32,
        process_start_key: u64,
        held_process_image_identity_sha256: Sha256Digest,
        authenticated_readiness_session_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    InProcessReady {
        participant_id: String,
        host_participant_id: String,
        host_binary_ref: ArtifactRefV1,
        host_process_identity_sha256: Sha256Digest,
        compiled_registration_sha256: Sha256Digest,
        authenticated_readiness_session_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    WasmReady {
        participant_id: String,
        host_participant_id: String,
        signed_package_ref: ArtifactRefV1,
        loaded_component_sha256: Sha256Digest,
        sandbox_policy_readback_sha256: Sha256Digest,
        authenticated_readiness_session_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    HyperVContainerReady {
        participant_id: String,
        supervisor_participant_id: String,
        signed_package_ref: ArtifactRefV1,
        running_image_manifest_sha256: Sha256Digest,
        vm_identity_sha256: Sha256Digest,
        isolation_policy_readback_sha256: Sha256Digest,
        authenticated_readiness_session_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    DeferredAbsent {
        participant_id: String,
        reason: RuntimeParticipantDeferredReasonV1,
        observed_service_count: u32,
        observed_process_count: u32,
        observed_endpoint_count: u32,
        observed_package_count: u32,
        observed_resource_reservation_count: u32,
        absence_probe_set_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct WindowsRuntimeDeploymentReadbackSetV1 {
    pub schema_version: u32,
    pub deployment_closure_ref: ArtifactRefV1,
    pub host_fingerprint_sha256: Sha256Digest,
    pub rows: Vec<WindowsRuntimeParticipantInstallReadbackV1>,
    pub canonical_row_set_sha256: Sha256Digest,
    pub completed_at_unix_ms: i64,
}
```

The parser enforces the carrier/artifact/readback matrix exactly: `WINDOWS_SERVICE` requires `WINDOWS_SERVICE_PE/WINDOWS_SERVICE_RUNNING`; `JOB_OBJECT_WORKER` requires `JOB_OBJECT_WORKER_PE/JOB_OBJECT_WORKER_RUNNING`; `IN_PROCESS` requires `IN_PROCESS_HOST/IN_PROCESS_READY`; `WASM_SANDBOX` requires `WASM_COMPONENT/WASM_READY`; and `HYPER_V_CONTAINER` requires `HYPER_V_CONTAINER/HYPER_V_CONTAINER_READY`. Every subordinate carrier row names an existing active host/supervisor participant and the dependency graph must be acyclic; host/supervisor identity, package/binary and policy digest repeat exactly in its readback. A deferred row carries no artifact/service/capability/resource field and requires `DEFERRED_ABSENT` with all five observed counts equal zero; an active row carries no deferred reason or absence probes. Service/worker readiness is bound to boot/PID/start-key/held-image/token SID and an authenticated session; a Job Worker additionally exact-binds its supervisor, Job Object identity and live policy, not a PID or Boolean alone. The full participant ID set and each delivery state exact-match graph projection → deployment closure → readback set, so disabling a participant is itself a governed configuration-generation change rather than omission from the installer.

The common schema strict-parses exactly those three types with unknown fields denied. Private verification wrappers require canonical JCS, `started_at_unix_ms <= finished_at_unix_ms < expires_at_unix_ms`, the closed PRE_FREEZE/SIGNED_CANDIDATE formulas, exact run/attempt/runner/host binding and the caller's typed expected carrier phase. Runtime exports the nominals and verified read-only wrappers; it exposes no generic JSON constructor, release/P340 dependency, signer or PASS builder.

`f57-release-carrier-common-v1-goldens.json` is the byte-authoritative round-trip fixture for all three common nominals and both binding variants. Task 10 tests import its schema directly, prove the runtime-to-schema dependency graph is acyclic, and reject unknown fields, alternate enum spelling, reversed/equal time bounds where forbidden, cross-run/attempt/runner/host substitution, a PRE_FREEZE binding carrying a candidate ref, and a SIGNED_CANDIDATE binding missing or mismatching that ref. Tasks 13 and 14 consume this fixture unchanged and may not create another common-carrier golden.

- [ ] **Step 4: Run the Windows carrier static engineering rehearsal.**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G6_RELEASE --through 20261025092500`

Expected: PASS on a clean PostgreSQL 16 database through the service-fence migration, with exactly one canonical AuthorityEpoch source in `platform_authz` and no `platform_ops` epoch sequence.

Run through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_BUILD_MSI_V1 -- -Mode StaticProjectionOnly -ArtifactManifestOut target\f57\g6-10\windows-authority-manifest.v1.json -MsiOut target\f57\g6-10\ep-authority.msi -BuildReceiptOut target\f57\g6-10\windows-authority-static-build-receipt.v1.json`.

Expected: deterministic engineering MSI plus extracted static service table, manifest and the Task-10-owned strict unsigned static build receipt at exactly those task-derived paths for offline equality checks only; no output path may be omitted, defaulted, discovered or scanned. No artifact-set file or candidate/run/attempt/signed evidence is emitted. The command refuses installation/start because the five ServiceMain modes do not exist until Task 14. This is not a frozen or release-signed candidate.

The build script's closed mode enum is exactly `StaticProjectionOnly|Release`; missing/unknown mode or a missing exact output/staging argument fails before build, signing, installation or service action. `StaticProjectionOnly` cannot call the installer or emit candidate evidence, and `Release` is reachable only through Task 15's fixed signed carrier recipe.

Run: `cargo xtask f57 windows-manifest project --check --manifest target/f57/g6-10/windows-authority-manifest.v1.json --wix-include installer/windows/generated/AuthorityServices.wxi --msi target/f57/g6-10/ep-authority.msi --build-receipt target/f57/g6-10/windows-authority-static-build-receipt.v1.json`

Expected: the source-derived manifest rows, generated WiX include and extracted built-MSI rows are byte-equivalent in the neutral Rust projection; handwritten duplicate service rows, stale generated output, or any one-field mutation fails.

Run: `cargo test -p ep-testkit --test f57_windows_ipc --test f57_windows_time --test f57_authority_fencing --test f57_windows_runtime_deployment -- --nocapture`

Run: `cargo xtask f57 graph generate --check`

Run: `cargo test -p ep-platform-runtime -p ep-adapter-ipc -p ep-adapter-db-pg -p core-server -p ep-xtask -p ep-testkit --all-targets --locked`

Expected: PASS as static component/rehearsal evidence with graph/projection digest closure and an explicit negative proving Task 10 cannot install or start any authority service. Task 14 only lands/compiles the entrypoints; Task 15 performs the first production build and installation from clean frozen `HEAD`.

- [ ] **Step 5: Commit Windows carrier.**

```bash
cargo xtask f57 task stage --task G6-10
cargo xtask f57 task verify-staged --task G6-10
git commit -m "feat: add native Windows authority static projection"
```

### Task 11: Add and rehearse streaming append-only backup and offline media

**Files:**
- Create: `crates/platform/backup/Cargo.toml`
- Create: `crates/platform/backup/src/lib.rs`
- Create: `crates/platform/backup/src/envelope.rs`
- Create: `crates/platform/backup/src/checkpoint.rs`
- Create: `crates/platform/backup/src/checkpoint_signer.rs`
- Create: `crates/platform/backup/src/recovery_cut.rs`
- Create: `crates/platform/backup/src/safeguard.rs`
- Create: `crates/platform/backup/src/topology_signing_trust.rs`
- Create: `crates/platform/backup/src/postgres16_windows.rs`
- Create: `crates/platform/backup/src/postgres16_log_retention.rs`
- Create: `crates/platform/backup/src/data_hdd_disaster_replacement.rs`
- Create: `crates/platform/backup/src/model.rs`
- Create: `crates/platform/backup/src/ports.rs`
- Create: `docs/evidence/f57-backup-checkpoint.v1.schema.json`
- Create: `docs/evidence/f57-authority-recovery-cut-manifest.v1.schema.json`
- Create: `docs/evidence/f57-backup-storage-safeguard.v1.schema.json`
- Create: `docs/evidence/f57-backup-topology-signing-trust-manifest.v1.schema.json`
- Create: `docs/evidence/f57-backup-topology-signing-trust-current-pointer.v1.schema.json`
- Create: `docs/evidence/f57-postgres16-windows-install.v1.schema.json`
- Create: `docs/evidence/f57-postgres16-log-retention.v1.schema.json`
- Create: `docs/evidence/f57-data-hdd-disaster-replacement.v1.schema.json`
- Create: `docs/evidence/f57-windows-server-component-set.v1.schema.json`
- Create: `installer/windows/postgresql16.lock.json`
- Create: `installer/windows/postgresql16-tls-policy.json`
- Read/import: `docs/evidence/f57-recovery-domain-manifest.schema.json`
- Create: `crates/platform/backup/tests/backup_contract.rs`
- Create: `crates/platform/backup/tests/fixtures/f57-backup-checkpoint-v1-golden.json`
- Create: `crates/platform/backup/tests/fixtures/f57-authority-recovery-cut-manifest-v1-golden.json`
- Create: `crates/platform/backup/tests/fixtures/f57-backup-storage-safeguard-v1-goldens.json`
- Create: `crates/platform/backup/tests/fixtures/f57-backup-topology-signing-trust-v1-goldens.json`
- Create: `crates/platform/backup/tests/fixtures/f57-backup-protection-transition-v1-goldens.json`
- Create: `crates/platform/backup/tests/fixtures/f57-postgres16-windows-install-v1-goldens.json`
- Create: `crates/platform/backup/tests/fixtures/f57-postgres16-windows-event-log-fixture-set-v1.json`
- Create: `crates/platform/backup/tests/fixtures/f57-recovery-tool-scheduled-task-v1-golden.json`
- Create: `crates/adapter/backup/Cargo.toml`
- Create: `crates/adapter/backup/src/lib.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_store.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_checkpoint_signing_attempt_store.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/backup_topology_signing_trust_store.rs`
- Create: `crates/adapter/db-pg/src/postgres16_windows.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `apps/backup-writer/Cargo.toml`
- Modify: `apps/backup-writer/src/main.rs`
- Create: `apps/backup-writer/src/wiring/backup.rs`
- Create: `apps/backup-writer/tests/backup_composition.rs`
- Modify: `apps/backup-writer/src/wiring/mod.rs`
- Create: `apps/backup-checkpoint-signer/Cargo.toml`
- Create: `apps/backup-checkpoint-signer/src/main.rs`
- Create: `apps/backup-checkpoint-signer/src/wiring/checkpoint.rs`
- Create: `apps/backup-checkpoint-signer/tests/checkpoint_composition.rs`
- Create: `apps/backup-target/Cargo.toml`
- Create: `apps/backup-target/src/main.rs`
- Modify: `apps/recovery-tool/Cargo.toml`
- Modify: `apps/recovery-tool/src/main.rs`
- Create: `apps/recovery-tool/src/backup.rs`
- Create: `apps/recovery-tool/src/scheduled_task.rs`
- Create: `apps/recovery-tool/src/operation_protocol.rs`
- Create: `apps/recovery-tool/src/operation_journal.rs`
- Create: `apps/recovery-tool/tests/scheduled_task_protocol.rs`
- Create: `apps/pg-passphrase-helper/Cargo.toml`
- Create: `apps/pg-passphrase-helper/src/main.rs`
- Create: `crates/platform/backup/src/windows_components.rs`
- Create: `crates/platform/backup/src/windows_components/unlock.rs`
- Create: `apps/data-volume-unlock-broker/Cargo.toml`
- Create: `apps/data-volume-unlock-broker/src/main.rs`
- Create: `apps/data-volume-unlock-broker/tests/windows_service_protocol.rs`
- Read/import byte-for-byte from G0: `apps/evidence-signing-broker`, its signed package/static install row, deterministic WiX fragment, and `docs/evidence/f57-evidence-signer-broker-windows-install-readback.v1.schema.json`
- Create: `installer/windows/BackupComponents.wxs`
- Regenerate: `installer/windows/generated/BackupComponents.wxi`
- Create: `docs/deployment/f57-windows-backup-components.v1.json`
- Create: `docs/evidence/f57-data-hdd-bitlocker-unlock.v1.schema.json`
- Regenerate: `docs/generated/f57/security/data-hdd-unlock-bootstrap-locator-set.v1.json`
- Create: `crates/platform/backup/tests/windows_components.rs`
- Create: `crates/platform/backup/tests/data_hdd_unlock.rs`
- Create: `crates/platform/backup/tests/fixtures/f57-data-hdd-unlock-bootstrap-locator-set-v1-golden.json`
- Create: `crates/platform/backup/tests/fixtures/f57-data-hdd-unlock-broker-bootstrap-readback-v1-golden.json`
- Create: `crates/platform/backup/tests/fixtures/f57-data-hdd-bitlocker-unlock-v1-goldens.json`
- Create: `crates/platform/backup/tests/fixtures/f57-evidence-signer-broker-windows-install-readback-v1-golden.json`
- Create: `testkit/tests/f57_windows_server_2022_data_hdd_unlock.rs`
- Create: `testkit/tests/f57_windows_service_inventory.rs`
- Create: `testkit/tests/f57_postgres16_windows_install.rs`
- Create: `testkit/tests/f57_postgres16_log_retention.rs`
- Create: `testkit/tests/f57_data_hdd_disaster_replacement.rs`
- Create: `testkit/tests/f57_backup_storage_safeguard.rs`
- Create: `testkit/tests/f57_backup_topology_signing_trust.rs`
- Create: `testkit/tests/f57_backup_checkpoint_transition.rs`
- Create: `scripts/windows/archive-wal.ps1`
- Create: `scripts/windows/test-postgres16-pitr.ps1`
- Create after final signing: `scripts/windows/trust/F57_PS_ARCHIVE_WAL_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_TEST_POSTGRES16_PITR_V1.authenticode.json`
- Regenerate after both descriptor verifications: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after both descriptor verifications: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after both descriptor verifications: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Create: `db/migrations/platform_ops/V20261025092510__platform_ops_create_backup_sets_media_and_certification.sql`
- Create: `testkit/tests/f57_backup_target.rs`
- Create: `testkit/tests/f57_backup_envelope.rs`
- Create: `testkit/tests/f57_postgres16_recovery.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: ADR-0021/0024 backup envelopes, dedicated backup/replication identity, external append-only target, two offline encrypted HDDs.
- Produces: streamed encrypted chunks, complete readback, a strict fourteen-field full-authority recovery-cut manifest, an unsigned immutable checkpoint draft, a separately authorized signed checkpoint, PITR and media rotation evidence. `ep-platform-backup` is the sole production-linkable owner of `BackupSourceV1`, `AppendOnlySinkV1`, `AuthorityRecoveryCutManifestV1`, the neutral private `VerifiedBackupCheckpointRequestV1`, `BackupCheckpointDraftStoreV1`, `BackupCheckpointSigningAttemptStoreV1`, `BackupCheckpointObjectStoreV1`, `BackupCheckpointSignPortV1`, `BackupCheckpointVerifyPortV1`, all checkpoint payload/state types, private verified wrappers and the closed `BackupErrorV1`; adapters implement only those ports and applications only compose them. Its request can represent a scheduled backup or a package-maintenance barrier without importing package types. `backup-writer` can produce and mark a complete draft READY but its dependency graph and service identity have no checkpoint signing port, signing key, checkpoint-object bind permission, or historical-decrypt permission. The separately installed `backup-checkpoint-signer` identity is the sole normal caller of `BackupCheckpointSignPortV1`; it cannot read plaintext chunks or write backup data and advances the durable `OPERATION_COMMITTED -> ENVELOPE_SPOOLED -> OBJECT_BOUND` protocol. The checkpoint media is `application/vnd.ep.f57-backup-checkpoint-v1+json`, purpose is `EP-F57-BACKUP-CHECKPOINT-V1`; the cut media is `application/vnd.ep.f57-authority-recovery-cut-manifest-v1+json`, purpose is `EP-F57-AUTHORITY-RECOVERY-CUT-MANIFEST-V1`. Signing/verifying uses only the dedicated backup-recovery trust domain. Neither root enters the 89-row candidate-evidence signer registry. No adapter or application may redeclare the wire, choose a purpose/media/signer, construct a verified checkpoint, or collapse writer and signer identities.

`crates/platform/backup/src/topology_signing_trust.rs`, `crates/platform/backup/src/safeguard.rs` and their three Task-11 strict schemas solely own five anti-ransomware roots. Signed `BackupTopologySigningTrustManifestV1` has purpose/media `EP-F57-BACKUP-TOPOLOGY-SIGNING-TRUST-MANIFEST-V1` / `application/vnd.ep.f57-backup-topology-signing-trust-manifest-v1+json`; signed `BackupTopologySigningTrustCurrentPointerV1` has `EP-F57-BACKUP-TOPOLOGY-SIGNING-TRUST-CURRENT-POINTER-V1` / `application/vnd.ep.f57-backup-topology-signing-trust-current-pointer-v1+json`; signed `BackupTopologyV1` has `EP-F57-BACKUP-TOPOLOGY-V1` / `application/vnd.ep.f57-backup-topology-v1+json`; strict plain `StorageSafeguardReadbackV1` has `EP-F57-STORAGE-SAFEGUARD-READBACK-V1` / `application/vnd.ep.f57-storage-safeguard-readback-v1+json`; and closed `StorageSafeguardSupportEvidenceV1` has `EP-F57-STORAGE-SAFEGUARD-SUPPORT-EVIDENCE-V1` / `application/vnd.ep.f57-storage-safeguard-support-evidence-v1+json`. Every signed envelope uses the one strict JCS/detached-CMS verifier; every support signature is strict RFC-4648 padded CMS over the exact JCS bytes of `{schema_version,purpose,payload}`, re-encodes byte-identically, and its vector is strictly sorted and unique by `(signer_principal_id,signer_spki)`. A `TARGET_RECEIPT` has exactly one signature: `signer_principal_id` byte-equals the current topology's `role_bindings[BACKUP_TARGET_AGENT].principal_id` rather than the literal role wire, `signer_spki` byte-equals `continuous_target.target_server_spki`, and `SHA256(JCS(signer_spki))` byte-equals that role binding's `credential_identity_sha256`. Either offline branch has exactly the two topology-pinned human custodian signatures. These five roots add no candidate-evidence signer row.

Currentness is explicit, independent and acyclic. The deployment bootstrap pins exactly one trust-manifest authority DN `CN=EP F57 Backup Topology Trust Manifest Authority,O=Enterprise Platform`, one SPKI and its offline verification policy; only that authority can sign the trust manifest and current pointer. The trusted monotonic active configuration carries `backup_topology_signing_trust_current_ref` and `backup_topology_ref`; both must byte-equal the supplied envelopes, so no caller, directory scan, filename, timestamp or merely still-valid older object selects either. Pointer and manifest genesis are each generation 1 with null predecessor; every successor generation is checked prior+1, names the exact immutable prior signed-envelope ref, retains deployment identity, never decreases authority epoch, has strictly later issue time and cannot fork or overflow. The current pointer typed-loads exactly one manifest with the same positive `trust_generation`, deployment and valid time window. That manifest fixes the topology signer to the literal DN `CN=EP F57 Backup Topology Authority,O=Enterprise Platform`, exactly one leaf `SignerSpkiTokenV1`, and exact typed offline certificate-chain, revocation-snapshot and transparency-checkpoint refs. `BackupTopologyAuthorityV1` has no public raw-wrapper constructor and can be composed only from private `VerifiedBackupTopologySigningTrustCurrentV1`; it verifies a topology only against that current manifest, never against fields in the topology, storage manifest, support evidence, candidate signer rows, ambient Windows roots, `ApplicationRecoveryDomainManifestPayloadV1`, its ADR-0020 `PIV_SHAMIR_2_OF_3_V1` recipient roster, `BackupRecoveryDomainManifestPayloadV1`, or any backup-key-envelope share roster. Verification checks exact CMS content, current offline chain/revocation/checkpoint, `issued_at_unix_ms < expires_at_unix_ms`, trusted `now` inside the inclusive validity window, certificate validity, and CMS signing time inside both certificate validity and `[issued_at_unix_ms,min(issued_at_unix_ms+300000,expires_at_unix_ms)]`. Each topology exact-repeats the current pointer ref, its manifest ref and trust generation. Topology genesis is `revision=1` with null predecessor; a successor is create-new with revision prior+1, exact prior signed-envelope ref, same deployment, nondecreasing authority epoch/generation and strictly later issue time. `authority_storage_manifest_ref` byte-equals the current trusted `F57AuthorityStorageManifestV1`; deployment and authority epoch exact-match, topology generation exact-matches the current signed generation, `backup_target_ids` is exactly `[continuous_target.target_id]`, and `policy_ids` contains exactly one `backup-topology-signing-trust-current-sha256:<64-lowerhex>` entry whose suffix is the current pointer envelope digest. A rotation reuses the already verified current trust tuple or, when topology-signer trust changes, first writes and independently verifies the successor trust manifest/pointer; it then writes and verifies a storage-manifest successor when storage or its trust pin changes and always writes a topology successor referencing the resulting current tuple; one serialized active-config CAS advances the pointer/topology/storage tuple and opens the typed `CURRENT_ROOTS_ROTATION` transition below. This ordering creates no self-authentication or content-addressed cycle. Missing/expired/untrusted material, wrong DN/SPKI/media/purpose, pointer/manifest generation or predecessor error, fork, time rollback, trust/root substitution, epoch/generation rollback, manifest/target/policy-pin mismatch, stale config or old-topology substitution fails closed.

That normal-rotation ordering is additionally guarded by upper admission and may never begin with the active-config CAS. From the fresh `HEALTHY` source readback and already verified successor tuple, the upper coordinator first creates/adopts one deployment-wide `ProductionAdmissionHoldV1{cause=CURRENT_ROOTS_ROTATION{target_backup_topology_ref,target_authority_storage_manifest_ref,target_topology_signing_trust_current_ref}}`, rejects new request and long-job leases, drains every already accepted lease and durably commits one `write_barrier_id`. Only the same hold/cause/barrier-bound operation may then execute the serialized active-config CAS, whose target byte-equals the cause tuple. The hold remains current throughout `TRANSITIONING` and `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION`; neither bridge creation, response-loss adoption nor A/B copy may release it. Reopening is one upper admission CAS that requires a fresh `HEALTHY/None` readback, exact cause tuple/new head, current authority epoch and OBSERVED generation, and zero pre-barrier lease, then appends `REOPENED` for that same hold. A CAS-before-hold, hold scoped below deployment, cross-cause ref, barrier-before-drain, release in either transition state, or healthy readback for an old tuple fails closed.

Canonical topology vectors have one representation: `role_bindings` contains exactly the six `BackupRoleV1` values in enum order with unique principal and credential identities; `offline_media` is exactly `[ROTATION_A,ROTATION_B]`. `BACKUP_WRITER.credential_identity_sha256=SHA256(JCS(continuous_target.writer_client_spki))` and `BACKUP_TARGET_AGENT.credential_identity_sha256=SHA256(JCS(continuous_target.target_server_spki))`; observed writer/target SPKIs and every target-receipt signer exact-join those values. One off-host continuous HDD target and both offline HDD rows exact-declare host/media, failure, administration, credential, custody and physical-location domains. Across A/B, each pair of `media_id`, `hardware_serial_sha256`, `volume_identity`, `volume_guid`, and live `volume.physical_disk_identity_sha256` is nonempty and unequal; each live row's role/media ID/hardware serial/volume identity/volume GUID byte-equals its topology row, so two volumes on one physical HDD cannot satisfy A/B. Continuous live `observed_failure_domain_id|observed_administration_domain_id|observed_credential_domain_id`, and offline live `failure_domain_id|administration_domain_id|custody_domain_id|credential_domain_id|physical_location_id`, byte-equal their topology rows and come from the signed support receipt/observation projections; the three legacy `shared_*` booleans are recomputed results, never evidence inputs. Each media row has exactly two `custodian_bindings`, strictly sorted by principal ID, distinct in principal/SPKI/administration domain, disjoint from every six-role principal/credential/administration domain, and current because the selected topology is current; the live vector byte-equals it. Zero, one, duplicate, three, unsorted or stale-topology custodians fail.

Domain separation is a complete pairwise formula, not only the existing continuous-versus-production comparison. Let `E=[PRODUCTION,CONTINUOUS,ROTATION_A,ROTATION_B]` and `D=[FAILURE,ADMINISTRATION,CREDENTIAL,CUSTODY,LOCATION]`; the topology and live support projections carry one nonempty domain ID for every `(d,e)`, and the verifier checks `domain(d,E[i]) != domain(d,E[j])` for every `d` and every `0 <= i < j < 4` (exactly 30 inequalities). Negative goldens independently alias every pair and include same tenant/root or management group, same SPKI/secret/recovery credential, same host/rack/UPS/room failure boundary, same custody roster and same physical location. A single `shared_* = false`, provider assertion without raw support, or equality hidden behind different labels is rejected.

Every `capacity_calculation_input_ref|quota_exhaustion_probe_ref|emergency_reserve_probe_ref|partial_timeout_probe_ref|expired_partial_cleanup_probe_ref|bulk_work_suspension_readback_ref|retention_policy_readback_ref|just_written_create_receipt_ref|just_written_exact_readback_ref` typed-loads the exact corresponding `TARGET_RECEIPT` kind; every permission row's `denial_receipt_ref` typed-loads `PERMISSION_DENIAL`. Each repeats topology/target/nonce/session/probe, has `subject_projection_sha256=SHA256(JCS(the schema-defined complete containing-field projection))`, and exact-matches before/after/outcome/times. Offline `state_transition_ref` typed-loads `OFFLINE_MEDIA_TRANSITION`; `safe_eject_readback_ref|physical_disconnection_attestation_ref|custody_record_ref|health_readback_ref` typed-load the matching `OFFLINE_MEDIA_OBSERVATION` kind and exact-bind topology/media/current transition/state/nonce/session and the complete containing-row projection. Wrong kind/media/signer/ref or field substitution fails offline verification.

The offline transition chain is deterministic. Sequence 1 has null predecessor/previous state and enters `BLANK`; every successor is prior+1, names the exact prior evidence ref, has `previous_state=prior.current_state`, and uses only the eight edges `BLANK->ENROLLED`, `ENROLLED->ACTIVE_APPEND`, `ACTIVE_APPEND->VERIFIED_DISCONNECTED`, `VERIFIED_DISCONNECTED->ROTATION_DUE`, `ROTATION_DUE->ACTIVE_APPEND`, `ACTIVE_APPEND->SEALED_VERIFIED`, `SEALED_VERIFIED->RETIRED_PENDING_DISPOSAL`, and `RETIRED_PENDING_DISPOSAL->DESTROYED`. `SEALED_VERIFIED` can never return to a writable state and `DESTROYED` is terminal; reuse of destroyed physical media requires a new `media_id` and a new sequence-1 `BLANK` chain. Live `transition_sequence` byte-equals the head payload sequence, `state_transition_ref` is that head, and `transition_chain_head_sha256` equals the ref's complete-object SHA-256. Gap, fork, illegal edge, state mismatch, ID reuse or hash substitution fails.

Every install, checkpoint preparation, PITR, activation and every retry uses a new unpredictable 32-byte/64-lowerhex nonce, new authenticated session binding, new support receipts/observations and current boot/attempt; reuse fails even for the same activation ID or checkpoint set. `maximum_safeguard_readback_age_seconds` is checked nonzero and at most 300. Top-level expiry checked-equals `observed_at_unix_ms + maximum_safeguard_readback_age_seconds*1000` without overflow, every live support object is valid through that expiry, the topology remains the selected current head through it, and a consumer verifies trusted `observed_at <= now <= expires_at`. Every consumer typed-loads `backup_topology_ref` as the signed current `BackupTopologyV1`, requires `StorageSafeguardReadbackV1.topology_signing_trust_current_ref` to exact-equal the topology field and active-config current pointer, typed-loads that pointer plus its independent current trust manifest into private `VerifiedBackupTopologySigningTrustCurrentV1`, and verifies the topology signer only through that value, typed-loads the exact current `F57AuthorityStorageManifestV1` and every support-evidence ref, and reconstructs the checkpoint chain rather than trusting repeated scalar fields. `latest_backup_checkpoint_ref` is respectively the service-install binding's explicit expected option, the checkpoint-preparation binding's `expected_prior_backup_checkpoint_ref`, `Some(Postgres16Pitr.backup_checkpoint_ref)`, or `Some(ProductionActivation.certified_latest_backup_checkpoint_ref)`; activation's value also equals the certificate/recovery chain's certified latest current checkpoint. Inference from a “newest” storage object is forbidden. `AuthorityRecoveryCutManifestV1.backup_topology_ref` exact-loads this same topology throughout target receipts, checkpoint, PITR and activation, and its storage-manifest ref/digest, deployment, epoch, generation, backup set, context and barrier exact-join the checkpoint request, draft and payload.

Checkpoint bootstrap and chaining are total rather than circular. `protection_transition` is not a caller-supplied explanation: the sole authority projects one immutable transition object from the monotonic active configuration, and later readbacks repeat it byte-for-byte rather than mutating a stored “next sequence.” `HEALTHY` requires it to be `None`; `INITIALIZING|BOOTSTRAPPING|TRANSITIONING` require `Some` with the exact current transition ID/generation/targets, while `NON_SUPPRESSIBLE_RISK` preserves that `Some` iff an active transition exists but never grants an operation. A clean `WindowsServiceInstall` with `expected_latest_backup_checkpoint_ref=None` still verifies the independent current trust pointer/manifest, current topology, singleton-target storage manifest, fresh support evidence and every permission/capacity/lifecycle invariant. It requires top-level latest/head/continuous/A/B vectors all empty and `INITIALIZING + INITIAL_POPULATION`: all five prior/anchor fields are `None` and all three target refs equal the active current tuple. This can PASS infrastructure installation but never PITR, release, recovery certification or production activation. `BackupCheckpointPreparation` is collected only after the immutable recovery-cut manifest exists and before draft construction; its binding exact-repeats backup set, positive sequence, context, barrier, cut ref and expected prior head. Draft and signed payload byte-repeat those values and the exact fresh safeguard ref. The sole first-checkpoint exception accepts that exact `INITIALIZING` shape, derives sequence 1 from the empty chain, fixes prior `None`, and creates a checkpoint under the current trust/topology/storage tuple. Thereafter a fresh readback becomes `BOOTSTRAPPING + INITIAL_POPULATION` with the identical immutable transition object and a nonempty current chain. `BOOTSTRAPPING + INITIAL_POPULATION` authorizes another checkpoint only while the distinct continuous and A/B-union sequence counts are still below `minimum_retained_generation_count`, both A and B are already nonempty verified subsets, their union includes the current head, every other `HEALTHY` invariant passes, and the requested sequence is the checked current-head sequence plus one with prior equal to that head. Thus with the required minimum of two, checkpoint 1 is first copied and verified into both A/B, checkpoint 2 can then be made, and its verified offline copy closes the minimum; larger configured minima iterate the same derived-head rule without changing the transition object. As soon as the complete healthy formula is true, the next fresh readback must be `HEALTHY` with `protection_transition=None`; remaining in `BOOTSTRAPPING` is invalid. No readback is required to contain the not-yet-created checkpoint, avoiding a reference cycle.

Every checkpoint ref is typed-loaded as `BackupCheckpointV1`: deployment is constant; historical topology/storage/trust/recovery-domain refs are retained and must be fully verified ancestors on their respective predecessor chains; epoch, release generation and config generation never decrease; and `backup_set_id` is nonnil and unique. Continuous refs are strictly sorted and unique by `(checkpoint_sequence,uri,sha256)`, begin at sequence 1, increment exactly one, and every predecessor is the immediately prior ref; head equals the last row or is `None` iff empty, and top-level latest byte-equals head. In `HEALTHY|BOOTSTRAPPING`, the head exact-binds the current topology, storage manifest and topology-signing current pointer. The sole old-head exception is a serialized current-root rotation begun from a fresh `HEALTHY` readback: after independently verifying the successor trust manifest/pointer, storage manifest and topology, one active-config CAS advances that tuple and opens one immutable `CURRENT_ROOTS_ROTATION` with a new transition ID/generation; its five prior/anchor fields are all `Some` and exact-load the prior healthy readback, old head, old topology, old storage manifest and old trust pointer, while its three target refs are the new active tuple. A fresh readback is then `TRANSITIONING`. It permits exactly one bridge checkpoint whose sequence is derived as checked `anchored_old_head.sequence+1`, whose previous ref is that anchor and whose cut, draft, payload and safeguard bind the new active tuple; response loss adopts the same signing operation and cannot create a second bridge. Once that checkpoint is signed, `TRANSITIONING` is illegal: the next readback is `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION` while the new head awaits required offline verification, and that state authorizes no additional checkpoint—only exact A/B copy/verification can advance it to `HEALTHY/None`. A second topology, storage or trust rotation is rejected until `HEALTHY`; no caller may reset the transition ID/generation or substitute a different anchor/target. In `HEALTHY`, continuous and both A/B vectors are nonempty; each offline vector is a strictly sequence-sorted unique subset with support evidence verified through that media/custodian chain, their union includes latest, and both continuous and A/B-union distinct counts meet the minimum. Only a shortage of retained generations during `INITIAL_POPULATION`, or a pending exact offline copy after the bridge, with all trust, chain, permission, capacity, lifecycle and freshness checks passing is a bootstrap deficiency. Any malformed transition, wrong current/ancestor relation, gap/fork, stale or duplicate head/set/sequence, failed security/resource probe, one-sided media, transition collision or state that should already be healthy yields `NON_SUPPRESSIBLE_RISK` and authorizes no checkpoint. `INITIALIZING|BOOTSTRAPPING|TRANSITIONING|NON_SUPPRESSIBLE_RISK` are categorically ineligible for PITR, release/recovery certification and production activation; those consumers accept only a fresh explicit `HEALTHY` head and never a valid ancestor or inferred newest object.

Catastrophic loss of the old DATA_HDD is not that old-head exception and never asks the dead media to emit fresh `HEALTHY`. `DataHddDisasterReplacementAuthorityV1` solely owns the strict `DATA_HDD_DISASTER_REPLACEMENT` protocol and can start only from independently typed-loaded off-host current configuration/trust roots plus the last authenticated checkpoint and full recovery cut. It requires two distinct recovery custodians; creates/adopts a deployment-wide hold with tagged `DATA_HDD_DISASTER_REPLACEMENT{recovery_attempt_ref}` cause; drains or proves the old authority unreachable, fences every old write lease/volume/host path; checked-increments `authority_epoch` and storage generation; enrolls a replacement physical-disk identity, volume ID/GUID, GPT/NTFS and 100% software BitLocker `{PUBLIC_KEY,RECOVERY_PASSWORD}`; and signs a successor storage manifest that can name only that new identity. The recovery worker then performs clean-host PostgreSQL PITR and exact cluster/WAL verification plus byte/count/digest reconciliation of every enabled cut class including attachments, audit, Outbox, vault, packages, identity/authz and durable jobs. It bootstraps the continuous target and A/B from an empty chain under the new tuple until a fresh `HEALTHY/None` head exists, then requires a new P340 infrastructure/capacity certification. Only the final upper admission CAS may promote the new authority. This hold-scoped disaster PITR is a private recovery capability and does not permit ordinary PITR from any non-healthy runtime state. A future graph-versioned IaaS profile must supply its own independent disaster-replacement and recertification branch. Missing off-host currentness, one-person approval, unfenced old authority, unchanged epoch/generation or volume identity, old-manifest reuse, partial reconciliation, inherited backup head, stale P340 capacity evidence or any pre-final-CAS route leaves production closed.

All capacity arithmetic is performed in checked `u128` and rejected unless representable as `u64`. Continuous `resources.quota_policy` byte-equals the topology policy; all four quota maxima and the partial timeout are nonzero. `required_total_capacity_bytes` equals retained ciphertext + restore-validation workspace + 30-day growth + emergency reserve; physical total is at least required total, free is at least validation + growth + reserve, `maximum_stored_bytes + reserve <= total`, `used_object_count <= maximum_object_count`, `used_bytes <= maximum_stored_bytes`, `observed_ingress_bytes_per_second <= maximum_ingress_bytes_per_second`, `active_upload_count <= maximum_concurrent_uploads`, and `reserve <= emergency_reserve_free_bytes <= free`. The four `*_limit_enforced` values are all true and are not trusted alone: `quota_exhaustion_probe_ref` is the signed `QUOTA_EXHAUSTION` target receipt whose schema-defined projection exact-binds this policy, these live counters and four isolated boundary probes; each dimension accepts a checked in-policy request, denies the first checked out-of-policy request, leaves completed/pinned history unchanged and leaves no partial probe object. Each offline required total equals recoverable set + validation workspace + growth; physical total is at least it and free is at least validation + growth. Expired partial count/bytes are zero; active count zero iff active bytes zero and oldest age is `None`, while positive count requires positive bytes and `Some(age < partial_object_timeout_seconds)`. Completed/pinned history is unchanged and exhaustion suspends bulk work. Every volume is GPT/NTFS/HDD/BitLocker software XTS-AES-256/100%; continuous protectors are exactly `{PUBLIC_KEY,RECOVERY_PASSWORD}`, offline exactly `{RECOVERY_PASSWORD}`. Retention checked-equals `max(site_legal,7776000,2*detection_lag_p99+clean_restore_validation_window,2*rotation_interval)`, offline age is at most 604800 seconds, retained generations at least two, and `bundle_contains_recovery_material=false` for both A/B in every binding and any PITR leaf.

`permission_negative_probes` is the exact vector sorted by `(actor_role,operation)`: writer denies history enumeration/read/re-read, overwrite/delete/rename, ACL/ownership, retention/quota/reserve and partial cleanup; target-agent denies **every** `BackupPermissionProbeOperationV1` direct/unbound operation (normal append is only the private capability path below); partial-maintenance denies cleanup of non-expired/completed/pinned objects; retention-custodian denies shortening/create/restore; checkpoint-signer denies create/enumerate/content-read/delete; recovery-custodian denies source-retention mutation/source-delete. Every row's signed target receipt exact-matches actor/operation/probe and unchanged before/after digest. Production A/B must be `VERIFIED_DISCONNECTED|SEALED_VERIFIED`, zero-attached at authority and target, authorization revoked, safely ejected, physically disconnected, healthy and dual-custodied under the current transition. Any unknown, stale, incomplete, insufficient-capacity, shared/mismatched-domain or probe failure yields `NON_SUPPRESSIBLE_RISK`. `AppendOnlySinkV1` exposes only authenticated append, a private affine just-written capability consumed by exactly one exact-object verification read, and seal; response-loss adoption returns the persisted signed receipt without a second content read, and no list/history/rename/delete/ACL/retention API exists. Schema/crash goldens cover every support kind/ref substitution, signer/quorum/custodian mutation, topology rollback/fork, A/B media/serial/volume/physical-disk alias, every illegal lifecycle edge or destroyed-ID reuse, all four quota boundaries/flags, domain/SPKI mismatch, arithmetic overflow/shortfall, partial optionality, recovery-material bit and permission row.

Task 11 solely owns the deployment source and strict schema for exactly six backup/recovery component IDs: `BACKUP_WRITER_SERVICE`, `BACKUP_CHECKPOINT_SIGNER_SERVICE`, `DATA_VOLUME_UNLOCK_BROKER_SERVICE`, `RECOVERY_TOOL`, `PG_PASSPHRASE_HELPER`, and `BACKUP_TARGET_AGENT`. The first three are distinct Authority-host `WINDOWS_SERVICE` rows; `RECOVERY_TOOL` is the sole `ON_DEMAND_SCHEDULED_TASK`, `PG_PASSPHRASE_HELPER` is the sole `ON_DEMAND_EXECUTABLE`, and `BACKUP_TARGET_AGENT` is `OFF_HOST_ONLY` and forbidden on the P340. The generated `BackupComponents.wxi` contains exactly five on-host binaries, three typed service rows and one immutable Scheduled Task row; the off-host package contains only the target agent. Static source, schema, generated registry, WiX, extracted MSI and live readback compare every field byte-for-byte. Task 14 imports this registry unchanged; Task 15 alone builds, signs, installs, starts and collects dynamic readback. A caller-selected argv/path/task, co-located target, shared writer/signer/unlock identity, untyped helper, second task/SCM row, extra/missing component or off-host binary in the Authority MSI fails closed.

`crates/platform/backup/src/postgres16_windows.rs` and `docs/evidence/f57-postgres16-windows-install.v1.schema.json` solely own five strict plain JCS roots: 19-field `Postgres16WindowsPackageLockV1`, 13-field `Postgres16WindowsInstallContractV1`, 4-field `Postgres16WindowsEventLogFixtureSetV1`, 19-field `Postgres16WindowsEventLogScanCoverageV1` and 17-field `Postgres16WindowsInstallReadbackV1`. Their purpose/media pairs are respectively `EP-F57-POSTGRES16-WINDOWS-PACKAGE-LOCK-V1` / `application/vnd.ep.f57-postgres16-windows-package-lock-v1+json`, `EP-F57-POSTGRES16-WINDOWS-INSTALL-CONTRACT-V1` / `application/vnd.ep.f57-postgres16-windows-install-contract-v1+json`, `EP-F57-POSTGRES16-WINDOWS-EVENT-LOG-FIXTURE-SET-V1` / `application/vnd.ep.f57-postgres16-windows-event-log-fixture-set-v1+json`, `EP-F57-POSTGRES16-WINDOWS-EVENT-LOG-SCAN-COVERAGE-V1` / `application/vnd.ep.f57-postgres16-windows-event-log-scan-coverage-v1+json`, and `EP-F57-POSTGRES16-WINDOWS-INSTALL-READBACK-V1` / `application/vnd.ep.f57-postgres16-windows-install-readback-v1+json`. The signed 23-field `WindowsAuthorityArtifactSetV1` authenticates the contract; its embedded six-field Event Log scan contract typed-loads the committed fixture set, while the contract also typed-loads its package lock and its `server_component_set_ref` byte-equals the parent's exact six-row set. The signed 22-field `ReleaseWindowsServiceInstallEvidenceV1` authenticates the live install readback and therefore its exact typed scan-coverage ref; neither Event Log root adds a signer row. No additional installer/service-configuration PowerShell or signer row is added: existing trusted `install-services.ps1` strict-loads the contract, installs it and performs the complete live readback. The listed signed `archive-wal.ps1` and `test-postgres16-pitr.ps1` are closed operational/test wrappers and never configure the service or choose package/path policy.

The package lock is a closed reproducibility and anti-rollback value. `downgrade_allowed` is exactly `false` in V1 and has no policy override. `installed_files` is the complete package/SBOM-bijective vector of regular installed files beneath the engine root, strictly sorted and unique by bytewise UTF-8 `canonical_relative_path`; every path is NFC, uses `/`, is relative with no empty/`.`/`..` component, drive/UNC/device prefix, trailing dot/space, reserved Win32 name or case-fold collision. Each row contains only canonical path, exact reopened byte length and SHA-256; directories, timestamps, archive order, compression and ACL metadata do not enter it, while symlink/reparse, ADS and hard-link aliases are forbidden. `expected_installed_file_set_sha256=SHA256(JCS(installed_files))`. Package construction and engine readback independently enumerate by final handle and must rebuild the byte-identical vector/digest; no exclusion or ambient extra file is legal. V1 admits only a clean first install or idempotent adoption/repair whose existing package-lock ref, package digest, `server_version|server_version_num`, catalog/control versions and installed file-set digest all byte-equal the candidate lock/readback. Any different existing build—older **or newer**—returns `MAINTENANCE_UPGRADE_REQUIRED` before service or data mutation; PostgreSQL upgrade edges are deliberately deferred to a future separately signed maintenance contract and cannot be invented by version comparison. `bundled_extensions` is the exact set derived from the locked package's `share/extension/*.control` filenames and the SBOM, each name matching `[a-z][a-z0-9_]{0,62}`, strictly ASCII-sorted and duplicate-free. Engine readback's available set byte-equals it; configuration freezes strictly sorted, duplicate-free installed/enabled sets, each a subset of bundled, and the two effective sets byte-equal those projections. The observed initdb provider/locale/encoding byte-equal the lock; `cluster_system_identifier|pg_control_system_identifier|sql_system_identifier` and outer `Postgres16WindowsInstallReadbackV1.postgres_system_identifier` are nonempty canonical unsigned decimal and all byte-equal, with the control and SQL values obtained independently from reopened control bytes and the authenticated SQL probe. A path canonicalization/file-set/SBOM drift, locale/encoding/provider/identifier drift, SQL-installed extension outside projection or extension name/case/order drift fails.

`path_projection` contains exactly nine rows, once each and in `Postgres16WindowsPathRoleV1` declaration order. Their canonical paths are `ENGINE_INSTALL_ROOT=C:\Program Files\EnterprisePlatform\PostgreSQL\16`, `PGDATA=<data_root>\postgres\data`, `LIVE_WAL=<data_root>\postgres\data\pg_wal`, `WAL_ARCHIVE_STAGING=<data_root>\postgres\wal`, `PROCESS_TEMP=<data_root>\postgres\temp\process`, `RESTORE_SCRATCH=<data_root>\postgres\temp\restore`, `SERVER_LOG=<data_root>\logs\postgresql`, `TLS_SECRET=<data_root>\secrets\PostgresTls`, and `CONFIGURATION=<data_root>\postgres\data`. Engine is the signed RUNTIME_SSD identity with `customer_authority_bytes_allowed=false`; all other rows are the signed DATA_HDD identity with that flag true. Every row has `reparse_point_allowed|alternate_data_stream_allowed|profile_or_environment_fallback_allowed=false`. Its `canonical_sddl_template` is the exact unresolved row below and `canonical_sddl_template_sha256=SHA256(UTF8(canonical_sddl_template))`; no machine-local SID is required or guessed during build. Only `CONFIGURATION=PGDATA` and `LIVE_WAL` as the exact strict descendant of PGDATA are permitted overlaps; every other equality, alias, ancestor crossing or final-handle collision fails. During service install—after exact account/service creation or adoption—the installer resolves the three placeholders from the same parent evidence's service/task readbacks, deterministically substitutes them, canonicalizes the resolved SDDL, applies it and records the byte-identical `canonical_dacl_sddl` plus `canonical_dacl_sha256=SHA256(UTF8(canonical_dacl_sddl))`. `path_readbacks` has the same exact nine-role order and is a bijective field-for-field observation of that computation: no missing, extra, duplicate, reordered, cross-role, unresolved/mismatched SID, owner/group/inheritance/ACE drift or fallback row is accepted.

| path role | exact protected owner/group/DACL template |
|---|---|
| `ENGINE_INSTALL_ROOT` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001200A9;;;{SID_EP_POSTGRES16})(A;OICI;0x001200A9;;;{SID_EP_F57_RECOVERY})` |
| `PGDATA` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_POSTGRES16})(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})` |
| `LIVE_WAL` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_POSTGRES16})(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})` |
| `WAL_ARCHIVE_STAGING` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_POSTGRES16})(A;OICI;0x001301BF;;;{SID_EP_F57_BACKUP_WRITER})(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})` |
| `PROCESS_TEMP` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_POSTGRES16})(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})` |
| `RESTORE_SCRATCH` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})(A;OICI;0x001200A9;;;{SID_EP_POSTGRES16})` |
| `SERVER_LOG` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001301BF;;;{SID_EP_POSTGRES16})(A;OICI;0x00120089;;;{SID_EP_F57_RECOVERY})` |
| `TLS_SECRET` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x00120089;;;{SID_EP_POSTGRES16})(A;OICI;0x00120089;;;{SID_EP_F57_RECOVERY})` |
| `CONFIGURATION` | `O:SYG:SYD:P(A;OICI;0x001F01FF;;;SY)(A;OICI;0x001F01FF;;;{SID_EP_POSTGRES16})(A;OICI;0x001F01FF;;;{SID_EP_F57_RECOVERY})` |

The placeholders resolve only from exact installed identities `NT SERVICE\ep-postgres16`, the fixed `EPF57Recovery` account, and `NT SERVICE\EPF57BackupWriter`; unresolved/duplicate/changed SID fails before ACL application. ACE order is the displayed order, inheritance is protected, inherited ACE count is zero, and no unlisted allow/deny ACE is tolerated. AccessCheck goldens use ordinary authenticated/Builtin Users, Authority, checkpoint-signer, unlock-broker and other unlisted component tokens against read/write/change-permissions/take-ownership, plus the listed principals at their exact masks; an extra Administrators ACE, broader mask, owner/group change, inheritance, template/hash/string mismatch or one-field access result drift fails.

The embedded Event Log scan contract is executable evidence policy, not a desired-state Boolean. It fixes channel `Application`, the sorted unique provider set `[EnterprisePlatform.PostgreSQL16,PostgreSQL]`, one strict 4-field synthetic-customer-token fixture-set ref plus its exact digest, `maximum_customer_token_match_count=0` and `coverage_complete_required=true`. The fixture set has a nonempty canonical ID and a nonempty UTF-8-byte-sorted/unique token vector; the ref's digest, the separately repeated fixture digest and `SHA256(JCS(fixture_set))` are equal. The installer captures the start bookmark before this attempt's first provider registration/adoption, service mutation or PostgreSQL process start; it captures the end bookmark only after provider registration readback, every fixture exercise, configuration load and the authenticated runtime/client probes. The resulting strict coverage root exact-repeats the contract ref/digest, boot, channel and fixture identities; its start/end bookmarks are nonempty and their record IDs/times are nondecreasing within that one boot. `provider_registration_rows` is exactly the two contract providers, sorted and unique, with canonical final registry path, complete registration digest and installed-file-verified message binary. `expected_fixture_count=exercised_fixture_count=len(fixture tokens)>0`, `customer_token_match_count=0`, and `channel_clear_count|collector_dropped_record_count|unexplained_record_id_gap_count=0`; `scanned_record_count` covers the complete inclusive bookmark interval and `coverage_complete=true`. `service_readback.event_log_scan_coverage_ref` typed-loads only `application/vnd.ep.f57-postgres16-windows-event-log-scan-coverage-v1+json`; a missing provider/record interval/object, bookmark/boot/time reversal, clear, retention truncation, dropped subscription record, unexplained gap, incomplete fixture execution, fixture/ref/digest mismatch, token hit or claimed complete scan without complete coverage fails installation.

Configuration vectors are canonical data, not informal lists. `critical_setting_rows` has exactly one `ascii-lowercase-key=<JCS canonical scalar-or-list>` row for each closed key `archive_command|archive_mode|config_file|data_directory|event_source|fsync|full_page_writes|hba_file|ident_file|listen_addresses|log_destination|log_directory|log_filename|log_rotation_age|log_rotation_size|log_truncate_on_rotation|logging_collector|max_connections|password_encryption|reserved_connections|ssl|ssl_cert_file|ssl_key_file|ssl_min_protocol_version|superuser_reserved_connections|synchronous_commit|temp_tablespaces|wal_level|wal_sync_method`, strictly key-sorted and duplicate-free. Values derive from the referenced signed archive/TLS/path policies and fix loopback-only listening, TLS, SCRAM, `fsync/full_page_writes/synchronous_commit=on`, `wal_level=replica`, `archive_mode=on`, empty temp tablespaces and exact `max_connections=64|reserved_connections=4|superuser_reserved_connections=3`. The typed scalar fields exact-repeat those GUCs as `reserved_connections=migration_connection_reserve=4`, `superuser_reserved_connections=recovery_connection_reserve=3` and `unallocatable_safety_reserve=2`.

On Windows, `wal_sync_method=fsync_writethrough` is a compatibility pin selecting the certified PostgreSQL 16 WAL synchronization path; the setting name, like `fsync=on`, is not by itself evidence that a drive, controller or cache honors durability. Before production activation, the bundled, installed-file-verified `pg_test_fsync.exe` runs `fsync` and `fsync_writethrough` against the same test file beneath the same final-handle DATA_HDD root. `Postgres16WindowsFsyncQualificationV1` must exact-repeat the selected method and volume, say both methods are supported with positive throughput, `fsync_and_fsync_writethrough_same_test_file=true` and `io_error_count=0`, and bind the current storage-driver-stack and write-cache-policy digests. Production durability is established only when Task 15 exact-joins those identities/digests to the same candidate's signed P340 UPS/write-cache policy and controlled HDD flush/power-cut evidence; the textual GUC never substitutes for that chain. A volume, driver-stack, firmware, write-cache policy, UPS policy, tool binary or selected method change makes the qualification and dependent production evidence stale.

Logging is equally closed: `logging_collector=on`, `log_destination=stderr` (therefore never `eventlog`), `log_directory` exact-matches SERVER_LOG on DATA_HDD, `log_filename=postgresql-%Y-%m-%d_%H%M%S.log`, `log_rotation_age=24h`, `log_rotation_size=100MB`, `log_truncate_on_rotation=off`, and both PostgreSQL `event_source` and the `pg_ctl` Windows-service `-e` source in the exact argv are fixed non-customer infrastructure identifiers. Before configuration load, only pg_ctl/early-start infrastructure diagnostics may reach the two contract-registered Windows providers; after configuration load, all server stderr is captured and rotated on DATA_HDD. Only the typed complete Event Log coverage object above—not source names or a zero scalar—proves that no fixture token reached that channel.

The independent signed `Postgres16LogRetentionPolicyV1` is transitively authenticated by the candidate authority artifact set and has no ambient/config/CLI override. V1 fixes `maximum_age_seconds=2592000`, `maximum_total_bytes=21474836480`, `minimum_retained_age_seconds=604800` and `delete_current_log=false`; site legal/contract retention and object legal holds can only retain longer. Final-handle enumeration forms a complete unique vector of current and closed SERVER_LOG objects, counts all bytes once and derives the eligible deletion vector strictly by `(closed_at_unix_ms,canonical_path,sha256)`. Current, younger-than-seven-days and held rows are categorically ineligible. The only executor is a signed two-person-authorized `POSTGRES_LOG_RETENTION_CLEANUP` typed operation in the existing `EPAuthorityControl`; it exact-binds policy, trusted time, preview vector, legal-hold and free-space readbacks, refuses any post-preview drift, deletes only that vector, reopens every survivor/result and commits before/after set digests and the audit receipt. PostgreSQL, backup-writer and ordinary Authority ACL/AccessCheck goldens deny historical delete, ACL/ownership change, hold removal and policy mutation. When held/protected bytes make 30 days and 20 GiB simultaneously impossible, the system preserves them and fails closed rather than deleting protected rows.

Free-space escalation takes the stricter boundary: batch work pauses below `max(existing yellow_free,50 GiB)` and a deployment-wide admission hold begins below `max(existing red_free,40 GiB)`. The P340 formula `reserve=max(20 GiB,capacity*5%)`, `yellow=max(2*reserve,30-day P95 growth)` and the existing 100-GiB platform file floor remain authoritative when higher, so the nominal 1-TiB profile is normally about 100 GiB for bulk pause and 50 GiB for global hold. The 20-GiB log cap is not extra reserve. Boundary goldens prove equality, one byte below, legal-hold overage, cleanup failure and that no new 50/40 floor relaxes an older/higher threshold.

`client_connection_rows` is a controlled enrichment of the pre-build compiled-graph `DatabaseConsumerV1` vector, strictly sorted/unique by `consumer_id`; each source-owned field—`consumer_id|service_identity|database_role|purpose|connection_privilege_class|steady_pool_max|peak_pool_max|acquire_timeout_ms|statement_timeout_ms|capacity_budget_weight`—byte-equals its consumer. `database_name=ep`, source CIDRs are exactly `[127.0.0.1/32,::1/128]`, TLS is required, authentication is `SCRAM_SHA256`, client policy is `client_channel_binding=REQUIRE`, and `0 < steady_pool_max <= peak_pool_max`. Let `N|R|S` be checked sums of `peak_pool_max` for `NORMAL|RESERVED|SUPERUSER`: `N+2<=64-4-3`, `R<=4`, `N+R+2<=64-3`, `S<=3`, and `N+R+S+2<=64`. Every application consumer is `NORMAL`; its role has `rolsuper=false` and no `pg_use_reserved_connections`. Only graph-designated migration consumers may be `RESERVED`, with `rolsuper=false` and effective membership in `pg_use_reserved_connections`; only the recovery consumer may be `SUPERUSER`. Thus normal applications cannot consume the four reserved or three superuser slots, migration remains able to acquire its four slots after normal saturation, recovery remains able to acquire its three after reserved saturation, and the two-slot safety margin has no consumer. Runtime topology and P340 remeasure that same candidate without feeding a future ref into this contract.

`effective_critical_setting_rows` byte-equals the complete vector. `hba_rows|ident_rows` preserve semantic file order as contiguous zero-padded `00000:<canonical parsed row>` entries and are the complete parser output of the exact referenced bytes; HBA is nonempty, ident may be empty only when its referenced file parses empty, and effective vectors byte-equal projections. Every HBA row exact-projects one client row only as database `ep`, the two loopback CIDRs, record type `hostssl` and method `scram-sha-256`; HBA does not and cannot prove libpq channel binding. `authenticated_client_probe_rows` is instead a bijection with client rows and proves TLS, SCRAM, client-side `channel_binding=require`, negotiated channel binding, the authenticated session digest and observed role attributes; runtime `authenticated_probe_sha256=SHA256(JCS(authenticated_client_probe_rows))`. `trust`, plain `host`, external CIDR, duplicate/case alias, unknown include, ambient override, client-policy drift or an unbound/missing probe is invalid. Runtime `listen_addresses` is exactly sorted unique `127.0.0.1|::1`. Schema/byte goldens mutate file/SBOM/DACL fields, client source/class/role attributes, each classified budget boundary, either reserved GUC, channel-binding policy/probe while leaving HBA unchanged, HBA `hostssl`/SCRAM, Event Log contract/fixture/provider/bookmark/count/coverage, logging/rotation, WAL compatibility pin or either same-file fsync qualification, identifiers/extensions/paths/effective equality/ambient includes/existing lock; every mutation fails closed.

The service readback exact-matches product sources `observed_pg_ctl_event_source=EnterprisePlatform.PostgreSQL16` and `observed_postgres_event_source=EnterprisePlatform.PostgreSQL16` plus PostgreSQL's only permitted pre-parameter fallback `observed_early_fallback_event_source=PostgreSQL`, and requires `server_eventlog_destination_enabled=false`. Its `event_log_scan_coverage_ref` strict-loads the 19-field coverage root and exact-joins the install contract's scan-contract digest, one boot, the exact two provider-registration rows, nondecreasing start/end bookmarks and times, complete record interval, the committed fixture ref/digest and all zero clear/drop/gap/token counters. An absent, truncated, cleared, dropped, gapped, cross-boot, cross-contract, wrong-provider, wrong-fixture, digest-mismatched or `coverage_complete=false` capture fails; desired configuration or a zero scalar alone is not evidence.

Runtime readback accepts only typed `service_state=RUNNING`; an arbitrary SCM string, pending/stopped state or unknown value cannot satisfy readiness. Its held image identity exact-joins engine readback, token SID exact-joins the resolved service row, listener count equals the canonical loopback vector length, and both nonloopback counters are zero.

The sole PostgreSQL row is `ep-postgres16` / `Enterprise Platform PostgreSQL 16` / `NT SERVICE\ep-postgres16`, `UNRESTRICTED`, `DEMAND_START`, `NO_AUTOMATIC_RESTART`, `WIN32_OWN_PROCESS`, normal error control, not delayed. It executes the package-locked SSD binary `C:\Program Files\EnterprisePlatform\PostgreSQL\16\bin\pg_ctl.exe` with exact argv `[runservice,-N,ep-postgres16,-D,<signed-PGDATA>,-e,EnterprisePlatform.PostgreSQL16,-w]`, while configuration fixes `event_source=EnterprisePlatform.PostgreSQL16`; dependencies are `[CryptSvc,EPF57DataVolumeUnlockBroker,EventLog,KeyIso,RpcSs]` and privilege is `[SeChangeNotifyPrivilege]`. SYSTEM alone has full service control; Authority receives only query/start `0x00020015`, Recovery only query/start/stop `0x00020035`, and neither can change config or delete. Dependency order is not readiness: Authority may explicitly start PostgreSQL only after the same-boot DATA_HDD unlock, storage-manifest, vault, configuration and TLS-policy reads pass; pre-HDD process count and non-loopback outbound socket count are both zero.

The engine is SSD Set A, while PGDATA (including live `pg_wal` and database temporary relations), WAL archive staging, process/restore scratch, server logs, TLS material and effective config all resolve by final handle to their signed DATA_HDD roots. `postgres/wal` is archive staging only and `postgres/temp` is process/restore scratch only. V1 forbids `initdb --waldir`, user tablespaces, reparse descendants, alternate config includes and `postgresql.auto.conf` overrides. Package lock/config projection fixes the PostgreSQL 16 build/signature/SBOM-bijective installed-file vector/locale/encoding/checksums plus loopback `hostssl`/SCRAM HBA, separate authenticated client `channel_binding=require`, exact `max_connections=64|reserved_connections=4|superuser_reserved_connections=3`, classified role budgets, `fsync=on`, `full_page_writes=on`, `synchronous_commit=on`, `wal_sync_method=fsync_writethrough` as a compatibility pin, same-file `fsync` plus `fsync_writethrough` DATA_HDD qualification, `wal_level=replica`, `archive_mode=on`, the signed archive executor, collector-backed rotated HDD logs with no server eventlog destination, no `trust`, no external CIDR and `temp_tablespaces=''`; both exact files and parsed effective values are read back. Only Task 15's exact join to current P340 driver/cache/UPS plus controlled HDD flush/power-cut evidence completes the physical-durability claim. PostgreSQL remains outside the six backup components and nine product-owned SCM rows, so the host formula remains `10 + active_additional_windows_service_count`.

`Postgres16WindowsInstallReadbackV1.tls_readback_ref` and `.firewall_readback_ref` each fix `application/octet-stream` in `EvidenceObjectStoreV1`, decoded only by this owner's closed field parsers `EP_F57_POSTGRES16_TLS_READBACK_V1|EP_F57_POSTGRES16_FIREWALL_READBACK_V1`. TLS output exact-binds contract/policy, certificate SPKI/chain and the nonexportable key provider/service-SID ACL with zero unauthorized access. It also exact-binds every typed authenticated-client probe to its consumer/session: TLS established, SCRAM authentication, client policy `channel_binding=require`, negotiated channel binding, session-binding digest and observed `rolsuper`/`pg_use_reserved_connections` attributes for the declared `NORMAL|RESERVED|SUPERUSER` class. HBA output proves only `hostssl` plus `scram-sha-256` and cannot stand in for the client probe. Firewall output exact-binds installed `postgres.exe`, service SID and loopback endpoints with zero external inbound allow, nonloopback listener/outbound socket or unexpected rule. Both parsed outputs exact-equal the typed configuration/runtime scalars, including `authenticated_probe_sha256=SHA256(JCS(authenticated_client_probe_rows))`; their hashes are authenticated by the signed strict install-readback parent, so neither gains a purpose/schema/signature row. Missing/unbound probe bytes, cross-contract capture, generic-parser bytes or desired-policy echo fails.

The third service is fixed as `EPF57DataVolumeUnlockBroker` / `Enterprise Platform F57 Data Volume Unlock Broker` / `LocalSystem`, executable `C:\Program Files\EnterprisePlatform\Authority\data-volume-unlock-broker.exe`, argv `[--service-mode,windows-scm,--component,DATA_VOLUME_UNLOCK_BROKER_SERVICE]`, source `apps/data-volume-unlock-broker/src/main.rs#windows_service_main`, `RESTRICTED`, `AUTO_START`, `RESTART_ON_FAILURE_MAX_THREE`, dependencies `[BDESVC,CryptSvc,EventLog,KeyIso,RpcSs,Winmgmt]`, privileges `[SeChangeNotifyPrivilege,SeManageVolumePrivilege]`, endpoint `\\.\pipe\EnterprisePlatform.F57.DataVolumeUnlockBroker.v1`, capability `[UNLOCK_BOUND_DATA_HDD]`, `outbound_network_allowed=false` and `data_volume_identity_binding_required=true`. Its service SDDL is `O:SYG:SYD:P(A;;0x000F01FF;;;SY)(A;;0x00020005;;;{SID_EP_AUTHORITY_SERVER})(A;;0x00020005;;;{SID_EP_F57_RECOVERY})`; its pipe is SYSTEM-owned and grants SYSTEM plus the broker service SID full instance rights and AS only concrete client mask `0x00120183`; its executable is SYSTEM-owned and grants the component read/execute and AS/RT read only. Runtime readback proves token user SID `S-1-5-18`, distinct service and dynamic logon SIDs, `RESTRICTED`, the canonical restricted-SID set/digest, exact privileges, final-handle executable/DACL, SCM command line parsed back to the fixed argv, pipe DACL, zero outbound sockets and the one-volume capability. RT may query but cannot start/stop the unlock broker, which is the sole `KEEP_RUNNING` component across full-cut recovery.

DATA_HDD BitLocker uses the exact protector set `{PUBLIC_KEY,RECOVERY_PASSWORD}` and never `EXTERNAL_KEY` or Windows fixed-data auto-unlock. The PUBLIC_KEY protector is bound to a nonexportable TPM-backed certificate key in `Microsoft Platform Crypto Provider`; normal reboot reopens that existing key and calls only `UnlockWithCertificateThumbprint` through the restricted LocalSystem broker. The pipe accepts only `BEGIN_OR_ADOPT_UNLOCK|QUERY_UNLOCK` and request `{schema_version,operation_id,windows_boot_id,deployment_id,authority_epoch,unlock_authority_ref,expected_data_volume_identity}`. Callers cannot supply a path, WQL, server, namespace, object path, thumbprint, protector, method, provider, volume, PIN or credential. The broker uses `LocalMachine\My`, one explicit nonzero thumbprint, empty PIN, local `ROOT\CIMV2\Security\MicrosoftVolumeEncryption`, packet privacy, impersonate-without-delegation and the compiled volume identity. The namespace policy is `MERGE_REQUIRED_ACE_INVARIANT`: preserve required OS/provider ACEs, merge the broker SID with `WBEM_ENABLE|WBEM_METHOD_EXECUTE` (`0x00000003`), forbid its remaining rights, and read it back; the fixed signed binary/WDAC and negative tests—not a fictitious per-method ACL—enforce the sole WMI method. Restricted-LocalSystem WS2022 qualification must observe enabled normal-token Administrators SID, method return code `0`, protector type `7`, certificate type `2`, exact certificate/volume, unlocked+mounted+dirty-clear, zero export/leak and `windows_fixed_data_auto_unlock_enabled=false`, otherwise this profile cannot ship.

The pre-HDD locator policy is generated, immutable Set A and exactly nine rows: `C:\Program Files\EnterprisePlatform\Authority\bootstrap\data-hdd-unlock-trust-registry.v1.json`; `C:\ProgramData\EnterprisePlatform\Bootstrap\data-hdd-unlock-authority.v1.json`; `C:\ProgramData\EnterprisePlatform\Bootstrap\data-hdd-unlock\objects\sha256`; and under `C:\Program Files\EnterprisePlatform\Authority\bootstrap\trust\`, `registry-trust-bundle.p7b`, `registry-revocation.bin`, `registry-checkpoint.bin`, `unlock-certificate-ca-chain.p7b`, `unlock-certificate-revocation.bin`, and `unlock-certificate-checkpoint.bin`. Registry/authority/public-object limits are `1048576/1048576/16777216`; each trust bundle/revocation snapshot is at most `4194304` and each checkpoint `65536`. Final-handle resolution must prove the RUNTIME_SSD, exact media/digests/descriptors, zero reparse/ADS, legal hard links and zero unregistered locators. `DataHddBitLockerCertificatePolicyV1` fixes the four 64-bit FVE values `FDVAllowUserCert=1`, `FDVEnforceUserCert=0`, `CertificateOID=1.3.6.1.4.1.311.67.1.1`, and `IdentificationField=EP-F57-<deployment-id>`, the same EKU, exact KeyUsage `[DATA_ENCIPHERMENT,KEY_ENCIPHERMENT]`, exclusive pinned offline CA chain/revocation/checkpoint, no URL retrieval, hardware-backed nonexportable key and zero ambient-root completion. `crates/platform/backup/src/windows_components/unlock.rs` plus `docs/evidence/f57-data-hdd-bitlocker-unlock.v1.schema.json` are the sole wire/schema owners for locator, trust registry, policy/readbacks, broker bootstrap readback, signed unlock authority and strict plain unlock readback.

The fixed G0 `EPF57EvidenceSignerBroker` is a ninth EnterprisePlatform-owned SCM row outside both the five Authority roles and six backup components. Task 11 byte-imports and install/adopts its signed package/static row, collects the complete `F57EvidenceSignerBrokerWindowsInstallReadbackV1`, and places the ref in `ReleaseWindowsServiceInstallEvidencePayloadV1.evidence_signer_broker_install_readback_ref`. The row is `AUTO_START`, `UNRESTRICTED`, `RESTART_ON_FAILURE_MAX_THREE`, dependencies `[CryptSvc,EventLog,KeyIso,RpcSs]`, privilege `[SeChangeNotifyPrivilege]`, endpoint `\\.\pipe\EnterprisePlatform\F57EvidenceSignerV1`; its client group gets only `0x00120183`. The readback closes static row digest, final-handle/AuthentiCode, parsed ImagePath/argv, account/service/group/member SIDs, all four resolved ACLs and hashes, active session/registry, nonce/session/process identity, provider/readiness, DATA_HDD manifest/volume/root/state closure, outbound policy and `runtime_ssd_mutable_fallback_count=0`. Before DATA_HDD is ready it may only return `WAITING_FOR_DATA_HDD/NOT_READY`. Fixed EnterprisePlatform inventory is therefore exactly nine SCM rows, one product Scheduled Task and one on-demand executable; complete host inventory is `10 + active_additional_windows_service_count` after adding pinned `ep-postgres16` and graph-active non-aliased Windows services.

After their final edits, both Task-11 scripts are signed/timestamped and their descriptors are verified before the rehearsal. PostgreSQL archive execution and the PITR wrapper call only the registered fixed-host executor by the two closed script IDs; neither a PostgreSQL setting nor a caller may provide a script path or command.

- [ ] **Step 1: Write failing no-local-copy and recovery-only tests.**

```rust
#[test]
fn backup_writer_cannot_decrypt_history_or_delete_target() {
    assert!(backup_writer_capabilities().is_disjoint(&["BACKUP_DECRYPT", "BACKUP_DELETE"]));
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_backup_target --test f57_backup_envelope --test f57_backup_storage_safeguard --test f57_backup_topology_signing_trust --test f57_backup_checkpoint_transition --test f57_postgres16_windows_install --test f57_postgres16_recovery -- --nocapture`

Run: `cargo test -p backup-writer --test backup_composition --locked`

Run: `cargo test -p backup-checkpoint-signer --test checkpoint_composition --locked`

Expected: FAIL.

- [ ] **Step 3: Implement the fixed streaming pipeline.**

```rust
pub type BackupPortFutureV1<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, BackupErrorV1>> + Send + 'a>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BackupCheckpointPurposeV1 {
    #[serde(rename = "EP-F57-BACKUP-CHECKPOINT-V1")]
    EpF57BackupCheckpointV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct BackupSetIdV1(UuidV1);

pub struct VerifiedBackupSetRequestV1 {
    backup_set_id: BackupSetIdV1,
    source_identity_sha256: Sha256Digest,
    sink_identity_sha256: Sha256Digest,
    recovery_dek_envelope_ref: ArtifactRefV1,
    maximum_plaintext_chunk_bytes: u64,
    requested_at_unix_ms: i64,
}

pub struct BackupSourceSessionV1 {
    backup_set_id: BackupSetIdV1,
    source_session_id: UuidV1,
    source_identity_sha256: Sha256Digest,
    maximum_plaintext_chunk_bytes: u64,
    started_at_unix_ms: i64,
}

pub struct BackupPlaintextChunkV1 {
    backup_set_id: BackupSetIdV1,
    source_session_id: UuidV1,
    sequence: u64,
    plaintext_sha256: Sha256Digest,
    plaintext: Zeroizing<Vec<u8>>,
    is_final: bool,
}

pub struct BackupSourceCutV1 {
    backup_set_id: BackupSetIdV1,
    source_session_id: UuidV1,
    final_sequence: u64,
    postgres_checkpoint_lsn: u64,
    attachment_manifest_ref: ArtifactRefV1,
    source_cut_ref: ArtifactRefV1,
}

pub struct BackupCiphertextChunkV1 {
    backup_set_id: BackupSetIdV1,
    sequence: u64,
    plaintext_sha256: Sha256Digest,
    nonce: [u8; 24],
    ciphertext_sha256: Sha256Digest,
    ciphertext: Vec<u8>,
}

pub struct BackupAppendReceiptV1 {
    backup_set_id: BackupSetIdV1,
    sequence: u64,
    ciphertext_sha256: Sha256Digest,
    target_object_ref: ArtifactRefV1,
    receipt_ref: ArtifactRefV1,
}

pub struct BackupChunkReadbackV1 {
    backup_set_id: BackupSetIdV1,
    sequence: u64,
    ciphertext_sha256: Sha256Digest,
    target_object_ref: ArtifactRefV1,
    readback_ref: ArtifactRefV1,
}

pub struct BackupSinkSealV1 {
    backup_set_id: BackupSetIdV1,
    ordered_chunk_receipt_refs: Vec<ArtifactRefV1>,
    sink_identity_sha256: Sha256Digest,
    seal_ref: ArtifactRefV1,
}

pub trait BackupAdapterDtoV1 {
    fn backup_set_id(&self) -> &BackupSetIdV1;
    fn canonical_identity_bytes(&self) -> Result<Vec<u8>, BackupErrorV1>;
}

impl BackupSetIdV1 {
    pub fn as_uuid(&self) -> UuidV1 { self.0 }
}

impl BackupSourceSessionV1 {
    pub fn try_from_source_readback(
        request: &VerifiedBackupSetRequestV1,
        source_session_id: UuidV1,
        source_identity_sha256: Sha256Digest,
        maximum_plaintext_chunk_bytes: u64,
        started_at_unix_ms: i64,
    ) -> Result<Self, BackupErrorV1>;
    pub fn set_id(&self) -> &BackupSetIdV1 { &self.backup_set_id }
}

impl BackupPlaintextChunkV1 {
    pub fn try_from_bounded_source_read(
        session: &BackupSourceSessionV1,
        sequence: u64,
        plaintext: Zeroizing<Vec<u8>>,
        is_final: bool,
    ) -> Result<Self, BackupErrorV1>;
    pub fn sequence(&self) -> u64 { self.sequence }
    pub fn plaintext(&self) -> &[u8] { self.plaintext.as_slice() }
}

impl BackupSourceCutV1 {
    pub fn try_from_frozen_source_readback(
        session: &BackupSourceSessionV1,
        final_sequence: u64,
        postgres_checkpoint_lsn: u64,
        attachment_manifest_ref: ArtifactRefV1,
        source_cut_ref: ArtifactRefV1,
    ) -> Result<Self, BackupErrorV1>;
    pub fn source_cut_ref(&self) -> &ArtifactRefV1 { &self.source_cut_ref }
}

impl BackupAppendReceiptV1 {
    pub fn try_from_append_readback(
        chunk: &BackupCiphertextChunkV1,
        target_object_ref: ArtifactRefV1,
        receipt_ref: ArtifactRefV1,
    ) -> Result<Self, BackupErrorV1>;
    pub fn receipt_ref(&self) -> &ArtifactRefV1 { &self.receipt_ref }
}

impl BackupChunkReadbackV1 {
    pub fn try_from_exact_target_readback(
        receipt: &BackupAppendReceiptV1,
        ciphertext_sha256: Sha256Digest,
        readback_ref: ArtifactRefV1,
    ) -> Result<Self, BackupErrorV1>;
}

impl BackupSinkSealV1 {
    pub fn try_from_immutable_sink_readback(
        request: &VerifiedBackupSetRequestV1,
        ordered_chunk_receipt_refs: Vec<ArtifactRefV1>,
        sink_identity_sha256: Sha256Digest,
        seal_ref: ArtifactRefV1,
    ) -> Result<Self, BackupErrorV1>;
    pub fn seal_ref(&self) -> &ArtifactRefV1 { &self.seal_ref }
}

impl VerifiedBackupSetRequestV1 {
    pub fn set_id(&self) -> &BackupSetIdV1 { &self.backup_set_id }
    pub fn source_identity_sha256(&self) -> Sha256Digest { self.source_identity_sha256 }
    pub fn sink_identity_sha256(&self) -> Sha256Digest { self.sink_identity_sha256 }
    pub fn recovery_dek_envelope_ref(&self) -> &ArtifactRefV1 { &self.recovery_dek_envelope_ref }
    pub fn maximum_plaintext_chunk_bytes(&self) -> u64 { self.maximum_plaintext_chunk_bytes }
}

impl BackupCiphertextChunkV1 {
    pub fn sequence(&self) -> u64 { self.sequence }
    pub fn ciphertext_sha256(&self) -> Sha256Digest { self.ciphertext_sha256 }
    pub fn ciphertext(&self) -> &[u8] { &self.ciphertext }
}

pub struct VerifiedRecoveryOnlyBackupDekEnvelopeV1 {
    artifact_ref: ArtifactRefV1,
    recovery_domain_identity_sha256: Sha256Digest,
}

pub struct BackupSetRequestAuthorityV1;

pub trait BackupSetRequestMintV1 {
    fn verify_and_mint(
        &self,
        backup_set_id: BackupSetIdV1,
        source_identity_sha256: Sha256Digest,
        sink_identity_sha256: Sha256Digest,
        recovery_dek: &VerifiedRecoveryOnlyBackupDekEnvelopeV1,
        maximum_plaintext_chunk_bytes: u64,
        requested_at_unix_ms: i64,
    ) -> Result<VerifiedBackupSetRequestV1, BackupErrorV1>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BackupErrorV1 {
    Source,
    ChunkTooLarge,
    Sequence,
    Encryption,
    Append,
    Readback,
    Digest,
    Seal,
    CheckpointConflict,
    Signer,
    Signature,
    RecoveryDomain,
    Storage,
    Safeguard,
}

pub trait BackupChunkEncryptorV1: Send + Sync {
    fn encrypt_and_authenticate(
        &self,
        request: &VerifiedBackupSetRequestV1,
        chunk: BackupPlaintextChunkV1,
    ) -> Result<BackupCiphertextChunkV1, BackupErrorV1>;
}

pub trait BackupCheckpointSignPortV1: Send + Sync {
    fn sign_checkpoint<'a>(
        &'a self,
        operation: &'a VerifiedBackupCheckpointSigningOperationV1,
    ) -> BackupPortFutureV1<'a, Vec<u8>>;
}

pub trait BackupCheckpointVerifyPortV1: Send + Sync {
    fn verify_checkpoint(
        &self,
        exact_envelope_jcs: &[u8],
        expected_ref: &ArtifactRefV1,
    ) -> Result<VerifiedBackupCheckpointV1, BackupErrorV1>;
}

pub struct BackupAuthorityV1<E> {
    verified_request: VerifiedBackupSetRequestV1,
    encryptor: E,
}

impl<E: BackupChunkEncryptorV1> BackupAuthorityV1<E> {
    pub fn compose(
        verified_request: VerifiedBackupSetRequestV1,
        encryptor: E,
    ) -> Self {
        Self { verified_request, encryptor }
    }

    pub fn verified_request(&self) -> &VerifiedBackupSetRequestV1 {
        &self.verified_request
    }
}

pub trait BackupSourceV1: Send {
    fn begin_or_adopt_set<'a>(&'a mut self, request: &'a VerifiedBackupSetRequestV1)
        -> BackupPortFutureV1<'a, BackupSourceSessionV1>;
    fn next_bounded_chunk<'a>(&'a mut self, session: &'a BackupSourceSessionV1)
        -> BackupPortFutureV1<'a, Option<BackupPlaintextChunkV1>>;
    fn freeze_source_cut<'a>(&'a mut self, session: &'a BackupSourceSessionV1)
        -> BackupPortFutureV1<'a, BackupSourceCutV1>;
}

pub struct VerifiedJustAppendedObjectV1 {
    /* private affine receipt + one-time exact-object read capability, or persisted readback adoption */
}

pub trait AppendOnlySinkV1: Send {
    fn append_create_or_adopt<'a>(&'a mut self, chunk: &'a BackupCiphertextChunkV1)
        -> BackupPortFutureV1<'a, VerifiedJustAppendedObjectV1>;
    fn readback_exact<'a>(&'a mut self, just_appended: VerifiedJustAppendedObjectV1)
        -> BackupPortFutureV1<'a, BackupChunkReadbackV1>;
    fn seal_immutable_set<'a>(&'a mut self, set_id: &'a BackupSetIdV1)
        -> BackupPortFutureV1<'a, BackupSinkSealV1>;
}

pub trait BackupCheckpointDraftStoreV1: Send {
    fn create_ready_or_adopt<'a>(
        &'a mut self,
        draft: BackupCheckpointDraftPayloadV1,
    ) -> BackupPortFutureV1<'a, VerifiedBackupCheckpointReadyDraftV1>;
}

pub trait BackupCheckpointObjectStoreV1: Send {
    fn create_or_adopt<'a>(&'a mut self, set_id: &'a BackupSetIdV1, exact_envelope: &'a [u8])
        -> BackupPortFutureV1<'a, ArtifactRefV1>;
    fn load_exact<'a>(&'a mut self, set_id: &'a BackupSetIdV1, expected: &'a ArtifactRefV1)
        -> BackupPortFutureV1<'a, Vec<u8>>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupCheckpointSigningStageV1 {
    OperationCommitted,
    EnvelopeSpooled,
    ObjectBound,
}

pub struct VerifiedBackupCheckpointReadyDraftV1 { /* private exact draft/ref/READY checkpoint */ }
pub struct VerifiedBackupCheckpointSigningOperationV1 { /* private exact operation + payload */ }

pub trait BackupCheckpointSigningAttemptStoreV1: Send {
    fn commit_or_adopt_operation<'a>(
        &'a mut self,
        draft: &'a VerifiedBackupCheckpointReadyDraftV1,
    ) -> BackupPortFutureV1<'a, VerifiedBackupCheckpointSigningOperationV1>;
    fn spool_or_adopt_exact_envelope<'a>(
        &'a mut self,
        operation: &'a VerifiedBackupCheckpointSigningOperationV1,
        exact_envelope_jcs: &'a [u8],
    ) -> BackupPortFutureV1<'a, ArtifactRefV1>;
    fn bind_or_adopt_object<'a>(
        &'a mut self,
        operation: &'a VerifiedBackupCheckpointSigningOperationV1,
        envelope_spool_ref: &'a ArtifactRefV1,
        checkpoint_object_ref: &'a ArtifactRefV1,
    ) -> BackupPortFutureV1<'a, ()>;
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "checkpoint_context_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum BackupCheckpointContextV1 {
    Scheduled {
        schedule_identity_sha256: Sha256Digest,
    },
    PackageMaintenance {
        maintenance_reservation_ref: ArtifactRefV1,
        recovery_checkpoint_policy_ref: ArtifactRefV1,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BackupTopologyPurposeV1 {
    #[serde(rename = "EP-F57-BACKUP-TOPOLOGY-V1")]
    BackupTopology,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BackupTopologySigningTrustManifestPurposeV1 {
    #[serde(rename = "EP-F57-BACKUP-TOPOLOGY-SIGNING-TRUST-MANIFEST-V1")]
    BackupTopologySigningTrustManifest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum BackupTopologySigningTrustCurrentPointerPurposeV1 {
    #[serde(rename = "EP-F57-BACKUP-TOPOLOGY-SIGNING-TRUST-CURRENT-POINTER-V1")]
    BackupTopologySigningTrustCurrentPointer,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupTopologySigningTrustManifestPayloadV1 {
    pub schema_version: u32,
    pub purpose: BackupTopologySigningTrustManifestPurposeV1,
    pub trust_manifest_id: UuidV1,
    pub deployment_id: UuidV1,
    pub trust_generation: u64,
    pub predecessor_trust_manifest_ref: Option<ArtifactRefV1>,
    pub topology_signer_subject_dn: String,
    pub topology_signer_spki: SignerSpkiTokenV1,
    pub topology_signer_offline_certificate_chain_ref: ArtifactRefV1,
    pub topology_signer_offline_revocation_snapshot_ref: ArtifactRefV1,
    pub topology_signer_offline_transparency_checkpoint_ref: ArtifactRefV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type BackupTopologySigningTrustManifestV1 =
    SignedBusinessArtifactV1<BackupTopologySigningTrustManifestPayloadV1>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupTopologySigningTrustCurrentPointerPayloadV1 {
    pub schema_version: u32,
    pub purpose: BackupTopologySigningTrustCurrentPointerPurposeV1,
    pub current_pointer_id: UuidV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub pointer_generation: u64,
    pub trust_generation: u64,
    pub current_trust_manifest_ref: ArtifactRefV1,
    pub predecessor_current_pointer_ref: Option<ArtifactRefV1>,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type BackupTopologySigningTrustCurrentPointerV1 =
    SignedBusinessArtifactV1<BackupTopologySigningTrustCurrentPointerPayloadV1>;

pub struct VerifiedBackupTopologySigningTrustCurrentV1 {
    current_pointer:
        VerifiedBusinessArtifactV1<BackupTopologySigningTrustCurrentPointerPayloadV1>,
    current_manifest: VerifiedBusinessArtifactV1<BackupTopologySigningTrustManifestPayloadV1>,
    active_configuration_generation: u64,
    verified_at_unix_ms: i64,
}

impl VerifiedBackupTopologySigningTrustCurrentV1 {
    pub fn current_pointer(
        &self,
    ) -> &VerifiedBusinessArtifactV1<BackupTopologySigningTrustCurrentPointerPayloadV1> {
        &self.current_pointer
    }
    pub fn current_manifest(
        &self,
    ) -> &VerifiedBusinessArtifactV1<BackupTopologySigningTrustManifestPayloadV1> {
        &self.current_manifest
    }
    pub fn active_configuration_generation(&self) -> u64 {
        self.active_configuration_generation
    }
    pub fn verified_at_unix_ms(&self) -> i64 { self.verified_at_unix_ms }
}

pub struct BackupTopologyAuthorityV1 {
    current_signing_trust: VerifiedBackupTopologySigningTrustCurrentV1,
}

impl BackupTopologyAuthorityV1 {
    pub fn compose(current_signing_trust: VerifiedBackupTopologySigningTrustCurrentV1) -> Self {
        Self { current_signing_trust }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StorageSafeguardReadbackPurposeV1 {
    #[serde(rename = "EP-F57-STORAGE-SAFEGUARD-READBACK-V1")]
    StorageSafeguardReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum StorageSafeguardSupportEvidencePurposeV1 {
    #[serde(rename = "EP-F57-STORAGE-SAFEGUARD-SUPPORT-EVIDENCE-V1")]
    StorageSafeguardSupportEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupTargetSupportReceiptKindV1 {
    CapacityCalculation,
    QuotaExhaustion,
    EmergencyReserve,
    PartialTimeout,
    ExpiredPartialCleanup,
    BulkWorkSuspension,
    PermissionDenial,
    RetentionPolicy,
    JustWrittenCreate,
    JustWrittenExactReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfflineMediaObservationKindV1 { SafeEject, PhysicalDisconnection, Custody, Health }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageSafeguardSupportOutcomeV1 { Passed, Denied }

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "support_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum StorageSafeguardSupportPayloadV1 {
    TargetReceipt {
        receipt_kind: BackupTargetSupportReceiptKindV1,
        backup_topology_ref: ArtifactRefV1,
        target_id: String,
        collector_challenge_nonce_lowerhex: String,
        collector_session_binding_sha256: Sha256Digest,
        probe_id: UuidV1,
        subject_projection_sha256: Sha256Digest,
        state_before_sha256: Sha256Digest,
        state_after_sha256: Sha256Digest,
        outcome: StorageSafeguardSupportOutcomeV1,
        observed_at_unix_ms: i64,
        expires_at_unix_ms: i64,
    },
    OfflineMediaTransition {
        backup_topology_ref: ArtifactRefV1,
        media_role: OfflineMediaRoleV1,
        media_id: String,
        transition_sequence: u64,
        predecessor_transition_ref: Option<ArtifactRefV1>,
        previous_state: Option<OfflineMediaStateV1>,
        current_state: OfflineMediaStateV1,
        subject_projection_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    OfflineMediaObservation {
        evidence_kind: OfflineMediaObservationKindV1,
        backup_topology_ref: ArtifactRefV1,
        state_transition_ref: ArtifactRefV1,
        media_role: OfflineMediaRoleV1,
        media_id: String,
        transition_sequence: u64,
        current_state: OfflineMediaStateV1,
        collector_challenge_nonce_lowerhex: String,
        collector_session_binding_sha256: Sha256Digest,
        subject_projection_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
        expires_at_unix_ms: i64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorageSafeguardSupportSignatureV1 {
    pub signer_principal_id: String,
    pub signer_spki: SignerSpkiTokenV1,
    pub detached_cms_der_base64: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorageSafeguardSupportEvidenceV1 {
    pub schema_version: u32,
    pub purpose: StorageSafeguardSupportEvidencePurposeV1,
    pub payload: StorageSafeguardSupportPayloadV1,
    pub signatures: Vec<StorageSafeguardSupportSignatureV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupRoleV1 {
    BackupWriter,
    BackupTargetAgent,
    BackupCheckpointSigner,
    RetentionCustodian,
    RecoveryCustodian,
    PartialObjectMaintenance,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupTransportProfileV1 { HttpsMutualTls13V1 }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupPhysicalMediaV1 { Hdd }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfflineMediaRoleV1 { RotationA, RotationB }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfflineMediaStateV1 {
    Blank,
    Enrolled,
    ActiveAppend,
    VerifiedDisconnected,
    RotationDue,
    SealedVerified,
    RetiredPendingDisposal,
    Destroyed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupProtectionStateV1 {
    Initializing,
    Bootstrapping,
    Transitioning,
    Healthy,
    NonSuppressibleRisk,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupProtectionTransitionKindV1 {
    InitialPopulation,
    CurrentRootsRotation,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupProtectionTransitionV1 {
    pub transition_kind: BackupProtectionTransitionKindV1,
    pub transition_id: UuidV1,
    pub transition_generation: GenerationNumberV1,
    pub pre_transition_healthy_safeguard_readback_ref: Option<ArtifactRefV1>,
    pub pre_transition_checkpoint_head_ref: Option<ArtifactRefV1>,
    pub prior_backup_topology_ref: Option<ArtifactRefV1>,
    pub prior_authority_storage_manifest_ref: Option<ArtifactRefV1>,
    pub prior_topology_signing_trust_current_ref: Option<ArtifactRefV1>,
    pub target_backup_topology_ref: ArtifactRefV1,
    pub target_authority_storage_manifest_ref: ArtifactRefV1,
    pub target_topology_signing_trust_current_ref: ArtifactRefV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupRoleBindingV1 {
    pub role: BackupRoleV1,
    pub principal_id: String,
    pub credential_identity_sha256: Sha256Digest,
    pub credential_domain_id: String,
    pub administration_domain_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupVolumePolicyV1 {
    pub physical_media: BackupPhysicalMediaV1,
    pub partition_style: PartitionStyleV1,
    pub filesystem: FileSystemKindV1,
    pub bitlocker_encryption_method: BitLockerEncryptionMethodV1,
    pub expected_bitlocker_protector_kinds: Vec<BitLockerProtectorKindV1>,
    pub required_encryption_percentage: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupTargetQuotaPolicyV1 {
    pub maximum_object_count: u64,
    pub maximum_stored_bytes: u64,
    pub maximum_ingress_bytes_per_second: u64,
    pub maximum_concurrent_uploads: u32,
    pub writer_unavailable_emergency_reserve_bytes: u64,
    pub partial_object_timeout_seconds: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuousBackupTargetTopologyV1 {
    pub target_id: String,
    pub target_host_fingerprint_sha256: Sha256Digest,
    pub endpoint_origin: String,
    pub tls_server_name: String,
    pub transport_profile: BackupTransportProfileV1,
    pub mtls_trust_policy_ref: ArtifactRefV1,
    pub writer_client_spki: SignerSpkiTokenV1,
    pub target_server_spki: SignerSpkiTokenV1,
    pub target_volume_identity: String,
    pub volume_policy: BackupVolumePolicyV1,
    pub failure_domain_id: String,
    pub administration_domain_id: String,
    pub credential_domain_id: String,
    pub quota_policy: BackupTargetQuotaPolicyV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineMediaCustodianBindingV1 {
    pub principal_id: String,
    pub signer_spki: SignerSpkiTokenV1,
    pub administration_domain_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineMediaTopologyRowV1 {
    pub media_role: OfflineMediaRoleV1,
    pub media_id: String,
    pub hardware_serial_sha256: Sha256Digest,
    pub volume_identity: String,
    pub volume_guid: String,
    pub volume_policy: BackupVolumePolicyV1,
    pub failure_domain_id: String,
    pub administration_domain_id: String,
    pub custody_domain_id: String,
    pub credential_domain_id: String,
    pub physical_location_id: String,
    pub custodian_bindings: Vec<OfflineMediaCustodianBindingV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupTopologyPayloadV1 {
    pub schema_version: u32,
    pub purpose: BackupTopologyPurposeV1,
    pub topology_id: UuidV1,
    pub revision: u64,
    pub predecessor_topology_ref: Option<ArtifactRefV1>,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub topology_signing_trust_current_ref: ArtifactRefV1,
    pub topology_signing_trust_manifest_ref: ArtifactRefV1,
    pub topology_signing_trust_generation: u64,
    pub authority_host_fingerprint_sha256: Sha256Digest,
    pub authority_failure_domain_id: String,
    pub authority_administration_domain_id: String,
    pub authority_credential_domain_id: String,
    pub role_bindings: Vec<BackupRoleBindingV1>,
    pub continuous_target: ContinuousBackupTargetTopologyV1,
    pub offline_media: Vec<OfflineMediaTopologyRowV1>,
    pub backup_recovery_domain_manifest_ref: ArtifactRefV1,
    pub site_legal_retention_seconds: u64,
    pub measured_detection_lag_p99_seconds: u64,
    pub clean_restore_validation_window_seconds: u64,
    pub offline_rotation_interval_seconds: u64,
    pub effective_minimum_retention_seconds: u64,
    pub maximum_offline_generation_age_seconds: u64,
    pub minimum_retained_generation_count: u32,
    pub maximum_safeguard_readback_age_seconds: u64,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type BackupTopologyV1 = SignedBusinessArtifactV1<BackupTopologyPayloadV1>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupVolumeReadbackV1 {
    pub volume_identity: String,
    pub physical_disk_identity_sha256: Sha256Digest,
    pub physical_media: BackupPhysicalMediaV1,
    pub partition_style: PartitionStyleV1,
    pub filesystem: FileSystemKindV1,
    pub bitlocker_encryption_method: BitLockerEncryptionMethodV1,
    pub bitlocker_conversion_status: BitLockerConversionStatusV1,
    pub bitlocker_protection_status: BitLockerProtectionStatusV1,
    pub bitlocker_encryption_percentage: u8,
    pub bitlocker_protector_kinds: Vec<BitLockerProtectorKindV1>,
    pub total_capacity_bytes: u64,
    pub free_capacity_bytes: u64,
    pub final_handle_root_identity_sha256: Sha256Digest,
    pub reparse_point_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupTargetResourceReadbackV1 {
    pub quota_policy: BackupTargetQuotaPolicyV1,
    pub used_object_count: u64,
    pub used_bytes: u64,
    pub observed_ingress_bytes_per_second: u64,
    pub active_upload_count: u32,
    pub object_count_limit_enforced: bool,
    pub stored_bytes_limit_enforced: bool,
    pub ingress_rate_limit_enforced: bool,
    pub concurrent_upload_limit_enforced: bool,
    pub emergency_reserve_free_bytes: u64,
    pub required_retained_ciphertext_bytes: u64,
    pub required_validation_workspace_bytes: u64,
    pub required_growth_margin_30d_bytes: u64,
    pub required_total_capacity_bytes: u64,
    pub active_partial_object_count: u64,
    pub active_partial_bytes: u64,
    pub oldest_active_partial_age_seconds: Option<u64>,
    pub expired_partial_object_count: u64,
    pub expired_partial_bytes: u64,
    pub capacity_calculation_input_ref: ArtifactRefV1,
    pub quota_exhaustion_probe_ref: ArtifactRefV1,
    pub emergency_reserve_probe_ref: ArtifactRefV1,
    pub partial_timeout_probe_ref: ArtifactRefV1,
    pub expired_partial_cleanup_probe_ref: ArtifactRefV1,
    pub bulk_work_suspension_readback_ref: ArtifactRefV1,
    pub completed_history_before_sha256: Sha256Digest,
    pub completed_history_after_sha256: Sha256Digest,
    pub completed_or_pinned_mutation_count: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackupPermissionProbeOperationV1 {
    EnumerateHistory,
    ReadHistoricalObject,
    RepeatJustWrittenReadback,
    OverwriteCompletedObject,
    DeleteCompletedObject,
    RenameCompletedObject,
    ChangeAcl,
    TakeOwnership,
    ShortenRetention,
    ChangeQuota,
    ConsumeEmergencyReserve,
    CleanupPartialObject,
    CleanupNonExpiredPartialObject,
    CleanupCompletedObject,
    CleanupPinnedObject,
    CreateBackupObject,
    RestoreBackupObject,
    ReadBackupContent,
    MutateSourceRetention,
    DeleteSourceObject,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupPermissionProbeReadbackV1 {
    pub actor_role: BackupRoleV1,
    pub operation: BackupPermissionProbeOperationV1,
    pub probe_id: UuidV1,
    pub denial_receipt_ref: ArtifactRefV1,
    pub state_before_sha256: Sha256Digest,
    pub state_after_sha256: Sha256Digest,
    pub observed_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuousBackupTargetSafeguardReadbackV1 {
    pub target_id: String,
    pub authority_host_fingerprint_sha256: Sha256Digest,
    pub target_host_fingerprint_sha256: Sha256Digest,
    pub endpoint_origin: String,
    pub endpoint_identity_sha256: Sha256Digest,
    pub transport_profile: BackupTransportProfileV1,
    pub mtls_trust_policy_ref: ArtifactRefV1,
    pub observed_writer_client_spki: SignerSpkiTokenV1,
    pub observed_target_server_spki: SignerSpkiTokenV1,
    pub observed_failure_domain_id: String,
    pub observed_administration_domain_id: String,
    pub observed_credential_domain_id: String,
    pub authenticated_mtls_session_sha256: Sha256Digest,
    pub authenticated_challenge_response_sha256: Sha256Digest,
    pub target_authenticated_response_sha256: Sha256Digest,
    pub volume: BackupVolumeReadbackV1,
    pub resources: BackupTargetResourceReadbackV1,
    pub just_written_create_receipt_ref: ArtifactRefV1,
    pub just_written_exact_readback_ref: ArtifactRefV1,
    pub just_written_read_capability_consumed: bool,
    pub permission_negative_probes: Vec<BackupPermissionProbeReadbackV1>,
    pub retention_policy_readback_ref: ArtifactRefV1,
    pub retained_checkpoint_refs: Vec<ArtifactRefV1>,
    pub checkpoint_chain_head_ref: Option<ArtifactRefV1>,
    pub shared_admin_domain: bool,
    pub shared_credential_domain: bool,
    pub shared_failure_domain: bool,
    pub authority_host_installation_count: u32,
    pub observed_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct OfflineMediaSafeguardReadbackV1 {
    pub media_role: OfflineMediaRoleV1,
    pub media_id: String,
    pub hardware_serial_sha256: Sha256Digest,
    pub volume_guid: String,
    pub volume: BackupVolumeReadbackV1,
    pub required_recoverable_set_bytes: u64,
    pub required_validation_workspace_bytes: u64,
    pub required_growth_margin_30d_bytes: u64,
    pub required_total_capacity_bytes: u64,
    pub current_state: OfflineMediaStateV1,
    pub transition_sequence: u64,
    pub state_transition_ref: ArtifactRefV1,
    pub transition_chain_head_sha256: Sha256Digest,
    pub verified_checkpoint_refs: Vec<ArtifactRefV1>,
    pub safe_eject_readback_ref: ArtifactRefV1,
    pub device_authorization_revoked: bool,
    pub authority_attachment_count: u32,
    pub target_attachment_count: u32,
    pub physical_disconnection_attestation_ref: ArtifactRefV1,
    pub custody_record_ref: ArtifactRefV1,
    pub failure_domain_id: String,
    pub administration_domain_id: String,
    pub custody_domain_id: String,
    pub credential_domain_id: String,
    pub physical_location_id: String,
    pub custodian_bindings: Vec<OfflineMediaCustodianBindingV1>,
    pub health_readback_ref: ArtifactRefV1,
    pub bundle_contains_recovery_material: bool,
    pub last_verified_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "collection_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum StorageSafeguardCollectionBindingV1 {
    WindowsServiceInstall {
        candidate_run: CandidateRunIdentityV1,
        execution_attempt_id: UuidV1,
        expected_latest_backup_checkpoint_ref: Option<ArtifactRefV1>,
    },
    Postgres16Pitr {
        candidate_run: CandidateRunIdentityV1,
        execution_attempt_id: UuidV1,
        backup_checkpoint_ref: ArtifactRefV1,
    },
    BackupCheckpointPreparation {
        backup_set_id: BackupSetIdV1,
        checkpoint_sequence: u64,
        checkpoint_context: BackupCheckpointContextV1,
        write_barrier_id: UuidV1,
        authority_recovery_cut_manifest_ref: ArtifactRefV1,
        expected_prior_backup_checkpoint_ref: Option<ArtifactRefV1>,
    },
    ProductionActivation {
        activation_id: UuidV1,
        activation_retry_ordinal: u32,
        observed_activation_attempt_id: UuidV1,
        candidate_run: CandidateRunIdentityV1,
        release_certificate_ref: ArtifactRefV1,
        generation_observed_selection_ref: ArtifactRefV1,
        certified_latest_backup_checkpoint_ref: ArtifactRefV1,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorageSafeguardReadbackV1 {
    pub schema_version: u32,
    pub purpose: StorageSafeguardReadbackPurposeV1,
    pub binding: StorageSafeguardCollectionBindingV1,
    pub collector_challenge_nonce_lowerhex: String,
    pub collector_session_binding_sha256: Sha256Digest,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub authority_windows_boot_id: String,
    pub backup_topology_ref: ArtifactRefV1,
    pub topology_signing_trust_current_ref: ArtifactRefV1,
    pub latest_backup_checkpoint_ref: Option<ArtifactRefV1>,
    pub backup_recovery_domain_manifest_ref: ArtifactRefV1,
    pub continuous_target: ContinuousBackupTargetSafeguardReadbackV1,
    pub offline_media: Vec<OfflineMediaSafeguardReadbackV1>,
    pub protection_state: BackupProtectionStateV1,
    pub protection_transition: Option<BackupProtectionTransitionV1>,
    pub observed_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum AuthorityRecoveryCutManifestPurposeV1 {
    #[serde(rename = "EP-F57-AUTHORITY-RECOVERY-CUT-MANIFEST-V1")]
    AuthorityRecoveryCutManifest,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityRecoveryCutRowV1 {
    pub authority_class_id: String,
    pub authority_root_id: String,
    pub write_barrier_id: UuidV1,
    pub source_cut_ref: ArtifactRefV1,
    pub source_sha256: Sha256Digest,
    pub source_size_bytes: u64,
    pub target_receipt_refs: Vec<ArtifactRefV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityRecoveryCutManifestV1 {
    pub schema_version: u32,
    pub purpose: AuthorityRecoveryCutManifestPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub backup_set_id: BackupSetIdV1,
    pub checkpoint_context: BackupCheckpointContextV1,
    pub write_barrier_id: UuidV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub authority_storage_manifest_sha256: Sha256Digest,
    pub backup_topology_ref: ArtifactRefV1,
    pub rows: Vec<AuthorityRecoveryCutRowV1>,
    pub authority_class_set_sha256: Sha256Digest,
    pub frozen_at_unix_ms: i64,
}

pub struct VerifiedBackupCheckpointRequestV1 {
    /* private: typed context + exact storage manifest/barrier/scope/prior-head bindings */
}

pub struct VerifiedBackupCheckpointPreparationV1 {
    /* private: exact request + immutable cut + derived next sequence/binding */
}

pub struct VerifiedBackupCheckpointSafeguardReadbackV1 {
    /* private: exact typed readback bytes, content ref and verified preparation binding */
}

pub trait BackupCheckpointSafeguardCollectorV1: Send {
    fn collect_checkpoint_preparation<'a>(
        &'a mut self,
        preparation: &'a VerifiedBackupCheckpointPreparationV1,
    ) -> BackupPortFutureV1<'a, VerifiedBackupCheckpointSafeguardReadbackV1>;
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupCheckpointDraftPayloadV1 {
    pub schema_version: u32,
    pub backup_set_id: BackupSetIdV1,
    pub checkpoint_sequence: u64,
    pub checkpoint_context: BackupCheckpointContextV1,
    pub write_barrier_id: UuidV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub authority_storage_manifest_sha256: Sha256Digest,
    pub authority_recovery_cut_manifest_ref: ArtifactRefV1,
    pub authority_recovery_cut_manifest_sha256: Sha256Digest,
    pub previous_checkpoint_ref: Option<ArtifactRefV1>,
    pub storage_safeguard_readback_ref: ArtifactRefV1,
    pub backup_manifest_ref: ArtifactRefV1,
    pub epb1_cipher_graph_ref: ArtifactRefV1,
    pub target_receipt_refs: Vec<ArtifactRefV1>,
    pub backup_key_envelope_ref: ArtifactRefV1,
    pub backup_key_envelope_sha256: Sha256Digest,
    pub recovery_domain_manifest_ref: ArtifactRefV1,
    pub recovery_domain_manifest_sha256: Sha256Digest,
    pub release_generation: u64,
    pub config_generation: u64,
    pub postgres_base_backup_ref: ArtifactRefV1,
    pub postgres_wal_span_ref: ArtifactRefV1,
    pub attachment_recovery_cut_ref: ArtifactRefV1,
    pub source_cut_ref: ArtifactRefV1,
    pub sink_seal_ref: ArtifactRefV1,
    pub ready_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackupCheckpointPayloadV1 {
    pub schema_version: u32,
    pub purpose: BackupCheckpointPurposeV1,
    pub signing_operation_id: UuidV1,
    pub backup_set_id: BackupSetIdV1,
    pub checkpoint_sequence: u64,
    pub checkpoint_draft_ref: ArtifactRefV1,
    pub checkpoint_context: BackupCheckpointContextV1,
    pub write_barrier_id: UuidV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub authority_storage_manifest_sha256: Sha256Digest,
    pub authority_recovery_cut_manifest_ref: ArtifactRefV1,
    pub authority_recovery_cut_manifest_sha256: Sha256Digest,
    pub previous_checkpoint_ref: Option<ArtifactRefV1>,
    pub storage_safeguard_readback_ref: ArtifactRefV1,
    pub backup_manifest_ref: ArtifactRefV1,
    pub epb1_cipher_graph_ref: ArtifactRefV1,
    pub target_receipt_refs: Vec<ArtifactRefV1>,
    pub backup_key_envelope_ref: ArtifactRefV1,
    pub backup_key_envelope_sha256: Sha256Digest,
    pub recovery_domain_manifest_ref: ArtifactRefV1,
    pub recovery_domain_manifest_sha256: Sha256Digest,
    pub release_generation: u64,
    pub config_generation: u64,
    pub postgres_base_backup_ref: ArtifactRefV1,
    pub postgres_wal_span_ref: ArtifactRefV1,
    pub attachment_recovery_cut_ref: ArtifactRefV1,
    pub source_cut_ref: ArtifactRefV1,
    pub sink_seal_ref: ArtifactRefV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}
pub type BackupCheckpointV1 = SignedBusinessArtifactV1<BackupCheckpointPayloadV1>;
pub struct VerifiedBackupCheckpointV1 { value: BackupCheckpointV1, artifact_ref: ArtifactRefV1 }

impl VerifiedBackupCheckpointV1 {
    pub fn value(&self) -> &BackupCheckpointV1 { &self.value }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
}

impl<E: BackupChunkEncryptorV1> BackupAuthorityV1<E> {
pub async fn stream_backup(
    &self,
    checkpoint_request: &VerifiedBackupCheckpointRequestV1,
    source: &mut dyn BackupSourceV1,
    sink: &mut dyn AppendOnlySinkV1,
    safeguard_collector: &mut dyn BackupCheckpointSafeguardCollectorV1,
    draft_store: &mut dyn BackupCheckpointDraftStoreV1,
) -> Result<VerifiedBackupCheckpointReadyDraftV1, BackupErrorV1> {
    let session = source.begin_or_adopt_set(self.verified_request()).await?;
    while let Some(chunk) = source.next_bounded_chunk(&session).await? {
        let encrypted = self.encryptor.encrypt_and_authenticate(self.verified_request(), chunk)?;
        let just_appended = sink.append_create_or_adopt(&encrypted).await?;
        sink.readback_exact(just_appended).await?;
    }
    let source_cut = source.freeze_source_cut(&session).await?;
    let sink_seal = sink.seal_immutable_set(session.set_id()).await?;
    let preparation = self.verify_checkpoint_preparation(
        checkpoint_request,
        &source_cut,
        &sink_seal,
    )?;
    let safeguard_readback = safeguard_collector
        .collect_checkpoint_preparation(&preparation)
        .await?;
    self.build_verify_and_store_ready_draft(
        checkpoint_request,
        source_cut,
        sink_seal,
        safeguard_readback,
        draft_store,
    ).await
}
}

pub struct BackupCheckpointSignerAuthorityV1<S, V> {
    signer: S,
    verifier: V,
}

impl<S: BackupCheckpointSignPortV1, V: BackupCheckpointVerifyPortV1>
    BackupCheckpointSignerAuthorityV1<S, V>
{
    pub fn compose(signer: S, verifier: V) -> Self { Self { signer, verifier } }
    pub async fn sign_bind_and_verify(
        &self,
        draft: &VerifiedBackupCheckpointReadyDraftV1,
        attempts: &mut dyn BackupCheckpointSigningAttemptStoreV1,
        objects: &mut dyn BackupCheckpointObjectStoreV1,
    ) -> Result<VerifiedBackupCheckpointV1, BackupErrorV1>;
}
```

All ports are object-safe traits, not field-bearing nominals. Source/target adapters may construct only a session/chunk/cut/receipt/readback/seal by passing the bound request or preceding DTO into its validating constructor; the writer authority constructs ciphertext and the complete draft, but cannot construct a signing operation, signed payload, verified safeguard value or verified checkpoint. Every constructor exact-checks set/session/sequence/digest/size/identity/ref continuity and zeroizes plaintext on drop. `BackupAuthorityV1::compose` receives only the private set request and encryptor; `stream_backup` additionally requires the private neutral checkpoint request and the narrow `BackupCheckpointSafeguardCollectorV1` port. The request is constructed only by the backup authority from a verified current storage manifest/topology, the explicit prior checkpoint head and either a schedule identity or upper-provided package-maintenance reservation/policy refs plus the already committed barrier. It contains no package nominal and cannot create the barrier itself. Only the authority can construct private `VerifiedBackupCheckpointPreparationV1` after the cut; the collector can return only a freshly verified, stored `BACKUP_CHECKPOINT_PREPARATION` readback/ref for that exact private value and has no caller-field overload.

At the barrier the backup authority derives and create-new stores one strict `AuthorityRecoveryCutManifestV1`. Its canonical row exact-set equals every enabled authority-data class/root in the same signed storage manifest—not only PostgreSQL, WAL and attachments—including audit, Outbox, generation/package registry and execution state, vault, identity/authz/tenancy, holds/tombstones, flows/automations and every enabled DATA_HDD authority class. The cut header carries the exact private request's `BackupSetIdV1`, `BackupCheckpointContextV1` and common `write_barrier_id`; every row repeats that barrier, immutable source ref/digest/size and the complete off-host target-receipt set. Missing, extra, duplicate, cross-backup-set, cross-context, cross-root or cross-barrier data fails before READY. After that immutable cut exists and before draft construction, the authority derives the next positive checkpoint sequence and private preparation binding, obtains one new safeguard readback through the narrow port, verifies its nonce/session/current topology/storage manifest/prior-head/state rules, and exact-binds its ref into the draft. The preparation, checkpoint draft and signed payload repeat the same typed backup set, context and barrier plus sequence, storage-manifest ref/digest, cut-manifest ref/digest, prior checkpoint option and safeguard readback ref; signer verification re-loads every one. For `PACKAGE_MAINTENANCE`, reservation and checkpoint-policy refs exact-match the request; thus Task 14 can close/drain, commit a barrier, ask this lower port for a checkpoint, and receive a private `VerifiedBackupCheckpointV1` without creating package -> backup dependency. The authority also checks contiguous EPB1 ordinals, bounded size, ciphertext/receipt/readback digests, exact backup manifest and cipher graph, complete target receipts, key/recovery-domain digests, release/config generations, base/WAL/attachment facts, source cut and sink seal before create-new storing READY.

PITR keeps its 16-field outer root and existing 18/17/30 subordinate registry. Its APPEND_ONLY parser and both OFFLINE_ROTATION_MEDIA leaves each add the identical `storage_safeguard_readback_ref`; all three strict-load this PITR attempt's `StorageSafeguardReadbackV1`, whose `backup_topology_ref` exact-equals `AuthorityRecoveryCutManifestV1.backup_topology_ref`. This is a field expansion inside existing parsers, not a new parser, recipe, signer or subordinate slot.

Only `BackupCheckpointSignerAuthorityV1::compose` receives the dedicated signer. It consumes one private READY draft, create-new commits an unpredictable operation ID and immutable checkpoint payload before signing, signs only that payload, fsyncs/adopts the exact envelope spool, create-new stores and exact-reloads the checkpoint object, verifies it through the separate verify-only port, and finally CAS-binds `OBJECT_BOUND`. Recovery at `OPERATION_COMMITTED` reuses the frozen payload/operation and requests byte-identical signing; recovery at `ENVELOPE_SPOOLED` adopts exact bytes; `OBJECT_BOUND` is terminal. A second payload, cross-set/cross-context/cross-barrier draft, response-loss re-sign with different bytes, partial target receipt set, incomplete full-authority cut, missing graph/key/recovery/generation/base/WAL/attachment closure, writer dependency on the signing port, or raw checkpoint/ref substitution fails closed. Composition tests prove `backup-writer` links no signing provider while `backup-checkpoint-signer` links no source/plaintext/encryption/target-delete/recovery-decrypt capability. `BackupErrorV1` remains the displayed closed enum and maps without string fallback. Task 11 adds direct `zeroize` and `async-trait` dependencies where imported and tests that no plaintext clone or internal-HDD persistence API exists.

No full base backup or plaintext copy lands on the internal HDD. Each backup set has an independent recovery-only DEK envelope; operational service tokens cannot decrypt history. Offline media is connected only for controlled rotation/readback. `backup_store` is the only SQL adapter for set/media/certification metadata and is composed into backup-writer through its restricted operations identity; `ep-platform-backup`, backup-target, and recovery-tool contain no authority SQL. The rehearsal exercises the complete bounded state machine through this concrete store, including crash after append/before receipt, crash after seal/before checkpoint, exact adoption, conflict quarantine, and pairwise-disjoint writer/target/recovery credentials.

`recovery-tool` implements one immutable Scheduled Task runner and a closed authenticated request/result/query protocol. The request contains only operation ID, binding digest and one pre-reserved discriminated operation payload; the Task Scheduler invocation has no caller-controlled arguments. Before an effect, the runner create-new appends its request and intent to the fixed recovery-domain journal, verifies the Task-11 component row, held executable handle, Authenticode, account/token privileges and IPC peer, then executes or query/adopts the same operation. Results repeat the request ref/digest, operation ID, exact effect receipt and post-effect readback. Repeated IDs with changed bytes conflict; missing/UNKNOWN proof keeps admission closed. Static schema/WiX/MSI goldens and a live zero-side-effect self-test prove the task path, account, privileges, SDDL, action, IPC DACL and allowlist byte-equal the one `RECOVERY_TOOL` component row.

`BackupCheckpointSigningStageV1` serializes exactly as `OPERATION_COMMITTED|ENVELOPE_SPOOLED|OBJECT_BOUND`; the checkpoint-signing attempt byte golden rejects PascalCase, an unknown or skipped stage, and every extra field.

- [ ] **Step 4: Rehearse Fresh PG/PITR/media rotation.**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G6_RELEASE --through 20261025092510`

Expected: PASS.

Run through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_TEST_POSTGRES16_PITR_V1 -- -Mode EngineeringRehearsal`.

Expected: restores and exact-verifies every enabled authority-data row named by one `AuthorityRecoveryCutManifestV1` at one engineering barrier, including PostgreSQL/WAL/attachments and all other HDD authority classes. This proves the component before commit but is not final-candidate recovery evidence; Tasks 12/15 repeat three clean-hardware runs against the frozen digest and controlled media.

Run: `cargo test -p ep-testkit --test f57_backup_target --test f57_backup_envelope --test f57_backup_storage_safeguard --test f57_backup_topology_signing_trust --test f57_backup_checkpoint_transition --test f57_postgres16_windows_install --test f57_postgres16_recovery -- --nocapture`

Run: `cargo test -p ep-platform-backup -p ep-adapter-backup -p ep-adapter-db-pg -p backup-writer -p backup-checkpoint-signer -p backup-target -p recovery-tool -p pg-passphrase-helper -p ep-testkit --all-targets --locked`

Expected additionally: checkpoint/cut/topology/safeguard and all five PostgreSQL roots are exact: 19-field package lock, 13-field install contract, 4-field Event Log fixture set, 19-field Event Log scan coverage and 17-field install readback. Install/readback tests reject a missing, truncated, cleared, dropped, gapped, cross-boot, wrong-provider, wrong-fixture or digest-mismatched Event Log interval and any incomplete coverage or customer-token hit. PostgreSQL package lock, service `RUNNING` readback, HDD paths and effective config PASS; exact `max_connections=64|reserved_connections=4|superuser_reserved_connections=3`, the two-slot unallocatable margin and every `NORMAL|RESERVED|SUPERUSER` consumer/role attribute satisfy the five classified budget inequalities, including normal saturation followed by migration then recovery acquisition. HBA proves only loopback `hostssl` plus `scram-sha-256`; each matching authenticated client probe separately proves `channel_binding=require` and successful negotiation. Same-file DATA_HDD `fsync` and `fsync_writethrough` qualification PASS as the Task-11 compatibility/driver/cache prerequisite; it must not claim final power-loss durability, which Task 15 alone completes by exact-joining the frozen candidate to current P340 UPS/write-cache plus controlled HDD flush/power-cut evidence. Backup retention/capacity/permission/one-time-read/offline-disconnect probes PASS; the six-row backup/recovery registry and generated WiX projection are byte-equal; writer, checkpoint signer and data-volume unlock broker are three distinct on-host services, recovery tool is the single immutable on-demand Scheduled Task, passphrase helper is an on-demand executable, and target agent is a separately packaged `OFF_HOST_ONLY` row absent from the Authority MSI. PUBLIC_KEY policy/locator/bootstrap/unlock goldens and the restricted-LocalSystem WS2022 method test PASS; the complete G0 evidence-signer-broker install/readback golden exact-matches its imported package. The product-owned inventory is exactly nine SCM rows, one task and one on-demand executable, and the complete host formula is `10 + active_additional_windows_service_count`. Capability and Cargo-DAG tests reject target co-location, shared accounts, writer→signer, signer→plaintext/source, unlock→database/backup/signing, Authority→off-host-target private-key edges, any extra recovery task/SCM row and caller-controlled recovery command/path/argv.

Run: `cargo xtask f57 graph generate --check`

Expected: PASS with the backup capability nodes and generated projections bound to the same graph digest.

Expected backup-trust/transition closure additionally: both trust schemas and all byte goldens prove current-pointer/manifest genesis, checked generation+1 successors, exact predecessors, fixed trust-manifest-authority and topology-signer DN/SPKI, offline chain/revocation/transparency refs, active-config current selection, storage-policy digest pin and topology/readback joins. Negative fixtures reject self-authentication, old-current/fork/gap/rollback, wrong signer/ref/media/purpose/time, and every ADR-0020 `PIV_SHAMIR_2_OF_3_V1`, application/backup recovery-domain or backup-envelope recipient-roster substitution; neither trust root adds a candidate-evidence signer row. Transition tests exercise minimum-retained counts 2 and greater than 2 through empty `INITIALIZING` -> sequence 1 -> immutable `INITIAL_POPULATION/BOOTSTRAPPING` -> derived head+1 repetitions -> A/B closure -> `HEALTHY/None`; they reject sequence 2 before current-head A/B verification and reject early/sticky HEALTHY or BOOTSTRAPPING. Rotation tests begin only from fresh HEALTHY, atomically advance unchanged-or-successor trust plus storage/topology roots, accept exactly one old-head+1 bridge under `TRANSITIONING`, adopt response loss byte-identically, then allow only offline copy in `CURRENT_ROOTS_ROTATION/BOOTSTRAPPING` before HEALTHY. A second bridge/rotation, mutable transition, wrong anchor/target, crash fork or any `INITIALIZING|BOOTSTRAPPING|TRANSITIONING|NON_SUPPRESSIBLE_RISK` PITR/release/recovery-certification/activation attempt fails closed.

Expected resilience closure additionally: normal-root tests prove deployment-wide hold, lease drain and durable barrier all precede the active-config CAS; the hold survives every `TRANSITIONING` and `BOOTSTRAPPING` crash cut and can be released only by fresh `HEALTHY/None`, exact new-root binding and the upper admission CAS. Disaster-replacement tests begin with an unreadable/dead old DATA_HDD and therefore no fresh old-volume readback; they accept only off-host current config/trust plus the last authenticated checkpoint/cut, two distinct recovery custodians, old-authority fencing, strictly higher epoch/storage generation, new disk/volume/BitLocker/storage-manifest identity, clean PITR and all-data reconciliation, empty-chain continuous/A/B bootstrap and fresh carrier/capacity recertification before takeover. Pairwise-domain tests cover all 30 inequalities and each same-admin/credential/failure/location/custody alias. PostgreSQL-log tests prove the exact 30-day/20-GiB/7-day/current/legal-hold rules, signed preview/adopt cleanup, ACL denial for PostgreSQL and every non-control identity, stricter effective 100/50-ish P340 thresholds, one-byte boundaries and fail-close legal-hold/free-space conflict.

- [ ] **Step 5: Commit backup pipeline.**

```bash
cargo xtask f57 task stage --task G6-11
cargo xtask f57 task verify-staged --task G6-11
git commit -m "feat: add streaming append only backup"
```

### Task 12: Implement ransomware isolation and rehearse clean restore

**Files:**
- Create: `crates/platform/flow/src/security_incident.rs`
- Create: `crates/platform/flow/tests/security_incident.rs`
- Create: `apps/ops-agent/src/security_incident.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/security_incident_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `apps/ops-agent/src/wiring/security_incident.rs`
- Create: `apps/ops-agent/tests/security_incident_composition.rs`
- Modify: `apps/ops-agent/src/wiring/mod.rs`
- Modify: `apps/ops-agent/Cargo.toml`
- Create: `scripts/windows/backup-restore-drill.ps1`
- Create after final signing: `scripts/windows/trust/F57_PS_BACKUP_RESTORE_DRILL_V1.authenticode.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after descriptor verification: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after descriptor verification: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Create: `docs/evidence/f57-recovery-cut-selection.v1.schema.json`
- Create: `docs/evidence/f57-recovery-certification-policy.v1.schema.json`
- Create: `crates/platform/flow/tests/fixtures/f57-recovery-cut-selection-v1-golden.json`
- Create: `db/migrations/platform_ops/V20261025092520__platform_ops_create_security_incidents_and_recovery_cuts.sql`
- Create: `testkit/tests/f57_ransomware_recovery.rs`
- Create: `testkit/tests/f57_recovery_cut.rs`
- Create: `testkit/tests/f57_security_incident.rs`
- Modify: `crates/platform/flow/src/lib.rs`
- Modify: `crates/platform/flow/Cargo.toml`
- Modify: `apps/ops-agent/src/main.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: signed clean cuts, 2-of-3 backup recovery domain, distinct 2-of-3 application-vault recovery domain, clean Windows host.
- Produces: security incident lifecycle, quarantined restore, business reconciliation, typed selection of a Task-11-owned full recovery cut, and three-step `RecoveryCertificationPolicyV1` evidence. `crates/platform/flow/src/security_incident.rs` solely owns private-field `SecurityIncidentV1`, `RecoveryCutSelectionV1`, `RecoveryCertificationPolicyV1`, `RecoveryCertificationRecordV1`, their typed IDs, both closed state machines, the closed `SecurityIncidentErrorV1`, and durable `SecurityIncidentStoreV1`; it imports and references the backup-owned checkpoint/cut rather than copying their authority rows. db-pg only implements the port and ops-agent only composes it.

The restore-drill script is signed/timestamped only after its final byte and is executable solely through its closed script ID and descriptor. Its clean-room child operations are compiled Rust or other members of the same trusted script closure; it cannot download, generate or invoke an unregistered script.

- [ ] **Step 1: Write failing poisoned-latest and key-domain separation tests.**

```rust
#[test]
fn latest_poisoned_cut_is_rejected_and_keys_cannot_cross_domains() {
    assert_eq!(select_restore_cut(cuts_with_poisoned_latest()).id, "KNOWN_CLEAN_PREDECESSOR");
    assert_eq!(unwrap_backup_with_application_shares().unwrap_err().code(), "RECOVERY_DOMAIN_MISMATCH");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_ransomware_recovery --test f57_recovery_cut --test f57_security_incident -- --nocapture`

Run: `cargo test -p ops-agent --test security_incident_composition --locked`

Expected: FAIL.

- [ ] **Step 3: Implement fail-closed incident and recovery state.**

```rust
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct SecurityIncidentIdV1(UuidV1);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct RecoveryCutSelectionIdV1(UuidV1);

pub enum RecoveryCutSelectionPurposeV1 {
    #[serde(rename = "EP-F57-RECOVERY-CUT-SELECTION-V1")]
    RecoveryCutSelection,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct RecoveryCertificationPolicyIdV1(UuidV1);

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityIncidentStateV1 {
    Declared,
    Isolated,
    CleanCutBound,
    RestoreQuarantined,
    MalwareVerified,
    IntegrityVerified,
    BusinessReconciled,
    KeysRotated,
    ReleaseApproved,
    Certified,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryCutSelectionV1 {
    schema_version: u32,
    purpose: RecoveryCutSelectionPurposeV1,
    recovery_cut_selection_id: RecoveryCutSelectionIdV1,
    backup_checkpoint_ref: ArtifactRefV1,
    authority_recovery_cut_manifest_ref: ArtifactRefV1,
    authority_recovery_cut_manifest_sha256: Sha256Digest,
    write_barrier_id: UuidV1,
    clean_scan_evidence_ref: ArtifactRefV1,
    selection_authorization_refs: Vec<ArtifactRefV1>,
    selected_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityIncidentV1 {
    schema_version: u32,
    incident_id: SecurityIncidentIdV1,
    version: u64,
    state: SecurityIncidentStateV1,
    previous_incident_ref: Option<ArtifactRefV1>,
    previous_record_sha256: Option<Sha256Digest>,
    suspected_event_ref: ArtifactRefV1,
    write_fence_ref: ArtifactRefV1,
    recovery_cut_selection_ref: Option<ArtifactRefV1>,
    evidence_refs: Vec<ArtifactRefV1>,
    authorized_actor: PrincipalRefV1,
    recorded_at_unix_ms: i64,
}

pub struct SecurityIncidentRefV1 {
    incident_id: SecurityIncidentIdV1,
    version: u64,
    artifact_ref: ArtifactRefV1,
    record_sha256: Sha256Digest,
}

pub struct SecurityIncidentCasV1 {
    incident_id: SecurityIncidentIdV1,
    expected_version: u64,
    expected_artifact_ref: ArtifactRefV1,
    expected_record_sha256: Sha256Digest,
}

pub struct SecurityIncidentTransitionV1 {
    cas: SecurityIncidentCasV1,
    from: SecurityIncidentStateV1,
    to: SecurityIncidentStateV1,
    recovery_cut_selection_ref: Option<ArtifactRefV1>,
    evidence_refs: Vec<ArtifactRefV1>,
    authorized_actor: PrincipalRefV1,
    recorded_at_unix_ms: i64,
    exact_next_incident_jcs_bytes: Vec<u8>,
}

pub struct VerifiedSecurityIncidentCreateV1 {
    exact_incident_jcs_bytes: Vec<u8>,
    value: SecurityIncidentV1,
}

pub struct VerifiedSecurityIncidentSnapshotV1 {
    value: SecurityIncidentV1,
    reference: SecurityIncidentRefV1,
    cas: SecurityIncidentCasV1,
    exact_incident_jcs_bytes: Vec<u8>,
}

pub struct VerifiedRecoveryCutSelectionV1 {
    value: RecoveryCutSelectionV1,
    artifact_ref: ArtifactRefV1,
    exact_jcs_bytes: Vec<u8>,
}

pub enum SecurityIncidentErrorV1 {
    NotFound,
    VersionConflict,
    HashChain,
    TransitionForbidden,
    EvidenceMissing,
    CutPoisoned,
    RecoveryDomainMismatch,
    WriteFence,
    CertificationIncomplete,
    Storage,
}

pub trait SecurityIncidentAuthorizationPortV1: Send + Sync {
    fn authorize_transition(
        &self,
        actor: &PrincipalRefV1,
        from: SecurityIncidentStateV1,
        to: SecurityIncidentStateV1,
    ) -> Result<(), SecurityIncidentErrorV1>;
}

pub trait SecurityIncidentTrustedTimePortV1: Send + Sync {
    fn now_unix_ms(&self) -> Result<i64, SecurityIncidentErrorV1>;
}

pub struct SecurityIncidentAuthorityV1<'a> {
    store: &'a dyn SecurityIncidentStoreV1,
    authorization: &'a dyn SecurityIncidentAuthorizationPortV1,
    trusted_time: &'a dyn SecurityIncidentTrustedTimePortV1,
}

impl SecurityIncidentRefV1 {
    pub fn incident_id(&self) -> &SecurityIncidentIdV1 { &self.incident_id }
    pub fn version(&self) -> u64 { self.version }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
}

impl VerifiedSecurityIncidentSnapshotV1 {
    pub fn value(&self) -> &SecurityIncidentV1 { &self.value }
    pub fn reference(&self) -> &SecurityIncidentRefV1 { &self.reference }
    pub fn cas(&self) -> &SecurityIncidentCasV1 { &self.cas }
    pub fn exact_jcs_bytes(&self) -> &[u8] { &self.exact_incident_jcs_bytes }
}

impl VerifiedSecurityIncidentCreateV1 {
    pub fn value(&self) -> &SecurityIncidentV1 { &self.value }
    pub fn exact_jcs_bytes(&self) -> &[u8] { &self.exact_incident_jcs_bytes }
}

impl SecurityIncidentCasV1 {
    pub fn incident_id(&self) -> &SecurityIncidentIdV1 { &self.incident_id }
    pub fn expected_version(&self) -> u64 { self.expected_version }
    pub fn expected_artifact_ref(&self) -> &ArtifactRefV1 { &self.expected_artifact_ref }
    pub fn expected_record_sha256(&self) -> Sha256Digest { self.expected_record_sha256 }
}

impl SecurityIncidentTransitionV1 {
    pub fn cas(&self) -> &SecurityIncidentCasV1 { &self.cas }
    pub fn from(&self) -> SecurityIncidentStateV1 { self.from }
    pub fn to(&self) -> SecurityIncidentStateV1 { self.to }
    pub fn exact_next_incident_jcs_bytes(&self) -> &[u8] { &self.exact_next_incident_jcs_bytes }
}

impl<'a> SecurityIncidentAuthorityV1<'a> {
    pub fn compose(
        store: &'a dyn SecurityIncidentStoreV1,
        authorization: &'a dyn SecurityIncidentAuthorizationPortV1,
        trusted_time: &'a dyn SecurityIncidentTrustedTimePortV1,
    ) -> Self {
        Self { store, authorization, trusted_time }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum RecoveryCertificationPolicyPurposeV1 {
    #[serde(rename = "EP-F57-RECOVERY-CERTIFICATION-POLICY-V1")]
    EpF57RecoveryCertificationPolicyV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryCertificationPolicyV1 {
    schema_version: u32,
    purpose: RecoveryCertificationPolicyPurposeV1,
    policy_id: RecoveryCertificationPolicyIdV1,
    candidate_manifest_ref: ArtifactRefV1,
    carrier_staging_plan_ref: ArtifactRefV1,
    outer_carrier_execution_attempt_id: UuidV1,
    certification_id: UuidV1,
    recovery_execution_attempt_ids: [UuidV1; 3],
    clean_source_cut_ref: ArtifactRefV1,
    required_distinct_recovery_execution_attempts: u8, // exactly 3
    maximum_age_seconds: u64,                          // exactly 7_776_000
    issued_at_unix_ms: i64,
    expires_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecoveryCertificationStateV1 {
    Unverified,
    InitialRestoreVerified,
    CandidateMeasured,
    Certified,
    Expired,
    Invalidated,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryCertificationRecordV1 {
    policy_id: RecoveryCertificationPolicyIdV1,
    sequence: u8,
    state: RecoveryCertificationStateV1,
    predecessor_ref: Option<ArtifactRefV1>,
    recovery_execution_attempt_id: Option<UuidV1>,
    recovery_evidence_ref: Option<ArtifactRefV1>,
    previous_record_sha256: Sha256Digest,
    recorded_at_unix_ms: i64,
}

pub struct VerifiedRecoveryCertificationPolicyV1 {
    value: RecoveryCertificationPolicyV1,
    artifact_ref: ArtifactRefV1,
    exact_jcs_bytes: Vec<u8>,
}

pub struct RecoveryCertificationCasV1 {
    policy_ref: ArtifactRefV1,
    expected_sequence: Option<u8>,
    expected_predecessor_ref: Option<ArtifactRefV1>,
    expected_predecessor_sha256: Sha256Digest,
}

pub struct VerifiedRecoveryCertificationAppendV1 {
    cas: RecoveryCertificationCasV1,
    record: RecoveryCertificationRecordV1,
    artifact_ref: ArtifactRefV1,
    exact_jcs_bytes: Vec<u8>,
}

pub struct VerifiedRecoveryCertificationRecordV1 {
    value: RecoveryCertificationRecordV1,
    artifact_ref: ArtifactRefV1,
    exact_jcs_bytes: Vec<u8>,
}

impl VerifiedRecoveryCertificationPolicyV1 {
    pub fn value(&self) -> &RecoveryCertificationPolicyV1 { &self.value }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
    pub fn exact_jcs_bytes(&self) -> &[u8] { &self.exact_jcs_bytes }
}

impl VerifiedRecoveryCertificationAppendV1 {
    pub fn cas(&self) -> &RecoveryCertificationCasV1 { &self.cas }
    pub fn record(&self) -> &RecoveryCertificationRecordV1 { &self.record }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
    pub fn exact_jcs_bytes(&self) -> &[u8] { &self.exact_jcs_bytes }
}

impl VerifiedRecoveryCertificationRecordV1 {
    pub fn value(&self) -> &RecoveryCertificationRecordV1 { &self.value }
    pub fn artifact_ref(&self) -> &ArtifactRefV1 { &self.artifact_ref }
    pub fn exact_jcs_bytes(&self) -> &[u8] { &self.exact_jcs_bytes }
}

pub trait SecurityIncidentCommandMintV1 {
    fn declare(
        &self,
        incident_id: UuidV1,
        suspected_event_ref: ArtifactRefV1,
        write_fence_ref: ArtifactRefV1,
        evidence_refs: Vec<ArtifactRefV1>,
        actor: PrincipalRefV1,
    ) -> Result<VerifiedSecurityIncidentCreateV1, SecurityIncidentErrorV1>;

    fn select_recovery_cut(
        &self,
        recovery_cut_selection_id: UuidV1,
        backup_checkpoint_ref: ArtifactRefV1,
        clean_scan_evidence_ref: ArtifactRefV1,
        selection_authorization_refs: Vec<ArtifactRefV1>,
    ) -> Result<VerifiedRecoveryCutSelectionV1, SecurityIncidentErrorV1>;

    fn bind_recovery_cut(
        &self,
        snapshot: &VerifiedSecurityIncidentSnapshotV1,
        cut: &VerifiedRecoveryCutSelectionV1,
        actor: PrincipalRefV1,
    ) -> Result<SecurityIncidentTransitionV1, SecurityIncidentErrorV1>;

    fn transition(
        &self,
        snapshot: &VerifiedSecurityIncidentSnapshotV1,
        to: SecurityIncidentStateV1,
        evidence_refs: Vec<ArtifactRefV1>,
        actor: PrincipalRefV1,
    ) -> Result<SecurityIncidentTransitionV1, SecurityIncidentErrorV1>;

    fn verify_recovery_policy(
        &self,
        policy: RecoveryCertificationPolicyV1,
        exact_jcs_bytes: Vec<u8>,
        expected_ref: ArtifactRefV1,
    ) -> Result<VerifiedRecoveryCertificationPolicyV1, SecurityIncidentErrorV1>;

    fn append_recovery_state(
        &self,
        policy: &VerifiedRecoveryCertificationPolicyV1,
        predecessor: Option<&VerifiedRecoveryCertificationRecordV1>,
        next_state: RecoveryCertificationStateV1,
        recovery_execution_attempt_id: Option<UuidV1>,
        recovery_evidence_ref: Option<ArtifactRefV1>,
    ) -> Result<VerifiedRecoveryCertificationAppendV1, SecurityIncidentErrorV1>;
}

pub trait SecurityIncidentStoreV1: Send + Sync {
    fn create_or_adopt_incident(
        &self,
        command: &VerifiedSecurityIncidentCreateV1,
    ) -> Result<VerifiedSecurityIncidentSnapshotV1, SecurityIncidentErrorV1>;
    fn compare_and_append_incident(
        &self,
        transition: &SecurityIncidentTransitionV1,
    ) -> Result<VerifiedSecurityIncidentSnapshotV1, SecurityIncidentErrorV1>;
    fn load_incident_exact(
        &self,
        id: &SecurityIncidentIdV1,
        version: u64,
    ) -> Result<VerifiedSecurityIncidentSnapshotV1, SecurityIncidentErrorV1>;
    fn create_or_adopt_recovery_cut_selection(
        &self,
        selection: &VerifiedRecoveryCutSelectionV1,
    ) -> Result<VerifiedRecoveryCutSelectionV1, SecurityIncidentErrorV1>;
    fn load_recovery_cut_selection_exact(
        &self,
        expected: &ArtifactRefV1,
    ) -> Result<VerifiedRecoveryCutSelectionV1, SecurityIncidentErrorV1>;
    fn create_or_adopt_recovery_policy(
        &self,
        policy: &VerifiedRecoveryCertificationPolicyV1,
    ) -> Result<VerifiedRecoveryCertificationPolicyV1, SecurityIncidentErrorV1>;
    fn compare_and_append_recovery_record(
        &self,
        append: &VerifiedRecoveryCertificationAppendV1,
    ) -> Result<VerifiedRecoveryCertificationRecordV1, SecurityIncidentErrorV1>;
    fn load_recovery_record_exact(
        &self,
        expected: &ArtifactRefV1,
    ) -> Result<VerifiedRecoveryCertificationRecordV1, SecurityIncidentErrorV1>;
}
```

The only forward state sequence is `DECLARED -> ISOLATED -> CLEAN_CUT_BOUND -> RESTORE_QUARANTINED -> MALWARE_VERIFIED -> INTEGRITY_VERIFIED -> BUSINESS_RECONCILED -> KEYS_ROTATED -> RELEASE_APPROVED -> CERTIFIED`; `FAILED` is terminal from any nonterminal state and never re-enters the sequence. Each transition exact-binds incident ID, prior version/ref/hash CAS, one content-addressed `recovery_cut_selection_ref`, actor authorization, evidence refs, trusted time, exact next JCS bytes and next version; replay is idempotent only for byte-identical input. The strict selection purpose/media are `EP-F57-RECOVERY-CUT-SELECTION-V1` / `application/vnd.ep.f57-recovery-cut-selection-v1+json`. `select_recovery_cut` typed-loads one Task-11 signed checkpoint and its exact `AuthorityRecoveryCutManifestV1`, derives and repeats the common barrier and whole-cut digest plus clean-scan and canonical two-person authorization refs, creates exact JCS/ref once, and returns only `VerifiedRecoveryCutSelectionV1`. It contains no duplicate PostgreSQL/attachment/audit/vault/generation snapshot list. The selection is create-new persisted and codec-reloaded before the incident CAS may enter `CLEAN_CUT_BOUND`; later states reload only that exact ref. `SecurityIncidentAuthorityV1::compose` is the sole cross-crate construction entry and its command authority alone mints selection/create/transition/policy/record wrappers after authorization, evidence and time verification. The store exposes only the eight exact methods shown above—create/adopt or exact load for selection, plus create/adopt, compare-and-append and exact keyed load for incident or recovery state—never latest, scan, overwrite, raw event or caller state mutation. The db-pg adapter persists exact JCS plus indexed CAS in one serialized transaction, calls the flow-owned strict codec on every reload, and returns only verified snapshots. `SecurityIncidentErrorV1` is the exact enum shown above with wires `NOT_FOUND|VERSION_CONFLICT|HASH_CHAIN|TRANSITION_FORBIDDEN|EVIDENCE_MISSING|CUT_POISONED|RECOVERY_DOMAIN_MISMATCH|WRITE_FENCE|CERTIFICATION_INCOMPLETE|STORAGE`; unknown codes fail deserialization.

The strict plain policy purpose/media are exactly `EP-F57-RECOVERY-CERTIFICATION-POLICY-V1` / `application/vnd.ep.f57-recovery-certification-policy-v1+json`; its dedicated schema is the sole wire owner and the flow authority is the sole private constructor. The policy is create-new/content-addressed before the first restore and exact-binds the signed carrier plan, its one outer `execution_attempt_id`, one `certification_id`, and the plan's ordered three distinct subordinate IDs. It has no signer row: each signed recovery raw reaches the same policy ref and authenticates its bytes, while the release verifier independently exact-loads the already signed plan.

The policy object itself is the implicit `UNVERIFIED` genesis; there is no invented genesis record. The first forward row is exactly `sequence=0/INITIAL_RESTORE_VERIFIED`, requires `expected_predecessor=None` and `record.predecessor_ref=None`, and sets `previous_record_sha256` to the whole strict policy object's SHA-256. A direct `UNVERIFIED -> EXPIRED|INVALIDATED` terminal row uses that same `sequence=0/None/policy-digest` genesis rule. Rows `sequence=1/CANDIDATE_MEASURED` and `sequence=2/CERTIFIED`, and any terminal row reached after a persisted forward row, require `expected_predecessor=Some(exact immediately prior record ref)`, repeat that ref in `record.predecessor_ref`, and set `previous_record_sha256` to that record object's digest. Thus the store can write the first row without a fabricated ref while compare-and-append remains fenced and byte-identical crash replay adopts the same object.

The recovery-policy transition graph is exactly `UNVERIFIED -> INITIAL_RESTORE_VERIFIED -> CANDIDATE_MEASURED -> CERTIFIED`; `EXPIRED` is allowed from any of those four states only when trusted time exceeds the fixed policy expiry, and `INVALIDATED` is allowed from any of them on candidate/source-cut/policy/key-domain/malware/integrity drift. `EXPIRED|INVALIDATED` are terminal, reverse/skipped/self edges are forbidden, and byte-identical replay merely adopts the existing record. The three forward rows require exact ordinals `00|01|02`, the corresponding distinct unpredictable `recovery_execution_attempt_id` from the policy/plan, byte-identical candidate/policy/clean-cut/outer-attempt bindings, the exact genesis/predecessor rule above, and issue/expiry no later than the checked 90-day cap. Every signed raw context continues to repeat the one outer carrier attempt; it never substitutes for the subordinate ID. A new candidate, cut, policy revision, key rotation outside the bound evidence, failed re-scan, or expired input invalidates the chain; it can never be “refreshed” by extending time or replacing a predecessor.

Each clean-hardware run restores and exact-verifies the complete canonical row set from the same backup-owned full cut and common barrier; a missing/extra/duplicate/cross-cut row fails even if PostgreSQL starts. Business writes remain fenced until malware, integrity, business reconciliation, key rotation and independent release approval pass. `RecoveryCertificationRecordV1` can be built only after exactly three consecutive complete clean-restore results over the same frozen candidate/policy/checkpoint/cut and three distinct plan-bound recovery subattempts inside one outer carrier attempt; an engineering rehearsal cannot leave `UNVERIFIED`. Migration `92520` creates immutable `platform_ops.recovery_cut_selections` keyed by selection ID and unique content ref/digest, and every incident row stores only `recovery_cut_selection_ref`; selection insertion precedes the incident CAS and response-loss adopts identical bytes. `security_incident_store` is the sole SQL adapter for incident/selection/policy/record state and is composed into ops-agent through its restricted operations identity; flow contains no SQL. The rehearsal persists and resumes the exact selection, incident Objective and hash chains across restart, including the selection-create and compare-and-append acknowledgement cuts. Tests cover every legal edge, forbidden cross-product, predecessor/genesis rules, duplicate/mixed IDs, incomplete authority-row sets, exact 90-day boundary, invalidation and restart adoption.

- [ ] **Step 4: Run one engineering clean-restore rehearsal.**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G6_RELEASE --through 20261025092520`

Expected: PASS on a clean PostgreSQL 16 database through the incident/recovery-cut migration.

Run once on isolated engineering hardware through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_BACKUP_RESTORE_DRILL_V1 -- -Mode EngineeringRehearsal`.

Expected: `ENGINEERING_RESTORE_VERIFIED`. It may find implementation defects but cannot advance `RecoveryCertificationPolicyV1`; Task 15 runs the required three consecutive full clean restores against the final frozen candidate.

Run: `cargo test -p ep-testkit --test f57_ransomware_recovery --test f57_recovery_cut --test f57_security_incident -- --nocapture`

Run: `cargo test -p ep-platform-flow -p ep-adapter-db-pg -p ops-agent -p ep-testkit --all-targets --locked`

Run: `cargo xtask f57 graph generate --check`

Expected: PASS with incident/recovery capability nodes and generated projections exact-bound to the graph digest.

- [ ] **Step 5: Commit clean-restore controls.**

```bash
cargo xtask f57 task stage --task G6-12
cargo xtask f57 task verify-staged --task G6-12
git commit -m "test: prove ransomware clean restore"
```

### Task 13: Implement current P340, UPS, HDD-routing, and capacity certification harnesses

**Files:**
- Create: `scripts/windows/run-p340-certification.ps1`
- Create: `scripts/windows/verify-windows-server-2022.ps1`
- Create: `scripts/windows/verify-bitlocker.ps1`
- Create: `scripts/windows/verify-boot-security.ps1`
- Create: `scripts/windows/verify-residency.ps1`
- Create: `scripts/windows/verify-filesystem-geometry.ps1`
- Create: `scripts/windows/test-power-shutdown.ps1`
- Create after final signing: `scripts/windows/trust/F57_PS_RUN_P340_CERTIFICATION_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_WINDOWS_SERVER_2022_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_BITLOCKER_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_BOOT_SECURITY_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_RESIDENCY_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_VERIFY_FILESYSTEM_GEOMETRY_V1.authenticode.json`
- Create after final signing: `scripts/windows/trust/F57_PS_TEST_POWER_SHUTDOWN_V1.authenticode.json`
- Regenerate after all seven descriptor verifications: `crates/platform/powershell-trust/src/generated_registry.rs`
- Regenerate after all seven descriptor verifications: `docs/generated/f57/powershell-script-registry.v1.json`
- Regenerate after all seven descriptor verifications: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Create: `crates/platform/runtime/src/capacity/p340.rs`
- Create: `crates/platform/runtime/src/storage/runtime_ssd_exceptions.rs`
- Create: `crates/platform/ups-contract/Cargo.toml`
- Create: `crates/platform/ups-contract/src/lib.rs`
- Create: `crates/platform/ups-contract/src/model.rs`
- Create: `crates/platform/ups-contract/src/ports.rs`
- Create: `crates/adapter/ups-windows/Cargo.toml`
- Create: `crates/adapter/ups-windows/src/lib.rs`
- Create: `crates/adapter/ups-windows/src/standard_power_status.rs`
- Create: `crates/adapter/ups-windows/src/signed_vendor.rs`
- Create: `crates/platform/runtime/tests/p340.rs`
- Create: `crates/platform/runtime/tests/runtime_ssd_exceptions.rs`
- Create: `crates/platform/runtime/tests/windows_persistence_and_telemetry.rs`
- Create: `crates/adapter/file/src/p340_qualification_store.rs`
- Create: `crates/adapter/file/tests/p340_qualification_store.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/Cargo.toml`
- Create: `xtask/src/f57/p340_qualification.rs`
- Modify: `xtask/src/f57/mod.rs`
- Modify: `xtask/Cargo.toml`
- Read: `crates/platform/runtime/src/topology.rs`
- Read: `crates/platform/runtime/src/storage/manifest.rs`
- Read: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Read: `docs/schemas/f57-authority-storage-manifest.v1.schema.json`
- Read: `docs/evidence/f57-release-carrier-common.v1.schema.json`
- Read: `docs/evidence/f57-foundation.v1.schema.json`
- Read/import: `crates/platform/gate-journal-contract/src/journal.rs`
- Read/import: `crates/platform/gate-journal-contract/src/port.rs`
- Read: `docs/evidence/f57-gate-run-journal.v1.schema.json`
- Read: `xtask/tests/f57_run_journal.rs`
- Create: `docs/evidence/f57-p340-soak-evidence.schema.json`
- Create: `docs/evidence/f57-ups-contract.v1.schema.json`
- Create: `docs/schemas/f57-runtime-ssd-exception-registry.v1.schema.json`
- Regenerate: `docs/generated/f57/policy/runtime-ssd-exception-registry.v1.json`
- Regenerate: `docs/generated/f57/policy/windows-os-telemetry-data-minimization-policy.v1.json`
- Regenerate: `docs/generated/f57/policy/p340-certification-policy.v1.json`
- Create: `xtask/tests/fixtures/f57-p340-certification-policy-definition-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-certification-policy-attestation-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-capacity-input-manifest-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-server-2022-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-boot-security-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-bitlocker-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-residency-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-filesystem-geometry-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-capacity-certificate-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-workload-timeseries-evidence-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-smart-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-temperature-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-hdd-watermark-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-patch-policy-attestation-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-bitlocker-recovery-custody-attestation-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-ups-power-write-cache-policy-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-hdd-flush-verification-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-ups-identity-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-ups-contract-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-ups-runtime-loss-episode-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-ssd-clean-reinstall-restore-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-ssd-data-hdd-recovery-and-reenrollment-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-runtime-ssd-exception-registry-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-runtime-ssd-reproducible-inventory-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-runtime-ssd-exception-scan-manifest-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-persistent-file-policy-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-os-telemetry-data-minimization-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-p340-memory-test-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-latency-histogram-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-workload-sample-v1-golden.jcs`
- Create: `xtask/tests/fixtures/f57-p340-policy-boundary-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-p340-qualification-plan-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-p340-qualification-journal-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-p340-ssd-clean-reinstall-duration-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-p340-typed-ref-negative-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-p340-soak-evidence-v1-golden.json`
- Read generated: `testkit/tests/f57_p340_capacity.rs`
- Create: `testkit/tests/f57_p340_evaluator.rs`
- Create: `testkit/tests/f57_p340_schema.rs`
- Create: `testkit/tests/f57_power_shutdown.rs`
- Create: `testkit/tests/f57_ups_adapter_contract.rs`
- Create: `testkit/tests/f57_ups_command_reconciliation.rs`
- Create: `testkit/tests/f57_runtime_ssd_residency.rs`
- Create: `testkit/tests/f57_windows_persistence_policy.rs`
- Create: `testkit/tests/f57_windows_telemetry_minimization.rs`
- Create: `testkit/tests/f57_clean_ssd_data_hdd_reenrollment.rs`
- Create: `testkit/src/f57_cases/g6/p340_capacity.rs`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`
- Modify: `crates/platform/runtime/src/capacity/mod.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes exactly the current physical P340/i5-10500/32GB/256GB SSD/1TB HDD plus actual UPS, the G0-owned strict plain `RuntimeTopologyDeclarationV1` only through G1's private verified wrapper/ref, the verified signed authority-storage manifest and the non-expiring P340 policy definition. Candidate-bound capacity/certification and post-infrastructure `RuntimeTopologyCertificationV1` are later release outputs, never pre-qualification inputs. No provider/tenant/vTPM/cache/snapshot/provider-power input is legal in this graph version.
- Produces committed measurement/evaluator tooling only for the physical profile: deterministic non-expiring `P340CertificationPolicyDefinitionV1`, strict `P340QualificationPlanV1`/store, canonical P340 schema/goldens and four P340 Requirement handlers; final 72-hour soak, power-fault evidence and `SINGLE_DISK_DEGRADED_PRODUCTION` certificate remain Task-15-only. `crates/platform/runtime/src/capacity/p340.rs` is the sole current infrastructure-capacity owner. Task 13 modifies neither the gate-journal Rust wire/enum/codec nor its schema. Future IaaS requires a new graph/profile version and separate task/owner family; its reserved canonical schema path is `docs/evidence/f57-iaas-windows-hdd-strict-certification.v1.schema.json`, but this current plan neither creates nor reads it and defines no `iaas_windows_hdd_strict.rs`, handler, test, carrier mapping or positive terminal.

`crates/platform/ups-contract` and `docs/evidence/f57-ups-contract.v1.schema.json` are the sole nominal/schema owners for `UpsAdapterManifestV1`, `UpsStatusReadbackV1`, `UpsRuntimeLossEpisodeV1`, `UpsOutletCycleCommandV1`, `UpsOutletCycleCommandAckV1`, their closed vocabularies and the separated status/control ports; P340, resilience admission and release import them. The plain manifest purpose/media are `EP-F57-UPS-ADAPTER-MANIFEST-V1` / `application/vnd.ep.f57-ups-adapter-manifest-v1+json` and it is transitively authenticated only by `WindowsAuthorityArtifactSetV1.ups_adapter_manifest_ref`, so no signer row is added. Status uses `EP-F57-UPS-STATUS-READBACK-V1` / `application/vnd.ep.f57-ups-status-readback-v1+json`; the strict plain immutable episode uses `EP-F57-UPS-RUNTIME-LOSS-EPISODE-V1` / `application/vnd.ep.f57-ups-runtime-loss-episode-v1+json`; command uses `EP-F57-UPS-OUTLET-CYCLE-COMMAND-V1` / `application/vnd.ep.f57-ups-outlet-cycle-command-v1+json`; ACK uses `EP-F57-UPS-OUTLET-CYCLE-COMMAND-ACK-V1` / `application/vnd.ep.f57-ups-outlet-cycle-command-ack-v1+json`. The episode exact-binds the newest trigger status, deployment/epoch, manifest, identity, configuration, runtime/process and monotonic source, proves `first_stale_observed_at_unix_ms >= valid_until_unix_ms` and checked `recovery_deadline_monotonic_tick_ms=first_stale_monotonic_tick_ms+60000`, and is create-new/adopted before its ref enters the hold cause; no later status changes it. `crates/adapter/ups-windows` executes only inside existing `EPAuthorityControl` from the held candidate kernel: manifest `implementation_binary_ref` byte-equals `WindowsAuthorityArtifactSetV1.authority_kernel_binary_ref`, and that object's reopened digest byte-equals runtime `held_implementation_binary_sha256`. It adds no alternate implementation ref, service, vendor DLL, child process, shell or signer. Manifest, kernel ref, service SID, runtime PID/start-key, configuration generation/digest and live security readback must all agree.

`IAAS_WINDOWS_SERVER_HDD_STRICT` is a future independent seam, not a renamed P340 carrier and not an executable recipe in this plan. Enabling it requires a reviewed graph-version amendment that adds a separate typed profile, schema, handler set, provider/tenant/region/vTPM/media/cache/snapshot/operations-boundary evidence, provider-power equivalence, backup-domain proof, clean-VM recovery, capacity policy, certificate family, acceptance and activation terminal in one coherent change. Until that amendment exists, the current recipe registry, six carrier slots, offline-schema closure, runtime-topology certification and activation authority all reject IaaS before STARTED; no P340 serial, CPU, host fingerprint, physical-UPS evidence, threshold, certificate or acceptance ID may be reused to bridge that seam.

`UpsAdapterVersionV1` has only a validated constructor and hand-written `Deserialize`; it accepts canonical Semantic Versioning 2.0.0 grammar without build metadata and exposes no `Default` or unchecked `From<String>`. `WindowsDeviceInterfaceClassGuidV1` likewise has only a validated constructor/hand-written `Deserialize` and accepts exactly lowercase hyphenated 36-byte GUID text without braces. The first frozen timing tuple is exactly `status_poll_interval_seconds=5`, `maximum_status_age_seconds=15`, `maximum_self_test_age_seconds=86400`, `maximum_command_ack_seconds=30`. `configuration_projection.configuration_generation` is positive and contains the only deployment selection; endpoint and credential remain solely in the manifest and cannot be overridden. `WINDOWS_STANDARD_POWER_STATUS` has an exact empty `supported_device_profiles`, combines only `WindowsSystemPowerStatus` plus credential `NONE`, declares exactly `{READ_AC_INPUT,READ_BATTERY_SUPPLY,READ_BATTERY_CHARGE,READ_REMAINING_RUNTIME}`, and has all three optional selection fields null. `SIGNED_VENDOR_ADAPTER` has a nonempty profile vector strictly sorted and duplicate-free by `device_profile_id`; every row has nonempty manufacturer/model, nonempty strictly sorted duplicate-free `firmware_revisions` and `controlled_outlet_group_ids`, the projection exact-selects one profile and one member outlet group with nonnull P340 power-path digest, and capabilities are the exact ten-value set.

Local-device transport pairs only with `SERVICE_SID_DEVICE_ACL_ONLY`; its device-instance ID is 1..200 ASCII bytes, uppercase canonical Configuration-Manager form with no NUL, `device_instance_id_sha256=SHA256(UTF-8 canonical ID)`, and the ACL parser exact-matches GUID/vendor/product/instance. HTTPS mutual TLS pairs only with `CNG_NON_EXPORTABLE_CLIENT_CERTIFICATE`; SNMPv3 authPriv pairs only with `DPAPI_NG_SERVICE_SID_SEALED_SECRET`. Network transport uses `UpsCanonicalIpAddressV1` numeric octets—never text, DNS name, IPv4-mapped alias or alternate IPv6 spelling—and `NonZeroU16` port inside one `UpsNetworkDestinationV1`; DNS/proxy/redirect booleans are all false. Runtime destination rows are strictly sorted/unique; standard/local vectors are empty and network is exactly the one manifest destination including peer identity. Any other carrier/profile/transport/credential/capability/destination combination is invalid.

The common status wire remains honest across both carriers. In every identity/status/command/ACK, `configuration_generation=manifest.configuration_projection.configuration_generation` and `adapter_configuration_sha256=SHA256(JCS(manifest.configuration_projection))`; no arbitrary digest, ambient configuration file or later override exists. Standard status has `device_profile_id=null` and logical `ups_adapter_identity=sha256(JCS({carrier_kind,adapter_manifest_ref,adapter_configuration_sha256,configuration_generation}))`; this identifies only the Windows carrier/configuration and never claims a physical UPS identity. Vendor status has `device_profile_id=Some(manifest.configuration_projection.selected_device_profile_id)` and hardware `ups_adapter_identity=sha256(JCS({carrier_kind,adapter_manifest_ref,device_profile_id,manufacturer,model,serial_number,firmware_revision}))`; the signed vendor-only identity readback and host fingerprint repeat that digest, selected outlet group and protected power path, while outlet command and ACK repeat those fields directly. Vendor status has no outlet-group or protected-path field: it repeats the manifest ref, configuration digest/generation, selected profile and hardware identity, and thereby binds the selected outlet group and protected power path only transitively through the exact manifest plus `configuration_projection`. Every status has `runtime_security_binding_sha256=SHA256(JCS(UpsIdentityReadbackV1.runtime_security_readback))`. The identity's initial status and POWER previous/trigger statuses exact-match that digest; previous and trigger also share the same boot/PID/process-start binding. Within that verified `process_start_key`, `status_sequence` starts at 1 and strictly increases; a process restart requires a new signed identity/runtime binding before a new sequence 1, and cross-binding sequence comparison or ref substitution fails. `valid_until_unix_ms` checked-equals `observed_at_unix_ms + 15000`, collection is scheduled every 5 seconds, a known battery percentage is in `0..=100`, and `active_alerts` is enum-sorted and duplicate-free. Freshness is exactly half-open: accept only when `trusted_now_unix_ms < valid_until_unix_ms`; equality and every later observation are stale. The supervisor records the first stale observation on the trusted monotonic source, and only that monotonic tick drives the 60-second deadline.

The runtime supervisor consumes that expiry as a hard safety transition. At the first observation satisfying `trusted_now_unix_ms >= valid_until_unix_ms`—including equality—it creates or adopts one immutable `UpsRuntimeLossEpisodeV1` that freezes the original identity/configuration/runtime binding and the first stale monotonic tick, then synchronously creates or adopts the deployment-wide `ProductionAdmissionHoldV1{cause=UPS_RUNTIME_LINK_LOSS{runtime_loss_episode_ref}}`; the unique route gate rejects new request leases and new long-job starts, and a displayed AC value cannot suppress the hold. The recovery deadline is checked `first_stale_monotonic_tick+60000` and never moves after intermittent samples. Hold release requires exactly two consecutive status sequences from the same frozen identity/binding, each independently satisfying `trusted_now_unix_ms < valid_until_unix_ms` at consumption and each communication/self-test/output/runtime PASS, followed by the cause-bound upper admission CAS. Identity change, one pass, PASS/UNKNOWN/PASS, stale second pass or arrival after the aggregate deadline cannot release. At deadline without closure, the control broker drains or positively reconciles accepted leases, executes the bounded local barrier, audit/Outbox/attachment fsync, fresh PostgreSQL checkpoint, database stop and Windows shutdown sequence even when AC remains online. Outlet control is optional for this fail-safe branch: inability to control it or obtain same-command typed ACK means no external ACK claim, not permission to skip local shutdown. Restart remains manual/held until fresh device, power, DATA_HDD and PostgreSQL recovery evidence plus the same cause-bound admission CAS. Crash and flapping tests cut equality, one tick before/after and every hold, sample, 60-second deadline, checkpoint, stop and shutdown boundary and prove no timer reset or premature reopen.

Standard status has `self_test=UNKNOWN`. Vendor `PASSED|FAILED` carries a nonnull `provider_attestation_ref` with exact `application/octet-stream` media parsed only by UPS-owner field parser `EP_F57_UPS_SELF_TEST_ATTESTATION_V1`; authenticated bytes exact-bind adapter/profile/hardware identity, serial/firmware, provider test ID, result and completion time. For P340 qualification and POWER, only `PASSED` is eligible and checked `0 <= status.observed_at_unix_ms-completed_at_unix_ms <= 86400000`; future, stale, missing, generic or cross-device attestation fails. Operation 05 remains `READ_ONLY`: the system reads this fresh provider-authenticated result and blocks until an operator/vendor has actually run a self-test; it never fabricates a test or silently turns a read into an unjournaled side effect.

The adapter persists its private operation row and samples `adapter_call_started_monotonic_tick_ms` from the command's exact `pre_shutdown_boot_id|boot_monotonic_source_id` **before** the first provider call can occur; crash/adopt may load but never resample or reset that marker. A scheduled provider response must return a canonical nonempty 1..128 ASCII `provider_operation_id` matching `[A-Za-z0-9][A-Za-z0-9._:-]{0,127}`; the adapter durably binds it to `(ups_adapter_identity,command_id,command_sha256)` before ACK creation, and every provider schedule response, exact-ID query and operation-log readback must repeat it byte-for-byte. Empty, normalized-alias, changed or cross-command provider IDs are `COMMAND_STATE_UNKNOWN`, never grounds to resend. A provider result is admissible only when the ACK repeats that boot/source and checked arithmetic proves `adapter_call_started_monotonic_tick_ms <= acknowledgement_observed_monotonic_tick_ms <= min(adapter_call_started_monotonic_tick_ms + 30000, command.dispatch_deadline_monotonic_tick_ms)`. Response loss may query/adopt the already scheduled operation only inside that same inner deadline and must return the byte-identical ACK; at the inner deadline, an absent or unqueryable result becomes `COMMAND_STATE_UNKNOWN` with no new send. `first_dispatch_started_at_unix_ms|accepted_at_unix_ms` are reporting-only UTC observations and never authorize, order or time out the command. The separate 600-second POWER deadline governs only the outer User32/composite/preshutdown reconciliation and cannot extend or revive the 30-second adapter window.

Runtime-security optionals are a closed matrix: standard has both refs absent and an empty destination vector; local-device has a present device-ACL ref, a present deny-all outbound-firewall ref and an empty destination vector; network has no device-ACL ref, a present exact-endpoint firewall ref and exactly one structured destination row byte-equal to the manifest destination. Both optional field artifacts use `application/octet-stream` and are parsed only by UPS-owner field parsers `EP_F57_UPS_DEVICE_ACL_READBACK_V1` and `EP_F57_UPS_FIREWALL_READBACK_V1`; they add no schema, signer or release POWER opaque-parser row. Every mode requires zero unexpected loaded modules, zero child processes and zero credential export/secret exposure.

`WINDOWS_STANDARD_POWER_STATUS` is monitoring-only: Windows unknown values remain typed `UNKNOWN`, communication/self-test/output remain `UNKNOWN`, and its control port always returns `CAPABILITY_INSUFFICIENT`. It can never satisfy the highest production tier, controlled-outlet policy or POWER evidence. Those require a candidate-bound `SIGNED_VENDOR_ADAPTER` with the exact status/query/idempotent-control capability set. A local USB adapter has zero network and a device ACL limited to SYSTEM plus the `EPAuthorityControl` service SID. A network adapter permits exactly one manifest `UpsNetworkDestinationV1` whose address is canonical IPv4/IPv6 numeric octets with nonzero port, protocol and pinned peer identity; textual IP/DNS aliases, proxy, redirect and every extra socket are forbidden. Credentials are either a nonexportable service-bound CNG key or DATA_HDD DPAPI-NG service-SID-sealed secret and never appear in argv, environment, log, evidence or ordinary configuration.

UPS schema/unit/byte goldens reject an alternate implementation ref/digest, arbitrary configuration digest/generation or illegal standard/vendor selection projection; noncanonical/build-metadata adapter versions or GUID/device IDs; DNS/textual/IPv4-mapped/aliased IP, port zero, duplicate/extra destination; a standard manifest with any profile or a standard status with nonnull profile/non-UNKNOWN self-test; a vendor manifest with empty/unsorted/duplicate profiles, firmware or outlet groups; vendor profile/hardware/identity drift; missing/cross-process runtime binding, sequence rollback/reset without a new signed identity; any expiry other than `+15000ms`; percentage outside `0..=100`; unsorted/duplicate alerts; stale/future/missing/cross-device self-test attestation; empty/noncanonical/changed/cross-command provider operation ID; a missing/resampled/private start marker, boot/source drift, monotonic overflow, acknowledgement observation after the inner `30000ms` deadline, query/adopt after that deadline, UTC mutation changing a verdict, and any attempt to use the `600000ms` composite window as an adapter-timeout override.

Runtime-loss goldens additionally prove the half-open boundary: one tick before expiry is fresh, equality and one tick after are stale and create/adopt the immediate global tagged hold with no new long-job admission. They cover two same-identity fresh PASS samples completed at the inclusive 60-second recovery deadline, one pass only, identity/config/runtime drift, PASS/UNKNOWN/PASS, a second PASS one tick late, flapping that attempts to reset the first-stale monotonic tick, AC-online after timeout, crash at every fail-safe phase, outlet unavailable and missing ACK. Only the exact two-pass deadline boundary may reopen; every other case reaches or remains in local checkpoint/database-stop/Windows-shutdown with manual fresh-evidence recovery.

The command carries no endpoint, credential, executable path, argv or raw vendor payload. For the same `(ups_adapter_identity,command_id)`, the same command digest must query/adopt and return the byte-identical typed ACK; a changed digest is `COMMAND_ID_CONFLICT`, and an unqueryable outcome is `COMMAND_STATE_UNKNOWN` with no resend. `NEW|ADOPTED` is private execution metadata, never an ACK field. A boot change before the composite ACK is durable remains the existing `DISPATCH_ACKNOWLEDGEMENT_ABSENT_AFTER_BOOT_CHANGE`; postboot visibility cannot reconstruct PASS. P340 and POWER real-hardware matrices mutate every manifest/config/service/firmware/transport/credential field and exercise call-before/after-accept/response-loss/process-loss/boot-change cuts.

After the fixed P340 signer issues the one candidate-bound policy attestation, one qualification authority captures one trusted issue time, one unpredictable `qualification_id`, seven unpredictable operation execution-attempt IDs and a strict plain `P340QualificationPlanV1` before any helper side effect. The plan exact-binds that attestation and its expiry is the checked minimum of issue time plus the 90-day cap, journal-header expiry and policy-attestation expiry while still covering the latest possible 72-hour finish. The exact ordinal/class sequence is `01 WINDOWS_PATCH_POLICY_ATTESTATION/READ_ONLY`, `02 BITLOCKER_RECOVERY_CUSTODY_ATTESTATION/READ_ONLY`, `03 UPS_POWER_WRITE_CACHE_POLICY/READ_ONLY`, `04 HDD_FLUSH_POWER_CUT_VERIFICATION/SIDE_EFFECTING`, `05 UPS_IDENTITY_READBACK/READ_ONLY`, `06 SSD_CLEAN_REINSTALL_RESTORE/SIDE_EFFECTING`, `07 MEMORY_TEST_READBACK/SIDE_EFFECTING`. The runtime authority alone constructs private `VerifiedP340QualificationStoreV1` from the verified DATA_HDD root, verified journal header/run ID, frozen plan and qualification ID. Through that wrapper, `P340QualificationStoreV1` derives—never accepts from argv—the root `runs/<gate-run-id>/p340-qualification/<qualification-id>/`, `plan.v1.json`, and exact outputs `01-windows-patch-policy-attestation.v1.json`, `02-bitlocker-recovery-custody-attestation.v1.json`, `03-ups-power-write-cache-policy.v1.json`, `04-hdd-flush-power-cut-verification.v1.json`, `05-ups-identity-readback.v1.json`, `06-ssd-clean-reinstall-restore.v1.json`, and `07-memory-test-readback.v1.json`. Its API is only create/adopt plan, create/adopt exact ordinal output, typed load and fixed-prefix checkpoint; caller paths, directory scans and generic media are impossible. Plan/output create-new, fsync, reload, lease and non-reparse rules forbid scans, overwrite and path/media aliases; the plain plan is authenticated by journal closure and gains no signer row. Crash goldens cover every cut before/after create, fsync, journal terminal and closure.

Task 13 activates exactly five qualification semantics on the sole G0 gate journal: `P340_QUALIFICATION_OPERATION_STARTED|COMPLETED|UNKNOWN|RECONCILED|CLOSURE_BOUND`. All five variants and their exact field sets were already reserved and parsed by `ep-platform-gate-journal-contract`; Task 13 supplies only the runtime-owned domain handler/reconciler and registers it in xtask composition. Immediately before each operation it durably records STARTED with exactly five fields `{qualification_id,operation_kind,execution_attempt_id,qualification_plan_ref,started_at_unix_ms}`; the fixed output locator comes only from the authenticated plan and an `output_ref` or any sixth field in STARTED is invalid. Only then does the compiled kind-to-executor mapping run. A started operation is never rerun, including read-only rows. After a crash the successor first records UNKNOWN and may only typed-load the plan-derived fixed output; exact same-attempt bytes permit RECONCILED, while absence/partial/conflict remains UNKNOWN and fails this run. Output refs occur only in COMPLETED, RECONCILED and CLOSURE_BOUND. Exactly seven terminal outputs precede CLOSURE_BOUND and its `GateRunJournalCheckpointV1`; the capacity input exact-repeats the plan, seven refs and that checkpoint, and the P340 carrier cannot reach `TEST_STARTED` until closure, checkpoint, capacity input and staging plan are durable. Task 14 later activates the remaining eight reserved semantics, so the final F57 journal delta is exactly 13; neither task changes the journal type, enum, codec or schema.

All seven Task-13 scripts are Authenticode/RFC-3161-signed after their final edits, strict-verified into the seven listed descriptors and invoked only through the G0 fixed-host executor. `run-p340-certification` may dispatch its six nested collectors only by compiled `PowerShellScriptIdV1`; the POWER preparation path uses the same mechanism. A raw child path, unsigned collector, post-sign edit, stale descriptor or changed call-graph edge fails before physical side effects.

Ordinal 06 duration has exactly two authorities: start is the durable `P340_QUALIFICATION_OPERATION_STARTED.started_at_unix_ms`, and finish is the same-attempt signed `SsdCleanReinstallRestoreReadbackV1.completed_at_unix_ms`; the helper's own `started_at_unix_ms` must byte-equal the journal STARTED value. Require signed-helper completion `>=` STARTED, then checked `delta_ms=helper.completed_at_unix_ms-STARTED.started_at_unix_ms` and `duration_seconds=(delta_ms+999)/1000`. Exactly `28_800_000 ms` yields `28_800 s` and passes; `28_800_001 ms` yields `28_801 s` and fails. `P340_QUALIFICATION_OPERATION_COMPLETED` carries the fixed output ref, not a substitute finish timestamp; a live clock, process timer, payload duration claim, journal-completion resample or restart-time resample is forbidden. SSD contains runtime/temporary bytes only; every qualification output, journal, plan, capacity input and final evidence object persists on the verified DATA_HDD root.

- [ ] **Step 1: Write failing workload-mix and honest-profile tests.**

```rust
#[test]
fn p340_certificate_never_claims_redundancy() {
    let cert = evaluate_p340(valid_measurements()).unwrap();
    assert_eq!(cert.payload.certified_storage_profile, P340StorageProfileV1::SingleDiskDegradedProduction);
    assert_eq!(cert.payload.certified_concurrent_users, 20);
    assert!(!cert.payload.high_availability_certified);
}

#[test]
fn p340_schema_owns_the_complete_exact_wire_set() {
    assert_eq!(p340_nested_root_set(), [
        "BitLockerReadbackV1", "BootSecurityReadbackV1",
        "FileSystemGeometryReadbackV1", "P340CapacityCertificateV1",
        "P340HddWatermarkReadbackV1", "P340SmartReadbackV1",
        "P340TemperatureReadbackV1", "P340WorkloadTimeseriesEvidenceV1",
        "ResidencyReadbackV1", "WindowsServer2022ReadbackV1",
    ]);
    assert_eq!(p340_supporting_root_set(), [
        "BitLockerRecoveryCustodyAttestationV1", "HddFlushVerificationV1",
        "P340MemoryTestReadbackV1", "SsdCleanReinstallRestoreReadbackV1",
        "UpsIdentityReadbackV1", "UpsPowerWriteCachePolicyV1",
        "WindowsPatchPolicyAttestationV1",
    ]);
    assert_all_p340_goldens_round_trip_byte_for_byte();
}

#[test]
fn p340_qualification_is_one_seven_operation_crash_safe_attempt() {
    assert_eq!(p340_qualification_plan().rows.len(), 7);
    assert_exact_qualification_order_and_classes_01_through_07();
    assert_eq!(p340_qualification_delta_event_kinds(), [
        "P340_QUALIFICATION_OPERATION_STARTED",
        "P340_QUALIFICATION_OPERATION_COMPLETED",
        "P340_QUALIFICATION_OPERATION_UNKNOWN",
        "P340_QUALIFICATION_OPERATION_RECONCILED",
        "P340_QUALIFICATION_CLOSURE_BOUND",
    ]);
    assert_start_is_durable_before_each_operation_and_started_operation_never_reruns();
    assert_crash_adopts_only_exact_fixed_output_then_reconciles();
    assert_closure_checkpoint_precedes_capacity_input_and_carrier_started();
}

#[test]
fn clean_ssd_reinstall_duration_uses_started_event_and_signed_helper_finish() {
    assert_eq!(ssd_restore_duration_seconds_from_started_and_signed_helper(0, 28_800_000).unwrap(), 28_800);
    assert_eq!(ssd_restore_duration_seconds_from_started_and_signed_helper(0, 28_800_001).unwrap(), 28_801);
    assert!(evaluate_ssd_restore_duration_from_started_and_signed_helper(0, 28_800_000).is_ok());
    assert_code(evaluate_ssd_restore_duration_from_started_and_signed_helper(0, 28_800_001), "P340_SSD_RESTORE_DURATION_EXCEEDED");
    assert_helper_started_at_exactly_equals_ordinal_six_started_event();
    assert_no_journal_completion_time_live_clock_or_payload_claim_can_replace_signed_helper_finish();
}

#[test]
fn p340_policy_requires_all_25_metric_variants_and_exact_predicates() {
    assert_eq!(p340_required_metrics(), [
        "ACTION_MIX", "ACTIVE_PRINCIPAL_MIX", "AUDIT_CHECKPOINT", "AUTOMATION_EFFECTS",
        "BACKUP_LAG", "COMMIT_MEMORY", "CONTROL_CENTER_LANE", "CPU",
        "CPU_SSD_HDD_TEMPERATURE", "ERROR_RATE", "HDD_FREE_AND_GROWTH", "HDD_HEALTH",
        "HDD_LATENCY", "HDD_QUEUE", "HDD_THROUGHPUT", "HEAVY_REPORT_ADMISSION",
        "HTTP_WORKER_PLUGIN_QUEUES", "PAGE_FAULTS", "POSTGRES_COMMIT_WAL_LOCKS_CHECKPOINT",
        "POSTGRES_CONNECTIONS", "READ_LATENCY", "SSD_HEALTH", "UPS_HEALTH",
        "WORKING_SET", "WRITE_LATENCY",
    ]);
    assert_all_25_metrics_have_deterministic_predicates();
    assert_all_scalar_boundaries_pass_at_equality_and_fail_one_beyond();
    assert_code(postgres_connections_with_active_above_maximum(), "P340_POSTGRES_ACTIVE_EXCEEDS_MAXIMUM");
    assert_initial_hdd_ratio_cross_multiplication_boundaries_and_one_byte_negatives();
    assert_hdd_watermark_uses_checked_exact_ceil_five_percent_and_20_gib_floor();
    assert_capacity_history_is_qualification_synthetic_exact_90_utc_days_with_30_day_suffix();
    assert_recomputed_nearest_rank_30_and_90_day_p95s_match_input_manifest();
    assert_real_wmi_cpu_golden_maps_only_to_typed_i5_10500_sku();
    assert_raw_device_and_ntfs_volume_capacities_cross_all_input_geometry_and_sample_wires();
    assert_initial_queues_are_exact_graph_derived_identity_set_without_runtime_scalars();
}

#[test]
fn latency_is_recomputed_from_the_fixed_histogram_not_averaged_percentiles() {
    let merged = merge_histograms(valid_sharded_histograms()).unwrap();
    assert_eq!(merged.operation_count, merged.bucket_count_sum());
    assert_eq!(p95_from_histogram(&merged), expected_p95_ms());
    assert_code(histogram_with_bucket_sum_drift(), "P340_HISTOGRAM_COUNT_MISMATCH");
    assert_code(average_of_shard_p95s(), "P340_PERCENTILE_AGGREGATION_FORBIDDEN");
}

#[test]
fn p340_sample_series_histograms_and_counters_are_exact() {
    let series = valid_workload_series();
    assert_eq!(series.samples.len(), 4321);
    assert_eq!(series.sequence_range(), 1..=4321);
    assert_first_and_last_schedule_equal_common_interval_bounds(&series);
    assert_observations_within_five_seconds_strictly_increasing_and_in_interval(&series);
    assert_one_boot_id_and_monotonic_source_with_boot_scoped_delta(&series);
    assert_complete_frame_hash_chain_and_final_root(&series);
    assert_each_sample_has_distinct_15_3_2_principals_plus_distinct_control_center(&series);
    assert_each_sample_has_exact_action_mix_overlays_and_all_25_metrics(&series);
    assert_eq!(fixed_latency_bucket_wires().len(), 16);
    assert_histogram_maximum_and_nearest_rank_rules(&series);
    assert_every_frame_histogram_count_covers_action_mix(&series);
    assert_every_error_rate_operation_count_is_checked_sum_of_read_write_high_risk_and_attachment_counts(&series);
    assert_every_error_rate_completed_count_covers_the_matching_action_mix(&series);
    assert_cumulative_counters_start_zero_never_decrease_and_match_final_summary(&series);
    assert_wal_starts_zero_is_monotonic_and_finishes_positive(&series);
    assert_sample_one_capacity_and_each_frame_readback_exact_match_input_host_and_outer_context(&series);
    assert_code(read_histogram_count_below_action_mix(), "P340_HISTOGRAM_ACTION_COUNT_INCOMPLETE");
    assert_code(write_histogram_count_below_action_mix(), "P340_HISTOGRAM_ACTION_COUNT_INCOMPLETE");
    assert_code(merged_histogram_with_zero_operations(), "P340_HISTOGRAM_OPERATION_COUNT_ZERO");
    assert_code(percentile_landing_in_over_10000_bucket(), "P340_LATENCY_P95_UNBOUNDED");
}

#[test]
fn every_p340_ref_is_typed_and_bound_to_one_host_interval_policy_and_input() {
    assert_code(cross_host_subordinate_ref(), "P340_HOST_BINDING_MISMATCH");
    assert_code(cross_interval_measurement_ref(), "P340_INTERVAL_BINDING_MISMATCH");
    assert_code(generic_octet_ref_for_named_helper(), "P340_TYPED_REF_REQUIRED");
    assert_code(aliased_measurement_refs(), "P340_MEASUREMENT_REF_ALIAS");
    assert_p340_media_and_purpose_wires_exact();
    assert_pre_start_policy_then_helpers_then_input_are_frozen_before_test_started();
    assert_p340_nonaggregate_payload_and_trusted_signing_windows_are_exact();
    assert_cross_signed_time_order_and_exact_259200000_millisecond_context();
}

#[test]
fn p340_rejects_wrong_machine_security_or_residency() {
    assert_code(windows_2019_readback(), "P340_WINDOWS_PRODUCT_MISMATCH");
    assert_code(secure_boot_or_tpm_disabled(), "P340_BOOT_SECURITY_INCOMPLETE");
    assert_code(bitlocker_data_volume_unprotected(), "P340_BITLOCKER_INCOMPLETE");
    assert_code(authority_bytes_found_on_ssd(), "P340_AUTHORITY_RESIDENCY_VIOLATION");
    assert_code(cross_host_subordinate_ref(), "P340_HOST_BINDING_MISMATCH");
    assert_code(capacity_claiming_ha(), "P340_HA_CLAIM_FORBIDDEN");
    assert_code(scanner_definition_from_future(), "P340_SCANNER_DEFINITION_TIME_INVALID");
    assert_code(scanner_age_with_floor_rounding(), "P340_SCANNER_DEFINITION_AGE_MISMATCH");
    assert!(evaluate_p340(scanner_provider_drift_from_host_input()).is_err());
    assert!(evaluate_p340(scanner_policy_digest_zero_or_drift()).is_err());
    assert!(evaluate_p340(scanner_engine_version_empty()).is_err());
    assert!(evaluate_p340(scanner_definition_digest_not_bound_to_authenticated_issued_metadata()).is_err());
    assert_code(w32time_offset_i64_min(), "P340_W32TIME_OFFSET_INVALID");
    assert_code(smart_summary_not_recomputed_from_full_interval(), "P340_SMART_SUMMARY_MISMATCH");
    assert_code(smart_with_unsafe_shutdown_increment(), "P340_SMART_UNSAFE_SHUTDOWN_INCREMENT");
    assert_code(temperature_series_missing_one_of_4321_sequences(), "P340_TEMPERATURE_SERIES_INCOMPLETE");
    assert_code(temperature_with_negative_or_sentinel_value(), "P340_TEMPERATURE_VALUE_INVALID");
    assert_code(ups_identity_with_empty_required_field(), "P340_UPS_IDENTITY_INCOMPLETE");
    assert_code(unhealthy_ups_identity(), "P340_UPS_COMMUNICATION_UNHEALTHY");
    assert_code(hdd_cache_enabled_without_power_loss_flush_policy(), "P340_HDD_CACHE_POLICY_UNSAFE");
    assert_code(hdd_flush_without_forced_cut(), "P340_HDD_FLUSH_CUT_MISSING");
    assert_code(hdd_flush_with_acknowledged_loss(), "P340_HDD_FLUSH_WRITE_LOSS");
    assert!(evaluate_p340(hdd_flush_created_after_test_started()).is_err());
    assert!(evaluate_p340(hdd_flush_for_another_host_or_serial()).is_err());
    assert!(evaluate_p340(hdd_flush_expiring_before_soak_finish()).is_err());
    assert!(evaluate_p340(reboot_or_storage_drop_or_retry_during_soak()).is_err());
    assert!(evaluate_p340(cross_signed_timestamp_order_violation()).is_err());
    assert!(evaluate_p340(helper_requiring_payload_time_equal_later_tsa_time()).is_err());
    assert!(evaluate_p340(helper_signed_outside_five_minute_or_input_expiry_window()).is_err());
    assert!(evaluate_p340(neighbor_or_ambiguous_cpu_sku()).is_err());
    assert!(evaluate_p340(raw_capacity_below_volume_or_role_floor()).is_err());
    assert!(evaluate_p340(filesystem_volume_total_drift()).is_err());
    assert!(evaluate_p340(initial_queue_with_depth_or_age_scalar()).is_err());
    assert_context_bound_raw_and_nested_signatures_use_physical_finish_window();
    assert!(evaluate_p340(context_bound_readback_with_none_or_before_finish_signature_window()).is_err());
    assert!(evaluate_p340(context_bound_readback_signed_after_finish_plus_five_minutes_or_expiry()).is_err());
    assert_code(device_health_true_over_failed_predicate(), "P340_DEVICE_HEALTH_SUMMARY_MISMATCH");
    assert_all_p340_u64_max_and_i64_extreme_arithmetic_goldens_fail_closed();
}
```

- [ ] **Step 2: Run the authored synthetic evaluator and verify RED.**

Run: `cargo test -p ep-testkit --test f57_p340_evaluator --test f57_p340_schema --test f57_power_shutdown --test f57_ups_adapter_contract --test f57_ups_command_reconciliation --test f57_runtime_ssd_residency --test f57_windows_persistence_policy --test f57_windows_telemetry_minimization --test f57_clean_ssd_data_hdd_reenrollment -- --nocapture`

Run: `cargo test -p ep-platform-ups-contract -p ep-adapter-ups-windows --all-targets --locked`

Expected: FAIL.

- [ ] **Step 3: Implement the exact workload envelope.**

```rust
pub const P340_SESSIONS: P340SessionMixV1 = P340SessionMixV1 { workbench: 15, customer_portal: 3, supplier_portal: 2, control_center_reserved: 1 };
pub const P340_ACTIONS: P340ActionMixV1 = P340ActionMixV1 { reads: 11, writes: 5, high_risk_commands: 2, attachment_operations: 2 };
```

Overlay automation, incremental backup, audit checkpoint, and one heavy report. Capture latency/error/queue, memory, CPU, HDD throughput/latency/fill/growth, SMART, temperature, PostgreSQL connections, backup lag, UPS health, and shutdown ordering for 72 continuous hours.

`run-p340-certification.ps1` is the only P340 orchestrator. In `Release72Hour` it invokes the fixed AllSigned collectors itself, with no caller-supplied script/command or evidence identity; Rust typed-verifies the exact ten nested envelopes and all reachable named supporting envelopes, then constructs the capacity certificate and outer soak wire. Every nested root carries one `P340EvidenceBindingV1`; its byte-identical context, signed-candidate ref, certification-policy-attestation ref, and capacity-input-manifest ref bind one release candidate, `CandidateRunIdentityV1`, unpredictable execution-attempt ID, trusted runner, physical host fingerprint, common interval, expiry, immutable policy, and qualified input. The ten nested media/purpose pairs and seven helper media/purpose pairs are exactly those frozen in master §§3 and 5. A generic ref, alias, cross-host/run/attempt/interval/policy/input helper, wrong media/purpose/signer, or unreachable supporting envelope fails closed.

Every context-bound raw carrier or nested physical readback that has `ReleasePhysicalEvidenceContextV1` but no own issue field receives one stable generated artifact type ID and the sole G0 `CmsSignableArtifactV1` generated implementation. Its exhaustive issuance descriptor selects the exact inclusive rule `[context.finished_at_unix_ms,min(context.finished_at_unix_ms+300000,context.expires_at_unix_ms)]` under checked `i128`; there is no parallel `BusinessArtifactIssuanceV1` trait. Signing can begin only through `prepare_cms_signing_request_v1`, and the architecture check exact-joins every such payload to one generated descriptor. The rule may never return no window, sign before physical finish, sign after the five-minute/expiry bound, or alter context/window during crash recovery. Signer and offline verifier both prove the actual CMS/RFC-3161 time lies in that window; they never require a payload time to equal the later TSA time.

The six security/capacity nested roots plus four measurement roots each carry the outer soak's byte-identical `ReleasePhysicalEvidenceContextV1`; their explicit candidate ref must exact-match that context's `SIGNED_CANDIDATE` binding. The capacity certificate's sorted `measurement_refs` is exactly the five non-certificate security readbacks plus the four measurement roots, all nine distinct. The outer soak repeats the four measurement refs and exact-matches the certificate. Input host fingerprint, Windows readback, workload time series, capacity certificate and outer soak also cross-check the same hardware profile, typed `P340CpuSkuV1::IntelCoreI5_10500`, `34359738368`-byte RAM/module sum, SSD/HDD roles, volume identities, raw-device capacities and NTFS volume totals; the first workload frame's HDD `used_bytes` and `free_bytes` exact-match input fill and checked-sum to DATA_HDD `volume_total_bytes`. The evaluator rejects a different attempt ID, runner, host, hardware/capacity scalar, start/finish/expiry window, candidate/run, signer, aliased ref, or missing measurement even when each individual readback would otherwise PASS.

The only legal definition is the deterministic projection `docs/generated/f57/policy/p340-certification-policy.v1.json`, with media `application/vnd.ep.f57-p340-certification-policy-definition-v1+json`, purpose `EP-F57-P340-CERTIFICATION-POLICY-DEFINITION-V1`, and ID `F57_P340_CERTIFICATION_POLICY_V1`. Before the qualification plan and every helper side effect—and therefore before the input manifest and physical STARTED—the fixed assessor signs one `P340CertificationPolicyAttestationV1` that binds that exact projection member, final candidate and projection manifest. For that policy attestation, the capacity-input manifest and all seven pre-start helpers, payload `issued_at_unix_ms|generated_at_unix_ms` is captured before hashing; actual trusted CMS/RFC-3161 signing time must be in `[payload_time,min(payload_time+300000,payload_expiry)]` under checked `i128`, never required to equal a time learned after the payload digest exists. Payload and signing times are `<=TEST_STARTED.recorded_at_unix_ms`, every expiry covers `context.finished_at_unix_ms`, and `expiry<=payload_time+7776000000`. An empty/inverted window, signature after any input expiry, five-minute drift, changed crash-recovery payload time, or equality-to-future-TSA contract fails closed. The definition fixes `continuous_duration_seconds=259200`, sample/gap `60/120` seconds, `maximum_retry_basis_points=100`, and the complete exact 25-variant `P340RequiredMetricV1` set. Read/write P95 are `<=2000/3000 ms`; errors are exactly zero; CPU P95/every sample are `<=8500/9500` basis points; working/commit memory are `<=17179869184/25769803776` bytes; adjacent-sample hard-page-fault delta is `<=3000`; HDD read/write P95 are `<=100/200 ms`; HDD queue P95/max are `<=4/16`; each read/write throughput is positive; HDD free is always at least `1500` basis points; SSD free/rollback budgets are at least `42949672960/21474836480` bytes. PostgreSQL configured/active connections are `<=64/48` and every sample also requires `active<=maximum`; commit P95 is `<=500 ms`, waiting locks `<=4`, long transactions `=0`, and checkpoint age `<=900 s`. HTTP/worker/plugin queues are `<=40/200/40`; automation due/running/unknown are `<=1000/64/0`, exact-once counters are zero, and every sample uses checked `u128` to require `retry_count*10000 <= maximum_retry_basis_points*accepted_command_count`; `accepted_command_count=0` requires `retry_count=0`, while the final aggregate additionally requires positive accepted commands. Backup age/lag are `<=900/300 s`; audit checkpoint age/pending are `<=300 s/100`; UPS is online/healthy with runtime `>=900 s`; Control Center response P95/queue are `<=2000 ms/10`; heavy-report decision is `<=2000 ms` with background concurrency exactly `1`. Goldens cover equality, one retry over, `0/0`, retry-positive/accepted-zero, final accepted zero, and each cross-product overflow.

The same policy requires each capacity-relevant table to have at least `10000` rows, at least `10000` attachments totaling `53687091200` bytes with all three buckets nonzero, index/WAL bytes at least `5368709120/2147483648`, initial HDD used ratio `3500..=7000` basis points, SSD life `>=20%`, one complete zero-error memory pass over all `34359738368` bytes, clean SSD reinstall plus authority restore `<=28800 s`, data HDD exactly CMR with rating `>=55 TB/year` and warranty through evidence expiry, scanner age `<=259200 s`, absolute W32Time offset `<=1000 ms`, sync age `<=900 s`, CPU/SSD/HDD temperatures `<=85000/70000/55000` millicelsius, SSD/HDD raw-device capacity at least `240000000000/1000000000000` bytes, and the exact watermark relation `minimum_free_bytes >= yellow_free_bytes > red_free_bytes >= emergency_reserve_bytes`, `maximum_used_bytes = total_bytes - minimum_free_bytes`, and `minimum_free_bytes >= emergency_reserve_bytes + 30*p95_daily_growth_bytes`. For each role, checked `raw_device_capacity_bytes>=volume_total_bytes>0`; the device floor applies only to raw capacity, while each `volume_total_bytes` exact-matches its filesystem-geometry row. Initial HDD ratio never uses a rounded intermediate and operates on the DATA_HDD volume total: checked `u128` requires `hdd_total_bytes>0`, `hdd_used_bytes<=hdd_total_bytes`, `hdd_used_bytes*10000 >= 3500*hdd_total_bytes`, and `hdd_used_bytes*10000 <= 7000*hdd_total_bytes`. The watermark's five-percent term is exactly `ceil_div(checked_u128(total_bytes)*500,10000)` and `emergency_reserve_bytes=max(21474836480,five_percent_term)`; quotient/remainder, multiplication, `u64` downcast and all later additions/subtractions are checked. Goldens cover both initial-ratio inclusive boundaries and one byte below/above, raw-vs-volume inversion and floor boundaries, the exact-divisible and nonzero-remainder ceiling branches, the 20-GiB crossover, and overflow/downcast failure. All 25 metric variants have executable predicates; no required metric is presence-only or informational.

All P340 evaluator arithmetic is fail-closed and independent of compiler overflow mode. Nonnegative add, subtract, multiply, sum, histogram merge, ratio cross-product, ceiling division and rank calculation promote operands to checked `u128`; subtraction first proves the left operand is not smaller. Signed timestamp/offset arithmetic uses checked `i128`. Downcast occurs only after proving destination range. Saturating, wrapping, floating-point and build-profile-dependent operations are forbidden; overflow, underflow or failed downcast returns `P340_ARITHMETIC_OVERFLOW` before PASS. Goldens exercise `u64::MAX` for retry ratios, 90-day floors, bucket/frame sums, HDD watermark/growth and `i64::MIN|MAX` time/offset values.

Every one of the 25 variants has one deterministic predicate. Principal, action and overlay variants exact-equal the frozen mixes. `ErrorRate` has exact fields `operation_count,error_count,error_basis_points,high_risk_completed_count,attachment_completed_count`; checked `u128` requires `operation_count=read_histogram.operation_count+write_histogram.operation_count+high_risk_completed_count+attachment_completed_count`, each completed count is at least its matching `ActionMix`, `operation_count>0`, `error_count<=operation_count`, and `error_basis_points=min(10000,ceil(error_count*10000/operation_count))`, followed by the zero-error release requirement. CPU P95 is the one-based nearest-rank sample at `ceil(95*N/100)` after numeric sorting. `PageFaults` starts at cumulative zero, never decreases and bounds every adjacent delta. HDD free/growth uses checked arithmetic to prove `free_bytes+used_bytes=input.hdd_total_bytes`, the basis-point floor, and `projected_daily_growth_bytes=max(input.p95_daily_growth_bytes_30d,input.p95_daily_growth_bytes_90d)`; the watermark recomputes the same growth, minimum free and maximum used. PostgreSQL requires `active<=maximum` in addition to both absolute caps; cumulative WAL is exactly zero in frame 1, never decreases, and is positive in frame 4321. Automation, SSD and page-fault counters begin at zero and never decrease. Heavy report requires `accepted=true`, the exact decision bound and concurrency one. Generated goldens cover equality and one-beyond failure for every scalar and relational predicate; no metric is informational or presence-only.

The UTF-8/no-BOM `P340WorkloadSampleV1` series contains exactly `4321` create-new records using `<8 lowercase hex byte length>\t<JCS row bytes>\n` framing. Sequence is exactly `1..=4321`, and `scheduled_at_unix_ms=context.started_at_unix_ms+(sequence-1)*60000`, making the first/last scheduled values equal the common start/finish. Observed time remains inside the interval, strictly increases, and is within `±5000 ms` of schedule. Every frame carries one nonempty byte-identical `windows_boot_id` and `boot_monotonic_source_id`; `P340WorkloadTimeseriesEvidenceV1` repeats both, while `WindowsServer2022ReadbackV1` and outer `P340SoakEvidenceV1` repeat the same boot ID. `monotonic_elapsed_ms` is the checked delta from that boot-scoped monotonic source at common start, equals zero in frame 1, and strictly increases—wall-clock subtraction or a process-local reset cannot substitute. Sequence 1 has the all-zero predecessor; later `previous_frame_sha256` hashes the entire previous frame including length, tab and LF, and `sample_chain_root_sha256` hashes the final complete frame. Every sample contains 20 distinct authenticated principals/sessions split exactly `15/3/2`, a distinct authenticated Control Center principal/session, exact `11/5/2/2` action mix, every overlay and exactly one of all 25 tagged metric variants.

`P340LatencyHistogramV1` is the only latency aggregation wire. Its exact 16 mutually exclusive buckets are `<=1`, `(1,2]`, `(2,5]`, `(5,10]`, `(10,20]`, `(20,50]`, `(50,100]`, `(100,200]`, `(200,500]`, `(500,1000]`, `(1000,1500]`, `(1500,2000]`, `(2000,3000]`, `(3000,5000]`, `(5000,10000]`, and `>10000` milliseconds. Counts sum to `operation_count`; every frame requires read `operation_count>=ActionMix.reads` and write `operation_count>=ActionMix.writes`, so both are positive under `11/5/2/2`. `maximum_ms=0` iff count is zero and otherwise agrees with the highest nonempty bucket; a valid P340 frame never takes the zero branch. The evaluator merges matching bucket counts, hard-fails if either merged `N=0`, and defines P95 as the least finite upper bound reaching `ceil(95*N/100)`; landing in `>10000` fails. Averaging stored/shard percentiles is forbidden. Automation, SSD and page-fault cumulative counters are zero in sample 1 and nondecreasing; final outer accepted/lost/duplicate/unexplained/customer-linked-write summaries exact-match final cumulative values. The evaluator recomputes count, maximum gap, chain root, barriers, both P95s, CPU nearest-rank P95, queues, overlays and reconciliation totals, rejecting every mismatch. Goldens cover exact histogram bytes, one complete framed sample, the complete frame chain, boundary predicates, zero count, count below action mix, merged-zero and malformed maximum/counter/summary negatives.

The Windows readback exact-matches both `scanner_provider_identity` and new `scanner_policy_sha256` to the host fingerprint. Provider and engine version are nonempty, policy and definition digests are nonzero, and `scanner_definition_sha256` identifies the exact authenticated definition bytes whose metadata supplies `scanner_definition_issued_at_unix_ms`. It recomputes scanner-definition age from the common trusted finish time with no rounding loophole: require `finish>=definition_issued`, checked `delta_ms=finish-definition_issued`, then `scanner_definition_age_seconds=ceil(delta_ms/1000)` before applying `<=259200`. Provider drift, policy drift/zero, empty engine, definition-digest/issued-metadata mismatch, future definition, overflow or claimed-age mismatch fails. W32Time checks `abs(i128::from(w32time_offset_milliseconds))<=1000`; `i64::MIN`, overflow, empty source or stale last-sync age fails. Rollback and fast-forward negatives remain mandatory.

The pre-start UPS policy fixes `remaining_time_trigger_seconds=900`; if `data_hdd_write_cache_enabled=true`, then `data_hdd_power_loss_flush_required=true`. UPS identity requires nonempty manufacturer/model/serial/firmware, `SIGNED_VENDOR_ADAPTER`, the artifact-set-pinned manifest, nominal adapter identity, exact configuration digest/generation, zero-drift runtime-security readback and a fresh strict initial status whose communication/self-test/output/runtime fields are known and passing; `WINDOWS_STANDARD_POWER_STATUS` is insufficient. `HddFlushVerificationV1` is also a pre-start qualification helper with `P340PreStartQualificationBindingV1`, exact-matches the same host fingerprint and input HDD serial, requires at least one qualification forced-power cut, zero acknowledged write loss and filesystem consistency PASS, and expires no earlier than the eventual soak finish. The causal order is fixed: policy attestation; strict qualification plan; all seven typed helpers through their journaled STARTED/terminal transitions; closure plus checkpoint; signed `P340CapacityInputManifestV1` carrying the plan/checkpoint/all helper refs; signed carrier staging plan carrying policy/input refs; durable `TEST_STARTED`; physical measurement. A later helper, an input that predates one of its helpers, or any pre-start policy/helper/input expiry before finish fails closed. No helper is a loose log, interval-produced substitute or override Boolean.

Every SMART snapshot set covers the same common interval and exact device identity; the evaluator derives summaries rather than trusting them. SSD life/free/rollback values equal interval minima; media/drop/retry values equal terminal nondecreasing counters; `unsafe_shutdown_count` exact-matches the terminal device counter and has zero interval increase. HDD free equals the interval minimum, every health counter equals its terminal nondecreasing value, and its flush ref exact-matches the input manifest's pre-start qualification. Across the 72-hour window, the boot ID never changes and HDD/SSD drop and retry interval deltas are zero. Temperature series exact-cover all `4321` workload sequences; the three signed maxima are recomputed, and every sample is structurally within `0..=policy maximum` millicelsius. Missing, negative, NaN/sentinel, summary-drift, snapshot-drift, interval-drift, reboot, drop or retry fails closed.

Both filesystem rows typed-load the same signed UPS policy; their `write_cache_enabled` values exact-match the policy's runtime-SSD and data-HDD cache flags. An enabled HDD cache additionally requires the power-loss-flush flag and the same-host/same-serial pre-start `HddFlushVerificationV1` PASS. Both SMART `device_health_passed` values may be true only after every underlying predicate passes; no helper Boolean can mask a failed counter, cache, identity or flush condition.

The SSD is runtime-only. `verify-residency.ps1` exact-joins the signed storage manifest and proves that every PostgreSQL data/WAL, attachment, audit, Outbox, search-index, working-file, and attributable generated-data root is on the single HDD. Its scan has two disjoint coverage sets. Set A is `RuntimeSsdReproducibleRuntimeInventoryV1`: catalog-verified clean Windows components, exact candidate launcher/MSI/native code, bounded reconstructible OS caches, immutable pre-HDD bootstrap locator bytes, and the noncustomer, nonexportable CNG certificate/key binding classified only as `TPM_BOUND_REENROLLABLE_MACHINE_KEY_METADATA`. Set B is exactly four mutable exception classes—`POWER_CONTROL_CAPSULE`, `PACKAGE_RECOVERY_CONTINUATION_CAPSULE`, `AUTHORITY_KERNEL_SLOT_POINTER_AND_JOURNAL_HEAD`, and `RECONSTRUCTIBLE_SIGNED_NATIVE_CODE_SLOT_OR_CACHE`—under their exact master roots, grammars, object/media maps, budgets, retention and rebuild authorities. No fifth rule or mutable fallback is legal. `RuntimeSsdExceptionRegistryV1` is a generated five-field policy at `C:\Program Files\EnterprisePlatform\Authority\policy\f57-runtime-ssd-exception-registry.v1.json`, compiled into launcher/kernel, MSI/readback byte-equal, and pinned by storage-manifest token `runtime-ssd-exception-registry-sha256:<digest>`.

| Exact Set-B class / root / grammar | Hard budget | Retention / rebuild authority |
|---|---:|---|
| `POWER_CONTROL_CAPSULE` / `C:\ProgramData\EnterprisePlatform\continuation-control\power-shutdown` / `POWER_ATTEMPT_V1` | total `1073741824`; instance/object `33554432`; entries `4096` | one active + eight mirrored terminal attempts; age `7776000`; `[DATA_HDD_EVIDENCE_OBJECT_STORE,OFF_HOST_CONTROL_MIRROR]` |
| `PACKAGE_RECOVERY_CONTINUATION_CAPSULE` / `C:\ProgramData\EnterprisePlatform\continuation-control\package-maintenance` / `PACKAGE_RECOVERY_OPERATION_V1` | total `4294967296`; operation `536870912`; object `268435456`; entries `8192` | one active + eight mirrored terminal operations; age `15552000`; `[DATA_HDD_PACKAGE_RECOVERY_STORE,OFF_HOST_CONTROL_MIRROR,TPM_MONOTONIC_HEAD]` |
| `AUTHORITY_KERNEL_SLOT_POINTER_AND_JOURNAL_HEAD` / `C:\ProgramData\EnterprisePlatform\authority-kernel\control` / `KERNEL_POINTER_AND_HEAD_V1` | total `67108864`; object/head `4194304`; entries `4096` | current pointer + previous pointer + current head only; `[DATA_HDD_AUTHORITY_KERNEL_POINTER_ARCHIVE,TPM_MONOTONIC_HEAD]` |
| `RECONSTRUCTIBLE_SIGNED_NATIVE_CODE_SLOT_OR_CACHE` / `C:\Program Files\EnterprisePlatform\Authority\versions` / `KERNEL_DIGEST_SLOT_V1` | total `17179869184`; entries `16384`; DLL `4294967296`, SBOM `67108864`, ABI/AuthentiCode each `16777216` | at most current + previous + one verified staging slot; `[DATA_HDD_SIGNED_PACKAGE_STORE,OFF_HOST_RELEASE_BUNDLE]` |

The scanner resolves the whole runtime volume, every product namespace and effective Windows persistence locator by final handle, enumerates ADS/hard links/offline/VSS views, and independently classifies all allocated streams. Every accepted entry maps byte-exactly to Set A or one of Set B's twenty path rows; per-object/instance/class budgets and retention recompute from entry bytes. PASS requires empty rejected set and `unregistered_persistent_entry_count=inaccessible_entry_count=partial_enumeration_count=rejected_or_unclassified_allocated_bytes=customer_authority_match_bytes=customer_canary_match_count=known_business_digest_match_count=0`, thereby deriving `runtime_ssd_customer_authority_bytes=0`. Set overlap, catalog/source mismatch, VSS/reparse/hard-link/ADS escape, inaccessible data, zero-summary without canonical entry bytes, or scan partiality fails. SSD-loss rehearsal rebuilds Set A and all four Set-B rows only from signed build, authenticated DATA_HDD/off-host copies and current TPM/head proof; no SSD byte is sole recovery authority.

Windows may not create a hidden fifth exception. The installed and post-reboot `WindowsPersistentFilePolicyReadbackV1` has exactly eight enum-sorted rows: `PAGE_FILE`, `SWAP_FILE`, `HIBERNATION_FILE`, `KERNEL_OR_FULL_CRASH_DUMP`, `MINI_DUMP`, and `WER_LOCAL_DUMP` are `DISABLED`; `VSS_DIFF_AREA` final-handle resolves only to the verified DATA_HDD; `PRODUCT_MALWARE_QUARANTINE` resolves only to the storage-manifest DATA_HDD quarantine root. Task 15 sets `PagingFiles=[]`, disables automatic managed pagefiles and `SwapfileControl`, hibernation, kernel/full/minidumps and global/per-executable WER LocalDumps, clears every dump locator, fixes VSS shadow storage to DATA_HDD and configures product quarantine before reboot, then reads registry plus actual handles/files back. A page/swap/hiber/dump/WER file on either disk, or VSS/quarantine on RUNTIME_SSD, fails rather than becoming an exception. The no-pagefile profile ships only if the 72-hour run proves 32 GiB physical RAM, commit limit `>=32212254720`, peak/every-sample committed bytes `<=25769803776`, working set `<=17179869184`, no commit-allocation failure/OOM/SCM restart and zero hard-fault counter reset.

The generated telemetry policy has exactly seven rows: `PRODUCT_OPERATIONAL_EVENT_LOG` at `67108864` and `PRODUCT_OPERATIONAL_ETL` at `67108864` are bounded RUNTIME_SSD no-customer-field schemas; `RECOVERY_TASK_OPERATIONAL_EVENT_LOG` is likewise bounded to `33554432`; `DEFENDER_OPERATIONAL_AND_HISTORY` is bounded to `268435456` with exact customer-authority roots excluded and the controlled product scanner covering them; `WINDOWS_FIREWALL_TEXT_LOG` is disabled with cap zero; `HTTP_SERVICE_ERROR_LOG` is DATA_HDD-capped `268435456`; and `AUTHORITY_HTTP_ACCESS_AUDIT` is DATA_HDD under storage-manifest retention. Every SSD row has a nonnull content-addressed allowed-field schema, while disabled/DATA_HDD rows have null. Post-reboot readback exact-checks all channel/session/registry settings, caps and final locations, HTTP.sys restart application, exact Defender exclusions, absent firewall text logs, zero unregistered channel/session and zero canary/business-digest hits. This minimizes Windows telemetry and does not create another Set-B row.

Clean-SSD recovery is explicitly not ordinary reboot. Destroying the SSD destroys the old CNG provider/container binding; DER/SPKI/TPM handle/public metadata cannot recreate it. Ordinal 06 keeps admission closed and, under a two-person off-host 48-digit recovery-password ceremony, executes exactly the hash-chained eight-step sequence `RECOVERY_PASSWORD_UNLOCK_VERIFIED -> NEW_TPM_KEY_ATTESTED -> NEW_CERTIFICATE_CHAIN_VERIFIED -> NEW_PUBLIC_KEY_PROTECTOR_ADDED_AND_TESTED -> NEW_AUTHORITY_AND_NV_COMMITTED -> NORMAL_REBOOT_UNLOCK_VERIFIED -> OLD_PUBLIC_KEY_PROTECTOR_REMOVED -> REENROLLMENT_CLOSURE_COMMITTED`. All rows share one operation ID, sequences `1..8`, strictly increasing trusted times and predecessor hashes. The enclosing `SsdDataHddRecoveryAndReenrollmentReadbackV1` derives its IDs/epoch/Booleans from that chain, has both recovery-secret persistence counters zero, exact-matches the new bootstrap and normal-reboot broker unlock refs, proves a strictly higher authority epoch and TPM policy-protected NV advance, and requires `admission_opened_before_reenrollment_completed=false`. Only after this closure and every current certification gate may the separate admission authority reopen. `verify-filesystem-geometry.ps1` separately proves the same two physical identities are GPT/NTFS with valid geometry and UPS-bound write-cache policy. Adding a future HDD requires a new hardware/storage profile and new certification run.

Before STARTED, the signed `P340CapacityInputManifestV1` exact-binds PostgreSQL 16/build, graph-derived complete table and queue sets, attachment/index/WAL volumes, HDD fill, host fingerprint input/digest, and all seven pre-start helper refs including `hdd_flush_verification_ref`. It embeds—without another signer—the closed `capacity_history_kind=P340CapacityHistoryKindV1::QualificationSynthetic` wire `QUALIFICATION_SYNTHETIC` and exactly 90 canonical ascending, consecutive, complete UTC `P340CapacityHistoryDayV1` rows ending on the UTC date immediately before the date containing `TEST_STARTED`. Each day carries the same sorted unique complete graph-derived table set as `P340CapacityHistoryTableInsertV1 { table_id,insert_count }` plus `hdd_growth_bytes`; the final 30 rows are the unique 30-day suffix. Checked one-based nearest-rank P95 over numeric ascending values at `ceil(95*N/100)` is independently recomputed for every table and HDD growth over both the 30-row suffix and all 90 rows, and exact-matches each table's `p95_daily_insert_count_30d|90d` plus the manifest's `p95_daily_growth_bytes_30d|90d`; all recomputed values are positive. Each table requires `row_count>=max(10000,90*p95_daily_insert_count_90d)`. Missing/duplicate/nonconsecutive/non-UTC/partial-day history, local-time alias, a wrong table set/suffix/rank, zero value, caller-supplied summary, or any observed-site/customer-data classification fails closed.

`host_fingerprint_sha256=sha256(JCS(host_fingerprint_input))` covers SMBIOS UUID, `cpu_wmi_identity`, typed `cpu_sku`, canonical RAM modules, tagged SSD/HDD model/serial/firmware/raw-device-capacity/volume-total/controller/volume identities, HDD CMR/rating/warranty, controller driver, Windows build/UBR, scanner/provider/policy, UPS adapter, software/configuration generations and configuration digest. The frozen canonicalizer removes only enumerated trademark tokens and clock suffix and normalizes ASCII case/space: real WMI golden `Intel(R) Core(TM) i5-10500 CPU @ 3.10GHz` maps only to wire `INTEL_CORE_I5_10500`; i5-10400, 10500T, 10600 and ambiguous identities fail. Outer `cpu_sku` exact-matches that typed value—there is no free-form `cpu_model`. RAM identities are nonempty/unique and checked-sum to `34359738368`. Storage roles are exactly `{RUNTIME_SSD,DATA_HDD}`; each raw capacity is at least its volume total, each volume total exact-matches filesystem geometry, and only raw capacity is compared with device floors. `input.hdd_total_bytes` equals DATA_HDD `volume_total_bytes`, every watermark total, and every frame's checked `free+used`; sample 1 exact-repeats input HDD used/free state. `initial_queues` is only the sorted unique exact graph-derived `Vec<P340InitialQueueIdentityV1 { queue_id }>`; it has no depth or oldest-age scalar, because the 25 runtime metric predicates own queue measurements. Empty/small/stale/unknown input, any hardware/configuration identity drift, neighboring CPU SKU, raw/volume inversion, extra/missing queue ID, dead queue scalar, or any crossed capacity/CPU/RAM/role value fails before load and requires a new certification attempt.

The workload interval is one uninterrupted exact 259,200-second window: checked signed arithmetic requires `context.finished_at_unix_ms-context.started_at_unix_ms=259200000`. Cross-signature causality is exactly `TEST_STARTED.recorded_at_unix_ms <= context.started_at_unix_ms <= context.finished_at_unix_ms <= CarrierStagingCompletionV1.completed_at_unix_ms <= ReleaseCarrierResultV1.issued_at_unix_ms <= terminal_journal_record.recorded_at_unix_ms`. It holds 20 business users (`15+3+2`) plus one independently reserved Control Center lane, action mix `11+5+2+2`, and every overlay active across all 4,321 scheduled samples. A timestamp inversion/overflow, pause/reboot, sampler gap beyond 120 seconds, timing/hash/counter discontinuity, storage drop/retry, reduced or duplicate principal concurrency, missing metric/overlay, 71-hour result, stitched measurements, cross-host readback, or `high_availability_certified=true` is terminal failure. Engineering smoke exercises the same evaluator with an explicit rehearsal context but cannot serialize or sign any candidate-bound P340 envelope.

`p340_capacity.rs` implements exactly `NFR-001`, `NFR-003`, `NFR-005`, and `NFR-007`. The handlers require a final-candidate digest and return `NOT_COVERED`, never PASS, when invoked with engineering-smoke evidence.

- [ ] **Step 4: Generate bindings, prove Rust GREEN, and run bounded engineering carrier smoke.**

Run: `cargo xtask f57 graph generate`

Run: `cargo test -p ep-testkit --test f57_p340_evaluator --test f57_p340_schema --test f57_power_shutdown --test f57_ups_adapter_contract --test f57_ups_command_reconciliation -- --nocapture`

Run: `cargo test -p ep-platform-ups-contract -p ep-adapter-ups-windows --all-targets --locked`

Run: `cargo check -p ep-testkit --test f57_p340_capacity`

Run: `cargo xtask f57 graph generate --check`

Run: `cargo test -p ep-platform-runtime -p ep-adapter-file -p ep-xtask -p ep-testkit --all-targets --locked`

Expected: the synthetic P340 evaluator and UPS runtime-loss/command state-machine tests PASS, the generated canonical facade and exact current handler binding compile, and an attempted `IAAS_WINDOWS_SERVER_HDD_STRICT` or provider-power recipe is rejected as absent from this graph version before STARTED; no G6 Requirement is reported PASS without final-candidate real P340/UPS carrier evidence.

Run through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_RUN_P340_CERTIFICATION_V1 -- -Mode EngineeringSmoke`.

Expected: the exact 15/3/2+1 session mix and 11/5/2/2 action mix start, measure, and stop cleanly during a bounded smoke; no 72-hour or release claim is emitted.

Run through the fixed-host/final-handle trust executor: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_TEST_POWER_SHUTDOWN_V1 -- -Mode EngineeringSmoke`.

Expected: engineering UPS/readback path PASS with no Win32 shutdown call, UPS schedule, signed-candidate raw or service activation. The script's closed mode enum is exactly `EngineeringSmoke|PrepareOnly`; missing/unknown mode fails before every side effect. Task 14's fixed recipe is the sole `PrepareOnly` caller, and only its authenticated broker may dispatch shutdown.

- [ ] **Step 5: Commit hardware certification code.**

```bash
cargo xtask f57 task stage --task G6-13
cargo xtask f57 task verify-staged --task G6-13
git commit -m "test: add p340 and power certification harnesses"
```

### Task 14: Commit all final-candidate and release-gate tooling

**Files:**
- Create: `xtask/src/f57/final_candidate.rs`
- Create: `xtask/src/f57/l3.rs`
- Create: `xtask/src/f57/release_gate.rs`
- Create: `xtask/src/f57/production_activation.rs`
- Create: `xtask/src/f57/runtime_topology_certification.rs`
- Create: `xtask/src/f57/windows_runtime_deployment.rs`
- Create: `xtask/src/f57/final_installed_generation.rs`
- Create: `xtask/src/f57/power_shutdown.rs`
- Create: `xtask/src/f57/release_subordinate_readback.rs`
- Create: `crates/platform/release/src/release_evidence.rs`
- Read/import: `crates/platform/release/src/l2.rs`
- Create: `crates/platform/release/src/power_shutdown.rs`
- Create: `crates/platform/release/src/runtime_topology_certification.rs`
- Create: `crates/platform/release/src/offline_schema.rs`
- Create: `crates/platform/release/src/production_activation.rs`
- Create: `crates/platform/release/tests/release_evidence.rs`
- Create: `crates/platform/release/tests/power_shutdown.rs`
- Create: `crates/platform/release/tests/runtime_topology_certification.rs`
- Create: `crates/platform/release/tests/production_activation.rs`
- Create: `crates/platform/generation-activation/src/final_installed.rs`
- Create: `crates/platform/generation-activation/src/package_maintenance.rs`
- Create: `crates/platform/generation-activation/src/production_admission.rs`
- Create: `crates/platform/generation-activation/src/resilience_admission.rs`
- Create: `crates/platform/generation-activation/src/admission_store.rs`
- Create: `crates/platform/generation-activation/src/admission_gate.rs`
- Create: `crates/platform/generation-activation/tests/final_installed.rs`
- Create: `crates/platform/generation-activation/tests/package_maintenance.rs`
- Create: `crates/platform/generation-activation/tests/production_admission.rs`
- Create: `crates/platform/generation-activation/tests/production_admission_execution_lease.rs`
- Create: `crates/platform/generation-activation/tests/production_admission_bypass_registry.rs`
- Create: `crates/platform/generation-activation/tests/production_admission_races.rs`
- Create: `crates/platform/generation-activation/tests/resilience_admission.rs`
- Modify: `crates/platform/generation-activation/src/lib.rs`
- Modify: `crates/platform/generation-activation/Cargo.toml`
- Modify: `crates/platform/package/src/maintenance.rs`
- Modify: `crates/platform/package/src/participant.rs`
- Modify: `crates/platform/package/src/lifecycle.rs`
- Modify: `crates/platform/package/Cargo.toml`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Create: `crates/adapter/file/src/power_shutdown_continuation_store.rs`
- Create: `crates/adapter/file/src/offline_schema_manifest_store.rs`
- Create: `crates/adapter/file/tests/power_shutdown_continuation_store.rs`
- Create: `crates/adapter/file/tests/offline_schema_manifest_store.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Create: `testkit/tests/f57_final_candidate.rs`
- Create: `testkit/tests/f57_release_gate_unit.rs`
- Create: `testkit/tests/f57_release_dependency_dag.rs`
- Create: `testkit/tests/f57_final_installed_generation.rs`
- Create: `testkit/tests/f57_package_maintenance_production.rs`
- Create: `testkit/tests/f57_production_activation.rs`
- Create: `testkit/tests/f57_production_generation_admission.rs`
- Create: `testkit/tests/f57_production_admission_execution_lease.rs`
- Create: `testkit/tests/f57_production_admission_bypass.rs`
- Create: `testkit/tests/f57_production_admission_races.rs`
- Create: `testkit/tests/f57_resilience_admission.rs`
- Create: `testkit/tests/f57_postgres_log_retention_control.rs`
- Create: `crates/adapter/db-pg/src/platform_core/production_activation_store.rs`
- Create: `crates/adapter/db-pg/src/platform_core/production_activation_admission_commit_store.rs`
- Create: `crates/adapter/db-pg/src/platform_core/production_generation_admission_store.rs`
- Create: `crates/adapter/db-pg/src/platform_core/production_admission_hold_store.rs`
- Create: `crates/adapter/db-pg/src/platform_core/production_admission_execution_lease_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Create: `db/migrations/platform_core/V20261025092530__platform_core_create_production_activation_and_admission.sql`
- Modify: `xtask/src/f57/l2.rs`
- Modify: `xtask/src/f57/cli.rs`
- Modify: `xtask/src/f57/carrier.rs`
- Read/import: `crates/platform/gate-journal-contract/src/journal.rs`
- Read/import: `crates/platform/gate-journal-contract/src/port.rs`
- Modify: `xtask/src/f57/mod.rs`
- Create: `apps/core-server/src/platform/windows_service/dispatcher.rs`
- Create: `apps/core-server/src/platform/windows_service/mod.rs`
- Create: `apps/core-server/src/kernel/abi.rs`
- Create/regenerate from the one ABI generator: `apps/core-server/src/kernel/generated_abi.rs`
- Create: `apps/core-server/src/kernel/loader.rs`
- Create: `apps/core-server/src/kernel/slot_pointer.rs`
- Create: `apps/core-server/src/kernel/mod.rs`
- Modify: `crates/platform/command/src/pipeline.rs`
- Create: `crates/platform/command/src/admission.rs`
- Create: `crates/platform/command/tests/admission_boundary.rs`
- Create: `crates/platform/authority-kernel/Cargo.toml`
- Create: `crates/platform/authority-kernel/src/lib.rs`
- Create: `crates/platform/authority-kernel/src/abi.rs`
- Create/regenerate from the one ABI generator: `crates/platform/authority-kernel/src/generated_abi.rs`
- Create: `crates/platform/authority-kernel/src/dispatch.rs`
- Create: `crates/platform/authority-kernel/src/application/admin/final_installed_generation.rs`
- Create: `crates/platform/authority-kernel/src/application/admin/package_maintenance.rs`
- Create: `crates/platform/authority-kernel/src/application/admin/production_activation.rs`
- Create: `crates/platform/authority-kernel/src/application/admin/postgres_log_retention.rs`
- Create: `crates/platform/authority-kernel/src/application/admin/mod.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/final_installed_generation.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/package_maintenance.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/production_activation.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/production_admission.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/resilience_admission.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/postgres_log_retention.rs`
- Create: `crates/platform/authority-kernel/src/application/wiring/mod.rs`
- Create: `crates/platform/authority-kernel/src/application/http/router.rs`
- Create: `crates/platform/authority-kernel/src/application/http/mod.rs`
- Create: `crates/platform/authority-kernel/src/application/mod.rs`
- Create: `crates/platform/authority-kernel/tests/abi_compatibility.rs`
- Create: `crates/platform/authority-kernel/tests/abi_export_and_layout.rs`
- Create: `crates/platform/authority-kernel/tests/windows_service_dynamic_readback.rs`
- Create: `crates/platform/authority-kernel/tests/power_shutdown_continuation_composition.rs`
- Create: `crates/platform/authority-kernel/tests/package_maintenance_composition.rs`
- Create: `crates/platform/authority-kernel/tests/final_installed_generation_composition.rs`
- Create: `crates/platform/authority-kernel/tests/production_activation_composition.rs`
- Create: `crates/platform/authority-kernel/tests/production_admission_gate_composition.rs`
- Modify: `apps/core-server/src/platform/mod.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/core-server/tests/windows_service_process_dispatch.rs`
- Create: `apps/core-server/tests/authority_kernel_loader_composition.rs`
- Create: `apps/core-server/tests/authority_kernel_abi_binding.rs`
- Create: `tools/authority-kernel-abi-gen/Cargo.toml`
- Create: `tools/authority-kernel-abi-gen/src/main.rs`
- Create/regenerate: `include/ep_authority_kernel_api_v1.h`
- Create/regenerate: `crates/platform/authority-kernel/ep-authority-kernel.def`
- Create: `apps/recovery-tool/src/package_maintenance.rs`
- Create: `apps/recovery-tool/src/package_recovery_capsule.rs`
- Create: `apps/recovery-tool/src/kernel_slot.rs`
- Create: `apps/recovery-tool/tests/package_maintenance_recovery.rs`
- Modify: `apps/recovery-tool/src/main.rs`
- Modify: `apps/recovery-tool/Cargo.toml`
- Modify: `tools/release-gate/src/main.rs`
- Modify: `tools/release-gate/Cargo.toml`
- Create: `docs/evidence/f57-release-evidence.schema.json`
- Read/import: `docs/evidence/f57-windows-server-component-set.v1.schema.json`
- Create: `docs/evidence/f57-production-activation.v1.schema.json`
- Create: `docs/evidence/f57-production-admission.v1.schema.json`
- Create: `docs/evidence/f57-production-admission-bypass-registry.v1.schema.json`
- Create: `docs/evidence/f57-package-recovery-control.v1.schema.json`
- Create: `docs/evidence/f57-offline-schema-manifest.v1.schema.json`
- Read: `docs/evidence/f57-l2-candidate-evidence.schema.json`
- Read: `docs/evidence/f57-foundation.v1.schema.json`
- Read: `docs/evidence/f57-generation.v1.schema.json`
- Read/import: `docs/evidence/f57-generation-observed-release-selection.v1.schema.json`
- Read/import: `docs/evidence/f57-backup-checkpoint.v1.schema.json`
- Read/import: `docs/evidence/f57-authority-recovery-cut-manifest.v1.schema.json`
- Read/import: `docs/evidence/f57-backup-storage-safeguard.v1.schema.json`
- Read/import: `docs/evidence/f57-postgres16-windows-install.v1.schema.json`
- Read/import: `docs/evidence/f57-capability-package.v1.schema.json`
- Read/import: `docs/schemas/f57-capability-package-trust-registry.v1.schema.json`
- Read/import: `docs/evidence/f57-recovery-domain-manifest.schema.json`
- Read: `docs/schemas/f57-generation-approval-registry.v1.schema.json`
- Read: `docs/evidence/f57-release-carrier-common.v1.schema.json`
- Read: `docs/evidence/f57-windows-authority-manifest.v1.schema.json`
- Read: `docs/evidence/f57-p340-soak-evidence.schema.json`
- Read/import: `docs/evidence/f57-ups-contract.v1.schema.json`
- Read/import: `docs/evidence/f57-recovery-certification-policy.v1.schema.json`
- Read: `docs/evidence/f57-client-stack-decision-archive.v1.schema.json`
- Read: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Read/import: `docs/evidence/f57-windows-runtime-deployment.v1.schema.json`
- Read: `crates/platform/runtime/src/topology.rs`
- Read: `crates/platform/release/src/generation.rs`
- Read: `crates/platform/release/src/generation_approval.rs`
- Read: `crates/platform/release/src/participant.rs`
- Read/import: `crates/platform/release/src/activation_attempt.rs`
- Read/import: `crates/platform/release/src/carrier_contract.rs`
- Read/import: `crates/platform/backup/src/windows_components.rs`
- Read/import: `crates/platform/backup/src/windows_components/unlock.rs`
- Read/import: `docs/evidence/f57-data-hdd-bitlocker-unlock.v1.schema.json`
- Read/import: `docs/evidence/f57-evidence-signer-broker-windows-install-readback.v1.schema.json`
- Read/import: `crates/platform/backup/src/checkpoint.rs`
- Read/import: `crates/platform/backup/src/ports.rs`
- Read/import: `crates/platform/backup/src/safeguard.rs`
- Read/import: `crates/platform/backup/src/postgres16_windows.rs`
- Read/import: `crates/platform/runtime/src/capacity/p340.rs`
- Read/import: `crates/platform/ups-contract/src/lib.rs`
- Read/import: `crates/platform/ups-contract/src/model.rs`
- Read/import: `crates/platform/ups-contract/src/ports.rs`
- Read/import: `crates/adapter/ups-windows/src/lib.rs`
- Read/import: `crates/adapter/ups-windows/src/standard_power_status.rs`
- Read/import: `crates/adapter/ups-windows/src/signed_vendor.rs`
- Read/import: `docs/deployment/f57-windows-backup-components.v1.json`
- Read/import: `crates/platform/capability-graph/src/compiler.rs`
- Read/import: `crates/platform/capability-graph/src/canonical.rs`
- Read: `crates/platform/runtime/tests/fixtures/runtime-topology-certification-v1-golden.json`
- Read: `docs/evidence/f57-gate-run-journal.v1.schema.json`
- Read: `docs/evidence/f57-gate-receipt.v1.schema.json`
- Read: `docs/f57-artifact-signer-registry.v1.json`
- Read: `docs/schemas/f57-artifact-signer-registry.v1.schema.json`
- Read generated: `testkit/tests/f57_full_release_evidence.rs`
- Read generated: `testkit/tests/f57_storage_key_boundary.rs`
- Read generated: `testkit/tests/f57_transactional_evidence.rs`
- Read generated: `testkit/tests/f57_windows_recovery_security.rs`
- Read generated: `testkit/tests/f57_control_center_contract.rs`
- Create: `testkit/src/f57_cases/g6/control_center_contract.rs`
- Create: `testkit/src/f57_cases/g6/full_release_evidence.rs`
- Create: `testkit/src/f57_cases/g6/storage_key_boundary.rs`
- Create: `testkit/src/f57_cases/g6/transactional_evidence.rs`
- Create: `testkit/src/f57_cases/g6/windows_recovery_security.rs`
- Create: `xtask/tests/f57_release_carrier.rs`
- Create: `xtask/tests/f57_windows_runtime_deployment.rs`
- Modify: `xtask/tests/f57_run_journal.rs`
- Create: `xtask/tests/fixtures/f57-release-carrier-raw-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-release-carrier-compiled-registry-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-carrier-staging-plan-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-carrier-staging-completion-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-carrier-staging-crash-cuts-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-control-capsule-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-security-descriptors-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-dispatch-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-resume-controller-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-success-spool-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-power-shutdown-failure-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-release-subordinate-readback-registry-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-gate-run-journal-v1-golden.jcs.jsonl`
- Read: `xtask/tests/fixtures/f57-gate-run-journal-checkpoint-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-release-carrier-result-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-authority-artifact-set-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-windows-runtime-deployment-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-server-component-set-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-windows-server-component-install-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-evidence-signer-broker-windows-install-readback-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-data-hdd-bitlocker-unlock-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-offline-schema-closure-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-offline-schema-manifest-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-client-stack-decision-trust-closure-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-client-stack-decision-archive-manifest-v1-golden.json`
- Read: `xtask/tests/fixtures/f57-gate-receipt-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-release-candidate-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-l2-final-release-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-l3-release-certification-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-release-certificate-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-activation-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-generation-admission-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-admission-hold-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-admission-execution-lease-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-admission-bypass-registry-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-production-admission-race-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-package-recovery-control-v1-goldens.json`
- Create: `xtask/tests/fixtures/f57-authority-kernel-abi-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-authority-kernel-export-table-v1-golden.json`
- Create: `xtask/tests/fixtures/f57-authority-kernel-msi-projection-v1-golden.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`
- Modify: `scripts/windows/build-msi.ps1`
- Modify: `scripts/windows/install-services.ps1`
- Modify: `installer/windows/Product.wxs`
- Modify: `installer/windows/Services.wxs`
- Create/regenerate: `installer/windows/generated/AuthorityKernelBootstrap.wxi`
- Modify: `scripts/windows/run-l2-candidate.ps1`
- Create: `scripts/windows/run-l3-release.ps1`
- Create: `scripts/windows/trust/F57_PS_RUN_L3_RELEASE_V1.authenticode.json`
- Modify/regenerate after final edit: `scripts/windows/trust/F57_PS_RUN_L2_CANDIDATE_V1.authenticode.json`
- Modify/regenerate after final edit: `scripts/windows/trust/F57_PS_BUILD_MSI_V1.authenticode.json`
- Modify/regenerate after final edit: `scripts/windows/trust/F57_PS_INSTALL_SERVICES_V1.authenticode.json`
- Modify/regenerate final exact 18-row closure: `crates/platform/powershell-trust/src/generated_registry.rs`
- Modify: `crates/platform/powershell-trust/src/lib.rs`
- Modify/regenerate final exact 18-row closure: `docs/generated/f57/powershell-script-registry.v1.json`
- Modify/regenerate independent exact 18-row fixture: `crates/platform/powershell-trust/tests/fixtures/f57-powershell-script-registry-v1-golden.json`
- Modify: `.github/workflows/ci.yml`
- Modify: `xtask/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`

**Interfaces:**
- Consumes: committed carrier/hardware/recovery harnesses from Tasks 10–13 and the exact migration/Requirement registries.
- Produces: the complete source code for `ReleaseCandidateV1`, signed `L2CandidateEvidenceV1`, signed `L3CandidateEvidenceV1`, `F57OfflineSchemaManifestV1`, `WindowsAuthorityArtifactSetV1`, import/materialization of the Task-11-owned six-row `WindowsServerComponentSetV1`, runtime deployment closure/readback, topology certification, final-installed OBSERVED authority, release-owned post-certificate activation-ready proof, upper-owned atomic activation+genesis admission commit, `ProductionGenerationAdmissionV1|ProductionAdmissionHoldV1|ProductionAdmissionExecutionLeaseV1`, the generated exact ten-row bypass registry and unique business-route gate, `RELEASE_CERTIFIED`, all 36 G6 handlers, migration `92530`, and exactly the six G0-owned carrier mappings. Release owns candidate/carrier/L3/certificate and release-activation verification through `LIVE_READBACK_BOUND`; `ep-platform-generation-activation` solely owns package maintenance orchestration, the joint activation/genesis commit, delta/rollback-reopen admission, hold/lease state and private verified admission wrappers. `production_activation.rs` has no terminal commit and never independently opens a route. Runtime owns live collector ports/readback types; backup owns checkpoint/full cut and all six Windows component rows; G0 evidence trust owns the fixed signer-broker row/readback; flow owns recovery policy/selection; release and upper import rather than copy them. Gate-journal-contract owns journal variants/prefixes and client-common owns client IDs. `xtask`/release-gate are composition only. Task 14 implements authorities and stores but emits no live certificate/admission; Task 15 alone executes them against the final installation.

This task is the sole point that adds final direct workspace dependencies `ep-platform-release -> ep-platform-runtime|ep-platform-client-common|ep-platform-delivery-registry|ep-platform-backup|ep-platform-ups-contract` and `ep-platform-generation-activation -> ep-platform-package|ep-platform-backup|ep-platform-tenancy`, updates `Cargo.lock`, and freezes their locked-metadata golden. Release's direct UPS-contract edge is mandatory because release owns the POWER carrier and names nominal command/ACK types; a transitive dependency through runtime is not accepted. The first group supports topology/candidate/component/UPS verification; the second lets the upper coordinator typed-join package, Task-11 checkpoint/full-cut and current tenancy scope. `ep-platform-package` never depends on backup, tenancy or generation activation; backup/package/release/runtime/tenancy never import the upper coordinator. No inverse edge or cycle is legal. `ep-authority-kernel` directly composes `ep-platform-ups-contract` with `ep-adapter-ups-windows` plus the production adapters, concrete stores, sealed live collectors, security resolver, executors, HTTP router and command pipeline into application authorities. `core-server` links only the runtime loader/SCM dispatcher and never links db-pg, package, backup, tenancy, release business authorities, UPS provider code, HTTP business handlers or their composition modules. A locked-metadata/import golden proves that fixed-PE dependency boundary.

`package_maintenance.rs` adds private `VerifiedGenerationPackageItemSlotV1` plus the sole `PackageMaintenanceGenerationAuthorityV1`; it consumes, and never redeclares, release-owned apply/rollback requests. Under the serialized generation lease, `reserve` derives the static slot, current tagged scope from a strict tenancy snapshot, next generation, participant/item, action, class, executor and rollback strategy from compiled graph plus current OBSERVED installed-state readback. It allocates positive reservation ordinal and unpredictable IDs internally. The package plan authority receives that reservation, typed source/target package and implementation manifests, sealed source readback and two current private security contexts. Its Task-7 journal creates the two ordinal-1 decisions and signs/stores a historical 30-field structural plan containing only `recovery_checkpoint_policy_ref`; it binds the plan to the reservation. The separately derived nine-field desired item remains pure and reusable. No actual checkpoint, hold, execution authorization or attempt ref exists before generation signing.

Activation is the one crash-recoverable two-phase sequence. The upper coordinator first creates a `ProductionAdmissionHoldV1{cause=PACKAGE_MAINTENANCE{reservation_ref}}` prefix, exact-loads that reservation, closes exactly the impacted routes through the unique gate, drains every accepted request and commits `write_barrier_id`. It then sends Task 11 a private neutral checkpoint request carrying the reservation/policy/barrier/current topology/current storage manifest/explicit prior checkpoint head; backup creates and verifies one full `AuthorityRecoveryCutManifestV1`, collects a new cut-bound `BACKUP_CHECKPOINT_PREPARATION` safeguard through the narrow port, and produces the next sequence signed checkpoint for every enabled HDD authority class. Only after typed reload of the checkpoint, its prior edge, fresh safeguard and full cut does the upper authority take a fresh source-state readback, create fresh ordinal-N dual-control decisions when necessary, freeze the full participant-specific `CapabilityPackageExecutionTrustSnapshotV1`, issue `CapabilityPackageMaintenanceExecutionAuthorizationV1`, create/store the per-attempt transition and commit the first exact operation request. The common trust snapshot also applies to atomic/drain in their first-intent CAS; `VERIFY_UNCHANGED` uses direct readback and no snapshot/operation. A stale/cross-cause/cross-reservation/cross-participant/cross-scope/cross-cut/cross-topology/prior-head binding fails before effect.

`ResilienceAdmissionAuthorityV1` reuses the same deployment lock, `ProductionAdmissionHoldV1` store, route gate, accepted execution-lease drain and barrier CAS for exactly three non-package tagged causes: `CURRENT_ROOTS_ROTATION{target_backup_topology_ref,target_authority_storage_manifest_ref,target_topology_signing_trust_current_ref}`, `DATA_HDD_DISASTER_REPLACEMENT{recovery_attempt_ref}` and `UPS_RUNTIME_LINK_LOSS{runtime_loss_episode_ref}`. Current-roots rotation creates the deployment-wide hold from the already verified successor tuple, reaches zero accepted leases and persists the barrier before calling the Task-11 active-config CAS; the CAS target must byte-equal the frozen cause, and its immutable hold ID/cause/barrier are repeated into bridge preparation and remain current throughout `TRANSITIONING|BOOTSTRAPPING`. Its only graph is `HOLD_INTENT -> ADMISSION_CLOSED -> DRAIN_COMPLETE -> BARRIER_COMMITTED -> FORWARD_INTENT_COMMITTED -> CHECKPOINT_BOUND -> REOPENED -> TERMINAL`; the reopen CAS exact-loads a new fresh `HEALTHY/None` safeguard whose current head binds the cause tuple, current epoch and current OBSERVED generation. Disaster replacement never invokes the normal-rotation constructor or asks the dead old volume for a readback; it consumes only Task-11's private verified off-host recovery input and the immutable two-custodian recovery attempt named by the cause. Its graph is the same closed prefix, but `DRAIN_COMPLETE` may use only zero accepted leases or that attempt's typed unreachable-and-fenced-old-authority proof, and its reopen CAS additionally requires the higher epoch/storage generation, new-volume manifest, complete reconciliation, fresh backup bootstrap and current first-release P340 recertification. UPS loss first creates one immutable episode at the equality-inclusive status expiry and embeds its ref in the hold; its recovery-before-deadline graph is `HOLD_INTENT -> ADMISSION_CLOSED -> REOPENED -> TERMINAL`, while timeout uses `HOLD_INTENT -> ADMISSION_CLOSED -> DRAIN_COMPLETE -> BARRIER_COMMITTED -> CHECKPOINT_BOUND -> FORWARD_INTENT_COMMITTED -> REOPENED -> TERMINAL` and requires manual fresh-evidence recovery after local shutdown. Cause, scope and predecessor repeat byte-identically on every row; cross-cause refs/states, a generic reason string, null cause ref, timer reset, a resilience `TERMINAL` without its cause-specific `REOPENED`, or any `REOPENED|TERMINAL` written outside the matching upper admission CAS cannot open routes. Package-maintenance success remains the disjoint joint `FORWARD_INTENT_COMMITTED -> TERMINAL + PACKAGE_DELTA|ROLLBACK_REOPEN admission` path. Crash/race tests prove hold-before-CAS, barrier-after-drain or exact disaster fence, no route in transition, no old-volume HEALTHY dependency, no timer reset and one final cause-bound reopen CAS.

The existing `EPAuthorityControl` service also gains the disjoint typed protocol tag `POSTGRES_LOG_RETENTION`; this is an operation inside the existing broker and 18-object security model, not a new service, pipe, shell or generic file-delete API. `POSTGRES_LOG_RETENTION_CLEANUP` accepts only the Task-11 private verified policy/preview/legal-hold/free-space/dual-control value and derives every final handle from that value. The broker rechecks current-log identity, seven-day floor, hold set and pre-delete set digest, deletes exactly the eligible closed-log vector, reopens results, persists the audit receipt and returns it. The PostgreSQL SID and every non-control identity have negative AccessCheck coverage for DELETE/WRITE_DAC/WRITE_OWNER/hold-policy mutation. Preview drift, current/young/held selection, untyped path, caller glob, partial deletion, low-space threshold downgrade or audit failure leaves the global hold/fail-close state in effect.

The executor/strategy table selects only the master's closed operation pair: elevated ordinary classes use `EPAuthorityControl` package artifact switch/restore; `RUST_KERNEL` uses the recovery Scheduled Task runtime-slot switch/restore; database/crypto/storage use the recovery task foundation-apply/checkpoint-restore pair. `EPAuthorityControl` retains one fixed authenticated pipe and the exact 18-object SDDL model, but its control root has disjoint typed `POWER|PACKAGE|RESILIENCE|POSTGRES_LOG_RETENTION` protocols/capabilities; no tag can issue another tag's operations. The recovery task accepts only the Task-11 fixed request/result/query protocol, component-row identity and durable operation ID—never shell, SQL, caller path, task/service name or argv. Registry rotation/expiry after first intent permits only query/adopt, measurement and upper-authorized reverse; it never authorizes a new forward call.

A failed/UNKNOWN participant result cannot invoke reverse locally. The upper coordinator first durably commits `ROLLBACK_STARTED{rollback_execution_attempt_id}`, derives the exact reverse subset, then dispatches the private rollback request. Restoring a real predecessor returns tagged `DESIRED_ITEM`; a newly installed item returns `DEACTIVATED_RETAIN_DATA` with an `ABSENT`/retained-data readback; generation-1 rollback uses `NO_OBSERVED_GENERATION`. Before a foundation checkpoint restore, recovery freezes a signed `PackageRecoveryContinuationCapsuleV1` and content-addressed terminal reseal payload outside the restore set on the closed runtime-SSD control path, TPM/off-host monotonic-head protected and mirrored off host. After restore it verifies the same operation/result/readback, then idempotently reseals expected-restored SQL package/generation projections to terminal state before retiring the capsule. No forward replay or reconstruction from hashes is legal.

Every participant item uses independent `(activation_attempt_id,participant_id,item_id)` CAS/trust/operation/readback identity. Only complete typed apply readbacks can yield ACKs and only the full ACK set can yield generation OBSERVED. Package-local `APPLIED_VERIFIED` never opens business traffic. The deployment-wide admission authority atomically compares current OBSERVED with the sole admitted generation: ordinary deltas inside the existing certified resource envelope may create `PACKAGE_DELTA`; any global class or resource-envelope increase remains held pending full recertification; rollback may create `ROLLBACK_REOPEN` only from a fresh exact predecessor readback. Crash tests cut every reservation/finalization/hold/drain/barrier/checkpoint/authorization/transition/operation/readback/ACK/OBSERVED/admission/capsule/reseal boundary and prove that UNKNOWN remains closed.

Task 14 makes `ep-core-server.exe` a fixed, signed recovery-domain launcher/service host and adds `ep-authority-kernel.dll` as a versioned `cdylib`; package updates never overwrite the running PE or change an SCM row. All five rows have `installed_executable_path=C:\Program Files\EnterprisePlatform\Authority\ep-core-server.exe`. Each SCM `ImagePath/BINARY_PATH_NAME` is instead the canonical raw command line: the Windows-quoted exact executable path followed by that role's exact `ServiceInstall.Arguments` token vector, which must round-trip through the frozen Windows argv parser. The bare executable path is never asserted equal to raw `ImagePath`. Maintenance changes neither executable path nor argv and never calls `ChangeServiceConfig`.

The DLL has exactly one named, non-forwarded export, `ep_authority_kernel_get_api_v1`, with exact signature `unsafe extern "system" fn(requested_abi_version: u32,out_api: *mut *const EpAuthorityKernelApiV1)->EpStatusV1`. The V1 `#[repr(C)]` table is exactly 48 bytes on the certified x64 target with field offsets `[0,4,8,16,24,32,40]` for `{abi_version,struct_size,initialize,run_service_main,dispatch_typed_control,shutdown,copy_last_error}` and five nonnull callback pointers. Every function is `extern "system"`; caller-owned fixed C-layout slices/handles/status codes cross the boundary, buffers never transfer allocation ownership, and panic/exception cannot unwind. One generator, `tools/authority-kernel-abi-gen`, is the sole source for launcher binding, kernel binding, `include/ep_authority_kernel_api_v1.h`, `ep-authority-kernel.def`, field/export goldens and the check-mode digest; handwritten or independently generated ABI declarations fail CI. PE export-table, header, DEF, both Rust modules and runtime `AuthorityKernelAbiReadbackV1` must agree on the one export, table size/offsets, function RVAs and executable/read-only section characteristics.

The launcher verifies signed slot pointer, TPM/DATA_HDD/off-host anti-rollback head, held final handle, Authenticode, binary/SBOM/implementation digests, ABI version/size and complete function table before calling any entry. Failure leaves business admission closed and either loads the exact prior pointer or stops. The initial signed pointer uses `AuthorityKernelSlotBindingV1::BOOTSTRAP_BUILD`, slot ordinal 1 and null predecessor; it binds candidate run/build attempt/source tree/manifest/kernel SBOM/ABI readback and intentionally has no generation, transition, package or admission ref. The pointer is signed before, and then referenced by, `WindowsAuthorityArtifactSetV1`, so there is no hash cycle. Later hotplug pointers use `GENERATION_TRANSITION` with ordinal greater than 1 and the exact predecessor/target generation/transition/package/implementation tuple. `WindowsNativeBinary.abi_readback_ref` is mandatory only for `RUST_KERNEL` and null for every other native class, closing DLL/SBOM/ABI/AuthentiCode slot reconstruction. Build goldens compile both PE and `cdylib` and reject missing/extra/forwarded export, null callback, layout/section drift, unsupported version, buffer/role/protocol error, binding/predecessor drift or generator check failure.

Task 10's `StaticProjectionOnly` MSI remains a five-service-table engineering shell and intentionally has no kernel component. Task 14 is the sole owner of the Release-mode delta: `Product.wxs`/`Services.wxs` select `installer/windows/generated/AuthorityKernelBootstrap.wxi` only when the closed build mode is `Release`. That generated fragment contains exactly one content-addressed slot-1 DLL component, its SBOM/ABI readback, the signed bootstrap pointer/head and their fixed locator metadata; it adds zero `ServiceInstall|ServiceControl` rows and never changes the five launcher ImagePaths. The MSI extractor emits separate `STATIC_SERVICE_SET` and `RELEASE_KERNEL_BOOTSTRAP_SET` projections and exact-matches the independent golden. Static mode rejects the Release fragment; Release mode rejects its absence, an extra native binary, a generation-bound bootstrap pointer or any service-row drift.

`apps/core-server/src/platform/windows_service/dispatcher.rs` is therefore only the immutable SCM/argv-to-role adapter. It recognizes the five manifest vectors, loads the one verified API table and forwards the closed numeric role; it contains no Authority boot, POWER, package, signer or recovery business handler. `crates/platform/authority-kernel/src/dispatch.rs` is the sole owner of those five role implementations. The ABI uses caller-owned buffers and opaque integer handles; every call returns a fixed status code, every callback is `extern "system"`, and every DLL boundary catches panic before it can unwind. The only export has exact signature `ep_authority_kernel_get_api_v1(requested_abi_version: u32, out_api: *mut *const EpAuthorityKernelApiV1) -> EpStatusV1`; V1's exact table order is `{abi_version,struct_size,initialize,run_service_main,dispatch_typed_control,shutdown,copy_last_error}`. `initialize` returns one opaque kernel handle, output/error bytes are copied into launcher-owned buffers, and `shutdown` consumes the handle. Missing/extra export, null pointer, undersized table/buffer, unsupported version, invalid role/protocol, cross-module allocator, panic or exception fails closed before route admission.

The versioned kernel owns the ordinary Authority Server, dormant continuation, least-privilege control broker, raw/journal signing facades and their application composition; the launcher owns only boot verification, DLL lifetime and SCM dispatch. It also lands sealed carrier-specific collectors for every runtime deployment row and the four fresh production-activation readback sets. Task 14 builds no final participant set and emits no candidate. Task 15's ordered build/install carriers create the first clean-HEAD PE, initial DLL slot and package closure, install/read back every graph-active participant, five permanent Authority services, the fixed G0 evidence-signer broker, the three component services, immutable recovery Scheduled Task and on-demand helper, while proving the off-host target and every deferred participant absent. Continuation remains dormant when `ActiveRecordPath` is absent; no attempt changes any service start type/raw ImagePath/argv, calls `ChangeServiceConfig`, creates/deletes a service or makes recovery-tool an SCM service. Static manifest/WiX/MSI and dynamic readbacks must all exact-match.

Task 14 also adds one authenticated Authority command, `FinalInstalledGenerationAuthorityV1::begin_or_adopt`, implemented in the already upper `ep-platform-generation-activation` crate and exposed only through `EPAuthorityServer`; xtask is a client/composition adapter. Its public wire contains only `{schema_version,candidate_run,precursor_journal_checkpoint_ref,windows_authority_build_result_ref,windows_service_install_result_ref}`. The Authority typed-loads the exact terminal same-run build/install chains, five Authority service runtime readbacks, the graph-bijective complete runtime deployment closure/readback set, the six-row backup/recovery component set, five on-host component readbacks, the complete fixed G0 evidence-signer-broker install/readback and off-host target proof; it does not accept a manifest, item, participant, process list, binary, generation number or topology ref from the caller. Under the G1 transition store lease it exact-loads the current OBSERVED predecessor and deterministically constructs the immediately next signed generation plus every signed reverse plan, using only final installed `ACTIVE` artifact/file identities, graph/projection/config generations, carrier-specific positive readbacks, service rows/endpoints and component capabilities as the forward item set. It builds and stores a new runtime-topology declaration whose participant set is a bijection with the closure's `ACTIVE` subset. Every deferred row is absence-only and excluded from artifact binding, declaration participants, database consumers, generation-required participants, participant-item edges and ACKs; the generation item vector remains an independent exact projection and is never classified by deferred participant status. It persists the new manifest/approval/declaration triple and calls the sole G1 activation coordinator until that exact attempt is durable `OBSERVED_COMMITTED` with the complete active-participant ACK set. The returned private `VerifiedFinalInstalledObservedGenerationV1` exposes only attempt/manifest/declaration/ACK refs and can be reconstructed only by exact command identity plus the transition store; no scan/latest API or second attempt exists. A crash before/after manifest, reverse plan, declaration, transition begin, participant dispatch, ACK draft/object or OBSERVED commit adopts the exact existing prefix. Pre-install invocation, omitted/extra/deferred-present participant, old authority/component/broker binary, desired-only state, mixed ACKs, stale graph/projection/config, missing service challenge, co-located backup target or generation that does not equal predecessor+1 fails before candidate freeze.

`ProductionActivationAuthorityV1` is a separate post-certificate verifier in `ep-platform-release`; certificate issuance never activates production implicitly and release never owns a business-route switch. Its sole `prepare(verified_release_certificate,acceptance_id,sealed_live_collectors)` entry reconstructs the exact candidate, observed-selection/final-installed generation, topology certification, P340/certified-HDD root, runtime deployment, Windows Authority/component installation, full-cut backup/restore chain and all safeguards. `acceptance_id` resolves only through the immutable Authority audit/fact store into a private two-person `VerifiedSingleDiskDegradedProductionAcceptanceV1` for exact 20-user profile/candidate/certificate/deployment and complete five-risk/safeguard closure. It captures four fresh sealed readbacks—topology, runtime deployment, installed Authority/components and strict `StorageSafeguardReadbackV1` under its exact purpose/media—then typed-loads the certified latest `BackupCheckpointV1`, reconstructs its predecessor chain, typed-loads the active-config-selected `BackupTopologySigningTrustCurrentPointerV1` and its independent current `BackupTopologySigningTrustManifestV1`, then verifies the selected current topology signer only through the resulting private `VerifiedBackupTopologySigningTrustCurrentV1`, exact-loads the current singleton-target authority-storage manifest and every topology-bound support-evidence object, and requires the fresh safeguard head/latest to byte-equal that certified checkpoint rather than a discovered newest object or valid ancestor. Only then does it require the safeguard's activation/retry/attempt/candidate/certificate/selection binding, new nonce/session/current boot, topology-derived expiry, `HEALTHY` state, passing target/quota/permission probes and exactly disconnected A/B media before it proves graph/active/deferred/item/ACK bijections and certified values, stores all four objects, persists only `LIVE_READBACK_BOUND`, and returns private `VerifiedProductionActivationReadyV1`. Failure appends `FAILED_HELD`; exact-command retry revalidates current certificate/acceptance, increments retry ordinal and recollects all four with a new challenge/object under the same activation ID; an earlier retry's still-live safeguard ref is rejected. Release exposes no `commit_activated_cas` path.

The core-server composition immediately passes that private proof—not a raw ref—to the upper `ProductionGenerationAdmissionAuthorityV1`. Its sole `commit_activation_and_genesis_admission_cas` port locks the activation-ready row, current OBSERVED tuple and admission head, derives the exact deployment-wide `ProductionGenerationAdmissionV1{admission_kind=GENESIS_FULL_CERTIFICATION}`, and in one PostgreSQL transaction appends activation `ACTIVATED`, inserts the admission, advances the current head and increments `business_api_generation`. One commit time populates both terminal records; response-loss reloads the same joint result, while every statement/commit crash exposes either neither side or both. Only that transaction can open routes. Later `PACKAGE_DELTA` admissions exact-bind predecessor admission, impacted scope set, complete ACK/transition set and unchanged certified resource envelope. Global foundation/kernel classes or any resource increase cannot delta-admit and remain held until a new full certification. `ROLLBACK_REOPEN` requires fresh predecessor installed-state/readback/ACK proof after reverse completion.

The one `ProductionAdmissionGateV1` is injected at the first common `CommandPipeline` and query-router boundary, but it does not perform a racy read-only check. After strict route/payload parsing, authentication, field/row authorization and current-tenancy verification produce private `VerifiedAdmissionTargetScopeSetV1`, `begin_or_adopt_business_request(request_id,request_binding_sha256,scope_set)` takes the same deployment-scoped serializable lock/CAS used by admission and hold writers. Under that lock it exact-checks the server-derived current authority epoch plus current OBSERVED/admission/API generation, requires that epoch to equal the typed-loaded admission, OBSERVED generation and verified security context, intersects every current hold regardless of tagged cause, and create-new inserts—or response-loss adopts—the exact `ProductionAdmissionExecutionLeaseV1{state=ACCEPTED}` before returning an affine permit. The server-derived epoch is covered by `request_binding_sha256`; no client field/header supplies it. The command transaction consumes the permit and terminalizes the lease as `COMPLETED_IN_PLACE` with its durable result and unchanged epoch, or exact-once Outbox/workflow handoff terminalizes it as `HANDED_OFF_EXACT_ONCE`. Hold closure takes the same lock, writes `ADMISSION_CLOSED` before release, and may reach `DRAIN_COMPLETE|BARRIER_COMMITTED` only after the indexed intersecting ACCEPTED count across all epochs is durably zero; only a `DATA_HDD_DISASTER_REPLACEMENT` cause may substitute its exact typed unreachable-and-fenced-old-authority proof. An old-epoch orphan without positive no-effect/handoff proof or that exact disaster fence remains accepted and keeps the hold closed. Reusing one `(deployment_id,request_id)` across an epoch change fails rather than executing twice; an existing terminal result is exposed only through the registered receipt query. There is no check/handler gap, process-local counter, raw client scope, forged result ref, cross-cause fence or handler-local Boolean.

The bypass is a generated exact ten-row registry, never a prefix or wildcard: `GET /internal/v1/health/live -> HEALTH_LIVE/f57.platform.health.read`; `GET /internal/v1/health/ready -> HEALTH_READY/f57.platform.health.read`; three `POST /control/v1/commands` selectors `PRODUCTION_ACTIVATION_COMMAND/f57.production.activate`, `PACKAGE_MAINTENANCE_COMMAND/f57.package.maintenance.execute`, and `RECOVERY_OPERATION_COMMAND/f57.recovery.operate`; four `POST /control/v1/queries` selectors `PRODUCTION_ADMISSION_STATUS_QUERY/f57.production.admission.read`, `PACKAGE_MAINTENANCE_QUERY/f57.package.maintenance.read`, `RECOVERY_OPERATION_QUERY/f57.recovery.operation.read`, and `AUTHORITY_RECOVERY_PROOF_QUERY/f57.recovery.proof.read`; plus `GET /control/v1/commands/{request_id} -> ORIGINAL_COMMAND_BYPASS_INHERIT`, which exact-loads the stored original command classification/capability. Every row fixes `CONTROL_METADATA_NO_CUSTOMER_FIELDS` and `bypasses_only_production_admission_and_hold=true`; it does not bypass parse/auth/CSRF/authorization/MFA/reauth/SoD/epoch/operation ID/query-adopt/rate limit/audit/typed filtering and cannot construct `AuthorizedPgTx`. Router/registry exact-set tests deny every unknown route/selector and keep events, files, employee/portal, MCP and business APIs gated. Thus generation drift, missing admission, intersecting hold, UNKNOWN, store failure or lease conflict returns no permit even if the process is healthy.

`crates/platform/generation-activation/src/admission_store.rs` and `docs/evidence/f57-production-admission.v1.schema.json` are the sole Rust/schema owners. They freeze the exact eighteen-field `ProductionGenerationAdmissionV1` wire `{schema_version,purpose,admission_id,admission_kind,deployment_id,impacted_scopes,impacted_scope_set_sha256,authority_epoch,admitted_generation_number,admitted_generation_manifest_ref,admitted_generation_digest_sha256,observed_activation_attempt_id,exact_participant_ack_refs,exact_package_transition_refs,predecessor_admission_ref,certified_resource_envelope_ref,business_api_generation,admitted_at_unix_ms}` under purpose/media `EP-F57-PRODUCTION-GENERATION-ADMISSION-V1` / `application/vnd.ep.f57-production-generation-admission-v1+json`. The exact fifteen-field `ProductionAdmissionHoldV1` is `{schema_version,purpose,hold_id,deployment_id,scope,authority_epoch,cause,predecessor_admission_ref,state,sequence,previous_record_sha256,drain_result_ref,write_barrier_id,recovery_checkpoint_ref,recorded_at_unix_ms}`; `reservation_ref` at top level, `admitted_generation_ref`, a generic reason string and unknown cause are forbidden. `cause` is exactly one tagged `ProductionAdmissionHoldCauseV1`: `PACKAGE_MAINTENANCE{reservation_ref}`, `CURRENT_ROOTS_ROTATION{target_backup_topology_ref,target_authority_storage_manifest_ref,target_topology_signing_trust_current_ref}`, `DATA_HDD_DISASTER_REPLACEMENT{recovery_attempt_ref}` or `UPS_RUNTIME_LINK_LOSS{runtime_loss_episode_ref}`. The exact twenty-field `ProductionAdmissionExecutionLeaseV1` is `{schema_version,purpose,lease_id,request_id,request_binding_sha256,deployment_id,authority_epoch,target_scopes,target_scope_set_sha256,admission_ref,observed_generation_manifest_ref,observed_generation_digest_sha256,business_api_generation,state,terminal_result_ref,sequence,previous_record_sha256,cas_version,row_sha256,recorded_at_unix_ms}`. `authority_epoch` is server-derived and exact-matches current authority, typed admission, OBSERVED generation, verified security context and request-binding digest; the terminal row retains it. Their plain-JCS media are respectively `application/vnd.ep.f57-production-generation-admission-v1+json`, `application/vnd.ep.f57-production-admission-hold-v1+json`, and `application/vnd.ep.f57-production-admission-execution-lease-v1+json`; the embedded cause has no independent media. All deny unknown fields; schema, Rust, SQL and create-new/adopt byte goldens exact-match cause/reference closure, predecessor, canonical vectors, CAS/hash, the four cause-specific state graphs/nullability and one-way transitions.

Migration `20261025092530` creates `platform_core.production_activation_attempts`, deployment-wide immutable `production_generation_admissions`, CAS/hash-chained `production_admission_holds`, durable `production_admission_execution_leases`, their exact predecessor/current/request/state/scope indexes and the final unpoliced-table closure. Activation-attempt columns/states remain the master projection with `REQUEST_COMMITTED|LIVE_READBACK_BOUND|FAILED_HELD|RETRY_COMMITTED|ACTIVATED`; only the upper joint-commit adapter may write `ACTIVATED`. Admission rows persist all exact admission fields and canonical impact/ACK/transition vectors; one current head per deployment/epoch advances only by predecessor CAS. Holds persist the byte-identical `cause_kind|cause_jcs`, one-way cause-specific prefix and unique live scope overlap; no reservation-key assumption remains. Leases persist `authority_epoch bigint not null` immediately after deployment identity and remain unique by `(deployment_id,request_id)`—never by `(deployment_id,authority_epoch,request_id)`—so the same request ID cannot produce a second effect after an epoch change. Sequence-zero ACCEPTED exact-matches the server-derived current/admission/OBSERVED/security-context epoch and has null predecessor/result; sequence-one terminal retains that epoch, has a nonnull result and exact predecessor hash, and cannot reopen. The same deployment lock orders admission head, holds and leases, and a drain/barrier CAS requires exact intersecting ACCEPTED count across all epochs zero in the same transaction, except the disaster cause's exact typed unreachable-and-fenced proof. No table is delete/reset authority. Crash/race tests cut every statement, commit, response loss, permit-before-hold, hold-before-permit, multi-scope intersection, cross-cause substitution, terminal-CAS loss, Outbox handoff, cross-epoch request-ID replay and terminal epoch drift; they prove no standalone activation/admission half, no write after barrier and no orphan guess. Delta/rollback tests cover all-or-nothing impact sets and stale concurrent writers.

Both release-owned POWER authorities have exactly one public `compose` factory while all fields and proof constructors remain private. The plan authority receives the artifact verifier plus journal/object resolver/store ports; the control-broker authority receives the installed pipe/SCM identity verifier, one durable intent/marker/attempt-lock store, the narrow Win32 shutdown API port and exact compiled Service SID. Construction verifies the fixed pipe, DACL, Service SID and privilege profile. The real core-server composition test constructs both authorities with production adapters and exercises their public entrypoints; no default, test-only factory, ambient pipe identity or caller-selected service/path can enter.

Task 14 activates exactly eight semantics already reserved by the sole G0 gate-journal contract: five continuation events `CARRIER_CONTINUATION_ARMED|CARRIER_CONTINUATION_PRE_SHUTDOWN_COMMITTED|CARRIER_CONTINUATION_POST_RESTART_COMPLETED|CARRIER_CONTINUATION_DISARMED|CARRIER_CONTINUATION_FAILED` plus three completion-finalization events `CARRIER_STAGING_COMPLETION_FINALIZATION_STARTED|BOUND|RECONCILED`. With Task 13's five activated qualification semantics the final F57 delta remains exactly 13. The four success-path events ARMED/PRE_SHUTDOWN_COMMITTED/POST_RESTART_COMPLETED/DISARMED carry the same TestID/attempt and strict-extending typed `PowerShutdownContinuationStatePrefixV1`; ARMED also carries `staging_plan_ref`, and PRE_SHUTDOWN_COMMITTED carries the exact dispatch-intent ref. FAILED instead has exactly six fields `{test_id,execution_attempt_id,staging_plan_ref,failure_readback_ref,failure_cleanup_ref,error_code}` and no state prefix. `crates/platform/gate-journal-contract` and `docs/evidence/f57-gate-run-journal.v1.schema.json` remain the sole Rust/schema owners for prefix, events and checkpoints. Release supplies domain validators; xtask merely registers them. Neither Task 13 nor Task 14 changes the journal enum, codec or schema, and gate journal never imports release or P340. Goldens reject a FAILED event with an unknown/extra `state_prefix` as well as any missing or additional sixth-field-set member.

`docs/evidence/f57-offline-schema-manifest.v1.schema.json` is the unique minimal bootstrap owner for manifest/purpose/closure-root/descriptor/binding. Foundation/G0 nominals are imported, never copied. Release schema imports only canonical helpers, including UPS-common once; runtime, package, backup, PostgreSQL package-lock/install, UPS, tenancy and upper admission retain their sole schemas. For each generation package item, the object graph follows nine-field desired item -> portable registry + desired thirteen-field package -> eight-field implementation manifest -> complete artifact/SBOM/schema/WIT/signature/migration/foundation closure, plus the concrete scope's tenancy snapshot. From each selected fourteen-field ACK it follows `participant_apply_readback_ref` -> canonical tagged item readbacks -> nonnull per-participant transition -> trust snapshot, exact operation request/results and final installed-state readback. A maintenance transition additionally reaches reservation, historical 30-field plan, current execution authorization and decisions, hold, actual BACKUP checkpoint, its full `AuthorityRecoveryCutManifestV1`, bound `BackupTopologyV1` and applicable `StorageSafeguardReadbackV1`; reverse targets reach prior pure items or the typed retained-data absence proof. Production delta/reopen follows predecessor admission, exact ACK/transition set and resource envelope. No plan-finalization or mutable recovery journal is copied/replayed offline. No release nominal copy, registry/path scan, target-only shortcut, current/latest substitution, reverse edge or cycle is legal.

The flow-owned recovery-policy schema is a separate helper descriptor. Release evidence contains only the exact concrete-media `recovery_certification_policy_ref`; its verifier typed-loads the strict policy, exact-compares plan/candidate/clean-cut/outer-attempt/certification/subattempt rows, and makes that helper descriptor and object reachable in the offline closure. It never copies the policy nominal into release schema/Rust or invents a signer row.

`crates/platform/runtime/src/topology.rs` remains the sole Rust owner of both strict plain topology wires, their pure certification builder and strict parser; it imports neither release nor P340 types. `crates/platform/release/src/generation.rs` plus `docs/evidence/f57-generation.v1.schema.json` remain the sole G0 Rust/schema nominal owners of the exact signed manifest, signed reverse-plan and plain ACK family, while `crates/platform/release/src/generation_approval.rs` plus `docs/schemas/f57-generation-approval-registry.v1.schema.json` solely own the exact three-row approval registry and its verification wrapper. `crates/platform/release/src/activation_attempt.rs` remains the sole lower-layer owner of the durable activation-attempt record/codec/store port, including terminal `OBSERVED_COMMITTED`; Task 14 consumes that lower port plus the pure capability-graph compiler/runtime verifier and creates no local generation/reverse/approval/activation payload, envelope, ACK, purpose, media, item, digest type or upper activation wrapper. `ep-platform-generation-activation` stays an upper coordinator with the only private `VerifiedGenerationBoundCapabilityGraphV1` and a one-way dependency on release/runtime/capability-graph; release and runtime neither import, re-export, path-include nor copy its `coordinator` or `verified_graph` modules. `crates/platform/release/src/runtime_topology_certification.rs` solely owns the private production coordinator `RuntimeTopologyCertificationAuthorityV1` and its private non-wire result `VerifiedRuntimeTopologyCertificationV1`; the same-named xtask file only wires the CLI. Its sole `load_or_reconstruct_verified(candidate,p340_terminal_checkpoint,fresh_live_readback)` entry uses its privately injected journal/activation/object stores and deterministically reloads/reverifies the frozen candidate, matching terminal-PASS P340 result/receipt, declaration ref/exact bytes and journal checkpoint, then reconstructs the ephemeral wrapper. `fresh_live_readback` is exactly runtime's sealed `RuntimeTopologyReadbackCollectorV1`, never a caller-created DTO/provider. No serialized wrapper, raw certification ref, scan, “latest” object or caller-built proof crosses a process boundary; both POWER and `gate g6` call this same entry with a fresh sealed runtime readback.

The type's fields and result constructor remain private, but release exposes the one public composition factory `RuntimeTopologyCertificationAuthorityV1::compose(artifact_verifier,generation_approval_verifier,activation_attempt_store,journal_reader,object_contract_resolver,object_store)`. Cross-crate composition tests construct it from real ports and call the sole method. That method first invokes only G0's domain-neutral `load_exact_authenticated_prefix(checkpoint)`, then release-local `select_unique_terminal_infrastructure_record(candidate,prefix,profile_ref)` checks the read-only header/records/checkpoint accessors, exact candidate run, the one infrastructure-capacity TestID, recipe/profile coherence, unique terminal PASS result and no later row. In this graph version the recipe is exactly `P340_RELEASE72_HOUR` and typed-loads only the P340/capacity objects plus terminal `SINGLE_DISK_DEGRADED_PRODUCTION`; `IAAS_WINDOWS_SERVER_HDD_STRICT`, any provider-power recipe and every provider/VM/HDD-cache/snapshot/vTPM object are unregistered and rejected before STARTED. The G0 port never accepts a candidate, recipe or infrastructure type. Every graph, projection, declaration, P340 capacity and certification object load resolves its exact generated occurrence/class contract first; `recompile_and_exact_match_candidate_graph` receives the resolver as well as the store and has no raw/generic-media load path. Missing, mixed, duplicate, future-profile or cross-profile objects fail closed.

The authority first re-verifies declaration ref/media/size/digest/JCS/live equality and typed-traverses the P340 result to the exact soak and reachable capacity certificate. It then exact-loads only `candidate.generation_manifest_ref`, `candidate.generation_approval_registry_ref` and every `candidate.generation_participant_ack_refs` object from the explicit bundle, constructs private `VerifiedGenerationApprovalRegistryV1` only from the storage-manifest-pinned fixed-path registry, reconstructs private `VerifiedGenerationManifestV1` only through `GenerationApprovalVerifierV1`, and uses the release-owned `GenerationActivationAttemptStoreV1::load_exact` to require one terminal `OBSERVED_COMMITTED` record whose frozen manifest/registry/declaration/attempt/ACK exact-set equals the candidate triple; no scan, current-pointer shortcut or side-channel substitution is accepted. Independently of the upper activation crate, it exact-loads the manifest's graph/projection inputs, reruns the sole pure CapabilityGraph compiler/canonicalizer through its lower API, and applies the runtime-owned sealed declaration/live-readback verifier. The manifest whole-envelope digest must equal its ref and the declaration's `generation_digest_sha256`; the approval ref must equal the wrapper provenance; ACK refs use exact plain-ACK media and form the canonical unique complete manifest-required participant set from one OBSERVED activation attempt. Each ACK recomputes `participant_definition_sha256=SHA256(JCS(the exact RuntimeParticipantV1 row))` and `applied_item_set_sha256=SHA256(JCS(the canonical exact GenerationItemRefV1 subset named by required_item_ids))`, exact-matches manifest/declaration/deployment/epoch/number/digests, and agrees with fresh live participant/item readback. The authority further exact-matches candidate run/manifest, all three profiles, storage-manifest/DATA_HDD identity, P340 policy definition, declaration graph/generation digests, host fingerprint, `20` users and `259200` seconds. A generic verified envelope, payload hash, generation number, reserialized envelope, desired-only generation, wrong/adjacent/ambient registry, missing/extra/duplicate/mixed-attempt ACK, raw/double-wrapped ACK, upper-crate `VerifiedGenerationBoundCapabilityGraphV1` or locally reconstructed generation wire cannot substitute. Only then may it call the G0 pure builder.

Task 14 also imports, without redefining, G0's production-linkable `EvidenceObjectStoreV1` port and the sole G0/G1-staged `FileEvidenceObjectStoreV1` adapter. Release tooling receives the validated evidence-bundle lane by dependency injection; the installed authority service receives only the G1 DATA_HDD lane through core-server composition. `xtask/src/f57/run_artifact_store.rs` remains descriptor/tooling composition, never a production library, and no release/core-server module may import, re-export, path-include, or copy it. Workspace/all-target tests must compile both consumers against the same port/adapter byte engine and reject a second object-store implementation, any production-to-`xtask` edge, raw-root construction, SSD authority storage or URI/path divergence.

The authority canonicalizes the returned `RuntimeTopologyCertificationV1`, derives its sole content-addressed ref, create-new stores it through `EvidenceObjectStoreV1`, typed-reloads the exact bytes under media `application/vnd.ep.f57-runtime-topology-certification-v1+json`, and only then returns `VerifiedRuntimeTopologyCertificationV1` with private value/ref/checkpoint fields and read-only accessors. It has no signer row, signing/finalization event, caller path/ref, directory scan, generic-media overload or raw-wrapper constructor. Recovery before the object write recomputes only from the checkpoint-fixed candidate/declaration/generation/ACK/P340/capacity inputs; recovery after the write adopts only the identical derived bytes/ref. A pre-P340 call, non-P340 terminal receipt, ref/profile/storage/policy/graph/generation/approval/ACK/host/capacity/duration/live-readback drift, alternate object or second differing write fails closed. POWER entry requires this same verified wrapper to exist, while `certify_release` accepts only its ref accessor and never a caller-supplied certification ref.

`F57OfflineSchemaManifestV1` is the only offline schema locator. Its closure generator starts from final `ReleaseCertificateV1` and the unique `docs/evidence/f57-offline-schema-manifest.v1.schema.json` bootstrap owner, includes that bootstrap descriptor/binding, and follows every discriminator-reachable typed signed `ArtifactRefV1` contract through candidate—including the exact signed generation envelope, its pinned approval-registry provenance and all strict plain participant ACKs—L2/L3, six receipts, journal/checkpoints, all 276 `TestResultStoreV1` descriptor rows over the exact five `TestResultEnvelopeKindV1` kinds, client/package and the committed client decision's exact trust-closure/archive schema owners, all six current carrier roots including only P340 infrastructure plus physical UPS power safety, the canonical 89-row signer registry and reachable external trust-domain artifacts. `docs/evidence/f57-generation.v1.schema.json` must appear exactly once with the signed manifest, signed reverse-plan and plain ACK media; `docs/schemas/f57-generation-approval-registry.v1.schema.json` appears exactly once with the pinned signed three-row approval-registry media; runtime topology, Windows runtime deployment, backup topology/topology-signing-trust-manifest/topology-signing-trust-current-pointer/storage-safeguard, PostgreSQL Windows package-lock/install-contract/install-readback/log-retention, root-rotation/disaster-replacement, P340 and UPS helpers likewise appear exactly once as separate helper descriptors with their complete plain media. No IaaS/provider-power schema or descriptor is reachable in this graph version. None requires a release-schema nominal import. For each result row, the schema binding is exactly `artifact_kind=<TestResultEnvelopeKindV1 SCREAMING_SNAKE_CASE wire>`, `discriminator=<exact TestID wire>`, and `media_type=<descriptor fixed media>`—276 descriptor bindings, never 276 invented schema kinds. It recursively follows every canonical JSON Schema `$ref` and joins all 185 handler artifact-media contracts; opaque binary, log and framed sample objects remain digest/media checked and do not invent schemas. The output must exact-equal the complete transitive closure. A missing generation/reverse/ACK/approval/topology/runtime-deployment/backup-topology-signing-trust/backup-safeguard/PostgreSQL-install/log-retention/root-replacement/P340/UPS helper descriptor, any future-profile descriptor, duplicate nominal, reverse edge, developer-maintained allowlist or hardcoded four-file shortcut is forbidden.

Every `F57OfflineSchemaDescriptorV1` contains the exact seven fields `schema_id`, `artifact_bindings`, `imports`, `relative_path`, `media_type`, `sha256`, and `size_bytes`. `schema_id` is `urn:ep:f57:schema:<sha256(UTF-8 canonical repository schema path)>` and exact-matches source `$id`; bindings sort by `(artifact_kind,discriminator,media_type)`, helper-only imports have no bindings, `relative_path` is `schemas/repo/<canonical repository schema path>`, and imports use the corresponding bundle-relative descriptor paths. Source and copy bytes are identical and preserve reviewed relative `$ref` spellings. Descriptors sort by `relative_path`; path, ID and binding sets are unique. The current generated descriptor count and descriptor-set digest are byte-golden outputs in `f57-offline-schema-closure-v1-golden.json`, never hand-maintained constants.

The release owner defines the `OfflineSchemaManifestStoreV1` port and alone constructs private `VerifiedOfflineSchemaManifestStoreV1` from a verified bundle root, verified journal header/run ID and frozen candidate-run identity; the file adapter only implements its opaque derived locators. The API can create/adopt the exact descriptor path, copy exact schema bytes, typed-load the fixed manifest, and return a checkpoint—never accept a root/path, scan, or resolve “latest”. It create-new copies and fsyncs the exact closure below final `RUN_DIR`, adopting byte-identical existing copies and rejecting conflicts. It then freezes the descriptor set in a journal checkpoint and durably appends the exact five-field `EVIDENCE_ENVELOPE_FINALIZATION_STARTED{artifact_kind=OFFLINE_SCHEMA_MANIFEST,finalization_attempt_id,frozen_input_checkpoint_ref,issued_at_unix_ms,expires_at_unix_ms}` record; `kind`, signer-registry key `F57_OFFLINE_SCHEMA_MANIFEST_V1`, or any other alias is forbidden in the journal event. Only then may it sign `schemas/offline-schema-manifest.v1.json`; payload `closure_root=RELEASE_CERTIFICATE`, finalization ID, issue/expiry times and frozen input closure exact-match that bound/reconciled record. A pre-STARTED crash may leave only verified static copies; post-STARTED recovery uses the frozen checkpoint to create the first manifest or adopt exact existing bytes once, without re-signing or changing time. Crash tests cut every copy/create/fsync/event/sign/object-bind boundary and reject conflicts or another run's store wrapper. The manifest is signed under the one canonical 89-row `F57ArtifactSignerRegistryV1`, whose separate signer lookup is exactly `F57_OFFLINE_SCHEMA_MANIFEST_V1/NONE`; every Task 14 verifier resolves a signer only by that registry row, and ambient trusted signers or wildcard/default rows are forbidden. Existing Development/Integration goldens remain owned by G4/G5; Task 14 adds the Final L2 golden plus candidate—including its three generation artifact fields, mandatory observed-selection ref, and missing/extra/duplicate/mixed-ACK, generic-proof, wrong-registry and media/ref one-field mutations—offline closure/manifest with generation three-root, generation-approval and topology helper reachability, authority artifact set, staging plan/completion/crash cuts, one raw row per recipe, ordered three-row recovery set, carrier result, L3, certificate, and `1/1/1/3/1/1`, cross-purpose, unknown-field, imported gate-receipt-ref, and cross-schema negatives. It consumes G0's canonical generation/approval and gate-receipt schemas/goldens and creates no second definition. It produces no live frozen candidate, evidence envelope, release receipt, certificate, or production-activation claim in this implementation task.

The offline manifest URI uses the common canonical segment join. Empty `RUN_DIR_REL` yields exactly `evidence-relative://bundle/schemas/offline-schema-manifest.v1.json`; a nonempty value yields `evidence-relative://bundle/<RUN_DIR_REL>/schemas/offline-schema-manifest.v1.json`. Empty/doubled segments, leading/trailing slash, dot/parent, percent/case alias or path escape is noncanonical. Byte-goldens cover both bundle-root and nested run directories.

The release subordinate-readback registry is a closed table of exactly 18 field bindings and 17 strict parser definitions; rows 11/12 intentionally share one role-tagged offline-media parser. All leaves are strict plain JCS authenticated by their containing signed raw carrier, loaded by `EvidenceObjectStoreV1` only after exact digest/size/media verification, and gain no signer-registry row. Parser selection comes only from the containing root field plus closed role; scripts/callers never choose a parser or path.

The offline closure additionally binds `GenerationObservedReleaseSelectionRecordV1` to its sole strict schema/media and follows every selected backup checkpoint through the sole backup-checkpoint schema, its exact `authority_recovery_cut_manifest_ref`, every row of that full enabled-HDD authority cut, and the checkpoint's exact signed BACKUP recovery-domain-manifest ref. The PITR carrier's `ReleasePostgres16PitrEvidencePayloadV1.backup_checkpoint_ref` and every package-maintenance checkpoint use that same traversal rule. These are direct typed root references, not subordinate carrier leaves, so they add no signer-registry row and do not change the `18/17/30` subordinate counts. Goldens reject a missing/raw/generic/wrong-media/wrong-domain/cross-set checkpoint, a cross-backup-set/cross-context cut header, an incomplete/extra/duplicate/cross-barrier recovery-cut row, absent recovery trust descriptor, selection-schema omission, or a recovery `CLEAN_SOURCE_CUT` whose `source_pitr_raw_ref` resolves a different checkpoint.

| # | Exact field selector | Fixed parser ID |
|---:|---|---|
| 1 | `WINDOWS_SERVICE_INSTALL/service_sid_readback_ref` | `EP_F57_WINDOWS_SERVICE_SID_READBACK_V1` |
| 2 | `WINDOWS_SERVICE_INSTALL/service_dacl_readback_ref` | `EP_F57_WINDOWS_SERVICE_DACL_READBACK_V1` |
| 3 | `WINDOWS_SERVICE_INSTALL/ipc_dacl_readback_ref` | `EP_F57_WINDOWS_IPC_DACL_READBACK_V1` |
| 4 | `WINDOWS_SERVICE_INSTALL/storage_routing_readback_ref` | `EP_F57_AUTHORITY_STORAGE_ROUTING_READBACK_V1` |
| 5 | `WINDOWS_SERVICE_INSTALL/trusted_time_readback_ref` | `EP_F57_WINDOWS_TRUSTED_TIME_READBACK_V1` |
| 6 | `POSTGRES16_PITR/database_cut_ref` | `EP_F57_POSTGRES16_DATABASE_CUT_READBACK_V1` |
| 7 | `POSTGRES16_PITR/wal_cut_ref` | `EP_F57_POSTGRES16_WAL_CUT_READBACK_V1` |
| 8 | `POSTGRES16_PITR/attachment_cut_ref` | `EP_F57_ATTACHMENT_CUT_READBACK_V1` |
| 9 | `POSTGRES16_PITR/append_only_readback_ref` | `EP_F57_APPEND_ONLY_BACKUP_READBACK_V1` |
| 10 | `POSTGRES16_PITR/key_domain_readback_ref` | `EP_F57_BACKUP_KEY_DOMAIN_READBACK_V1` |
| 11 | `POSTGRES16_PITR/offline_media_readback_refs/ROTATION_A` | `EP_F57_OFFLINE_ROTATION_MEDIA_READBACK_V1` |
| 12 | `POSTGRES16_PITR/offline_media_readback_refs/ROTATION_B` | `EP_F57_OFFLINE_ROTATION_MEDIA_READBACK_V1` |
| 13 | `BACKUP_RESTORE_CERTIFICATION/clean_source_cut_ref` | `EP_F57_RECOVERY_CLEAN_SOURCE_CUT_READBACK_V1` |
| 14 | `BACKUP_RESTORE_CERTIFICATION/restored_database_ref` | `EP_F57_RESTORED_DATABASE_READBACK_V1` |
| 15 | `BACKUP_RESTORE_CERTIFICATION/restored_attachment_ref` | `EP_F57_RESTORED_ATTACHMENT_READBACK_V1` |
| 16 | `BACKUP_RESTORE_CERTIFICATION/audit_outbox_replay_ref` | `EP_F57_AUDIT_OUTBOX_REPLAY_READBACK_V1` |
| 17 | `BACKUP_RESTORE_CERTIFICATION/key_custody_readback_ref` | `EP_F57_RECOVERY_KEY_CUSTODY_READBACK_V1` |
| 18 | `BACKUP_RESTORE_CERTIFICATION/business_reopen_readback_ref` | `EP_F57_BUSINESS_REOPEN_READBACK_V1` |

One complete execution instantiates exactly `5 + 7 + (3 * 6) = 30` leaf objects: five service leaves, seven PITR slots, and six recovery leaves for each ordered raw `00=INITIAL_RESTORE_VERIFIED`, `01=CANDIDATE_MEASURED`, `02=CERTIFIED`. The three recovery raw contexts byte-repeat one outer carrier `execution_attempt_id`, share one certification ID and one exact flow-owned policy ref, while their mandatory `recovery_execution_attempt_id` values are distinct and byte-equal the corresponding signed-plan/policy rows. The two offline roles are canonical `[ROTATION_A,ROTATION_B]`; the same six recovery parser definitions are reused across all three raw ordinals, never redefined. The concrete-artifact key includes root type, raw ordinal, JSON pointer and vector role, derives the sole plan path, and must exact-match the raw field ref. Missing/extra/duplicate/aliased selector, wrong `18/17/30` count, mixed outer/subattempt, policy/plan/certification/phase drift, role/order drift, generic media, parser override, directory discovery or claimed PASS over a failing leaf fails before aggregation.

`EP_F57_POSTGRES16_DATABASE_CUT_READBACK_V1` exposes both nonempty `server_version` and positive integer `server_version_num`; PASS requires checked `server_version_num / 10000 == 16` and outer `postgres_version` byte-equal to `server_version`, in addition to the exact system/timeline/LSN/manifest/digest/row/integrity predicates. Goldens mutate the version string, numeric setting, major boundary and outer equality independently. The service leaves return exact five-role numeric SID, SCM/IPC DACL, DATA_HDD storage-routing and trusted-time projections; PITR leaves prove one gap-free database/WAL/attachment/append-only/key-domain/two-media cut; recovery leaves prove the same source cut, restored database/attachments, audit+Outbox replay, external 2-of-3 custody and authenticated business reopen for each of the ordered three runs. No Boolean-only PASS is trusted.

`ReleaseWindowsServiceInstallEvidencePayloadV1.authority_services` is the canonical five-role vector in ordinal order `AUTHORITY_SERVER|CONTINUATION|CONTROL_BROKER|RAW_EVIDENCE_SIGNER|GATE_RUN_JOURNAL_SIGNER`. For `AUTHORITY_SERVER`, control, raw facade and journal facade, the runtime variant is exactly `RUNNING`: the collector opens the SCM process by PID, binds a nonzero boot-scoped process-start key, holds and re-resolves its executable handle, exact-matches the Authenticode/file identity and service-token numeric SID, then performs one unpredictable 32-byte nonce challenge over an authenticated role-specific endpoint/session. The response binds role, service SID, boot ID, PID, process-start key, binary identity and nonce; PID reuse, an old boot, an endpoint swap, wrong-role response or a digest without authenticated session proof fails. The Authority row additionally proves ordinary recovery completed and the configured HTTPS API became ready before its recovery/readiness pipe answers. The continuation must instead have exactly one same-boot `DORMANT_CONTINUATION_SELF_CHECK`: SCM started the installed binary once with no `ActiveRecordPath`, the held image/token bindings match, its signed plain self-check receipt reports zero business/dispatch/state side effects, then it exited `STOPPED` with `exit_code=0` and zero recovery actions. Backup writer, checkpoint signer and data-volume unlock broker use distinct `WindowsBackupServiceRuntimeReadbackV1` rows whose nonce response binds the exact `WindowsBackupComponentIdV1`; the unlock row additionally binds the complete static install readback, restricted token and bootstrap ref. None can be serialized or accepted as an Authority service role. A continuation still RUNNING, never started, stopped by crash, nonzero exit/recovery action, activation value, old receipt or any process/image/token/challenge drift prevents `WINDOWS_SERVICE_INSTALL` PASS.

For the same candidate, `DATABASE_CUT.postgres_system_identifier`, `WAL_CUT.postgres_system_identifier` and every one of the three `RESTORED_DATABASE.postgres_system_identifier` values must byte-equal the `Postgres16WindowsInstallReadbackV1.postgres_system_identifier` reached through that candidate's exact terminal Windows-service-install evidence; that outer identifier already byte-equals the independently read engine `cluster_system_identifier|pg_control_system_identifier|sql_system_identifier`. The join follows typed candidate/evidence refs and never accepts a loose identifier, latest install, same-version cluster or caller-supplied ref. A one-field system-identifier substitution in any cut, restore run, install outer value or one of the three engine observations fails before PITR/recovery PASS.

- [ ] **Step 1: Write failing candidate, release aggregation, and G6 handler exact-set tests.**

```rust
#[test]
fn any_post_freeze_artifact_mutation_invalidates_candidate() {
    let mut candidate = frozen_candidate();
    mutate_one_release_artifact_digest(&mut candidate.payload.artifacts[0]);
    assert_eq!(candidate.verify().unwrap_err().code(), "FINAL_CANDIDATE_DIGEST_MISMATCH");
}

#[test]
fn final_candidate_rejects_migration_closure_or_artifact_ref_drift() {
    assert_code(candidate_with_wrong_f57_reservation_ref(), "FINAL_CANDIDATE_MIGRATION_CLOSURE_MISMATCH");
    assert_code(candidate_with_identity_closure_hash_mismatch(), "FINAL_CANDIDATE_IDENTITY_MIGRATION_MISMATCH");
    assert_code(candidate_with_signer_registry_digest_drift(), "FINAL_CANDIDATE_SIGNER_REGISTRY_MISMATCH");
    assert_code(candidate_with_wrong_offline_schema_manifest(), "FINAL_CANDIDATE_OFFLINE_SCHEMA_MISMATCH");
    assert_finalization_attempt_is_unpredictable_and_journal_bound();
}

#[test]
fn final_candidate_artifact_roles_are_exact_five_lanes() {
    assert_eq!(frozen_candidate().payload.release_artifact_lane_wires(), [
        "android-client", "ios-client", "macos-client", "windows-authority", "windows-client",
    ]);
    assert_code(candidate_missing_macos_role(), "FINAL_CANDIDATE_ARTIFACT_SET_INCOMPLETE");
    assert_code(candidate_with_duplicate_windows_authority_role(), "FINAL_CANDIDATE_ARTIFACT_ROLE_DUPLICATE");
    assert_code(candidate_with_wrong_windows_authority_payload(), "FINAL_CANDIDATE_AUTHORITY_CLOSURE_MISMATCH");
    assert_windows_authority_artifact_set_golden_exact();
}

#[test]
fn final_candidate_binds_all_pre_freeze_inputs_and_production_data_class() {
    let candidate = frozen_candidate();
    let generation = exact_load_candidate_verified_generation(&candidate).unwrap();
    assert_eq!(&candidate.payload.generation_manifest_ref, generation.artifact_ref());
    assert_eq!(&candidate.payload.generation_approval_registry_ref, generation.approval_registry_ref());
    assert_candidate_ack_refs_are_canonical_exact_observed_same_attempt_set(
        &candidate.payload.generation_participant_ack_refs,
        generation.payload().required_participants.as_slice(),
    );
    assert_candidate_ack_participant_definition_and_applied_item_subset_formulas_exact(&candidate);
    assert_candidate_generation_manifest_reaches_exact_verified_reverse_plan_per_item(&candidate);
    assert_client_artifact_set_typed_loads_release_mode_exact_four(&candidate.payload.client_artifact_set_ref);
    assert_eq!(candidate.payload.pre_freeze_carrier_refs.len(), 2);
    assert_pre_freeze_carriers_exact_pass_set(
        &candidate.payload.pre_freeze_carrier_refs,
        [ReleaseCarrierRecipeIdV1::WindowsAuthorityBuild, ReleaseCarrierRecipeIdV1::WindowsServiceInstall],
    );
    assert_eq!(candidate.payload.data_classification, CandidateDataClassificationV1::ProductionSignedNoBusinessData);
    assert_code(candidate_with_integration_client_artifact_set(), "FINAL_CANDIDATE_CLIENT_SET_MODE_MISMATCH");
    assert_code(candidate_with_post_freeze_carrier_in_precursor_set(), "FINAL_CANDIDATE_PRE_FREEZE_CARRIER_SET_INVALID");
    assert_code(candidate_with_generic_or_desired_only_generation(), "FINAL_CANDIDATE_GENERATION_NOT_OBSERVED");
    assert_code(candidate_with_wrong_generation_envelope_ref_media_or_declaration_digest(), "FINAL_CANDIDATE_GENERATION_MANIFEST_MISMATCH");
    assert_code(candidate_with_wrong_generation_approval_registry(), "FINAL_CANDIDATE_GENERATION_APPROVAL_MISMATCH");
    assert_code(candidate_with_missing_extra_duplicate_or_mixed_attempt_ack(), "FINAL_CANDIDATE_GENERATION_ACK_SET_MISMATCH");
    assert_code(candidate_with_wrong_ack_media_participant_digest_or_item_subset_digest(), "FINAL_CANDIDATE_GENERATION_ACK_MISMATCH");
    assert_code(candidate_with_ack_before_attempt_start_or_after_observed_commit(), "FINAL_CANDIDATE_GENERATION_ACK_TIME_INVALID");
    assert_code(candidate_with_signed_or_enveloped_ack(), "FINAL_CANDIDATE_GENERATION_ACK_WIRE_INVALID");
    assert_code(candidate_with_missing_unmaterialized_or_wrong_media_reverse_plan(), "FINAL_CANDIDATE_GENERATION_REVERSE_PLAN_MISMATCH");
    assert_code(candidate_with_reverse_plan_item_source_action_target_or_retention_drift(), "FINAL_CANDIDATE_GENERATION_REVERSE_PLAN_MISMATCH");
    assert_code(candidate_with_deidentified_data_classification(), "FINAL_CANDIDATE_DATA_CLASSIFICATION_MISMATCH");
}

#[tokio::test]
async fn runtime_topology_certification_has_one_post_p340_authority_and_store_result() {
    assert_cross_crate_composition_constructs_authority_only_through_public_compose_factory();
    let verified = produce_runtime_topology_certification_after_terminal_p340().await.unwrap();
    assert_eq!(verified.artifact_ref().media_type,
        "application/vnd.ep.f57-runtime-topology-certification-v1+json");
    assert_certification_exact_matches_candidate_declaration_terminal_p340_capacity_and_fresh_live_readback(&verified);
    assert_certification_is_plain_canonical_jcs_in_evidence_object_store_without_signer_row(&verified);
    assert_checkpoint_authenticates_terminal_p340_before_certification_object(&verified);
    assert_code(certification_before_terminal_p340_pass(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_NOT_DUE");
    assert_code(certification_from_wrong_candidate_recipe_or_declaration(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_MISMATCH");
    assert_code(certification_with_profile_storage_policy_host_capacity_or_duration_drift(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_MISMATCH");
    assert_code(certification_with_wrong_terminal_receipt_checkpoint_or_capacity_ref(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_MISMATCH");
    assert_code(certification_with_wrong_graph_generation_approval_registry_or_participant_ack_state(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_MISMATCH");
    assert_code(certification_with_stale_or_changed_live_readback(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_MISMATCH");
    assert_crash_before_or_after_object_create_adopts_the_same_derived_ref_and_bytes();
    assert_code(second_different_certification_for_same_candidate_run(), "F57_RUNTIME_TOPOLOGY_CERTIFICATION_CONFLICT");
    assert!(power_carrier_and_gate_g6_compile_against_same_load_or_reconstruct_method());
    assert!(topology_authority_uses_generic_checkpoint_prefix_then_release_local_p340_selection());
    assert!(every_topology_graph_projection_p340_capacity_and_certification_load_uses_generated_contract());
    assert!(!runtime_topology_certification_has_raw_ref_or_caller_readback_overload());
}

#[tokio::test]
async fn final_install_must_become_the_exact_observed_generation_before_freeze() {
    let verified = activate_final_installed_generation_after_terminal_install().await.unwrap();
    assert_manifest_items_exact_match_authority_component_service_and_endpoint_readbacks(&verified);
    assert_eq!(verified.generation_number(), predecessor_observed_generation_number() + 1);
    assert_exact_complete_same_attempt_ack_set_and_terminal_observed(&verified);
    assert_candidate_selection_chooses_only_this_attempt(&verified);
    assert_all_crash_cuts_adopt_one_transition_and_never_redispatch_known_applied_work();
    assert_code(begin_before_terminal_service_install(), "F57_FINAL_GENERATION_NOT_DUE");
    assert_code(begin_with_old_binary_graph_or_component_set(), "F57_FINAL_GENERATION_INSTALL_BINDING_MISMATCH");
    assert_code(freeze_with_preinstall_or_desired_only_generation(), "F57_FINAL_GENERATION_NOT_OBSERVED");
    assert_code(freeze_with_mixed_or_incomplete_ack_set(), "F57_FINAL_GENERATION_ACK_SET_MISMATCH");
}

#[test]
fn l3_rejects_184_rows_stale_l2_or_enabled_deferred_capability() {
    assert_eq!(release_gate(evidence_with_184_rows()).unwrap_err().code(), "RELEASE_REQUIREMENT_SET_INCOMPLETE");
    assert_eq!(release_gate(evidence_with_stale_l2()).unwrap_err().code(), "RELEASE_L2_CANDIDATE_MISMATCH");
    assert_eq!(release_gate(evidence_with_enabled_local_ai()).unwrap_err().code(), "DEFERRED_CAPABILITY_ENABLED");
}

#[test]
fn final_l2_and_l3_partition_is_the_frozen_149_36_split() {
    let l2_ids = generated_final_l2_test_ids();
    let l3_ids = generated_l3_test_ids();
    assert_eq!(l2_ids.len(), 149);
    assert_eq!(sha256_jcs(&l2_ids), "5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a");
    assert_eq!(l3_ids.len(), 36);
    assert_eq!(sha256_jcs(&l3_ids), "e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df");
    assert!(l2_ids.is_disjoint(&l3_ids));
    assert_eq!(canonical_union(&l2_ids, &l3_ids), all_185_requirement_test_ids());
    assert!(six_auxiliary_carrier_test_ids().is_disjoint(&canonical_union(&l2_ids, &l3_ids)));
}

#[test]
fn final_l2_l3_and_certificate_revalidate_the_same_four_closed_objectives() {
    let l2 = final_l2();
    let l3 = final_l3();
    assert_exact_four_objective_kinds_all_closed_and_no_open_obligations(&l2.payload.objective_closures);
    assert_eq!(l3.payload.objective_closures, l2.payload.objective_closures);
    assert_l3_revalidates_current_generation_for_each_objective(&l3);
    assert_certificate_revalidates_objectives_through_l3_before_finalization();
    assert_final_checkpoint_and_expiry_include_all_objective_closure_inputs();
    assert_eq!(fresh_final_g4_receipt().payload.objective_closures, l2.payload.objective_closures);
    assert_eq!(fresh_final_g5_receipt().payload.objective_closures, l2.payload.objective_closures);
    assert_g4_g5_due_and_probe_sets_remain_their_own_profiles();
    assert_code(final_l2_with_waiting_or_reopened_objective(), "RELEASE_OBJECTIVE_NOT_CLOSED");
    assert_code(l3_with_mutated_l2_objective_vector(), "RELEASE_OBJECTIVE_VECTOR_MISMATCH");
    assert_code(objective_reopened_between_l2_and_l3(), "RELEASE_OBJECTIVE_GENERATION_DRIFT");
    assert_code(certificate_after_objective_reopened(), "RELEASE_OBJECTIVE_REOPENED");
    assert_code(final_procurement_with_wrong_fact_owner_or_same_reviewer(), "RELEASE_PROCUREMENT_CLOSURE_INVALID");
    assert_code(final_closure_with_expired_or_unauthorized_result(), "RELEASE_OBJECTIVE_RESULT_INVALID");
    assert_code(final_run_reusing_development_g4_waiting_receipt(), "RELEASE_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(final_g4_receipt_with_waiting_procurement(), "RELEASE_G4_OBJECTIVE_CONTEXT_MISMATCH");
    assert_code(final_receipt_with_wrong_candidate_envelope_purpose(), "RELEASE_GATE_CANDIDATE_CONTEXT_MISMATCH");
    assert_gate_receipt_golden_covers_release_candidate_closed_g4_g5_contexts();
}

#[test]
fn release_inputs_are_exact_five_candidate_artifacts_plus_six_recipe_results() {
    assert_eq!(frozen_candidate().payload.release_artifact_lane_wires(), [
        "android-client", "ios-client", "macos-client", "windows-authority", "windows-client",
    ]);
    assert_eq!(final_l2().payload.carrier_recipe_ids(), [
        ReleaseCarrierRecipeIdV1::BackupRestoreCertification,
        ReleaseCarrierRecipeIdV1::P340Release72Hour,
        ReleaseCarrierRecipeIdV1::Postgres16Pitr,
        ReleaseCarrierRecipeIdV1::PowerShutdown,
        ReleaseCarrierRecipeIdV1::WindowsAuthorityBuild,
        ReleaseCarrierRecipeIdV1::WindowsServiceInstall,
    ]);
    assert_code(release_gate(evidence_missing_client_package(ClientPackageArtifactIdV1::MacosClient)), "FINAL_CANDIDATE_ARTIFACT_SET_INCOMPLETE");
    assert_code(release_gate(evidence_missing_recipe(ReleaseCarrierRecipeIdV1::P340Release72Hour)), "RELEASE_CARRIER_SET_INCOMPLETE");
    assert_code(release_gate(evidence_with_recipe_wire("RANSOMWARE_CLEANROOM")), "RELEASE_CARRIER_RECIPE_UNKNOWN");
}

#[test]
fn release_certificate_rejects_string_purpose_partial_payload_and_unknown_verdict() {
    assert_code(cert_from_json(with_string_gate_as_purpose()), "SIGNED_ARTIFACT_PURPOSE_MISMATCH");
    assert_code(cert_from_json(with_missing_test_results()), "RELEASE_CERTIFICATE_PAYLOAD_INCOMPLETE");
    assert_code(cert_from_json(with_unknown_release_verdict()), "RELEASE_CERTIFICATE_VERDICT_UNKNOWN");
    assert_code(cert_from_json(with_cross_gate_run_result()), "RELEASE_RESULT_GATE_RUN_MISMATCH");
    assert_code(cert_from_json(with_missing_l3_ref()), "RELEASE_CERTIFICATE_CHAIN_INCOMPLETE");
    assert_code(cert_from_json(with_missing_g3_prerequisite_receipt()), "RELEASE_PREREQUISITE_SET_INCOMPLETE");
    assert_code(cert_from_json(with_cross_run_prerequisite_receipt()), "RELEASE_PREREQUISITE_RUN_MISMATCH");
    assert_code(cert_from_json(with_gate_receipt_ref_label_mismatch()), "RELEASE_PREREQUISITE_GATE_MISMATCH");
    assert_code(cert_from_json(with_signed_artifact_ref_instead_of_gate_receipt_ref()), "RELEASE_PREREQUISITE_REF_TYPE_MISMATCH");
    assert_code(cert_from_json(with_checkpoint_rollback()), "F57_JOURNAL_CHECKPOINT_NOT_EXTENSION");
    assert_code(cert_from_json(with_expiry_later_than_input()), "RELEASE_CERTIFICATE_EXPIRY_EXTENSION");
    assert_code(cert_from_json(with_finalization_attempt_id_drift()), "RELEASE_CERTIFICATE_FINALIZATION_MISMATCH");
    assert_code(cert_from_json(with_live_clock_issue_time()), "RELEASE_CERTIFICATE_FINALIZATION_TIME_MISMATCH");
    assert_code(cert_from_json(with_latest_instead_of_frozen_checkpoint()), "RELEASE_CERTIFICATE_FINALIZATION_CHECKPOINT_MISMATCH");
    assert_code(
        cert_from_json(with_signer_artifact_kind_as_event_kind("RELEASE_CERTIFICATE_V1")),
        "F57_FINALIZATION_ARTIFACT_KIND_INVALID",
    );
    assert_code(cert_from_json(with_wrong_registry_signer_row()), "F57_ARTIFACT_SIGNER_REGISTRY_MISMATCH");
    assert_gate_receipt_ref_golden_and_sort_order_exact();
}

#[tokio::test]
async fn production_activation_is_explicit_durable_and_single_disk_honest() {
    assert_business_routes_quarantined_before_genesis_admission();
    let activated = activate_with_verified_certificate_acceptance_and_fresh_readbacks().await.unwrap();
    assert_eq!(activated.profile_wire(), "SINGLE_DISK_DEGRADED_PRODUCTION");
    assert_exact_two_distinct_customer_approvers_five_risks_twenty_users_and_safeguard_closure();
    assert_exact_final_installed_observed_generation_and_certified_topology_live_equality();
    assert_four_fresh_live_readbacks_include_graph_exact_runtime_deployment_set();
    assert_runtime_live_readback_bijection_and_deferred_absence_match_frozen_closure();
    let admission = admit_genesis_from_private_activation_proof(&activated).await.unwrap();
    assert_monotonic_business_api_generation_opens_routes_only_after_admission_cas(&admission);
    assert_unique_route_gate_requires_current_observed_equals_admitted_and_no_hold();
    assert_same_request_after_response_loss_returns_same_activation_id_and_record();
    assert_code(activate_without_customer_acceptance(), "F57_PRODUCTION_ACCEPTANCE_REQUIRED");
    assert_code(activate_with_stale_certificate_generation_or_topology(), "F57_PRODUCTION_ACTIVATION_BINDING_MISMATCH");
    assert_code(activate_with_co_located_target_missing_offline_media_or_ups(), "F57_PRODUCTION_SAFEGUARD_CLOSURE_INVALID");
    assert_code(second_or_concurrent_different_activation(), "F57_PRODUCTION_ALREADY_ACTIVATED_CONFLICT");
    assert_failed_live_readback_is_held_and_keeps_business_routes_quarantined();
    assert_code(activate_with_runtime_participant_or_carrier_drift(), "F57_PRODUCTION_RUNTIME_DEPLOYMENT_DRIFT");
    assert_same_activation_can_retry_only_after_exact_failure_hash_and_full_revalidation();
    assert_retry_recollects_all_live_readbacks_and_never_reuses_prior_objects();
    assert_code(retry_with_new_activation_id_or_untyped_failure_code(), "F57_PRODUCTION_ACTIVATION_RETRY_INVALID");
    assert_code(route_with_activated_but_no_admission(), "F57_PRODUCTION_GENERATION_NOT_ADMITTED");
    assert_ordinary_delta_within_resource_envelope_can_readmit_exact_ack_transition_set();
    assert_global_or_resource_increasing_delta_remains_held_for_full_recertification();
    assert_rollback_reopen_requires_fresh_exact_predecessor_readback();
}

#[test]
fn fixed_launcher_and_kernel_abi_are_recoverable_and_scm_immutable() {
    assert_all_five_image_paths_equal_fixed_core_server_launcher();
    assert_exact_single_c_abi_export_version_size_and_function_table();
    assert_initial_msi_slot_pointer_and_loaded_dll_readback_are_exact();
    assert_slot_head_is_tpm_data_hdd_and_off_host_anti_rollback_bound();
    assert_code(kernel_with_rust_abi_extra_export_or_pointer_fork(), "F57_AUTHORITY_KERNEL_ABI_INVALID");
    assert_no_package_path_calls_change_service_config_or_overwrites_launcher();
}

#[test]
fn release_carrier_dispatch_is_closed_and_journal_first() {
    let independent_golden = strict_parse_literal_carrier_registry_fixture(
        "xtask/tests/fixtures/f57-release-carrier-compiled-registry-v1-golden.json",
    );
    assert_eq!(release_carrier_registry(), &RELEASE_CARRIER_REGISTRY_V1);
    assert_eq!(project_registry_literal(&RELEASE_CARRIER_REGISTRY_V1), independent_golden);
    assert!(carrier_dispatcher_indexes_only_release_carrier_registry_v1());
    assert_every_carrier_row_exact_joins_verified_89_row_registry_by(
        |row| (row.raw_signer_registry_key, row.recipe.wire()),
        |row| (row.result_signer_registry_key, row.recipe.wire()),
    );
    assert!(release_carrier_registry().test_ids().is_disjoint(&real_delivery_registry().test_ids()));
    assert_code(certificate_with_aux_carrier_test_id_in_test_results(), "RELEASE_AUX_TEST_IN_DUE_SET");
    assert_code(carrier_with_arbitrary_script(), "F57_CARRIER_RECIPE_UNKNOWN");
    assert_code(carrier_invoked_without_durable_started_record(), "F57_CARRIER_START_NOT_DURABLE");
    assert_eq!(generated_powershell_release_registry().len(), 18);
    assert_eq!(project_powershell_registry_literal(), strict_parse_powershell_registry_golden());
    assert_exact_transitive_script_call_graph_and_no_raw_child_invocation();
    assert_all_18_scripts_final_hash_authenticode_spki_eku_rfc3161_revocation_and_descriptor_exact();
    assert_all_six_carrier_script_ids_resolve_same_path_in_powershell_registry();
    assert_executor_reverifies_same_host_and_script_file_ids_after_started_and_holds_handles_through_exit();
}

#[test]
fn carrier_staging_inputs_and_causal_order_are_exact() {
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::WindowsAuthorityBuild), []);
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::WindowsServiceInstall), [
        CarrierStagingInputIdV1::WindowsAuthorityArtifactSet,
        CarrierStagingInputIdV1::WindowsAuthorityManifest,
        CarrierStagingInputIdV1::WindowsAuthorityMsi,
    ]);
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::Postgres16Pitr), []);
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::BackupRestoreCertification), []);
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::P340Release72Hour), [
        CarrierStagingInputIdV1::P340CapacityInputManifest,
        CarrierStagingInputIdV1::P340CertificationPolicyAttestation,
    ]);
    assert_eq!(staging_input_ids(ReleaseCarrierRecipeIdV1::PowerShutdown), [
        CarrierStagingInputIdV1::UpsIdentityReadback,
        CarrierStagingInputIdV1::UpsPowerWriteCachePolicy,
    ]);
    assert_first_five_recipes_have_continuation_none();
    assert_power_shutdown_continuation_service_exact(
        "EPF57PowerShutdownContinuation",
        "Enterprise Platform F57 Power Shutdown Continuation",
        r"NT SERVICE\EPF57PowerShutdownContinuation",
        r"C:\Program Files\EnterprisePlatform\Authority\ep-core-server.exe",
        ["CryptSvc", "EPAuthorityControl", "EPAuthorityServer", "EventLog"],
        ["SeChangeNotifyPrivilege"],
        "AUTO_START",
        "UNRESTRICTED",
        "NO_AUTOMATIC_RESTART",
        r"HKLM\SYSTEM\CurrentControlSet\Services\EPF57PowerShutdownContinuation\Parameters\F57ActivationV1",
    );
    assert_power_shutdown_dispatcher_authority_exact(
        "EPAuthorityControl",
        r"NT SERVICE\EPAuthorityControl",
        ["SeChangeNotifyPrivilege", "SeShutdownPrivilege"],
        "SC_MANAGER_CONNECT",
        ["QUERY_STATUS", "START"],
    );
    assert_power_signing_brokers_exact_and_role_separated(
        ("EPF57PowerRawSigner", r"NT SERVICE\EPF57PowerRawSigner", "RAW_EVIDENCE"),
        ("EPF57GateJournalSigner", r"NT SERVICE\EPF57GateJournalSigner", "GATE_RUN_JOURNAL"),
    );
    assert_power_broker_static_rows_exact([
        expected_broker_row(
            "CONTROL_BROKER",
            "EPAuthorityControl",
            "Enterprise Platform Authority Control Broker",
            r"NT SERVICE\EPAuthorityControl",
            "crates/platform/authority-kernel/src/dispatch.rs#authority_control",
            ["power-shutdown-control-broker"],
            "AUTO_START",
            restart_on_failure_max_three(86400, [("RESTART", 5000), ("RESTART", 15000), ("RESTART", 60000), ("NONE", 0)]),
            preshutdown_enabled(600000),
            ["CryptSvc", "EventLog", "RpcSs"],
            ["SeChangeNotifyPrivilege", "SeShutdownPrivilege"],
            pipe_endpoint(r"\\.\pipe\EnterprisePlatform\EPAuthorityControlV1"),
        ),
        expected_broker_row(
            "RAW_EVIDENCE_SIGNER",
            "EPF57PowerRawSigner",
            "Enterprise Platform F57 Power Raw Evidence Signer Facade",
            r"NT SERVICE\EPF57PowerRawSigner",
            "crates/platform/authority-kernel/src/dispatch.rs#power_raw_signer",
            ["power-shutdown-signing-broker", "--role", "raw-evidence"],
            "AUTO_START",
            restart_on_failure_max_three(86400, [("RESTART", 5000), ("RESTART", 15000), ("RESTART", 60000), ("NONE", 0)]),
            preshutdown_disabled(),
            ["CryptSvc", "EventLog", "RpcSs"],
            ["SeChangeNotifyPrivilege"],
            keyless_facade(r"\\.\pipe\EnterprisePlatform\EPF57PowerRawSignerV1", r"\\.\pipe\EnterprisePlatform\F57EvidenceSignerV1", "RAW_EVIDENCE"),
        ),
        expected_broker_row(
            "GATE_RUN_JOURNAL_SIGNER",
            "EPF57GateJournalSigner",
            "Enterprise Platform F57 Gate Run Journal Signer Facade",
            r"NT SERVICE\EPF57GateJournalSigner",
            "crates/platform/authority-kernel/src/dispatch.rs#gate_journal_signer",
            ["power-shutdown-signing-broker", "--role", "gate-run-journal"],
            "AUTO_START",
            restart_on_failure_max_three(86400, [("RESTART", 5000), ("RESTART", 15000), ("RESTART", 60000), ("NONE", 0)]),
            preshutdown_disabled(),
            ["CryptSvc", "EventLog", "RpcSs"],
            ["SeChangeNotifyPrivilege"],
            keyless_facade(r"\\.\pipe\EnterprisePlatform\EPF57GateJournalSignerV1", r"\\.\pipe\EnterprisePlatform\F57EvidenceSignerV1", "GATE_RUN_JOURNAL"),
        ),
    ]);
    assert_power_shutdown_continuation_paths_and_argv_exact(
        "continuation-state/<ascii-lowercase-TestID>/<lowercase-hyphenated-execution_attempt_id>/power-shutdown-state.v1.jcs.jsonl",
        "continuation/power-shutdown-success-persistence-command.v1.json",
        "raw/00.v1.json",
        ["power-shutdown-continuation", "--activation-source", "scm-parameter"],
    );
    assert_authority_server_static_row_and_service_mode_are_exact();
    assert_power_continuation_exact_eighteen_object_action_sddl_bindings_and_readbacks();
    assert_keyless_signing_facades_forward_only_frozen_role_operations_to_g0_broker();
    assert_only_g0_evidence_broker_and_system_can_open_either_signing_key();
    assert_verify_existing_objects_are_never_repaired_and_attempt_objects_are_securely_created();
    assert_active_record_path_is_a_value_inside_the_acl_bearing_activation_child_key();
    assert_continuation_account_has_no_filesystem_write_and_only_control_broker_message_access();
    assert_control_broker_accepts_exact_six_output_discriminators_and_fixed_paths();
    assert_permanent_auto_start_service_is_dormant_by_absent_active_record_path();
    assert_attempt_never_changes_start_type_or_creates_deletes_disables_service();
    assert_code(power_plan_with_create_or_delete_scm_right(), "F57_CARRIER_CONTINUATION_SCM_RIGHT_INVALID");
    assert_code(power_plan_with_per_attempt_service_argv(), "F57_CARRIER_CONTINUATION_ARGV_INVALID");
    assert_eq!(power_continuation_event_wires(), [
        "CARRIER_CONTINUATION_ARMED",
        "CARRIER_CONTINUATION_PRE_SHUTDOWN_COMMITTED",
        "CARRIER_CONTINUATION_POST_RESTART_COMPLETED",
        "CARRIER_CONTINUATION_DISARMED",
        "CARRIER_CONTINUATION_FAILED",
    ]);
    assert_eq!(final_f57_gate_journal_delta_event_count(), 13);
    assert_power_continuation_events_have_exact_bytes_and_strict_prefixes_1_through_4();
    assert_code(power_with_unknown_continuation_event(), "F57_GATE_JOURNAL_EVENT_UNKNOWN");
    assert_code(power_with_cross_attempt_or_nonextending_prefix(), "F57_CARRIER_CONTINUATION_PREFIX_MISMATCH");
    assert_power_continuation_binary_is_authenticode_verified_installed_authority_binary();
    assert_core_server_all_targets_link_the_single_continuation_entrypoint();
    assert_plan_object_precedes_started_singleton_ref_then_staging_and_script();
    assert_p340_plan_then_seven_journaled_operations_then_closure_checkpoint_input_plan_and_started();
    assert_p340_started_context_completion_and_terminal_cross_signature_order();
    assert_context_bound_raw_and_nested_issuance_window_is_finish_to_min_finish_plus_300000_or_expiry();
    assert_code(started_with_non_plan_context_ref(), "F57_CARRIER_START_CONTEXT_INVALID");
    assert_code(staged_input_with_ref_or_media_drift(), "F57_CARRIER_STAGING_INPUT_MISMATCH");
    assert_code(service_inputs_from_different_authority_sets(), "F57_CARRIER_AUTHORITY_SET_MISMATCH");
    assert_code(service_raw_with_authority_artifacts_ref_drift(), "F57_CARRIER_AUTHORITY_SET_MISMATCH");
    assert_code(service_raw_with_installed_msi_ref_drift(), "F57_CARRIER_INSTALLED_MSI_MISMATCH");
    assert_authority_artifact_set_binds_graph_exact_runtime_deployment_closure();
    assert_authority_artifact_set_binds_exact_ten_row_production_admission_bypass_registry();
    assert_service_install_repeats_same_bypass_registry_ref_and_installed_bytes();
    assert_service_install_runtime_readbacks_are_bijective_with_active_and_deferred_rows();
    assert_local_ai_deferred_row_has_zero_service_process_endpoint_package_and_resource_counts();
    assert_code(runtime_readback_missing_extra_or_wrong_carrier(), "F57_RUNTIME_DEPLOYMENT_READBACK_MISMATCH");
    assert_code(active_runtime_artifact_not_built_from_clean_head(), "F57_RUNTIME_DEPLOYMENT_ARTIFACT_MISMATCH");
    assert_authority_artifact_set_binds_exact_six_row_backup_recovery_component_set();
    assert_service_install_proves_three_running_component_services_two_installed_tools_and_one_offhost_target();
    assert_backup_service_runtime_challenge_binds_component_id_not_authority_service_role();
    assert_code(component_set_missing_or_replacing_one_component(), "F57_WINDOWS_SERVER_COMPONENT_SET_MISMATCH");
    assert_code(backup_target_installed_on_authority_or_sharing_admin_credentials(), "F57_BACKUP_TARGET_PLACEMENT_INVALID");
    assert_code(writer_signer_capability_or_account_alias(), "F57_BACKUP_COMPONENT_SEPARATION_INVALID");
    assert!(p340_with_prestart_or_cross_signature_timestamp_inversion().is_err());
    assert!(context_bound_physical_envelope_with_none_before_finish_or_after_window_signature().is_err());
}

#[test]
fn carrier_staging_finalizer_is_crash_safe_without_rerunning_physical_work() {
    assert_all_carrier_staging_plan_completion_and_crash_goldens_exact();
    assert!(complete_raw_set_after_crash_is_finalized_once_without_script_rerun());
    assert_code(partial_raw_set_after_crash(), "F57_CARRIER_RESULT_UNKNOWN");
    assert_code(conflicting_staging_completion(), "F57_CARRIER_STAGING_CONFLICT");
    assert_backup_runs_one_and_two_are_raw_only_and_run_three_precedes_completion();
    assert_power_shutdown_all_cross_reboot_continuation_crash_cuts_exact();
    assert_power_prepare_only_script_cannot_shutdown_or_write_raw();
    assert_power_dispatcher_alone_invokes_planned_shutdown_once(
        0x80040001,
        "F57_RELEASE_POWER_SHUTDOWN;continuation_id=<uuid>;execution_attempt_id=<uuid>",
    );
    assert_api_commit_marker_is_durable_before_the_single_shutdown_api_call();
    assert_user32_1074_event_alone_is_never_durable_acknowledgement();
    assert_composite_ack_requires_exact_event_plus_authenticated_same_id_ups_schedule_ack();
    assert_shutdown_event_repeats_exact_initiating_executable_path_and_digest();
    assert_persisted_requested_at_is_exactly_repeated_and_never_resampled();
    assert_marker_presence_forever_forbids_api_redispatch();
    assert_code(
        marker_without_durable_composite_ack_or_ambiguous_hardware_state(),
        "F57_CARRIER_RESULT_UNKNOWN",
    );
    assert_ack_absent_allows_clean_windows_shutdown_then_manual_power_on_and_terminal_unknown();
    assert_postboot_resume_controller_is_initialized_only_from_exact_marker_and_composite_ack();
    assert_valid_success_spool_wins_across_boot_before_context_change_failure();
    assert_service_main_sends_only_success_draft_and_broker_owns_tick_and_persisted_command();
    assert_success_spool_projects_exact_event_recovery_proof_and_restart_triple();
    assert_disarm_keeps_registration_present_auto_start_stopped_and_removes_active_record_path();
    assert_power_completion_result_and_terminal_require_disarmed_event();
    assert!(power_retry_that_requests_second_shutdown().is_err());
}

#[test]
fn release_carrier_raw_wires_and_cardinalities_are_exact() {
    assert_eq!(raw_ref_cardinalities_by_recipe(), [1, 1, 1, 3, 1, 1]);
    assert_code(pitr_with_one_offline_media(), "RELEASE_PITR_MEDIA_SET_INCOMPLETE");
    assert_code(recovery_with_runs_1_2_2(), "RELEASE_RECOVERY_PHASE_SET_INVALID");
    assert_code(p340_with_71_hours(), "RELEASE_P340_DURATION_INSUFFICIENT");
    assert_code(power_with_policy_trigger_mismatch(), "RELEASE_POWER_POLICY_MISMATCH");
    assert_code(carrier_with_wrong_raw_media_type(), "RELEASE_CARRIER_RAW_TYPE_MISMATCH");
    assert!(p340_with_unverified_capacity_history_summary().is_err());
    assert!(p340_with_boot_id_or_monotonic_source_drift().is_err());
    assert!(p340_with_error_rate_operation_or_wal_boundary_drift().is_err());
    assert!(p340_with_input_host_outer_capacity_drift().is_err());
    assert_power_raw_continuation_plan_ref_and_id_exact_match_started_plan();
    assert_power_continuation_has_six_plain_readbacks_and_four_state_records();
    assert_power_nine_closed_opaque_capture_parsers_and_typed_ups_ack_fixtures_exact();
    assert_all_opaque_capture_headers_repeat_same_run_attempt_continuation_and_barrier();
    assert_opaque_capture_media_is_octet_stream_without_schema_or_signer_rows();
    assert_power_ups_trigger_proves_actual_ac_loss_on_battery_first_900_second_crossing_and_restore();
    assert_power_quiesce_event_restart_and_disarm_relational_predicates_exact();
    assert_code(power_with_parser_header_or_authority_barrier_drift(), "RELEASE_POWER_CAPTURE_BINDING_MISMATCH");
}

#[test]
fn release_subordinate_registry_is_exact_eighteen_bindings_seventeen_parsers_thirty_leaves() {
    assert_eq!(release_subordinate_field_bindings().len(), 18);
    assert_eq!(release_subordinate_parser_definitions().len(), 17);
    assert_eq!(complete_release_subordinate_leaf_instances().len(), 30);
    assert_offline_rotation_a_and_b_share_only_the_one_role_tagged_parser();
    assert_three_recovery_raw_rows_reuse_the_same_six_parsers_without_aliasing_leaf_refs();
    assert_database_cut_has_nonempty_server_version_and_positive_server_version_num();
    assert_database_cut_server_version_num_major_is_exactly_16_and_outer_version_is_equal();
    assert_all_eighteen_field_parser_media_purpose_bindings_and_one_field_negatives_exact();
}

#[test]
fn consolidated_release_schema_and_generated_offline_closure_are_exact() {
    assert_eq!(release_schema_root_set(), expected_master_release_root_set());
    assert_eq!(release_schema_selected_infrastructure_relative_ref_count(), 1);
    assert_eq!(release_schema_l2_relative_ref_count(), 1);
    assert_eq!(release_schema_gate_journal_relative_ref_count(), 1);
    assert_eq!(release_schema_offline_manifest_relative_ref_count(), 1);
    assert_eq!(release_schema_gate_receipt_relative_ref_count(), 1);
    assert_eq!(release_schema_client_common_relative_ref_count(), 1);
    assert_eq!(release_schema_generation_relative_ref_count(), 0);
    assert_eq!(release_schema_generation_approval_relative_ref_count(), 0);
    assert_eq!(release_schema_runtime_topology_relative_ref_count(), 0);
    assert_gate_journal_power_continuation_event_schema_exact();
    assert_gate_journal_sole_owns_continuation_prefix_and_never_imports_release_schema();
    assert_release_schema_imports_gate_prefix_once_and_owns_state_plan_and_six_plain_readbacks();
    assert!(!release_schema_duplicates_selected_infrastructure_definitions());
    assert!(!release_schema_duplicates_l2_definitions());
    assert!(!release_schema_duplicates_gate_journal_definitions());
    assert!(!release_schema_duplicates_offline_manifest_definitions());
    assert!(!release_schema_duplicates_gate_receipt_or_ref_definitions());
    assert_offline_closure_contains_generation_manifest_reverse_plan_and_plain_ack_media_exactly_once();
    assert_offline_closure_contains_pinned_three_row_generation_approval_registry_exactly_once();
    assert_offline_closure_contains_both_plain_runtime_topology_media_exactly_once();
    assert_offline_closure_contains_windows_runtime_deployment_closure_and_readback_media_exactly_once();
    assert_candidate_generation_ref_media_contracts_reach_generation_and_approval_schemas_without_nominal_import();
    assert_eq!(offline_schema_closure_count_and_digest(), offline_schema_closure_golden());
    assert_offline_schema_descriptors_bind_exact_transitive_artifact_closure();
    assert_offline_schema_descriptor_paths_ids_imports_and_bindings_are_canonical();
    assert_eq!(offline_manifest_uri(""), "evidence-relative://bundle/schemas/offline-schema-manifest.v1.json");
    assert_eq!(offline_manifest_uri("release-candidate"), "evidence-relative://bundle/release-candidate/schemas/offline-schema-manifest.v1.json");
    assert_code(offline_manifest_uri("nested//run"), "OFFLINE_SCHEMA_URI_NONCANONICAL");
    assert_code(offline_manifest_uri("/nested/run/"), "OFFLINE_SCHEMA_URI_NONCANONICAL");
    assert_code(hardcoded_four_schema_allowlist(), "OFFLINE_SCHEMA_CLOSURE_INCOMPLETE");
    assert_code(closure_missing_handler_media_contract(), "OFFLINE_SCHEMA_HANDLER_BINDING_MISSING");
    assert_code(
        offline_finalization_with_signer_artifact_kind_alias("F57_OFFLINE_SCHEMA_MANIFEST_V1"),
        "F57_FINALIZATION_ARTIFACT_KIND_INVALID",
    );
    assert_offline_schema_manifest_golden_exact();
    assert_release_root_goldens_round_trip_byte_for_byte();
    assert_code(l2_development_payload_with_final_target(), "L2_PURPOSE_TARGET_MISMATCH");
    assert_code(l3_payload_with_integration_l2(), "L3_FINAL_L2_REQUIRED");
    assert_code(release_schema_with_local_gate_receipt_ref_copy(), "RELEASE_SCHEMA_OWNER_DUPLICATE");
    assert_code(release_schema_with_wrong_selected_infrastructure_digest(), "OFFLINE_SCHEMA_DIGEST_MISMATCH");
}

#[test]
fn g6_handler_set_is_exact() {
    assert_eq!(g6_handler_requirement_ids(), expected_36_g6_requirement_ids());
}

#[test]
fn production_package_maintenance_is_graph_reserved_privileged_and_recoverable() {
    assert_graph_derived_slot_is_the_only_plan_authoring_input();
    assert_structural_plan_binds_checkpoint_policy_before_any_actual_checkpoint_exists();
    assert_hold_drain_and_barrier_precede_task11_full_cut_checkpoint();
    assert_fresh_execution_authorization_and_trust_snapshot_follow_checkpoint();
    assert_desired_state_item_closes_install_enable_disable_upgrade_and_rollback();
    assert_every_many_to_many_item_uses_participant_specific_cas_trust_operation_and_readback();
    assert_control_broker_and_recovery_tool_accept_only_closed_typed_operations();
    assert_every_forward_and_restore_external_call_has_durable_intent_and_operation_id();
    assert_forward_failure_cannot_rollback_before_coordinator_rollback_started();
    assert_expiry_or_revocation_after_intent_allows_only_query_measure_or_rollback();
    assert_new_install_rollback_deactivates_and_retains_data_without_fictitious_predecessor();
    assert_foundation_restore_uses_external_capsule_then_exact_sql_terminal_reseal();
    assert_crash_cut_matrix_adopts_one_plan_forward_attempt_and_rollback_attempt();
}

#[test]
fn release_dependency_dag_and_cross_stage_type_owners_are_exact() {
    let metadata = locked_cargo_metadata();
    assert!(metadata.has_direct_edges(
        "ep-platform-generation-activation",
        ["ep-platform-release", "ep-platform-runtime", "ep-platform-capability-graph", "ep-platform-package", "ep-platform-backup", "ep-platform-tenancy"],
    ));
    assert!(!metadata.has_path("ep-platform-package", "ep-platform-generation-activation"));
    assert!(!metadata.has_path("ep-platform-backup", "ep-platform-generation-activation"));
    assert!(!metadata.has_path("ep-platform-release", "ep-platform-generation-activation"));
    assert!(!metadata.has_path("ep-platform-runtime", "ep-platform-generation-activation"));
    assert!(!metadata.has_path("ep-platform-package", "ep-platform-backup"));
    assert!(!metadata.has_path("ep-platform-package", "ep-platform-tenancy"));
    assert!(metadata.has_direct_edges(
        "ep-platform-release",
        ["ep-platform-runtime", "ep-platform-client-common", "ep-platform-delivery-registry", "ep-platform-backup"],
    ));
    assert!(!metadata.has_path("ep-platform-runtime", "ep-platform-release"));
    assert!(!metadata.has_path("ep-platform-client-common", "ep-platform-release"));
    assert!(!metadata.has_path("ep-platform-delivery-registry", "ep-platform-release"));
    assert!(!metadata.has_path("ep-platform-backup", "ep-platform-release"));
    assert!(workspace_dependency_graph_is_acyclic(&metadata));
    assert_eq!(rust_nominal_owner("ReleaseCarrierRecipeIdV1"),
        "crates/platform/release/src/carrier_contract.rs");
    assert_eq!(rust_nominal_owner("TargetGateV1"),
        "crates/platform/release/src/l2.rs");
    assert_eq!(rust_nominal_owner("IntegrationArtifactIdV1"),
        "crates/platform/client-common/src/model.rs");
    assert_eq!(rust_nominal_owner("MigrationClosureIdentityV1"),
        "crates/platform/delivery-registry/src/migration_closure.rs");
    assert_eq!(rust_nominal_owner("WindowsBackupComponentIdV1"),
        "crates/platform/backup/src/windows_components.rs");
}
```

- [ ] **Step 2: Run and verify RED.**

Run: `cargo test -p ep-testkit --test f57_final_candidate --test f57_final_installed_generation --test f57_package_maintenance_production --test f57_production_activation --test f57_production_generation_admission --test f57_windows_runtime_deployment --test f57_release_gate_unit --test f57_release_dependency_dag --test f57_ups_adapter_contract --test f57_ups_command_reconciliation -- --nocapture`

Run: `cargo test -p ep-xtask --test f57_release_carrier --test f57_windows_runtime_deployment --test f57_run_journal -- --nocapture`

Run: `cargo test -p core-server --all-targets`

Run: `cargo test -p ep-platform-ups-contract -p ep-adapter-ups-windows --all-targets --locked`

Expected: FAIL because the graph-exact runtime deployment materializer/collector, graph-reserved privileged package maintenance/rollback composition, final-installed generation, final candidate, L3 aggregation, exact typed staging/finalizer registry, production activation and G6 handler set do not exist.

Before any dispatcher behavior, `xtask/src/f57/carrier.rs` lands the one compiled six-row registry below. The enum remains imported from `ep-platform-release::carrier_contract`; this table is the only recipe-to-execution mapping and both the dispatcher and fixture serializer consume this same constant.

```rust
pub struct ReleaseCarrierRegistryRowV1 {
    pub recipe: ReleaseCarrierRecipeIdV1,
    pub script_repo_path: &'static str,
    pub script_specific_argv_template: &'static [&'static str],
    pub staging_input_ids: &'static [CarrierStagingInputIdV1],
    pub raw_schema_id: &'static str,
    pub raw_media_type: &'static str,
    pub raw_cardinality: usize,
    pub auxiliary_test_id: &'static str,
    pub trusted_runner_id: &'static str,
    pub raw_signer_registry_key: &'static str,
    pub required_raw_subject_dn: &'static str,
    pub result_signer_registry_key: &'static str,
    pub required_result_subject_dn: &'static str,
    pub reconciler_id: &'static str,
    pub script_trust_descriptor_id: &'static str,
}

pub const RELEASE_CARRIER_REGISTRY_V1: [ReleaseCarrierRegistryRowV1; 6] = [
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::WindowsAuthorityBuild,
        script_repo_path: "scripts/windows/build-msi.ps1",
        script_specific_argv_template: &[
            "-Mode", "Release",
            "-Candidate", "HEAD", "-StagingPlan", r"<staging>\plan.v1.json",
            "-ArtifactManifest", r"<staging>\artifacts\<authority-manifest-key-sha256>.bin",
            "-EvidenceOut", r"<staging>\raw\00.v1.json",
        ],
        staging_input_ids: &[],
        raw_schema_id: "ReleaseAuthorityBuildEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-authority-build-evidence-v1+json",
        raw_cardinality: 1,
        auxiliary_test_id: "T-F57-CARRIER-WINDOWS-AUTHORITY-BUILD",
        trusted_runner_id: "f57-windows-release-build",
        raw_signer_registry_key: "RELEASE_AUTHORITY_BUILD_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 Windows Release Build Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_CREATE_NEW_AUTHORITY_ARTIFACTS",
        script_trust_descriptor_id: "F57_PS_BUILD_MSI_V1",
    },
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::WindowsServiceInstall,
        script_repo_path: "scripts/windows/install-services.ps1",
        script_specific_argv_template: &[
            "-Mode", "ReleaseCandidate",
            "-StagingPlan", r"<staging>\plan.v1.json",
            "-ArtifactSet", r"<staging>\inputs\windows-authority-artifact-set.v1.json",
            "-ArtifactManifest", r"<staging>\inputs\windows-authority-manifest.v1.json",
            "-MsiPath", r"<staging>\inputs\windows-authority.msi",
            "-EvidenceOut", r"<staging>\raw\00.v1.json",
        ],
        staging_input_ids: &[
            CarrierStagingInputIdV1::WindowsAuthorityArtifactSet,
            CarrierStagingInputIdV1::WindowsAuthorityManifest,
            CarrierStagingInputIdV1::WindowsAuthorityMsi,
        ],
        raw_schema_id: "ReleaseWindowsServiceInstallEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-windows-service-install-evidence-v1+json",
        raw_cardinality: 1,
        auxiliary_test_id: "T-F57-CARRIER-WINDOWS-SERVICE-INSTALL",
        trusted_runner_id: "f57-windows-service-install-evidence",
        raw_signer_registry_key: "RELEASE_WINDOWS_SERVICE_INSTALL_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 Windows Service Install Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_WINDOWS_SCM_READBACK",
        script_trust_descriptor_id: "F57_PS_INSTALL_SERVICES_V1",
    },
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::Postgres16Pitr,
        script_repo_path: "scripts/windows/test-postgres16-pitr.ps1",
        script_specific_argv_template: &[
            "-Mode", "Release",
            "-StagingPlan", r"<staging>\plan.v1.json", "-CandidateManifest", "<candidate>",
            "-EvidenceOut", r"<staging>\raw\00.v1.json",
        ],
        staging_input_ids: &[],
        raw_schema_id: "ReleasePostgres16PitrEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-postgres16-pitr-evidence-v1+json",
        raw_cardinality: 1,
        auxiliary_test_id: "T-F57-CARRIER-POSTGRES16-PITR",
        trusted_runner_id: "f57-postgres16-pitr-evidence",
        raw_signer_registry_key: "RELEASE_POSTGRES16_PITR_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 PostgreSQL 16 PITR Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_SIGNED_PITR_CUT",
        script_trust_descriptor_id: "F57_PS_TEST_POSTGRES16_PITR_V1",
    },
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::BackupRestoreCertification,
        script_repo_path: "scripts/windows/backup-restore-drill.ps1",
        script_specific_argv_template: &[
            "-Mode", "Release",
            "-StagingPlan", r"<staging>\plan.v1.json", "-CertificationRun", "<1|2|3>",
            "-RecoveryExecutionAttemptId", "<signed-plan-run-id>",
            "-CandidateManifest", "<candidate>",
            "-EvidenceOut", r"<staging>\raw\<00|01|02>.v1.json",
        ],
        staging_input_ids: &[],
        raw_schema_id: "ReleaseRecoveryCertificationEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-recovery-certification-evidence-v1+json",
        raw_cardinality: 3,
        auxiliary_test_id: "T-F57-CARRIER-BACKUP-RESTORE-CERTIFICATION",
        trusted_runner_id: "f57-recovery-certification-evidence",
        raw_signer_registry_key: "RELEASE_RECOVERY_CERTIFICATION_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 Recovery Certification Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_RECOVERY_CERTIFICATION_ID",
        script_trust_descriptor_id: "F57_PS_BACKUP_RESTORE_DRILL_V1",
    },
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::P340Release72Hour,
        script_repo_path: "scripts/windows/run-p340-certification.ps1",
        script_specific_argv_template: &[
            "-Mode", "Release72Hour",
            "-StagingPlan", r"<staging>\plan.v1.json", "-CandidateManifest", "<candidate>",
            "-PolicyAttestation", r"<staging>\inputs\p340-certification-policy-attestation.v1.json",
            "-CapacityInputManifest", r"<staging>\inputs\p340-capacity-input-manifest.v1.json",
            "-EvidenceOut", r"<staging>\raw\00.v1.json",
        ],
        staging_input_ids: &[
            CarrierStagingInputIdV1::P340CapacityInputManifest,
            CarrierStagingInputIdV1::P340CertificationPolicyAttestation,
        ],
        raw_schema_id: "P340SoakEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-p340-soak-evidence-v1+json",
        raw_cardinality: 1,
        auxiliary_test_id: "T-F57-CARRIER-P340-RELEASE72-HOUR",
        trusted_runner_id: "f57-p340-release-hardware",
        raw_signer_registry_key: "P340_SOAK_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 P340 Release Hardware Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_P340_SOAK_RUN_ID",
        script_trust_descriptor_id: "F57_PS_RUN_P340_CERTIFICATION_V1",
    },
    ReleaseCarrierRegistryRowV1 {
        recipe: ReleaseCarrierRecipeIdV1::PowerShutdown,
        script_repo_path: "scripts/windows/test-power-shutdown.ps1",
        script_specific_argv_template: &[
            "-Mode", "PrepareOnly",
            "-StagingPlan", r"<staging>\plan.v1.json", "-CandidateManifest", "<candidate>",
            "-UpsIdentityReadback", r"<staging>\inputs\ups-identity-readback.v1.json",
            "-UpsPowerWriteCachePolicy", r"<staging>\inputs\ups-power-write-cache-policy.v1.json",
            "-QuiesceOut", r"<staging>\artifacts\<quiesce-readback-key-sha256>.bin",
        ],
        staging_input_ids: &[
            CarrierStagingInputIdV1::UpsIdentityReadback,
            CarrierStagingInputIdV1::UpsPowerWriteCachePolicy,
        ],
        raw_schema_id: "PowerShutdownEvidenceV1",
        raw_media_type: "application/vnd.ep.f57-power-shutdown-evidence-v1+json",
        raw_cardinality: 1,
        auxiliary_test_id: "T-F57-CARRIER-POWER-SHUTDOWN",
        trusted_runner_id: "f57-power-shutdown-evidence",
        raw_signer_registry_key: "POWER_SHUTDOWN_EVIDENCE_V1",
        required_raw_subject_dn: "CN=EP F57 Power Shutdown Evidence,O=Enterprise Platform",
        result_signer_registry_key: "RELEASE_CARRIER_RESULT_V1",
        required_result_subject_dn: "CN=EP F57 Release Evidence Aggregator,O=Enterprise Platform",
        reconciler_id: "RECONCILE_UPS_SHUTDOWN_BOOT_ID",
        script_trust_descriptor_id: "F57_PS_TEST_POWER_SHUTDOWN_V1",
    },
];
```

For the backup row the dispatcher expands exactly three ordered invocations by joining the verified recovery plan; no caller substitutes either placeholder. `script_specific_argv_template` contains only the arguments after the script name. It must never contain a host switch, `-File`, a script path, or a security-policy switch: the G0 trusted executor alone injects the verified canonical PowerShell host and exact `-NoProfile -NonInteractive -ExecutionPolicy AllSigned -File <canonical-absolute-descriptor-owned-script>` prefix while holding the verified handles. At activation, each signer key resolves one exact row in the already verified 89-row registry and the compiled DN must byte-equal that row while the verifier uses the row's pinned SPKI digest, EKU, trust domain and issuance policy. SPKI values are deliberately not copied into this table, preventing a second rotation authority. `f57-release-carrier-compiled-registry-v1-golden.json` is an independent literal projection of all fields above; the test parses that fixture and compares it directly to `RELEASE_CARRIER_REGISTRY_V1`, then proves the dispatcher indexes only this constant. There is no `expected_six_rows_from_master_carrier_table()` helper, second mapping or free-form script/argv/reconciler lookup.

The PowerShell invocation-graph closure is separately exact and contains these 18 `(script_id,repo_path)` rows, no more and no fewer: `F57_PS_RUN_L2_CANDIDATE_V1/run-l2-candidate.ps1`; `F57_PS_BUILD_MSI_V1/build-msi.ps1`; `F57_PS_INSTALL_SERVICES_V1/install-services.ps1`; `F57_PS_VERIFY_SERVICE_ACLS_V1/verify-service-acls.ps1`; `F57_PS_VERIFY_IPC_V1/verify-ipc.ps1`; `F57_PS_VERIFY_HDD_ROUTING_V1/verify-hdd-routing.ps1`; `F57_PS_VERIFY_TIME_V1/verify-time.ps1`; `F57_PS_ARCHIVE_WAL_V1/archive-wal.ps1`; `F57_PS_TEST_POSTGRES16_PITR_V1/test-postgres16-pitr.ps1`; `F57_PS_BACKUP_RESTORE_DRILL_V1/backup-restore-drill.ps1`; `F57_PS_RUN_P340_CERTIFICATION_V1/run-p340-certification.ps1`; `F57_PS_VERIFY_WINDOWS_SERVER_2022_V1/verify-windows-server-2022.ps1`; `F57_PS_VERIFY_BITLOCKER_V1/verify-bitlocker.ps1`; `F57_PS_VERIFY_BOOT_SECURITY_V1/verify-boot-security.ps1`; `F57_PS_VERIFY_RESIDENCY_V1/verify-residency.ps1`; `F57_PS_VERIFY_FILESYSTEM_GEOMETRY_V1/verify-filesystem-geometry.ps1`; `F57_PS_TEST_POWER_SHUTDOWN_V1/test-power-shutdown.ps1`; and `F57_PS_RUN_L3_RELEASE_V1/run-l3-release.ps1`, all below `scripts/windows/` and all resolving one same-ID descriptor below `scripts/windows/trust/`.

Task 14 first finishes every script byte, then re-signs and regenerates descriptors for its changed `build-msi`, `install-services`, `run-l2-candidate` and new `run-l3-release` scripts. It independently re-verifies the other 14 owner-produced final descriptors. Only then does the G0 cumulative generator typed-parse all 18 descriptors, derive the closed call graph, reject a missing/unreachable/extra/cyclic/raw child invocation, and atomically regenerate the existing byte-equal `generated_registry.rs`, JSON registry and independent fixture. It is the final state of the one registry created in G0 and incrementally regenerated by each script owner, not a second release-only registry. The generated Rust row stores the strict descriptor value and its descriptor-file SHA-256; its resolver accepts only `PowerShellScriptIdV1`, and each of the six carrier rows' `script_trust_descriptor_id` must resolve to the same script path in that registry. Nested scripts are launched through the same fixed-host/final-handle executor by compiled child ID, never directly by PowerShell path. Any edit after registry generation, unsigned nested script, descriptor mismatch, alternate host or Task-15 regeneration makes the release source dirty and fails before freeze.

- [ ] **Step 3: Implement final candidate closure, exact handlers, and release aggregation.**

```rust
pub struct ReleaseCandidatePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseCandidatePurposeV1,
    pub finalization_attempt_id: UuidV1,
    pub identity: CandidateIdentityV1,
    pub gate_run_id: UuidV1,
    pub precursor_journal_checkpoint_ref: ArtifactRefV1,
    pub fresh_pg_receipt: CandidateBoundFreshPgReceiptRefV1,
    pub artifact_signer_registry_ref: ArtifactRefV1,
    pub generation_manifest_ref: ArtifactRefV1,
    pub generation_approval_registry_ref: ArtifactRefV1,
    pub generation_participant_ack_refs: Vec<ArtifactRefV1>,
    pub generation_observed_selection_ref: ArtifactRefV1,
    pub offline_schema_manifest_ref: ArtifactRefV1,
    pub client_artifact_set_ref: ArtifactRefV1,
    pub pre_freeze_carrier_refs: Vec<ArtifactRefV1>,
    pub source_tree: ArtifactRefV1,
    pub artifacts: Vec<ReleaseArtifactRefV1>,
    pub graph_manifest: ArtifactRefV1,
    pub projection_manifest: ArtifactRefV1,
    pub requirement_facade_manifest: ArtifactRefV1,
    pub migration_closure: MigrationClosureIdentityV1,
    pub migration_closure_artifacts: MigrationClosureArtifactRefsV1,
    pub runtime_topology_declaration_ref: ArtifactRefV1,
    pub toolchain_manifest: ArtifactRefV1,
    pub data_classification: CandidateDataClassificationV1,
    pub storage_root_binding: G6VerifiedDataHddRootBindingV1,
}

use crate::carrier_contract::ReleaseCarrierRecipeIdV1;
use crate::l2::TargetGateV1;
use ep_client_common::ClientPackageArtifactIdV1;

pub enum ReleaseCarrierResultPurposeV1 { ReleaseCarrierResult }
pub enum CarrierStagingPlanPurposeV1 { CarrierStagingPlan }
pub enum CarrierStagingCompletionPurposeV1 { CarrierStagingCompletion }
pub enum CarrierStagingOutputClassV1 { OpaqueObject, SignedEnvelope }
pub enum WindowsAuthorityArtifactSetPurposeV1 { WindowsAuthorityArtifactSet }
pub enum L3EvidencePurposeV1 { ReleaseCertification }

#[serde(tag = "artifact_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReleaseArtifactRefV1 {
    WindowsAuthority {
        authority_artifact_set_ref: ArtifactRefV1,
    },
    ClientPackage {
        artifact_id: ClientPackageArtifactIdV1,
        signed_artifact_ref: ArtifactRefV1,
    },
}

pub struct MigrationClosureArtifactRefsV1 {
    pub baseline_registry: ArtifactRefV1,
    pub baseline_apply_manifest: ArtifactRefV1,
    pub f57_reservation_manifest: ArtifactRefV1,
    pub legacy_seed: ArtifactRefV1,
}

pub enum ReleaseCandidatePurposeV1 { ReleaseCandidate }

pub type ReleaseCandidateV1 = SignedBusinessArtifactV1<ReleaseCandidatePayloadV1>;

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct G6CertifiedDataHddRootV1 {
    pub verified_root_binding: G6VerifiedDataHddRootBindingV1,
    pub p340_soak_evidence_ref: ArtifactRefV1,
    pub residency_readback_ref: ArtifactRefV1,
    pub filesystem_geometry_readback_ref: ArtifactRefV1,
}

pub struct WindowsAuthorityArtifactSetPayloadV1 {
    pub schema_version: u32,
    pub purpose: WindowsAuthorityArtifactSetPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub runner_id: RunnerIdV1,
    pub source_tree_ref: ArtifactRefV1,
    pub authority_binary_ref: ArtifactRefV1,
    pub authority_kernel_binary_ref: ArtifactRefV1,
    pub authority_kernel_sbom_ref: ArtifactRefV1,
    pub authority_kernel_abi_readback_ref: ArtifactRefV1,
    pub authority_kernel_slot_pointer_ref: ArtifactRefV1,
    pub ups_adapter_manifest_ref: ArtifactRefV1,
    pub authority_manifest_ref: ArtifactRefV1,
    pub msi_ref: ArtifactRefV1,
    pub runtime_deployment_closure_ref: ArtifactRefV1,
    pub server_component_set_ref: ArtifactRefV1,
    pub postgres16_windows_install_contract_ref: ArtifactRefV1,
    pub production_admission_bypass_registry_ref: ArtifactRefV1,
    pub authenticode_readback_ref: ArtifactRefV1,
    pub toolchain_manifest_ref: ArtifactRefV1,
    pub build_log_ref: ArtifactRefV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type WindowsAuthorityArtifactSetV1 =
    SignedBusinessArtifactV1<WindowsAuthorityArtifactSetPayloadV1>;

use ep_platform_backup::windows_components::{
    PgPassphraseHelperTransportV1, RecoveryToolOperationV1, RecoveryToolRequestVerbV1,
    WindowsBackupComponentIdV1, WindowsBackupComponentPlacementV1,
    WindowsBackupServiceInstallRowV1, WindowsBackupServiceRecoveryPolicyV1,
    WindowsBackupServiceStartModeV1, WindowsOnDemandExecutablePolicyV1,
    WindowsScheduledTaskFolderAclEnforcementV1, WindowsScheduledTaskLogonKindV1,
    WindowsScheduledTaskPrincipalPolicyV1, WindowsScheduledTaskRegistrationPolicyV1,
    WindowsScheduledTaskRunLevelV1, WindowsServerComponentActivationV1,
    WindowsServerComponentArtifactV1, WindowsServerComponentSetV1,
    WindowsTokenElevationTypeV1, WindowsTokenIntegrityLevelV1,
    WindowsTokenPrivilegeAttributeRowV1, WindowsTokenSidAttributeRowV1,
};

pub enum ReleaseAuthorityBuildEvidencePurposeV1 { ReleaseAuthorityBuildEvidence }
pub struct ReleaseAuthorityBuildEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseAuthorityBuildEvidencePurposeV1,
    pub context: ReleasePhysicalEvidenceContextV1,
    pub authority_artifact_set_ref: ArtifactRefV1,
    pub outcome: ReleaseCarrierOutcomeV1,
}
pub type ReleaseAuthorityBuildEvidenceV1 =
    SignedBusinessArtifactV1<ReleaseAuthorityBuildEvidencePayloadV1>;

pub struct PowerShutdownInstalledFileIdentityV1 {
    pub canonical_path: String,
    pub volume_identity: String,
    pub file_id_128_lowerhex: String,
    pub size_bytes: u64,
    pub binary_sha256: Sha256Digest,
    pub authenticode_readback_ref: ArtifactRefV1,
}

pub struct PowerShutdownInstalledServiceAttestationV1 {
    pub install_plan_row_sha256: Sha256Digest,
    pub scm_dacl_canonical_sddl: String,
    pub scm_dacl_sha256: Sha256Digest,
    pub installed_file_identity: PowerShutdownInstalledFileIdentityV1,
    pub installed_file_identity_sha256: Sha256Digest,
    pub installed_binary_authenticode_ref: ArtifactRefV1,
    pub runtime_readback: WindowsAuthorityServiceRuntimeReadbackV1,
}

#[serde(tag = "runtime_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WindowsAuthorityServiceRuntimeReadbackV1 {
    Running {
        windows_boot_id: String,
        process_id: u32,
        process_start_key: u64,
        held_process_image_identity_sha256: Sha256Digest,
        token_service_sid: WindowsSidV1,
        challenged_role: PowerShutdownBrokerServiceRoleV1,
        challenge_nonce_lowerhex: String,
        authenticated_challenge_response_sha256: Sha256Digest,
        authenticated_session_binding_sha256: Sha256Digest,
        observed_at_unix_ms: i64,
    },
    DormantContinuationSelfCheck {
        windows_boot_id: String,
        service_state: String,
        last_process_id: u32,
        last_process_start_key: u64,
        held_process_image_identity_sha256: Sha256Digest,
        token_service_sid: WindowsSidV1,
        active_record_path_absent: bool,
        side_effect_count: u64,
        exit_code: u32,
        scm_recovery_action_count: u32,
        self_check_receipt_ref: ArtifactRefV1,
        observed_at_unix_ms: i64,
    },
}

#[serde(tag = "service_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerShutdownInstalledServiceReadbackV1 {
    AuthorityServer {
        service: PowerShutdownBrokerServiceV1,
        authority_recovery_proof_ipc_endpoint: String,
        authority_recovery_proof_ipc_acl_sha256: Sha256Digest,
        attestation: PowerShutdownInstalledServiceAttestationV1,
    },
    Continuation {
        service: PowerShutdownContinuationInstalledServiceV1,
        attestation: PowerShutdownInstalledServiceAttestationV1,
    },
    ControlBroker {
        service: PowerShutdownBrokerServiceV1,
        ipc_endpoint: String,
        ipc_acl_sha256: Sha256Digest,
        attestation: PowerShutdownInstalledServiceAttestationV1,
    },
    RawEvidenceSigner {
        broker: PowerShutdownSigningBrokerV1,
        attestation: PowerShutdownInstalledServiceAttestationV1,
    },
    GateRunJournalSigner {
        broker: PowerShutdownSigningBrokerV1,
        attestation: PowerShutdownInstalledServiceAttestationV1,
    },
}

#[serde(deny_unknown_fields)]
pub struct WindowsBackupServiceInstalledReadbackV1 {
    pub desired_install_row: WindowsBackupServiceInstallRowV1,
    pub resolved_account_sid: WindowsSidV1,
    pub resolved_service_sid: WindowsSidV1,
    pub observed_sid_type: PowerShutdownContinuationServiceSidTypeV1,
    pub observed_image_path: String,
    pub observed_exact_argv: Vec<String>,
    pub observed_start_mode: WindowsBackupServiceStartModeV1,
    pub observed_recovery_policy: WindowsBackupServiceRecoveryPolicyV1,
    pub observed_dependency_service_names: Vec<String>,
    pub observed_required_privilege_names: Vec<String>,
    pub canonical_service_sddl: String,
    pub service_sddl_sha256: Sha256Digest,
    pub observed_ipc_endpoint: String,
    pub canonical_ipc_dacl: String,
    pub ipc_dacl_sha256: Sha256Digest,
    pub canonical_executable_dacl: String,
    pub executable_dacl_sha256: Sha256Digest,
    pub observed_capability_ids: Vec<String>,
    pub observed_outbound_network_allowed: bool,
    pub observed_data_volume_identity_binding_required: bool,
}

#[serde(deny_unknown_fields)]
pub struct WindowsScheduledTaskFolderAclReadbackV1 {
    pub folder_path: String,
    pub enforcement: WindowsScheduledTaskFolderAclEnforcementV1,
    pub canonical_sddl: String,
    pub canonical_sddl_sha256: Sha256Digest,
    pub required_ace_set_sha256: Sha256Digest,
    pub forbidden_write_ace_count: u32,
}

#[serde(tag = "component_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum WindowsBackupRecoveryComponentInstallReadbackV1 {
    RunningService {
        component_id: WindowsBackupComponentIdV1,
        installed_file_identity: PowerShutdownInstalledFileIdentityV1,
        service_sid: WindowsSidV1,
        observed_install_row: WindowsBackupServiceInstalledReadbackV1,
        service_runtime: WindowsBackupServiceRuntimeReadbackV1,
        capability_set_sha256: Sha256Digest,
        data_hdd_unlock_broker_bootstrap_readback_ref: Option<ArtifactRefV1>,
    },
    OnDemandScheduledTask {
        component_id: WindowsBackupComponentIdV1,
        installed_file_identity: PowerShutdownInstalledFileIdentityV1,
        principal_policy: WindowsScheduledTaskPrincipalPolicyV1,
        account_name: String,
        task_path: String,
        account_sid: WindowsSidV1,
        logon_kind: WindowsScheduledTaskLogonKindV1,
        run_level: WindowsScheduledTaskRunLevelV1,
        assigned_account_right_names: Vec<String>,
        task_required_privilege_names: Vec<String>,
        denied_logon_right_names: Vec<String>,
        observed_direct_local_group_sids: Vec<String>,
        observed_local_group_sids: Vec<String>,
        observed_prohibited_group_intersection: Vec<String>,
        observed_account_enabled: bool,
        observed_user_flags_mask: u32,
        observed_account_expires_never: bool,
        observed_logon_hours_unrestricted: bool,
        task_scheduler_stored_password_present: bool,
        installer_plaintext_password_residue_count: u32,
        runtime_token_user_sid: WindowsSidV1,
        runtime_token_group_rows: Vec<WindowsTokenSidAttributeRowV1>,
        runtime_token_group_row_set_sha256: Sha256Digest,
        runtime_token_privilege_rows: Vec<WindowsTokenPrivilegeAttributeRowV1>,
        runtime_token_privilege_row_set_sha256: Sha256Digest,
        runtime_token_integrity_level: WindowsTokenIntegrityLevelV1,
        runtime_token_elevation_type: WindowsTokenElevationTypeV1,
        runtime_token_is_app_container: bool,
        runtime_token_is_restricted: bool,
        authenticated_self_test_token_projection_sha256: Sha256Digest,
        fixed_action_executable_path: String,
        fixed_action_arguments: Vec<String>,
        observed_registration_policy: WindowsScheduledTaskRegistrationPolicyV1,
        parent_folder_acl_readbacks: Vec<WindowsScheduledTaskFolderAclReadbackV1>,
        parent_folder_sddl_set_sha256: Sha256Digest,
        canonical_task_sddl: String,
        task_sddl_sha256: Sha256Digest,
        ipc_endpoint: String,
        canonical_ipc_dacl: String,
        ipc_dacl_sha256: Sha256Digest,
        canonical_executable_dacl: String,
        executable_dacl_sha256: Sha256Digest,
        request_verb_allowlist: Vec<RecoveryToolRequestVerbV1>,
        operation_allowlist: Vec<RecoveryToolOperationV1>,
        normalized_registration_projection_sha256: Sha256Digest,
        self_test_exit_code: u32,
        self_test_side_effect_count: u64,
        observed_at_unix_ms: i64,
    },
    OnDemandExecutable {
        component_id: WindowsBackupComponentIdV1,
        installed_file_identity: PowerShutdownInstalledFileIdentityV1,
        account_sid: WindowsSidV1,
        token_privilege_names: Vec<String>,
        policy: WindowsOnDemandExecutablePolicyV1,
        canonical_executable_dacl: String,
        executable_dacl_sha256: Sha256Digest,
        observed_exact_argv: Vec<String>,
        observed_transport: PgPassphraseHelperTransportV1,
        observed_job_active_process_limit: u32,
        observed_kill_on_job_close: bool,
        observed_child_process_creation_allowed: bool,
        observed_outbound_network_allowed: bool,
        process_mitigation_policy_sha256: Sha256Digest,
        protocol_readback_sha256: Sha256Digest,
        observed_plaintext_argv_count: u32,
        observed_plaintext_environment_count: u32,
        observed_plaintext_log_or_temp_file_count: u32,
        self_test_exit_code: u32,
        self_test_side_effect_count: u64,
        observed_at_unix_ms: i64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct WindowsBackupServiceRuntimeReadbackV1 {
    pub component_id: WindowsBackupComponentIdV1,
    pub windows_boot_id: String,
    pub process_id: u32,
    pub process_start_key: u64,
    pub held_process_image_identity_sha256: Sha256Digest,
    pub token_user_sid: WindowsSidV1,
    pub token_service_sid: WindowsSidV1,
    pub token_logon_sid: WindowsSidV1,
    pub token_enabled_group_sids: Vec<WindowsSidV1>,
    pub token_enabled_group_sid_set_sha256: Sha256Digest,
    pub token_restricted_sids: Vec<WindowsSidV1>,
    pub token_restricted_sid_set_sha256: Sha256Digest,
    pub token_privilege_names: Vec<String>,
    pub challenge_nonce_lowerhex: String,
    pub authenticated_challenge_response_sha256: Sha256Digest,
    pub authenticated_session_binding_sha256: Sha256Digest,
    pub observed_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Postgres16WindowsPackageLockPurposeV1 {
    #[serde(rename = "EP-F57-POSTGRES16-WINDOWS-PACKAGE-LOCK-V1")]
    Postgres16WindowsPackageLock,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Postgres16WindowsInstallContractPurposeV1 {
    #[serde(rename = "EP-F57-POSTGRES16-WINDOWS-INSTALL-CONTRACT-V1")]
    Postgres16WindowsInstallContract,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Postgres16WindowsInstallReadbackPurposeV1 {
    #[serde(rename = "EP-F57-POSTGRES16-WINDOWS-INSTALL-READBACK-V1")]
    Postgres16WindowsInstallReadback,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Postgres16WindowsEventLogFixtureSetPurposeV1 {
    #[serde(rename = "EP-F57-POSTGRES16-WINDOWS-EVENT-LOG-FIXTURE-SET-V1")]
    Postgres16WindowsEventLogFixtureSet,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Postgres16WindowsEventLogScanCoveragePurposeV1 {
    #[serde(rename = "EP-F57-POSTGRES16-WINDOWS-EVENT-LOG-SCAN-COVERAGE-V1")]
    Postgres16WindowsEventLogScanCoverage,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsServiceStartModeV1 { DemandStart }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsServiceRecoveryPolicyV1 { NoAutomaticRestart }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsServiceSidTypeV1 { Unrestricted }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsServiceTypeV1 { Win32OwnProcess }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsServiceErrorControlV1 { Normal }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsRuntimeServiceStateV1 { Running }
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsPathRoleV1 {
    EngineInstallRoot,
    Pgdata,
    LiveWal,
    WalArchiveStaging,
    ProcessTemp,
    RestoreScratch,
    ServerLog,
    TlsSecret,
    Configuration,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsPathVolumeRoleV1 { RuntimeSsd, DataHdd }
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16InstalledFileRowV1 {
    pub canonical_relative_path: String,
    pub file_size_bytes: u64,
    pub file_sha256: Sha256Digest,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsPackageLockV1 {
    pub schema_version: u32,
    pub purpose: Postgres16WindowsPackageLockPurposeV1,
    pub offline_package_ref: ArtifactRefV1,
    pub package_authenticode_policy_ref: ArtifactRefV1,
    pub package_authenticode_readback_ref: ArtifactRefV1,
    pub sbom_ref: ArtifactRefV1,
    pub installed_files: Vec<Postgres16InstalledFileRowV1>,
    pub expected_installed_file_set_sha256: Sha256Digest,
    pub server_version: String,
    pub server_version_num: u32,
    pub windows_target_triple: String,
    pub pg_control_version: u32,
    pub catalog_version: u32,
    pub initdb_locale_provider: String,
    pub initdb_locale: String,
    pub initdb_encoding: String,
    pub data_checksums_required: bool,
    pub bundled_extensions: Vec<String>,
    pub downgrade_allowed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEventLogFixtureSetV1 {
    pub schema_version: u32,
    pub purpose: Postgres16WindowsEventLogFixtureSetPurposeV1,
    pub fixture_set_id: String,
    pub synthetic_customer_tokens: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEventLogScanContractV1 {
    pub channel_name: String,
    pub expected_provider_names: Vec<String>,
    pub fixture_set_ref: ArtifactRefV1,
    pub fixture_set_sha256: Sha256Digest,
    pub maximum_customer_token_match_count: u32,
    pub coverage_complete_required: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEventLogProviderRegistrationReadbackV1 {
    pub provider_name: String,
    pub channel_name: String,
    pub registry_key_final_path: String,
    pub provider_registration_sha256: Sha256Digest,
    pub event_message_file_identity: PowerShutdownInstalledFileIdentityV1,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEventLogBookmarkV1 {
    pub bookmark_xml: String,
    pub record_id: u64,
    pub observed_at_unix_ms: i64,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEventLogScanCoverageV1 {
    pub schema_version: u32,
    pub purpose: Postgres16WindowsEventLogScanCoveragePurposeV1,
    pub install_contract_ref: ArtifactRefV1,
    pub scan_contract_sha256: Sha256Digest,
    pub windows_boot_id: String,
    pub channel_name: String,
    pub provider_registration_rows: Vec<Postgres16WindowsEventLogProviderRegistrationReadbackV1>,
    pub start_bookmark: Postgres16WindowsEventLogBookmarkV1,
    pub end_bookmark: Postgres16WindowsEventLogBookmarkV1,
    pub scanned_record_count: u64,
    pub channel_clear_count: u32,
    pub collector_dropped_record_count: u32,
    pub unexplained_record_id_gap_count: u32,
    pub fixture_set_ref: ArtifactRefV1,
    pub fixture_set_sha256: Sha256Digest,
    pub expected_fixture_count: u32,
    pub exercised_fixture_count: u32,
    pub customer_token_match_count: u32,
    pub coverage_complete: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsServiceRowV1 {
    pub service_name: String,
    pub display_name: String,
    pub virtual_account_name: String,
    pub service_sid_type: Postgres16WindowsServiceSidTypeV1,
    pub start_mode: Postgres16WindowsServiceStartModeV1,
    pub recovery_policy: Postgres16WindowsServiceRecoveryPolicyV1,
    pub service_type: Postgres16WindowsServiceTypeV1,
    pub error_control: Postgres16WindowsServiceErrorControlV1,
    pub delayed_auto_start: bool,
    pub installed_pg_ctl_path: String,
    pub raw_image_path: String,
    pub exact_argv: Vec<String>,
    pub dependency_service_names: Vec<String>,
    pub required_privilege_names: Vec<String>,
    pub authority_service_access_mask: u32,
    pub recovery_service_access_mask: u32,
    pub service_sddl_template: String,
    pub service_sddl_template_sha256: Sha256Digest,
    pub executable_sddl_template: String,
    pub executable_sddl_template_sha256: Sha256Digest,
    pub loopback_only_required: bool,
    pub outbound_network_allowed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsPathPolicyRowV1 {
    pub path_role: Postgres16WindowsPathRoleV1,
    pub canonical_path: String,
    pub volume_role: Postgres16WindowsPathVolumeRoleV1,
    pub expected_volume_identity: String,
    pub customer_authority_bytes_allowed: bool,
    pub reparse_point_allowed: bool,
    pub alternate_data_stream_allowed: bool,
    pub profile_or_environment_fallback_allowed: bool,
    pub canonical_sddl_template: String,
    pub canonical_sddl_template_sha256: Sha256Digest,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsClientAuthenticationV1 {
    ScramSha256,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Postgres16WindowsClientChannelBindingV1 {
    Require,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsClientConnectionRowV1 {
    pub consumer_id: String,
    pub service_identity: String,
    pub database_name: String,
    pub database_role: String,
    pub purpose: String,
    pub source_cidrs: Vec<String>,
    pub tls_required: bool,
    pub authentication: Postgres16WindowsClientAuthenticationV1,
    pub client_channel_binding: Postgres16WindowsClientChannelBindingV1,
    pub connection_privilege_class: DatabaseConnectionPrivilegeClassV1,
    pub steady_pool_max: u32,
    pub peak_pool_max: u32,
    pub acquire_timeout_ms: u64,
    pub statement_timeout_ms: u64,
    pub capacity_budget_weight: u32,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsClientConnectionProbeReadbackV1 {
    pub consumer_id: String,
    pub service_identity: String,
    pub database_role: String,
    pub connection_privilege_class: DatabaseConnectionPrivilegeClassV1,
    pub tls_established: bool,
    pub authentication: Postgres16WindowsClientAuthenticationV1,
    pub client_channel_binding: Postgres16WindowsClientChannelBindingV1,
    pub channel_binding_negotiated: bool,
    pub backend_rolsuper: bool,
    pub backend_has_reserved_connections_privilege: bool,
    pub authenticated_session_binding_sha256: Sha256Digest,
    pub observed_at_unix_ms: i64,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsConfigurationProjectionV1 {
    pub postgresql_conf_ref: ArtifactRefV1,
    pub pg_hba_conf_ref: ArtifactRefV1,
    pub pg_ident_conf_ref: ArtifactRefV1,
    pub tls_policy_ref: ArtifactRefV1,
    pub archive_executor_descriptor_ref: ArtifactRefV1,
    pub client_connection_rows: Vec<Postgres16WindowsClientConnectionRowV1>,
    pub reserved_connections: u32,
    pub superuser_reserved_connections: u32,
    pub migration_connection_reserve: u32,
    pub recovery_connection_reserve: u32,
    pub unallocatable_safety_reserve: u32,
    pub installed_extension_names: Vec<String>,
    pub enabled_extension_names: Vec<String>,
    pub critical_setting_rows: Vec<String>,
    pub hba_rows: Vec<String>,
    pub ident_rows: Vec<String>,
    pub ambient_include_allowed: bool,
    pub postgresql_auto_conf_override_allowed: bool,
    pub user_tablespaces_allowed: bool,
    pub temp_tablespaces: String,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsInstallContractV1 {
    pub schema_version: u32,
    pub purpose: Postgres16WindowsInstallContractPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub storage_root_binding: G6VerifiedDataHddRootBindingV1,
    pub package_lock_ref: ArtifactRefV1,
    pub engine_install_root: String,
    pub service_row: Postgres16WindowsServiceRowV1,
    pub path_projection: Vec<Postgres16WindowsPathPolicyRowV1>,
    pub configuration_projection: Postgres16WindowsConfigurationProjectionV1,
    pub event_log_scan_contract: Postgres16WindowsEventLogScanContractV1,
    pub server_component_set_ref: ArtifactRefV1,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsFsyncQualificationV1 {
    pub pg_test_fsync_file_identity: PowerShutdownInstalledFileIdentityV1,
    pub tested_directory: String,
    pub data_hdd_volume_identity: String,
    pub storage_driver_stack_sha256: Sha256Digest,
    pub write_cache_policy_sha256: Sha256Digest,
    pub wal_sync_method: String,
    pub fsync_supported: bool,
    pub fsync_ops_per_second_milli: u64,
    pub fsync_writethrough_supported: bool,
    pub fsync_writethrough_ops_per_second_milli: u64,
    pub fsync_and_fsync_writethrough_same_test_file: bool,
    pub io_error_count: u32,
    pub captured_at: Rfc3339MillisUtc,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsEngineReadbackV1 {
    pub engine_install_root: String,
    pub package_lock_ref: ArtifactRefV1,
    pub installed_file_set_sha256: Sha256Digest,
    pub pg_ctl_file_identity: PowerShutdownInstalledFileIdentityV1,
    pub postgres_file_identity: PowerShutdownInstalledFileIdentityV1,
    pub server_version: String,
    pub server_version_num: u32,
    pub pg_control_version: u32,
    pub catalog_version: u32,
    pub cluster_system_identifier: String,
    pub pg_control_system_identifier: String,
    pub sql_system_identifier: String,
    pub observed_initdb_locale_provider: String,
    pub observed_initdb_locale: String,
    pub observed_initdb_encoding: String,
    pub available_extension_names: Vec<String>,
    pub fsync_qualification: Postgres16WindowsFsyncQualificationV1,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsServiceReadbackV1 {
    pub desired_service_row_sha256: Sha256Digest,
    pub observed_service_name: String,
    pub observed_display_name: String,
    pub observed_virtual_account_name: String,
    pub resolved_service_sid: WindowsSidV1,
    pub observed_service_sid_type: Postgres16WindowsServiceSidTypeV1,
    pub observed_start_mode: Postgres16WindowsServiceStartModeV1,
    pub observed_recovery_policy: Postgres16WindowsServiceRecoveryPolicyV1,
    pub observed_service_type: Postgres16WindowsServiceTypeV1,
    pub observed_error_control: Postgres16WindowsServiceErrorControlV1,
    pub observed_delayed_auto_start: bool,
    pub observed_raw_image_path: String,
    pub observed_exact_argv: Vec<String>,
    pub observed_dependency_service_names: Vec<String>,
    pub observed_required_privilege_names: Vec<String>,
    pub canonical_service_sddl: String,
    pub service_sddl_sha256: Sha256Digest,
    pub canonical_executable_sddl: String,
    pub executable_sddl_sha256: Sha256Digest,
    pub observed_pg_ctl_event_source: String,
    pub observed_postgres_event_source: String,
    pub observed_early_fallback_event_source: String,
    pub server_eventlog_destination_enabled: bool,
    pub event_log_scan_coverage_ref: ArtifactRefV1,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsPathReadbackV1 {
    pub path_role: Postgres16WindowsPathRoleV1,
    pub canonical_final_path: String,
    pub volume_identity: String,
    pub final_handle_identity_sha256: Sha256Digest,
    pub reparse_point_count: u32,
    pub alternate_data_stream_count: u32,
    pub alias_or_fallback_count: u32,
    pub canonical_dacl_sddl: String,
    pub canonical_dacl_sha256: Sha256Digest,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsConfigurationReadbackV1 {
    pub configuration_projection_sha256: Sha256Digest,
    pub postgresql_conf_ref: ArtifactRefV1,
    pub pg_hba_conf_ref: ArtifactRefV1,
    pub pg_ident_conf_ref: ArtifactRefV1,
    pub effective_installed_extension_names: Vec<String>,
    pub effective_enabled_extension_names: Vec<String>,
    pub effective_critical_setting_rows: Vec<String>,
    pub effective_hba_rows: Vec<String>,
    pub effective_ident_rows: Vec<String>,
    pub authenticated_client_probe_rows: Vec<Postgres16WindowsClientConnectionProbeReadbackV1>,
    pub ambient_include_count: u32,
    pub postgresql_auto_conf_override_count: u32,
    pub trust_authentication_row_count: u32,
    pub external_listener_or_hba_row_count: u32,
    pub data_checksums_enabled: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsRuntimeReadbackV1 {
    pub windows_boot_id: String,
    pub process_id: u32,
    pub process_start_key: u64,
    pub held_postgres_image_identity_sha256: Sha256Digest,
    pub token_service_sid: WindowsSidV1,
    pub service_state: Postgres16WindowsRuntimeServiceStateV1,
    pub listen_addresses: Vec<String>,
    pub listener_count: u32,
    pub nonloopback_listener_count: u32,
    pub outbound_nonloopback_socket_count: u32,
    pub authenticated_probe_sha256: Sha256Digest,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres16WindowsInstallReadbackV1 {
    pub schema_version: u32,
    pub purpose: Postgres16WindowsInstallReadbackPurposeV1,
    pub install_contract_ref: ArtifactRefV1,
    pub package_lock_ref: ArtifactRefV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub storage_root_binding: G6VerifiedDataHddRootBindingV1,
    pub engine_readback: Postgres16WindowsEngineReadbackV1,
    pub service_readback: Postgres16WindowsServiceReadbackV1,
    pub path_readbacks: Vec<Postgres16WindowsPathReadbackV1>,
    pub configuration_readback: Postgres16WindowsConfigurationReadbackV1,
    pub tls_readback_ref: ArtifactRefV1,
    pub firewall_readback_ref: ArtifactRefV1,
    pub postgres_system_identifier: String,
    pub runtime_readback: Postgres16WindowsRuntimeReadbackV1,
    pub pre_data_hdd_process_start_count: u32,
    pub runtime_ssd_postgres_customer_authority_bytes: u64,
    pub observed_at_unix_ms: i64,
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalBackupTargetDeploymentReadbackV1 {
    pub component_id: WindowsBackupComponentIdV1,
    pub target_host_fingerprint_sha256: Sha256Digest,
    pub authority_host_fingerprint_sha256: Sha256Digest,
    pub package_ref: ArtifactRefV1,
    pub binary_authenticode_readback_ref: ArtifactRefV1,
    pub append_only_endpoint_identity_sha256: Sha256Digest,
    pub backup_topology_ref: ArtifactRefV1,
    pub storage_safeguard_readback_ref: ArtifactRefV1,
    pub authenticated_mtls_session_binding_sha256: Sha256Digest,
    pub shared_admin_domain: bool,
    pub shared_credential_domain: bool,
    pub authority_host_installation_count: u32,
    pub authenticated_version_challenge_sha256: Sha256Digest,
    pub observed_at_unix_ms: i64,
}

#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum F57EvidenceSignerProviderModeV1 {
    SelfHostedOsKeystorePivV1,
    ExistingEnterpriseSignerV1,
}
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum F57EvidenceSignerBrokerReadinessStateV1 {
    WaitingForDataHdd,
    Ready,
    Held,
}
#[serde(deny_unknown_fields)]
pub struct F57EvidenceSignerBrokerWindowsInstallRowV1 {
    pub service_name: String,
    pub display_name: String,
    pub virtual_account_name: String,
    pub sid_type: PowerShutdownContinuationServiceSidTypeV1,
    pub start_mode: WindowsBackupServiceStartModeV1,
    pub recovery_policy: WindowsBackupServiceRecoveryPolicyV1,
    pub source_entrypoint_path: String,
    pub installed_executable_path: String,
    pub exact_argv: Vec<String>,
    pub dependency_service_names: Vec<String>,
    pub required_privilege_names: Vec<String>,
    pub ipc_endpoint: String,
    pub service_sddl_template: String,
    pub executable_sddl_template: String,
    pub ipc_sddl_template: String,
    pub client_local_group_name: String,
    pub client_group_sddl_template: String,
}
pub enum F57EvidenceSignerBrokerWindowsInstallReadbackPurposeV1 {
    #[serde(rename = "EP-F57-EVIDENCE-SIGNER-BROKER-WINDOWS-INSTALL-READBACK-V1")]
    F57EvidenceSignerBrokerWindowsInstallReadback,
}
#[serde(deny_unknown_fields)]
pub struct F57EvidenceSignerBrokerWindowsInstallReadbackV1 {
    pub schema_version: u32,
    pub purpose: F57EvidenceSignerBrokerWindowsInstallReadbackPurposeV1,
    pub desired_install_row: F57EvidenceSignerBrokerWindowsInstallRowV1,
    pub install_row_sha256: Sha256Digest,
    pub installed_file_identity: PowerShutdownInstalledFileIdentityV1,
    pub installed_binary_authenticode_ref: ArtifactRefV1,
    pub observed_image_path: String,
    pub observed_exact_argv: Vec<String>,
    pub resolved_account_sid: WindowsSidV1,
    pub resolved_service_sid: WindowsSidV1,
    pub resolved_client_group_sid: WindowsSidV1,
    pub resolved_client_group_member_sids: Vec<WindowsSidV1>,
    pub client_group_member_sid_set_sha256: Sha256Digest,
    pub observed_sid_type: PowerShutdownContinuationServiceSidTypeV1,
    pub observed_start_mode: WindowsBackupServiceStartModeV1,
    pub observed_recovery_policy: WindowsBackupServiceRecoveryPolicyV1,
    pub observed_dependency_service_names: Vec<String>,
    pub observed_required_privilege_names: Vec<String>,
    pub canonical_service_sddl: String,
    pub service_sddl_sha256: Sha256Digest,
    pub canonical_executable_sddl: String,
    pub executable_sddl_sha256: Sha256Digest,
    pub canonical_ipc_sddl: String,
    pub ipc_sddl_sha256: Sha256Digest,
    pub canonical_client_group_sddl: String,
    pub client_group_sddl_sha256: Sha256Digest,
    pub active_broker_session_sha256: Sha256Digest,
    pub active_signer_registry_sha256: Sha256Digest,
    pub provider_mode: F57EvidenceSignerProviderModeV1,
    pub readiness_state: F57EvidenceSignerBrokerReadinessStateV1,
    pub authority_storage_manifest_ref: ArtifactRefV1,
    pub data_hdd_volume_identity: String,
    pub canonical_data_hdd_state_root: String,
    pub broker_state_closure_ref: ArtifactRefV1,
    pub runtime_ssd_mutable_fallback_count: u32,
    pub allowed_outbound_destination_tuples: Vec<String>,
    pub outbound_firewall_readback_ref: ArtifactRefV1,
    pub observed_outbound_socket_count: u32,
    pub windows_boot_id: String,
    pub process_id: u32,
    pub process_start_key: u64,
    pub held_process_image_identity_sha256: Sha256Digest,
    pub challenge_nonce_lowerhex: String,
    pub authenticated_challenge_response_sha256: Sha256Digest,
    pub authenticated_session_binding_sha256: Sha256Digest,
    pub observed_at_unix_ms: i64,
}

pub enum ReleaseWindowsServiceInstallEvidencePurposeV1 { ReleaseWindowsServiceInstallEvidence }
pub struct ReleaseWindowsServiceInstallEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseWindowsServiceInstallEvidencePurposeV1,
    pub context: ReleasePhysicalEvidenceContextV1,
    pub authority_artifacts_ref: ArtifactRefV1,
    pub installed_msi_ref: ArtifactRefV1,
    pub runtime_deployment_closure_ref: ArtifactRefV1,
    pub runtime_deployment_readback_set_ref: ArtifactRefV1,
    pub server_component_set_ref: ArtifactRefV1,
    pub production_admission_bypass_registry_ref: ArtifactRefV1,
    pub authority_services: Vec<PowerShutdownInstalledServiceReadbackV1>,
    pub installed_backup_recovery_components: Vec<WindowsBackupRecoveryComponentInstallReadbackV1>,
    pub external_backup_target: ExternalBackupTargetDeploymentReadbackV1,
    pub postgres16_windows_install_readback_ref: ArtifactRefV1,
    pub evidence_signer_broker_install_readback_ref: ArtifactRefV1,
    pub data_hdd_unlock_authority_ref: ArtifactRefV1,
    pub data_hdd_unlock_readback_ref: ArtifactRefV1,
    pub service_sid_readback_ref: ArtifactRefV1,
    pub service_dacl_readback_ref: ArtifactRefV1,
    pub ipc_dacl_readback_ref: ArtifactRefV1,
    pub storage_routing_readback_ref: ArtifactRefV1,
    pub trusted_time_readback_ref: ArtifactRefV1,
    pub outcome: ReleaseCarrierOutcomeV1,
}
pub type ReleaseWindowsServiceInstallEvidenceV1 =
    SignedBusinessArtifactV1<ReleaseWindowsServiceInstallEvidencePayloadV1>;

// The install carrier accepts this payload only after all five static rows and
// all five role-specific runtime readbacks below have been exact-verified.

pub enum ReleasePostgres16PitrEvidencePurposeV1 { ReleasePostgres16PitrEvidence }
pub struct ReleasePostgres16PitrEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleasePostgres16PitrEvidencePurposeV1,
    pub context: ReleasePhysicalEvidenceContextV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub backup_checkpoint_ref: ArtifactRefV1,
    pub postgres_version: String,
    pub authority_recovery_cut_manifest_ref: ArtifactRefV1,
    pub authority_recovery_cut_manifest_sha256: Sha256Digest,
    pub write_barrier_id: UuidV1,
    pub database_cut_ref: ArtifactRefV1,
    pub wal_cut_ref: ArtifactRefV1,
    pub attachment_cut_ref: ArtifactRefV1,
    pub append_only_readback_ref: ArtifactRefV1,
    pub key_domain_readback_ref: ArtifactRefV1,
    pub offline_media_readback_refs: Vec<ArtifactRefV1>,
    pub outcome: ReleaseCarrierOutcomeV1,
}
pub type ReleasePostgres16PitrEvidenceV1 =
    SignedBusinessArtifactV1<ReleasePostgres16PitrEvidencePayloadV1>;

// `backup_checkpoint_ref` is mandatory and is verified through the dedicated
// BACKUP recovery-domain trust descriptor, never the F57 signer registry. Its
// full-cut ref/digest/barrier tuple exact-equals the three fields above.

pub enum ReleaseRecoveryCertificationEvidencePurposeV1 { ReleaseRecoveryCertificationEvidence }
pub enum RecoveryCertificationPhaseV1 { InitialRestoreVerified, CandidateMeasured, Certified }
pub struct ReleaseRecoveryCertificationEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseRecoveryCertificationEvidencePurposeV1,
    pub context: ReleasePhysicalEvidenceContextV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub recovery_certification_policy_ref: ArtifactRefV1,
    pub certification_id: UuidV1,
    pub certification_run: u8,
    pub phase: RecoveryCertificationPhaseV1,
    pub recovery_execution_attempt_id: UuidV1,
    pub clean_source_cut_ref: ArtifactRefV1,
    pub restored_database_ref: ArtifactRefV1,
    pub restored_attachment_ref: ArtifactRefV1,
    pub audit_outbox_replay_ref: ArtifactRefV1,
    pub key_custody_readback_ref: ArtifactRefV1,
    pub business_reopen_readback_ref: ArtifactRefV1,
    pub outcome: ReleaseCarrierOutcomeV1,
}
pub type ReleaseRecoveryCertificationEvidenceV1 =
    SignedBusinessArtifactV1<ReleaseRecoveryCertificationEvidencePayloadV1>;

pub enum CarrierStagingInputIdV1 {
    WindowsAuthorityArtifactSet,
    WindowsAuthorityManifest,
    WindowsAuthorityMsi,
    P340CertificationPolicyAttestation,
    P340CapacityInputManifest,
    UpsIdentityReadback,
    UpsPowerWriteCachePolicy,
}

pub struct CarrierStagingInputV1 {
    pub input_id: CarrierStagingInputIdV1,
    pub artifact_ref: ArtifactRefV1,
    pub staging_relative_path: String,
    pub media_type: String,
}

pub struct CarrierStagingPlanEntryV1 {
    pub concrete_artifact_key: String,
    pub staging_relative_path: String,
    pub expected_media_type: String,
    pub output_class: CarrierStagingOutputClassV1,
}

use ep_platform_runtime::windows::authority_manifest::{
    PowerShutdownBrokerRecoveryPolicyV1, PowerShutdownBrokerServiceRoleV1,
    PowerShutdownBrokerServiceStartModeV1, PowerShutdownContinuationRecoveryPolicyV1,
    PowerShutdownContinuationServiceSidTypeV1, PowerShutdownContinuationServiceStartModeV1,
    PowerShutdownDispatcherScmRightV1, PowerShutdownServicePreshutdownPolicyV1,
    PowerShutdownSigningBrokerRoleV1, WindowsSidV1,
};

pub struct PowerShutdownBrokerServiceV1 {
    pub role: PowerShutdownBrokerServiceRoleV1,
    pub service_name: String,
    pub display_name: String,
    pub virtual_account_name: String,
    pub service_sid: WindowsSidV1,
    pub sid_type: PowerShutdownContinuationServiceSidTypeV1,
    pub start_mode: PowerShutdownBrokerServiceStartModeV1,
    pub recovery_policy: PowerShutdownBrokerRecoveryPolicyV1,
    pub source_entrypoint_path: String,
    pub installed_executable_path: String,
    pub installed_binary_ref: ArtifactRefV1,
    pub exact_argv: Vec<String>,
    pub dependency_service_names: Vec<String>,
    pub required_privilege_names: Vec<String>,
    pub preshutdown_policy: PowerShutdownServicePreshutdownPolicyV1,
}

pub struct PowerShutdownSigningBrokerV1 {
    pub role: PowerShutdownSigningBrokerRoleV1,
    pub service: PowerShutdownBrokerServiceV1,
    pub ipc_endpoint: String,
    pub ipc_acl_sha256: Sha256Digest,
    pub upstream_evidence_signer_endpoint: String,
    pub allowed_artifact_kind: String,
    pub allowed_discriminator: String,
    pub forwarding_policy_sha256: Sha256Digest,
}

pub struct PowerShutdownControlTokenScmPreflightReadbackV1 {
    pub control_service_sid: WindowsSidV1,
    pub token_privilege_names: Vec<String>,
    pub global_scm_rights: Vec<String>,
    pub continuation_service_name: String,
    pub continuation_service_rights: Vec<PowerShutdownDispatcherScmRightV1>,
}

pub struct PowerShutdownDispatcherAuthorityV1 {
    pub service: PowerShutdownBrokerServiceV1,
    pub ipc_endpoint: String,
    pub ipc_acl_sha256: Sha256Digest,
    pub token_privilege_names: Vec<String>,
    pub scm_rights: Vec<PowerShutdownDispatcherScmRightV1>,
    pub raw_evidence_signing_broker: PowerShutdownSigningBrokerV1,
    pub journal_record_signing_broker: PowerShutdownSigningBrokerV1,
    pub token_and_scm_preflight_readback: PowerShutdownControlTokenScmPreflightReadbackV1,
    pub token_and_scm_preflight_readback_sha256: Sha256Digest,
}

pub enum PowerShutdownSecurityDescriptorObjectV1 {
    ScmAuthorityServerService,
    ScmContinuationService,
    ScmControlBrokerService,
    ScmRawSignerService,
    ScmJournalSignerService,
    InstalledExecutable,
    BundleRoot,
    RunJournal,
    StagingRoot,
    ControlCapsuleRoot,
    PhaseState,
    ActivationChildRegistryKey,
    ControlBrokerPipe,
    RawSignerPipe,
    JournalSignerPipe,
    EvidenceBrokerRawAuthorizationKey,
    EvidenceBrokerJournalAuthorizationKey,
    AuthorityRecoveryProofPipe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerShutdownSecurityDescriptorActionV1 {
    VerifyExistingImmutable,
    CreateWithDescriptor,
}

pub struct PowerShutdownSecurityDescriptorBindingV1 {
    pub object: PowerShutdownSecurityDescriptorObjectV1,
    pub action: PowerShutdownSecurityDescriptorActionV1,
    pub canonical_sddl: String,
    pub canonical_sddl_sha256: Sha256Digest,
}

#[serde(deny_unknown_fields)]
pub struct WindowsAppendOnlyDurabilityPolicyV1 {
    pub desired_access_mask: u32,
    pub share_mode_mask: u32,
    pub file_flag_write_through: bool,
    pub force_file_pointer_to_end_before_each_write: bool,
    pub exclusive_range_lock_required: bool,
    pub flush_file_buffers_allowed: bool,
    pub close_and_reopen_readback_required: bool,
    pub append_only_durability_qualification_ref: ArtifactRefV1,
}

pub struct PowerShutdownContinuationSecurityDescriptorsV1 {
    pub descriptors: Vec<PowerShutdownSecurityDescriptorBindingV1>,
    pub append_only_durability_policy: WindowsAppendOnlyDurabilityPolicyV1,
}

pub struct PowerShutdownContinuationInstalledServiceV1 {
    pub service_name: String,
    pub display_name: String,
    pub virtual_account_name: String,
    pub service_sid: WindowsSidV1,
    pub sid_type: PowerShutdownContinuationServiceSidTypeV1,
    pub start_mode: PowerShutdownContinuationServiceStartModeV1,
    pub recovery_policy: PowerShutdownContinuationRecoveryPolicyV1,
    pub source_entrypoint_path: String,
    pub installed_executable_path: String,
    pub installed_binary_ref: ArtifactRefV1,
    pub exact_argv: Vec<String>,
    pub activation_parameter_registry_path: String,
    pub authority_recovery_proof_ipc_endpoint: String,
    pub dependency_service_names: Vec<String>,
    pub required_privilege_names: Vec<String>,
    pub preshutdown_policy: PowerShutdownServicePreshutdownPolicyV1,
}

pub enum PowerShutdownContinuationStatePurposeV1 { PowerShutdownContinuationState }
pub enum PowerShutdownContinuationPhaseV1 {
    Armed,
    PreShutdownCommitted,
    PostRestartCompleted,
    Disarmed,
}

pub struct PowerShutdownContinuationStateHeaderV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationStatePurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
}

pub struct PowerShutdownContinuationStateRecordV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationStatePurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub sequence: u8,
    pub previous_frame_sha256: Sha256Digest,
    pub phase: PowerShutdownContinuationPhaseV1,
    pub phase_readback_ref: ArtifactRefV1,
}

use ep_platform_gate_journal_contract::PowerShutdownContinuationStatePrefixV1;

pub enum PowerShutdownControlCapsuleResidencyClassV1 { NoBusinessData }
pub struct PowerShutdownControlCapsulePlanV1 {
    pub residency_class: PowerShutdownControlCapsuleResidencyClassV1,
    pub candidate_identity: CandidateIdentityV1,
    pub artifact_signer_registry_ref: ArtifactRefV1,
    pub runtime_ssd_volume_identity: String,
    pub capsule_root_path: String,
    pub plan_relative_path: String,
    pub artifact_signer_registry_relative_path: String,
    pub gate_prefix_relative_path: String,
    pub state_prefix_relative_path: String,
    pub dispatch_commit_intent_relative_path: String,
    pub dispatch_api_call_committed_relative_path: String,
    pub dispatch_acknowledgement_relative_path: String,
    pub postboot_resume_controller_relative_path: String,
    pub failure_relative_path: String,
    pub failure_cleanup_relative_path: String,
    pub maximum_total_bytes: u64,
}

#[serde(tag = "continuation_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CarrierContinuationPlanV1 {
    None,
    PowerShutdown {
        continuation_id: UuidV1,
        continuation_binary_ref: ArtifactRefV1,
        dispatcher_authority: PowerShutdownDispatcherAuthorityV1,
        service: PowerShutdownContinuationInstalledServiceV1,
        security_descriptors: PowerShutdownContinuationSecurityDescriptorsV1,
        bundle_root_path: String,
        run_journal_path: String,
        staging_root_path: String,
        control_capsule: PowerShutdownControlCapsulePlanV1,
        phase_state_run_relative_path: String,
        success_persistence_command_relative_path: String,
        raw_evidence_relative_path: String,
    },
}

pub struct RecoveryCertificationRunPlanV1 {
    pub certification_run: u8,
    pub phase: RecoveryCertificationPhaseV1,
    pub recovery_execution_attempt_id: UuidV1,
}

#[serde(tag = "recipe_specific_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CarrierRecipeSpecificPlanV1 {
    None,
    RecoveryCertification {
        certification_id: UuidV1,
        runs: [RecoveryCertificationRunPlanV1; 3],
    },
}

pub struct CarrierStagingPlanPayloadV1 {
    pub schema_version: u32,
    pub purpose: CarrierStagingPlanPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub recipe_id: ReleaseCarrierRecipeIdV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub inputs: Vec<CarrierStagingInputV1>,
    pub entries: Vec<CarrierStagingPlanEntryV1>,
    pub recipe_specific: CarrierRecipeSpecificPlanV1,
    pub continuation: CarrierContinuationPlanV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type CarrierStagingPlanV1 =
    SignedBusinessArtifactV1<CarrierStagingPlanPayloadV1>;

pub struct CarrierStagingCompletionEntryV1 {
    pub concrete_artifact_key: String,
    pub staging_relative_path: String,
    pub size_bytes: u64,
    pub sha256: Sha256Digest,
    pub media_type: String,
}

pub struct CarrierStagingCompletionPayloadV1 {
    pub schema_version: u32,
    pub purpose: CarrierStagingCompletionPurposeV1,
    pub finalization_attempt_id: UuidV1,
    pub frozen_input_checkpoint_ref: ArtifactRefV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub recipe_id: ReleaseCarrierRecipeIdV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub entries: Vec<CarrierStagingCompletionEntryV1>,
    pub completed_at_unix_ms: i64,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type CarrierStagingCompletionV1 =
    SignedBusinessArtifactV1<CarrierStagingCompletionPayloadV1>;

pub struct ReleaseCarrierResultPayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseCarrierResultPurposeV1,
    pub recipe_id: ReleaseCarrierRecipeIdV1,
    pub carrier_test_id: TestIdV1,
    pub binding: ReleaseCarrierBindingV1,
    pub gate_run_id: UuidV1,
    pub execution_attempt_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub staging_completion_ref: ArtifactRefV1,
    pub outcome: ReleaseCarrierOutcomeV1,
    pub evidence_refs: Vec<ArtifactRefV1>,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type ReleaseCarrierResultV1 =
    SignedBusinessArtifactV1<ReleaseCarrierResultPayloadV1>;

pub enum PowerShutdownContinuationArmReadbackPurposeV1 {
    PowerShutdownContinuationArmReadback,
}

pub struct PowerShutdownArmServiceConfigurationReadbackV1 {
    pub installed_service: PowerShutdownContinuationInstalledServiceV1,
    pub dispatcher_authority: PowerShutdownDispatcherAuthorityV1,
    pub security_descriptors: PowerShutdownContinuationSecurityDescriptorsV1,
    pub activation_parameter_absent: bool,
}

pub struct PowerShutdownContinuationArmReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationArmReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub pre_shutdown_boot_id: String,
    pub applied_dispatcher_authority: PowerShutdownDispatcherAuthorityV1,
    pub applied_service: PowerShutdownContinuationInstalledServiceV1,
    pub applied_security_descriptors: PowerShutdownContinuationSecurityDescriptorsV1,
    pub service_configuration_readback: PowerShutdownArmServiceConfigurationReadbackV1,
    pub service_configuration_readback_sha256: Sha256Digest,
    pub armed_at_unix_ms: i64,
}

pub enum PowerShutdownQuiesceReadbackPurposeV1 { PowerShutdownQuiesceReadback }

pub struct PowerShutdownQuiesceReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownQuiesceReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub host_fingerprint_sha256: Sha256Digest,
    pub pre_shutdown_boot_id: String,
    pub authority_barrier_id: UuidV1,
    pub authority_barrier_ref: ArtifactRefV1,
    pub postgres_checkpoint_ref: ArtifactRefV1,
    pub postgres_checkpoint_lsn: u64,
    pub postgres_wal_flush_lsn: u64,
    pub attachment_fsync_manifest_ref: ArtifactRefV1,
    pub audit_checkpoint_ref: ArtifactRefV1,
    pub outbox_checkpoint_ref: ArtifactRefV1,
    pub ups_trigger_readback_ref: ArtifactRefV1,
    pub in_flight_accepted_command_count: u64,
    pub quiesce_started_at_unix_ms: i64,
    pub quiesce_completed_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct UpsAdapterIdentityV1(pub Sha256Digest);

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(transparent)]
pub struct UpsAdapterVersionV1(String); /* private; hand-written Deserialize uses canonical SemVer 2.0.0 grammar, no build metadata */

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UpsAdapterManifestPurposeV1 {
    #[serde(rename = "EP-F57-UPS-ADAPTER-MANIFEST-V1")]
    UpsAdapterManifest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UpsStatusReadbackPurposeV1 {
    #[serde(rename = "EP-F57-UPS-STATUS-READBACK-V1")]
    UpsStatusReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UpsOutletCycleCommandPurposeV1 {
    #[serde(rename = "EP-F57-UPS-OUTLET-CYCLE-COMMAND-V1")]
    UpsOutletCycleCommand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum UpsOutletCycleCommandAckPurposeV1 {
    #[serde(rename = "EP-F57-UPS-OUTLET-CYCLE-COMMAND-ACK-V1")]
    UpsOutletCycleCommandAck,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsCarrierKindV1 {
    WindowsStandardPowerStatus,
    SignedVendorAdapter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsAdapterCapabilityV1 {
    ReadAcInput,
    ReadBatterySupply,
    ReadBatteryCharge,
    ReadRemainingRuntime,
    ReadCommunicationHealth,
    ReadSelfTest,
    ReadOutputState,
    ScheduleIdempotentOutletCycle,
    QueryOutletCycleByCommandId,
    ReadOutletCycleLog,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsAdapterExecutionRoleV1 { EpAuthorityControl }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsNetworkProtocolV1 { HttpsMutualTls13, SnmpV3AuthPriv }

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct WindowsDeviceInterfaceClassGuidV1(String);

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "address_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsCanonicalIpAddressV1 {
    Ipv4 { octets: [u8; 4] },
    Ipv6 { octets: [u8; 16] },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsNetworkDestinationV1 {
    pub protocol: UpsNetworkProtocolV1,
    pub remote_ip: UpsCanonicalIpAddressV1,
    pub remote_port: std::num::NonZeroU16,
    pub remote_peer_identity_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "transport_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsAdapterTransportPolicyV1 {
    WindowsSystemPowerStatus,
    LocalDeviceBound {
        device_interface_class_guid: WindowsDeviceInterfaceClassGuidV1,
        vendor_id: u16,
        product_id: u16,
        device_instance_id_uppercase: String,
        device_instance_id_sha256: Sha256Digest,
    },
    ExactNetworkEndpoint {
        destination: UpsNetworkDestinationV1,
        dns_allowed: bool,
        proxy_allowed: bool,
        redirects_allowed: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "credential_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsAdapterCredentialPolicyV1 {
    None,
    ServiceSidDeviceAclOnly,
    CngNonExportableClientCertificate {
        certificate_spki_sha256: Sha256Digest,
        key_locator_sha256: Sha256Digest,
    },
    DpapiNgServiceSidSealedSecret {
        credential_locator_sha256: Sha256Digest,
        protector_descriptor_sha256: Sha256Digest,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsSupportedDeviceProfileV1 {
    pub device_profile_id: String,
    pub manufacturer: String,
    pub model: String,
    pub firmware_revisions: Vec<String>,
    pub controlled_outlet_group_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsAdapterConfigurationProjectionV1 {
    pub configuration_generation: u64,
    pub selected_device_profile_id: Option<String>,
    pub selected_controlled_outlet_group_id: Option<String>,
    pub protected_host_power_path_sha256: Option<Sha256Digest>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsAdapterManifestV1 {
    pub schema_version: u32,
    pub purpose: UpsAdapterManifestPurposeV1,
    pub carrier_kind: UpsCarrierKindV1,
    pub adapter_id: String,
    pub adapter_version: UpsAdapterVersionV1,
    pub implementation_binary_ref: ArtifactRefV1,
    pub configuration_projection: UpsAdapterConfigurationProjectionV1,
    pub supported_device_profiles: Vec<UpsSupportedDeviceProfileV1>,
    pub capabilities: Vec<UpsAdapterCapabilityV1>,
    pub execution_role: UpsAdapterExecutionRoleV1,
    pub transport_policy: UpsAdapterTransportPolicyV1,
    pub credential_policy: UpsAdapterCredentialPolicyV1,
    pub status_poll_interval_seconds: u64,
    pub maximum_status_age_seconds: u64,
    pub maximum_self_test_age_seconds: u64,
    pub maximum_command_ack_seconds: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsAdapterRuntimeSecurityReadbackV1 {
    pub execution_role: UpsAdapterExecutionRoleV1,
    pub windows_service_name: String,
    pub windows_service_sid: WindowsSidV1,
    pub process_id: u32,
    pub process_start_key: u64,
    pub held_implementation_binary_sha256: Sha256Digest,
    pub transport_policy_sha256: Sha256Digest,
    pub credential_policy_sha256: Sha256Digest,
    pub device_acl_readback_ref: Option<ArtifactRefV1>,
    pub outbound_firewall_readback_ref: Option<ArtifactRefV1>,
    pub observed_outbound_destination_tuples: Vec<UpsNetworkDestinationV1>,
    pub unexpected_loaded_module_count: u32,
    pub spawned_child_process_count: u32,
    pub credential_export_or_secret_exposure_count: u32,
    pub observed_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsAcInputStateV1 { Online, Lost, Unknown }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsBatterySupplyStateV1 { OnBattery, NotOnBattery, Unknown }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsOutputStateV1 { Online, Off, Unknown }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsCommunicationStateV1 { Healthy, Lost, Unknown }

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "state", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsSelfTestReadbackV1 {
    Passed { completed_at_unix_ms: i64, provider_attestation_ref: ArtifactRefV1 },
    Failed { completed_at_unix_ms: i64, provider_attestation_ref: ArtifactRefV1 },
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "state", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsRemainingRuntimeReadbackV1 {
    KnownSeconds { seconds: u64 },
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "state", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum UpsBatteryChargeReadbackV1 {
    KnownPercent { percent: u8 },
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsAlertV1 {
    BatteryLow,
    ReplaceBattery,
    Overload,
    OverTemperature,
    InputOutOfRange,
    OutputFault,
    CommunicationLost,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsStatusReadbackV1 {
    pub schema_version: u32,
    pub purpose: UpsStatusReadbackPurposeV1,
    pub carrier_kind: UpsCarrierKindV1,
    pub adapter_manifest_ref: ArtifactRefV1,
    pub device_profile_id: Option<String>,
    pub ups_adapter_identity: UpsAdapterIdentityV1,
    pub adapter_configuration_sha256: Sha256Digest,
    pub configuration_generation: u64,
    pub runtime_security_binding_sha256: Sha256Digest,
    pub status_sequence: u64,
    pub ac_input_state: UpsAcInputStateV1,
    pub battery_supply_state: UpsBatterySupplyStateV1,
    pub output_state: UpsOutputStateV1,
    pub communication_state: UpsCommunicationStateV1,
    pub self_test: UpsSelfTestReadbackV1,
    pub remaining_runtime: UpsRemainingRuntimeReadbackV1,
    pub battery_charge: UpsBatteryChargeReadbackV1,
    pub active_alerts: Vec<UpsAlertV1>,
    pub observed_at_unix_ms: i64,
    pub valid_until_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsOutletCycleOperationV1 { OutletOffAfterDelayThenRestore }

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsOutletCycleCommandV1 {
    pub schema_version: u32,
    pub purpose: UpsOutletCycleCommandPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub command_id: UuidV1,
    pub adapter_manifest_ref: ArtifactRefV1,
    pub ups_identity_readback_ref: ArtifactRefV1,
    pub ups_adapter_identity: UpsAdapterIdentityV1,
    pub adapter_configuration_sha256: Sha256Digest,
    pub configuration_generation: u64,
    pub dispatch_owner_token_sha256: Sha256Digest,
    pub pre_shutdown_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub dispatch_deadline_monotonic_tick_ms: u64,
    pub controlled_outlet_group_id: String,
    pub protected_host_power_path_sha256: Sha256Digest,
    pub operation: UpsOutletCycleOperationV1,
    pub outlet_off_delay_seconds: u64,
    pub restore_output_on_ac_return: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UpsOutletCycleCommandStateV1 { Scheduled }

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpsOutletCycleCommandAckV1 {
    pub schema_version: u32,
    pub purpose: UpsOutletCycleCommandAckPurposeV1,
    pub command_id: UuidV1,
    pub command_sha256: Sha256Digest,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub adapter_manifest_ref: ArtifactRefV1,
    pub ups_identity_readback_ref: ArtifactRefV1,
    pub ups_adapter_identity: UpsAdapterIdentityV1,
    pub adapter_configuration_sha256: Sha256Digest,
    pub configuration_generation: u64,
    pub ups_serial_number: String,
    pub ups_firmware_revision: String,
    pub controlled_outlet_group_id: String,
    pub protected_host_power_path_sha256: Sha256Digest,
    pub operation: UpsOutletCycleOperationV1,
    pub outlet_off_delay_seconds: u64,
    pub restore_output_on_ac_return: bool,
    pub provider_operation_id: String,
    pub pre_shutdown_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub adapter_call_started_monotonic_tick_ms: u64,
    pub acknowledgement_observed_monotonic_tick_ms: u64,
    pub state: UpsOutletCycleCommandStateV1,
    pub first_dispatch_started_at_unix_ms: i64,
    pub accepted_at_unix_ms: i64,
}

pub struct VerifiedUpsAdapterBindingV1 { /* private manifest + configuration + live execution proof */ }
pub struct VerifiedUpsStatusReadbackV1 { /* private strict-decoded status + object ref */ }
pub struct VerifiedUpsOutletCycleCommandAckV1 { /* private strict-decoded byte-identical ACK */ }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpsPortErrorV1 {
    ManifestInvalid,
    ConfigurationMismatch,
    CapabilityInsufficient,
    StaleStatus,
    CommandIdConflict,
    CommandStateUnknown,
    TransportUnavailable,
    AuthenticationFailed,
    DeviceRejected,
    DeadlineExceeded,
    Storage,
}

pub trait UpsStatusPortV1: Send + Sync {
    fn read_status(
        &self,
        binding: &VerifiedUpsAdapterBindingV1,
    ) -> Result<VerifiedUpsStatusReadbackV1, UpsPortErrorV1>;
}

pub trait UpsOutletControlPortV1: Send + Sync {
    fn begin_or_adopt(
        &self,
        binding: &VerifiedUpsAdapterBindingV1,
        command: &UpsOutletCycleCommandV1,
    ) -> Result<VerifiedUpsOutletCycleCommandAckV1, UpsPortErrorV1>;

    fn query_exact(
        &self,
        binding: &VerifiedUpsAdapterBindingV1,
        command_id: UuidV1,
        command_sha256: Sha256Digest,
    ) -> Result<VerifiedUpsOutletCycleCommandAckV1, UpsPortErrorV1>;
}

pub enum PowerShutdownDispatchCommitIntentPurposeV1 { PowerShutdownDispatchCommitIntent }
pub struct PowerShutdownDispatchCommitIntentV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownDispatchCommitIntentPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub quiesce_readback_ref: ArtifactRefV1,
    pub dispatch_owner_token_sha256: Sha256Digest,
    pub outlet_cycle_command: UpsOutletCycleCommandV1,
    pub pre_shutdown_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub committed_monotonic_tick_ms: u64,
    pub dispatch_reconciliation_deadline_tick_ms: u64,
}

// Private, non-wire proof: only ep-platform-release::power_shutdown may construct it.
pub struct PowerShutdownDispatchPreparationV1 {
    candidate_run: CandidateRunIdentityV1,
    execution_attempt_id: UuidV1,
    continuation_id: UuidV1,
    staging_plan_ref: ArtifactRefV1,
    dispatch_owner_token_sha256: Sha256Digest,
}

// Private affine command: neither Clone nor Serialize; consumed by dispatch_once.
pub struct PowerShutdownDispatchOnceCommandV1 {
    candidate_run: CandidateRunIdentityV1,
    execution_attempt_id: UuidV1,
    continuation_id: UuidV1,
    staging_plan_ref: ArtifactRefV1,
    dispatch_commit_intent_ref: ArtifactRefV1,
    dispatch_owner_token_sha256: Sha256Digest,
}

// Private non-wire proof; only RuntimeTopologyCertificationAuthorityV1 constructs it.
pub struct VerifiedRuntimeTopologyCertificationV1 {
    value: RuntimeTopologyCertificationV1,
    artifact_ref: ArtifactRefV1,
    p340_terminal_checkpoint_ref: ArtifactRefV1,
}

impl VerifiedRuntimeTopologyCertificationV1 {
    pub fn value(&self) -> &RuntimeTopologyCertificationV1 {
        &self.value
    }

    pub fn artifact_ref(&self) -> &ArtifactRefV1 {
        &self.artifact_ref
    }

    pub fn p340_terminal_checkpoint_ref(&self) -> &ArtifactRefV1 {
        &self.p340_terminal_checkpoint_ref
    }
}

pub enum RuntimeTopologyCertificationErrorV1 {
    CandidateInvalid,
    CheckpointInvalid,
    P340NotTerminal,
    P340BindingMismatch,
    DeclarationInvalid,
    GenerationNotObserved,
    GraphMismatch,
    LiveReadbackMismatch,
    CertificationBindingMismatch,
    ObjectStore,
    ObjectConflict,
}

// Private-field production authority; xtask only receives it from composition.
pub struct RuntimeTopologyCertificationAuthorityV1<'deps, 'registry> {
    artifact_verifier: &'deps ArtifactVerifierV1,
    generation_approval_verifier: &'deps GenerationApprovalVerifierV1<'registry>,
    activation_attempt_store: &'deps dyn GenerationActivationAttemptStoreV1,
    journal_reader: &'deps dyn GateRunJournalReadPortV1,
    object_contract_resolver: &'deps dyn EvidenceObjectContractResolverV1,
    object_store: &'deps dyn EvidenceObjectStoreV1,
    declaration_occurrence_contract: VerifiedEvidenceObjectContractV1,
    certification_class_contract: VerifiedEvidenceObjectContractV1,
}

impl<'deps, 'registry: 'deps> RuntimeTopologyCertificationAuthorityV1<'deps, 'registry> {
    pub fn compose(
        artifact_verifier: &'deps ArtifactVerifierV1,
        generation_approval_verifier: &'deps GenerationApprovalVerifierV1<'registry>,
        activation_attempt_store: &'deps dyn GenerationActivationAttemptStoreV1,
        journal_reader: &'deps dyn GateRunJournalReadPortV1,
        object_contract_resolver: &'deps dyn EvidenceObjectContractResolverV1,
        object_store: &'deps dyn EvidenceObjectStoreV1,
    ) -> Result<Self, RuntimeTopologyCertificationErrorV1> {
        let declaration_occurrence_contract = object_contract_resolver.resolve(
            generated_evidence_bindings::release_candidate_runtime_topology_declaration_ref_v1(),
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::DeclarationInvalid)?;
        let certification_class_contract = object_contract_resolver.resolve_class(
            generated_evidence_bindings::runtime_topology_certification_v1(),
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::CertificationBindingMismatch)?;
        Ok(RuntimeTopologyCertificationAuthorityV1 {
            artifact_verifier,
            generation_approval_verifier,
            activation_attempt_store,
            journal_reader,
            object_contract_resolver,
            object_store,
            declaration_occurrence_contract,
            certification_class_contract,
        })
    }

    pub async fn load_or_reconstruct_verified(
        &self,
        candidate: &VerifiedBusinessArtifactV1<ReleaseCandidatePayloadV1>,
        p340_terminal_checkpoint: &VerifiedBusinessArtifactV1<GateRunJournalCheckpointPayloadV1>,
        fresh_live_readback: &RuntimeTopologyReadbackCollectorV1,
    ) -> Result<VerifiedRuntimeTopologyCertificationV1, RuntimeTopologyCertificationErrorV1> {
        let prefix = self.journal_reader
            .load_exact_authenticated_prefix(p340_terminal_checkpoint)
            .map_err(|_| RuntimeTopologyCertificationErrorV1::CheckpointInvalid)?;
        let terminal = select_unique_terminal_p340_record(
            candidate,
            &prefix,
            p340_release_72_hour_test_id(),
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::P340NotTerminal)?;
        let p340 = verify_exact_terminal_p340_closure(
            candidate, &terminal, self.object_contract_resolver, self.object_store,
        )
            .map_err(|_| RuntimeTopologyCertificationErrorV1::P340BindingMismatch)?;

        let declaration_bytes = self.object_store
            .load_exact(
                &candidate.payload().runtime_topology_declaration_ref,
                &self.declaration_occurrence_contract,
            )
            .map_err(|_| RuntimeTopologyCertificationErrorV1::DeclarationInvalid)?;
        let declaration = TopologyVerifier::verify_declaration(
            &candidate.payload().runtime_topology_declaration_ref,
            declaration_bytes.as_ref(),
            fresh_live_readback,
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::LiveReadbackMismatch)?;

        let observed = load_verify_exact_observed_generation_closure(
            candidate,
            self.artifact_verifier,
            self.generation_approval_verifier,
            self.activation_attempt_store,
            self.object_contract_resolver,
            self.object_store,
        ).await.map_err(|_| RuntimeTopologyCertificationErrorV1::GenerationNotObserved)?;
        recompile_and_exact_match_candidate_graph(
            candidate,
            &observed,
            &declaration,
            self.object_contract_resolver,
            self.object_store,
        )
            .map_err(|_| RuntimeTopologyCertificationErrorV1::GraphMismatch)?;

        let value = TopologyVerifier::build_certification(
            candidate_run_identity(candidate),
            candidate.artifact_ref().clone(),
            declaration.artifact_ref().clone(),
            p340.soak_ref().clone(),
            p340.capacity_certificate_ref().clone(),
            p340.host_fingerprint_sha256().clone(),
            p340.hardware_profile_id().clone(),
            p340.storage_profile_id().clone(),
            p340.workload_profile_id().clone(),
            20,
            259_200,
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::CertificationBindingMismatch)?;
        exact_match_certification_to_candidate_p340_observed_and_live(
            &value, candidate, &p340, &observed, &declaration,
        ).map_err(|_| RuntimeTopologyCertificationErrorV1::CertificationBindingMismatch)?;

        let exact_jcs = canonical_jcs(&value)
            .map_err(|_| RuntimeTopologyCertificationErrorV1::CertificationBindingMismatch)?;
        if self.certification_class_contract.media_type()
            != "application/vnd.ep.f57-runtime-topology-certification-v1+json"
        {
            return Err(RuntimeTopologyCertificationErrorV1::CertificationBindingMismatch);
        }
        let artifact_ref = self.object_store
            .create_or_adopt(&self.certification_class_contract, &exact_jcs)
            .map_err(map_topology_object_store_error)?;
        let reloaded = self.object_store.load_exact(
            &artifact_ref,
            &self.certification_class_contract,
        )
            .map_err(|_| RuntimeTopologyCertificationErrorV1::ObjectStore)?;
        let _reloaded_value: RuntimeTopologyCertificationV1 = strict_from_jcs(reloaded.as_ref())
            .map_err(|_| RuntimeTopologyCertificationErrorV1::ObjectConflict)?;
        if reloaded.as_ref() != exact_jcs.as_slice() {
            return Err(RuntimeTopologyCertificationErrorV1::ObjectConflict);
        }
        Ok(VerifiedRuntimeTopologyCertificationV1 {
            value,
            artifact_ref,
            p340_terminal_checkpoint_ref: p340_terminal_checkpoint.artifact_ref().clone(),
        })
    }
}

pub struct VerifiedPowerShutdownPlanV1 {
    verified_plan: VerifiedBusinessArtifactV1<CarrierStagingPlanPayloadV1>,
    topology_certification_ref: ArtifactRefV1,
    service_install_result_ref: ArtifactRefV1,
}

impl VerifiedPowerShutdownPlanV1 {
    pub fn payload(&self) -> &CarrierStagingPlanPayloadV1 {
        self.verified_plan.payload()
    }

    pub fn artifact_ref(&self) -> &ArtifactRefV1 {
        self.verified_plan.artifact_ref()
    }

    pub fn verified_artifact(&self) -> &VerifiedBusinessArtifactV1<CarrierStagingPlanPayloadV1> {
        &self.verified_plan
    }
}

pub struct VerifiedPowerShutdownControlBrokerSessionV1 {
    candidate_run: CandidateRunIdentityV1,
    execution_attempt_id: UuidV1,
    continuation_id: UuidV1,
    authenticated_control_service_sid: WindowsSidV1,
    dispatch_owner_token_sha256: Sha256Digest,
}

pub struct VerifiedPowerShutdownDispatchCommitIntentV1 {
    value: PowerShutdownDispatchCommitIntentV1,
    artifact_ref: ArtifactRefV1,
}

pub struct PowerShutdownAuthorityV1<'a> {
    artifact_verifier: &'a ArtifactVerifierV1,
    journal_reader: &'a dyn GateRunJournalReadPortV1,
    object_contract_resolver: &'a dyn EvidenceObjectContractResolverV1,
    object_store: &'a dyn EvidenceObjectStoreV1,
}

impl<'a> PowerShutdownAuthorityV1<'a> {
    pub fn compose(
        artifact_verifier: &'a ArtifactVerifierV1,
        journal_reader: &'a dyn GateRunJournalReadPortV1,
        object_contract_resolver: &'a dyn EvidenceObjectContractResolverV1,
        object_store: &'a dyn EvidenceObjectStoreV1,
    ) -> Result<Self, PowerShutdownErrorV1> {
        verify_power_plan_service_install_and_topology_contracts_exist(object_contract_resolver)?;
        Ok(Self { artifact_verifier, journal_reader, object_contract_resolver, object_store })
    }

    pub fn verify_plan(
        &self,
        plan: VerifiedBusinessArtifactV1<CarrierStagingPlanPayloadV1>,
        topology: &VerifiedRuntimeTopologyCertificationV1,
    ) -> Result<VerifiedPowerShutdownPlanV1, PowerShutdownErrorV1> {
        self.require_exact_power_recipe_and_input_set(&plan)?;
        self.require_same_candidate_topology_and_terminal_service_install(&plan, topology)?;
        let service_install_result_ref = self.load_terminal_service_install_ref_from_plan(&plan)?;
        let topology_certification_ref = topology.artifact_ref().clone();
        Ok(VerifiedPowerShutdownPlanV1 {
            verified_plan: plan,
            topology_certification_ref,
            service_install_result_ref,
        })
    }
}

pub trait PowerShutdownInstalledIdentityPortV1: Send + Sync {
    fn verify_control_service_peer_and_scm_identity(
        &self,
        plan: &VerifiedPowerShutdownPlanV1,
    ) -> Result<WindowsSidV1, PowerShutdownErrorV1>;
}

pub trait PowerShutdownDurableControlStoreV1: Send + Sync {
    fn acquire_exact_attempt_lock(
        &self,
        plan: &VerifiedPowerShutdownPlanV1,
    ) -> Result<(), PowerShutdownErrorV1>;
    fn load_intent_exact(
        &self,
        expected: &ArtifactRefV1,
    ) -> Result<Vec<u8>, PowerShutdownErrorV1>;
    fn create_or_adopt_dispatch_marker(
        &self,
        command: &PowerShutdownDispatchOnceCommandV1,
    ) -> Result<ArtifactRefV1, PowerShutdownErrorV1>;
    fn mark_attempt_consumed(
        &self,
        execution_attempt_id: UuidV1,
        marker_ref: &ArtifactRefV1,
    ) -> Result<(), PowerShutdownErrorV1>;
}

pub trait WindowsShutdownApiPortV1: Send + Sync {
    fn initiate_system_shutdown_ex_once(
        &self,
        command: &PowerShutdownDispatchOnceCommandV1,
    ) -> Result<u32, PowerShutdownErrorV1>;
}

pub struct PowerShutdownControlBrokerAuthorityV1<'a> {
    installed_identity: &'a dyn PowerShutdownInstalledIdentityPortV1,
    durable_control_store: &'a dyn PowerShutdownDurableControlStoreV1,
    shutdown_api: &'a dyn WindowsShutdownApiPortV1,
    expected_control_service_sid: WindowsSidV1,
    expected_control_pipe: &'static str,
}

pub struct PowerShutdownDispatchObservationV1 {
    marker_ref: ArtifactRefV1,
    win32_return_code: u32,
    api_call_may_have_occurred: bool,
}

pub enum PowerShutdownErrorV1 {
    PlanNotVerified,
    SessionNotAuthenticated,
    AttemptMismatch,
    PrefixNotPreShutdownCommitted,
    IntentNotDurable,
    OwnerTokenMismatch,
    DispatchAlreadyCommitted,
    CommandConsumed,
    ApiRejected,
    Storage,
}

impl<'a> PowerShutdownControlBrokerAuthorityV1<'a> {
    pub fn compose(
        installed_identity: &'a dyn PowerShutdownInstalledIdentityPortV1,
        durable_control_store: &'a dyn PowerShutdownDurableControlStoreV1,
        shutdown_api: &'a dyn WindowsShutdownApiPortV1,
        expected_control_service_sid: WindowsSidV1,
    ) -> Result<Self, PowerShutdownErrorV1> {
        verify_fixed_control_broker_identity_pipe_acl_and_privilege(
            installed_identity,
            &expected_control_service_sid,
            r"\\.\pipe\EnterprisePlatform\EPAuthorityControlV1",
        )?;
        Ok(Self {
            installed_identity,
            durable_control_store,
            shutdown_api,
            expected_control_service_sid,
            expected_control_pipe: r"\\.\pipe\EnterprisePlatform\EPAuthorityControlV1",
        })
    }

    pub fn open_authenticated_session(
        &mut self,
        plan: &VerifiedPowerShutdownPlanV1,
    ) -> Result<VerifiedPowerShutdownControlBrokerSessionV1, PowerShutdownErrorV1> {
        self.require_installed_control_identity_and_exact_attempt_lock(plan)?;
        self.open_fresh_owner_token_session_for_plan(plan)
    }

    pub fn load_persisted_intent(
        &mut self,
        plan: &VerifiedPowerShutdownPlanV1,
        prefix: &PowerShutdownContinuationStatePrefixV1,
    ) -> Result<VerifiedPowerShutdownDispatchCommitIntentV1, PowerShutdownErrorV1> {
        self.require_exact_pre_shutdown_committed_prefix(plan, prefix)?;
        let intent = self.typed_load_plan_fixed_intent_create_new_bytes(plan)?;
        self.require_intent_exact_matches_plan_prefix_and_permanent_object(plan, prefix, &intent)?;
        Ok(intent)
    }

    pub fn prepare_dispatch(
        &mut self,
        plan: &VerifiedPowerShutdownPlanV1,
        session: VerifiedPowerShutdownControlBrokerSessionV1,
    ) -> Result<PowerShutdownDispatchPreparationV1, PowerShutdownErrorV1> {
        self.require_live_unconsumed_session_exact_matches_plan(plan, &session)?;
        self.consume_session_into_private_preparation(plan, session)
    }

    pub fn freeze_once_command(
        &mut self,
        preparation: PowerShutdownDispatchPreparationV1,
        persisted_intent: &VerifiedPowerShutdownDispatchCommitIntentV1,
        prefix: &PowerShutdownContinuationStatePrefixV1,
    ) -> Result<PowerShutdownDispatchOnceCommandV1, PowerShutdownErrorV1> {
        self.require_exact_pre_shutdown_committed_prefix_for_intent(prefix, persisted_intent)?;
        self.require_preparation_matches_persisted_intent(&preparation, persisted_intent)?;
        self.consume_preparation_and_freeze_owner_token_once(preparation, persisted_intent)
    }

    pub fn dispatch_once(
        &mut self,
        command: PowerShutdownDispatchOnceCommandV1,
    ) -> Result<PowerShutdownDispatchObservationV1, PowerShutdownErrorV1> {
        self.require_no_fixed_dispatch_marker_or_consumed_owner_token(&command)?;
        self.consume_command_persist_marker_then_call_win32_once(command)
    }
}

pub enum PowerShutdownDispatchApiCallCommittedPurposeV1 { PowerShutdownDispatchApiCallCommitted }
pub struct PowerShutdownDispatchApiCallCommittedV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownDispatchApiCallCommittedPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub dispatch_commit_intent_ref: ArtifactRefV1,
    pub dispatch_owner_token_sha256: Sha256Digest,
    pub pre_shutdown_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub api_call_committed_monotonic_tick_ms: u64,
    pub requested_at_unix_ms: i64,
    pub api_function: String,
    pub machine_name: Option<String>,
    pub shutdown_message: String,
    pub timeout_seconds: u32,
    pub force_apps_closed: bool,
    pub reboot_after_shutdown: bool,
    pub shutdown_reason_code: u32,
}

pub struct PowerShutdownInlineControlBytesV1 {
    pub media_type: String,
    pub sha256: Sha256Digest,
    pub size_bytes: u64,
    pub base64_bytes: String,
}
pub enum PowerShutdownDispatchAcknowledgementPurposeV1 { PowerShutdownDispatchAcknowledgement }
pub struct PowerShutdownDispatchAcknowledgementV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownDispatchAcknowledgementPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub carrier_test_id: TestIdV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub dispatch_commit_intent_ref: ArtifactRefV1,
    pub dispatch_api_call_committed_ref: ArtifactRefV1,
    pub pre_shutdown_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub acknowledgement_committed_monotonic_tick_ms: u64,
    pub user32_event_record_id: u64,
    pub user32_event_record_bytes: PowerShutdownInlineControlBytesV1,
    pub ups_outlet_cycle_command_ack: UpsOutletCycleCommandAckV1,
}

pub enum PowerShutdownUpsTriggerReadbackPurposeV1 { PowerShutdownUpsTriggerReadback }

pub struct PowerShutdownUpsTriggerReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownUpsTriggerReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub ups_identity_readback_ref: ArtifactRefV1,
    pub ups_power_write_cache_policy_ref: ArtifactRefV1,
    pub previous_status_readback_ref: ArtifactRefV1,
    pub trigger_status_readback_ref: ArtifactRefV1,
    pub ups_serial_number: String,
    pub ups_firmware_revision: String,
    pub self_test_passed: bool,
    pub communication_healthy: bool,
    pub ac_input_lost: bool,
    pub on_battery: bool,
    pub output_power_maintained: bool,
    pub previous_remaining_runtime_seconds: u64,
    pub trigger_remaining_runtime_seconds: u64,
    pub trigger_threshold_seconds: u64,
    pub authenticated_ups_event_log_ref: ArtifactRefV1,
    pub ac_lost_at_unix_ms: i64,
    pub threshold_reached_at_unix_ms: i64,
    pub shutdown_authorized_at_unix_ms: i64,
}

pub enum PowerShutdownEventReadbackPurposeV1 { PowerShutdownEventReadback }
pub enum PowerShutdownEventKindV1 { PlannedOperatingSystemShutdown }

pub struct PowerShutdownEventReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownEventReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub dispatch_api_call_committed_ref: ArtifactRefV1,
    pub dispatch_acknowledgement_ref: ArtifactRefV1,
    pub pre_shutdown_boot_id: String,
    pub event_kind: PowerShutdownEventKindV1,
    pub event_provider: String,
    pub event_id: u32,
    pub shutdown_reason_code: u32,
    pub shutdown_comment: String,
    pub controlled_outlet_group_id: String,
    pub protected_host_power_path_sha256: Sha256Digest,
    pub outlet_cycle_command_id: UuidV1,
    pub outlet_cycle_command_ack_ref: ArtifactRefV1,
    pub outlet_off_delay_seconds: u64,
    pub restore_output_on_ac_return: bool,
    pub outlet_cycle_command_ack_at_unix_ms: i64,
    pub initiating_executable_path: String,
    pub initiating_executable_sha256: Sha256Digest,
    pub event_record_id: u64,
    pub event_record_bytes_ref: ArtifactRefV1,
    pub shutdown_lifecycle_event_log_ref: ArtifactRefV1,
    pub kernel_general_shutdown_event_record_id: u64,
    pub eventlog_clean_stop_event_record_id: u64,
    pub kernel_general_start_event_record_id: u64,
    pub eventlog_clean_start_event_record_id: u64,
    pub intervening_shutdown_request_count: u32,
    pub shutdown_abort_event_count: u32,
    pub unexpected_shutdown_event_count: u32,
    pub requested_at_unix_ms: i64,
    pub recorded_at_unix_ms: i64,
}

pub enum PowerShutdownAuthorityRecoveryProofPurposeV1 { PowerShutdownAuthorityRecoveryProof }
pub struct PowerShutdownAuthorityRecoveryProofV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownAuthorityRecoveryProofPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub authority_barrier_id: UuidV1,
    pub host_fingerprint_sha256: Sha256Digest,
    pub data_hdd_unlock_readback_ref: ArtifactRefV1,
    pub postgres_recovery_completed: bool,
    pub recovered_postgres_checkpoint_ref: ArtifactRefV1,
    pub recovered_postgres_lsn: u64,
    pub attachment_manifest_ref: ArtifactRefV1,
    pub audit_checkpoint_ref: ArtifactRefV1,
    pub outbox_checkpoint_ref: ArtifactRefV1,
    pub lost_accepted_command_count: u64,
    pub duplicate_effect_count: u64,
    pub unexplained_obligation_count: u64,
    pub issued_at_unix_ms: i64,
}

pub enum PowerShutdownRestartConsistencyReadbackPurposeV1 {
    PowerShutdownRestartConsistencyReadback,
}

pub struct PowerShutdownRestartConsistencyReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownRestartConsistencyReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub host_fingerprint_sha256: Sha256Digest,
    pub authority_barrier_id: UuidV1,
    pub pre_shutdown_boot_id: String,
    pub post_restart_boot_id: String,
    pub runtime_ssd_mounted: bool,
    pub data_hdd_mounted: bool,
    pub runtime_ssd_dirty_bit_clear: bool,
    pub data_hdd_dirty_bit_clear: bool,
    pub data_hdd_unlock_readback_ref: ArtifactRefV1,
    pub postgres_recovery_completed: bool,
    pub authority_recovery_proof_ref: ArtifactRefV1,
    pub recovered_postgres_checkpoint_ref: ArtifactRefV1,
    pub recovered_postgres_lsn: u64,
    pub attachment_manifest_ref: ArtifactRefV1,
    pub audit_checkpoint_ref: ArtifactRefV1,
    pub outbox_checkpoint_ref: ArtifactRefV1,
    pub ups_trigger_readback_ref: ArtifactRefV1,
    pub ups_outlet_cycle_event_log_ref: ArtifactRefV1,
    pub ups_output_off_observed: bool,
    pub ups_output_off_at_unix_ms: i64,
    pub ups_ac_input_restored: bool,
    pub ups_output_online: bool,
    pub ups_restored_at_unix_ms: i64,
    pub lost_accepted_command_count: u64,
    pub duplicate_effect_count: u64,
    pub unexplained_obligation_count: u64,
    pub completed_at_unix_ms: i64,
}

pub enum PowerShutdownSuccessPersistenceDraftPurposeV1 { PowerShutdownSuccessPersistenceDraft }
pub struct PowerShutdownSuccessPersistenceDraftV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownSuccessPersistenceDraftPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub event_readback: PowerShutdownEventReadbackV1,
    pub authority_recovery_proof: PowerShutdownAuthorityRecoveryProofV1,
    pub restart_readback: PowerShutdownRestartConsistencyReadbackV1,
}
pub enum PowerShutdownSuccessPersistenceCommandPurposeV1 { PowerShutdownSuccessPersistenceCommand }
pub struct PowerShutdownSuccessPersistenceCommandV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownSuccessPersistenceCommandPurposeV1,
    pub draft: PowerShutdownSuccessPersistenceDraftV1,
    pub broker_accepted_monotonic_tick_ms: u64,
}

pub enum PowerShutdownContinuationDisarmReadbackPurposeV1 {
    PowerShutdownContinuationDisarmReadback,
}

pub struct PowerShutdownContinuationDisarmReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationDisarmReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub service_name: String,
    pub service_stopped: bool,
    pub service_registration_present: bool,
    pub service_start_mode_auto: bool,
    pub activation_parameter_absent: bool,
    pub verified_at_unix_ms: i64,
}

pub enum PowerShutdownContinuationReadinessStageV1 {
    DispatchApi, DispatchAcknowledgement, PostbootContext,
    DataHddUnlockAndMount, JournalAndState, PostgresRecovery,
    DurableCuts, UpsRestored, SuccessPersistence,
}
pub enum PowerShutdownContinuationFailureCodeV1 {
    DispatchApiRejected,
    DispatchOwnerSessionLostBeforeApi,
    DispatchAcknowledgementDeadlineElapsed,
    DispatchAcknowledgementAbsentAfterBootChange,
    PostbootContextChanged,
    DataHddUnlockAndMountFailed,
    JournalAndStateFailed,
    PostgresRecoveryFailed,
    DurableCutsFailed,
    UpsRestoredFailed,
    SuccessPersistenceFailed,
}
pub enum PowerShutdownPostbootResumeControllerPurposeV1 { PowerShutdownPostbootResumeController }
pub enum PowerShutdownPostbootResumeRecordKindV1 {
    Initialized, StageEntered, ResumeAuthorized,
    SuccessCommandDurable, SuccessPersistenceCompleted, FailureDurable,
}
pub struct PowerShutdownPostbootResumeControllerHeaderV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownPostbootResumeControllerPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub pre_shutdown_boot_id: String,
    pub postboot_boot_id: String,
    pub boot_monotonic_source_id: String,
    pub first_postboot_monotonic_tick_ms: u64,
    pub deadline_monotonic_tick_ms: u64,
    pub maximum_resume_generation: u8,
}
pub struct PowerShutdownPostbootResumeControllerRecordV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownPostbootResumeControllerPurposeV1,
    pub sequence: u16,
    pub previous_frame_sha256: Sha256Digest,
    pub record_kind: PowerShutdownPostbootResumeRecordKindV1,
    pub stage: PowerShutdownContinuationReadinessStageV1,
    pub resume_generation: u8,
    pub observed_monotonic_tick_ms: u64,
    pub success_persistence_command_sha256: Option<Sha256Digest>,
}
#[serde(tag = "timing_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerShutdownContinuationFailureTimingV1 {
    ComparableWindow {
        observed_boot_id: String,
        boot_monotonic_source_id: String,
        deadline_monotonic_tick_ms: u64,
        observed_monotonic_tick_ms: u64,
    },
    BootContextChange {
        previous_boot_id: String,
        previous_boot_monotonic_source_id: String,
        previous_deadline_monotonic_tick_ms: u64,
        previous_last_monotonic_tick_ms: u64,
        observed_boot_id: String,
        observed_boot_monotonic_source_id: String,
        observed_first_monotonic_tick_ms: u64,
    },
}
#[serde(tag = "acknowledgement_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PowerShutdownDispatchAcknowledgementObservationV1 {
    NotApplicable,
    Incomplete { user32_1074_observed: bool, ups_schedule_ack_observed: bool },
    Complete,
}
pub enum PowerShutdownContinuationFailureReadbackPurposeV1 { PowerShutdownContinuationFailureReadback }
pub struct PowerShutdownContinuationFailureReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationFailureReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub dispatch_api_call_committed_ref: Option<ArtifactRefV1>,
    pub dispatch_acknowledgement_ref: Option<ArtifactRefV1>,
    pub failed_stage: PowerShutdownContinuationReadinessStageV1,
    pub failure_code: PowerShutdownContinuationFailureCodeV1,
    pub windows_error_code: u32,
    pub dispatch_acknowledgement: PowerShutdownDispatchAcknowledgementObservationV1,
    pub timing: PowerShutdownContinuationFailureTimingV1,
    pub observed_at_unix_ms: i64,
}
pub enum PowerShutdownContinuationFailureCleanupReadbackPurposeV1 { PowerShutdownContinuationFailureCleanupReadback }
pub struct PowerShutdownContinuationFailureCleanupReadbackV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownContinuationFailureCleanupReadbackPurposeV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub execution_attempt_id: UuidV1,
    pub continuation_id: UuidV1,
    pub staging_plan_ref: ArtifactRefV1,
    pub failure_readback_ref: ArtifactRefV1,
    pub service_stopped: bool,
    pub service_registration_present: bool,
    pub service_start_mode_auto: bool,
    pub activation_parameter_absent: bool,
    pub verified_at_unix_ms: i64,
}

pub enum PowerShutdownEvidencePurposeV1 { PowerShutdownEvidence }

pub struct PowerShutdownEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: PowerShutdownEvidencePurposeV1,
    pub context: ReleasePhysicalEvidenceContextV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub continuation_plan_ref: ArtifactRefV1,
    pub continuation_id: UuidV1,
    pub ups_identity_readback_ref: ArtifactRefV1,
    pub ups_power_write_cache_policy_ref: ArtifactRefV1,
    pub remaining_time_trigger_seconds: u64,
    pub ups_trigger_readback_ref: ArtifactRefV1,
    pub quiesce_readback_ref: ArtifactRefV1,
    pub shutdown_event_ref: ArtifactRefV1,
    pub pre_shutdown_boot_id: String,
    pub post_restart_boot_id: String,
    pub restart_consistency_ref: ArtifactRefV1,
    pub outcome: ReleaseCarrierOutcomeV1,
}

pub type PowerShutdownEvidenceV1 =
    SignedBusinessArtifactV1<PowerShutdownEvidencePayloadV1>;

pub enum F57OfflineSchemaManifestPurposeV1 { OfflineSchemaManifest }
pub enum F57OfflineSchemaClosureRootV1 { ReleaseCertificate }

pub struct F57OfflineArtifactSchemaBindingV1 {
    pub artifact_kind: String,
    pub discriminator: String,
    pub media_type: String,
}

pub struct F57OfflineSchemaDescriptorV1 {
    pub schema_id: String,
    pub artifact_bindings: Vec<F57OfflineArtifactSchemaBindingV1>,
    pub imports: Vec<String>,
    pub relative_path: String,
    pub media_type: String,
    pub sha256: Sha256Digest,
    pub size_bytes: u64,
}

pub struct F57OfflineSchemaManifestPayloadV1 {
    pub schema_version: u32,
    pub purpose: F57OfflineSchemaManifestPurposeV1,
    pub closure_root: F57OfflineSchemaClosureRootV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub candidate_identity_sha256: Sha256Digest,
    pub finalization_attempt_id: UuidV1,
    pub schemas: Vec<F57OfflineSchemaDescriptorV1>,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type F57OfflineSchemaManifestV1 =
    SignedBusinessArtifactV1<F57OfflineSchemaManifestPayloadV1>;

pub struct L3CandidateEvidencePayloadV1 {
    pub schema_version: u32,
    pub purpose: L3EvidencePurposeV1,
    pub candidate: CandidateIdentityV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub candidate_identity_sha256: Sha256Digest,
    pub gate_run_id: UuidV1,
    pub target_gate: TargetGateV1,
    pub final_l2_evidence_ref: ArtifactRefV1,
    pub test_results: Vec<TestResultRefV1>,
    pub client_conformance_refs: Vec<ArtifactRefV1>,
    pub carrier_refs: Vec<ArtifactRefV1>,
    pub objective_closures: Vec<ObjectiveClosureBindingV1>,
    pub run_journal_checkpoint_ref: ArtifactRefV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type L3CandidateEvidenceV1 =
    SignedBusinessArtifactV1<L3CandidateEvidencePayloadV1>;

pub enum ReleaseCertificatePurposeV1 { ReleaseCertificate }
pub enum ReleaseCertificationVerdictV1 { ReleaseCertified }

use ep_foundation::GateReceiptRefV1;

pub struct ReleaseCertificatePayloadV1 {
    pub schema_version: u32,
    pub purpose: ReleaseCertificatePurposeV1,
    pub verdict: ReleaseCertificationVerdictV1,
    pub finalization_attempt_id: UuidV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub candidate_identity_sha256: Sha256Digest,
    pub gate_run_id: UuidV1,
    pub l3_evidence_ref: ArtifactRefV1,
    pub certified_data_hdd_root: G6CertifiedDataHddRootV1,
    pub runtime_topology_certification_ref: ArtifactRefV1,
    pub prerequisite_receipts: Vec<GateReceiptRefV1>,
    pub delivery_registry_sha256: Sha256Digest,
    pub first_due_map_sha256: Sha256Digest,
    pub test_results: Vec<TestResultRefV1>,
    pub run_journal_checkpoint_ref: ArtifactRefV1,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

pub type ReleaseCertificateV1 = SignedBusinessArtifactV1<ReleaseCertificatePayloadV1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductionActivationProfileV1 { SingleDiskDegradedProduction }

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductionActivationStateV1 {
    RequestCommitted,
    LiveReadbackBound,
    FailedHeld,
    RetryCommitted,
    Activated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductionActivationFailureCodeV1 {
    CollectorUnavailable,
    LiveTopologyDrift,
    RuntimeDeploymentDrift,
    InstalledComponentDrift,
    StorageSafeguardDrift,
    CustomerAcceptanceExpiredOrRevoked,
    ConcurrentStateChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SingleDiskAcceptedRiskV1 {
    SingleDataHddNoRaid,
    NoHighAvailability,
    RansomwareRecoveryDependsOnExternalAppendOnlyAndOfflineRotation,
    ManualRecoveryMayBeRequired,
    LocalAiDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SingleDiskDegradedProductionAcceptanceV1 {
    pub schema_version: u32,
    pub acceptance_id: UuidV1,
    pub deployment_id: UuidV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub release_certificate_ref: ArtifactRefV1,
    pub profile: ProductionActivationProfileV1,
    pub certified_concurrent_users: u32,
    pub accepted_risks: Vec<SingleDiskAcceptedRiskV1>,
    pub safeguard_closure_refs: Vec<ArtifactRefV1>,
    pub first_approver_principal_id: PrincipalIdV1,
    pub second_approver_principal_id: PrincipalIdV1,
    pub accepted_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionActivationBindingV1 {
    pub activation_id: UuidV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub candidate_run: CandidateRunIdentityV1,
    pub candidate_manifest_ref: ArtifactRefV1,
    pub release_certificate_ref: ArtifactRefV1,
    pub runtime_topology_certification_ref: ArtifactRefV1,
    pub generation_observed_selection_ref: ArtifactRefV1,
    pub observed_activation_attempt_id: UuidV1,
    pub customer_acceptance_ref: ArtifactRefV1,
    pub profile: ProductionActivationProfileV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "event_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum ProductionActivationEventV1 {
    RequestCommitted {
        request_id: UuidV1,
        safeguard_closure_sha256: Sha256Digest,
    },
    LiveReadbackBound {
        live_topology_readback_ref: ArtifactRefV1,
        live_runtime_deployment_readback_set_ref: ArtifactRefV1,
        live_installed_component_readback_ref: ArtifactRefV1,
        live_storage_safeguard_readback_ref: ArtifactRefV1,
    },
    FailedHeld {
        failure_code: ProductionActivationFailureCodeV1,
        failure_readback_ref: ArtifactRefV1,
    },
    RetryCommitted {
        retry_ordinal: u32,
        prior_failure_record_sha256: Sha256Digest,
    },
    Activated {
        activated_at_unix_ms: i64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionActivationRecordV1 {
    pub schema_version: u32,
    pub binding: ProductionActivationBindingV1,
    pub sequence: u64,
    pub expected_prior_cas_version: u64,
    pub committed_cas_version: u64,
    pub previous_record_sha256: Option<Sha256Digest>,
    pub event: ProductionActivationEventV1,
    pub recorded_at_unix_ms: i64,
}

pub struct VerifiedProductionActivationRequestV1(/* private complete proof */);
pub struct VerifiedProductionActivationAttemptV1(/* private codec/store result */);
pub struct VerifiedProductionActivationReadyV1(/* private LIVE_READBACK_BOUND proof */);
pub struct VerifiedGenesisProductionAdmissionDraftV1(/* private exact admission inputs; no API generation/time */);
pub struct VerifiedProductionActivatedV1(/* private atomically committed activation + genesis admission */);

#[async_trait::async_trait]
pub trait ProductionActivationStoreV1: Send + Sync {
    async fn begin_or_adopt(
        &self,
        request: &VerifiedProductionActivationRequestV1,
    ) -> Result<VerifiedProductionActivationAttemptV1, ProductionActivationErrorV1>;
    async fn bind_live_readback_cas(
        &self,
        attempt: &VerifiedProductionActivationAttemptV1,
        event: ProductionActivationEventV1,
    ) -> Result<VerifiedProductionActivationAttemptV1, ProductionActivationErrorV1>;
    async fn hold_failed_cas(
        &self,
        attempt: &VerifiedProductionActivationAttemptV1,
        failure_code: ProductionActivationFailureCodeV1,
        failure_readback_ref: &ArtifactRefV1,
    ) -> Result<VerifiedProductionActivationAttemptV1, ProductionActivationErrorV1>;
    async fn resume_same_request_cas(
        &self,
        attempt: &VerifiedProductionActivationAttemptV1,
        expected_failure_record_sha256: Sha256Digest,
    ) -> Result<VerifiedProductionActivationAttemptV1, ProductionActivationErrorV1>;
}

#[async_trait::async_trait]
pub trait ProductionActivationAdmissionCommitPortV1: Send + Sync {
    async fn commit_activation_and_genesis_admission_cas(
        &self,
        activation_ready: &VerifiedProductionActivationReadyV1,
        admission: &VerifiedGenesisProductionAdmissionDraftV1,
        committed_at_unix_ms: i64,
    ) -> Result<VerifiedProductionActivatedV1, ProductionActivationErrorV1>;
}

pub enum ProductionActivationErrorV1 {
    ReleaseCertificateInvalid,
    CandidateOrGenerationMismatch,
    TopologyCertificationMismatch,
    CustomerAcceptanceInvalid,
    SafeguardClosureInvalid,
    FreshLiveReadbackMismatch,
    AlreadyActivatedConflict,
    CasConflict,
    StoreIntegrity,
    StoreUnavailable,
}

pub struct VerifiedReleaseDueResultV1 {
    test_result_ref: TestResultRefV1,
    verified_envelope: VerifiedSignedEnvelopeBytesV1,
}

pub struct VerifiedExactReleaseCarrierResultSetV1 {
    rows_by_recipe_ordinal: [VerifiedBusinessArtifactV1<ReleaseCarrierResultPayloadV1>; 6],
    candidate_run: CandidateRunIdentityV1,
    canonical_set_sha256: Sha256Digest,
}

pub struct VerifiedExactPrerequisiteReceiptSetV1 {
    rows_g0_through_g5: [VerifiedBusinessArtifactV1<GateReceiptPayloadV1>; 6],
    candidate_run: CandidateRunIdentityV1,
    canonical_set_sha256: Sha256Digest,
}

pub struct VerifiedExactReleaseDueResultSetV1 {
    rows_by_test_id: BTreeMap<TestIdV1, VerifiedReleaseDueResultV1>,
    delivery_registry_sha256: Sha256Digest,
    canonical_set_sha256: Sha256Digest,
}

pub struct VerifiedExactObjectiveClosureSetV1 {
    rows_by_objective_ordinal: [ObjectiveClosureBindingV1; 4],
    candidate_run: CandidateRunIdentityV1,
    canonical_set_sha256: Sha256Digest,
}

pub struct VerifiedG6CertifiedDataHddRootV1 {
    value: G6CertifiedDataHddRootV1,
    p340_residency_ref: ArtifactRefV1,
    filesystem_geometry_ref: ArtifactRefV1,
}

pub struct VerifiedDeliveryRegistryV1 {
    value: DeliveryRegistryV1,
    registry_sha256: Sha256Digest,
    first_due_map_sha256: Sha256Digest,
}

impl VerifiedDeliveryRegistryV1 {
    pub fn sha256(&self) -> Sha256Digest {
        self.registry_sha256.clone()
    }

    pub fn first_due_map_sha256(&self) -> Sha256Digest {
        self.first_due_map_sha256.clone()
    }
}

pub struct VerifiedEvidenceEnvelopeFinalizationV1 {
    artifact_kind: EvidenceEnvelopeKindV1,
    finalization_attempt_id: UuidV1,
    frozen_input_checkpoint_ref: ArtifactRefV1,
    issued_at_unix_ms: i64,
    expires_at_unix_ms: i64,
}

// The authority creates this only after exact journal lookup returns one matching row;
// zero or duplicate rows map to ReleaseGateErrorV1::FinalizationInvalid.
pub struct VerifiedExactReleaseCertificateFinalizationSetV1 {
    rows: [VerifiedEvidenceEnvelopeFinalizationV1; 1],
}

pub struct L3AggregateInputV1 {
    verified_signed_candidate: VerifiedBusinessArtifactV1<ReleaseCandidatePayloadV1>,
    verified_final_l2_evidence: VerifiedBusinessArtifactV1<L2CandidateEvidencePayloadV1>,
    verified_l3_evidence: VerifiedBusinessArtifactV1<L3CandidateEvidencePayloadV1>,
    verified_six_carrier_results: VerifiedExactReleaseCarrierResultSetV1,
    verified_g0_through_g5_receipts: VerifiedExactPrerequisiteReceiptSetV1,
    verified_release_due_results: VerifiedExactReleaseDueResultSetV1,
    verified_objective_closures: VerifiedExactObjectiveClosureSetV1,
    verified_certified_data_hdd_root: VerifiedG6CertifiedDataHddRootV1,
    verified_runtime_topology_certification: VerifiedRuntimeTopologyCertificationV1,
    verified_artifact_signer_registry: VerifiedBusinessArtifactV1<F57ArtifactSignerRegistryPayloadV1>,
    delivery_registry: VerifiedDeliveryRegistryV1,
    verified_release_certificate_finalization: VerifiedExactReleaseCertificateFinalizationSetV1,
}

pub struct ReleaseGateAuthorityV1<S: DetachedCmsSignerV1> {
    artifact_verifier: ArtifactVerifierV1,
    evidence_signing_authorizer: F57EvidenceSigningAuthorizerV1,
    detached_cms_signer: S,
    /* private object/input/journal/finalization stores */
}

impl<S: DetachedCmsSignerV1> ReleaseGateAuthorityV1<S> {
    pub async fn load_and_build_aggregate(
        &self,
        candidate: VerifiedBusinessArtifactV1<ReleaseCandidatePayloadV1>,
        final_l3: VerifiedBusinessArtifactV1<L3CandidateEvidencePayloadV1>,
    ) -> Result<L3AggregateInputV1, ReleaseGateErrorV1> {
        let verified_final_l2_evidence = self.load_final_l2_from_l3(&candidate, &final_l3)?;
        let verified_six_carrier_results = self.verify_exact_six_carriers(&candidate, &verified_final_l2_evidence, &final_l3)?;
        let verified_g0_through_g5_receipts = self.verify_exact_g0_through_g5_receipts(&candidate, &final_l3)?;
        let verified_release_due_results = self.verify_exact_185_due_results(&candidate, &verified_final_l2_evidence, &final_l3)?;
        let verified_objective_closures = self.verify_exact_four_objectives(&verified_final_l2_evidence, &final_l3)?;
        let verified_certified_data_hdd_root = self.verify_certified_data_hdd_root(&candidate, &verified_six_carrier_results)?;
        let verified_runtime_topology_certification = self
            .reconstruct_runtime_topology_certification(&candidate, &verified_six_carrier_results)
            .await?;
        let verified_artifact_signer_registry = self.load_candidate_signer_registry(&candidate)?;
        let delivery_registry = self.verify_delivery_registry_and_first_due_map(&candidate, &verified_release_due_results)?;
        let verified_release_certificate_finalization = self.load_or_freeze_exact_release_certificate_finalization_set(
            &candidate,
            &final_l3,
            &verified_six_carrier_results,
            &verified_g0_through_g5_receipts,
            &verified_release_due_results,
            &verified_objective_closures,
            &verified_certified_data_hdd_root,
            &verified_runtime_topology_certification,
        )?;
        Ok(L3AggregateInputV1 {
            verified_signed_candidate: candidate,
            verified_final_l2_evidence,
            verified_l3_evidence: final_l3,
            verified_six_carrier_results,
            verified_g0_through_g5_receipts,
            verified_release_due_results,
            verified_objective_closures,
            verified_certified_data_hdd_root,
            verified_runtime_topology_certification,
            verified_artifact_signer_registry,
            delivery_registry,
            verified_release_certificate_finalization,
        })
    }
}

impl L3AggregateInputV1 {
    fn verified_evidence_finalization(
        &self,
        kind: EvidenceEnvelopeKindV1,
    ) -> Result<&VerifiedEvidenceEnvelopeFinalizationV1, ReleaseGateErrorV1> {
        if kind != EvidenceEnvelopeKindV1::ReleaseCertificate {
            return Err(ReleaseGateErrorV1::FinalizationInvalid);
        }
        let row = &self.verified_release_certificate_finalization.rows[0];
        if row.artifact_kind != EvidenceEnvelopeKindV1::ReleaseCertificate {
            return Err(ReleaseGateErrorV1::FinalizationInvalid);
        }
        Ok(row)
    }
}

pub enum ReleaseGateErrorV1 {
    CandidateInvalid,
    FinalL2Invalid,
    L3Invalid,
    CarrierResultSetInvalid,
    PrerequisiteReceiptSetInvalid,
    ReleaseDueResultSetInvalid,
    ObjectiveClosureInvalid,
    CertifiedDataHddRootInvalid,
    RuntimeTopologyCertificationInvalid,
    SignerRegistryInvalid,
    DeliveryRegistryInvalid,
    JournalPrefixInvalid,
    FinalizationInvalid,
    ExpiryInvalid,
    SignatureInvalid,
    StorageConflict,
}

impl<S: DetachedCmsSignerV1> ReleaseGateAuthorityV1<S> {
pub fn certify_release(&self, input: L3AggregateInputV1) -> Result<ReleaseCertificateV1, ReleaseGateErrorV1> {
    require_final_l2_same_candidate(&input)?;
    require_release_due_185_of_185(&input)?;
    require_candidate_artifact_set_exact_five(&input)?;
    require_release_recipe_result_set_exact_six(&input)?;
    require_no_unknown_expired_or_enabled_deferred(&input)?;
    require_final_l2_and_l3_exact_chain(&input)?;
    require_verified_data_hdd_binding_upgraded_by_exact_p340_residency_and_geometry(&input)?;
    require_runtime_topology_certification_matches_candidate_declaration_and_p340(&input)?;
    require_final_l2_exact_four_closed_objectives(&input)?;
    require_l3_objective_vector_byte_equal_and_generations_current(&input)?;
    require_certificate_revalidates_objectives_through_l3(&input)?;
    require_journal_checkpoints_strictly_extend(&input)?;
    require_certificate_finalization_matches_frozen_inputs_including_objectives(&input)?;
    let finalization = input.verified_evidence_finalization(EvidenceEnvelopeKindV1::ReleaseCertificate)?;
    require_exact_evidence_finalization_kind(finalization, "RELEASE_CERTIFICATE")?;
    let finalization_attempt_id = finalization.finalization_attempt_id;
    let candidate_manifest_ref = input.verified_signed_candidate.artifact_ref().clone();
    let candidate_identity_sha256 = input.verified_signed_candidate.payload().identity.sha256_jcs();
    let gate_run_id = input.verified_signed_candidate.payload().gate_run_id;
    let l3_evidence_ref = input.verified_l3_evidence.artifact_ref().clone();
    let certified_data_hdd_root = input.verified_certified_data_hdd_root.value.clone();
    let runtime_topology_certification_ref = input.verified_runtime_topology_certification.artifact_ref().clone();
    let prerequisite_receipts = canonical_typed_g0_through_g5_gate_receipt_refs(&input)?;
    let test_results = canonical_release_test_results(&input)?;
    let run_journal_checkpoint_ref = finalization.frozen_input_checkpoint_ref.clone();
    require_gate_receipt_refs_typed_load_matching_gate_and_run(&prerequisite_receipts, gate_run_id)?;
    require_results_same_candidate_and_gate_run(&test_results, candidate_manifest_ref.sha256, gate_run_id)?;
    let issued_at_unix_ms = finalization.issued_at_unix_ms;
    let expires_at_unix_ms = finalization.expires_at_unix_ms;
    require_frozen_release_expiry_equals_aggregate_minimum(&input, issued_at_unix_ms, expires_at_unix_ms)?;
    let payload = ReleaseCertificatePayloadV1 {
        schema_version: 1,
        purpose: ReleaseCertificatePurposeV1::ReleaseCertificate,
        verdict: ReleaseCertificationVerdictV1::ReleaseCertified,
        finalization_attempt_id,
        candidate_manifest_ref,
        candidate_identity_sha256,
        gate_run_id,
        l3_evidence_ref,
        certified_data_hdd_root,
        runtime_topology_certification_ref,
        prerequisite_receipts,
        delivery_registry_sha256: input.delivery_registry.sha256(),
        first_due_map_sha256: input.delivery_registry.first_due_map_sha256(),
        test_results,
        run_journal_checkpoint_ref,
        issued_at_unix_ms,
        expires_at_unix_ms,
    };
    let request = prepare_cms_signing_request_v1(payload, &self.evidence_signing_authorizer)
        .map_err(|_| ReleaseGateErrorV1::SignatureInvalid)?;
    sign_business_artifact_v1(&self.detached_cms_signer, request)
        .map_err(|_| ReleaseGateErrorV1::SignatureInvalid)
}
}
```

The preceding `release_evidence.rs` block is the complete production landing point for its release-local purpose/output-class enums, authority set, four non-P340 raw roots, installed-file/service/component readbacks, recovery outer/subattempt fields, staging plan/completion, carrier result, certified-HDD root, L3 payload and every signed alias; P340 remains imported from its separate owner, `ReleaseCarrierRecipeIdV1` from `carrier_contract`, `TargetGateV1` from the sole L2 owner, runtime deployment types from `ep-platform-runtime`, the six backup component IDs/placements from `ep-platform-backup`, and the fixed G0 evidence-signer broker row/readback from evidence trust. No imported enum or deployment nominal is redeclared. `WindowsAuthorityArtifactSetV1.runtime_deployment_closure_ref` exact-loads the graph-bijective carrier-polymorphic closure, `server_component_set_ref` exact-loads one strict content-addressed `WindowsServerComponentSetV1` with the six Task-11 rows/packages, and `production_admission_bypass_registry_ref` exact-loads the generated ten-row registry. The Authority-set signature transitively authenticates all three exact byte graphs. The service-install raw repeats all three refs, exact-loads one runtime deployment readback row for every active/deferred participant, installs/readbacks writer/signer/unlock services plus recovery/passphrase tools, exact-loads the G0 broker's complete install/readiness/DATA_HDD closure, and embeds an authenticated off-host target deployment readback whose host differs, shared admin/credential flags are false and Authority-host installation count is zero. `ReleaseWindowsServiceInstallEvidencePayloadV1.production_admission_bypass_registry_ref` must byte-equal the signed Authority-set ref and installed/embedded final-handle policy bytes; no ambient/latest file can control routing. Overlapping runtime/component rows must name the same artifact, service identity, capability digest and live process; two installations cannot satisfy one participant. Strict parsers deny unknown fields and exact-check purpose/media, canonical JCS, signature/trust row, recipe-conditioned raw cardinality, all context/binding/run/attempt relations and every displayed field formula before private verified wrappers are constructed. Builders accept only those wrappers and journal-frozen IDs/times. `xtask` imports these APIs and may not mirror a nominal, deserialize directly into a PASS wrapper or invent a serializer default. Compile/byte goldens cover every enum wire, root round-trip, unknown field and one-field mutation.

All `L3AggregateInputV1` fields are private and the type has no public literal, default, deserialize or test-fixture constructor in a production build. `ReleaseGateAuthorityV1::load_and_build_aggregate` is its sole builder: starting from the two private verified top-level wrappers and terminal authenticated checkpoint, it follows signed refs to typed-load Final L2, the exact six carrier results, G0…G5 receipts, all 185 release-due results, four objective closures, certified HDD root and the ephemeral topology certification reconstructed through the release authority. It cold-verifies the signer/delivery registries, freezes the certificate finalization, and selects the `RELEASE_CERTIFICATE_V1/NONE` signer internally. A raw ref/digest, live clock, caller signer, loose vector, direct field mutation, wrong wrapper, alternate registry or newly sampled finalization cannot enter the aggregate. Each failure maps one-to-one to the closed `ReleaseGateErrorV1`; there is no string/other fallback. Unit tests compile-fail every direct construction and mutate every typed proof, plus reject signer/clock/ref overrides.

This builder, the topology reconstruction helper, POWER coordinator and CLI gate path are async end-to-end and await the object-safe PostgreSQL activation-store port; none uses `block_on`, a nested runtime or a synchronous database facade. The real `db-pg` trait object is compiled through both POWER and release-gate composition tests, including crash/retry and store-unavailable propagation.

`ReleaseCandidatePurposeV1` and `ReleaseCertificatePurposeV1` are closed single-variant enums serialized exactly as `EP-F57-RELEASE-CANDIDATE-V1` and `EP-F57-RELEASE-CERTIFICATE-V1`; `ReleaseCertificationVerdictV1` is the separate closed verdict serialized exactly as `RELEASE_CERTIFIED`. A gate ID/verdict is never passed as artifact purpose. `candidate freeze` writes `ReleaseCandidateV1`; later commands accept only its verified payload. After all candidate-required precursors are terminal, it signs the exact frozen precursor checkpoint and appends `CANDIDATE_MANIFEST_FINALIZATION_STARTED` with one unpredictable `finalization_attempt_id` and the same nonnull `generation_observed_selection_ref` before the first candidate signing/write; payload and bound/reconciled event repeat those frozen bindings. Atomic create-new output is followed by the one manifest-bound event, then the selector atomically binds that exact candidate ref and releases the generation-transition lease. Recovery signs once from frozen inputs or adopts exact existing bytes without resigning, changing time/selection fields, or generating another ID.

The candidate's `artifact_signer_registry_ref` typed-loads the exact materialized canonical 89-row registry, and its envelope digest must equal `identity.artifact_signer_registry_sha256` before any other evidence signature is accepted. `offline_schema_manifest_ref` typed-loads the one already bound same-run `closure_root=RELEASE_CERTIFICATE` manifest, exact-matches the candidate run, identity hash and journal finalization record, and becomes the only offline schema locator. `client_artifact_set_ref` typed-loads the same-run `RELEASE` mode exact four-platform set; `pre_freeze_carrier_refs`, sorted by `(uri,sha256)`, exact-load the two distinct PASS results `{WINDOWS_AUTHORITY_BUILD,WINDOWS_SERVICE_INSTALL}`. `runtime_topology_declaration_ref` exact-loads the plain declaration created by `FinalInstalledGenerationAuthorityV1` from those terminal installed facts and deliberately contains no later selected infrastructure result; `storage_root_binding` exact-copies the journal header's verified DATA_HDD binding. `data_classification` is exactly `PRODUCTION_SIGNED_NO_BUSINESS_DATA`. Its dedicated Fresh-PG reference must verify as `CANDIDATE_BOUND`, use `through_version=20261025092530`, and exact-match `sha256(JCS(payload.identity))` plus `gate_run_id`; binding the outer signed candidate digest would be circular and is forbidden. These refs are outside the artifact vector.

The three generation artifact fields plus `generation_observed_selection_ref` are mandatory only on this final `ReleaseCandidatePayloadV1`; G4/G5 `SignedIntegrationCandidateV1` retains its existing wire. Under the G0 journal writer guard, candidate freeze first creates/authenticates the current prefix and calls only the release-owned `GenerationObservedReleaseSelectionPortV1` for the already fixed run/deployment/epoch plus that `preselection_prefix`; the port serializes against the OBSERVED pointer and returns the private verified selection, terminal attempt and exact ACK set. Freeze create-new materializes the plain selection record, persists only authenticated monotonic pre-finalization progress while renewing its exact lease, and thereafter loads `GenerationActivationAttemptStoreV1` only through the selected exact attempt—not through current/latest/scan. With the explicit authority-generation bundle it accepts only private `VerifiedGenerationManifestV1`, reruns the lower pure graph compiler/canonicalizer and runtime declaration verifier, and performs one closed graph materialization. It neither links to `ep-platform-generation-activation` nor accepts/copies that upper crate's private verified-graph proof. The canonical signed approval-registry and authority-storage-manifest envelopes use their own trust policies and digest-named final `EvidenceInputStoreV1` locators; the six other storage-bootstrap members remain only at the signed-header fixed paths. Manifest, every reverse plan, every selected strict plain ACK and its participant apply-readback closure, selection record, declaration, graph/projection/policy and all other reachable objects go to exact content-addressed final paths with byte-identical relocatable URIs. Every destination is fsynced and typed-reloaded solely from the final bundle; the complete selection/generation closure and selection ref are frozen before candidate finalization. Reverse plans exact-match item/source/action/target/retention. The selected manifest whole-envelope digest equals its ref and declaration; approval provenance is exact; ACKs are canonical unique and exactly one per required participant from the selected OBSERVED attempt, use the exact fourteen-field plain wire including `participant_apply_readback_ref`, recompute participant/item-set digests, traverse typed item transitions/readbacks, and satisfy start/OBSERVED time bounds. The first `CANDIDATE_MANIFEST_FINALIZATION_STARTED` for `FINAL_RELEASE` is the sole `PRESELECTION -> SELECTION_BOUND` edge and its `generation_observed_selection_ref` must byte-equal the candidate field. Only after create-new candidate storage and durable matching `CANDIDATE_MANIFEST_BOUND` does freeze authenticate the extended progress and pass that private proof—not a raw ref—to `bind_candidate_and_release`; re-entry adopts that exact CAS. Desired-only state, generic proof, direct/current/latest activation lookup, selection expiry/drift, fixed-path/ambient registry substitution, authority-root-only ref, missing/extra/duplicate/mixed-attempt ACK, invalid reverse plan/declaration/dependency/readback, ACK time/wire drift, graph/storage/policy drift, caller refs or discovery fails before finalization. The offline golden removes authority-generation access and still traverses selection/reverse plans/ACKs/readbacks/declaration and completes topology certification/release verification from the final bundle.

`artifacts` is the canonical exact tagged set of one `WINDOWS_AUTHORITY{authority_artifact_set_ref}` plus four `CLIENT_PACKAGE{artifact_id,signed_artifact_ref}` lanes `{windows-client,macos-client,ios-client,android-client}`. The authority ref typed-loads the exact `WindowsAuthorityArtifactSetV1` produced inside the terminal `WINDOWS_AUTHORITY_BUILD` result and exact-matches its candidate run, execution-attempt ID, trusted runner, source tree, authority binary/manifest, production MSI, Authenticode readback, toolchain and build log. The service-install recipe obtains and stages the exact authority set plus that same set's `authority_manifest_ref` and `msi_ref`, and candidate freeze obtains the set only by typed traversal of the terminal build result; neither accepts a caller-supplied authority-artifact path, loose MSI/manifest, opaque wrapper or duplicated subset. Each client ref exact-matches the corresponding release-signed wrapper already fixed by `ClientArtifactSetV1`; the five tagged artifacts must exact-match the two precursor carrier refs and client-artifact-set ref rather than merely carry equivalent bytes. Missing/extra/duplicate/wrong-tag artifacts, duplicate URI/digest identity, a string/free-form purpose, wrong client mode/run, wrong data classification, partial certificate payload, missing/unknown verdict, wrong typed expectation, unknown field, unsigned payload, generic-ref substitution, circular/outer-digest binding, or mismatched signer/offline/Fresh-PG/checkpoint closure is a hard failure.

The certificate is a self-contained offline chain root: its `finalization_attempt_id` exact-matches the frozen release-certificate finalization and bound/reconciled event; `candidate_manifest_ref` points to the exact signed candidate; `l3_evidence_ref` points to the exact signed L3 envelope; L3 points to the exact final L2 envelope; and `prerequisite_receipts` is the canonical exact six-row `GateReceiptRefV1` set G0…G5 under one `CandidateRunIdentityV1`. `certified_data_hdd_root` upgrades the candidate/header's verified root only after exact-loading the PASS P340 soak plus its Residency and filesystem-geometry refs and proving the same storage-manifest ref, volume identity and root digests. `runtime_topology_certification_ref` is created only after P340 and exact-binds the candidate declaration, same candidate, host, 20-user capacity and 259200-second interval. Rows sort by `(gate ordinal,artifact.uri,artifact.sha256)`; each plain ref's `gate` must equal the typed-loaded signed `GateReceiptV1.payload.gate`, and each receipt must carry its required binding variant. `SignedArtifactRefV1`, a generic signed-ref wrapper, digest-only gate label, duplicate gate, or label/payload mismatch cannot substitute. Candidate/L2/L3/certificate point to immutable signed journal checkpoints whose prefixes strictly extend, and canonical `test_results` maps every one of 185 TestIDs bijectively to its typed signed result. Missing/extra/out-of-order/duplicate/cross-run prerequisite receipt fails closed. Every chain URI is `evidence-relative://bundle/...`. `evidence verify --bundle-root <absolute-path> --offline` starts with its minimal schema/signature bootstrap, authenticates the candidate-bound signer registry and offline manifest, schema-validates every reachable signed JSON envelope using the exact generated closure, and follows only signed refs; bootstrap parsing alone can never PASS, and it never scans, guesses siblings, or resolves a network schema. The trusted issue time and bounded expiry are frozen in the verified finalization record; the expiry minimum includes the bound offline manifest, P340/root-upgrade/topology-certification chain, all prerequisite receipts, Final L2/L3, requirement/carrier results, every reachable objective-closure state/evidence/fact/review result and journal checkpoint. The constructor never obtains these values from a live clock. Immutable candidate manifests and static content-addressed refs are exact-verified but do not participate in the expiry minimum; no untyped time-bearing input or expiry extension is accepted. Only the master registry-selected `DetachedCmsSignerV1`/`sign_business_artifact_v1` path may issue it.

`RELEASE_CERTIFIED` is necessary but is not itself production activation. Starting from the verified release certificate, the later activation entry exact-loads its candidate and the candidate's three frozen generation refs, reconstructs `VerifiedGenerationManifestV1` only through `GenerationApprovalVerifierV1`, revalidates the canonical same-attempt ACK set/formulas and requires it still byte-equal the durable OBSERVED state. It also requires the exact `VerifiedRuntimeTopologyCertificationV1` plus four freshly collected readbacks: topology, graph-exact runtime deployment, installed Authority/backup components and storage safeguards. Topology must byte-match the certified declaration/profile/storage, active participant-ID set, independent database-consumer projection and canonical generation item/subset relation. Runtime deployment must preserve the complete graph/closure/readback participant-ID bijection, the ACTIVE/positive/declaration/generation-required/ACK participant-ID bijection and both independent projections; every deferred participant remains absent from all active relations. It never defines a generation wire/domain wrapper or replaces a candidate ref with latest database state. A generic verified envelope, declaration alone, raw certification ref, desired-only generation, wrong registry, missing/stale/mixed ACK set, missing/extra/deferred-present runtime participant, consumer/item-relation drift, stale readback or certificate/certification mismatch cannot enable production.

`migration_closure` is the exact master `MigrationClosureIdentityV1`; `sha256(JCS(migration_closure))` must equal `payload.identity.migration_manifest_sha256`. The four named artifact refs are an exact set, and each digest must equal its corresponding closure field. There is no singular or directory-scanned migration manifest. The certificate does not invent a release-due map: it binds the complete generated delivery registry digest and the already frozen canonical first-due-map SHA-256 `a9547557f95a3a9892efa9f6751a0dd03accac65da344aa559a3203488fee086`; all 185 `release_due_profile=G6_RELEASE` rows are verified directly from that registry.

Release 输入只有两个彼此正交的 exact-set：候选内一个 tagged `WINDOWS_AUTHORITY` set ref 加四个 typed `CLIENT_PACKAGE` refs，以及 Final L2/L3 中六个 `ReleaseCarrierRecipeIdV1` 结果。两者不得合并成 `windows-client`、`backup-recovery`、`evidence-authority` 等第二套“carrier ID”命名层。四端各自的签名、安装/启动、升级、撤销、能力、资源、DLP、可访问性证明由其候选制品对应的八类生命周期证据闭包提供；恢复、勒索演练、当前 P340 载体/容量、物理 UPS 断电、PITR 与 Windows Authority 安装测量由六个固定 carrier slot 提供。未来 IaaS 不占用、不替换这些当前 slot；必须由后续图版本显式修订。缺失、额外、重复、过期、错候选、跨 profile/lane/recipe 或任意别名均失败关闭。

`candidate freeze` refuses a dirty source tree, missing/engineering signature, fewer or more than the selected four client packages plus the one typed authority artifact set, signer-registry/offline-schema/graph/projection/facade/migration/topology/toolchain drift, generation-manifest/approval-registry/participant-ACK drift, desired-only or generic generation proof, missing/extra/duplicate/mixed-attempt ACK, missing Fresh PG closure through `92530`, or an unpoliced protected table. After writing the external candidate manifest it never edits the repository.

Every carrier uses one causally fixed staging protocol. Before `TEST_STARTED`, Rust derives and signs one `CarrierStagingPlanV1` with the same unpredictable execution-attempt ID that will appear in STARTED, stores the plan object content-addressed, and fixes the exact recipe-conditioned `inputs` set: empty for `WINDOWS_AUTHORITY_BUILD`, `POSTGRES16_PITR`, and `BACKUP_RESTORE_CERTIFICATION`; `{WINDOWS_AUTHORITY_ARTIFACT_SET,WINDOWS_AUTHORITY_MANIFEST,WINDOWS_AUTHORITY_MSI}` for `WINDOWS_SERVICE_INSTALL`; `{P340_CAPACITY_INPUT_MANIFEST,P340_CERTIFICATION_POLICY_ATTESTATION}` for `P340_RELEASE72_HOUR`; and `{UPS_IDENTITY_READBACK,UPS_POWER_WRITE_CACHE_POLICY}` for `POWER_SHUTDOWN`. It then durably appends STARTED with singleton `start_context_refs=[staging_plan_ref]`; only after STARTED may it derive/create staging and copy the plan plus exact inputs. The three service inputs traverse the same terminal build result: the signed set is staged at `inputs/windows-authority-artifact-set.v1.json` with `application/vnd.ep.f57-windows-authority-artifact-set-v1+json`, its `authority_manifest_ref` at `inputs/windows-authority-manifest.v1.json` with `application/vnd.ep.f57-windows-authority-manifest-v1+json`, and its byte-verified `msi_ref` at `inputs/windows-authority.msi` with `application/octet-stream`. The fixed script receives exactly `-ArtifactSet <staging>\inputs\windows-authority-artifact-set.v1.json -ArtifactManifest <staging>\inputs\windows-authority-manifest.v1.json -MsiPath <staging>\inputs\windows-authority.msi`; raw `authority_artifacts_ref` and `installed_msi_ref` exact-match the source set ref and its `msi_ref`. Each input exact-binds typed source ref, staging-relative path and media. For P340, one journal-authenticated qualification plan precedes seven strictly ordered STARTED→COMPLETED-or-UNKNOWN→RECONCILED operations, CLOSURE_BOUND and its checkpoint; only then may the signed capacity input bind the seven outputs and 90-day `QUALIFICATION_SYNTHETIC` history, followed by the carrier plan and STARTED.

No IaaS alternative or provider-power staging set exists in this graph version. `IAAS_WINDOWS_SERVER_HDD_STRICT`, `IAAS_PROVIDER_POWER_SHUTDOWN_EQUIVALENCE`, every `IAAS_*` input discriminator and every provider/VM/vTPM/cache/snapshot ref is rejected before STARTED and cannot appear in a staging plan, carrier result, offline-schema manifest or certificate. The future seam must introduce its complete input sets and new graph version together; it may not borrow the current P340/UPS slots.

`CarrierStagingPlanPayloadV1.recipe_specific` is `NONE` for five recipes. `BACKUP_RESTORE_CERTIFICATION` alone carries `RECOVERY_CERTIFICATION` with one unpredictable `certification_id` and fixed ordered rows `1/INITIAL_RESTORE_VERIFIED`, `2/CANDIDATE_MEASURED`, `3/CERTIFIED`, each with a distinct unpredictable `recovery_execution_attempt_id`; all are generated and signed before carrier STARTED. The strict flow-owned recovery policy repeats that plan ref, candidate, one outer carrier attempt, certification ID and ordered subattempt set. Every raw context keeps the outer ID and its separate mandatory subattempt field exact-matches its row; no caller supplies either ID and no started subattempt is rerun.

`CarrierStagingPlanPayloadV1.continuation` is exactly `NONE` for the first five recipes. `POWER_SHUTDOWN` alone uses `POWER_SHUTDOWN` with a fresh `continuation_id`, `continuation_binary_ref` exact-matching the final candidate authority set's Authenticode-verified installed `ep-core-server.exe`, exact dispatcher authority, installed continuation row, 18-row object/action security-descriptor set, canonical absolute bundle/journal/staging roots, one runtime-SSD `NO_BUSINESS_DATA` control-capsule plan capped at `33554432` bytes, `phase_state_run_relative_path`, `success_persistence_command_relative_path=continuation/power-shutdown-success-persistence-command.v1.json`, and `raw_evidence_relative_path=raw/00.v1.json`. All five services are permanently `AUTO_START`; an attempt never changes start type, creates, deletes, disables or reconfigures them. The continuation is dormant solely when its dedicated activation child key contains no `ActiveRecordPath` and then exits `STOPPED` without side effects. Its exact identity/path/SID type/recovery are `EPF57PowerShutdownContinuation`, `Enterprise Platform F57 Power Shutdown Continuation`, `NT SERVICE\EPF57PowerShutdownContinuation`, `C:\Program Files\EnterprisePlatform\Authority\ep-core-server.exe`, `UNRESTRICTED`, and `NO_AUTOMATIC_RESTART`; dependencies sort `[CryptSvc,EPAuthorityControl,EPAuthorityServer,EventLog]`, privileges are `[SeChangeNotifyPrivilege]`, and source-order argv is `power-shutdown-continuation --activation-source scm-parameter`. The signed plan, not argv or discovery, supplies every per-attempt value.

The current power-safety slot always selects `POWER_SHUTDOWN`; there is no provider-power `NONE` continuation branch. A future IaaS graph version must define its own continuation and provider operation-ID/query-adopt semantics without borrowing physical UPS, outlet or POWER evidence.

The privileged allowlist broker is exact service/account/numeric SID `EPAuthorityControl` / `NT SERVICE\EPAuthorityControl` / canonical `WindowsSidV1`. Its token privileges are exactly `[SeChangeNotifyPrivilege,SeShutdownPrivilege]`; on global SCM it has only `SC_MANAGER_CONNECT`, and on the continuation object exactly `{QUERY_STATUS,START}`—never `STOP|CHANGE_CONFIG|DELETE`. It accepts only the signed plan over its fixed authenticated pipe, may set/delete only the separately ACLed activation parameter, perform the one dispatch protocol, supervise bounded postboot resume, and create-new only the six closed output discriminators at their plan-fixed paths. It cannot sign either evidence kind, alter service configuration or write another staging path. Live token/handle probes construct the embedded preflight projection and its digest; account labels, broad Administrators membership, an extra right or a nonnumeric SID fail before STARTED.

Raw-evidence signing and GateRunJournal signing use two separately installed least-privilege, keyless facades. Exact service/account/SID names are `EPF57PowerRawSigner` / `NT SERVICE\EPF57PowerRawSigner` and `EPF57GateJournalSigner` / `NT SERVICE\EPF57GateJournalSigner`. Each owns only one fixed authenticated facade IPC endpoint and an exact allowlist for its registry-selected artifact kind/discriminator; after independently validating the complete payload, it forwards the frozen operation digest/idempotency key over the already G0-owned authenticated `F57EvidenceSignerV1` durable operation API. The G0 evidence-signing broker/session remains the sole production owner and sole non-SYSTEM principal allowed to use the two non-exportable key containers named by the existing raw and journal registry rows. Neither facade, `EPAuthorityControl`, `EPAuthorityServer` nor the continuation can open a signing key; the facades cannot request any other kind/discriminator or arbitrary bytes. The signed plan freezes both facade identities/IPC DACLs/forwarding-policy digests, the two generic evidence-broker key descriptors and one control token/SCM preflight digest. Before STARTED, trusted Rust exact-readbacks all services and proves key non-exportability, facade role separation, upstream broker authentication and the exact service-object grants. A missing shutdown right, broad administrator membership, extra SCM right, aliased key/principal, exportable key, facade key ACE, cross-role forwarding, arbitrary-payload request or direct key access fails without adding a signer-registry row.

The continuation virtual account receives only read/execute on the installed binary, read-only capsule/bootstrap inputs and access to the authenticated control-broker pipe; it cannot append either journal, sign, access a CMS key, alter SCM/registry, request shutdown or write the executable/business bundle. `PowerShutdownContinuationSecurityDescriptorsV1.descriptors` is attempt-specific and has exactly 18 rows sorted by object enum: five SCM services, installed executable, bundle root, run journal, staging root, control-capsule root, phase state, the dedicated activation child registry key, three control/raw/journal broker-facade pipes, the distinct Authority-recovery-proof pipe and two G0 evidence-broker authorization CNG keys. Every row embeds one closed `action`, a canonical SDDL string whose numeric masks use literal lowercase `0x` followed by uppercase hexadecimal digits, and `sha256(canonical_sddl)`. Permanent SCM objects, installed executable, bundle/run roots, the dedicated activation child registry key, existing key containers and all four permanent pipe policies are `VERIFY_EXISTING_IMMUTABLE`: POWER opens by final handle, reads the live descriptor and fails on any mismatch; it never repairs or reapplies them. Only the attempt-owned staging root, control-capsule root and phase-state object are `CREATE_WITH_DESCRIPTOR`: the privileged authority creates each through a non-inheriting secure handle with the exact descriptor before exposing a name, then immediately reads it back. Permanent pipe DACLs are applied by their owning service at `CreateNamedPipeW`; every authorized client data ACE uses concrete `0x00120183`, never GENERIC_WRITE or pipe-instance creation, and POWER only authenticates the live handle. The activation row secures the already installed child key `...\Parameters\F57ActivationV1`, inside which `ActiveRecordPath` is a value; a registry value is never treated as an ACL-bearing object. The two key rows are exactly `EvidenceBrokerRawAuthorizationKey|EvidenceBrokerJournalAuthorizationKey`, name the G0 broker-held raw/journal authorization containers and grant key use only to SYSTEM plus that generic broker identity, never either facade. On continuation SCM, control has only `SERVICE_QUERY_STATUS|SERVICE_START`; all control/continuation ACEs omit change-config, stop and delete. ARM embeds the full service/dispatcher/18-row projection, recomputes its digest from live handle-based readback and proves activation is still absent. Object-kind/action byte goldens and pre-open/post-open tamper tests reject repair-on-mismatch, inheritance, writable binary, facade/direct/cross-role key grant, early activation, shell command, broad directory grant or caller-selected field before STARTED. The continuation and facades add no signer-registry row.

The first five recipes launch their fixed AllSigned scripts after durable STARTED. Power is different: its only script invocation is prepare-only `-NoProfile -NonInteractive -ExecutionPolicy AllSigned -File scripts/windows/test-power-shutdown.ps1 -Mode PrepareOnly -StagingPlan <staging>\plan.v1.json -CandidateManifest <candidate> -UpsIdentityReadback <staging>\inputs\ups-identity-readback.v1.json -UpsPowerWriteCachePolicy <staging>\inputs\ups-power-write-cache-policy.v1.json -QuiesceOut <staging>\artifacts\<quiesce-readback-key-sha256>.bin`. It cannot request shutdown or write raw. Before any ordinary boot path, `apps/core-server/src/main.rs` invokes the single Windows-only launcher dispatcher in `windows_service/dispatcher.rs`. It recognizes exactly `[authority-server,--service-mode,windows-scm]`, `[power-shutdown-continuation,--activation-source,scm-parameter]`, `[power-shutdown-control-broker]`, `[power-shutdown-signing-broker,--role,raw-evidence]`, and `[power-shutdown-signing-broker,--role,gate-run-journal]`; each vector maps to one closed numeric role and is forwarded only through the already verified `EpAuthorityKernelApiV1::run_service_main`. Ordinary recovery/API boot and all four specialized ServiceMain/facade implementations live exclusively in the selected kernel DLL. The launcher neither imports nor mirrors them. Plain no-service argv remains an engineering foreground mode and is forbidden in the installed manifest. An unknown/extra service-mode token, unknown role or ABI failure stops before any application composition, and no vector selects paths, IDs or per-attempt values. Real Windows process-level tests launch all five exact vectors plus every truncation/extra/unknown-role mutation and exact-match manifest, MSI, launcher, signed pointer, held DLL, ABI table and kernel entrypoint identity.

The continuation state file is a permanent non-staging sole-store wire at canonical URI `evidence-relative://bundle/<RUN_DIR_REL>/continuation-state/<ascii-lowercase-carrier-TestID>/<lowercase-hyphenated-execution_attempt_id>/power-shutdown-state.v1.jcs.jsonl`; it is never a carrier-staging member and survives deletion of the staging directory. UTF-8/no-BOM framing is one `PowerShutdownContinuationStateHeaderV1` plus exactly four success-path `PowerShutdownContinuationStateRecordV1` rows as `<8 lowercase hex byte length>\t<JCS bytes>\n`. Rows are sequence `1..=4`, phases `ARMED,PRE_SHUTDOWN_COMMITTED,POST_RESTART_COMPLETED,DISARMED`, and form a hash chain from the complete header frame. Each GateRunJournal continuation event carries the same TestID/attempt and a strict-extending typed `PowerShutdownContinuationStatePrefixV1` with `last_sequence=1|2|3|4`; ARMED additionally repeats `staging_plan_ref`, and PRE_SHUTDOWN_COMMITTED repeats the exact dispatch-intent ref. The four state refs typed-load ARM, `PowerShutdownDispatchCommitIntentV1`, the complete signed power raw, and DISARM. Failure uses the separate signed `CARRIER_CONTINUATION_FAILED` event with failure/cleanup refs and never fabricates a fifth success row.

The release owner defines `PowerShutdownContinuationStateStoreV1` and alone constructs private `VerifiedPowerShutdownContinuationStateStoreV1` from verified DATA_HDD/bundle root, journal header/run, signed POWER plan and exact attempt. The file adapter receives opaque derived locators and implements only create/adopt header, append exact next row, typed prefix load and quarantine; caller path/media, scanning and arbitrary append do not exist. `PowerShutdownContinuationStatePrefixV1` and all five continuation event payloads remain sole-owned by `ep-platform-gate-journal-contract`; release owns state header/record, plan and POWER controls/readbacks. Append/create-new, flush-through-disk, exclusive lease, torn-tail quarantine and no-overwrite rules equal the main journal. Goldens delete all staging bytes and still verify all four prefixes and offline PASS; every crash cut and cross-run/attempt store reuse fails closed.

After STARTED, trusted Rust applies/readbacks the exact attempt SDDL set while `ActiveRecordPath` remains absent, writes ARM/state row 1/ARMED, runs the prepare-only script, and typed-verifies QUIESCE/UPS_TRIGGER plus their previous/trigger `UpsStatusReadbackV1` objects. It opens one live authenticated control-broker session, obtains one owner-token digest, constructs the full strict `UpsOutletCycleCommandV1` and embeds it in `PowerShutdownDispatchCommitIntentV1`, then persists in strict WAL order: fixed capsule locator -> permanent HDD `EvidenceObjectStoreV1` object -> state row 2 -> PRE_SHUTDOWN_COMMITTED. Command/run/attempt/continuation, manifest/config/identity, owner token, boot/source/deadline, outlet group/path and policy exact-match; the command contains no endpoint, credential, executable path, argv or vendor raw payload. The broker alone persists `PowerShutdownDispatchApiCallCommittedV1` immediately before the one `InitiateSystemShutdownExW` call; its `requested_at_unix_ms` is captured once and EVENT byte-equals it. Marker presence means the call may have occurred and forbids redispatch. The API return and `User32/1074` are observations, never durable acknowledgement alone. Before any provider call, the UPS adapter create-new/fsyncs its private same-command operation row with the exact boot/source/start tick; recovery may load but never resample it. PASS requires one fsynced `PowerShutdownDispatchAcknowledgementV1` combining exact User32 bytes with strict same-command/digest `UpsOutletCycleCommandAckV1`; ACK boot/source exact-match and checked `adapter_call_started <= acknowledgement_observed <= min(adapter_call_started+30000,command deadline)`. Same identity/ID/digest may query/adopt the byte-identical ACK only inside that inner deadline; changed digest conflicts, unknown at 30 seconds never resends, and UTC fields are reporting-only. The 600-second outer window may finish only User32/composite/preshutdown reconciliation. If the composite is not durable, Windows may finish clean shutdown, manual power-on/UPS repair is required and the attempt closes UNKNOWN; shutdown is never aborted and neither irreversible action is retried.

On boot the permanently `AUTO_START` control broker runs before the continuation. It first resolves fixed pre-controller failure cases; with a marker but no durable old-boot composite ACK it writes `DISPATCH_ACKNOWLEDGEMENT_ABSENT_AFTER_BOOT_CHANGE` and never starts continuation. With exact marker+ACK and a distinct boot, it creates/adopts the signed/authenticated `PowerShutdownPostbootResumeControllerV1`, whose stages are `DISPATCH_ACKNOWLEDGEMENT -> DATA_HDD_UNLOCK_AND_MOUNT -> JOURNAL_AND_STATE -> POSTGRES_RECOVERY -> DURABLE_CUTS -> UPS_RESTORED -> SUCCESS_PERSISTENCE`, checked against one boot-scoped monotonic deadline and maximum generation 8. A previously valid `SUCCESS_COMMAND_DURABLE` controller frame or exact adoptable success-command spool wins across a B->C reboot before any context-change failure; clocks from different boots are never compared, and conflicting success/failure bytes are tamper/UNKNOWN.

ServiceMain is keyless and cannot write arbitrary files. It sends one `PowerShutdownSuccessPersistenceDraftV1` containing EVENT, AUTHORITY_RECOVERY_PROOF and RESTART values; only the broker captures its accepted monotonic tick and create-new/fsyncs `PowerShutdownSuccessPersistenceCommandV1` at the plan-fixed spool. That valid spool is the sole source for the exact three outputs and is cross-boot adoptable. The six closed broker output discriminators, fixed locators and controller transitions reject extra/repeated messages. The privileged finalizer typed-loads those outputs, asks only `EPF57PowerRawSigner` to create raw, appends state row 3/POST_RESTART_COMPLETED, removes only `ActiveRecordPath`, verifies the permanent service remains registered, `AUTO_START`, dormant and `STOPPED`, writes DISARM, and appends row 4/DISARMED. DISARM never disables a service. Only prefix 4 permits completion/result/terminal. Every missing boot transition, alternate binary/path/SID, raw before prefix 2, cross-broker signing, generation nine, spool/controller conflict, remaining activation parameter or second shutdown fails closed.

The six plain readbacks are ARM, QUIESCE, UPS_TRIGGER, EVENT, RESTART and DISARM; their exact purpose/media pairs are the corresponding `EP-F57-POWER-SHUTDOWN-...-READBACK-V1` and `application/vnd.ep.f57-power-shutdown-...-readback-v1+json` wires. ARM byte-repeats the control broker, the two keyless signer facades and their G0 upstream-broker bindings, the permanent continuation-service plan, same run/attempt/continuation/plan ref and pre-shutdown boot ID, and its SCM readback digest is nonzero. QUIESCE exact-binds the same host/boot and one unpredictable nonzero `authority_barrier_id`; it requires zero in-flight accepted commands and `postgres_wal_flush_lsn>=postgres_checkpoint_lsn`. Its five barrier/checkpoint refs are opaque immutable `application/octet-stream` objects in `EvidenceObjectStoreV1`, parsed only by the containing field's closed parser: `EP_F57_AUTHORITY_BARRIER_V1`, `EP_F57_PG_CHECKPOINT_V1`, `EP_F57_ATTACHMENT_FSYNC_MANIFEST_V1`, `EP_F57_AUDIT_CHECKPOINT_V1`, or `EP_F57_OUTBOX_CHECKPOINT_V1`. Every parsed header exact-repeats candidate identity, gate-run ID, execution-attempt ID, continuation ID and authority-barrier ID; typed scalars/digests are recomputed from bytes. These opaque objects have no purpose or signer row because the phase-prefix-authenticated QUIESCE digest authenticates them.

UPS_TRIGGER exact-loads the same signed-vendor manifest, UPS identity/policy, serial/firmware, runtime-security readback and fresh previous/trigger statuses; both statuses must have PASS self-test and healthy communication and no required `UNKNOWN` field; its `authenticated_ups_event_log_ref` is exact `application/octet-stream`, parsed only by `EP_F57_UPS_EVENT_LOG_V1`, whose authenticated header/samples repeat the same run/attempt/continuation and UPS identity. It proves actual AC loss, on-battery transition, maintained output, previous remaining runtime `>900`, the first trigger sample `<=900`, and checked `ac_lost_at<=threshold_reached_at<=shutdown_authorized_at`. EVENT is exactly `PLANNED_OPERATING_SYSTEM_SHUTDOWN`, provider `User32`, event ID `1074`, reason `0x80040001`, exact attempt-tagged comment, initiating executable identity and the API marker's persisted `requested_at_unix_ms`; it also binds the composite acknowledgement, same outlet group/power-path/command ID, authenticated outlet ACK, full shutdown lifecycle and zero abort/unexpected/intervening requests. The only field-selected opaque parsers here are `EP_F57_WINDOWS_EVENT_RECORD_V1` and `EP_F57_SHUTDOWN_LIFECYCLE_V1`; the outlet ACK is strict `UpsOutletCycleCommandAckV1` under the common UPS schema and media, and none of these objects alone authorizes dispatch.

RESTART exact-repeats run/attempt/continuation/host, the QUIESCE `authority_barrier_id`, both distinct nonempty boot IDs and the same trigger ref. Both `PowerShutdownAuthorityRecoveryProofV1` and `PowerShutdownRestartConsistencyReadbackV1` carry the same nonnull `data_hdd_unlock_readback_ref`; it typed-loads the current-boot PUBLIC_KEY broker result and exact-matches the certified volume, authority epoch, broker/bootstrap, mount and dirty-clear state. RESTART proves both certified volumes mounted/clean, PostgreSQL recovery at or beyond quiesced WAL, byte-equal attachment/audit/Outbox cuts, zero lost commands/duplicate effects/unexplained obligations, actual UPS output-off cycle, restored AC and online UPS with `threshold_reached_at<=ups_output_off_at<=ups_restored_at<=restart.completed_at`; `EP_F57_UPS_OUTLET_CYCLE_V1` owns that last opaque capture. DISARM proves STOPPED, permanent registration present, start mode `AUTO_START`, and activation parameter absent after raw finish and no later than completion. Task 14 owns exactly nine closed opaque parsers and byte fixtures: `EP_F57_AUTHORITY_BARRIER_V1`, `EP_F57_PG_CHECKPOINT_V1`, `EP_F57_ATTACHMENT_FSYNC_MANIFEST_V1`, `EP_F57_AUDIT_CHECKPOINT_V1`, `EP_F57_OUTBOX_CHECKPOINT_V1`, `EP_F57_UPS_EVENT_LOG_V1`, `EP_F57_WINDOWS_EVENT_RECORD_V1`, `EP_F57_SHUTDOWN_LIFECYCLE_V1`, and `EP_F57_UPS_OUTLET_CYCLE_V1`. The typed UPS ACK belongs to the common UPS schema; release owns only the nine containing exact-media `ArtifactRefV1` contracts and imports that schema once, so no phantom subordinate schema or signer exists. Ordinary reboot, User32-only acknowledgement, missing/current-boot unlock ref, AC loss/battery crossing/outlet cycle/restore, wrong reason/comment/path/digest, parser/header/barrier drift, LSN rollback, dirty volume, nonzero reconciliation count, missing registration, non-AUTO_START service, remaining activation parameter or Boolean-only PASS fails.

A crash before/after every capsule locator/object write, state/event append, API marker, API call, composite ACK, OS termination, boot, controller frame, success-command spool, raw, disarm or terminal resumes only the next legal row for the same attempt. The capsule lives on runtime SSD only as a bounded control-byte copy, contains no business data and is not authoritative evidence; all permanent nested bytes are copied to HDD `objects/sha256`, and deletion of the SSD capsule after a complete run must still allow offline PASS. Marker without composite ACK can never be promoted after boot; the four pre-controller failures and seven controller-stage failures produce strict signed failure plus cleanup, append FAILED and end UNKNOWN. External UPS ambiguity is intentionally not auto-recovered. Exact existing locators/spool/raw are adopted; quiesce, UPS schedule and shutdown are never repeated.

PowerShell never signs `CarrierStagingCompletionV1`. After all planned raw bytes exist—and for POWER only after verified prefix 4 plus permanent `AUTO_START` dormant state—Rust first constructs the complete unsigned strict `CarrierStagingCompletionPayloadV1` from the frozen raw/disarm prefix and derives `completion_payload_sha256=sha256(JCS(payload))`; only then may it fsync `CARRIER_STAGING_COMPLETION_FINALIZATION_STARTED{test_id,execution_attempt_id,finalization_attempt_id,staging_plan_ref,frozen_input_checkpoint_ref,completion_payload_sha256,completed_at_unix_ms,issued_at_unix_ms,expires_at_unix_ms}`. The payload exact-copies the corresponding eight non-digest identity/ref/time values, with `payload.carrier_test_id == STARTED.test_id`; the verifier recomputes the payload digest before signing or adoption. BOUND/RECONCILED each have exactly four fields `{test_id,execution_attempt_id,finalization_attempt_id,completion_ref}` and adopt only identical bytes; the verifier uses that finalization ID to exact-load STARTED and recover its frozen checkpoint rather than copying the checkpoint into either four-field event. Rust then constructs/signs/fsyncs the outer result and appends terminal. P340 additionally requires `TEST_STARTED.recorded_at_unix_ms <= context.started_at_unix_ms <= context.finished_at_unix_ms <= CarrierStagingCompletionV1.completed_at_unix_ms <= ReleaseCarrierResultV1.issued_at_unix_ms <= terminal_journal_record.recorded_at_unix_ms` and exact duration `259200000` milliseconds. Any raw/nested context-bound physical envelope has the non-optional signature window `[context.finished_at_unix_ms,min(context.finished_at_unix_ms+300000,context.expires_at_unix_ms)]`. Three-run restore creates raw 00/01 first and raw 02 before its single completion. Missing or wrong `completion_payload_sha256`, `test_id != payload.carrier_test_id`, an extra BOUND/RECONCILED `frozen_input_checkpoint_ref`, input/ref/path/media/key/cardinality/time drift, premature completion, caller-selected staging, discovery, overwrite, extra file or retry-induced physical work fails closed.

The exact G6 handler ownership is:

- `control_center_contract.rs`: `OPS-001`.
- `full_release_evidence.rs`: `NFR-011`, `NFR-013`, `NFR-018`.
- Task 13 current infrastructure-capacity owner `p340.rs`: `NFR-001`, `NFR-003`, `NFR-005`, `NFR-007`.
- `storage_key_boundary.rs`: `SEC-002`, `SEC-009`, `SEC-012`, `SEC-014`, `NFR-002`, `NFR-015`.
- `transactional_evidence.rs`: `SEC-010` only; the G1 `GOV-009` handler remains unchanged.
- `windows_recovery_security.rs`: `GOV-002`, `DBP-001`, `SEC-001`, `SEC-003`, `SEC-004`, `SEC-005`, `SEC-011`, `SEC-013`, `SEC-016`, `SEC-017`, `NFR-004`, `NFR-006`, `NFR-008`, `NFR-009`, `NFR-012`, `NFR-014`, `NFR-016`, `NFR-017`, `DEF-006`, `DEF-007`, `DEF-010`.

Each handler requires final-candidate-bound evidence and returns `NOT_COVERED` for a Task 10–13 rehearsal. Task 14 activates exactly six closed auxiliary carrier recipes in the G0 dispatcher; release PowerShell cannot be called directly as candidate evidence. Those six auxiliary TestIDs run first and become signed `carrier_refs`, never `test_results` and never part of the 185 due-set. Graph-bound G6 Requirement handlers typed-load them and produce canonical `RequirementEvidenceBindingV1` results under their registered due TestIDs. The partition is mechanically generated from the same DeliveryRegistry: Final L2 owns the canonical sorted 149 TestIDs with `first_due_profile=G0_BOOTSTRAP..G5_INTEGRATION` and `sha256(JCS(vector))=5ec5a8663b3763bcbf85d84438d6236e9a9680fc0749ad0ca3efa04f124a5a7a`; L3 owns the canonical sorted 36 TestIDs with `first_due_profile=G6_RELEASE` and digest `e7a2fae4fa1a3384c47592ffd0310b3b8717cb595bc5c4397f61e0be2cbc85df`. The vectors are disjoint and their union is the 185-row Requirement set. Neither stage may move, repeat or omit a row, and the six auxiliary carrier IDs appear only through `carrier_refs`. It never repeats a test to select a favorable result. Final L2, L3 and certificate policy caps are each exactly 90 days, but every aggregate expiry is shortened to the earliest consumed typed input and the run header's fixed expiry.

The six-carrier exact set has one current infrastructure-capacity recipe `P340_RELEASE72_HOUR` and one current power-safety recipe `POWER_SHUTDOWN`; together they retain the P340 seven-operation, physical-UPS/outlet contracts and terminal `SINGLE_DISK_DEGRADED_PRODUCTION`. `infrastructure_certification_profile_ref` must name that P340 profile, and every future IaaS/provider-power ref is an extra unregistered member that fails before STARTED. A later IaaS release must amend the graph, registries, handler bindings, carrier exact set and 185-Requirement projection coherently rather than add a favorable retry or reuse either current slot.

Final L2 also carries the exact same four `ObjectiveClosureBindingV1` rows required at G5, all `CLOSED` with no open obligations; procurement retains its three exact fact-kind/owner refs and distinct authorized reviewer binding. Every closure ref belongs to Final L2's same-run authorized result closure. Fresh G4 and G5 receipts issued inside this verified `ReleaseCandidateV1` context byte-equal that four-CLOSED vector even though their due/probe sets remain G4/G5; the standalone DevelopmentSlice G4 WAITING receipt is immutable history and cannot enter the final aggregate. L3 copies the vector byte-for-byte from Final L2, independently revalidates all four current generations under its later checkpoint, and fails if any objective reopened or advanced. Certificate construction immediately revalidates the same chain through L3 before freezing its inputs. All state/evidence/fact/review results participate in checkpoint and expiry. Wrong candidate purpose/context, `WAITING`, `REOPENED`, stale generation, missing/extra/reversed fact, owner permutation, same reviewer, outside-result ref, unequal L2/L3 vector or post-L3 reopen fails release.

- [ ] **Step 4: Run pre-commit tooling and migration rehearsals.**

Run: `cargo xtask f57 fresh-pg --profile G6_RELEASE --through 20261025092530`

Expected: engineering rehearsal PASS through the immutable 69-file baseline plus all 47 F57 reservations (`116` total), with zero unpoliced protected table; no candidate receipt is issued.

Run: `cargo xtask f57 graph generate`

Run: `cargo test -p ep-testkit --test f57_final_candidate --test f57_final_installed_generation --test f57_package_maintenance_production --test f57_production_activation --test f57_production_generation_admission --test f57_production_admission_execution_lease --test f57_production_admission_bypass --test f57_production_admission_races --test f57_resilience_admission --test f57_postgres_log_retention_control --test f57_data_hdd_disaster_replacement --test f57_windows_runtime_deployment --test f57_release_gate_unit --test f57_release_dependency_dag --test f57_ups_adapter_contract --test f57_ups_command_reconciliation -- --nocapture`

Run: `cargo test -p ep-xtask --test f57_release_carrier --test f57_windows_runtime_deployment -- --nocapture`

Run: `cargo test -p ep-platform-release -p ep-platform-package -p ep-platform-generation-activation -p ep-platform-backup -p ep-platform-tenancy -p ep-platform-ups-contract -p ep-adapter-ups-windows -p ep-authority-kernel -p ep-adapter-file -p ep-adapter-db-pg -p recovery-tool -p core-server -p ep-xtask -p ep-testkit -p ep-release-gate --all-targets --locked`

Run: `cargo xtask f57 graph generate --check`

Run: `cargo run -p authority-kernel-abi-gen --locked -- --check`

The `windows-f57-release-precommit` job in `.github/workflows/ci.yml` is mandatory and runs on the approved Windows Server 2022 x64 self-hosted runner with the locked MSVC toolchain; a Linux/macOS build or `cfg(windows)` elision cannot satisfy it. Its YAML is a thin adapter that invokes only `cargo xtask f57 verify --level l1 --profile windows-f57-release-precommit`; the Rust-owned frozen `WindowsF57ReleasePrecommitPlanV1` deterministically derives and executes the complete auditable subprocess transcript solely owned by [CI pipeline §7.1](../../ci-pipeline.md). The task-local commands above are additive engineering/rehearsal coverage and cannot replace, subtract from or restate a divergent Windows minimum.

Expected: PASS; the facade registry has all 185 handlers and 36 G6 rows. Windows tests launch five fixed SCM vectors through one immutable launcher, round-trip each quoted raw ImagePath to the exact argv, load the DLL only through the generated 48-byte/one-export ABI, verify all nine fixed EnterprisePlatform SCM rows plus the one task/helper and exercise the complete recovery-task principal/token protocol. Package-maintenance composition proves graph/tenancy-derived scope, structural plan before hold, tagged package cause, hold/drain/barrier before full-cut checkpoint, fresh execution authorization, per-participant `(activation_attempt_id,participant_id,item_id)` trust/operation/readback identity, tagged rollback readbacks, continuation capsule/reseal and no rollback before `ROLLBACK_STARTED`. Admission tests prove genesis/delta/rollback-reopen CAS, exact twenty-field durable ACCEPTED→terminal execution leases, server-derived authority-epoch equality, the exact ten-row signed-set-reachable bypass registry, all four hold causes and cause-specific state/nullability graphs, and permit/hold/barrier races; resilience tests additionally prove root-rotation hold/drain/barrier-before-CAS and held transitions, dead-old-disk disaster recovery without old `HEALTHY`, UPS equality-inclusive 15-second hold/aggregate-60-second two-PASS rule and local shutdown, all 30 backup-domain inequalities, exact PostgreSQL log retention/ACL/legal-hold/free-space behavior, current P340-only selection and rejection of every future IaaS/provider-power input before STARTED. Missing/wrong epoch, cross-cause ref/state, cross-epoch request-ID replay, old-epoch orphan ACCEPTED, terminal epoch drift, OBSERVED drift, an intersecting hold or any store error fails closed. DATA_HDD tests prove PUBLIC_KEY locator/policy/bootstrap/unlock and clean-SSD reenrollment, while G0 broker tests prove the complete static/dynamic readback and zero SSD fallback. `f57_release_dependency_dag` requires `ep-platform-generation-activation -> {ep-platform-release,ep-platform-runtime,ep-platform-capability-graph,ep-platform-package,ep-platform-backup,ep-platform-tenancy}` plus the displayed release edges; it rejects package -> backup/tenancy/upper, lower -> upper, aliases, second owners, cycles and production -> xtask/tools. No Task-14 command installs/starts production or emits live evidence; Task 15 is the first production build/install.

- [ ] **Step 5: Commit every gate-producing byte before freeze.**

```bash
cargo xtask f57 task stage --task G6-14
cargo xtask f57 task verify-staged --task G6-14
git commit -m "build: complete final f57 candidate and release gates"
```

### Task 15: Freeze once, issue RELEASE_CERTIFIED, then explicitly admit production

**Files:**
- Read/execute only: committed `xtask/src/f57/{carrier,run_journal,p340_qualification,runtime_topology_certification,final_installed_generation,final_candidate,l2,l3,release_gate,production_activation}.rs`
- Read/execute only: committed `crates/platform/runtime/src/{topology,storage/runtime_ssd_exceptions}.rs`, `crates/platform/release/src/{generation,generation_approval,participant,activation_attempt,carrier_contract,production_activation}.rs`, `crates/platform/generation-activation/src/{final_installed,package_maintenance,production_admission,admission_store,admission_gate,resilience_admission}.rs`, `crates/platform/package/src/{manifest,trust,maintenance,finalization,lifecycle,participant}.rs`, `crates/platform/backup/src/{checkpoint,recovery_cut,ports,windows_components,windows_components/unlock,postgres16_log_retention,data_hdd_disaster_replacement}.rs`, `crates/platform/capability-graph/src/{compiler,canonical}.rs`, `crates/platform/authority-kernel/src/{lib,abi,generated_abi,dispatch}.rs`, `docs/evidence/f57-generation.v1.schema.json`, `docs/evidence/f57-capability-package.v1.schema.json`, `docs/schemas/f57-capability-package-trust-registry.v1.schema.json`, `docs/schemas/f57-generation-approval-registry.v1.schema.json`, `docs/evidence/f57-windows-server-component-set.v1.schema.json`, `docs/evidence/f57-data-hdd-bitlocker-unlock.v1.schema.json`, `docs/evidence/f57-evidence-signer-broker-windows-install-readback.v1.schema.json`, `docs/schemas/f57-runtime-ssd-exception-registry.v1.schema.json`, `docs/evidence/f57-production-activation.v1.schema.json`, `docs/evidence/f57-production-admission.v1.schema.json`, `docs/evidence/f57-production-admission-bypass-registry.v1.schema.json`, `docs/evidence/f57-postgres16-log-retention.v1.schema.json`, `docs/evidence/f57-data-hdd-disaster-replacement.v1.schema.json`, and `docs/evidence/f57-package-recovery-control.v1.schema.json`; `f57-production-admission.v1.schema.json` is the sole combined owner of admission, tagged hold and lease wires, so no separate hold schema path exists
- Read/execute only: committed Task-11 `crates/platform/backup/src/{safeguard,postgres16_windows,postgres16_log_retention,data_hdd_disaster_replacement}.rs` plus `docs/evidence/{f57-backup-storage-safeguard.v1.schema.json,f57-postgres16-windows-install.v1.schema.json,f57-postgres16-log-retention.v1.schema.json,f57-data-hdd-disaster-replacement.v1.schema.json}`, and current Task-13 `crates/platform/runtime/src/capacity/p340.rs`, `crates/platform/ups-contract/src/{lib,model,ports}.rs`, `crates/adapter/ups-windows/src/{lib,standard_power_status,signed_vendor}.rs` plus `docs/evidence/{f57-p340-soak-evidence.schema.json,f57-ups-contract.v1.schema.json}`; Task 15 typed-loads only the current P340 infrastructure profile and physical UPS branch and may neither infer them from prose nor substitute an unregistered future profile, adapter or schema
- Read/execute only: committed `scripts/windows/{build-msi,install-services,run-l2-candidate,archive-wal,test-postgres16-pitr,backup-restore-drill,run-p340-certification,test-power-shutdown,run-l3-release}.ps1`
- Read only: the 22 generated canonical Requirement facades and 185 committed handlers
- Read as initial bootstrap only: signed authority-storage manifest plus deployment manifest/signature/trust-bundle/storage-root/revocation/checkpoint absolute paths
- Write evidence only: the store-derived `<g6-data-hdd-evidence-root>` and controlled external/offline evidence media. RUNTIME_SSD is the disjoint union of reproducible Set A—including TPM-bound reenrollable machine-key/certificate metadata—and exactly four mutable Set-B classes: bounded POWER capsule, signed package-recovery continuation capsule, signed kernel slot pointer plus journal head, and reconstructible content-addressed signed native-code slots/cache. No fifth mutable exception is legal. Every authority/business byte remains on DATA_HDD or off-host media; SSD loss rebuilds Set A/Set B from authenticated build/HDD/off-host/TPM sources and never becomes data loss
- Repository files created/modified/deleted: none

**Interfaces:**
- Consumes: clean committed `HEAD`, selected client-stack decision, the current exact `OBSERVED_COMMITTED` predecessor generation, the G1 generation/declaration construction and activation authorities, controlled four-platform signing runners, Windows Server 2022 with exactly the current physical `P340_RELEASE72_HOUR` infrastructure certification profile, external append-only target, two offline encrypted HDDs, the physical UPS branch, and clean restore capacity. After build/install, `FinalInstalledGenerationAuthorityV1` creates the new declaration and terminal OBSERVED attempt; only then may `GenerationObservedReleaseSelectionPortV1` select that exact manifest, approval-registry ref and same-attempt plain participant-ACK set for candidate freeze. No release-local generation payload, envelope, ACK, approval, purpose, media, item, digest or verified wrapper is defined; any IaaS/provider-power input is an unregistered extra.
- Produces: one immutable `ReleaseCandidateV1` with exact generation/selection/package closure; six terminal mutually coherent current carriers; post-P340 topology certification; signed Final L2/L3 and fresh G0…G5 receipts; exactly one offline-verifiable `ReleaseCertificateV1{verdict=RELEASE_CERTIFIED}`; and, only after separate customer acceptance, one release-activation proof followed atomically by upper-owned genesis `ProductionGenerationAdmissionV1`, yielding `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED` or nonzero held failure. The three generation artifact fields plus selection ref exist only on the final candidate. `gate g6` never creates a G6 receipt. Certificate, activation proof and production admission are three distinct states; no route opens before the admission CAS and there is no partial release status or repository commit. Future IaaS has no implementation, current recipe registration, certificate input or activation terminal in this plan.

- [ ] **Step 1: Assert the release source is clean and immutable.**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 graph generate --check`

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Expected: PASS. Record the committed source-tree digest; any later repository change aborts this task and starts a new candidate.

- [ ] **Step 2: Build/sign final artifacts, then freeze exactly once.**

`<g6-data-hdd-evidence-root>` is exactly `<ValidatedDataRootV1>\evidence\release-candidate`, derived by final-handle verification of the signed storage manifest; it is not a caller-selected placeholder and may be on any legal non-reparse DATA_HDD path. The one journal is `<g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`. The unique causal order is: client validation → client artifact set → `WINDOWS_AUTHORITY_BUILD` → `WINDOWS_SERVICE_INSTALL` → final-installed generation/declaration creation → exact activation/ACK/`OBSERVED_COMMITTED` → observed-selection lease → offline schema manifest → CandidateBound Fresh-PG → final candidate freeze/bound → `POSTGRES16_PITR` → `BACKUP_RESTORE_CERTIFICATION` → P340 qualification plan/seven operations/closure → `P340_RELEASE72_HOUR` → runtime-topology certification → `POWER_SHUTDOWN` → Final L2 with all six carriers → L3 → fresh G0…G5 receipts → release certificate. A later stage cannot run early or be relabeled; every checkpoint is a strict authenticated prefix extension. Every IaaS/provider-power recipe or input is absent from this graph version and fails before STARTED.

Run: `cargo xtask f57 client-gate validate-selected --selection-receipt docs/decisions/f57-client-stack-decision.v1.json --candidate HEAD --release --storage-manifest <absolute-path> --deployment-manifest <absolute-path> --deployment-manifest-signature <absolute-path> --deployment-trust-bundle <absolute-path> --storage-trust-root <absolute-path> --storage-revocation <absolute-path> --storage-checkpoint <absolute-path> --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\client-stack-validation.v1.json`

Expected: as the sole absent-header creation grammar, all seven storage/bootstrap paths occur exactly once. The command independently validates storage/deployment trust, obtains private `ValidatedDataRootV1`, exact-confirms supplied bundle root equals the derived DATA_HDD root, create-new copies/fsyncs the seven fixed bootstrap files, and only then writes the immutable journal header with verified root binding and one unpredictable `gate_run_id`. Once the header exists, every re-entry and later command rejects all seven flags and resolves only archived header refs while re-probing the live volume. It then replays the projection-bound committed stack decision/archive and runs current four-platform validation; any trust/root/reparse/lifecycle/archive conflict stops release and cannot select another stack.

Run: `cargo xtask f57 client-build --validation <g6-data-hdd-evidence-root>\client-stack-validation.v1.json --candidate HEAD --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\client-artifacts.v1.json`

Expected: validation produces exactly one release-signed Windows, macOS, iOS, and Android package byte set and proves install/start/upgrade/capability/resource behavior on those exact bytes. `client-build` only exact-wraps those same four refs, emits the selected-stack auxiliary `G5_FOUR_PLATFORM` conformance result, and binds both to clean `HEAD`, the same graph/projection manifests, validation ref and exact `gate_run_id`; any rebuild, repackage, resign or byte substitution fails.

Run: `cargo xtask f57 carrier run --recipe WINDOWS_AUTHORITY_BUILD --candidate HEAD --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Run: `cargo xtask f57 carrier run --recipe WINDOWS_SERVICE_INSTALL --candidate HEAD --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: `WINDOWS_AUTHORITY_BUILD` produces one signed exact 23-field `WindowsAuthorityArtifactSetV1` closing the clean source tree, immutable Authenticode-signed `ep-core-server.exe` launcher, initial versioned `ep-authority-kernel.dll`, generated exact C-ABI/export readback, signed initial slot pointer/head, Authority manifest, graph-exact runtime deployment closure, exact Task-11 six-row component set, the exact ten-row production-admission bypass registry ref, strict `UpsAdapterManifestV1`, strict `Postgres16WindowsInstallContractV1`, all carrier packages/signature readbacks, production MSI, separately packaged off-host target agent, toolchain and build log. The ABI check proves exactly one named non-forwarded export, the 48-byte x64 table with offsets `[0,4,8,16,24,32,40]`, five nonnull callbacks, and byte-equal generated launcher/kernel/header/DEF artifacts. Every ACTIVE native/WASM/Hyper-V row is built from the same `HEAD`; every deferred row has no installable artifact. First-release local AI is deferred and only `NullAiProviderV1` exists. `WINDOWS_SERVICE_INSTALL` accepts no loose path. Its exact three staging inputs are traversed from one terminal build result; the signed set transitively supplies launcher/DLL/pointer, runtime closure, six-row component set, bypass registry, UPS/PG contracts and packages. Raw refs must exact-match before PASS.

The installer activates every graph-`ACTIVE` row and returns one positive carrier readback per active participant plus one absence readback per deferred row. Complete graph/closure/readback IDs biject; active/positive/declaration/generation/ACK IDs form the second bijection, while consumers and many-to-many item edges remain independent exact projections. On this clean first install, its safeguard collection binding fixes `expected_latest_backup_checkpoint_ref=None`; the installer still typed-verifies the independently verified current topology-signing trust pointer/manifest, current topology, singleton-target current storage manifest and fresh support/capacity/permission/lifecycle evidence, but accepts only an empty continuous/A/B checkpoint chain with `protection_state=INITIALIZING`. That state can make the infrastructure carrier PASS while production remains quarantined; sequence 1 only advances the immutable initial-population transition to `BOOTSTRAPPING`, and only sequence 2 or later plus the complete minimum-retention and A/B verified-copy closure may produce a fresh `HEALTHY` readback before PITR, release certification or activation. It installs the five Authority services with canonical quoted raw ImagePath+exact argv, the fixed G0 `EPF57EvidenceSignerBroker`, and three component services `EPF57BackupWriter|EPF57BackupCheckpointSigner|EPF57DataVolumeUnlockBroker`; it also registers/zero-side-effect tests the one recovery Scheduled Task, tests the on-demand passphrase helper and never installs target agent on P340. The existing trusted installer interprets the artifact-set-bound PostgreSQL contract and installs exact `ep-postgres16` as `NT SERVICE\ep-postgres16`, `UNRESTRICTED`, `DEMAND_START`, `NO_AUTOMATIC_RESTART`, with the frozen dependencies/privilege/DACL and `pg_ctl runservice` argv; it adds no PostgreSQL-specific PowerShell. Before DATA_HDD/storage/vault/config/TLS qualification it proves zero PostgreSQL process starts; after explicit start it emits strict `Postgres16WindowsInstallReadbackV1`, transitively authenticated by the exact 22-field signed install evidence. Engine is SSD Set A only; PGDATA including live `pg_wal` and database temp relations, archive staging, process/restore scratch, logs, TLS and config are final-handle DATA_HDD with zero reparse/ADS/alias/fallback. Parsed effective configuration proves HBA loopback hostssl/SCRAM; matching authenticated client probes separately prove `channel_binding=require` and role class. It exact-proves the 64/4/3 GUC and classified budgets; `fsync_writethrough` is only a compatibility pin pending same-file qualification plus the Task-15 UPS/cache/flush/power-cut join. The typed Event Log coverage ref proves the complete provider/bookmark/fixture scan with zero clear/drop/gap/token. The fixed EnterprisePlatform inventory is nine SCM rows + one task + one helper; complete host inventory exact-equals `10 + active_additional_windows_service_count` after this pinned PostgreSQL row and graph-active service rows. Every service/static/runtime/executable/pipe ACL, account/service/logon/restricted SID set, exact privilege vector and authenticated nonce/process binding is read back. The recovery-task row additionally exact-proves S4U principal policy, account rights/groups, runtime token attributes/integrity/elevation/restriction, parent folders, task registration, executable/pipe ACLs, fixed empty caller argv and six-operation protocol.

Before normal Authority startup, the unlock broker final-handle verifies all nine Set-A boot locators, pinned policy/chain/bootstrap authority and its restricted-LocalSystem token, then performs only explicit-thumbprint PUBLIC_KEY unlock for the compiled DATA_HDD identity. The install result binds `data_hdd_unlock_authority_ref`, `data_hdd_unlock_readback_ref` and broker bootstrap ref; the Authority recovery proof and POWER restart later repeat the same unlock readback. G0 broker starts separately, is `WAITING_FOR_DATA_HDD` until that root is verified, then returns READY with full static/runtime/readiness/storage closure and zero mutable SSD fallback. The service-install result's bypass-registry ref must equal the signed Authority-set ref and installed final-handle bytes. This bootstrap pointer intentionally predates and contains no final-installed generation or production admission; the subsequent installed-facts command creates generation 1. Missing/extra participant/edge/service/component/task/export, loose MSI/path, PID reuse, endpoint/role/slot swap, raw SCM command-line drift, `ChangeServiceConfig`, caller-controlled task/unlock inputs, EXTERNAL_KEY/auto-unlock, broker fallback or co-located target fails.

The installer also applies the frozen runtime-residency policy before its required reboot: Set A is reproducible bytes plus TPM-bound reenrollable key metadata; Set B is exactly the four generated mutable classes and no fifth. It sets all six page/swap/hiber/crash/minidump/WER rows `DISABLED`, moves VSS diff area and product quarantine to final-handle DATA_HDD locations, applies the exact seven-row telemetry policy, and verifies registry/provider settings plus actual handles/files after reboot. Whole-volume residency then classifies every allocated stream and requires all unregistered/inaccessible/partial/customer/canary/business-digest counters zero. A hidden pagefile, dump, quarantine, VSS, telemetry or broker-state fallback on SSD aborts install and production admission.

Run: `cargo xtask f57 generation activate-installed-release --candidate HEAD --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: xtask authenticates to the installed `EPAuthorityServer` and sends only the exact same-run precursor checkpoint plus terminal build/install result refs. `FinalInstalledGenerationAuthorityV1` verifies graph/closure/live-readback and active/deferred/consumer/many-to-many item projections, begins or adopts the immediately next generation, builds/signs forward/reverse items plus declaration from installed facts, and invokes the G1 coordinator. Success requires the same attempt `OBSERVED_COMMITTED` with exactly one fourteen-field plain ACK per active participant. Each ACK carries a create-new stored/reloaded `participant_apply_readback_ref`; every package row reaches its participant-specific transition, trust, operation/result and installed-state proof. Deferred rows have no ACK. Process loss adopts the exact attempt/readbacks and never scans, creates a second attempt, redispatches known work or resamples an ACK.

Run: `cargo xtask f57 candidate freeze --candidate HEAD --client-artifacts <g6-data-hdd-evidence-root>\client-artifacts.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\candidate.v1.json`

Expected: the freeze command verifies the client-artifact validation and the two typed terminal carrier-result chains, obtains the Authority artifact set, graph-exact runtime deployment closure/readback set and backup/recovery component set only through the terminal build result, and adopts the journal header's existing final `gate_run_id` without accepting one from argv. It refuses to begin unless `FinalInstalledGenerationAuthorityV1` can exact-reconstruct the just-installed terminal `OBSERVED_COMMITTED` attempt; the current selected manifest/declaration/readback must prove the exact CapabilityGraph-to-runtime-deployment bijection, all five Authority services, PostgreSQL install contract/readback, every installed backup/recovery capability and mandatory absence of every deferred participant. It first materializes and verifies the canonical signed 89-row artifact-signer registry. Starting from the canonical `RELEASE_CERTIFICATE_V1` schema root plus the minimal offline-manifest bootstrap—not from a not-yet-issued certificate object—it deterministically generates the complete discriminator- and `$ref`-reachable schema closure, exact-joins the 185 handler media contracts, copies/fsyncs the exact bytes under `schemas/repo/`, checks the generated descriptor-count/set-digest golden, freezes the closure checkpoint, and finalizes the one signed `closure_root=RELEASE_CERTIFICATE` manifest with its journal-bound `finalization_attempt_id`. That closure reaches the separately owned signed manifest/reverse-plan/plain-ACK generation media, Windows runtime-deployment/component-set media, backup topology/topology-signing-trust-manifest/topology-signing-trust-current-pointer/storage safeguard, PostgreSQL package-lock/install-contract/install-readback, UPS common manifest/status/command/ACK, the plain observed-selection record and pinned three-row approval-registry media without a release-schema nominal import.

Before reading or copying a generation object, freeze holds the G0 journal writer's exclusive guard, create-new appends/fsyncs/reloads the exact current checkpoint as `VerifiedGateRunJournalPrefixV1`, and calls only `GenerationObservedReleaseSelectionPortV1::begin_or_adopt_current_observed(candidate_run,deployment_id,authority_epoch,&preselection_prefix)`. The durable selection row stores that authenticated checkpoint's ref/sequence/terminal hash, a nonnil lease ID/owner and `binds_selection=false`. Freeze requires the returned selection's attempt ID/ref/digest/declaration/ACK set to byte-equal the final-installed wrapper, materializes the exact selection object, and retains—or after process death recovery-reacquires—the G0 writer guard through selection insertion, every pre-finalization journal extension and the first selection-bound append. During long copy, offline-schema and Fresh-PG work, `GenerationObservedReleaseSelectionCommandAuthorityV1::verify_progress` may accept only an authenticated equal prefix or strict same-chain extension that contains no final-release candidate-finalization record; `renew_exact` persists that monotonic checkpoint and extends the exact live lease while binding remains `PRESELECTION`. It never invents a heartbeat journal event and never binds merely because bytes were copied.

If the process dies at any cut, same-run recovery passes only the exact verified signed gate-run header to `load_exact_for_recovery`; that port derives the unique candidate-run key, exact-loads the immutable selection row, and returns its last persisted checkpoint ref, after which G0 alone authenticates that prefix. It accepts no caller prefix/ref/run fields and performs no latest scan. The same equality-or-strict-extension verification recovers both pre-finalization and later bound progress. An expired lease can resume only the same immutable run/selection after rechecking the unchanged OBSERVED pointer, terminal hash, exact ACK set and monotonic checkpoint binding; an irrecoverable expired freeze requires two distinct currently authorized humans and immutable persisted resolution evidence before the sole `FAILED_RELEASED` transition. Timeout alone never releases, deletes, replaces or reassigns a selection.

Freeze then uses only the verified selection plus explicit authority-generation bundle, reconstructs the approved manifest through `GenerationApprovalVerifierV1`, and requires the exact durable OBSERVED attempt. It copies selection, approval/storage trust inputs and every content-addressed manifest/reverse-plan/fourteen-field ACK/apply-readback/transition/operation/installed-state/declaration/graph/projection/runtime/component dependency into the final bundle and typed-reloads the whole graph. Every ACK is from the same attempt, includes `participant_apply_readback_ref`, exact-matches participant/item formulas and falls inside start-to-OBSERVED time bounds. Maintenance branches also close historical plan/current authorization/decisions/hold/full-cut checkpoint; no plan-finalization journal enters the bundle. Any missing/cross-participant/cross-cut/ref/media/time/variant dependency, generic proof, current/latest lookup, authority-root-only reference, CMS-wrapped ACK or discovery aborts before finalization.

Only the first durable `CANDIDATE_MANIFEST_FINALIZATION_STARTED{candidate_kind=FINAL_RELEASE,generation_observed_selection_ref=<exact selection_ref>}` is the binding point. The authenticated progress verifier changes `PRESELECTION` to `SELECTION_BOUND` exactly once and every later prefix must retain that record/ref. After the signed candidate is create-new stored and matching `CANDIDATE_MANIFEST_BOUND` is durable, freeze authenticates the extended prefix again; `bind_candidate_and_release` accepts only that private verified progress—not a raw candidate ref—derives its exact nonnull candidate ref, CAS-persists the checkpoint/candidate and returns the codec-reloaded `BOUND_RELEASED` selection. A crash resumes/adopts this last CAS and never silently releases the lease. An offline test then makes the authority-generation root unavailable and must still verify the whole copied selection/generation/runtime-deployment/component closure and build topology certification from the final bundle. The preceding bound schema manifest and CandidateBound Fresh-PG result both occur while the selection is still protected; Fresh-PG runs against clean committed `HEAD`, proves the 69-file baseline plus complete 47-file F57 suffix (`116` total) with zero unpoliced protected table, and never uses the public/direct `fresh-pg` CLI as candidate evidence. Both refs and their digests exact-match the candidate identity and run.

After every precursor is terminal, freeze signs the complete precursor checkpoint, durably appends `CANDIDATE_MANIFEST_FINALIZATION_STARTED` with one unpredictable `finalization_attempt_id`, and atomically create-new writes one immutable signed manifest binding that same ID, checkpoint, Fresh-PG receipt, signer/offline-schema refs, exact `generation_manifest_ref`, exact `generation_approval_registry_ref`, the canonical exact same-attempt `generation_participant_ack_refs`, the `RELEASE` client set, two pre-freeze carrier refs, source tree, tagged authority plus four client wrappers, graph/projection/facade/migration/toolchain, the exact final-installed `runtime_topology_declaration_ref`, header-derived `storage_root_binding`, run ID and `data_classification=PRODUCTION_SIGNED_NO_BUSINESS_DATA`. It never embeds the later selected infrastructure certification. The matching bound event follows; crash recovery signs once from those frozen bytes/refs or adopts exact bytes without new ID/time/signature/path.

Run: `git status --porcelain=v1`

Expected: no output. Any candidate artifact or repository mutation from here invalidates the manifest.

- [ ] **Step 3: Produce all release-bearing carrier evidence on that digest through the closed dispatcher.**

For every carrier below, Rust first derives/signs and content-addresses the closed `CarrierStagingPlanV1`, then fsyncs `TEST_STARTED` with the same unpredictable `execution_attempt_id` and singleton `start_context_refs=[staging_plan_ref]`; only afterward does it derive staging and copy exact inputs. P340 additionally requires its seven-operation qualification plan, five journal transitions, closure checkpoint and capacity input before its carrier plan. The first five recipes invoke only fixed AllSigned commands. POWER starts the five permanently `AUTO_START` SCM vectors through the Authenticode-covered immutable `ep-core-server.exe` launcher, which verifies the signed slot/head and delegates each role only through the exact versioned-kernel ABI; no role implementation lives in the PE. Continuation dormancy is absence of `ActiveRecordPath` inside the activation child key, never service disablement. Its 18-row object/action SDDL set remains exactly 18 when the control root gains tagged `POWER|PACKAGE|RESILIENCE|POSTGRES_LOG_RETENTION` protocol subroots: the tag is authorization data inside the existing broker pipe/root, not a nineteenth securable object or a broader ACE. Its runtime-SSD control capsule, fixed marker/composite-ACK/controller/failure/spool locators and nine opaque parsers plus the typed common-schema UPS ACK are signed-plan derived. The broker writes the API-commit marker before the one call; marker presence forbids retry, and only User32/1074 plus authenticated same-ID UPS schedule ACK forms durable acknowledgement. Missing ACK follows fail-safe clean shutdown/manual power-on/UNKNOWN. On postboot, marker+ACK initialize the bounded resume controller; a valid cross-boot success spool wins before context-change failure, and ServiceMain sends only the strict success draft while broker owns the acceptance tick and command spool. Finalizer asks the keyless raw facade to submit the exact frozen operation to the G0 broker, removes only `ActiveRecordPath`, verifies registered `AUTO_START`/STOPPED dormancy and records DISARMED. Every complete raw set then goes through the journal-frozen completion STARTED/BOUND/RECONCILED protocol; physical work is never rerun. Cross-signature order and context signing windows remain exact, partial/conflicting bytes are UNKNOWN/quarantine, and no terminal precedes completion plus required disarm.

The P340/POWER mechanics in the preceding paragraph are the only current infrastructure/power path. Any `IAAS_WINDOWS_SERVER_HDD_STRICT`, provider-power or future-profile input is rejected before staging or STARTED and cannot reuse this journal as an alternate branch.

Run: `cargo xtask f57 carrier run --recipe POSTGRES16_PITR --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: its staging input set is empty. The cut exact-loads the signed `BackupTopologyV1`; APPEND_ONLY and both `ROTATION_A|ROTATION_B` leaves each carry the same current-attempt strict `StorageSafeguardReadbackV1` ref and exact-match that topology, checkpoint, barrier and cut. The safeguard uses a fresh nonce/session/object and proves topology-pinned mTLS target, checked retention/capacity/reserve, zero expired partials/mutation, complete permission-negative matrix, consumed one-time just-written capability and physically disconnected/custodied A/B. For `E={PRODUCTION,CONTINUOUS,A,B}` and `D={failure,administration,credential,custody,location}`, it proves every one of the `C(4,2)*5=30` pairwise inequalities `domain_d(x) != domain_d(y)` and rejects shared tenant/admin, inherited credential, co-custody, same facility, provider snapshot in the production account, and two labels on one device. External append-only readback, recovery-only key use, PostgreSQL/WAL/attachments at one signed full authority cut, and exactly two distinct offline-media refs must all PASS; any stale/reused/unknown/shared-domain/attached state is `NON_SUPPRESSIBLE_RISK`. The outer PITR root remains exact 16 fields and the existing 18/17/30 subordinate registry is unchanged.

Run: `cargo xtask f57 carrier run --recipe BACKUP_RESTORE_CERTIFICATION --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: its staging input set is empty. Before carrier STARTED, the signed `recipe_specific=RECOVERY_CERTIFICATION` plan freezes one unpredictable `certification_id` and the exact ordered rows `1/INITIAL_RESTORE_VERIFIED`, `2/CANDIDATE_MEASURED`, `3/CERTIFIED`, each with its own distinct unpredictable `recovery_execution_attempt_id`. The flow authority create-new stores the strict recovery policy exact-binding that plan, candidate, clean cut, one outer carrier `execution_attempt_id`, certification ID and three subordinate IDs. The dispatcher invokes the fixed signed restore script three times under the one TestID/outer physical attempt and passes only the matching signed-plan subattempt ID; callers cannot supply it. Every raw context repeats the outer ID, while its mandatory `recovery_execution_attempt_id` and policy ref exact-match the corresponding plan/policy row. Runs 1 and 2 create only raw `00|01`; run 3 creates raw `02`, after which Rust creates the one completion over all three. Any missing/duplicate/mixed outer or subordinate ID, policy/plan/certification/phase drift, failure or partial raw set leaves that outer attempt UNKNOWN/non-green; it cannot rerun a started subattempt, restart phase 1, finalize early or select a better result. Another physical attempt requires a new run ID and final candidate.

Run the one current infrastructure-capacity recipe:

Run: `cargo xtask f57 carrier run --recipe P340_RELEASE72_HOUR --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: before carrier STARTED, the fixed P340 signer first issues the one candidate-bound policy attestation; one qualification authority then captures one trusted issue time, one unpredictable `qualification_id`, seven unpredictable operation-attempt IDs and the attestation-bound strict plan before any helper side effect. It executes the fixed ordinals `01..07` only through `P340_QUALIFICATION_OPERATION_STARTED|COMPLETED|UNKNOWN|RECONCILED`, where STARTED has exactly `{qualification_id,operation_kind,execution_attempt_id,qualification_plan_ref,started_at_unix_ms}` and no `output_ref` or sixth field. Every output path comes only from the authenticated plan/store; a started operation is never rerun. After seven terminal outputs, `P340_QUALIFICATION_CLOSURE_BOUND` and its checkpoint authenticate the exact policy attestation and seven typed helpers—including the same-host/same-HDD forced-power-cut `HddFlushVerificationV1`—before the capacity input is signed. Ordinal 06 starts from a genuinely clean SSD: admission remains closed, two authorized humans use the off-host 48-digit recovery-password ceremony to unlock DATA_HDD, and the helper proves the exact eight-step reenrollment chain from recovery-password unlock through new TPM key/certificate/PUBLIC_KEY protector, higher authority+NV commit, normal-reboot broker unlock, old-protector removal and closure. It persists no secret, never reconstructs an old CNG key from public metadata, exact-binds the new bootstrap/unlock refs and never opens admission. Its duration uses only STARTED's durable `started_at_unix_ms` and the same-attempt signed `SsdCleanReinstallRestoreReadbackV1.completed_at_unix_ms`; helper start byte-equals STARTED, checked ceiling milliseconds-to-seconds makes `28_800_000` ms pass and `28_800_001` ms fail, and the COMPLETED journal row supplies only the fixed output ref. Their actual trusted signing times fall only in their payload-time five-minute/expiry windows. The capacity input exact-repeats the plan, seven refs and closure checkpoint, embeds exactly 90 consecutive complete UTC `QUALIFICATION_SYNTHETIC` days ending the day before carrier STARTED, uses the last 30 as the unique suffix, recomputes every table/HDD-growth 30/90-day nearest-rank P95, and carries only the exact graph-derived queue-ID set. The signed staging plan's typed input set is exactly `{P340_CAPACITY_INPUT_MANIFEST,P340_CERTIFICATION_POLICY_ATTESTATION}` and carrier STARTED still carries only singleton `start_context_refs=[staging_plan_ref]`. It then proves one boot-scoped exact `259200000`-millisecond window at the exact 15/3/2+1 session and 11/5/2/2 action mixes with automation, backup, audit, and heavy-report overlays. Real WMI CPU identity canonicalizes only to `INTEL_CORE_I5_10500`; RAM checked-sums to 32 GiB; raw SSD/HDD capacities meet their device floors and are each at least the exact NTFS volume total; filesystem, input, watermark and every sample agree on those volume totals. Scanner provider/policy match the host input, engine/digests are nonempty/nonzero, and authenticated definition metadata owns its issue time. ErrorRate checked-sums read/write/high-risk/attachment completions, WAL is `0 -> monotonic -> positive`, no reboot/drop/retry occurs, and each frame/time-series/readback/input/outer artifact cross-matches boot/source, typed CPU SKU, RAM, storage roles/capacities and sample-1 HDD fill. The raw/nested signatures fall inside the physical-finish five-minute/expiry windows. The exact checked five-percent/20-GiB watermark, ten nested roots, seven reachable helpers, Set-A/four-Set-B whole-volume residency, eight Windows persistence rows, seven telemetry rows, fixed 25-metric predicates, histogram/sample chain and capacity certificate all exact-share the soak context/policy/input and pass; the terminal result remains `SINGLE_DISK_DEGRADED_PRODUCTION`.

After terminal P340 and before POWER, the coordinator calls only `RuntimeTopologyCertificationAuthorityV1` with verified candidate, terminal result/checkpoint and a fresh sealed live readback. It exact-loads the frozen generation/approval/fourteen-field ACK plus participant-apply-readback closure from the explicit bundle, reconstructs private generation proof, and compares the entire tuple to durable OBSERVED. Every ACK/readback/transition participant-item binding and time is verified; no CMS wrapper or release-local generation nominal exists. It derives, stores and typed-reloads one plain topology certification. Pre-P340, current/latest substitution, stale/mixed/cross-participant closure, candidate/profile/storage/policy/host/20-user/259200-second drift, a second object or a future profile fails. This is the POWER precondition and certificate input, not another carrier or signer row.

Run the one current power-safety recipe:

Run: `cargo xtask f57 carrier run --recipe POWER_SHUTDOWN --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: before deriving the POWER staging plan, the dispatcher requires and typed-reloads the same `VerifiedRuntimeTopologyCertificationV1` produced immediately after terminal P340; a raw ref, missing object, changed live topology, declaration-only proof, desired-only generation or stale/mixed participant ACK set is not enough. Typed staging inputs remain exactly `{UPS_IDENTITY_READBACK,UPS_POWER_WRITE_CACHE_POLICY}` from the same terminal P340 chain, and STARTED carries only the plan ref. The plan freezes exact `EPAuthorityServer`, `EPAuthorityControl`, both signing facades, five canonical numeric SIDs, installed Authenticode binary, permanent `EPF57PowerShutdownContinuation` `AUTO_START` row with dependencies `[CryptSvc,EPAuthorityControl,EPAuthorityServer,EventLog]`, exact source-order argv, 18 object/action SDDL rows, `NO_BUSINESS_DATA` capsule capped at 32 MiB, fixed activation/state/success-spool/raw paths and the six broker output discriminators `EVENT|AUTHORITY_RECOVERY_PROOF|RESTART|DISARM|FAILURE|FAILURE_CLEANUP`. It re-verifies permanent descriptors and never repairs them; the facades have no key ACE. No attempt changes service configuration. Same-host UPS/cache/flush qualification, actual AC loss/on-battery/output-maintained state, first 900-second crossing, authenticated command ACK and actual outlet off/restore cycle must all independently PASS; an ordinary reboot cannot substitute.

After STARTED, Rust verifies immutable permanent descriptors and securely creates/readbacks attempt-owned descriptors across the exact 18-row object/action set with activation absent, records ARM/row 1, runs prepare-only PowerShell, persists QUIESCE/UPS_TRIGGER and all nine opaque-parser captures plus the typed UPS status/ACK objects, then WAL-persists dispatch intent, permanent HDD object, row 2 and PRE_SHUTDOWN_COMMITTED before activation. Broker persists marker with one `requested_at_unix_ms`, makes the only shutdown call and may commit composite ACK only from exact User32 plus same-command/same-digest typed UPS ACK. On boot it resolves permanent pre-controller failures before any controller/start; exact marker+ACK creates the controller, which advances through dispatch-ack, HDD mount, journal/state, PostgreSQL recovery, durable cuts, UPS restored and success persistence. ServiceMain supplies draft values only; broker fsyncs command spool and projects EVENT/proof/RESTART. Valid spool/controller success remains authoritative across another boot. Finalizer uses the raw signing facade/G0 broker operation, records row 3, removes activation, verifies `AUTO_START` dormancy, creates DISARM and records row 4 before completion. Failure produces signed failure+cleanup, FAILED and UNKNOWN. All nine opaque parsers plus the typed UPS objects, marker/ACK/request time, controller/spool and four state rows must exact-match one run/attempt/continuation/barrier; no User32-only acknowledgement, API redispatch, human-triggered finalization, descriptor repair, service disablement, extra output, facade key use, cross-role signing or capsule-only offline dependency is accepted.

After Windows Server, DATA_HDD, PostgreSQL and the normal Authority recovery endpoint are ready again, run the identical command a second time:

Run: `cargo xtask f57 carrier run --recipe POWER_SHUTDOWN --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`

Expected: explicit candidate/run/recipe lookup adopts the one already STARTED execution attempt; it does not create a second plan/attempt, invoke PowerShell, call `InitiateSystemShutdownExW`, allocate another UPS command ID, reschedule the outlet or repeat any physical fact. It first invokes the same control-broker supervisor audit, adopts/completes the authenticated controller and success spool, and only then asks the raw signer for the first exact raw envelope, appends state row 3/`POST_RESTART_COMPLETED`, removes only `ActiveRecordPath`, proves the permanent service remains `AUTO_START`/registered/STOPPED, persists DISARM, appends state row 4/`DISARMED`, and finalizes completion/result/terminal. If the first raw envelope does not yet exist, the finalizer samples `raw.context.finished_at_unix_ms` exactly once immediately before constructing its immutable payload and requires `restart.completed_at_unix_ms <= raw.context.finished_at_unix_ms < raw.context.expires_at_unix_ms`; the CMS issuance window begins at that frozen raw finish. If exact raw bytes already exist, it adopts their finish and signature without resampling, overwriting or resigning. Restart completion, service-login time and operator-login/re-entry time can neither replace that raw finish nor impose an operator-login SLA. Boundary goldens cover equality at restart, one millisecond before expiry, equality with expiry, absent/existing raw at every crash cut and a second identical invocation; any conflict, resample or failure branch ends UNKNOWN/quarantine before Final L2.

- [ ] **Step 4: Run final L2 after the selected long carrier tests, then aggregate L3.**

Run through the frozen 18-row fixed-host/final-handle trust registry: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_RUN_L2_CANDIDATE_V1 -- -TargetGate RELEASE_CERTIFIED -CandidateManifest <g6-data-hdd-evidence-root>\candidate.v1.json -BundleRoot <g6-data-hdd-evidence-root> -RunJournal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`.

Expected: L2 PASS on the frozen candidate after the 72-hour/restore work, so its 90-day policy window is not consumed by those long tests; earlier G4/G5 L2 receipts are not reused. The script hashes the exact signed release-candidate manifest bytes and runs `cargo xtask f57 verify --level l2 --candidate <candidate-manifest-sha256> --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\l2-evidence.v1.json`; the output is a verified-CMS signed `L2CandidateEvidenceV1` with exact manifest ref, separate identity digest, run ID, bijective TestID/result pairs, and the latest checkpoint. Through the graph-generated conformance manifest it dispatches the auxiliary TestIDs for selected-stack `G3_SHELL` and `G4_CTC_UI_API` only when absent, exact-joins the artifact set's same-run auxiliary `G5_FOUR_PLATFORM`, then first starts candidate-bound `T-F57-CLI-007` over the frozen release package bytes. A preserved Tauri fixture can never enter a Flutter-selected release, and no pre-candidate Requirement result is relabeled.

Final L2 also public-queries and freezes the exact four current `ObjectiveClosureBindingV1` rows, all `CLOSED`, including procurement's three owner-tagged facts and distinct-reviewer proof. Every state/evidence/fact/review ref must be a same-run authorized result, and all reachable expiry values shorten the L2 lifetime. Any WAITING/reopened/stale/spoofed closure stops before L2 PASS.

- [ ] **Step 5: Issue the certificate and verify the complete signed chain offline without changing the repository.**

Run through the frozen 18-row fixed-host/final-handle trust registry: `cargo run -p powershell-trust-tool --locked -- execute --script-id F57_PS_RUN_L3_RELEASE_V1 -- -CandidateManifest <g6-data-hdd-evidence-root>\candidate.v1.json -L2Evidence <g6-data-hdd-evidence-root>\l2-evidence.v1.json -BundleRoot <g6-data-hdd-evidence-root> -RunJournal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl`.

Expected: the wrapper validates all supplied absolute paths and forwards them byte-for-byte. It first runs `cargo xtask f57 verify --level l3 --candidate <candidate-manifest-sha256> --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --l2-evidence <g6-data-hdd-evidence-root>\l2-evidence.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --out <g6-data-hdd-evidence-root>\l3-evidence.v1.json`, where the argv digest is the exact signed manifest-byte SHA. L3 exact-loads the final-L2 due TestID set and the same six auxiliary `ReleaseCarrierResultV1` refs, starts only remaining release/provider due TestIDs, and lets graph-bound handlers turn carrier evidence into canonical requirement results; auxiliary IDs never enter `test_results`. It emits a signed envelope whose `final_l2_evidence_ref` exact-hashes the supplied L2.

Before certificate construction, L3 typed-reloads Final L2, requires its four-row objective vector byte-for-byte, and independently verifies all four generations are still current under the later checkpoint. A reopen/mutation between levels is a hard failure, never a refreshed vector.

When `gate g6` freshly emits G4 and G5 receipts under the verified `ReleaseCandidateV1`, both receipts must byte-equal Final L2's four-CLOSED objective snapshot while retaining their own G4/G5 due and probe sets. An older standalone DevelopmentSlice G4 receipt with procurement WAITING, or a final-context G4 receipt that repeats that WAITING row, is rejected rather than treated as a prerequisite.

Only after L3 is bound does the wrapper invoke `cargo xtask f57 gate g6 --candidate-manifest <g6-data-hdd-evidence-root>\candidate.v1.json --l2-evidence <g6-data-hdd-evidence-root>\l2-evidence.v1.json --l3-evidence <g6-data-hdd-evidence-root>\l3-evidence.v1.json --bundle-root <g6-data-hdd-evidence-root> --run-journal <g6-data-hdd-evidence-root>\gate-run.jcs.jsonl --evidence-out <g6-data-hdd-evidence-root>\g6`. That command freshly emits or exact-adopts G0, G1, G2, G3, G4 and G5 receipts in ascending order, each only after its frozen finalization checkpoint/record, then forms six canonical `GateReceiptRefV1 { gate, artifact }` rows sorted by `(gate ordinal,uri,sha256)` and typed-loads every row's matching signed receipt. It revalidates the same four CLOSED objectives through L3; exact-loads the candidate's three frozen generation fields; reconstructs private `VerifiedGenerationApprovalRegistryV1` from the candidate/storage pin and the same private `VerifiedGenerationManifestV1` only through `GenerationApprovalVerifierV1`; and revalidates the approval-registry provenance, exact same-attempt OBSERVED participant-ACK set/formulas, every ACK's `acknowledged_at_unix_ms` interval and no-CMS/plain-wire shape, and byte equality to the generation state used by topology certification. It also revalidates procurement facts/reviewer and every authorized closure ref. Generic generation proof, latest-database substitution, wrong/ambient registry, missing/extra/mixed/time-invalid/signed/enveloped ACK or different topology-certification generation cannot enter certificate issuance. Only then does it freeze the complete certificate input checkpoint and durably append the exact five-field `EVIDENCE_ENVELOPE_FINALIZATION_STARTED{artifact_kind=RELEASE_CERTIFICATE,finalization_attempt_id,frozen_input_checkpoint_ref,issued_at_unix_ms,expires_at_unix_ms}` record; `kind`, signer key `RELEASE_CERTIFICATE_V1`, and every other alias are forbidden in this journal field. The constructor selects `verified_evidence_finalization(EvidenceEnvelopeKindV1::ReleaseCertificate)`, takes its ID, checkpoint and both time fields only from that frozen record, then independently exact-validates the `RELEASE_CERTIFICATE_V1/NONE` signer-registry row before signing/create-new or exact-adopting the certificate once. It proves Final L2 has exactly the frozen 149-row vector/digest, L3 exactly the frozen 36-row vector/digest, the vectors are disjoint with union 185, and all six carrier auxiliary IDs remain only in `carrier_refs`; it writes `g6\release-certificate.v1.json` only if all carriers and objective closures pass. Certificate expiry cannot exceed any objective state/evidence/fact/review result, the bound offline-schema manifest, any other consumed time-bearing evidence input or the journal run; immutable candidate/static content refs do not participate in expiry aggregation. It never discovers a default file, accepts an unsigned/bare payload or `SignedArtifactRefV1` receipt substitute, reruns a started TestID, includes an auxiliary ID in the due-set, reorders the unique causal sequence, obtains certificate time/ID from a live clock or latest mutable state, confuses event and signer naming domains, or selects a better outcome.

Run: `cargo xtask f57 evidence verify --receipt <g6-data-hdd-evidence-root>\g6\release-certificate.v1.json --bundle-root <g6-data-hdd-evidence-root> --expect-type RELEASE_CERTIFICATE_V1 --offline`

Run after the Authority has recorded the exact two-person customer acceptance fact: `cargo xtask f57 production activate --receipt <g6-data-hdd-evidence-root>\g6\release-certificate.v1.json --bundle-root <g6-data-hdd-evidence-root> --acceptance-id <approved-single-disk-acceptance-id>`

Expected: certificate verification completes first and is still not activation. `EPAuthorityServer` resolves two-person acceptance and exact five-risk/20-user/safeguard closure for the current P340 profile, creates or adopts one activation attempt, captures all four fresh sealed readbacks and persists only `LIVE_READBACK_BOUND` after graph/active/deferred/consumer/item/ACK equality. It additionally requires fresh `HEALTHY` backup state with exact current-root binding; all 30 backup-domain inequalities; exact PostgreSQL-log policy (`max_age=30d`, `max_total=20GiB`, current log undeletable, minimum seven days, legal-hold preservation, typed `EPAuthorityControl` cleanup only, PostgreSQL identity unable to delete history); and no capacity/resilience hold. DATA_HDD free space below the stricter of the existing configured yellow limit and 50 GiB pauses batch work; below the stricter of the existing red limit and 40 GiB creates the global hold, so the existing P340 approximately 100/50-GiB and platform 100-GiB floors remain authoritative where stricter. It passes private `VerifiedProductionActivationReadyV1` to the upper admission authority, whose sole joint transaction appends activation `ACTIVATED`, creates the genesis `ProductionGenerationAdmissionV1`, advances its head and `business_api_generation`, then commits once. The gate exact-loads the same signed-set/install-evidence ten-row bypass registry and immediately rechecks current OBSERVED equals admitted and no hold exists. Every subsequent business request is accepted only by same-lock creation/adoption of one `ProductionAdmissionExecutionLeaseV1`; command/query completion or exact-once handoff terminalizes it, and a hold cannot drain/barrier while an intersecting ACCEPTED lease remains except for the exact disaster fence. Response-loss adopts the one joint result or one request lease; every statement/commit/race crash exposes neither side or both and no write after a committed barrier. Stale certificate, expired/one-person acceptance, missing safeguard, shared backup domain, log-retention drift, co-located target, one offline disk, desired-only/mixed readback, admission/lease conflict, ambient/changed bypass registry, any IaaS/provider-power input or graph/resource drift fails closed; repairable live failure remains `FAILED_HELD` and retries only the same activation ID with four new readbacks. A normal current-root rotation must already have created its tagged global hold before the config CAS and must remain held throughout `TRANSITIONING|BOOTSTRAPPING`; only fresh `HEALTHY`, exact cause tuple/new-root binding and the cause-bound final admission CAS append `REOPENED`. A disaster replacement is never smuggled into activation: the dead old disk needs no fresh `HEALTHY`, but release stays held until the sole two-person/off-host-trust/checkpoint/fence/higher-epoch-and-storage-generation/new-BitLocker-volume-and-manifest/clean-restore/PITR-full-reconciliation/fresh-A-B-bootstrap/P340-recertification chain completes. Terminal status is exactly `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED`; it never claims RAID, HA or an IaaS terminal.

Run: `git status --porcelain=v1`

Expected: verifier uses only its minimal envelope/ref/locator/offline-manifest/JCS/CMS/bootstrap TCB, validates the generated transitive descriptor/object closure without network/discovery, and reaches final-installed generation, reverse plans, approval registry, deployment closure/readback, components and fourteen-field ACKs. For each package item it traverses pure desired item -> registry/package -> implementation closure/scope, then ACK -> typed apply readback -> per-participant transition -> frozen trust, operations/results and installed readback; maintenance continues through reservation/historical plan/live authorization/decisions/hold/full-cut checkpoint, and production through admission predecessor/delta envelope. It never follows a plan-finalization journal. It re-runs graph/closure/readback and ACTIVE bijections, independent consumer/item projections, trust pins, CMS roles, lifecycle/action/scope formulas, rollback-ID/variant relations and admission equality. Missing/orphan/out-of-set/cross-participant/cross-cut/target-only/current substitute, fictitious predecessor on a new-install rollback, or package-local `APPLIED_VERIFIED` treated as OBSERVED/admitted fails. Certificate PASS still means 185/185 and all release invariants; production remains separately genesis-admitted. Repository stays unchanged.

## Final completion check

After Task 15 has completed, perform read-only verification; do not rerun or replace a release-bearing test result:

```powershell
git status --porcelain=v1
cargo xtask f57 graph generate --check
cargo xtask f57 evidence verify --receipt <g6-data-hdd-evidence-root>\candidate.v1.json --bundle-root <g6-data-hdd-evidence-root> --expect-type RELEASE_CANDIDATE_V1 --offline
cargo xtask f57 evidence verify --receipt <g6-data-hdd-evidence-root>\g6\release-certificate.v1.json --bundle-root <g6-data-hdd-evidence-root> --expect-type RELEASE_CERTIFICATE_V1 --offline
```

Expected: empty Git status, zero graph drift, one candidate digest across Final L2/L3 and the six coherent current carrier receipts, certificate `RELEASE_CERTIFIED`, and—only after the separate activation command has produced both its terminal proof and matching genesis admission—deployment `PRODUCTION_ACTIVATED_SINGLE_DISK_DEGRADED` on P340. Without the current `ProductionGenerationAdmissionV1`, the unique route gate stays quarantined even when certificate and activation proof verify. Admission requires exact P340/physical-UPS evidence, certified external backup, two offline media, all 30 backup-domain inequalities, recovery custody, BitLocker/TPM, clean hardware, valid recovery/capacity/log-retention evidence, no live tagged root-rotation/disaster/UPS/capacity hold, four fresh live readbacks and two-person acceptance of all single-disk risks. Every IaaS/provider-power profile, schema, result or activation terminal is absent from this graph version and must fail before STARTED.
