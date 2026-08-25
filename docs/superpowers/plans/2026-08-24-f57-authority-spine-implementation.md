# F-57 G1/G2 Authority Spine and CTC Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:test-driven-development for every task, then superpowers:verification-before-completion before claiming either gate. Execute only after a valid `G0_BOOTSTRAP_GREEN` receipt exists.

**Goal:** Build the Windows authority trust boundary, dynamic authorization, atomic command/evidence path, durable closed-loop engine, and the feature-first PostgreSQL persistence required by the CTC-01 development slice.

**Architecture:** PostgreSQL 16 is the only authoritative database. Business repositories accept only an unforgeable `AuthorizedPgTx`; every command commits current state, feature-owned immutable facts, audit, Outbox, and its receipt in one transaction. The existing `ep-platform-flow` crate owns durable Objective/Obligation/Effect/Evidence/Cycle execution. Seven feature-first business crates own CTC-01 state and communicate only through generated public commands and committed facts.

**Tech Stack:** Rust 2021/MSVC, SQLx, PostgreSQL 16, Windows Server 2022, Windows CNG/TPM/BitLocker/PIV integration seams, PowerShell, property/fault tests, and the G0 `cargo xtask f57` evidence runner.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md`, `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`, and `docs/superpowers/plans/2026-08-24-f57-converged-program.md`

**Status:** `READY_NOT_AUTHORIZED` / `BLOCKED_BY_G0`; no task in this file may start until both separate development authorization and a valid `G0_BOOTSTRAP_GREEN` receipt exist.

## Global constraints

- Verify the input receipt first:

  ```text
  cargo xtask f57 evidence verify \
    --receipt target/f57/evidence/g0/bootstrap-receipt.v1.json \
    --bundle-root target/f57/evidence \
    --expect-gate G0_BOOTSTRAP_GREEN
  ```

- The verified G0 receipt authorizes dependency entry and freezes the interfaces to consume; it is not current evidence after the tree changes. Each G1/G2 gate reruns all predecessor conformance on its own current repository tree and emits a fresh receipt set with one capability-graph, generator, migration-reservation, toolchain, and repository-tree identity. A mismatch is a hard stop, never a warning.
- Migrations are limited to the exact versions and paths in the master plan §4.1. One migration writes only its named schema. A row changes from `RESERVED_NOT_CREATED` to `CREATED` only in the same commit as its SQL.
- Every `fresh-pg` and G1/G2 gate first verifies and applies the candidate-bound 69-file pre-F57 baseline from G0, then the contiguous `CREATED` F57 suffix through the requested endpoint. It never scans/apply-selects arbitrary SQL and never applies any of the 310 legacy or nine absent baseline paths.
- Normative Files-list expansion: every task below that creates one or more `db/migrations/**` files also modifies `docs/f57-migration-reservations.v2.tsv`, changing exactly those rows to `CREATED`. That registry path is part of the task's exact staging/commit set even where the repetitive Files or Commit shorthand omits it. Fresh PG before that commit is explicitly engineering rehearsal, not candidate evidence. G1/G2 aggregate gates rerun the full due prefix from clean committed `HEAD` and issue the only candidate-bound Fresh PG receipt; aggregate tasks never delay row transitions.
- Business code cannot receive a raw pool, `PgConnection`, `PgTx`, `&mut dyn Tx`, session-scoped security GUC, database URL, or SQL text.
- `SET`, `set_config(..., false)`, and session residue are forbidden on business connections. Verified context is transaction-local; setup, verification, readback, rollback, or cleanup failure destroys the connection instead of returning it to the pool.
- Migration, recovery, context-issuer, and operations identities use separate least-privilege pools and cannot be passed to business repositories.
- The client cannot assert actor, principal, legal entity, policy result, authority epoch, MFA, SoD result, device posture, or verified generation. The server derives and verifies them.
- Roles, positions, and job names are grant templates only. Runtime decisions use current principal grants, revocations, delegations, scope, field/query-use rights, device/risk conditions, and authority epoch.
- Network/provider calls never occur inside the command transaction. An ambiguous external result becomes `UNKNOWN` and cannot be retried until reconciliation produces owner-accepted evidence.
- Every legal-entity-owned business table has the common security/version columns, same-entity composite foreign keys, compare-and-swap versioning, `ENABLE RLS`, and `FORCE RLS`.
- G1 and G2 do not implement HTTP/UI clients, L2, installers, real P340 TPM/PIV evidence, production backup, or ransomware recovery. Passing G2 does not imply `DEV_SLICE_GREEN`.
- No Requirement is first due at `G2_CTC_DATA`. Its receipt therefore has zero canonical Requirement `test_results` and exactly 26 registered typed `SliceProbeEvidenceBindingV1` refs in `probe_results`; they cannot satisfy or partially pass a parent Requirement whose first due profile is G4, G5, or G6.
- G0 owns the generated canonical Requirement facades. G1 tasks create only concrete handler modules under `testkit/src/f57_cases/g1/`: G1-02 owns `authz_matrix.rs` (`AUTH-001..007`); G1-04 owns `authority_command.rs` (`GOV-001`,`GOV-008`,`INT-005`) and `transactional_evidence.rs` (`GOV-009`); G1-05 owns `generation_faults.rs` (`GOV-003`); G1-06 owns `automation_fault_matrix.rs` (`AUT-001..007`). Each task registers exactly those IDs in the CapabilityGraph handler binding and regenerates the facade manifest, test manifest, projection manifest, and `testkit/src/f57_cases/generated_bindings.rs`; it never edits `testkit/tests/f57_*.rs` or `xtask/src/f57check.rs` by hand. The generated bindings must compile and exact-match the delivered registry before commit. The G1 gate invokes all 19 first-due G1 symbols by exact name and rejects any additional, missing, unlinked, skipped, or `NOT_DELIVERED` result.
- Any G2 task that adds a command, query, fact, authorization rule, file operation, or feature owner first modifies the corresponding `ActivationReady` nodes in `docs/capability-graph/f57-core.v1.json`, then runs the generator. `src/public/generated.rs` and every changed member named by `docs/generated/f57/projection-manifest.v1.json` are generated outputs and may not be authored by hand. G2-07 only verifies `graph generate --check`; it cannot retroactively add missing graph semantics.
- Every task runs in its own clean F-57 worktree and begins with `cargo xtask f57 task begin --task <task-id>`. Every `Commit:` step implicitly requires `cargo xtask f57 task stage --task <task-id>` and `cargo xtask f57 task verify-staged --task <task-id>` first. Raw `git add`, brace/glob expansion, directory staging, pre-task dirty paths, and cached-set drift are forbidden.

## 1. Frozen dependency and migration map

| Task | Requires | Exact migrations | Gate contribution |
|---|---|---|---|
| G1-01 deployment/storage/secrets | `G0_BOOTSTRAP_GREEN` | `90000`, `90100` | G1 |
| G1-02 principal authorization | G1-01 | `90200` | G1 |
| G1-03 verified transaction context | G1-02 | `90210` | G1 |
| G1-04 command/evidence commit | G1-03 | `90300`, `90310` | G1 |
| G1-05 generation ACK/pins | G1-01, G1-04 | `90400` | G1 |
| G1-06 objectives/reconciliation/capacity | G1-02, G1-04, G1-05 | `90500`, `90600` | G1 |
| G1-07 authority aggregate | G1-01, G1-02, G1-03, G1-04, G1-05, G1-06 | none | `G1_AUTHORITY_SPINE_GREEN` |
| G2-01 files/quarantine | G1-01, G1-03, G1-04 | `90700` | G2 |
| G2-02 customer/contract | `G1_AUTHORITY_SPINE_GREEN`, G2-01 | `90800`, `90900` | G2 |
| G2-03 sales order | G2-02 | `91000` | G2 |
| G2-04 procurement | G2-03 | `91100` | G2 |
| G2-05 inventory fulfilment | G2-03, G2-04 | `91200` | G2 |
| G2-06 invoicing/receivable/cash | G2-03, G2-05 | `91300`, `91310` | G2 |
| G2-07 authority/domain integration | G2-01, G2-02, G2-03, G2-04, G2-05, G2-06 | none | `G2_CTC_DATA_GREEN` |

The exact paths are not restated with aliases: each task below uses the path registered in the master migration table.

## 2. Frozen public contracts

The following names and fields are shared with G0, G3–G6, generated protocols, and evidence. Do not introduce a parallel type. G0-01 is the sole Rust/schema owner of `PrincipalKindV1|PrincipalRefV1` in `crates/foundation/src/principal.rs` and the zero-import `docs/evidence/f57-foundation.v1.schema.json`; `crates/foundation/src/identifier.rs` is the sole Rust owner of the strict foundation `UuidV1` nominal. Every G1/G2 task imports those exact nominals and may not modify, copy, regenerate, or locally redefine them; no task may introduce a second UUID wrapper or use raw `uuid::Uuid` in a wire contract.

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QueryUseV1 {
    Filter,
    Sort,
    Group,
    Aggregate,
    Search,
    Export,
}

pub struct VerifiedSecurityContextV1(
    /* G1-03-owned private, non-serialized constructor product derived from authenticated
       principal/session/legal-entity/device/capability/MFA/SoD/policy/epoch/generation facts */
);

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SecurityContextPurposeV1 {
    #[serde(rename = "EP-F57-SECURITY-CONTEXT-V1")]
    SecurityContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum SecurityContextMacAlgorithmV1 {
    #[serde(rename = "HMAC_SHA_256")]
    HmacSha256,
}

pub struct SecurityContextMacV1([u8; 32]);
/* private bytes; strict JSON wire is exactly 64 lowercase hex */

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityContextEnvelopeV1 {
    pub schema_version: u32,
    pub purpose: SecurityContextPurposeV1,
    pub issuer_id: UuidV1,
    pub key_id: UuidV1,
    pub key_epoch: u64,
    pub mac_algorithm: SecurityContextMacAlgorithmV1,
    pub principal: PrincipalRefV1,
    pub legal_entity_id: UuidV1,
    pub policy_version: u64,
    pub authority_epoch: u64,
    pub generation_digest_sha256: Sha256Digest,
    pub issued_at_unix_ms: i64,
    pub expires_at_unix_ms: i64,
    pub nonce: UuidV1,
    pub signature: SecurityContextMacV1,
}

pub struct VerifiedSecurityContextEnvelopeV1(
    /* private to db-pg: exact verified envelope + binding digest + issuer row identity */
);
struct IssuedSecurityContextEnvelopeV1(SecurityContextEnvelopeV1);
/* private issuer product; no public/raw-envelope constructor */
struct SecurityContextIssuerV1 {
    /* private handle to the disjoint context-issuer pool */
}
struct SecurityContextBackendBindingV1 {
    database_oid: u32,
    backend_pid: i32,
    backend_start_unix_us: i64,
    transaction_id_xid8: u64,
    database_role_oid: u32,
}
struct SecurityContextBackendTicketV1([u8; 32]);
/* private, non-serialized, single-delivery ticket; raw bytes are never persisted */

pub enum SecurityContextVerificationErrorV1 {
    InvalidEnvelope,
    IssuanceFailed,
    IssuerRejected,
    SignatureRejected,
    Expired,
    NonceReplay,
    TicketRejected,
    BackendBindingMismatch,
    TransactionContextMismatch,
    ContextReadbackMismatch,
    PoolSanitizationFailed,
    DatabaseFailure,
}

pub struct AuthorizedPgTx<'a> { /* all fields and constructors private to db-pg */ }

impl SecurityContextIssuerV1 {
    async fn issue(
        &self,
        verified: &VerifiedSecurityContextV1,
    ) -> Result<IssuedSecurityContextEnvelopeV1, SecurityContextVerificationErrorV1>;
}

impl PgUnitOfWork {
    pub async fn begin_authorized(
        &self,
        verified: &VerifiedSecurityContextV1,
    ) -> Result<AuthorizedPgTx<'_>, SecurityContextVerificationErrorV1>;
}
```

G1-03 solely owns `VerifiedSecurityContextV1|SecurityContextPurposeV1|SecurityContextMacAlgorithmV1|SecurityContextMacV1|SecurityContextEnvelopeV1|SecurityContextVerificationErrorV1` at `crates/foundation/src/security/context.rs`, private `SecurityContextIssuerV1|IssuedSecurityContextEnvelopeV1|VerifiedSecurityContextEnvelopeV1|SecurityContextBackendBindingV1|SecurityContextBackendTicketV1` and the ticket/verifier at `crates/adapter/db-pg/src/context_ticket.rs`, and `AuthorizedPgTx` at `crates/adapter/db-pg/src/authorized_tx.rs`. `docs/schemas/f57-security-context-envelope.v1.schema.json` is the sole JSON-Schema owner for the complete security-context envelope wire and has exactly one import, `../evidence/f57-foundation.v1.schema.json`, for the G0-owned principal/UUID/digest definitions. The foundation schema imports nothing, so this edge cannot form a cycle. Exact media is `application/vnd.ep.f57-security-context-envelope-v1+json`; the internal transaction credential is not a signed business artifact, evidence artifact, offline-schema descriptor, or signer-registry row.

The strict envelope fixes `schema_version=1`, purpose `EP-F57-SECURITY-CONTEXT-V1`, algorithm `HMAC_SHA_256`, positive `key_epoch|policy_version|authority_epoch`, nonnil issuer/key/principal/legal-entity/nonce UUIDs, nonzero generation digest, and `issued_at_unix_ms < expires_at_unix_ms` with checked lifetime `1..=30000` ms. `SecurityContextMacV1` has no public raw-byte constructor and accepts/serializes exactly one 64-lowercase-hex 32-byte value. Issuer time, nonce, and every authorization/version field come only from private `SecurityContextIssuerV1::issue(&VerifiedSecurityContextV1)` plus the trusted context-issuer clock; no ingress/client payload or public caller can construct or submit an envelope, and no overload accepts raw fields, JSON, envelope, key ID, time, or nonce.

The MAC preimage is one library-independent fixed binary value, never JCS, JSON text, PostgreSQL `jsonb`, a field-name map, or a caller-supplied digest:

```text
ASCII("EP-F57-SECURITY-CONTEXT-MAC-V1\0")
|| u32be(schema_version)
|| purpose_u8
|| issuer_uuid16
|| key_uuid16
|| u64be(key_epoch)
|| algorithm_u8
|| principal_kind_u8
|| principal_uuid16
|| legal_entity_uuid16
|| u64be(policy_version)
|| u64be(authority_epoch)
|| generation_sha256_32
|| i64be(issued_at_unix_ms)
|| i64be(expires_at_unix_ms)
|| nonce_uuid16
```

UUID bytes are RFC-4122 network-order bytes and signed integers are two's-complement big-endian. `purpose_u8=0x01`, `algorithm_u8=0x01`, and principal-kind bytes are exactly `USER=01,GROUP=02,TEAM=03,PROJECT=04,DEPARTMENT=05,SERVICE=06,AI=07,PLUGIN=08,CUSTOMER=09,SUPPLIER=0a`. The signature is exactly `HMAC-SHA-256(the selected 32-byte issuer secret,preimage)` and is compared without variant-specific error leakage. The immutable binding digest is exactly `SHA256(ASCII("EP-F57-SECURITY-CONTEXT-BINDING-V1\0") || preimage || signature32)`. Rust uses fixed-width `to_be_bytes`/UUID bytes; PostgreSQL uses equivalent `int4send|int8send|uuid_send` bytea construction. Cross-language byte goldens must match before G1-03 passes.

The G1-03 migration owns the closed `platform_core.security_context_issuer_keys` registry keyed by `(issuer_id,key_id,key_epoch)`, exact states `ISSUE_ACTIVE|VERIFY_ONLY|REVOKED`, one 32-byte secret, `valid_from_unix_ms < issue_not_after_unix_ms <= verify_not_after_unix_ms`, and a partial uniqueness constraint permitting exactly one `ISSUE_ACTIVE` row per issuer. Only the NOLOGIN SECURITY-DEFINER owner can read key bytes; business, context-issuer, migration, operations, and recovery roles remain disjoint. Every function fixes `search_path=pg_catalog,platform_core`, uses no caller-resolved object or dynamic SQL, and returns no key material.

Private `SecurityContextIssuerV1::issue` accepts only `&VerifiedSecurityContextV1`, invokes the fixed issuer-pool SECURITY-DEFINER issuance function, revalidates current policy/authority epoch/generation, samples trusted issue time and a fresh CSPRNG nonce, selects the sole current `ISSUE_ACTIVE` row itself, and requires issue time in `[valid_from_unix_ms,issue_not_after_unix_ms)`. Only strict parsing plus exact field/preimage readback may wrap the result as private `IssuedSecurityContextEnvelopeV1`. Rotation holds the issuer row lock, requires `new_epoch=old_epoch+1`, installs a fresh key, and atomically changes the previous key to `VERIFY_ONLY` with `verify_not_after=min(old_verify_not_after,rotation_time+30000)`. Epoch/key reuse, rollback, a second active key, revival of `REVOKED`, or a mixed key ID/epoch/MAC is impossible. Verification selects the exact envelope tuple, accepts `ISSUE_ACTIVE|VERIFY_ONLY` only while trusted `now` lies in the checked intersection `[envelope.issued_at_unix_ms,min(envelope.expires_at_unix_ms,verify_not_after_unix_ms)]` and original issuance time remains inside that row's issuance interval, and rejects revoked, unknown, expired, ambient, or network keys.

The database adapter obtains an `AuthorizedPgTx` only in this order:

```text
private SecurityContextIssuerV1::issue(&VerifiedSecurityContextV1)
→ acquire one business connection
→ BEGIN
→ read SecurityContextBackendBindingV1 {database_oid,backend_pid,backend_start_unix_us,transaction_id_xid8,database_role_oid}
→ disjoint context-ticket pool commits the nonce claim and 256-bit one-use backend-bound ticket
→ business SECURITY DEFINER apply independently verifies MAC/ticket/nonce/expiry/current policy-authority-generation/backend tuple
→ insert the exact verified-transaction-context row and apply only transaction-local context
→ adapter compares the complete typed readback byte-for-byte
→ private AuthorizedPgTx construction
```

`PgUnitOfWork::begin_authorized(&VerifiedSecurityContextV1)` is the sole public constructor path; no public or private sibling accepts `SecurityContextEnvelopeV1`. `backend_start_unix_us` is checked signed microseconds since Unix epoch from the same connection's `pg_stat_activity.backend_start`; `transaction_id_xid8` is the positive full `pg_current_xact_id()`, never a wrapping 32-bit XID. In one separate committed control transaction, the ticket function independently reconstructs/verifies the MAC and current policy-authority-generation tuple, inserts the unique nonce claim `(issuer_id,key_id,nonce)`, and generates exactly 32 CSPRNG bytes. It stores only `SHA256(ticket32)`, the binding digest, all five backend fields, and trusted `issued_at_unix_ms < expires_at_unix_ms=min(issued_at+5000,envelope.expires_at)`. It durably marks single delivery before returning the raw ticket once. Nonce/ticket tombstones survive business rollback until the key can no longer verify any envelope.

Loss before/during issuance returns no issuer product and retry creates a fresh trusted time/nonce; a complete issue response is invocation-local and loss after that response cannot be adopted by another invocation; loss after a complete envelope but before ticket commit leaves no nonce/ticket authority; loss after ticket commit burns that nonce/ticket and cannot adopt or replay it. Every retry restarts at `issue`, never from a retained or caller envelope. The business apply function hashes the ticket, exact-matches the durable row/current five-field backend tuple, inserts exactly one `platform_core.verified_transaction_contexts` row keyed by `(database_oid,backend_pid,backend_start_unix_us,transaction_id_xid8)`, then applies only transaction-local `set_config(...,true)`. RLS authorizes only by exact-joining the current backend/xid8 tuple to that verified row and matching the binding digest; raw `SET`, `SET LOCAL`, copied GUC, forged row/function call, or a prior transaction's audit row is never authority. Only after exact readback may the adapter construct private `VerifiedSecurityContextEnvelopeV1` and `AuthorizedPgTx`; repositories accept only `&mut AuthorizedPgTx`.

Pool hygiene is mandatory. Checkout requires protocol `IDLE`, the expected session/current role, and null F57 context GUCs. Every commit, rollback, cancellation, timeout, panic, and constructor error runs out-of-transaction `DISCARD ALL`, then rechecks role, protocol state, and every context GUC before check-in. If rollback, discard, or readback is uncertain or fails, the socket is closed and never pooled.

The ingress envelope is protocol-facing; it contains no trusted authorization assertion:

```rust
// Sole nominal owner: ep-platform-authz::device_signature (G1-02).
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceCommandSignatureAlgorithmV1 { Ed25519 }

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceCommandSignatureV1 {
    pub algorithm: DeviceCommandSignatureAlgorithmV1,
    pub key_id: UuidV1,
    pub signature_b64url: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmployeeCommandEnvelopeV1 {
    pub request_id: RequestId,
    pub command_type: CapabilityIdV1,
    pub idempotency_key: UuidV1,
    pub expected_generation: GenerationNumberV1,
    pub expected_subject_version: Option<u64>,
    pub generation_report: ClientGenerationReportV1,
    pub client_version: String,
    pub device_signature: DeviceCommandSignatureV1,
    pub payload: serde_json::Value,
}
```

There is no `command_id`; internal command identity is the `request_id` byte-for-byte. `DeviceCommandSignatureV1.key_id` is the sole key locator and must be a nonnil lowercase-hyphenated `UuidV1`; there is no parallel string key ID. `algorithm` has only `ED25519`, and `signature_b64url` is canonical unpadded base64url decoding to exactly 64 bytes. The signed message is exactly `ASCII("EP-F57-EMPLOYEE-COMMAND-V1\0") || JCS(the complete EmployeeCommandEnvelopeV1 fields except device_signature)`. The authenticated Employee adapter loads that exact active device key, proves it is bound to the authenticated device/principal/legal entity, verifies Ed25519 before JSON payload projection, and rejects revoked, cross-device, unknown-algorithm, noncanonical encoding, extra-field and one-byte preimage mutations without error-detail leakage. Control and Portal have their own closed envelopes. Each authenticated ingress adapter projects its surface proof into authenticated facts, obtains `VerifiedSecurityContextV1` only through G1-03's checked private-constructor boundary, and separately derives `CapabilityCommandV1`; it never defines or constructs the security-context nominal itself.

```rust
#[derive(Clone, Debug)]
pub struct IngressCommandEnvelopeV1 {
    pub request_id: RequestId,
    pub command_type: CapabilityIdV1,
    pub idempotency_key: UuidV1,
    pub expected_generation: GenerationNumberV1,
    pub expected_subject_version: Option<u64>,
    pub payload: CapabilityCommandPayloadV1,
}

pub struct VerifiedIngressContextV1 {
    /* private constructor; binds authenticated surface proof and derived security context */
}

#[async_trait::async_trait]
pub trait AuthorityCommandGatewayV1: Send + Sync {
    async fn execute(
        &self,
        ingress: &VerifiedIngressContextV1,
        envelope: IngressCommandEnvelopeV1,
    ) -> Result<CommandReceiptV1, CommandErrorV1>;
}

#[async_trait::async_trait]
pub trait CommandPipeline: Send + Sync {
    async fn execute(
        &self,
        security: VerifiedSecurityContextV1,
        command: CapabilityCommandV1,
    ) -> Result<CommandReceiptV1, CommandErrorV1>;
}

// CommandCommitSetV1<R>, SubjectRefV1, FactDraftV1, AuditDraftV1 and
// OutboxDraftV1 are the exact master-owned G1-04 contracts imported here;
// this child plan does not define a second shape.
```

`AuthorityCommandGatewayV1` is a transport convergence adapter, not a second command bus. It verifies that the surface-specific proof is current, obtains the G1-03-constructed `VerifiedSecurityContextV1`, derives `CapabilityCommandV1`, and calls the one `CommandPipeline`. Only authenticated Control, Employee, and Portal adapters can construct `VerifiedIngressContextV1`; none can bypass or duplicate the G1-03 constructor.

The durable flow closed sets are exact:

```text
ObjectiveStateV1 =
OPEN | WAITING | RECONCILING | INCIDENT | CLOSURE_REVIEW | CLOSED | ABANDONED

EffectStateV1 =
PREPARED | DISPATCHED | UNKNOWN | CONFIRMED |
FAILED_NOT_EXECUTED | COMPENSATED | CONFLICTED

HumanEffectDecisionV1 =
CONFIRMED_SUCCEEDED | CONFIRMED_NOT_EXECUTED |
CONFIRMED_COMPENSATED | UNRESOLVED_CONTAINED
```

### Task G1-01: Establish deployment, HDD storage, trusted time, and secret recovery boundaries

**Files**

- Create: `crates/platform/runtime/src/storage/mod.rs`
- Create: `crates/platform/runtime/src/storage/manifest.rs`
- Create: `crates/platform/runtime/src/storage/manifest_rotation.rs`
- Create: `crates/platform/runtime/src/storage/policy.rs`
- Create: `crates/platform/runtime/src/storage/windows_volume.rs`
- Create: `crates/platform/runtime/tests/storage_manifest_wire.rs`
- Create: `crates/platform/runtime/tests/storage_manifest_rotation.rs`
- Create: `crates/platform/runtime/tests/fixtures/authority-storage-manifest-rotation-generation-v1.jcs.json`
- Create: `crates/platform/runtime/tests/fixtures/authority-storage-manifest-rotation-package-v1.jcs.json`
- Create: `crates/platform/runtime/tests/validated_data_root.rs`
- Create: `docs/schemas/f57-authority-storage-manifest.v1.schema.json`
- Create: `docs/schemas/f57-authority-storage-manifest-rotation.v1.schema.json`
- Read/import unchanged: `crates/platform/runtime/src/topology.rs`
- Read/import unchanged: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Read/import unchanged: `crates/platform/capability-graph/src/model.rs`
- Read/import unchanged: `crates/platform/release/src/generation.rs`
- Read/import unchanged: `crates/platform/release/src/generation_approval.rs`
- Create: `crates/platform/release/src/generation_creation_attempt.rs`
- Create: `crates/platform/release/tests/generation_creation_attempt.rs`
- Modify: `crates/platform/release/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Read/import unchanged: `docs/evidence/f57-generation.v1.schema.json`
- Read/import unchanged: `docs/schemas/f57-generation-approval-registry.v1.schema.json`
- Read/import unchanged: `crates/platform/runtime/src/evidence/object_store.rs`
- Read/import unchanged: `crates/platform/runtime/src/evidence/input_store.rs`
- Read/import unchanged: `crates/platform/gate-journal-contract/src/storage_root_binding.rs`
- Read/import unchanged: `crates/platform/gate-journal-contract/src/port.rs`
- Create: `crates/platform/runtime/src/secrets/mod.rs`
- Create: `crates/platform/runtime/src/secrets/broker.rs`
- Create: `crates/platform/runtime/src/secrets/envelope.rs`
- Create: `crates/platform/runtime/src/secrets/recovery.rs`
- Create: `crates/platform/runtime/src/secrets/generation_trust.rs`
- Create: `crates/platform/runtime/tests/generation_trust_provisioning.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Modify: `crates/platform/runtime/src/boot.rs`
- Modify: `crates/platform/runtime/src/config/secret.rs`
- Modify: `crates/platform/runtime/src/selfcheck/secrets.rs`
- Create: `crates/adapter/file/src/windows_volume.rs`
- Create: `crates/adapter/file/src/storage_manifest_rotation_store.rs`
- Create: `crates/adapter/file/src/generation_creation_store.rs`
- Create: `crates/adapter/file/tests/storage_manifest_rotation_store.rs`
- Create: `crates/adapter/file/tests/generation_creation_store.rs`
- Modify: `crates/adapter/file/src/evidence_object_store.rs`
- Modify: `crates/adapter/file/src/evidence_input_store.rs`
- Modify: `crates/adapter/file/src/gate_run_journal_store.rs`
- Modify: `crates/adapter/file/src/authority_storage_bootstrap_archive.rs`
- Create: `crates/adapter/file/tests/authority_evidence_object_store.rs`
- Create: `crates/adapter/file/tests/authority_evidence_input_store.rs`
- Create: `crates/adapter/file/tests/authority_gate_run_journal_store.rs`
- Modify: `crates/adapter/file/tests/authority_storage_bootstrap_archive.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Modify: `crates/adapter/file/Cargo.toml`
- Create: `crates/adapter/kms/src/windows_tpm.rs`
- Create: `crates/adapter/kms/src/piv_recovery.rs`
- Modify: `crates/adapter/kms/src/lib.rs`
- Modify: `crates/adapter/kms/Cargo.toml`
- Create: `crates/adapter/kms/tests/fixtures/adr0020-piv-shamir-v1.json`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/deployment_manifest_store.rs`
- Modify: `crates/adapter/db-pg/src/platform_core/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_core/secret_vault_store.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `apps/recovery-tool/Cargo.toml`
- Create: `apps/recovery-tool/src/main.rs`
- Create: `apps/recovery-tool/src/verify.rs`
- Create: `apps/recovery-tool/src/storage_manifest_rotation.rs`
- Create: `apps/recovery-tool/src/generation_trust.rs`
- Create: `apps/recovery-tool/tests/storage_manifest_rotation_composition.rs`
- Modify: `apps/core-server/src/config.rs`
- Modify: `apps/core-server/src/wiring/context.rs`
- Modify: `apps/core-server/src/wiring/kms.rs`
- Create: `apps/core-server/src/wiring/storage.rs`
- Create: `apps/core-server/src/wiring/generation.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `crates/platform/command/Cargo.toml`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/core-server/tests/evidence_object_store_composition.rs`
- Create: `apps/core-server/tests/evidence_input_store_composition.rs`
- Create: `apps/core-server/tests/gate_run_journal_store_composition.rs`
- Create: `apps/core-server/tests/authority_storage_bootstrap_archive_composition.rs`
- Create: `db/migrations/platform_ops/V20261025090000__platform_ops_create_deployment_manifests.sql`
- Create: `db/migrations/platform_core/V20261025090100__platform_core_create_customer_secret_vault.sql`
- Create: `testkit/tests/f57_g1_pre_db_boundary.rs`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

**Interfaces:**
- Consumes: G0 foundation signed-artifact/CMS contracts, the signed authority-storage manifest, product-pinned deployment trust, `ValidatedAbsolutePathV1`, DATA_HDD final-handle verification, trusted time and the provider-specific generation trust handles. It accepts no raw storage root, caller revision/time, latest-file selection, caller event or second manifest writer.
- Produces: the deployment-global `AuthorityStorageManifestRotationCoordinatorV1` port/private state/strict fifteen-field schema, its sole DATA_HDD file-store implementation and recovery-tool composition; a generation-domain typed change adapter with no independent lock/CAS/journal/projection; private `ValidatedDataRootV1`; four-role `GenerationTrustProviderV1`; the create-new signed generation/reverse-plan authority and generation/declaration bootstrap triple. Task 7 consumes the pre-reserved package-trust variant unchanged.

- [ ] **Step 1: Write the failing pre-database boundary tests**

```rust
#[tokio::test]
async fn invalid_manifest_or_ssd_data_root_opens_zero_database_connections() {
    for fault in [
        Fault::BadSignature,
        Fault::RolledBackRevision,
        Fault::OsVolumeDataRoot,
        Fault::ReparsePointEscape,
        Fault::UntrustedClock,
        Fault::GenerationApprovalRegistryDigestDrift,
        Fault::GenerationApprovalRegistryRollback,
        Fault::GenerationApprovalRegistrySelfRoot,
        Fault::GenerationTrustRoleMissing,
        Fault::GenerationTrustKeyExportable,
        Fault::GenerationTrustHalfInstalledPair,
    ] {
        let probe = boot_with(fault).await;
        assert_eq!(probe.database_connect_attempts(), 0);
        assert_eq!(probe.readiness(), ReadinessV1::NotReady);
    }
}

#[tokio::test]
async fn generation_then_declaration_is_the_only_post_storage_order() {
    let boot = boot_through_verified_data_root().await;
    let generation = boot.create_first_generation().await.unwrap();
    let declaration = boot.create_topology_declaration(&generation).await.unwrap();
    assert_eq!(declaration.generation_digest_sha256,
        generation.whole_envelope_digest_sha256());
    assert_eq!(generation.approval_registry_ref(),
        boot.storage_manifest_pinned_generation_approval_registry_ref());
    assert!(generation.every_item_has_exact_verified_reverse_plan());
    assert_code(boot.create_declaration_without_verified_generation().await,
        "F57_VERIFIED_GENERATION_REQUIRED");
    assert_code(boot.create_second_first_generation().await,
        "F57_GENERATION_CREATE_NEW_CONFLICT");
}

#[tokio::test]
async fn generation_creation_reuses_one_frozen_attempt_across_every_crash_cut() {
    for cut in GenerationCreationCrashCutV1::all() {
        let recovered = create_generation_with_crash_then_resume(cut).await.unwrap();
        assert_eq!(recovered.reverse_plan_refs(), frozen_expected_plan_refs());
        assert_eq!(recovered.manifest_ref(), frozen_expected_manifest_ref());
        assert_eq!(recovered.declaration_ref(), frozen_expected_declaration_ref());
        assert_eq!(recovered.signature_count_per_output(), 1);
    }
}

#[tokio::test]
async fn every_trust_change_uses_the_one_global_storage_manifest_coordinator() {
    let generation = staged_generation_trust_change().await;
    let first = global_rotation_coordinator().submit(generation).await.unwrap();
    assert!(first.committed_storage_manifest_revision() > first.base_revision());
    assert!(recover_every_rotation_crash_cut(first.rotation_id()).await.is_ok());
    assert_code(try_independent_generation_manifest_projection().await,
        "AUTHORITY_STORAGE_MANIFEST_WRITER_FORBIDDEN");
    assert_code(submit_stale_or_cross_domain_base().await,
        "AUTHORITY_STORAGE_MANIFEST_ROTATION_BASE_CONFLICT");
}
```

- [ ] **Step 2: Run the focused test and confirm RED**

Run: `cargo test -p ep-testkit --test f57_g1_pre_db_boundary -- --nocapture`

Expected: FAIL because the storage manifest, secret broker, deployment migrations, and post-storage signed-generation-then-topology-declaration production path do not exist.

- [ ] **Step 3: Implement the minimum trusted bootstrap**

The immutable locator is on a non-OS HDD at `\EnterprisePlatform\packages\authority-storage-manifest.v1.json`. Resolve the opened handle to stable volume/device identity; never trust a drive letter, junction, mount point, symlink, or path string alone.

`crates/platform/runtime/src/storage/manifest.rs` implements, without renaming or extending, the master's current exact `F57AuthorityStorageManifestPayloadV1` and `SignedF57AuthorityStorageManifestV1`; the historical 2026-08-23 shape is not an implementation authority. It solely generates `docs/schemas/f57-authority-storage-manifest.v1.schema.json`, exact media `application/vnd.ep.f57-authority-storage-manifest-v1+json`, and byte/unknown-field fixtures. Purpose, revision/time/deployment, two role/volume/root triples, complete canonical `customer_authority_data_roots`, backup/hardware/epoch/API/policy fields and six trust/anti-rollback fields are all mandatory and use the master's exact names/order/validation. The signed manifest routes PostgreSQL data/WAL/temp, attachments, quarantine, customer logs, imports/exports, spool, search, packages, vault, generations, both evidence roots, backup staging, and any dump/pagefile containing customer bytes to HDD or marks the feature disabled. Boot order is product-pinned deployment bootstrap → Stage-14 deployment manifest/CMS/trust bundle → storage manifest signer membership/chain/revocation/checkpoint → WDAC/trusted boot/BitLocker → final-handle volume/root policy and anti-rollback → trusted-time decision → secret broker → first DB connect → database mirror comparison. A self-authorizing root, ambient Windows root, network chain completion, wrong deployment manifest, root/checkpoint rollback, missing authority root class or storage-schema drift produces zero database connections. `deployment_manifest_store` and `secret_vault_store` are the only SQL adapters for the two tables; the runtime and KMS crates depend only on their ports. Their parent `lib.rs`/`mod.rs` registrations and the core-server composition are completed in this same task, and the all-target test must prove the concrete stores are reachable—file existence is not evidence.

The storage manifest's canonical `policy_ids` must contain exactly one member matching `generation-approval-registry-sha256:<64-lowerhex>` and exactly one matching `runtime-ssd-exception-registry-sha256:<64-lowerhex>`; other separately registered policy families may coexist but duplicates/unknown F57 prefixes fail. The latter pins exact static bytes of the master-defined four-row `WINDOWS_SERVER_2022_P340_SINGLE_DISK_V1` policy and has no manifest back-reference. After DATA_HDD final-handle verification, the runtime resolves only `<data_root>\generations\trust\generation-approval-registry.v1.json`, requires its exact envelope digest to equal the generation pin, and bootstrap-verifies its signed seven-field payload/three exact rows using the product-pinned deployment trust bundle plus exact registry-authority SPKI/DN/current revocation. Only then may `generation_approval.rs` construct private `VerifiedGenerationApprovalRegistryV1`. Missing/duplicate/malformed pin, another path, valid-old registry, deployment/revision/time/row/media/DN/SPKI drift, wildcard, self-root, ambient Windows trust or copied 89-row evidence registry fails before generation signing.

This task creates the one deployment-global `AuthorityStorageManifestRotationCoordinatorV1`; no later trust domain may create another storage-manifest writer, lock, CAS or journal. `crates/platform/runtime/src/storage/manifest_rotation.rs` is the sole owner of its port, private verified state, transition codec and exact fifteen-field record `{schema_version,purpose,rotation_id,trust_domain,sequence,previous_record_sha256,base_storage_manifest_ref,base_storage_manifest_revision,base_storage_manifest_sha256,domain_change,staged_fixed_path_refs,resulting_storage_manifest_ref,resulting_storage_manifest_revision,resulting_storage_manifest_sha256,event}`. Purpose/media are exactly `EP-F57-AUTHORITY-STORAGE-MANIFEST-ROTATION-V1` / `application/vnd.ep.f57-authority-storage-manifest-rotation-v1+json`. The strict schema imports only the foundation `ArtifactRefV1` owner and copies no generation/package root. `trust_domain` is the closed `GENERATION_TRUST|CAPABILITY_PACKAGE_TRUST` set, and `domain_change` is the corresponding tagged union: each variant carries only that domain's strict old/new registry refs, revisions, whole-envelope digests, signer pins and portable registry ref. G1 exercises `GENERATION_TRUST`; the future package adapter consumes the already frozen second variant without changing this schema or owning a sibling coordinator. The generation event chain is exactly `GENERATION_TRUST_ROTATION_STARTED -> STORAGE_MANIFEST_BOUND -> REGISTRY_PROJECTED -> ROTATION_COMMITTED`; the reserved package chain is exactly `PACKAGE_TRUST_ROTATION_STARTED -> STORAGE_MANIFEST_BOUND -> REGISTRY_PROJECTED -> ROTATION_COMMITTED`.

The implementation copies the following master-owned shape exactly; this is the concrete Task G1-01 codec, not illustrative pseudocode:

```rust
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorityStorageManifestRotationTrustDomainV1 {
    GenerationTrust,
    CapabilityPackageTrust,
}
#[serde(tag = "domain_change_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum AuthorityStorageManifestRotationDomainChangeV1 {
    GenerationTrust {
        old_registry_ref: Option<ArtifactRefV1>, old_registry_revision: Option<u64>,
        old_registry_envelope_sha256: Option<Sha256Digest>, old_signer_pins: Vec<SignerSpkiTokenV1>,
        old_signer_pin_set_sha256: Sha256Digest, new_registry_ref: ArtifactRefV1,
        new_registry_revision: u64, new_registry_envelope_sha256: Sha256Digest,
        new_signer_pins: Vec<SignerSpkiTokenV1>, new_signer_pin_set_sha256: Sha256Digest,
        portable_registry_ref: ArtifactRefV1,
    },
    CapabilityPackageTrust {
        old_registry_ref: Option<ArtifactRefV1>, old_registry_revision: Option<u64>,
        old_registry_envelope_sha256: Option<Sha256Digest>, old_signer_pins: Vec<SignerSpkiTokenV1>,
        old_signer_pin_set_sha256: Sha256Digest, new_registry_ref: ArtifactRefV1,
        new_registry_revision: u64, new_registry_envelope_sha256: Sha256Digest,
        new_signer_pins: Vec<SignerSpkiTokenV1>, new_signer_pin_set_sha256: Sha256Digest,
        portable_registry_ref: ArtifactRefV1,
    },
}
#[serde(tag = "event_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum AuthorityStorageManifestRotationEventV1 {
    GenerationTrustRotationStarted { request_sha256: Sha256Digest },
    PackageTrustRotationStarted { request_sha256: Sha256Digest },
    StorageManifestBound { staged_storage_manifest_ref: ArtifactRefV1 },
    RegistryProjected { fixed_path_id: String, projected_registry_ref: ArtifactRefV1 },
    RotationCommitted { committed_at_unix_ms: i64 },
}
#[serde(deny_unknown_fields)]
pub struct AuthorityStorageManifestRotationRecordV1 {
    pub schema_version: u32, pub purpose: AuthorityStorageManifestRotationPurposeV1,
    pub rotation_id: UuidV1, pub trust_domain: AuthorityStorageManifestRotationTrustDomainV1,
    pub sequence: u64, pub previous_record_sha256: Option<Sha256Digest>,
    pub base_storage_manifest_ref: ArtifactRefV1, pub base_storage_manifest_revision: u64,
    pub base_storage_manifest_sha256: Sha256Digest,
    pub domain_change: AuthorityStorageManifestRotationDomainChangeV1,
    pub staged_fixed_path_refs: Vec<ArtifactRefV1>,
    pub resulting_storage_manifest_ref: ArtifactRefV1,
    pub resulting_storage_manifest_revision: u64,
    pub resulting_storage_manifest_sha256: Sha256Digest,
    pub event: AuthorityStorageManifestRotationEventV1,
}
```

On initial install the three old-registry option fields are all null, old pins are `[]`, and their digest is `SHA256(JCS([]))`; otherwise all three are nonnull, old pins are nonempty and byte-match current state. New pins are always canonical nonempty unique and both set digests recompute over JCS. Sequence is exactly `0..3`; only sequence zero has null predecessor, and each later predecessor equals `SHA256(JCS(the complete prior record))`. All records repeat one immutable tuple; `staged_fixed_path_refs` is exactly the immutable new registry revision plus resulting manifest revision, and resulting revision is checked `base+1`. The event tag must match the record/domain and its refs must belong to that staged set. There is no alternate partial hash projection and no failure event that rewrites a valid prefix.

`crates/adapter/file/src/storage_manifest_rotation_store.rs` is the only implementation of the coordinator store. It derives its fixed DATA_HDD location only from `ValidatedDataRootV1`, takes the one global maintenance lock, checks a monotonic base-manifest CAS, writes the common `len8<TAB>JCS<LF>` fsynced hash-chain journal, and projects only the exact immutable staged bytes named by the verified record. `apps/recovery-tool/src/storage_manifest_rotation.rs` is the only composition root and recovery driver. It resumes the same `rotation_id`, base, staged refs and resulting manifest after every crash cut; it never scans for a latest file, rebases implicitly or fabricates another domain change. A stale base, cross-domain concurrent winner, alternate fixed path, hash-chain fork, partial/conflicting pair or any writer outside this port fails closed before database boot. Cross-domain goldens submit generation plus the reserved package-domain fixture before and after every frame and prove one winner followed only by an explicit clean rebase, with no forked storage-manifest revision.

This task also owns the generation trust-material closure rather than assuming it exists. `crates/platform/runtime/src/secrets/generation_trust.rs` owns only the private `GenerationTrustProviderV1` port. It exposes exactly four separated roles—approval registry, generation manifest, generation reverse plan and migration plan—and only two adapters: self-hosted Windows CNG/PIV or an approved existing enterprise signer. Both expose identical typed contract and validation/error semantics; byte-identical CMS is required only when querying/adopting the same frozen `(signing_operation_id,authorization_sha256)`, including any RFC-3161 step, and is never asserted across different provider modes or keys; provisioning rejects a provider that can only make a fresh signature after response loss. Neither may generate an unpinned root, export a key or accept caller-selected role metadata. Local key-container identities are exactly `EP-F57-GENERATION-APPROVAL-REGISTRY-V1`, `EP-F57-GENERATION-MANIFEST-V1`, `EP-F57-GENERATION-REVERSE-PLAN-V1`, and `EP-F57-MIGRATION-PLAN-V1`; provisioning/import verifies nonexportability, signing usage, ACL, certificate/SPKI/DN, current chain and operation query/adopt behavior. Core-server can obtain only manifest/reverse-plan handles; registry signing is a maintenance ceremony and migration signing remains behind the approved migration workflow.

Here “Windows CNG” is the API boundary, not permission to leave an unrecoverable software key or claim public-key reconstruction. Production self-hosted mode accepts a PIV/PKCS#11 hardware slot or a Microsoft Platform Crypto Provider TPM-backed nonexportable CNG machine key. PIV private bytes stay on the token. For the TPM branch, public area/role/certificate/attestation plus bounded CNG provider and certificate-binding metadata are classified as TPM_BOUND_REENROLLABLE_MACHINE_KEY_METADATA in signed reproducible Set A and mirrored to DATA_HDD/off-host custody; the separate TPM NV record is anti-rollback state only. The design does not assert that a TPM handle, DER/SPKI or public metadata can recreate or rebind the lost CNG private-key object after a clean SSD. A pure software-CNG key whose only container blob resides on RUNTIME_SSD remains UNSUPPORTED_PRODUCTION.

The SSD-loss rehearsal first unlocks DATA_HDD through its independent two-person recovery-password contract. An unchanged PIV slot may be rebound only after exact SPKI/role/ACL verification. A lost Windows TPM/CNG binding instead requires dual-control generation of a new TPM-backed key/certificate, a strictly newer signed signer-registry/session revision and a new candidate/run; it can never finish the old signature under a new SPKI. An interrupted operation may query/adopt only an already committed byte-identical CMS result from the HDD/off-host journal. If no committed result exists, the old attempt fails closed and cannot be silently re-signed. Missing hardware, key/SPKI drift, orphan provider blob, SSD-only mutable journal, or mutable SSD fallback keeps boot/admission closed; public certificate/policy/bootstrap caches qualify only as signed bounded Set-A inventory and never a fifth mutable SSD exception.

`apps/recovery-tool/src/generation_trust.rs` is only a generation-domain typed adapter: it creates or imports the four role credentials, uses `GenerationApprovalRegistryBootstrapAuthorizerV1 -> prepare_cms_signing_request_v1 -> sign_authorized` for the registry, emits the whole-envelope digest for independent storage-manifest signing, verifies the mutually bound signed registry/storage-manifest pair, creates immutable revision bytes below `<data_root>\generations\trust\revisions\`, and submits exactly one immutable `GENERATION_TRUST` domain change to `AuthorityStorageManifestRotationCoordinatorV1`. All serialization, monotonic-CAS, active-path projection and crash recovery are invoked through the global coordinator. Only that coordinator may project the selected bytes to `generations\trust\generation-approval-registry.v1.json` and the canonical storage-manifest location. Initial install and rotation query/adopt the same registry signing operation after response loss; a newer signed storage manifest must pin the new registry digest before projection. The active path is atomically replaced only by the global journal, never byte-edited or selected through directory/latest-file discovery; old revisions remain immutable. Missing role, non-idempotent provider, exportable/aliased key, half-installed pair, manifest rollback or a second writer leaves boot closed.

The same file is the unique Rust owner of the non-serialized `AuthorityStorageTrustBootstrapInputPathsV1`, private `ValidatedDataRootV1`, closed `AuthorityStorageRootVerificationErrorV1`, and `verify_authority_data_root_v1(storage_manifest_path,trust_input_paths)`. The six trust fields accept only `ValidatedAbsolutePathV1`; together with the separate signed-manifest path they map one-to-one to CLI flags `--deployment-manifest|--deployment-manifest-signature|--deployment-trust-bundle|--storage-trust-root|--storage-revocation|--storage-checkpoint|--storage-manifest`. The constructor alone performs complete product-pinned deployment verification, manifest signer roster/ACTIVE-chain/revocation/checkpoint checks, storage anti-rollback, DATA_HDD role, live final-handle volume identity and non-reparse descent, then returns a value carrying that verified seven-file identity tuple. Its fields and constructor are not public; G5 architecture and the first G6 run creator accept `&ValidatedDataRootV1` only, never a path/string/manifest shortcut. Tests prove each wrong-file permutation, missing/duplicate semantic role, relative/alias/reparse path, SSD role, live-volume substitution, root/checkpoint rollback and attempted struct fabrication fail before any authority store or database write.

Only after this task defines `ValidatedDataRootV1` does it extend all four relevant G0 file adapters with private authority constructors. The generation lane's ObjectStore/InputStore derive only `<verified-data-root>\generations\authority-evidence\`. A private `AuthorityReleaseEvidenceRootFactoryV1`, constructible only from `&ValidatedDataRootV1`, derives and final-handle verifies exact `<verified-data-root>\evidence\release-candidate` and returns one lane-tagged aggregate containing `FileEvidenceObjectStoreV1`, `FileEvidenceInputStoreV1`, `FileGateRunJournalStoreV1` and `AuthorityStorageBootstrapArchiveStoreV1` for that same release root. Generation and release lane tokens are distinct private types and cannot be interchanged; both use the one shared byte engine and identical content-addressing grammar. G1 lands and tests the factory but has zero production run/header caller; the first release-lane call remains the G6 run creator. G0's `ValidatedBundleRootV1` tooling constructors remain unchanged. Core-server composition injects the four production-linkable ports through explicit `ep-adapter-file`; it never depends on or path-includes `xtask`. Input materialization accepts only foundation `VerifiedSignedEnvelopeBytesV1` and a placement-class contract. Cold reload first calls `resolve_class_location|resolve_location`, then `load_exact_unverified`; the returned `LoadedEvidenceInputV1` can feed only the owning ArtifactVerifier. Only `verify_trust` plus `finalize_loaded(loaded,contract,verified)` may expose `VerifiedLoadedEvidenceInputV1` for semantic use, exact-binding bytes/ref/class/media to the verifier-derived deployment-specific nine-field policy. This closes the proof-before-read cycle and rejects policy reuse from another envelope without a runtime→release dependency. The seven-row bootstrap archive accepts only the already verified authority-storage envelope and six verified bootstrap bytes at its fixed G6 paths; it is not used for the authority-generation bundle. Goldens prove generation/release create-new/adopt byte parity, cross-lane token noninterchangeability, trust rotation, cross-deployment/registry rejection, fixed-row/media enforcement, release object/input/journal/archive shared-root equality, generation-root separation and `NotFound` versus I/O/partial/conflict. Authority tests reject a tooling token, raw path, SSD root, wrong deployment/volume, reparse escape, alternate media/filename/trust policy or any write before DATA_HDD verification. `generation_creation_store.rs` consumes G0 generation nominals through the explicit `ep-platform-release` dependency declared by `crates/adapter/file/Cargo.toml`; no copied wire or reverse dependency is allowed.

G0 owns the generation/topology wire schemas, strict validators, whole-envelope generation-digest helper, domain-specific approval-verifier boundary and pure topology builder/verifier API; it has no production generation, ACK, declaration or certification call site. After—and only after—`verify_authority_data_root_v1` returns `ValidatedDataRootV1`, G1-01's private server-only `GenerationManifestAuthorityV1` and `GenerationReversePlanAuthorityV1` in `apps/core-server/src/wiring/generation.rs` become the sole first production generation producers. They consume the complete compiled CapabilityGraph, exact projection manifest, verified storage manifest, frozen non-expiring P340 policy definition, exact prior durable OBSERVED item set, compiled rollback policy, and the deployment-pinned generation/migration approval registry. The registry and signed storage-manifest source bytes first pass their owning verifier, expose foundation `VerifiedSignedEnvelopeBytesV1`, and are create-new materialized/fsynced/reloaded through the authority `EvidenceInputStoreV1` at `inputs/<whole-envelope-sha256>.json`; graph/projection/policy and every other manifest dependency are materialized through their closed object contracts. The reverse-plan authority first derives one and only one action: existing prior item → `RESTORE_ARTIFACT` to that distinct prior ref; new safely deactivatable item → `DEACTIVATE_RETAIN_DATA`; compiled-explicit safe no-op → `NO_OP`; otherwise fail. No caller supplies a plan, action, target, ID, time or ref. The manifest authority derives all thirteen `GenerationManifestV1` fields, including complete canonical required-participant/item subsets, and accepts no caller field, raw digest, unsigned plan, participant/client payload, ambient file discovery or F57 evidence signer.

Both payload types use their generated stable `CmsArtifactTypeIdV1`, the exact child row of `GenerationApprovalSigningAuthorizerV1`, foundation `prepare_cms_signing_request_v1`, and only `DetachedCmsSignerV1::sign_authorized`; the issuance union is the checked `[issued_at_unix_ms,issued_at_unix_ms+300000]` window under current verification. Before each provider request the attempt journal durably binds the operation ID, descriptor/payload/window/context/handle digests, signer role and exact nine-field authorization digest. A response loss queries/adopts the same provider operation and byte-identical CMS. Only after the resulting foundation envelope is create-new spooled, fsynced and typed-reloaded does G1 compute its whole-envelope digest/ref, append OBJECT_BOUND, ingest/reload through the authority object store and allow that ref into another artifact. The manifest path invokes `GenerationApprovalVerifierV1` to bind the relocatable approval-registry input ref and obtain private `VerifiedGenerationManifestV1`; the generic `VerifiedBusinessArtifactV1<GenerationManifestV1>` cannot leave that boundary or authorize activation. Generation 1 is positive with previous OBSERVED digest null; later creation is serialized against the exact current durable OBSERVED signed-envelope digest and cannot skip, fork or use a desired-only predecessor. `generation_digest_sha256` is the SHA-256 of the complete exact signed-envelope bytes and equals stored `ArtifactRefV1.sha256`, never `payload_sha256` or a caller digest. The generation trust domain adds no row to the 89-row F57 evidence registry.

`crates/platform/release/src/generation_creation_attempt.rs` is the sole production-linkable owner of `GenerationCreationAttemptStoreV1`, its immutable header/record/event types, strict transition validator, codec and port; `crates/adapter/file/src/generation_creation_store.rs` only implements that port. The store writes only below `<verified-data-root>\generations\creation-attempts\<deployment-id>\<20-digit-authority-epoch>\<20-digit-generation-number>\`. The exact journal is `creation-attempt.v1.jcs.jsonl`; fixed spools are `spool\reverse-plans\<GenerationItemIdV1>.signed.v1.json` and `spool\generation-manifest.signed.v1.json`. Frames are `<8 lowercase hex byte length>\t<JCS record bytes>\n`, contiguous from sequence 0 with exact previous-frame SHA-256, OS-exclusive lease, fsync and torn-tail quarantine; no alternate filename/root, scan or overwrite is legal. The closed event set is `ATTEMPT_FROZEN|SIGNING_AUTHORIZATION_COMMITTED|SIGNED_ENVELOPE_SPOOLED|OBJECT_BOUND|OBJECT_INGESTED|DECLARATION_BOUND|ATTEMPT_COMPLETED|ATTEMPT_FAILED`. `ATTEMPT_FROZEN` fixes complete re-derivable verified inputs, one unpredictable creation-attempt ID, one plan ID per canonical item, all plan/manifest issue times, registry/dependency refs, exact spools, stable artifact type IDs and one unpredictable signing-operation ID per output. It deliberately contains no expected object digest/ref/destination because CMS bytes do not exist yet. Every reload re-derives graph/prior-OBSERVED/policy/actions and exact-matches the frozen header; port errors are the closed set `NotFound|ExistingConflict|PartialWrite|SequenceConflict|HashChainInvalid|TransitionInvalid|PathEscape|ReparsePoint|FsyncFailed|Io`.

For each output, the authority derives the canonical payload, calls the generated authorizer and `prepare_cms_signing_request_v1`, then durably appends `SIGNING_AUTHORIZATION_COMMITTED` with the output's operation ID plus descriptor/payload/window/context/handle and exact nine-field authorization digests before provider access. Response loss queries/adopts only that same provider/broker operation; a provider unable to return byte-identical CMS is ineligible. The complete envelope is create-new spooled, file and directory fsynced, and structurally strict-reloaded as exact JCS/CMS bytes; `SIGNED_ENVELOPE_SPOOLED` binds only that fixed spool identity. The authority then computes the whole-envelope digest/ref and durably appends `OBJECT_BOUND`, ingests/fsyncs/reloads only that bound ref through the authority ObjectStore, appends `OBJECT_INGESTED`, and only then invokes `ArtifactVerifierV1` plus `GenerationApprovalVerifierV1` to create the domain proof. Domain verification cannot precede `OBJECT_BOUND` because its expected content-addressed ref does not yet exist. All verified reverse-plan objects complete before manifest authorization; the verified manifest completes before declaration creation/binding. Recovery follows only journal and fixed spools, adopting the exact provider result, spool, bound ref or object without regenerating ID/time/action, re-signing, overwrite, scan, registry change or predicted digest. Crash goldens cut before/after every event, provider request/response/query-adopt, spool create/fsync/reload, object ingest/reload, domain verification, manifest and declaration; every cut converges to byte-identical refs and one provider operation per output. The initial generation may use `NO_OP` only where compiled rollback policy explicitly permits it; an impossible reverse path aborts before manifest authorization and leaves prior OBSERVED unchanged.

Only after that verified signed generation exists is G1-01's private `RuntimeTopologyDeclarationAuthorityV1` the sole first-stage production caller of `TopologyVerifier::build_declaration`. It accepts `VerifiedGenerationManifestV1`, not a raw digest or the generic signed-artifact proof, and passes the complete compiled CapabilityGraph plus build facts derived from that wrapper, the same verified storage manifest and the same policy ref. The lower builder is pure: it derives the canonical participant/database-consumer sets and profile IDs and emits strict canonical `RuntimeTopologyDeclarationV1` bytes without consulting ambient live state. G1-01 create-new stores the bytes through the same authority-lane `EvidenceObjectStoreV1`, typed-reloads the exact media/size/digest and relocatable `evidence-relative://bundle/objects/sha256/<digest>` URI, and durably records the returned declaration ref beside the generation and approval-registry refs as the only triple allowed at G1-05. The mandatory order is `verified DATA_HDD -> signed generation object -> declaration object`; one transactionally durable bootstrap record binds that triple, so recovery adopts exact existing bytes or fails on conflict and never signs or writes a second artifact pair or registry binding. G1-05—not this task—performs complete live-readback equality through `VerifiedRuntimeTopologyDeclarationV1` immediately before activation and constructs participant ACKs. A generation/declaration before storage verification, declaration before generation, a raw/generic generation proof, a second production builder or signer caller, different graph/generation/storage/policy/approval-registry ref, ambient participant discovery, double/signed topology wrapper, certification field, overwrite or directory scan fails; neither artifact adds a TestID.

Daily operational unwrap uses the TPM recipient. Offline recovery uses only `PIV_SHAMIR_2_OF_3_V1`; exactly three distinct custodians hold shares and any two reconstruct inside zeroizing memory. This task supplies the portable contract, known-answer tests, and Windows evidence-runner tests—not a P340 hardware certification.

- [ ] **Step 4: Verify migrations from a fresh PostgreSQL 16 instance**

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090100`

Expected: PASS; the two registry rows are `CREATED`, both schemas remain isolated, and the server refuses to open the first business connection before the pre-DB checks pass.

- [ ] **Step 5: Verify GREEN and commit only this task**

Run: `cargo test -p ep-platform-runtime -p ep-platform-release -p ep-adapter-file -p ep-adapter-kms -p recovery-tool -p core-server --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g1_pre_db_boundary -- --nocapture`

Expected: PASS on the declared G1 evidence lane, including four-role self-hosted/existing-signer provisioning equivalence, the one global storage-manifest rotation port/store/schema/recovery-tool composition, immutable registry/storage-manifest pair install/rotation recovery, generation-trust as a typed request adapter and the global coordinator as the only serialization/projection/recovery owner, fixed sources copied to relocatable input refs, storage verification → separately signed reverse-plan spool/object/reload → independently trusted signed generation spool/object/reload → declaration object order, exact 13/9-field manifest/reverse-plan wires and whole-envelope digest, exact declaration ref/media/digest/JCS, the shared object-store authority constructor and cross-lane byte parity, locked Cargo edges with no production dependency on `xtask`, exhaustive issuance descriptors, sole production signer/builder callsites, every create-new crash-cut adoption, and all missing-role/exportable-key/half-install/early/alternate/double-wrap/overwrite negatives; participant ACK, hardware-backed evidence and runtime-topology certification remain `NOT_DUE`.

Commit: `feat(runtime): establish authority storage and secret boundary`

### Task G1-02: Replace fixed roles with dynamic principal capability authorization

**Files**

- Read/import unchanged: `crates/foundation/src/principal.rs`
- Read/import unchanged: `crates/foundation/src/identifier.rs`
- Create: `crates/platform/authz/src/grant.rs`
- Create: `crates/platform/authz/src/delegation.rs`
- Create: `crates/platform/authz/src/decision.rs`
- Create: `crates/platform/authz/src/live_authority.rs`
- Create: `crates/platform/authz/src/device_signature.rs`
- Create: `crates/platform/authz/tests/device_signature.rs`
- Create: `docs/schemas/f57-device-command-signature.v1.schema.json`
- Create: `crates/platform/tenancy/src/snapshot.rs`
- Modify: `crates/platform/tenancy/src/lib.rs`
- Modify: `crates/platform/tenancy/Cargo.toml`
- Create: `crates/platform/tenancy/tests/authority_snapshot.rs`
- Create: `crates/platform/tenancy/tests/fixtures/f57-tenancy-authority-snapshot-v1-golden.json`
- Create: `docs/schemas/f57-tenancy-authority-snapshot.v1.schema.json`
- Modify: `crates/platform/authz/src/lib.rs`
- Modify: `crates/platform/authz/src/types.rs`
- Modify: `crates/platform/authz/src/decider.rs`
- Modify: `crates/platform/authz/src/snapshot.rs`
- Create: `crates/adapter/db-pg/src/platform_authz/grants.rs`
- Create: `crates/adapter/db-pg/src/platform_authz/delegations.rs`
- Create: `crates/adapter/db-pg/src/platform_authz/authority_epoch.rs`
- Create: `crates/adapter/db-pg/src/platform_core/tenancy_authority_snapshot.rs`
- Modify: `crates/adapter/db-pg/src/platform_authz/mod.rs`
- Modify: `apps/core-server/src/wiring/authz.rs`
- Modify: `crates/platform/authz/Cargo.toml`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `apps/core-server/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.lock`
- Create: `db/migrations/platform_authz/V20261025090200__platform_authz_create_principal_grants.sql`
- Create: `testkit/tests/f57_g1_dynamic_authz.rs`
- Create: `testkit/src/f57_cases/g1/authz_matrix.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `docs/generated/f57/test-manifest.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

- [ ] **Step 1: Write failing identity, query-use, delegation, and revocation tests**

```rust
#[tokio::test]
async fn same_uuid_with_different_principal_kind_never_shares_a_grant() {
    let id = UuidV1::new_v4();
    grant(PrincipalRefV1 { kind: PrincipalKindV1::User, id }).await;
    assert_denied(PrincipalRefV1 { kind: PrincipalKindV1::Service, id }).await;
}

#[tokio::test]
async fn filter_permission_does_not_imply_export_permission() {
    grant_query_use(QueryUseV1::Filter).await;
    assert_denied_query_use(QueryUseV1::Export).await;
}
```

- [ ] **Step 2: Run the focused test and confirm RED**

Run: `cargo test -p ep-testkit --test f57_g1_dynamic_authz -- --nocapture`

Expected: FAIL because the dynamic principal and query-use model is incomplete.

- [ ] **Step 3: Implement the current-grant decision path**

Create append-only grant revisions/revocations, bounded delegations, authority epochs, capability/scope/field/query-use constraints, validity windows, device/risk conditions, SoD, reauthentication, and approval requirements. A child delegation may only narrow its parent. This task reads/imports the exact G0 `PrincipalKindV1|PrincipalRefV1` nominals; it owns authorization semantics but cannot edit their Rust or foundation-schema definitions. `device_signature.rs` is the sole Rust owner of `DeviceCommandSignatureAlgorithmV1|DeviceCommandSignatureV1`, its exact Ed25519 preimage helper and verifier boundary; the same task owns the strict helper schema. Employee OpenAPI/TypeScript generated later imports this owner and cannot copy or widen it.

`platform.tenancy` is the sole owner of the strict plain `TenancyAuthoritySnapshotV1`, media `application/vnd.ep.f57-tenancy-authority-snapshot-v1+json`, its seven-field root and three-field legal-entity rows. The adapter builds it only from the same serialized current deployment/tenancy/key-domain authority read, canonical-sorts active/suspended/retired rows by legal-entity UUID, computes `legal_entity_set_sha256=SHA256(JCS(legal_entities))`, create-new stores and typed-reloads it, and exposes only a private verified snapshot. Package code stores only its `ArtifactRefV1`; the later upper generation authority has the explicit one-way dependency on tenancy and is the sole component that may turn a graph `scope_mode` into concrete `CapabilityPackageScopeV1`. Caller entity sets, latest scans, mixed tenancy/key generations and a scope not equal to the snapshot fail.

Decision order is fixed: constitutional deny → legal entity/classification/SoD deny → revocation/validity → capability/scope/field/query-use/device/risk → reauthentication/approval → allow. Each decision carries policy, grant, authority-epoch, and generation digests. A static role snapshot can optimize a read but is never authority. `authz_matrix.rs` implements exactly `AUTH-001..007` and no other RequirementID.

- [ ] **Step 4: Verify the migration and GREEN**

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090200`

Run: `cargo test -p ep-platform-authz -p ep-platform-tenancy -p ep-adapter-db-pg -p core-server -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g1_dynamic_authz -- --nocapture`

Expected: PASS, including revoke-during-command ordering, expired delegation, SoD, field masking, independent query-use, cross-kind UUID negatives, exact Ed25519 device-command signature wire/preimage/key binding, revoked/cross-device/noncanonical/unknown-algorithm rejection, exact tenancy/key-domain snapshot schema/readback/digest, and locked direct consumer edges.

- [ ] **Step 5: Commit only this task**

Commit: `feat(authz): add dynamic principal capability grants`

### Task G1-03: Make verified transaction-local context the only business database entry

**Files**

- Read/import unchanged: `crates/foundation/src/identifier.rs`
- Modify: `crates/foundation/src/security/context.rs`
- Modify: `crates/foundation/src/security/mod.rs`
- Modify: `crates/foundation/Cargo.toml`
- Modify: `crates/foundation/src/port/tx.rs`
- Create: `crates/adapter/db-pg/src/context_ticket.rs`
- Create: `crates/adapter/db-pg/src/authorized_tx.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/adapter/db-pg/src/conn.rs`
- Modify: `crates/adapter/db-pg/src/fake.rs`
- Modify: `crates/adapter/db-pg/src/pool.rs`
- Modify: `crates/adapter/db-pg/src/session.rs`
- Modify: `crates/adapter/db-pg/src/tx.rs`
- Modify: `crates/adapter/db-pg/src/budget.rs`
- Modify: `db/bootstrap/01_roles.sql`
- Modify: `db/bootstrap/02_cluster_params.sql`
- Modify: `db/bootstrap/03_role_defaults.sql`
- Modify: `db/bootstrap/04_pg_hba.fragment`
- Modify: `db/checks/03_rls_conformance.sql`
- Create: `db/checks/14_verified_transaction_context.sql`
- Modify: `db/checks/README.md`
- Modify: `apps/core-server/src/wiring/db.rs`
- Create: `docs/schemas/f57-security-context-envelope.v1.schema.json`
- Create: `db/migrations/platform_core/V20261025090210__platform_core_create_verified_transaction_contexts.sql`
- Create: `crates/foundation/tests/security_context_wire.rs`
- Modify: `testkit/Cargo.toml`
- Create: `testkit/tests/f57_g1_authorized_pg_tx.rs`

- [ ] **Step 1: Write the failing transaction-context attack matrix**

In `crates/foundation/tests/security_context_wire.rs`, cover the exact strict envelope/schema/media, all ten imported G0 principal kinds, every field mutation, unknown fields/variants, UUID/text/endianness aliases, invalid MAC length/case, both checked time boundaries/overflow, and Rust↔PostgreSQL equality for the frozen binary preimage, HMAC, and context-binding digest. Prove that the security-context schema has exactly one relative import to `../evidence/f57-foundation.v1.schema.json`, does not copy principal/UUID/digest definitions, and cannot create a reverse or network edge.

In `f57_g1_authorized_pg_tx.rs`, cover caller/raw/replayed envelope construction or submission, raw `SET|SET LOCAL`, direct SECURITY-DEFINER invocation, forged verified-context row/GUC/signature, unknown/rotated/revoked issuer key, key epoch reuse/rollback, concurrent issue/rotation cuts, issue-response loss and cross-invocation adoption rejection, wrong nonce/replay, expiry, ticket response loss/reuse, wrong database OID/backend PID/backend-start/xid8/database-role tuple, wrong principal kind/legal entity/policy/authority epoch/generation, readback mismatch, second apply, a repository call without `AuthorizedPgTx`, and every crash cut before/after issue, active-key rotation, ticket commit, and context apply. The pool matrix must reuse the same physical socket after commit, rollback, savepoint rollback, cancellation, timeout, and panic and prove zero role/GUC/protocol residue; uncertain rollback/discard/readback must prove socket destruction.

- [ ] **Step 2: Run the focused tests and confirm RED**

Run: `cargo test -p ep-foundation --test security_context_wire -- --nocapture`

Run: `cargo test -p ep-testkit --test f57_g1_authorized_pg_tx -- --nocapture`

Expected: FAIL because the exact security-context wire/preimage and `AuthorizedPgTx` DB-side verification do not exist.

- [ ] **Step 3: Implement the private constructor and DB verification**

Implement the frozen §2 contract without aliases or overloads. `crates/foundation/src/security/context.rs` is G1-03's sole Rust owner for private-constructor `VerifiedSecurityContextV1` and the exact envelope/purpose/algorithm/MAC types; the strict security-context schema is its sole wire owner and imports only the zero-import foundation schema. `context_ticket.rs` alone owns private `SecurityContextIssuerV1|IssuedSecurityContextEnvelopeV1|SecurityContextBackendBindingV1|SecurityContextBackendTicketV1`, fixed binary preimage/HMAC verification, issuer-key registry access, nonce claims, ticket issuance, and typed verification readback. `authorized_tx.rs` alone owns the opaque `AuthorizedPgTx` and `begin_authorized(&VerifiedSecurityContextV1)` implementation. No path accepts a caller envelope, raw security fields, caller time/nonce/key, a raw pool/connection, or a repository operation before authorization.

The migration creates the closed issuer-key registry, durable nonce/ticket tombstones, and `verified_transaction_contexts` with their exact keys, state checks, partial uniqueness, expiry, five-field backend/xid8 binding, and least-privilege SECURITY-DEFINER functions. Issuance uses a disjoint context-issuer transaction and the sole `ISSUE_ACTIVE` key; rotation is atomic old-or-new with `new_epoch=old+1` and previous-key `VERIFY_ONLY` grace bounded to 30000 ms. The separate committed ticket transaction stores only the SHA-256 of 32 CSPRNG ticket bytes, durably burns delivery/replay authority, and never rolls back with the business transaction. Apply independently re-verifies HMAC, nonce/ticket, current policy/authority/generation, expiry, current backend tuple, inserts the unique verified row, uses only transaction-local `set_config(...,true)`, and returns every field for byte-exact adapter readback before private construction.

RLS helpers authorize only through the verified-row/current-backend-xid8 join and binding digest; they never trust a raw GUC. Checkout proves `IDLE`, expected roles, and null context. Every exit first reaches out-of-transaction state, runs `DISCARD ALL`, and rechecks role/protocol/GUCs; any uncertainty closes the socket. Response loss follows the frozen three-way rule: no issuer product before completed issue, no authority before ticket commit, and irreversible burn after ticket commit; every retry begins with a fresh private `issue`.

- [ ] **Step 4: Prove architecture and Fresh PostgreSQL conformance**

Run: `cargo xtask sqlcheck`

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090210`

Run: `cargo xtask archcheck`

Expected: PASS; static source scanning finds no business repository accepting a raw connection, no caller-envelope constructor or deserializer-to-authority path, no session-scoped security context, one exact G1-03 Rust/schema owner set, and the single foundation-schema import with no cycle or copied nominal.

- [ ] **Step 5: Run GREEN and commit**

Run: `cargo test -p ep-foundation -p ep-adapter-db-pg --all-targets`

Run: `cargo test -p ep-testkit --test f57_g1_authorized_pg_tx -- --nocapture`

Expected: PASS for exact wire/preimage/HMAC equality, private issue-only construction, active-key rotation and rollback rejection, nonce/ticket response-loss burn, every backend binding mismatch, direct row/function/GUC forgery, typed readback, crash cuts, and real physical-connection residue checks.

Commit: `feat(db): require verified authorized PostgreSQL transactions`

### Task G1-04: Add one typed atomic command, fact, audit, Outbox, and receipt path

**Files**

- Read/import unchanged: `crates/foundation/src/identifier.rs`
- Create: `crates/platform/command/Cargo.toml`
- Create: `crates/platform/command/src/lib.rs`
- Create: `crates/platform/command/src/envelope.rs`
- Create: `crates/platform/command/src/generated_contracts.rs`
- Create: `crates/platform/command/src/gateway.rs`
- Create: `crates/platform/command/src/handler.rs`
- Create: `crates/platform/command/src/registry.rs`
- Create: `crates/platform/command/src/pipeline.rs`
- Create: `crates/platform/command/src/receipt.rs`
- Create: `crates/platform/command/src/commit.rs`
- Create: `crates/platform/command/tests/registry.rs`
- Create: `crates/platform/command/tests/wire.rs`
- Create: `crates/platform/command/tests/fixtures/internal-command-receipt-v1.jcs.json`
- Create: `docs/schemas/f57-internal-command-contract.v1.schema.json`
- Modify: `crates/foundation/src/port/db.rs`
- Create: `crates/adapter/db-pg/src/platform_msg/command_receipt.rs`
- Create: `crates/adapter/db-pg/src/platform_msg/outbox.rs`
- Create: `crates/adapter/db-pg/src/platform_msg/atomic_commit.rs`
- Modify: `crates/adapter/db-pg/src/platform_msg/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/store.rs`
- Create: `crates/adapter/db-pg/src/platform_audit/chain_store.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/platform/audit/src/lib.rs`
- Modify: `crates/platform/outbox/src/lib.rs`
- Create: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Create: `db/migrations/platform_msg/V20261025090300__platform_msg_create_command_receipts_and_outbox.sql`
- Create: `db/migrations/platform_audit/V20261025090310__platform_audit_create_entries_and_segments.sql`
- Create: `testkit/tests/f57_g1_command_pipeline.rs`
- Create: `testkit/src/f57_cases/g1/authority_command.rs`
- Create: `testkit/src/f57_cases/g1/transactional_evidence.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

- [ ] **Step 1: Write failing atomicity and idempotency tests**

```rust
#[tokio::test]
async fn every_failure_point_leaves_all_five_commit_classes_or_none() {
    for point in CommitPointV1::all() {
        inject_failure_after(point).await;
        assert_zero_partial_commit().await;
    }
}

#[tokio::test]
async fn same_idempotency_key_with_different_payload_is_rejected() {
    let first = execute(canonical_command_a()).await.unwrap();
    assert_replayed(execute(canonical_command_a()).await.unwrap(), first);
    assert_code(execute(canonical_command_b()).await.unwrap_err(), "COMMAND_IDEMPOTENCY_PAYLOAD_MISMATCH");
}
```

- [ ] **Step 2: Run the focused test and confirm RED**

Run: `cargo test -p ep-testkit --test f57_g1_command_pipeline -- --nocapture`

Expected: FAIL because no single command registry and commit coordinator exists.

- [ ] **Step 3: Implement the only command path**

Handlers register against graph-known command/payload/result schema IDs. The idempotency key is uniquely scoped by legal entity, principal kind/id, command type, and idempotency key; canonical payload digest distinguishes replay from misuse. `EmployeeCommandEnvelopeV1.payload` may exist as raw `serde_json::Value` only inside the Employee protocol adapter before strict graph-selected decoding. The adapter must reject unknown fields/tags and convert it to the closed generated `CapabilityCommandPayloadV1` before constructing `IngressCommandEnvelopeV1`; raw JSON, an untyped byte buffer, or a caller-selected schema ID cannot cross the gateway boundary.

This task implements the master's exact internal contract set and is its sole Rust/schema owner for `SubjectRefV1`; the six generated no-fallback command/result/fact/audit/outbox unions; `CapabilityCommandV1`; strict internal `CommandReceiptV1`; closed `CommandErrorV1`; and `CommandCommitSetV1` with its three draft types. G1-04 imports and consumes G1-03's private-constructor `VerifiedSecurityContextV1` nominal but never owns, redefines, deserializes, or raw-constructs it. `generated_contracts.rs` is generated only from the compiled capability graph and byte-compared to the registry; raw `serde_json::Value`, String/Other variants and client-supplied security fields fail compilation or parsing. `receipt.rs` produces exact JCS media `application/vnd.ep.f57-internal-command-receipt-v1+json`; `wire.rs` covers both outcomes, unknown fields, tag/case drift, positive subject version, sorted/unique fact/Outbox IDs and schema-byte equality. This internal receipt is persisted for idempotent replay and is explicitly excluded from OpenAPI and TypeScript; later Control/Employee/Portal adapters must project it into their route-seed-owned named response types.

`commit.rs` validates that commit subject, every fact, the one audit and every Outbox row are byte-identical; facts are nonempty and sorted by `fact_id`, Outbox is sorted by `outbox_message_id`, IDs are unique/nonzero, and `subject_version=locked_previous+1`. The transaction adds actor/time/generation/audit-hash fields only from `VerifiedSecurityContextV1`, commits the typed result/receipt bytes together, and makes a lost-response retry return those bytes unchanged. `gateway.rs` owns the exhaustive safe mapping from every `CommandErrorV1` variant to the per-route generated error allowlist; unlisted mappings collapse to `PLATFORM.SYSTEM.NOT_READY` without debug text. Atomic DB readback tests compare feature state, fact rows, audit chain, Outbox and receipt as one five-class exact set after every injected failure and retry.

The transaction sequence is fixed:

```text
begin AuthorizedPgTx
→ reserve receipt
→ lock authority epoch and evaluate live authorization
→ verify generation and acquire pin
→ validate expected subject version
→ feature current-state mutation plus feature-owned immutable facts
→ deterministic in-transaction fact reactors
→ append one audit-chain entry
→ append Outbox rows
→ finalize receipt
→ COMMIT
```

An external call is represented by an Outbox/effect request and happens after commit. No alternate handler, admin SQL, plugin, Excel, MCP, portal, or worker path may bypass the pipeline.

- [ ] **Step 4: Verify Fresh PostgreSQL, failure injection, and GREEN**

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090310`

`authority_command.rs` implements exactly `GOV-001`, `GOV-008`, and `INT-005`; `transactional_evidence.rs` implements only `GOV-009`. The generated canonical facades remain unchanged.

Run: `cargo test -p ep-platform-command -p ep-platform-audit -p ep-platform-outbox -p ep-adapter-db-pg -p core-server -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g1_command_pipeline -- --nocapture`

Expected: PASS for duplicate response loss, stale version, revoke-before-commit, crash before/after commit, audit-chain continuity, and zero partial commit.

- [ ] **Step 5: Commit only this task**

Commit: `feat(command): commit state facts audit outbox and receipt atomically`

### Task G1-05: Persist signed generation activation, participant ACK, and artifact pins

**Files**

- Modify: `crates/platform/release/src/generation.rs`
- Modify: `crates/platform/release/src/participant.rs`
- Create: `crates/platform/release/tests/participant_readback.rs`
- Modify: `docs/evidence/f57-generation.v1.schema.json`
- Create: `crates/platform/release/tests/fixtures/generation-participant-apply-readback-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-participant-rollback-readback-v1.jcs.json`
- Modify: `crates/platform/release/tests/fixtures/generation-participant-ack-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-participant-deactivated-retain-data-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-participant-no-observed-generation-v1.jcs.json`
- Modify: `crates/platform/release/src/pin.rs`
- Create: `crates/platform/release/src/store.rs`
- Create: `crates/platform/release/src/activation_attempt.rs`
- Create: `crates/platform/release/tests/activation_attempt.rs`
- Create: `crates/platform/release/tests/fixtures/generation-transition-row-hash-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-observed-release-selection-v1.jcs.json`
- Create: `crates/platform/release/tests/fixtures/generation-observed-release-selection-resolution-v1.jcs.json`
- Create: `docs/evidence/f57-generation-observed-release-selection.v1.schema.json`
- Create: `docs/evidence/f57-generation-observed-release-selection-resolution.v1.schema.json`
- Modify: `crates/platform/release/src/lib.rs`
- Create: `crates/platform/generation-activation/Cargo.toml`
- Create: `crates/platform/generation-activation/src/lib.rs`
- Create: `crates/platform/generation-activation/src/coordinator.rs`
- Create: `crates/platform/generation-activation/src/verified_graph.rs`
- Create: `crates/platform/generation-activation/src/model.rs`
- Create: `crates/platform/generation-activation/tests/activation.rs`
- Read: `crates/platform/runtime/src/topology.rs`
- Read: `docs/evidence/f57-runtime-topology.v1.schema.json`
- Create: `crates/adapter/db-pg/src/platform_meta/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/generation_store.rs`
- Create: `crates/adapter/db-pg/src/platform_meta/artifact_retention_store.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/platform/release/Cargo.toml`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `apps/core-server/src/wiring/release.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `apps/job-worker/src/wiring/generation.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`
- Modify: `crates/platform/flow/Cargo.toml`
- Modify: `crates/platform/runtime/Cargo.toml`
- Modify: `apps/job-worker/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.lock`
- Create: `db/migrations/platform_meta/V20261025090400__platform_meta_create_generations_participants_and_pins.sql`
- Create: `testkit/tests/f57_g1_generation_activation.rs`
- Create: `testkit/src/f57_cases/g1/generation_faults.rs`
- Modify: `Cargo.toml`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

**Interfaces:**
- Consumes: the G0-owned approved generation/reverse-plan roots and existing generation schema owner, private `VerifiedGenerationManifestV1`, G1-01's exact persisted manifest/approval/declaration triple, G0's domain-neutral authenticated journal-prefix read port, G1-03's private `VerifiedSecurityContextV1`, the authority `EvidenceObjectStoreV1`, and injected trusted clock/boot-instance sources. It accepts no raw attempt ID, current/latest query, caller event, caller clock, raw resolution ref or unverified journal bytes.
- Produces: the exact `13/9/14` generation-manifest/reverse-plan/ACK family; strict apply/rollback participant-readback roots with tagged item/rollback-target unions; their media/schema/goldens, create-new object-store path and durable SQL reference projection; `GenerationActivationAttemptStoreV1`, separate first-write/persisted ACK-draft nominals, `GenerationTransitionStoreV1` plus exact `recover_on_authority_boot`, `GenerationObservedReleaseSelectionPortV1` with live renew, same-run expired resume, proof-only candidate bind/release and the separate authorized/persisted dual-control resolution nominals for `FAILED_RELEASED`, the two strict selection/resolution schemas and goldens, and PostgreSQL-enforced transition/attempt/selection/pin state. A package generation transition is reachable only as `ACK.participant_apply_readback_ref -> apply-readback.applied_items[DESIRED_ITEM].generation_transition_ref`; G6 and later package orchestration consume these ports without redefining any wire or querying a latest generation.

- [ ] **Step 1: Write failing activation and garbage-collection tests**

```rust
#[tokio::test]
async fn observed_moves_only_after_the_exact_required_ack_set() {
    let pending = activate_valid_signed_generation().await;
    ack_all_but_one(&pending).await;
    assert_ne!(desired_digest().await, observed_digest().await);
}

#[tokio::test]
async fn ack_is_server_derived_and_one_attempt_cannot_be_mixed() {
    let a = activate_valid_signed_generation().await;
    let b = restart_activation_without_adopting(a.activation_attempt_id()).await;
    assert_code(b, "GENERATION_ACTIVATION_ATTEMPT_CONFLICT");
    assert_code(submit_participant_supplied_ack(&a).await,
        "GENERATION_PARTICIPANT_ACK_INPUT_FORBIDDEN");
    assert_code(store_ack_with_other_item_set_or_definition(&a).await,
        "GENERATION_ACK_BINDING_MISMATCH");
}

#[tokio::test]
async fn ack_reaches_each_package_transition_only_through_its_apply_readback() {
    let expected_transition_ref = frozen_media_constrained_package_transition_ref();
    let observed = activate_generation_with_capability_package_item(
        expected_transition_ref.clone(),
    )
    .await
    .unwrap();
    let ack = load_exact_ack(observed.ack_ref()).await.unwrap();
    assert_eq!(ack.field_count(), 14);
    let apply = load_exact_apply_readback(ack.participant_apply_readback_ref())
        .await
        .unwrap();
    assert_eq!(apply.field_count(), 16);
    let package_row = apply.desired_item("capability-package").unwrap();
    assert_eq!(package_row.generation_transition_ref(), Some(&expected_transition_ref));
    assert_eq!(expected_transition_ref.media_type,
        "application/vnd.ep.f57-capability-package-generation-transition-v1+json");
    assert_code(apply_with_deactivated_retain_data_variant().await,
        "GENERATION_APPLY_READBACK_ITEM_KIND_FORBIDDEN");
    assert_code(package_apply_without_transition_ref().await,
        "GENERATION_PACKAGE_TRANSITION_REF_REQUIRED");
}

#[tokio::test]
async fn rollback_target_and_item_variants_are_total_for_generation_one() {
    let first = fail_forward_activation_of_generation_one().await;
    let rolled_back = rollback_without_prior_observed_generation(first).await.unwrap();
    assert_eq!(rolled_back.rollback_target_kind(), "NO_OBSERVED_GENERATION");
    assert!(rolled_back
        .restored_items()
        .iter()
        .any(|row| row.readback_kind() == "DEACTIVATED_RETAIN_DATA"));
    assert_code(rollback_generation_one_with_fabricated_predecessor().await,
        "GENERATION_ROLLBACK_TARGET_MISMATCH");
    assert_code(rollback_later_generation_without_predecessor().await,
        "GENERATION_ROLLBACK_TARGET_MISMATCH");
    assert_code(rollback_with_deactivated_row_for_existing_predecessor_item().await,
        "GENERATION_ROLLBACK_ITEM_KIND_MISMATCH");
}

#[tokio::test]
async fn boot_recovery_reuses_the_exact_transition_attempt_at_every_cut() {
    for cut in [
        ActivationCrashCutV1::AttemptStarted,
        ActivationCrashCutV1::ParticipantDispatchStarted,
        ActivationCrashCutV1::ParticipantDispatchUnknown,
        ActivationCrashCutV1::ParticipantReconciled,
        ActivationCrashCutV1::AckDraftFrozen,
        ActivationCrashCutV1::AckObjectBound,
        ActivationCrashCutV1::ObservedAttemptCommittedBeforePointer,
        ActivationCrashCutV1::RollbackStarted,
        ActivationCrashCutV1::RollbackAttemptCommittedBeforePointer,
    ] {
        let before = activate_until_crash(cut).await;
        let recovered = boot_recover_exact(before.deployment_id(), before.authority_epoch()).await.unwrap();
        assert_eq!(recovered.activation_attempt_id(), before.activation_attempt_id());
        assert_eq!(recovered.physical_redispatch_count(), 0);
        assert_code(boot_recovery_with_new_attempt_or_pointer_drift(cut).await,
            "GENERATION_TRANSITION_ATTEMPT_MISMATCH");
    }

    assert_compile_fail_transition_store_caller_clock_owner_or_expiry();
    let held = owner_death_before_exact_lease_boundary().await;
    assert_code(held, "GENERATION_TRANSITION_LEASE_HELD");
    assert!(owner_death_at_exact_lease_boundary_resumes_same_attempt().await.is_ok());
    assert_code(takeover_with_replayed_or_modified_persisted_lease_tuple().await,
        "GENERATION_TRANSITION_LEASE_CAS_CONFLICT");
}

#[tokio::test]
async fn codec_valid_ack_ahead_of_store_cannot_become_a_freeze_command() {
    assert_compile_fail_persisted_ack_proof_as_freeze_command();
    assert_code(forged_snapshot_with_unpersisted_ack_draft().await,
        "GENERATION_ACTIVATION_CAS_CONFLICT");
    assert!(crash_after_frozen_ack_before_object_ingest_resumes_same_bytes().await.is_ok());
    assert!(crash_after_ack_object_ingest_before_bound_resumes_same_ref().await.is_ok());
}

#[tokio::test]
async fn expired_release_selection_has_only_same_run_resume_or_dual_control_release() {
    assert_code(begin_selection_with_stale_non_head_or_unlocked_prefix().await,
        "GENERATION_SELECTION_PROGRESS_CHECKPOINT_MISMATCH");
    let before_checkpoint =
        expire_selection_after_insert_before_first_selection_bound_checkpoint().await;
    let bootstrap_recovered =
        recover_selection_from_same_run_authenticated_preselection_prefix(&before_checkpoint)
            .await
            .unwrap();
    assert_eq!(bootstrap_recovered.selection_ref(), before_checkpoint.selection_ref());
    assert!(bootstrap_recovered.lease_version() > before_checkpoint.lease_version());
    let live = renew_same_preselection_checkpoint_while_copy_is_running(&bootstrap_recovered)
        .await
        .unwrap();
    let copying = append_preselection_copy_checkpoint_then_renew(&live)
        .await
        .unwrap();
    assert_eq!(copying.checkpoint_binding(),
        GenerationObservedSelectionCheckpointBindingV1::Preselection);
    assert_code(append_first_candidate_finalization_with_missing_or_wrong_selection(&copying).await,
        "GENERATION_SELECTION_PROGRESS_CHECKPOINT_MISMATCH");
    assert_compile_fail_selection_bind_with_raw_candidate_ref();
    assert_code(bind_candidate_before_matching_bound_progress(&copying).await,
        "GENERATION_SELECTION_CANDIDATE_BINDING_CONFLICT");
    assert!(append_first_selection_bound_checkpoint_then_renew(&copying).await.is_ok());
    assert!(recover_after_selection_bound_append_before_progress_cas()
        .await
        .is_ok());

    let expired = expire_selection_after_candidate_bytes_before_bound().await;
    let resumed = resume_with_same_authenticated_prefix(&expired).await.unwrap();
    assert_eq!(resumed.selection_ref(), expired.selection_ref());
    assert!(resumed.lease_version() > expired.lease_version());
    assert_code(resume_with_other_run_or_observed_or_checkpoint(&expired).await,
        "GENERATION_SELECTION_PROGRESS_CHECKPOINT_MISMATCH");
    let released = append_matching_candidate_bound_then_release(&resumed).await.unwrap();
    assert_eq!(released.state(), GenerationObservedReleaseSelectionStateV1::BoundReleased);
    assert!(released.candidate_manifest_ref().is_some());
    assert!(adopt_after_bound_release_response_loss(&released).await.is_ok());

    let unrecoverable = expire_selection_with_unrecoverable_freeze_progress().await;
    assert_code(resolve_with_one_or_same_operator(&unrecoverable).await,
        "GENERATION_SELECTION_RESOLUTION_NOT_AUTHORIZED");
    assert_compile_fail_raw_resolution_wire_or_ref_as_persisted_proof();
    let resolution_cut = expire_separate_selection_for_resolution_object_crash().await;
    assert!(crash_after_resolution_object_requires_fresh_two_human_authorization(&resolution_cut)
        .await
        .is_ok());
    assert_code(begin_another_run_before_resolution(&unrecoverable).await,
        "GENERATION_SELECTION_LEASE_CONFLICT");
    let resolved = resolve_with_two_distinct_authorized_humans(&unrecoverable).await.unwrap();
    assert_eq!(resolved.state(), GenerationObservedReleaseSelectionStateV1::FailedReleased);
    assert!(resolved.resolution_ref_is_immutable_and_typed());
    assert!(begin_another_run_after_failed_release(&unrecoverable).await.is_ok());
}

#[tokio::test]
async fn unknown_effect_persistent_reference_blocks_artifact_gc() {
    let pin = persist_unknown_effect_reference().await;
    expire_lease_clock().await;
    assert_eq!(collect(pin.artifact()).await, CollectOutcomeV1::BlockedReferenced);
}
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p ep-testkit --test f57_g1_generation_activation -- --nocapture`

Expected: FAIL because activation state, participant ACKs, and persisted pins do not exist.

- [ ] **Step 3: Implement verified activation only**

The sole activation entry accepts verified graph, topology, and generation artifacts:

```rust
pub async fn activate(
    &self,
    generation: VerifiedGenerationManifestV1,
) -> Result<ActivationOutcomeV1, ActivationErrorV1>;

// Called exactly once by ordinary Authority boot composition after the verified
// deployment/epoch and this boot's unpredictable instance ID are installed in
// the coordinator. It accepts no generation, attempt ID, pointer, path or clock.
pub async fn recover_on_authority_boot(
    &self,
) -> Result<ActivationBootOutcomeV1, ActivationErrorV1>;
```

`crates/platform/generation-activation/src/model.rs` solely owns the non-wire public outcome/error boundary:

```rust
pub enum ActivationOutcomeV1 {
    Observed { activation_attempt_id: UuidV1, generation_digest_sha256: Sha256Digest },
    AlreadyObserved { activation_attempt_id: UuidV1, generation_digest_sha256: Sha256Digest },
    Unknown { activation_attempt_id: UuidV1, participant_id: String },
    RolledBack {
        activation_attempt_id: UuidV1,
        rollback_target: GenerationParticipantRollbackTargetV1,
    },
}

pub enum ActivationBootOutcomeV1 {
    Stable,
    Resumed { activation_attempt_id: UuidV1, terminal: GenerationActivationTerminalStateV1 },
    HeldByLiveOwner { activation_attempt_id: UuidV1, lease_expires_at_unix_ms: i64 },
    Failed { activation_attempt_id: UuidV1, failure_readback_ref: ArtifactRefV1 },
}

pub enum ActivationErrorV1 {
    GenerationProofInvalid,
    PersistedTripleMismatch,
    TransitionConflict,
    TransitionLeaseHeld,
    TransitionPointerDrift,
    BootRecoveryConflict,
    GraphRecompileMismatch,
    ProjectionMismatch,
    TopologyReadbackMismatch,
    ParticipantSetMismatch,
    ParticipantResponseUnknown,
    AckDraftConflict,
    AckObjectConflict,
    ObservedCommitConflict,
    RollbackTargetMismatch,
    StoreUnavailable,
}
```

`crates/platform/release/src/participant.rs` and `docs/evidence/f57-generation.v1.schema.json` solely own these strict plain roots. The already G0-owned generation manifest and reverse plan remain exact thirteen- and nine-field signed payloads; G1-05 changes neither wire. The ACK becomes the exact fourteen-field plain authenticated-internal root shown below:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationParticipantApplyReadbackPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-PARTICIPANT-APPLY-READBACK-V1")]
    GenerationParticipantApplyReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationParticipantRollbackReadbackPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-PARTICIPANT-ROLLBACK-READBACK-V1")]
    GenerationParticipantRollbackReadback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationParticipantReadbackOutcomeV1 {
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "readback_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum GenerationParticipantItemReadbackV1 {
    DesiredItem {
        item_id: GenerationItemIdV1,
        generation_item_ref: ArtifactRefV1,
        generation_transition_ref: Option<ArtifactRefV1>,
        installed_state_readback_ref: ArtifactRefV1,
    },
    DeactivatedRetainData {
        item_id: GenerationItemIdV1,
        rolled_back_generation_item_ref: ArtifactRefV1,
        generation_transition_ref: ArtifactRefV1,
        installed_state_readback_ref: ArtifactRefV1,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationParticipantApplyReadbackV1 {
    pub schema_version: u32,
    pub purpose: GenerationParticipantApplyReadbackPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub activation_attempt_id: UuidV1,
    pub generation_manifest_ref: ArtifactRefV1,
    pub generation_digest_sha256: Sha256Digest,
    pub topology_declaration_ref: ArtifactRefV1,
    pub participant_id: String,
    pub participant_definition_sha256: Sha256Digest,
    pub applied_items: Vec<GenerationParticipantItemReadbackV1>,
    pub applied_item_set_sha256: Sha256Digest,
    pub readiness_refs: Vec<ArtifactRefV1>,
    pub outcome: GenerationParticipantReadbackOutcomeV1,
    pub observed_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "rollback_target_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum GenerationParticipantRollbackTargetV1 {
    PriorObservedGeneration {
        predecessor_generation_manifest_ref: ArtifactRefV1,
        predecessor_generation_digest_sha256: Sha256Digest,
    },
    NoObservedGeneration,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationParticipantRollbackReadbackV1 {
    pub schema_version: u32,
    pub purpose: GenerationParticipantRollbackReadbackPurposeV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub activation_attempt_id: UuidV1,
    pub rollback_execution_attempt_id: UuidV1,
    pub participant_id: String,
    pub rollback_target: GenerationParticipantRollbackTargetV1,
    pub restored_items: Vec<GenerationParticipantItemReadbackV1>,
    pub restored_item_set_sha256: Sha256Digest,
    pub readiness_refs: Vec<ArtifactRefV1>,
    pub outcome: GenerationParticipantReadbackOutcomeV1,
    pub observed_at_unix_ms: i64,
}

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
```

`crates/platform/release/src/activation_attempt.rs` freezes the following lower-layer, object-safe storage boundary before the upper coordinator is compiled:

```rust
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationActivationAttemptBindingV1 {
    pub activation_attempt_id: UuidV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub generation_number: GenerationNumberV1,
    pub generation_manifest_ref: ArtifactRefV1,
    pub generation_digest_sha256: Sha256Digest,
    pub generation_approval_registry_ref: ArtifactRefV1,
    pub topology_declaration_ref: ArtifactRefV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationActivationAttemptHeaderV1 {
    pub schema_version: u32,
    pub binding: GenerationActivationAttemptBindingV1,
    pub required_participant_ids: Vec<String>,
    pub started_at_unix_ms: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationActivationAttemptRecordV1 {
    pub schema_version: u32,
    pub binding: GenerationActivationAttemptBindingV1,
    pub sequence: u64,
    pub expected_prior_cas_version: u64,
    pub committed_cas_version: u64,
    pub previous_record_sha256: Option<Sha256Digest>,
    pub event: GenerationActivationAttemptEventV1,
    pub recorded_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationParticipantReconciliationOutcomeV1 {
    Applied,
    NotApplied,
    StillUnknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationActivationFailureCodeV1 {
    ParticipantReadbackMismatch,
    ParticipantEffectUnknown,
    AckDraftConflict,
    AckObjectConflict,
    ExactAckSetMismatch,
    RollbackTargetMismatch,
    RollbackFailed,
    StoreIntegrityFailure,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "event_kind", rename_all = "SCREAMING_SNAKE_CASE", deny_unknown_fields)]
pub enum GenerationActivationAttemptEventV1 {
    AttemptStarted {
        required_participant_ids: Vec<String>,
        started_at_unix_ms: i64,
    },
    ParticipantDispatchStarted {
        participant_id: String,
        dispatch_attempt_id: UuidV1,
        required_item_set_sha256: Sha256Digest,
    },
    ParticipantDispatchUnknown {
        participant_id: String,
        dispatch_attempt_id: UuidV1,
        uncertainty_readback_ref: ArtifactRefV1,
    },
    ParticipantReconciled {
        participant_id: String,
        dispatch_attempt_id: UuidV1,
        measured_readback_ref: ArtifactRefV1,
        reconciliation_outcome: GenerationParticipantReconciliationOutcomeV1,
    },
    AckDraftFrozen {
        participant_id: String,
        ack_jcs_bytes: Vec<u8>,
        ack_ref: ArtifactRefV1,
        participant_definition_sha256: Sha256Digest,
        applied_item_set_sha256: Sha256Digest,
        participant_apply_readback_ref: ArtifactRefV1,
        acknowledged_at_unix_ms: i64,
    },
    AckObjectBound {
        participant_id: String,
        ack_ref: ArtifactRefV1,
    },
    ObservedCommitted {
        exact_ack_refs: Vec<ArtifactRefV1>,
        observed_at_unix_ms: i64,
    },
    RollbackStarted {
        rollback_target: GenerationParticipantRollbackTargetV1,
        rollback_execution_attempt_id: UuidV1,
    },
    RollbackCommitted {
        rollback_target: GenerationParticipantRollbackTargetV1,
        participant_rollback_readback_refs: Vec<ArtifactRefV1>,
        committed_at_unix_ms: i64,
    },
    Failed {
        error_code: GenerationActivationFailureCodeV1,
        failure_readback_ref: ArtifactRefV1,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationActivationTerminalStateV1 {
    InProgress,
    Unknown,
    ObservedCommitted,
    RollbackCommitted,
    Failed,
}

// Public nominals with private fields. Only the release-owned verifier/command
// authority can construct them; adapters receive accessors plus canonical bytes.
pub struct GenerationActivationAttemptLookupV1 {
    binding: GenerationActivationAttemptBindingV1,
}

impl GenerationActivationAttemptLookupV1 {
    pub(crate) fn for_release_verification(
        manifest: &VerifiedGenerationManifestV1,
        approval_registry_ref: &ArtifactRefV1,
        topology_declaration_ref: &ArtifactRefV1,
        activation_attempt_id: UuidV1,
    ) -> Result<Self, GenerationActivationAttemptStoreErrorV1>;

    pub fn binding(&self) -> &GenerationActivationAttemptBindingV1 { &self.binding }
}

pub struct GenerationActivationCasV1 {
    activation_attempt_id: UuidV1,
    committed_cas_version: u64,
    last_record_sha256: Sha256Digest,
}

impl GenerationActivationCasV1 {
    pub fn activation_attempt_id(&self) -> UuidV1 { self.activation_attempt_id }
    pub fn committed_cas_version(&self) -> u64 { self.committed_cas_version }
    pub fn last_record_sha256(&self) -> Sha256Digest { self.last_record_sha256 }
}

pub struct VerifiedBeginGenerationActivationAttemptV1 {
    header: GenerationActivationAttemptHeaderV1,
    canonical_header_jcs_bytes: Vec<u8>,
    started_record: GenerationActivationAttemptRecordV1,
    canonical_started_record_jcs_bytes: Vec<u8>,
}

impl VerifiedBeginGenerationActivationAttemptV1 {
    pub fn header(&self) -> &GenerationActivationAttemptHeaderV1 { &self.header }
    pub fn canonical_header_jcs_bytes(&self) -> &[u8] { &self.canonical_header_jcs_bytes }
    pub fn started_record(&self) -> &GenerationActivationAttemptRecordV1 { &self.started_record }
    pub fn canonical_started_record_jcs_bytes(&self) -> &[u8] {
        &self.canonical_started_record_jcs_bytes
    }
}

pub struct VerifiedGenerationActivationTransitionV1 {
    expected: GenerationActivationCasV1,
    record: GenerationActivationAttemptRecordV1,
    canonical_record_jcs_bytes: Vec<u8>,
}

// Live first-write command. Only GenerationActivationAttemptCommandAuthorityV1
// can construct this value from the verified request, fresh measured readback,
// and the one trusted acknowledgement time.
pub struct VerifiedFreezeGenerationAckDraftCommandV1 {
    expected: GenerationActivationCasV1,
    record: GenerationActivationAttemptRecordV1,
    canonical_record_jcs_bytes: Vec<u8>,
    exact_ack_ref: ArtifactRefV1,
    exact_ack_jcs_bytes: Vec<u8>,
}

// Recovery/read-side proof. It can be derived only from a loaded snapshot that
// already contains the exact persisted ACK_DRAFT_FROZEN record. It is not a
// store command and can only be consumed by ack_object_bound.
pub struct VerifiedPersistedGenerationAckDraftV1 {
    frozen_at: GenerationActivationCasV1,
    participant_id: String,
    canonical_frozen_record_jcs_bytes: Vec<u8>,
    exact_ack_ref: ArtifactRefV1,
    exact_ack_jcs_bytes: Vec<u8>,
}

pub struct VerifiedObservedGenerationCommitV1 {
    expected: GenerationActivationCasV1,
    record: GenerationActivationAttemptRecordV1,
    canonical_record_jcs_bytes: Vec<u8>,
    exact_ack_refs: Vec<ArtifactRefV1>,
}

pub trait VerifiedGenerationActivationStoreCommandViewV1 {
    fn expected(&self) -> &GenerationActivationCasV1;
    fn record(&self) -> &GenerationActivationAttemptRecordV1;
    fn canonical_record_jcs_bytes(&self) -> &[u8];
}

impl VerifiedGenerationActivationStoreCommandViewV1
    for VerifiedGenerationActivationTransitionV1
{
    fn expected(&self) -> &GenerationActivationCasV1 { &self.expected }
    fn record(&self) -> &GenerationActivationAttemptRecordV1 { &self.record }
    fn canonical_record_jcs_bytes(&self) -> &[u8] { &self.canonical_record_jcs_bytes }
}

impl VerifiedGenerationActivationStoreCommandViewV1
    for VerifiedFreezeGenerationAckDraftCommandV1
{
    fn expected(&self) -> &GenerationActivationCasV1 { &self.expected }
    fn record(&self) -> &GenerationActivationAttemptRecordV1 { &self.record }
    fn canonical_record_jcs_bytes(&self) -> &[u8] { &self.canonical_record_jcs_bytes }
}

impl VerifiedGenerationActivationStoreCommandViewV1
    for VerifiedObservedGenerationCommitV1
{
    fn expected(&self) -> &GenerationActivationCasV1 { &self.expected }
    fn record(&self) -> &GenerationActivationAttemptRecordV1 { &self.record }
    fn canonical_record_jcs_bytes(&self) -> &[u8] { &self.canonical_record_jcs_bytes }
}

impl VerifiedFreezeGenerationAckDraftCommandV1 {
    pub fn exact_ack_ref(&self) -> &ArtifactRefV1 { &self.exact_ack_ref }
    pub fn exact_ack_jcs_bytes(&self) -> &[u8] { &self.exact_ack_jcs_bytes }
}

impl VerifiedPersistedGenerationAckDraftV1 {
    pub fn exact_ack_ref(&self) -> &ArtifactRefV1 { &self.exact_ack_ref }
    pub fn exact_ack_jcs_bytes(&self) -> &[u8] { &self.exact_ack_jcs_bytes }
}

impl VerifiedObservedGenerationCommitV1 {
    pub fn exact_ack_refs(&self) -> &[ArtifactRefV1] { &self.exact_ack_refs }
}

pub struct VerifiedGenerationActivationAttemptV1 {
    header: GenerationActivationAttemptHeaderV1,
    records: Vec<GenerationActivationAttemptRecordV1>,
    canonical_header_jcs_bytes: Vec<u8>,
    canonical_record_jcs_bytes: Vec<Vec<u8>>,
    cas: GenerationActivationCasV1,
    terminal_state: GenerationActivationTerminalStateV1,
}

impl VerifiedGenerationActivationAttemptV1 {
    pub fn persisted_frozen_ack_draft(
        &self,
        participant_id: &str,
    ) -> Result<VerifiedPersistedGenerationAckDraftV1,
                GenerationActivationAttemptStoreErrorV1>;
}

pub struct VerifiedObservedGenerationActivationAttemptV1 {
    verified: VerifiedGenerationActivationAttemptV1,
    exact_ack_refs: Vec<ArtifactRefV1>,
    started_at_unix_ms: i64,
    observed_at_unix_ms: i64,
}

impl VerifiedObservedGenerationActivationAttemptV1 {
    pub fn attempt(&self) -> &VerifiedGenerationActivationAttemptV1 { &self.verified }
}

pub struct GenerationActivationAttemptCodecV1;

impl GenerationActivationAttemptCodecV1 {
    pub fn verify_loaded(
        header_jcs_bytes: &[u8],
        record_jcs_bytes: &[Vec<u8>],
    ) -> Result<VerifiedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    pub fn into_terminal_observed(
        attempt: VerifiedGenerationActivationAttemptV1,
    ) -> Result<VerifiedObservedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;
}

pub trait VerifiedGenerationActivationAttemptViewV1 {
    fn binding(&self) -> &GenerationActivationAttemptBindingV1;
    fn header(&self) -> &GenerationActivationAttemptHeaderV1;
    fn records(&self) -> &[GenerationActivationAttemptRecordV1];
    fn cas(&self) -> &GenerationActivationCasV1;
    fn canonical_header_jcs_bytes(&self) -> &[u8];
    fn canonical_record_jcs_bytes(&self) -> &[Vec<u8>];
    fn terminal_state(&self) -> GenerationActivationTerminalStateV1;
    fn into_terminal_observed(
        self,
    ) -> Result<VerifiedObservedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>
    where
        Self: Sized;
}

pub trait VerifiedObservedGenerationActivationAttemptViewV1 {
    fn binding(&self) -> &GenerationActivationAttemptBindingV1;
    fn required_participant_ids(&self) -> &[String];
    fn exact_ack_refs(&self) -> &[ArtifactRefV1];
    fn started_at_unix_ms(&self) -> i64;
    fn observed_at_unix_ms(&self) -> i64;
}

impl VerifiedGenerationActivationAttemptViewV1 for VerifiedGenerationActivationAttemptV1 {
    fn binding(&self) -> &GenerationActivationAttemptBindingV1 { &self.header.binding }
    fn header(&self) -> &GenerationActivationAttemptHeaderV1 { &self.header }
    fn records(&self) -> &[GenerationActivationAttemptRecordV1] { &self.records }
    fn cas(&self) -> &GenerationActivationCasV1 { &self.cas }
    fn canonical_header_jcs_bytes(&self) -> &[u8] { &self.canonical_header_jcs_bytes }
    fn canonical_record_jcs_bytes(&self) -> &[Vec<u8>] { &self.canonical_record_jcs_bytes }
    fn terminal_state(&self) -> GenerationActivationTerminalStateV1 { self.terminal_state }
    fn into_terminal_observed(
        self,
    ) -> Result<VerifiedObservedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1> {
        GenerationActivationAttemptCodecV1::into_terminal_observed(self)
    }
}

impl VerifiedObservedGenerationActivationAttemptViewV1
    for VerifiedObservedGenerationActivationAttemptV1
{
    fn binding(&self) -> &GenerationActivationAttemptBindingV1 {
        &self.verified.header.binding
    }
    fn required_participant_ids(&self) -> &[String] {
        &self.verified.header.required_participant_ids
    }
    fn exact_ack_refs(&self) -> &[ArtifactRefV1] { &self.exact_ack_refs }
    fn started_at_unix_ms(&self) -> i64 { self.started_at_unix_ms }
    fn observed_at_unix_ms(&self) -> i64 { self.observed_at_unix_ms }
}

pub struct GenerationActivationAttemptCommandAuthorityV1;

pub trait GenerationActivationAttemptCommandMintV1 {
    fn begin(
        &self,
        manifest: &VerifiedGenerationManifestV1,
        approval_registry_ref: &ArtifactRefV1,
        topology_declaration_ref: &ArtifactRefV1,
        activation_attempt_id: UuidV1,
        required_participant_ids: &[String],
        started_at_unix_ms: i64,
    ) -> Result<VerifiedBeginGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn resume_lookup(
        &self,
        manifest: &VerifiedGenerationManifestV1,
        approval_registry_ref: &ArtifactRefV1,
        topology_declaration_ref: &ArtifactRefV1,
        persisted_activation_attempt_id: UuidV1,
    ) -> Result<GenerationActivationAttemptLookupV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn participant_dispatch_started(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        request: &VerifiedGenerationParticipantApplyRequestV1,
        dispatch_attempt_id: UuidV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn participant_dispatch_unknown(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        request: &VerifiedGenerationParticipantApplyRequestV1,
        dispatch_attempt_id: UuidV1,
        uncertainty_readback_ref: &ArtifactRefV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn participant_reconciled(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        request: &VerifiedGenerationParticipantApplyRequestV1,
        dispatch_attempt_id: UuidV1,
        measured: &GenerationParticipantApplyReadbackV1,
        outcome: GenerationParticipantReconciliationOutcomeV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn freeze_ack_draft(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        request: &VerifiedGenerationParticipantApplyRequestV1,
        measured: &GenerationParticipantApplyReadbackV1,
        acknowledged_at_unix_ms: i64,
    ) -> Result<VerifiedFreezeGenerationAckDraftCommandV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn ack_object_bound(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        frozen: &VerifiedPersistedGenerationAckDraftV1,
        reloaded_ack_ref: &ArtifactRefV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn observed_commit(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        exact_ack_refs: &[ArtifactRefV1],
        observed_at_unix_ms: i64,
    ) -> Result<VerifiedObservedGenerationCommitV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn rollback_started(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        rollback_target: &GenerationParticipantRollbackTargetV1,
        rollback_execution_attempt_id: UuidV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn rollback_committed(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        rollback_target: &GenerationParticipantRollbackTargetV1,
        participant_rollback_readback_refs: &[ArtifactRefV1],
        committed_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;

    fn failed(
        &self,
        snapshot: &VerifiedGenerationActivationAttemptV1,
        error_code: GenerationActivationFailureCodeV1,
        failure_readback_ref: &ArtifactRefV1,
        recorded_at_unix_ms: i64,
    ) -> Result<VerifiedGenerationActivationTransitionV1,
                GenerationActivationAttemptStoreErrorV1>;
}

pub enum GenerationActivationAttemptStoreErrorV1 {
    NotFound,
    CreateConflict,
    IdentityMismatch,
    CasConflict,
    SequenceOrHashChainInvalid,
    TransitionInvalid,
    AckDraftConflict,
    AckObjectNotBound,
    AckSetMismatch,
    TerminalConflict,
    CodecOrIntegrity,
    StoreUnavailable,
}

#[async_trait::async_trait]
pub trait GenerationActivationAttemptStoreV1: Send + Sync {
    async fn begin_or_adopt(
        &self,
        command: &VerifiedBeginGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    async fn load_exact(
        &self,
        lookup: &GenerationActivationAttemptLookupV1,
    ) -> Result<VerifiedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    async fn append_cas(
        &self,
        expected: &GenerationActivationCasV1,
        transition: &VerifiedGenerationActivationTransitionV1,
    ) -> Result<VerifiedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    async fn freeze_ack_draft(
        &self,
        expected: &GenerationActivationCasV1,
        draft: &VerifiedFreezeGenerationAckDraftCommandV1,
    ) -> Result<VerifiedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;

    async fn commit_observed_with_exact_ack_set(
        &self,
        expected: &GenerationActivationCasV1,
        commit: &VerifiedObservedGenerationCommitV1,
    ) -> Result<VerifiedObservedGenerationActivationAttemptV1,
                GenerationActivationAttemptStoreErrorV1>;
}

pub const GENERATION_TRANSITION_LEASE_DURATION_MS: i64 = 300_000;

// Sole source of lease time and ownership for both transition and observed-
// selection stores. The core-server composition constructs its implementation
// only after G1-01 trusted-time verification and fixes one unpredictable ID for
// this authority boot. Store methods expose no caller time/owner parameters.
pub trait GenerationLeaseAuthorityV1: Send + Sync {
    fn boot_instance_id(&self) -> UuidV1;
    fn trusted_now_unix_ms(&self) -> Result<i64, GenerationLeaseAuthorityErrorV1>;
    fn new_unpredictable_uuid(&self) -> Result<UuidV1, GenerationLeaseAuthorityErrorV1>;
}

pub enum GenerationLeaseAuthorityErrorV1 {
    TrustedTimeUnavailable,
    BootInstanceUnavailable,
    EntropyUnavailable,
    CheckedTimeOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationTransitionPhaseV1 {
    Stable,
    Activating,
    RollingBack,
    Failed,
}

pub struct GenerationTransitionCasV1 {
    deployment_id: UuidV1,
    authority_epoch: u64,
    cas_version: u64,
    row_sha256: Sha256Digest,
}

// Exact lease tuple read from the same transition row/CAS as the attempt.
// Boot recovery carries this value unchanged so an expired takeover cannot
// CAS from a disposition label or a newly sampled approximation.
pub struct GenerationTransitionPersistedLeaseV1 {
    lease_id: UuidV1,
    lease_owner_instance_id: UuidV1,
    lease_version: u64,
    lease_expires_at_unix_ms: i64,
}

pub struct VerifiedGenerationTransitionSnapshotV1 {
    phase: GenerationTransitionPhaseV1,
    cas: GenerationTransitionCasV1,
    observed_generation_number: Option<GenerationNumberV1>,
    observed_generation_manifest_ref: Option<ArtifactRefV1>,
    observed_generation_digest_sha256: Option<Sha256Digest>,
    observed_activation_attempt_id: Option<UuidV1>,
    observed_terminal_record_sha256: Option<Sha256Digest>,
    desired_generation_number: Option<GenerationNumberV1>,
    desired_generation_manifest_ref: Option<ArtifactRefV1>,
    desired_generation_digest_sha256: Option<Sha256Digest>,
    desired_approval_registry_ref: Option<ArtifactRefV1>,
    desired_topology_declaration_ref: Option<ArtifactRefV1>,
    active_activation_attempt_id: Option<UuidV1>,
    rollback_execution_attempt_id: Option<UuidV1>,
    rollback_target: Option<GenerationParticipantRollbackTargetV1>,
    failure_readback_ref: Option<ArtifactRefV1>,
}

// Begin creates one nonnil unpredictable lease_id at version 1. Renewal keeps
// lease_id/owner and increments version exactly once. Expired takeover creates
// a new nonnil unpredictable lease_id/current-boot owner and increments the old
// version exactly once; every operation CASes the complete prior tuple.
pub struct VerifiedGenerationTransitionLeaseV1 {
    snapshot: VerifiedGenerationTransitionSnapshotV1,
    lease_id: UuidV1,
    lease_owner_instance_id: UuidV1,
    lease_version: u64,
    lease_expires_at_unix_ms: i64,
}

impl GenerationTransitionCasV1 {
    pub fn deployment_id(&self) -> UuidV1 { self.deployment_id }
    pub fn authority_epoch(&self) -> u64 { self.authority_epoch }
    pub fn cas_version(&self) -> u64 { self.cas_version }
    pub fn row_sha256(&self) -> Sha256Digest { self.row_sha256 }
}

impl GenerationTransitionPersistedLeaseV1 {
    pub fn lease_id(&self) -> UuidV1 { self.lease_id }
    pub fn lease_owner_instance_id(&self) -> UuidV1 { self.lease_owner_instance_id }
    pub fn lease_version(&self) -> u64 { self.lease_version }
    pub fn lease_expires_at_unix_ms(&self) -> i64 { self.lease_expires_at_unix_ms }
}

impl VerifiedGenerationTransitionSnapshotV1 {
    pub fn phase(&self) -> GenerationTransitionPhaseV1 { self.phase }
    pub fn cas(&self) -> &GenerationTransitionCasV1 { &self.cas }
    pub fn observed_generation_number(&self) -> Option<GenerationNumberV1> {
        self.observed_generation_number
    }
    pub fn observed_generation_manifest_ref(&self) -> Option<&ArtifactRefV1> {
        self.observed_generation_manifest_ref.as_ref()
    }
    pub fn observed_generation_digest_sha256(&self) -> Option<Sha256Digest> {
        self.observed_generation_digest_sha256
    }
    pub fn observed_activation_attempt_id(&self) -> Option<UuidV1> {
        self.observed_activation_attempt_id
    }
    pub fn observed_terminal_record_sha256(&self) -> Option<Sha256Digest> {
        self.observed_terminal_record_sha256
    }
    pub fn desired_generation_number(&self) -> Option<GenerationNumberV1> {
        self.desired_generation_number
    }
    pub fn desired_generation_manifest_ref(&self) -> Option<&ArtifactRefV1> {
        self.desired_generation_manifest_ref.as_ref()
    }
    pub fn desired_generation_digest_sha256(&self) -> Option<Sha256Digest> {
        self.desired_generation_digest_sha256
    }
    pub fn desired_approval_registry_ref(&self) -> Option<&ArtifactRefV1> {
        self.desired_approval_registry_ref.as_ref()
    }
    pub fn desired_topology_declaration_ref(&self) -> Option<&ArtifactRefV1> {
        self.desired_topology_declaration_ref.as_ref()
    }
    pub fn active_activation_attempt_id(&self) -> Option<UuidV1> {
        self.active_activation_attempt_id
    }
    pub fn rollback_execution_attempt_id(&self) -> Option<UuidV1> {
        self.rollback_execution_attempt_id
    }
    pub fn rollback_target(&self) -> Option<&GenerationParticipantRollbackTargetV1> {
        self.rollback_target.as_ref()
    }
    pub fn failure_readback_ref(&self) -> Option<&ArtifactRefV1> {
        self.failure_readback_ref.as_ref()
    }
}

impl VerifiedGenerationTransitionLeaseV1 {
    pub fn snapshot(&self) -> &VerifiedGenerationTransitionSnapshotV1 { &self.snapshot }
    pub fn lease_id(&self) -> UuidV1 { self.lease_id }
    pub fn lease_owner_instance_id(&self) -> UuidV1 { self.lease_owner_instance_id }
    pub fn lease_version(&self) -> u64 { self.lease_version }
    pub fn lease_expires_at_unix_ms(&self) -> i64 { self.lease_expires_at_unix_ms }
}

pub enum GenerationTransitionLeaseDispositionV1 {
    OwnedByThisInstance,
    HeldByAnotherInstance,
    ExpiredTakeoverPermitted,
}

pub enum VerifiedGenerationBootRecoveryV1 {
    Uninitialized {
        deployment_id: UuidV1,
        authority_epoch: u64,
    },
    Stable {
        transition: VerifiedGenerationTransitionSnapshotV1,
    },
    ResumeActivation {
        transition: VerifiedGenerationTransitionSnapshotV1,
        attempt: VerifiedGenerationActivationAttemptV1,
        persisted_lease: GenerationTransitionPersistedLeaseV1,
        lease_disposition: GenerationTransitionLeaseDispositionV1,
    },
    ResumeRollback {
        transition: VerifiedGenerationTransitionSnapshotV1,
        attempt: VerifiedGenerationActivationAttemptV1,
        persisted_lease: GenerationTransitionPersistedLeaseV1,
        lease_disposition: GenerationTransitionLeaseDispositionV1,
    },
    Failed {
        transition: VerifiedGenerationTransitionSnapshotV1,
        failure_readback_ref: ArtifactRefV1,
    },
}

pub enum GenerationTransitionStoreErrorV1 {
    NotFound,
    ManifestOrTripleMismatch,
    AttemptMismatch,
    ActiveTransitionConflict,
    LeaseHeld,
    LeaseExpired,
    LeaseCasConflict,
    ObservedCommitMismatch,
    RollbackTargetMismatch,
    RollbackCommitMismatch,
    PointerDrift,
    CodecOrIntegrity,
    StoreUnavailable,
}

// Non-wire, release-owned exact PostgreSQL projection. Reference byte columns
// remain raw here so the release codec—not db-pg—strict-parses them and proves
// byte equality to canonical ArtifactRefV1 JCS before constructing a wrapper.
pub struct GenerationTransitionDbRowV1 {
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub cas_version: u64,
    pub row_sha256: Sha256Digest,
    pub phase: GenerationTransitionPhaseV1,
    pub lease_id: Option<UuidV1>,
    pub lease_owner_instance_id: Option<UuidV1>,
    pub lease_version: Option<u64>,
    pub lease_expires_at_unix_ms: Option<i64>,
    pub desired_generation_number: Option<GenerationNumberV1>,
    pub desired_generation_manifest_ref_jcs: Option<Vec<u8>>,
    pub desired_generation_digest_sha256: Option<Sha256Digest>,
    pub desired_approval_registry_ref_jcs: Option<Vec<u8>>,
    pub desired_topology_declaration_ref_jcs: Option<Vec<u8>>,
    pub active_activation_attempt_id: Option<UuidV1>,
    pub observed_generation_number: Option<GenerationNumberV1>,
    pub observed_generation_manifest_ref_jcs: Option<Vec<u8>>,
    pub observed_generation_digest_sha256: Option<Sha256Digest>,
    pub observed_activation_attempt_id: Option<UuidV1>,
    pub observed_terminal_record_sha256: Option<Sha256Digest>,
    pub rollback_execution_attempt_id: Option<UuidV1>,
    pub rollback_target_jcs: Option<Vec<u8>>,
    pub failure_readback_ref_jcs: Option<Vec<u8>>,
    pub created_at_unix_ms: i64,
    pub updated_at_unix_ms: i64,
}

pub struct GenerationTransitionCodecV1;

impl GenerationTransitionCodecV1 {
    pub fn canonical_hash_projection_jcs(
        row: &GenerationTransitionDbRowV1,
    ) -> Result<Vec<u8>, GenerationTransitionStoreErrorV1>;

    pub fn verify_loaded(
        row: GenerationTransitionDbRowV1,
        exact_attempt: Option<&VerifiedGenerationActivationAttemptV1>,
    ) -> Result<VerifiedGenerationTransitionSnapshotV1,
                GenerationTransitionStoreErrorV1>;

    pub fn verify_loaded_lease(
        row: GenerationTransitionDbRowV1,
        exact_attempt: &VerifiedGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationTransitionLeaseV1,
                GenerationTransitionStoreErrorV1>;

    pub fn verify_loaded_boot_parts(
        row: GenerationTransitionDbRowV1,
        exact_attempt: &VerifiedGenerationActivationAttemptV1,
    ) -> Result<(VerifiedGenerationTransitionSnapshotV1,
                 GenerationTransitionPersistedLeaseV1),
                GenerationTransitionStoreErrorV1>;
}

#[async_trait::async_trait]
pub trait GenerationTransitionStoreV1: Send + Sync {
    async fn begin_or_adopt_activation(
        &self,
        begin: &VerifiedBeginGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationTransitionLeaseV1,
                GenerationTransitionStoreErrorV1>;

    async fn load_exact_for_boot(
        &self,
        deployment_id: UuidV1,
        authority_epoch: u64,
    ) -> Result<VerifiedGenerationBootRecoveryV1,
                GenerationTransitionStoreErrorV1>;

    async fn resume_or_take_over_exact(
        &self,
        recovery: &VerifiedGenerationBootRecoveryV1,
    ) -> Result<VerifiedGenerationTransitionLeaseV1,
                GenerationTransitionStoreErrorV1>;

    async fn renew_exact(
        &self,
        lease: &VerifiedGenerationTransitionLeaseV1,
    ) -> Result<VerifiedGenerationTransitionLeaseV1,
                GenerationTransitionStoreErrorV1>;

    async fn commit_observed_exact(
        &self,
        lease: &VerifiedGenerationTransitionLeaseV1,
        observed: &VerifiedObservedGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationTransitionSnapshotV1,
                GenerationTransitionStoreErrorV1>;

    async fn mark_rollback_started_exact(
        &self,
        lease: &VerifiedGenerationTransitionLeaseV1,
        attempt: &VerifiedGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationTransitionLeaseV1,
                GenerationTransitionStoreErrorV1>;

    async fn commit_rollback_exact(
        &self,
        lease: &VerifiedGenerationTransitionLeaseV1,
        attempt: &VerifiedGenerationActivationAttemptV1,
    ) -> Result<VerifiedGenerationTransitionSnapshotV1,
                GenerationTransitionStoreErrorV1>;

    async fn mark_failed_exact(
        &self,
        lease: &VerifiedGenerationTransitionLeaseV1,
        attempt: &VerifiedGenerationActivationAttemptV1,
        failure_readback_ref: &ArtifactRefV1,
    ) -> Result<VerifiedGenerationTransitionSnapshotV1,
                GenerationTransitionStoreErrorV1>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationObservedReleaseSelectionPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-OBSERVED-RELEASE-SELECTION-V1")]
    EpF57GenerationObservedReleaseSelectionV1,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationObservedReleaseSelectionRecordV1 {
    pub schema_version: u32,
    pub purpose: GenerationObservedReleaseSelectionPurposeV1,
    pub selection_id: UuidV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub observed_activation_attempt_id: UuidV1,
    pub observed_generation_manifest_ref: ArtifactRefV1,
    pub observed_generation_digest_sha256: Sha256Digest,
    pub observed_terminal_record_sha256: Sha256Digest,
    pub exact_ack_refs: Vec<ArtifactRefV1>,
    pub selected_at_unix_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationObservedReleaseSelectionStateV1 {
    Leased,
    BoundReleased,
    FailedReleased,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationObservedSelectionCheckpointBindingV1 {
    Preselection,
    SelectionBound,
}

// First-write command produced by the release-owned authority after the store
// has locked and strict-loaded the exact current OBSERVED attempt. db-pg may
// persist these bytes but cannot construct or modify the record/ref/checkpoint.
pub struct VerifiedBeginObservedReleaseSelectionV1 {
    record: GenerationObservedReleaseSelectionRecordV1,
    canonical_record_jcs_bytes: Vec<u8>,
    selection_ref: ArtifactRefV1,
    preselection_checkpoint_ref: ArtifactRefV1,
    preselection_checkpoint_sequence: u64,
    preselection_checkpoint_record_sha256: Sha256Digest,
}

impl VerifiedBeginObservedReleaseSelectionV1 {
    pub fn record(&self) -> &GenerationObservedReleaseSelectionRecordV1 { &self.record }
    pub fn canonical_record_jcs_bytes(&self) -> &[u8] { &self.canonical_record_jcs_bytes }
    pub fn selection_ref(&self) -> &ArtifactRefV1 { &self.selection_ref }
    pub fn preselection_checkpoint_ref(&self) -> &ArtifactRefV1 {
        &self.preselection_checkpoint_ref
    }
    pub fn preselection_checkpoint_sequence(&self) -> u64 {
        self.preselection_checkpoint_sequence
    }
    pub fn preselection_checkpoint_record_sha256(&self) -> Sha256Digest {
        self.preselection_checkpoint_record_sha256
    }
}

pub struct VerifiedObservedReleaseSelectionV1 {
    selection_id: UuidV1,
    candidate_run: CandidateRunIdentityV1,
    observed: VerifiedObservedGenerationActivationAttemptV1,
    record: GenerationObservedReleaseSelectionRecordV1,
    selection_ref: ArtifactRefV1,
    state: GenerationObservedReleaseSelectionStateV1,
    lease_id: UuidV1,
    lease_owner_instance_id: UuidV1,
    lease_version: u64,
    lease_expires_at_unix_ms: i64,
    last_candidate_checkpoint_ref: ArtifactRefV1,
    last_candidate_checkpoint_sequence: u64,
    last_candidate_checkpoint_record_sha256: Sha256Digest,
    checkpoint_binding: GenerationObservedSelectionCheckpointBindingV1,
    candidate_manifest_ref: Option<ArtifactRefV1>,
    resolution: Option<GenerationObservedReleaseSelectionResolutionV1>,
    resolution_ref: Option<ArtifactRefV1>,
}

impl VerifiedObservedReleaseSelectionV1 {
    pub fn selection_id(&self) -> UuidV1 { self.selection_id }
    pub fn candidate_run(&self) -> &CandidateRunIdentityV1 { &self.candidate_run }
    pub fn observed(&self) -> &VerifiedObservedGenerationActivationAttemptV1 { &self.observed }
    pub fn record(&self) -> &GenerationObservedReleaseSelectionRecordV1 { &self.record }
    pub fn selection_ref(&self) -> &ArtifactRefV1 { &self.selection_ref }
    pub fn state(&self) -> GenerationObservedReleaseSelectionStateV1 { self.state }
    pub fn lease_id(&self) -> UuidV1 { self.lease_id }
    pub fn lease_owner_instance_id(&self) -> UuidV1 { self.lease_owner_instance_id }
    pub fn lease_version(&self) -> u64 { self.lease_version }
    pub fn lease_expires_at_unix_ms(&self) -> i64 { self.lease_expires_at_unix_ms }
    pub fn last_candidate_checkpoint_ref(&self) -> &ArtifactRefV1 {
        &self.last_candidate_checkpoint_ref
    }
    pub fn last_candidate_checkpoint_sequence(&self) -> u64 {
        self.last_candidate_checkpoint_sequence
    }
    pub fn last_candidate_checkpoint_record_sha256(&self) -> Sha256Digest {
        self.last_candidate_checkpoint_record_sha256
    }
    pub fn checkpoint_binding(&self) -> GenerationObservedSelectionCheckpointBindingV1 {
        self.checkpoint_binding
    }
    pub fn candidate_manifest_ref(&self) -> Option<&ArtifactRefV1> {
        self.candidate_manifest_ref.as_ref()
    }
    pub fn resolution(&self) -> Option<&GenerationObservedReleaseSelectionResolutionV1> {
        self.resolution.as_ref()
    }
    pub fn resolution_ref(&self) -> Option<&ArtifactRefV1> {
        self.resolution_ref.as_ref()
    }
}

// Minted only by the release-owned verifier from this selection plus one G0
// authenticated journal prefix. It must be the same run and equal or extend the
// persisted checkpoint. Strict extensions before candidate finalization may
// remain PRESELECTION; the first candidate-finalization record must name this
// exact selection ref, changes binding to SELECTION_BOUND, and every later
// accepted prefix must retain that binding.
pub struct VerifiedGenerationObservedSelectionLeaseProgressV1 {
    selection_ref: ArtifactRefV1,
    candidate_run: CandidateRunIdentityV1,
    checkpoint_ref: ArtifactRefV1,
    checkpoint_last_sequence: u64,
    checkpoint_last_record_sha256: Sha256Digest,
    checkpoint_binding: GenerationObservedSelectionCheckpointBindingV1,
    candidate_manifest_ref_if_bound: Option<ArtifactRefV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum GenerationObservedReleaseSelectionResolutionPurposeV1 {
    #[serde(rename = "EP-F57-GENERATION-OBSERVED-RELEASE-SELECTION-RESOLUTION-V1")]
    EpF57GenerationObservedReleaseSelectionResolutionV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GenerationObservedReleaseSelectionResolutionReasonV1 {
    FinalizationInputConflict,
    FinalizationOutputConflict,
    UnrecoverableStorageFailure,
    OperatorCancelledAfterCrash,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GenerationObservedReleaseSelectionResolutionV1 {
    pub schema_version: u32,
    pub purpose: GenerationObservedReleaseSelectionResolutionPurposeV1,
    pub selection_ref: ArtifactRefV1,
    pub candidate_run: CandidateRunIdentityV1,
    pub observed_activation_attempt_id: UuidV1,
    pub observed_terminal_record_sha256: Sha256Digest,
    pub failed_freeze_checkpoint_ref: ArtifactRefV1,
    pub reason: GenerationObservedReleaseSelectionResolutionReasonV1,
    pub initiator_principal: PrincipalRefV1,
    pub approver_principal: PrincipalRefV1,
    pub authorization_decision_refs: Vec<ArtifactRefV1>,
    pub resolved_at_unix_ms: i64,
}

// Live dual-control command. The public plain resolution wire cannot construct
// this nominal type; it carries the exact authorized bytes/ref and the minimum
// validity deadline of the two verified human contexts.
pub struct AuthorizedGenerationObservedSelectionResolutionV1 {
    value: GenerationObservedReleaseSelectionResolutionV1,
    canonical_resolution_jcs_bytes: Vec<u8>,
    resolution_ref: ArtifactRefV1,
    authorization_not_after_unix_ms: i64,
}

pub struct VerifiedPersistedGenerationObservedSelectionResolutionV1 {
    value: GenerationObservedReleaseSelectionResolutionV1,
    canonical_resolution_jcs_bytes: Vec<u8>,
    resolution_ref: ArtifactRefV1,
    authorization_not_after_unix_ms: i64,
}

impl VerifiedGenerationObservedSelectionLeaseProgressV1 {
    pub fn selection_ref(&self) -> &ArtifactRefV1 { &self.selection_ref }
    pub fn candidate_run(&self) -> &CandidateRunIdentityV1 { &self.candidate_run }
    pub fn checkpoint_ref(&self) -> &ArtifactRefV1 { &self.checkpoint_ref }
    pub fn checkpoint_last_sequence(&self) -> u64 { self.checkpoint_last_sequence }
    pub fn checkpoint_last_record_sha256(&self) -> Sha256Digest {
        self.checkpoint_last_record_sha256
    }
    pub fn checkpoint_binding(&self) -> GenerationObservedSelectionCheckpointBindingV1 {
        self.checkpoint_binding
    }
    pub fn candidate_manifest_ref_if_bound(&self) -> Option<&ArtifactRefV1> {
        self.candidate_manifest_ref_if_bound.as_ref()
    }
}

impl VerifiedPersistedGenerationObservedSelectionResolutionV1 {
    pub fn value(&self) -> &GenerationObservedReleaseSelectionResolutionV1 { &self.value }
    pub fn canonical_resolution_jcs_bytes(&self) -> &[u8] {
        &self.canonical_resolution_jcs_bytes
    }
    pub fn resolution_ref(&self) -> &ArtifactRefV1 { &self.resolution_ref }
    pub fn authorization_not_after_unix_ms(&self) -> i64 {
        self.authorization_not_after_unix_ms
    }
}

impl AuthorizedGenerationObservedSelectionResolutionV1 {
    pub fn value(&self) -> &GenerationObservedReleaseSelectionResolutionV1 { &self.value }
    pub fn canonical_resolution_jcs_bytes(&self) -> &[u8] {
        &self.canonical_resolution_jcs_bytes
    }
    pub fn resolution_ref(&self) -> &ArtifactRefV1 { &self.resolution_ref }
}

pub struct GenerationObservedReleaseSelectionCommandAuthorityV1 {
    lease_authority: std::sync::Arc<dyn GenerationLeaseAuthorityV1>,
}

impl GenerationObservedReleaseSelectionCommandAuthorityV1 {
    pub fn new(
        lease_authority: std::sync::Arc<dyn GenerationLeaseAuthorityV1>,
    ) -> Self;

    pub fn begin_selection(
        &self,
        candidate_run: &CandidateRunIdentityV1,
        deployment_id: UuidV1,
        authority_epoch: u64,
        observed: &VerifiedObservedGenerationActivationAttemptV1,
        preselection_prefix: &VerifiedGateRunJournalPrefixV1,
    ) -> Result<VerifiedBeginObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    pub fn verify_progress(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        prefix: &VerifiedGateRunJournalPrefixV1,
    ) -> Result<VerifiedGenerationObservedSelectionLeaseProgressV1,
                GenerationObservedReleaseSelectionErrorV1>;

    pub fn authorize_failed_resolution(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        prefix: &VerifiedGateRunJournalPrefixV1,
        reason: GenerationObservedReleaseSelectionResolutionReasonV1,
        initiator: &VerifiedSecurityContextV1,
        approver: &VerifiedSecurityContextV1,
    ) -> Result<AuthorizedGenerationObservedSelectionResolutionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    pub fn verify_persisted_resolution(
        &self,
        authorized: AuthorizedGenerationObservedSelectionResolutionV1,
        reloaded_exact_jcs_bytes: &[u8],
        resolution_ref: &ArtifactRefV1,
    ) -> Result<VerifiedPersistedGenerationObservedSelectionResolutionV1,
                GenerationObservedReleaseSelectionErrorV1>;
}

pub enum GenerationObservedReleaseSelectionErrorV1 {
    NoObservedGeneration,
    CandidateRunConflict,
    SelectionConflict,
    LeaseConflict,
    LeaseExpired,
    ProgressCheckpointMismatch,
    ObservedPointerChanged,
    CandidateBindingConflict,
    ResolutionNotAuthorized,
    ResolutionEvidenceConflict,
    Integrity,
    StoreUnavailable,
}

// Non-wire, release-owned exact PostgreSQL projection. All *_jcs columns are
// passed as stored bytes and are strict-parsed/recanonicalized by the codec.
pub struct GenerationObservedReleaseSelectionDbRowV1 {
    pub selection_id: UuidV1,
    pub candidate_run_identity_jcs: Vec<u8>,
    pub candidate_run_identity_sha256: Sha256Digest,
    pub deployment_id: UuidV1,
    pub authority_epoch: u64,
    pub observed_activation_attempt_id: UuidV1,
    pub observed_terminal_record_sha256: Sha256Digest,
    pub observed_exact_ack_set_sha256: Sha256Digest,
    pub selection_object_jcs: Vec<u8>,
    pub selection_ref_jcs: Vec<u8>,
    pub lease_id: UuidV1,
    pub lease_owner_instance_id: UuidV1,
    pub lease_version: u64,
    pub lease_expires_at_unix_ms: i64,
    pub last_candidate_checkpoint_ref_jcs: Vec<u8>,
    pub last_candidate_checkpoint_sequence: u64,
    pub last_candidate_checkpoint_record_sha256: Sha256Digest,
    pub last_candidate_checkpoint_binds_selection: bool,
    pub candidate_manifest_ref_jcs: Option<Vec<u8>>,
    pub resolution_evidence_jcs: Option<Vec<u8>>,
    pub resolution_evidence_ref_jcs: Option<Vec<u8>>,
    pub resolution_reason: Option<GenerationObservedReleaseSelectionResolutionReasonV1>,
    pub state: GenerationObservedReleaseSelectionStateV1,
    pub created_at_unix_ms: i64,
    pub updated_at_unix_ms: i64,
}

pub struct GenerationObservedReleaseSelectionCodecV1;

impl GenerationObservedReleaseSelectionCodecV1 {
    pub fn verify_loaded(
        row: GenerationObservedReleaseSelectionDbRowV1,
        exact_observed: VerifiedObservedGenerationActivationAttemptV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;
}

#[async_trait::async_trait]
pub trait GenerationObservedReleaseSelectionPortV1: Send + Sync {
    async fn begin_or_adopt_current_observed(
        &self,
        candidate_run: &CandidateRunIdentityV1,
        deployment_id: UuidV1,
        authority_epoch: u64,
        preselection_prefix: &VerifiedGateRunJournalPrefixV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    async fn load_exact_for_recovery(
        &self,
        same_run_header: &VerifiedBusinessArtifactV1<GateRunJournalHeaderPayloadV1>,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    async fn renew_exact(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        progress: &VerifiedGenerationObservedSelectionLeaseProgressV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    async fn resume_expired_same_run(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        progress: &VerifiedGenerationObservedSelectionLeaseProgressV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    async fn bind_candidate_and_release(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        progress: &VerifiedGenerationObservedSelectionLeaseProgressV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;

    async fn resolve_failed_and_release(
        &self,
        selection: &VerifiedObservedReleaseSelectionV1,
        progress: &VerifiedGenerationObservedSelectionLeaseProgressV1,
        resolution: &VerifiedPersistedGenerationObservedSelectionResolutionV1,
    ) -> Result<VerifiedObservedReleaseSelectionV1,
                GenerationObservedReleaseSelectionErrorV1>;
}
```

`GenerationActivationAttemptEventV1` is the strict tagged enum shown above. Every record carries the exact common eight-field `binding`; its event contains only the displayed variant fields: `ATTEMPT_STARTED {required_participant_ids,started_at_unix_ms}`; `PARTICIPANT_DISPATCH_STARTED {participant_id,dispatch_attempt_id,required_item_set_sha256}`; `PARTICIPANT_DISPATCH_UNKNOWN {participant_id,dispatch_attempt_id,uncertainty_readback_ref}`; `PARTICIPANT_RECONCILED {participant_id,dispatch_attempt_id,measured_readback_ref,reconciliation_outcome}`; `ACK_DRAFT_FROZEN {participant_id,ack_jcs_bytes,ack_ref,participant_definition_sha256,applied_item_set_sha256,participant_apply_readback_ref,acknowledged_at_unix_ms}`; `ACK_OBJECT_BOUND {participant_id,ack_ref}`; `OBSERVED_COMMITTED {exact_ack_refs,observed_at_unix_ms}`; `ROLLBACK_STARTED {rollback_target,rollback_execution_attempt_id}`; `ROLLBACK_COMMITTED {rollback_target,participant_rollback_readback_refs,committed_at_unix_ms}`; and `FAILED {error_code,failure_readback_ref}`. Participant, ACK and rollback-readback vectors are canonical-sorted and unique. Both rollback events repeat one byte-identical tagged target: `PRIOR_OBSERVED_GENERATION` exact-binds the predecessor manifest ref/digest, while `NO_OBSERVED_GENERATION` is legal only for generation 1 when the prior OBSERVED tuple is wholly null. The strict codec requires the sole initial record to have sequence `0` and every later record to be its exact contiguous successor; it rejects initial `1`, a repeated/gapped later sequence, unknown tag/field, non-monotonic CAS, binding change, wrong previous-record digest, invalid time order, an ACK draft whose bytes do not exact-parse to its ref/bindings, a missing/mismatched typed apply readback, ACK_OBJECT_BOUND without the same frozen draft, OBSERVED without exactly one bound ACK per required participant, rollback-target drift, or rollback commit without exactly one successful typed rollback readback per rollback participant. Byte goldens make initial `0` PASS and initial `1` FAIL.

Only `GenerationActivationAttemptCommandAuthorityV1` has public methods that mint first-write opaque commands, and each method requires the relevant verified manifest plus approval/declaration refs, loaded snapshot/CAS, participant request/readback or store-proven persisted ACK draft. The upper `ep-platform-generation-activation` coordinator first verifies the runtime declaration, then passes only `VerifiedRuntimeTopologyDeclarationV1::artifact_ref()` into `begin`; this release-owned activation-attempt module never imports the runtime wrapper. `begin` constructs both the header and the sole canonical initial record: `sequence=0`, `expected_prior_cas_version=0`, `committed_cas_version=1`, `previous_record_sha256=null`, event `ATTEMPT_STARTED` with the same required-participant vector/start time, and `recorded_at_unix_ms == header.started_at_unix_ms`. Production code never calls the lower attempt `begin_or_adopt` independently: `GenerationTransitionStoreV1::begin_or_adopt_activation` is the sole coordinator entry and, in one PostgreSQL serialized transaction, CASes the deployment/epoch transition row, writes the desired pointer/lease, create-new writes the attempt header plus sequence-0 record, codec-reloads both, and returns one private transition lease. A desired pointer without its attempt, an attempt without its pointer, adapter-chosen time, second initial event, or partial commit is impossible. There is no public raw event constructor. `GenerationActivationAttemptLookupV1::for_release_verification` is crate-visible only and derives the full eight-field binding from the domain-verified manifest/registry/declaration plus the unique attempt ID proved by the candidate's exact ACK set; Task 14 cannot query by generation number alone. The PostgreSQL adapter stores exact header/record JCS bytes and indexed repeats of the binding/CAS under one serialized transaction, then round-trips them through the release codec before returning a verified snapshot. G1-05 adds the sole direct `ep-platform-release -> ep-platform-gate-journal-contract` edge only for its domain-neutral verified-prefix accessor; the foundation-only gate contract has no reverse edge and selection code cannot import a P340/release payload. Final workspace metadata also allows only the later Task-14 `ep-platform-release -> ep-platform-runtime` edge needed by runtime-topology certification; it forbids `runtime -> release`, either lower crate importing the upper activation crate, and cycles.

The live ACK-freeze command and persisted ACK recovery proof are deliberately different nominal types. `freeze_ack_draft` on the command authority alone creates `VerifiedFreezeGenerationAckDraftCommandV1` from the authenticated request, fresh measured readback and one trusted time; only the store's `freeze_ack_draft` accepts it. After that store call commits, `VerifiedGenerationActivationAttemptV1::persisted_frozen_ack_draft` may expose only `VerifiedPersistedGenerationAckDraftV1`; that type does not implement `VerifiedGenerationActivationStoreCommandViewV1`, cannot be passed back to `freeze_ack_draft`, and is accepted only by `ack_object_bound`. Before appending `ACK_OBJECT_BOUND`, the command authority and store both exact-match the currently loaded persisted snapshot/CAS, participant, complete frozen record bytes, ACK bytes/ref and reloaded object ref. A codec-valid caller byte sequence that is ahead of the database cannot mint a first-write command or pass the store CAS. Goldens cover forged-ahead-of-store bytes, crash immediately before/after frozen-record commit, object ingest, object-bound append and response return.

`GenerationTransitionStoreV1` is the exact current-pointer, owner-death and boot-recovery boundary that the one-argument `activate(VerifiedGenerationManifestV1)` entry consumes. It is keyed only by exact `(deployment_id,authority_epoch)`; `load_exact_for_boot` never scans or selects a latest row. Core-server constructs the PostgreSQL implementation once with the G1-01-backed `GenerationLeaseAuthorityV1`; no public store method accepts a time, owner, duration or lease ID. On every begin, renew or expired takeover, the store samples that injected source exactly once, fixes its private boot ID as owner, and sets `lease_expires_at_unix_ms=checked(trusted_now_unix_ms+GENERATION_TRANSITION_LEASE_DURATION_MS)`; overflow or untrusted time fails before a write. `begin_or_adopt_activation` accepts only the command-authority-produced begin value, exact-matches the persisted G1-01 manifest/approval/declaration triple and either creates one new unpredictable attempt or adopts the byte-identical active attempt. `load_exact_for_boot` reads the transition row and its exact attempt, when present, in one repeatable transaction. Stable/failed rows go through `GenerationTransitionCodecV1::verify_loaded`; active/rollback rows go through `verify_loaded_boot_parts`, the sole constructor of their snapshot plus complete persisted lease tuple, after which the store reports the disposition derived from its internally sampled current boot/time. The disposition is not CAS evidence: `resume_or_take_over_exact` must exact-match the recovery value's `(deployment_id,authority_epoch,cas_version,row_sha256,lease_id,lease_owner_instance_id,lease_version,lease_expires_at_unix_ms)` plus the attempt CAS. It returns the same owned live lease without rewriting, rejects another boot's live lease, or takes an expired lease only at `trusted_now_unix_ms >= persisted_lease.lease_expires_at_unix_ms` by allocating a new unpredictable lease ID/current-boot owner, incrementing the persisted version exactly once and committing one new row CAS. An absent transition produces `Uninitialized` only when an exact anti-join proves there is no activation-attempt row for that deployment/epoch; an orphan attempt or desired pointer is `CodecOrIntegrity`, never stable. Takeover never allocates an attempt, changes a desired pointer, reruns a known-applied participant, or trusts a disposition/expiry sampled by the caller. `commit_observed_exact` atomically exact-joins the terminal observed wrapper, moves the authoritative observed pointer, clears desired/active/lease fields and returns `STABLE`; rollback and failed transitions use their displayed typed methods under the same CAS. Thus boot recovery from `ATTEMPT_STARTED|PARTICIPANT_DISPATCH_UNKNOWN|ACK_DRAFT_FROZEN|ACK_OBJECT_BOUND` has one route and never needs a caller attempt ID, newest-row query or directory/database scan.

Pre-candidate freeze does not use that lookup and does not scan ACKs to discover an attempt. `GenerationObservedReleaseSelectionPortV1` is the one explicit bridge: under the same serialized generation-transition lock it exact-loads the authoritative OBSERVED pointer, its unique terminal activation record and bound ACK exact-set, persists one immutable selection object keyed by the already fixed `CandidateRunIdentityV1`, and returns the private verified selection. Core-server wires the port and `GenerationObservedReleaseSelectionCommandAuthorityV1` with the same `Arc<dyn GenerationLeaseAuthorityV1>`; no second constructor, ambient clock or caller UUID/time source is registered. While holding the G0 journal writer's exclusive guard, candidate freeze first creates/fsyncs/reloads one checkpoint of the exact current prefix and passes that authenticated value as `preselection_prefix`; it retains or recovery-reacquires that guard through selection insertion, every pre-finalization journal extension and the first selection-bound append, releasing it only after that append is fsynced or on process death. A previously valid but non-head checkpoint, a prefix captured without that guard or a branch fails before the selection transaction. The release-owned `begin_selection` builder alone derives the strict record/JCS/ref plus preselection tuple from the locked current OBSERVED wrapper, that prefix, and an internally sampled selection ID/time; db-pg only persists the verified command. The insert transaction stores its checkpoint ref/sequence/terminal hash with binding `PRESELECTION` beside the selection and lease. This makes every cut after selection insert but before candidate finalization recoverable without guessing a new ref. Re-entry with the same run adopts only the byte-identical selection and an equal-or-extending prefix; another run/generation cannot replace it. The store uses the same injected lease authority and exact checked 300000-millisecond duration, and blocks a new desired/observed transition until `bind_candidate_and_release` atomically consumes a journal-verified exact candidate binding or an explicitly authorized failed freeze reaches `FAILED_RELEASED`; timeout alone never silently releases it. `GenerationObservedReleaseSelectionCommandAuthorityV1::verify_progress` consumes only G0's authenticated generic prefix, proves the same candidate run and an equal-or-strict hash-chain extension of the stored checkpoint, derives the exact checkpoint/sequence/hash and optional bound candidate ref, and rejects rollback, a branch or another selection. While binding is `PRESELECTION`, an equal prefix is legal for idempotent live renewal, same-run crash resume or failed-resolution proof, and a strict extension containing no candidate-finalization record remains `PRESELECTION`; `renew_exact` persists either legal form under the live exact lease so long-running copy, schema and Fresh-PG work can renew without inventing a new journal event. The first `CANDIDATE_MANIFEST_FINALIZATION_STARTED` whose `candidate_kind=FINAL_RELEASE` in the accepted prefix is the sole binding point: it must carry this exact nonnull `generation_observed_selection_ref`, changes the binding monotonically to `SELECTION_BOUND`, and any missing/wrong ref or earlier conflicting finalization fails. Every later accepted prefix must retain that exact record and ref. `candidate_manifest_ref_if_bound` remains null until the matching durable `CANDIDATE_MANIFEST_BOUND` exact-binds its finalization ID and candidate ref. `bind_candidate_and_release` accepts only that verified progress value—never a raw ref—requires `SELECTION_BOUND` plus nonnull candidate ref, CAS-persists its exact checkpoint triple/candidate ref and returns a codec-reloaded `BOUND_RELEASED` wrapper. No CLI accepts `activation_attempt_id`, no `latest` or directory/database scan exists, and all post-candidate topology/release verification goes back to candidate-derived `GenerationActivationAttemptLookupV1::for_release_verification` plus `load_exact`.

An expired `LEASED` row has exactly two explicit routes. Recovery first obtains the old private wrapper only through `load_exact_for_recovery(&VerifiedBusinessArtifactV1<GateRunJournalHeaderPayloadV1>)`: the port derives the candidate-run digest from that exact signed header and performs one equality lookup on the unique run key, then strict-reloads the stored selection object/ref and observed closure. It accepts no caller run fields, selection ref or checkpoint and has no latest/by-time/by-generation overload. The returned wrapper exposes its exact last persisted checkpoint ref, which G0's read port uses to load the authenticated prefix; this breaks the preselection-crash lookup cycle without scanning. The route works both before the first selection-bound checkpoint and after later freeze progress. `verify_progress` then proves the supplied prefix equals that stored last checkpoint or is its valid strict extension; `resume_expired_same_run` CASes only that same selection/run and complete lease tuple after rechecking the immutable selection object/ref, unchanged authoritative OBSERVED pointer/terminal hash/exact ACK set and monotonic checkpoint binding. It allocates only a new lease ID/version/expiry from the internal lease authority; it cannot change the selection, observed generation or candidate ref, and it remains valid when exact candidate bytes were create-new written but `CANDIDATE_MANIFEST_BOUND` or the final selector CAS was interrupted. If those invariants cannot be recovered, `authorize_failed_resolution` requires two distinct currently authorized human `VerifiedSecurityContextV1` principals, exact initiate/approve capabilities, same deployment/legal-entity scope, current MFA/reauthentication and SoD. It samples `resolved_at_unix_ms` only from the injected trusted authority, freezes the displayed twelve-field resolution plus its exact JCS/ref, and returns the non-wire `AuthorizedGenerationObservedSelectionResolutionV1` whose private deadline is the checked minimum validity bound of both contexts. The release authority create-new stores/fsyncs/reloads those exact bytes through `EvidenceObjectStoreV1`; `verify_persisted_resolution` consumes that live authorized nominal, exact-parses the reloaded bytes, recomputes the typed ref and alone mints the persisted wrapper carrying those same bytes and accepted by `resolve_failed_and_release`. That CAS writes the wrapper's exact bytes/ref and returns a codec-reloaded `FAILED_RELEASED` wrapper exposing the immutable resolution/ref, so response-loss re-entry can return the already committed terminal result. Raw wire/JCS/ref, a reconstructed authorization command or a persisted object without the live nominal cannot mint the wrapper. A crash after object creation but before the selection CAS leaves an unreachable immutable object and requires a fresh two-human authorization; recovery never scans for or adopts that orphan. When binding is still `PRESELECTION`, `failed_freeze_checkpoint_ref` is exactly the current last authenticated checkpoint persisted in the row, whether it is the initial checkpoint or a later pre-finalization extension; when binding is `SELECTION_BOUND`, it is the last verified selection-bound checkpoint. The resolution ref/reason/principals/decision refs are immutable history. A single operator, service principal, same principal twice, raw resolution ref, unpersisted bytes, pointer/checkpoint drift, candidate already bound, another run, second resolution or silent expiry release fails; no row is deleted or reassigned.

The exact graph capabilities for that resolution are `f57.generation.observed-selection.failed-release.initiate` and `f57.generation.observed-selection.failed-release.approve`; G1-05 adds both reviewed non-client administrative nodes to `docs/capability-graph/f57-core.v1.json`. The initiator context must carry a current ALLOW decision for the former action and the approver context a current ALLOW decision for the latter action at the exact selection deployment/legal-entity scope; the two `PrincipalRefV1` values and two decision refs must differ, both contexts must prove human kind, current MFA plus reauthentication, and the policy's separation-of-duty result. The canonical decision-ref vector is sorted/unique by `(uri,sha256,size_bytes,media_type)`. `resolve_failed_and_release` samples trusted time once inside its CAS transaction, requires `state=LEASED && now>=lease_expires_at_unix_ms` and `resolved_at_unix_ms<=now<=authorization_not_after_unix_ms`, exact-matches the persisted resolution/checkpoint/selection and complete expired lease tuple, and rejects a still-live lease or expired dual-control proof. It never requires a physically impossible post-fsync millisecond equality.

`crates/platform/release/src/activation_attempt.rs` and `docs/evidence/f57-generation-observed-release-selection.v1.schema.json` are the sole Rust/schema owners of the selection record. The schema imports only `f57-foundation.v1.schema.json`, binds media `application/vnd.ep.f57-generation-observed-release-selection-v1+json`, and requires exactly the twelve fields displayed above with purpose `EP-F57-GENERATION-OBSERVED-RELEASE-SELECTION-V1`; it has no signature or generic envelope. The content-addressed `selection_ref` uses only that media and exact JCS bytes. The same Rust module plus `docs/evidence/f57-generation-observed-release-selection-resolution.v1.schema.json` solely own the separate strict plain twelve-field resolution root under purpose/media `EP-F57-GENERATION-OBSERVED-RELEASE-SELECTION-RESOLUTION-V1` / `application/vnd.ep.f57-generation-observed-release-selection-resolution-v1+json`; that schema also imports only foundation and adds no signer-registry row. Its two authorization-decision refs are canonical sorted/unique and correspond exactly to the displayed distinct principals. Contract/golden tests reject missing/extra fields, a signed wrapper, wrong media/purpose/run/attempt/manifest/digest/ACK/checkpoint/principal/decision/time, a nonclosed reason or a second schema owner.

The same `generation_store.rs` and migration own the concrete PostgreSQL implementation. `platform_meta.generation_transitions` has exact durable columns `{deployment_id uuid not null,authority_epoch bigint not null,cas_version bigint not null,row_sha256 bytea not null,phase text not null,lease_id uuid null,lease_owner_instance_id uuid null,lease_version bigint null,lease_expires_at_unix_ms bigint null,desired_generation_number bigint null,desired_generation_manifest_ref_jcs bytea null,desired_generation_digest_sha256 bytea null,desired_approval_registry_ref_jcs bytea null,desired_topology_declaration_ref_jcs bytea null,active_activation_attempt_id uuid null,observed_generation_number bigint null,observed_generation_manifest_ref_jcs bytea null,observed_generation_digest_sha256 bytea null,observed_activation_attempt_id uuid null,observed_terminal_record_sha256 bytea null,rollback_execution_attempt_id uuid null,rollback_target_jcs bytea null,failure_readback_ref_jcs bytea null,created_at_unix_ms bigint not null,updated_at_unix_ms bigint not null,primary key(deployment_id,authority_epoch)}`. `phase` is exactly `STABLE|ACTIVATING|ROLLING_BACK|FAILED`; `cas_version>=1`; all four lease columns are jointly null or nonnull; ACTIVATING requires the complete desired quintuple plus active attempt and live lease while rollback ID/target are null; ROLLING_BACK requires active attempt, rollback ID, strict tagged rollback target and lease; STABLE clears desired/active/rollback ID/rollback target/lease/failure; FAILED requires the exact failure ref and clears the lease. The observed five fields are jointly null only before generation 1 and otherwise jointly nonnull. `PRIOR_OBSERVED_GENERATION` must exact-match those observed manifest/digest fields; `NO_OBSERVED_GENERATION` requires all observed fields null and desired generation number `1`. Attempt IDs are foreign keys. `row_sha256` is exactly `SHA256(GenerationTransitionCodecV1::canonical_hash_projection_jcs(&row))`. That function emits one strict JCS object with every displayed SQL column except `row_sha256`; scalar/nullable values retain their displayed column names, each `*_ref_jcs` key loses only the `_jcs` suffix and contains the strict-parsed typed `ArtifactRefV1` JSON value or null, and `rollback_target_jcs` becomes strict tagged `rollback_target` or null. It first requires every nonnull stored ref/tagged byte string to equal its canonical JCS. The release codec exact-matches the active/terminal attempt binding, rollback target and phase constraints and alone constructs the private snapshot; `generation-transition-row-hash-v1.jcs.json` freezes the projection bytes and digest. All transition/attempt/pointer changes above use one serializable transaction and CAS exact `(deployment_id,authority_epoch,cas_version,row_sha256)`.

`platform_meta.generation_observed_release_selections` has exact durable columns `{selection_id uuid primary key,candidate_run_identity_jcs bytea not null unique,candidate_run_identity_sha256 bytea not null unique,deployment_id uuid not null,authority_epoch bigint not null,observed_activation_attempt_id uuid not null,observed_terminal_record_sha256 bytea not null,observed_exact_ack_set_sha256 bytea not null,selection_object_jcs bytea not null,selection_ref_jcs bytea not null unique,lease_id uuid not null,lease_owner_instance_id uuid not null,lease_version bigint not null,lease_expires_at_unix_ms bigint not null,last_candidate_checkpoint_ref_jcs bytea not null,last_candidate_checkpoint_sequence bigint not null,last_candidate_checkpoint_record_sha256 bytea not null,last_candidate_checkpoint_binds_selection boolean not null,candidate_manifest_ref_jcs bytea null,resolution_evidence_jcs bytea null,resolution_evidence_ref_jcs bytea null,resolution_reason text null,state text not null,created_at_unix_ms bigint not null,updated_at_unix_ms bigint not null}`. `state` is checked to `LEASED|BOUND_RELEASED|FAILED_RELEASED`; lease IDs/owners are nonnil, `lease_version>=1`, begin and each expired resume create a new unpredictable `lease_id` owned by the injected current boot, and renew preserves ID/owner while incrementing version and checked-extending expiry. Checkpoint binding begins false, may remain false across equal or strict authenticated pre-finalization extensions, changes false-to-true only when the first `CANDIDATE_MANIFEST_FINALIZATION_STARTED` for `FINAL_RELEASE` exact-names the selection ref, and never changes back. LEASED requires null candidate/resolution fields; BOUND_RELEASED requires one candidate ref obtained only from the matching journal-bound progress proof, true checkpoint binding and null resolution fields; FAILED_RELEASED requires null candidate ref plus all three resolution fields. The attempt ID is a foreign key to the unique terminal OBSERVED attempt, and checked digests are recomputed from exact JCS on every load. `begin_or_adopt_current_observed` serializes on the deployment/epoch transition row and atomically inserts selection, lease and the authenticated preselection checkpoint triple. `load_exact_for_recovery` derives the unique candidate-run digest only from its verified signed header and performs one equality lookup; it never accepts a selection ref/checkpoint or scans an index. `renew_exact` CASes `(selection_id,lease_id,lease_owner_instance_id,lease_version,state=LEASED,not-expired)`, requires current-boot ownership, and persists only monotonic authenticated progress: false-to-false before candidate finalization, false-to-true at the exact selection-bearing finalization record, or true-to-true afterward. `resume_expired_same_run` CASes the same immutable row and complete old lease tuple only after expiry and all observed/progress checks, including equal or strict legal preselection progress; `bind_candidate_and_release` accepts the verified progress proof, CASes its exact checkpoint triple and proof-derived candidate ref against the current live tuple, requires true selection binding plus the matching `CANDIDATE_MANIFEST_BOUND`, and returns a codec-reloaded terminal wrapper. `resolve_failed_and_release` requires the exact expired tuple and dual-control resolution. Those are the only release transitions. `GenerationObservedReleaseSelectionCodecV1` strict-parses and exact-recanonicalizes every stored JCS field, recomputes run/selection/ACK digests, joins the exact terminal OBSERVED wrapper and enforces these state constraints before constructing the private wrapper. There is no delete, latest, expiry sweeper, cross-run lease steal, mutable observed pointer inside a selection, or adapter-created verified wrapper.

`crates/platform/release/src/activation_attempt.rs` solely owns the durable activation-attempt, transition and observed-selection contracts, strict codecs and ports; `db-pg` only implements persistence. The activation event set is `ATTEMPT_STARTED|PARTICIPANT_DISPATCH_STARTED|PARTICIPANT_DISPATCH_UNKNOWN|PARTICIPANT_RECONCILED|ACK_DRAFT_FROZEN|ACK_OBJECT_BOUND|OBSERVED_COMMITTED|ROLLBACK_STARTED|ROLLBACK_COMMITTED|FAILED`. Every event repeats one attempt/generation/declaration identity and contiguous sequence/CAS version. Before ACK freeze, the server create-new stores and typed-reloads the strict apply readback. `ACK_DRAFT_FROZEN` atomically persists the complete canonical fourteen-field ACK bytes, deterministic content-addressed ref, exact `participant_apply_readback_ref`, participant/definition/item-set bindings and one trusted `acknowledged_at_unix_ms`; `ACK_OBJECT_BOUND` follows only after create-new ingest/fsync/reload exact-matches that frozen ref. Rollback commit analogously requires the create-new/reloaded exact rollback-readback set. The lower attempt port exposes only `begin_or_adopt`, `load_exact`, `append_cas`, first-write `freeze_ack_draft`, and `commit_observed_with_exact_ack_set`; production begin is reachable only through the composite transition store. Neither port has delete, overwrite, scan-latest, reset-unknown or caller event construction. The SQL migration constrains one active attempt per transition, one frozen ACK draft and one bound ACK per required participant, one terminal OBSERVED/rollback outcome, one selection per candidate run, monotonic checkpoint progress and one immutable release or resolution terminal.

The migration's `platform_meta.generation_activation_attempt_records` projection stores the canonical `event_jcs` as authority and adds only three strict indexed repeats: `participant_apply_readback_ref_jcs bytea null`, `rollback_target_jcs bytea null`, and `participant_rollback_readback_refs_jcs bytea null`. The first is nonnull only for `ACK_DRAFT_FROZEN` and exact-matches both the event and the fourteen-field ACK; `rollback_target_jcs` is nonnull only for `ROLLBACK_STARTED|ROLLBACK_COMMITTED` and exact-recanonicalizes the tagged target; the rollback-ref vector is nonnull only for `ROLLBACK_COMMITTED`, canonical sorted/unique and exact-matches that event. Every nonnull ref is strict-parsed and byte-equal to canonical `ArtifactRefV1` JCS before the release codec constructs a wrapper. In the same serialized CAS, the retention projection records immutable reachability `ACK ref -> apply-readback ref -> each nonnull package generation-transition ref` and `rollback terminal -> rollback-readback refs -> each restored-item transition/readback ref`; these are graph edges extracted only from typed-reloaded objects, not caller SQL. Response loss adopts the same edges, and missing/extra/cross-attempt/cross-item refs prevent OBSERVED or rollback commit.

Do not provide raw graph, topology, readback, payload, ref-only or generic-proof overloads. The coordinator lives only in upper `ep-platform-generation-activation`; at G1 it points one way to release/runtime/capability-graph/foundation. Task 14 may add the explicit one-way upper -> package/backup dependencies solely for graph-reserved maintenance after their lower contracts exist; package/backup/release/runtime never import the upper crate and no cycle is legal. Release owns generation wires/state/attempt/participant ports; runtime owns topology/readback; Task 14 separately adds release-to-runtime certification. The public activation input remains private `VerifiedGenerationManifestV1`. Under one uninterrupted transition lease the coordinator reloads the persisted generation/approval/declaration triple, registry, graph and projections; reruns the sole compiler; and exact-compares graph, items and participant derivation. Only this reconstruction creates `VerifiedGenerationBoundCapabilityGraphV1`; package maintenance reservation is derived from that same private value and current OBSERVED predecessor, never from a caller graph/slot/ref. The sole generation digest remains SHA-256 of exact signed-envelope JCS, with positive contiguous numbering and every typed artifact/reverse plan rechecked.

`crates/platform/release/src/participant.rs` additionally owns both stable non-evidence participant boundaries and both strict readback roots. Forward owns private-field `GenerationParticipantApplyRequestV1`, `VerifiedGenerationParticipantApplyRequestV1`, verifier and `GenerationParticipantApplyPortV1`; its request exact-binds `{deployment_id,authority_epoch,generation_number,activation_attempt_id,generation_manifest_ref,generation_digest_sha256,topology_declaration_ref,participant_id,participant_definition_sha256,required_items,required_item_set_sha256}`. Its exact sixteen-field `GenerationParticipantApplyReadbackV1` repeats those identities, has canonical `applied_items`, item-set digest, typed `readiness_refs`, closed `SUCCEEDED|FAILED|UNKNOWN` outcome and one observed time. Apply accepts only the tagged `DESIRED_ITEM` row. That variant has exactly `{item_id,generation_item_ref,generation_transition_ref,installed_state_readback_ref}`; `generation_transition_ref` is nonnull exactly for `CAPABILITY_PACKAGE` and null for every other generation item kind. At G1 the ref is constrained to exact future media `application/vnd.ep.f57-capability-package-generation-transition-v1+json` but is not semantically opened; Task 7's lower package owner later supplies the sole schema/verifier, so release has no package dependency. This makes the immutable offline path exact: fourteen-field ACK → `participant_apply_readback_ref` → sixteen-field apply readback → package `generation_transition_ref`. The ACK or SQL may not carry a shortcut transition ref, and no directory/database scan may discover one.

Rollback separately owns private-field request/wrapper/port. Its request binds the same identity plus unpredictable `rollback_execution_attempt_id`, one exact `GenerationParticipantRollbackTargetV1`, and canonical reverse subset `{item_id,reverse_plan_ref,source_item_ref,target_item_ref}`. The exact fourteen-field rollback readback repeats those bindings, canonical `restored_items`, set digest, readiness refs, outcome and observed time. A real predecessor uses tagged `PRIOR_OBSERVED_GENERATION {predecessor_generation_manifest_ref,predecessor_generation_digest_sha256}` plus `DESIRED_ITEM` rows. Rolling back generation 1 uses tagged `NO_OBSERVED_GENERATION` and may use `DEACTIVATED_RETAIN_DATA {item_id,rolled_back_generation_item_ref,generation_transition_ref,installed_state_readback_ref}` for each newly introduced package item; that row exact-binds the forward transition and an `ABSENT` installed-state readback carrying the retained-data/absence proof, never a fictitious predecessor. Apply rejects `DEACTIVATED_RETAIN_DATA`; rollback accepts it only for a new item whose reverse plan is `DEACTIVATE_RETAIN_DATA`. Canonical item-set digests include the variant tag and every displayed field.

Exact readback media are `application/vnd.ep.f57-generation-participant-apply-readback-v1+json` and `application/vnd.ep.f57-generation-participant-rollback-readback-v1+json`, with displayed `EP-F57-*` purposes; the ACK purpose/media are exactly `EP-F57-GENERATION-PARTICIPANT-ACK-V1` / `application/vnd.ep.f57-generation-participant-ack-v1+json`. All three are strict plain JCS roots, never CMS envelopes. The participant returns exact bytes only; the server verifies request equality, create-new stores/reloads the readback and only then may build the exact fourteen-field ACK or commit rollback. The forward request is minted only from a domain-verified manifest plus durable `ATTEMPT_STARTED`; rollback only after durable `ROLLBACK_STARTED`. Authenticated Service-SID IPC alone constructs either verified request. A participant cannot return an ACK/OBSERVED verdict, choose a rollback target or create a rollback ID. The generic coordinator remains independent of implementations; Task 14's upper package-maintenance orchestration may add acyclic upper → package/backup/tenancy edges while both lower boundaries remain release-owned.

Under that same uninterrupted lease, the coordinator loads the recorded declaration ref from `EvidenceObjectStoreV1` and invokes `TopologyVerifier::verify_declaration(&ArtifactRefV1, &[u8], &RuntimeTopologyReadbackCollectorV1)`. Runtime owns and seals the injected collector; it alone captures the fresh declaration-shaped OS/service/IPC/storage/database readback and constructs private `ActualRuntimeReadbackV1`. The verifier alone constructs private `VerifiedRuntimeTopologyDeclarationV1`, checks media/size/digest/content-addressed URI, strict JCS, purpose/profile and complete live equality, then exact-matches the declaration/ref to the newly reconstructed graph, exact generation envelope, storage-manifest ref and P340 policy-definition ref before desired moves. Neither wrapper nor readback crosses the public boundary, and no lock is released before transition commit. This is pre-release activation only; production remains impossible until G6 terminal P340 evidence, matching topology certification, release certificate and a new exact live readback. A stale pre-lock proof, caller readback/wrapper/ref/bytes, certification, retired `RuntimeTopologyManifestV1`, or signed-wrapper substitute is structurally unaccepted.

The coordinator durably creates one unpredictable `activation_attempt_id` and desired pointer before forward dispatch. It addresses only exact declaration endpoints and asks each required participant to apply its canonical item subset; participants return measured state, never an ACK. The private forward request exposes each exact item and its already domain-verified reverse plan only so the participant can validate/freeze recovery prerequisites; possession of that handle never authorizes reverse effects. On failure/UNKNOWN, the participant returns measurement and stops. The coordinator alone calls `mark_rollback_started_exact`, durably fixing the exact tagged rollback target plus unpredictable `rollback_execution_attempt_id`, then mints and dispatches the private rollback request; a package/participant must bind that ID and target before any reverse intent and query/adopt the same ID after response loss. `PRIOR_OBSERVED_GENERATION` requires fresh predecessor readback; `NO_OBSERVED_GENERATION` requires fresh absence/retained-data readback. Only the matching successful set permits `ROLLBACK_COMMITTED`. On forward success, the server alone freezes/ingests the strict fourteen-field ACK and may commit OBSERVED only with the exact complete set. Recovery never starts another forward/rollback attempt, reruns a known operation, resamples ACK time or lets a participant synthesize authority.

The OBSERVED pointer moves in the same serialized durable commit only after an exact-set join proves one and only one ACK for every canonical required participant and no extra row, all with the same attempt/generation/declaration/definition/item-set values. Missing, duplicate, stale, cross-attempt, cross-generation, participant-supplied or otherwise mismatched ACKs fail closed. The migration enforces those uniqueness/FK/check constraints; no process-memory count or timeout can stand in for the exact join. Rollback uses only the signed predecessor manifest and its already verified reverse-plan refs and records a new serialized attempt; it never recompiles a best-effort reverse operation.

Desired and observed pointers, exact required participant ACKs, activation attempts, rollback window, worker drain state, leases, and persistent references are durable. A lease timeout alone never releases artifacts still referenced by a workflow, effect, reconciliation, backup, audit, legal hold, or rollback window. `generation_faults.rs` implements only `GOV-003`.

G0 already owns the exact four retention wires in `pin.rs`; this task may only add repository-facing behavior and must not change their serde/schema bytes. `artifact_retention_store.rs` is the sole PostgreSQL owner for lease and persistent-reference tables. It implements serialized `acquire|renew|release`, evaluates ACTIVE lease liveness against one injected trusted `now`, preserves expired ACTIVE rows as history, and performs aggregate `can_reclaim(artifact,now)` only after locking all lease/reference/generation/rollback/observed-selection reachability rows for that artifact. Reclaim is true exactly when every lease is RELEASED or expired, every persistent reference has a nonnull release time, no signed current/rollback generation reaches the digest, and no immutable selection row reaches it through `selection_ref_jcs`, the selection's observed manifest/ACK refs, `candidate_manifest_ref_jcs` or `resolution_evidence_ref_jcs`. Selection/resolution history is append-only and has no deletion/release transition, so those exact refs remain permanent audit reachability without inventing another pin wire. Release is idempotent CAS; renewal preserves identity/acquisition fields, strictly extends within the 300000-ms bound and rejects expired/released/stale writers. No deletion, timeout rewrite to RELEASED, single-lease helper or process-memory cache may authorize collection. Migration checks use the exact `ACTIVE|RELEASED` and seven kind wires; tests cover boundary `now==expires`, crash before/after every commit, concurrent renew/release/reclaim, stale writer, legal hold, UNKNOWN effect/reconciliation, backup/audit, rollback and all three selection-state reachability paths.

- [ ] **Step 4: Verify Fresh PostgreSQL and GREEN**

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090400`

Run: `cargo test -p ep-platform-release -p ep-platform-runtime -p ep-platform-generation-activation -p ep-adapter-db-pg -p core-server -p job-worker -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g1_generation_activation -- --nocapture`

Expected: PASS for exact `13/9/14` generation-manifest/reverse-plan/ACK wires, strict sixteen-field apply and fourteen-field rollback readbacks, tagged `DESIRED_ITEM|DEACTIVATED_RETAIN_DATA` item rows, tagged `PRIOR_OBSERVED_GENERATION|NO_OBSERVED_GENERATION` rollback targets, generation-1 rollback with no fabricated predecessor, ACK → apply-readback → package-transition reachability, durable SQL/readback/transition reference edges, lost ACK, partial participant activation, process crash, generation rollback, exact boot recovery through the same transition/attempt at every nonterminal/terminal-before-pointer cut, owner-death lease takeover only at/after expiry with complete persisted-lease CAS and no caller clock/owner input, exact generation-bound plain CapabilityGraph object/compiled-graph equality, exact verified declaration/live-readback matching, release-owned request minting from manifest + loaded attempt + participant ID, authenticated request/item-set exactness, participant measured-readback/UNKNOWN reconciliation, server-only ACK canonicalization plus authority-object-store ingest/reload and relocatable ref persistence, compile-time separation of first-write ACK commands from persisted ACK recovery proofs, structural inability for a participant to return/create an ACK or commit OBSERVED, same-run expired selection recovery across initial/equal/strict preselection progress and after the first selection-bound checkpoint, proof-only candidate bind/release, dual-human/capability immutable `FAILED_RELEASED` with no raw-wire proof upgrade, rejection of a generic signed graph proof, caller compiled graph or raw/old/signed/certification topology substitute, declaration-only production-admission rejection, pin acquire/renew/release boundaries, all transition/attempt/selection/retention tables, stale/crash concurrency, observed-selection permanent-reachability and aggregate safe-collection negatives, declared direct Cargo edges and an acyclic `generation-activation -> {release,runtime,capability-graph}` boundary with one lower `release -> gate-journal-contract` generic-prefix edge and no edge to a concrete participant crate.

- [ ] **Step 5: Commit only this task**

Commit: `feat(release): coordinate signed generations and artifact pins`

### Task G1-06: Upgrade `ep-platform-flow` into the durable closed-loop engine and capacity governor

**Files**

- Create: `crates/platform/flow/src/objective.rs`
- Create: `crates/platform/flow/src/obligation.rs`
- Create: `crates/platform/flow/src/effect.rs`
- Create: `crates/platform/flow/src/evidence.rs`
- Create: `crates/platform/flow/src/cycle.rs`
- Create: `crates/platform/flow/src/reconcile.rs`
- Create: `crates/platform/flow/src/human_decision.rs`
- Create: `crates/platform/flow/src/assignment.rs`
- Create: `crates/platform/flow/src/engine.rs`
- Modify: `crates/platform/flow/src/lib.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/objective_store.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/effect_store.rs`
- Create: `crates/adapter/db-pg/src/platform_flow/reconcile_store.rs`
- Create: `crates/platform/runtime/src/capacity/mod.rs`
- Create: `crates/platform/runtime/src/capacity/certificate.rs`
- Create: `crates/platform/runtime/src/capacity/governor.rs`
- Create: `crates/platform/runtime/src/capacity/permit.rs`
- Create: `crates/adapter/db-pg/src/platform_ops/capacity.rs`
- Modify: `crates/adapter/db-pg/src/platform_ops/mod.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `crates/platform/runtime/src/lib.rs`
- Create: `apps/job-worker/src/wiring/flow.rs`
- Create: `apps/job-worker/src/wiring/capacity.rs`
- Modify: `apps/job-worker/src/wiring/mod.rs`
- Create: `db/migrations/platform_flow/V20261025090500__platform_flow_create_objectives_effects_and_checkpoints.sql`
- Create: `db/migrations/platform_ops/V20261025090600__platform_ops_create_capacity_evidence.sql`
- Create: `testkit/tests/f57_g1_durable_flow.rs`
- Create: `testkit/src/f57_cases/g1/automation_fault_matrix.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate: `docs/generated/f57/requirement-test-facades.v1.json`
- Regenerate: `testkit/src/f57_cases/generated_bindings.rs`

- [ ] **Step 1: Write failing durable-flow and capacity tests**

```rust
#[tokio::test]
async fn response_loss_is_unknown_and_never_blindly_retried() {
    dispatch_then_drop_response().await;
    assert_eq!(effect_state().await, EffectStateV1::Unknown);
    assert_eq!(objective_state().await, ObjectiveStateV1::Reconciling);
    assert_eq!(provider_dispatch_count().await, 1);
}

#[tokio::test]
async fn unavailable_governor_rejects_heavy_work_but_never_throttles_wal() {
    stop_governor().await;
    assert_denied(WorkClassV1::HeavyReport).await;
    assert_admitted(WorkClassV1::PostgresWal).await;
}
```

- [ ] **Step 2: Run RED**

Run: `cargo test -p ep-testkit --test f57_g1_durable_flow -- --nocapture`

Expected: FAIL because the durable objective/effect stores and capacity permit contract are absent.

- [ ] **Step 3: Implement persisted loops and strict effect semantics**

Each Objective persists its kind, cycle, subject, state, obligations, closure predicate version, pinned generation/workflow/package versions, assignments, deadlines, checkpoints, evidence, and incident links. A worker restart resumes from a committed checkpoint.

`DISPATCHED → UNKNOWN` is mandatory after ambiguous delivery. Reconciliation observations cannot directly rewrite business state; they invoke the owning typed command. Opposing later evidence produces `CONFLICTED` and an incident. Closure review creates a work item and requires a distinct currently-authorized reviewer with SoD and reauthentication.

Capacity permits are signed, expiring, and measured by work class, bytes, concurrency, and HDD pressure. The governor fails closed for heavy background work. It may never suspend or throttle PostgreSQL/WAL, audit, interactive saves, emergency recovery, or the reserved Control Center session. `automation_fault_matrix.rs` implements exactly `AUT-001..007`.

- [ ] **Step 4: Verify Fresh PostgreSQL and GREEN**

Run: `cargo xtask f57 fresh-pg --profile G1_AUTHORITY_SPINE --through 20261025090600`

Run: `cargo test -p ep-platform-flow -p ep-platform-runtime -p ep-adapter-db-pg -p job-worker -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g1_durable_flow -- --nocapture`

Expected: PASS for crash/restart, duplicate worker, timeout-after-success, independent receipt reconciliation, opposing evidence, reviewer SoD, generation pins, and governor failure.

- [ ] **Step 5: Commit only this task**

Commit: `feat(flow): add durable objectives reconciliation and capacity governance`

### Task G1-07: Issue `G1_AUTHORITY_SPINE_GREEN`

**Files**

- Create: `testkit/tests/f57_g1_authority_spine.rs`
- Create: `testkit/src/f57_cases/probes/g1_authority_spine.rs`
- Create: `testkit/tests/f57_slice_probes_g1_authority_spine.rs`
- Modify: `xtask/src/f57/gate.rs`

- [ ] **Step 1: Write a failing aggregate receipt test**

The test rejects a receipt if any prerequisite receipt, migration, graph digest, generator version, security negative, Fresh PostgreSQL run, or repository-tree binding differs. It also exact-compares the 19 G1 first-due RequirementIDs and their concrete handlers: 7 authz + 7 automation + 3 authority-command + 1 generation + 1 transactional-evidence; duplicate, missing, extra, skipped, or `NOT_DELIVERED` fails.

- [ ] **Step 2: Run RED**

Run: `cargo test -p ep-testkit --test f57_g1_authority_spine -- --nocapture`

- [ ] **Step 3: Implement the gate without duplicating verdict logic**

The gate runs current-candidate G0 and G1 conformance, then aggregates only the receipts produced by that run. It cannot turn a skipped, ignored, unavailable, stale, synthetic-hardware, or different-tree result into success, and it cannot re-sign the original G0 receipt.

- [ ] **Step 4: Run pre-commit G1 verification**

Run: `cargo xtask f57 graph generate --check`

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Expected: all pre-commit commands exit 0; no gate receipt is issued from a dirty worktree.

- [ ] **Step 5: Commit the gate registration**

Commit: `test(f57): certify the G1 authority spine`

- [ ] **Step 6: Issue and verify G1 only from clean committed HEAD**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 gate g1 --bundle-root target/f57/evidence --run-journal target/f57/evidence/g1/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g1`

Run: `cargo xtask f57 evidence verify --receipt target/f57/evidence/g1/authority-spine-receipt.v1.json --bundle-root target/f57/evidence --expect-gate G1_AUTHORITY_SPINE_GREEN`

Expected: all commands exit 0; the fresh G0/G1 receipt set binds committed `HEAD`, the unchanged baseline/apply-manifest digests, and exact migration counts `baseline=69,f57=9,total=78` through `20261025090600`. The G1 receipt explicitly states that clients, L2, P340, backup, recovery, and production readiness are unproven. No repository file changes after issuance.

### Task G2-01: Govern HDD-backed file intake, quarantine, and clean evidence

**Files**

- Create: `crates/platform/file/src/object.rs`
- Create: `crates/platform/file/src/quarantine.rs`
- Create: `crates/platform/file/src/intake.rs`
- Create: `crates/platform/file/src/evidence.rs`
- Modify: `crates/platform/file/src/lib.rs`
- Modify: `crates/platform/file/src/scan.rs`
- Modify: `crates/platform/file/src/upload.rs`
- Create: `crates/adapter/file/src/hdd_store.rs`
- Create: `crates/adapter/file/src/windows_defender.rs`
- Modify: `crates/adapter/file/src/lib.rs`
- Create: `crates/adapter/db-pg/src/platform_file/mod.rs`
- Create: `crates/adapter/db-pg/src/platform_file/file_store.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Create: `crates/platform/file/src/public/mod.rs`
- Generate: `crates/platform/file/src/public/generated.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/platform_file/V20261025090700__platform_file_create_objects_quarantine_and_scan_evidence.sql`
- Create: `testkit/tests/f57_g2_governed_file.rs`
- Modify: `crates/platform/file/Cargo.toml`
- Modify: `crates/adapter/file/Cargo.toml`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write failing quarantine and TOCTOU tests**

Run: `cargo test -p ep-testkit --test f57_g2_governed_file -- --nocapture`

Expected: FAIL because there is no governed publishable file version.

- [ ] **Step 2: Implement intake as a state machine**

Bytes land on the approved HDD quarantine volume. Publishability requires a stable final-handle volume identity, exact content digest, fresh approved scanner definition, clean outcome, and a second digest/handle check after scanning. `UNKNOWN`, `SKIPPED`, timeout, stale definitions, archive bombs, macro/polyglot ambiguity, and any path replacement remain quarantined. A business object links an immutable clean file version, never a mutable path.

- [ ] **Step 3: Verify Fresh PostgreSQL and GREEN**

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025090700`

Run: `cargo test -p ep-platform-file -p ep-adapter-file -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_governed_file -- --nocapture`

- [ ] **Step 4: Commit**

Commit: `feat(file): govern HDD quarantine and clean evidence`

### Task G2-02: Add customer-master and contracting feature owners

**Files**

- Create: `crates/features/customer-master/Cargo.toml`
- Create: `crates/features/customer-master/src/lib.rs`
- Create: `crates/features/customer-master/src/public/mod.rs`
- Generate: `crates/features/customer-master/src/public/generated.rs`
- Create: `crates/features/customer-master/src/public/spi.rs`
- Create: `crates/features/customer-master/src/domain/customer.rs`
- Create: `crates/features/customer-master/src/application/commands.rs`
- Create: `crates/features/customer-master/tests/customer.rs`
- Create: `crates/features/contracting/Cargo.toml`
- Create: `crates/features/contracting/src/lib.rs`
- Create: `crates/features/contracting/src/public/mod.rs`
- Generate: `crates/features/contracting/src/public/generated.rs`
- Create: `crates/features/contracting/src/public/spi.rs`
- Create: `crates/features/contracting/src/domain/contract.rs`
- Create: `crates/features/contracting/src/application/commands.rs`
- Create: `crates/features/contracting/tests/contract.rs`
- Create: `crates/adapter/db-pg/src/mdm/mod.rs`
- Create: `crates/adapter/db-pg/src/mdm/customer_repository.rs`
- Create: `crates/adapter/db-pg/src/clm/mod.rs`
- Create: `crates/adapter/db-pg/src/clm/contract_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/mdm/V20261025090800__mdm_create_customer_master.sql`
- Create: `db/migrations/clm/V20261025090900__clm_create_contracting.sql`
- Create: `testkit/tests/f57_g2_customer_contract.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write failing state, ownership, and activation tests**

The public graph-generated contract contains `CreateCustomerV1 → CustomerCreatedFactV1`, contract draft/version, payment milestone, clean attachment link, submit, approve, signature, activation, and `ContractBecameEffectiveV1`.

Run: `cargo test -p ep-testkit --test f57_g2_customer_contract -- --nocapture`

Expected: FAIL because the two feature owners and tables do not exist.

- [ ] **Step 2: Implement feature-first boundaries**

Only `public` is exported. `domain` and `application` remain private; repository ports are in `public::spi`, SQL stays in db-pg, and neither feature reads the other's tables. Contract activation requires its approved immutable version, signature evidence, payment schedule, and clean attachment policy. Every same-entity reference is composite and CAS protected.

- [ ] **Step 3: Regenerate contracts and verify GREEN**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025090900`

Run: `cargo test -p ep-feature-customer-master -p ep-feature-contracting -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_customer_contract -- --nocapture`

Run: `cargo xtask archcheck`

- [ ] **Step 4: Commit**

Commit: `feat(ctc): persist customer and contract authority`

### Task G2-03: Add the STANDARD contract-source sales-order owner

**Files**

- Create: `crates/features/sales-order/Cargo.toml`
- Create: `crates/features/sales-order/src/lib.rs`
- Create: `crates/features/sales-order/src/public/mod.rs`
- Generate: `crates/features/sales-order/src/public/generated.rs`
- Create: `crates/features/sales-order/src/public/spi.rs`
- Create: `crates/features/sales-order/src/domain/order.rs`
- Create: `crates/features/sales-order/src/application/commands.rs`
- Create: `crates/features/sales-order/tests/order.rs`
- Create: `crates/adapter/db-pg/src/sales/mod.rs`
- Create: `crates/adapter/db-pg/src/sales/order_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/sales/V20261025091000__sales_create_standard_orders.sql`
- Create: `testkit/tests/f57_g2_standard_sales.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write RED for exact-one release semantics**

Run: `cargo test -p ep-testkit --test f57_g2_standard_sales -- --nocapture`

The failing test requires only `sales_type=STANDARD` and `source=CONTRACT_VERSION`, one canonical `SalesOrderReleasedV1` fact, one `SALES_ORDER_FULFILMENT` Objective cycle, replay stability, quantity/money conservation, and same-entity provenance.

- [ ] **Step 2: Implement, regenerate, and verify GREEN**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025091000`

Run: `cargo test -p ep-feature-sales-order -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_standard_sales -- --nocapture`

- [ ] **Step 3: Commit**

Commit: `feat(ctc): persist standard contract sales orders`

### Task G2-04: Add sales-driven procurement and honest settlement-gap evidence

**Files**

- Create: `crates/features/procurement/Cargo.toml`
- Create: `crates/features/procurement/src/lib.rs`
- Create: `crates/features/procurement/src/public/mod.rs`
- Generate: `crates/features/procurement/src/public/generated.rs`
- Create: `crates/features/procurement/src/public/spi.rs`
- Create: `crates/features/procurement/src/domain/procurement.rs`
- Create: `crates/features/procurement/src/application/commands.rs`
- Create: `crates/features/procurement/tests/procurement.rs`
- Create: `crates/adapter/db-pg/src/procure/mod.rs`
- Create: `crates/adapter/db-pg/src/procure/procurement_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/procure/V20261025091100__procure_create_sales_order_demand_and_purchase_orders.sql`
- Create: `testkit/tests/f57_g2_procurement.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write RED for demand, purchase-order, and UNKNOWN behavior**

Run: `cargo test -p ep-testkit --test f57_g2_procurement -- --nocapture`

The test covers `CreateDemandFromSalesOrderV1`, PO create/approve/issue, response loss after issue, zero blind retry, and the typed gap below:

```rust
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcurementSettlementGapV1 {
    pub purchase_invoice_recorded: bool,
    pub payable_recognized: bool,
    pub supplier_payment_settled: bool,
}
```

This type is owned only by `procurement::public::generated`. The generic flow engine persists its graph-resolved schema reference and canonical JSON; it does not introduce a global hard-coded `PURCHASE_AP_CLOSED` enum.

- [ ] **Step 2: Implement, regenerate, and verify GREEN**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025091100`

Run: `cargo test -p ep-feature-procurement -p ep-platform-flow -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_procurement -- --nocapture`

- [ ] **Step 3: Commit**

Commit: `feat(ctc): persist sales-driven procurement`

### Task G2-05: Add inventory receipt and customer-delivery evidence

**Files**

- Create: `crates/features/inventory-fulfilment/Cargo.toml`
- Create: `crates/features/inventory-fulfilment/src/lib.rs`
- Create: `crates/features/inventory-fulfilment/src/public/mod.rs`
- Generate: `crates/features/inventory-fulfilment/src/public/generated.rs`
- Create: `crates/features/inventory-fulfilment/src/public/spi.rs`
- Create: `crates/features/inventory-fulfilment/src/domain/fulfilment.rs`
- Create: `crates/features/inventory-fulfilment/src/application/commands.rs`
- Create: `crates/features/inventory-fulfilment/tests/fulfilment.rs`
- Create: `crates/adapter/db-pg/src/inventory/mod.rs`
- Create: `crates/adapter/db-pg/src/inventory/fulfilment_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/inventory/V20261025091200__inventory_create_receipt_and_delivery_facts.sql`
- Create: `testkit/tests/f57_g2_inventory_fulfilment.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write RED for quantity conservation and legal-entity isolation**

Run: `cargo test -p ep-testkit --test f57_g2_inventory_fulfilment -- --nocapture`

Require `RecordGoodsReceiptV1 → GoodsReceiptRecordedV1` and `RecordDeliveryEvidenceV1 → CustomerDeliveryAcceptedV1`; partial receipts/deliveries cannot exceed released/received quantities, and no cross-entity reference is enumerable.

- [ ] **Step 2: Implement, regenerate, and verify GREEN**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025091200`

Run: `cargo test -p ep-feature-inventory-fulfilment -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_inventory_fulfilment -- --nocapture`

- [ ] **Step 3: Commit**

Commit: `feat(ctc): persist receipt and delivery evidence`

### Task G2-06: Keep sales invoicing and receivable/cash as two independent owners

**Files**

- Create: `crates/features/sales-invoicing/Cargo.toml`
- Create: `crates/features/sales-invoicing/src/lib.rs`
- Create: `crates/features/sales-invoicing/src/public/mod.rs`
- Generate: `crates/features/sales-invoicing/src/public/generated.rs`
- Create: `crates/features/sales-invoicing/src/public/spi.rs`
- Create: `crates/features/sales-invoicing/src/domain/invoice.rs`
- Create: `crates/features/sales-invoicing/src/application/commands.rs`
- Create: `crates/features/sales-invoicing/tests/invoice.rs`
- Create: `crates/features/receivable-cash/Cargo.toml`
- Create: `crates/features/receivable-cash/src/lib.rs`
- Create: `crates/features/receivable-cash/src/public/mod.rs`
- Generate: `crates/features/receivable-cash/src/public/generated.rs`
- Create: `crates/features/receivable-cash/src/public/spi.rs`
- Create: `crates/features/receivable-cash/src/domain/receivable.rs`
- Create: `crates/features/receivable-cash/src/application/commands.rs`
- Create: `crates/features/receivable-cash/tests/receivable.rs`
- Create: `crates/adapter/db-pg/src/invoice/mod.rs`
- Create: `crates/adapter/db-pg/src/invoice/sales_invoice_repository.rs`
- Create: `crates/adapter/db-pg/src/finance/mod.rs`
- Create: `crates/adapter/db-pg/src/finance/receivable_cash_repository.rs`
- Modify: `crates/adapter/db-pg/src/lib.rs`
- Modify: `docs/capability-graph/f57-core.v1.json`
- Regenerate (`GENERATED_MANIFEST_SET`): `docs/generated/f57/projection-manifest.v1.json`
- Create: `db/migrations/invoice/V20261025091300__invoice_create_sales_invoicing.sql`
- Create: `db/migrations/finance/V20261025091310__finance_create_receivable_and_cash.sql`
- Create: `testkit/tests/f57_g2_invoice_cash.rs`
- Modify: `crates/adapter/db-pg/Cargo.toml`
- Modify: `testkit/Cargo.toml`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Write RED for ownership, allocation, reversal, and closure**

Run: `cargo test -p ep-testkit --test f57_g2_invoice_cash -- --nocapture`

Invoice owns `IssueSalesInvoiceV1 → SalesInvoiceIssuedV1`. Finance owns `RecognizeReceivableFromSalesInvoiceV1 → ReceivableBecameEffectiveV1`, cash receipt, allocation, and `CashReceiptAllocatedV1`. Exact allocation coverage closes the receivable; over-allocation fails; reversal appends a fact and opens a new Objective cycle.

`billing-cash` is only a product-navigation label. It is not a crate, schema, semantic owner, database writer, or graph owner.

- [ ] **Step 2: Implement, regenerate, and verify GREEN**

Run: `cargo xtask f57 graph generate`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025091310`

Run: `cargo test -p ep-feature-sales-invoicing -p ep-feature-receivable-cash -p ep-adapter-db-pg -p ep-testkit --all-targets --locked`

Run: `cargo test -p ep-testkit --test f57_g2_invoice_cash -- --nocapture`

Run: `cargo xtask archcheck`

- [ ] **Step 3: Commit**

Commit: `feat(ctc): persist invoicing receivables and cash`

### Task G2-07: Prove server-side CTC-01 integration without falsely claiming the client slice

**Files**

- Create: `apps/core-server/src/business/mod.rs`
- Create: `apps/core-server/src/business/ctc01.rs`
- Create: `apps/core-server/src/wiring/features.rs`
- Modify: `apps/core-server/src/wiring/mod.rs`
- Modify: `apps/core-server/src/wiring/command.rs`
- Modify: `apps/core-server/src/main.rs`
- Modify: `apps/core-server/Cargo.toml`
- Create: `testkit/fixtures/ctc01/authority-standard-cycle.v1.json`
- Create: `testkit/tests/f57_g2_ctc01_authority_integration.rs`
- Create: `testkit/src/f57_cases/probes/g2_ctc_data.rs`
- Create: `testkit/tests/f57_slice_probes_g2_ctc_data.rs`
- Modify: `xtask/src/f57/gate.rs`

- [ ] **Step 1: Write the failing full authority/domain test**

The test uses only `CommandPipeline`, generated public commands, real PostgreSQL 16, and committed facts. It must assert:

```text
CONTRACT_FULFILMENT       = CLOSED
SALES_ORDER_FULFILMENT    = CLOSED
RECEIVABLE_COLLECTION     = CLOSED
PROCUREMENT_FULFILMENT    = WAITING
```

The procurement Objective has exactly one blocking obligation, `PURCHASE_AP_CLOSED`, and exact evidence:

```rust
ProcurementSettlementGapV1 {
    purchase_invoice_recorded: false,
    payable_recognized: false,
    supplier_payment_settled: false,
}
```

The database contains no committed PurchaseInvoice, AP-recognition, or SupplierPayment fact. A manual close attempt returns closure-predicate-not-met; replay returns the same receipts/Objectives.

- [ ] **Step 2: Run RED**

Run: `cargo test -p ep-testkit --test f57_g2_ctc01_authority_integration -- --nocapture`

Expected: FAIL until every G2 owner is wired through the G1 command/flow contracts.

- [ ] **Step 3: Wire only generated public contracts**

The composition root may depend on each feature's `public` surface but never its private modules or tables. This task adds no HTTP route, UI, external provider, Windows Workbench, L2 test, purchase-invoice/AP/supplier-payment implementation, or production claim.

- [ ] **Step 4: Run pre-commit G2 verification**

Run: `cargo xtask f57 graph generate --check`

Run: `cargo xtask f57 fresh-pg --profile G2_CTC_DATA --through 20261025091310`

Run: `cargo test -p ep-testkit --test f57_g2_ctc01_authority_integration -- --nocapture`

Run: `cargo xtask archcheck`

Run: `cargo xtask sqlcheck`

Run: `cargo xtask f57 verify --level l1 --changed-from HEAD^`

Expected: every pre-commit command exits 0; no receipt is issued from the dirty integration worktree.

- [ ] **Step 5: Commit the integration gate**

Commit: `test(ctc): certify G2 authority data integration`

- [ ] **Step 6: Issue and verify G2 only from clean committed HEAD**

Run: `git status --porcelain=v1`

Expected: no output.

Run: `cargo xtask f57 gate g2 --bundle-root target/f57/evidence --run-journal target/f57/evidence/g2/gate-run.jcs.jsonl --evidence-out target/f57/evidence/g2`

Run: `cargo xtask f57 evidence verify --receipt target/f57/evidence/g2/ctc-data-receipt.v1.json --bundle-root target/f57/evidence --expect-gate G2_CTC_DATA_GREEN`

Expected: every command exits 0. `gate g2` reruns G0/G1/G2 conformance and emits a fresh same-candidate prerequisite set bound to committed `HEAD`; the G2 receipt binds the unchanged 69-file baseline plus 17-file F57 suffix (`total=86`) through `20261025091310` and the graph/generator digests. It records zero Requirement results, exactly 26 current-profile typed probe results, and `objective_closures=[]`; each probe has its distinct auxiliary TestID and terminal journal ref, and the receipt explicitly states `DEV_SLICE_GREEN=false`. G4 is the first gate allowed to bind the four-row Objective snapshot. No repository file changes after issuance.

## 3. Final completion check

Before handing off to the CTC-01 client plan, run in order:

```text
cargo fmt -- --check
cargo clippy --workspace --all-targets --locked --offline
cargo test --workspace --exclude ep-testkit --all-targets --locked --offline
cargo test -p ep-testkit --test f57_g1_pre_db_boundary --test f57_g1_dynamic_authz --test f57_g1_authorized_pg_tx --test f57_g1_command_pipeline --test f57_g1_generation_activation --test f57_g1_durable_flow --test f57_g1_authority_spine --test f57_g2_governed_file --test f57_g2_customer_contract --test f57_g2_standard_sales --test f57_g2_procurement --test f57_g2_inventory_fulfilment --test f57_g2_invoice_cash --test f57_g2_ctc01_authority_integration -- --nocapture
cargo xtask archcheck
cargo xtask sqlcheck
cargo xtask f57 graph generate --check
cargo xtask f57 verify --level l1 --changed-from HEAD^
cargo xtask f57 evidence verify --receipt target/f57/evidence/g2/ctc-data-receipt.v1.json --bundle-root target/f57/evidence --expect-gate G2_CTC_DATA_GREEN
```

Expected final state:

- `G1_AUTHORITY_SPINE_GREEN=true` as a fresh prerequisite embedded and exact-joined by the G2 aggregate; the earlier standalone G1 receipt is retained only as historical evidence for its own tree.
- `G2_CTC_DATA_GREEN=true`.
- `DEV_SLICE_GREEN=false` until the separate G3/G4 plan passes.
- PostgreSQL has no PurchaseInvoice/AP/SupplierPayment facts for CTC-01.
- No business repository accepts a raw connection or unverified context.
- No external effect in `UNKNOWN` is redispatched.
- No UI, production, hardware, backup, recovery, or release claim is emitted.

Stop and amend the design before continuing if any generated contract has a second owner, any table lacks enforced same-entity RLS, any command can partially commit, any generation can mix participant versions, or any procurement test closes without all three settlement facts.
