# Comprehensive Review Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close every confirmed security, CI, runtime-topology, Windows-operability, and authority-document defect from the 2026-08-31 repository audit without disguising intentionally unimplemented product stages as green.

**Architecture:** Keep the current Rust C/S architecture and its authority boundaries. Make security inputs server-owned, bound all anonymous admission state, sanitize pooled PostgreSQL connections before reuse, remove the obsolete integration database pool, strengthen CI evidence validation, add native Windows development controls, and update every authoritative mirror of each ruling in the same change set.

**Tech Stack:** Rust 2021, Axum, SQLx/PostgreSQL, Bash, PowerShell 7 / Windows PowerShell 5.1, GitHub Actions, Markdown/TSV machine contracts.

**Spec:** `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`, `docs/adr/ADR-0018-integration-gateway-zero-database.md`, `docs/adr/ADR-0019-f57-runtime-topology-and-measured-connection-budget.md`, `docs/adr/ADR-0027-ci-platform-github-only.md`

## Global Constraints

- Production authority runs on Windows Server 2022; client-supplied `x-ep-*` values never become trusted identity or correlation state.
- The P340 server envelope is i5-10500, 32 GiB RAM and a 256 GB runtime SSD. The currently installed 1 TB HDD is development/validation-only with `production_eligible=false`; a single-disk degraded production candidate requires a recertified CMR DATA_HDD with raw capacity `>=2,000,000,000,000` bytes and checked NTFS `volume_total_bytes>=max(1,589,137,899,520,1,481,763,717,120+107,374,182,400+measured_unclassifiable_filesystem_allocation_bytes)`. A later RAID1 requires each member to meet that same 2 TB profile; the old 1 TB disk cannot be reused as a member or backup target.
- The current P340 implementation seed has four pools: `rw=20`, `ro=10`, `worker=5`, `ops=2`; resident 37, temporary 10, reserve 5, peak 52, `max_connections=64`. Per ADR-0019 these are measured-generation inputs, not immutable product truth; production admission still requires an exact consumer set and a hardware/config-generation capacity certificate.
- `integration-gateway` has zero runtime database access.
- Known-red product gaps remain visible and failing until their owning implementation stage lands; remediation must not skip, ignore, or weaken those gates.
- Every behavior change follows RED → GREEN tests; document-only corrections use exact-reference scans and machine-contract checks.
- No push, merge, publish, destructive database operation, or production deployment is part of this plan.

---

### Task 1: Bound anonymous admission and make request metadata server-owned

**Files:**
- Modify: `apps/core-server/src/platform/middleware.rs`
- Modify: `crates/platform/runtime/src/http/server.rs`
- Test: unit tests in both modules

**Interfaces:**
- Consumes: Axum `ConnectInfo<SocketAddr>` injected by both runtime serve functions.
- Produces: a bounded `PreAuthRateLimiter`, canonical peer-IP source keys, and freshly generated internal `x-ep-request-id` / `x-ep-trace-id` headers; authenticated requests additionally receive the verified device ID.

- [x] Add failing tests proving active-window unique login/source keys cannot exceed fixed caps, overlong login names are rejected without allocation, spoofed `X-Forwarded-For` is ignored, and every inbound `x-ep-*` header is removed.
- [x] Run the focused core-server/runtime tests and confirm each new test fails for the missing behavior.
- [x] Add hard cardinality and key-length bounds, periodic bounded eviction, peer-socket extraction, prefix-wide `x-ep-*` stripping, server correlation injection, and verified device injection.
- [x] Serve routers through `into_make_service_with_connect_info::<SocketAddr>()` in both listener paths.
- [x] Re-run focused tests and `cargo test -p core-server -p ep-platform-runtime --locked`.

### Task 2: Sanitize PostgreSQL connections and enforce the four-pool topology

**Files:**
- Modify: `crates/adapter/db-pg/src/pool.rs`
- Modify: `crates/adapter/db-pg/src/budget.rs`
- Modify: `crates/adapter/db-pg/tests/live_pg.rs`
- Modify: `crates/platform/runtime/src/config/sections.rs`
- Modify: `apps/core-server/src/wiring/db.rs`
- Modify: `apps/job-worker/src/wiring/db.rs`
- Modify: affected adapter/runtime/wiring tests and configuration references

**Interfaces:**
- Consumes: ADR-0018 four-pool authority and SQLx `after_release` retention contract.
- Produces: rollback-before-clear connection cleanup; `PoolKind::{Rw,Ro,Worker,Ops}` only; totals 37/10/5/52.

- [x] Add failing tests for the cleanup command order and for rejection of any fifth `Integ` pool in budget/config/wiring surfaces.
- [x] Run focused adapter/runtime tests and confirm failures identify the stale ordering/topology.
- [x] Execute `ROLLBACK` before session-level `set_config(..., '', false)` clears; propagate either failure so SQLx discards the connection.
- [x] Remove the obsolete `Integ` enum/config/spec/wiring path and update exact pool-count assertions without changing the 64-connection server ceiling.
- [x] Add an ignored live-PostgreSQL regression that opens a transaction after setting session GUCs, returns the connection, reacquires it, and observes four empty GUCs.
- [x] Re-run adapter/runtime/wiring tests; run the live test only when `EP_TEST_PG_URL` is present.

### Task 3: Make CI baseline evidence fail closed

**Files:**
- Modify: `.github/ci/compare-red-baseline.sh`
- Modify: `.github/ci/tests/run-negative.sh`
- Modify: test fixtures under `.github/ci/tests/fixtures/` if an executable command shim is needed

**Interfaces:**
- Consumes: the exact baseline gate set `archcheck`, `sqlcheck`, `codecheck`, `errorcodes`, `configdoc`, `eventcatalog`, `cargo-test`.
- Produces: deterministic outer comparator exit categories `0=no new regression`, `2=new regression`, `3=uncovered`, `64=usage error`; measured gate outcomes remain payload data and must not leak through as the comparator's outer result. Malformed, partial, duplicate, unknown, or unmeasurable evidence never reports 0.

- [x] Extend the negative suite first with unreadable path, missing gate, duplicate gate, unknown gate, invalid expected exit, invalid count, and non-ordinal exit-transition cases; replace all three unconditional exit-70 skips with deterministic command shims.
- [x] Run `bash .github/ci/tests/run-negative.sh` and confirm the new cases fail against the old comparator/harness.
- [x] Fix `${BASELINE}` interpolation, validate the exact unique gate allowlist before measurement, validate allowed exit/count pairs, and compare exit states categorically rather than numerically.
- [x] Re-run the negative suite until it has zero failed and zero unconstructible cases.

### Task 4: Add reliable Windows and Unix local-environment controls

**Files:**
- Modify: `scripts/dev-up.sh`
- Create: `scripts/dev-up.ps1`
- Create: `scripts/dev-down.ps1`
- Create or modify: script-level tests under `scripts/tests/`
- Modify: `docs/superpowers/plans/2026-08-10-first-release-dev-plan/01-engineering-baseline.md` only if invocation text needs exact alignment

**Interfaces:**
- Consumes: existing Docker Compose/Podman Compose environment and `.env.dev` contract.
- Produces: idempotent up/down commands on Windows Server 2022 and guaranteed-length generated secrets on Unix.

- [x] Add failing script tests that force short random input, verify an exact secret length, verify PowerShell argument/help behavior, and verify `--db-only` / full-stack command selection without starting containers.
- [x] Replace fixed-size filtered randomness with a loop or fixed-width encoding that always produces the required length.
- [x] Implement PowerShell 5.1-compatible engine detection, environment creation, database/full-stack up, status reporting, and down/volume-preservation behavior matching the shell scripts.
- [x] Run shell tests and PowerShell parser/tests when `pwsh` is available; otherwise record parser verification as an external-environment boundary.

### Task 5: Normalize machine contracts and authority mirrors

**Files:**
- Modify: `docs/superpowers/specs/2026-08-23-f57-business-execution-contract.md`
- Modify: `docs/superpowers/plans/2026-08-24-f57-g0-bootstrap-implementation.md`
- Modify: `docs/superpowers/reviews/2026-08-23-f57-authority-supersession-register.md`
- Modify: `docs/adr/README.md`
- Modify: `docs/ci-pipeline.md`
- Modify: `docs/superpowers/plans/2026-08-10-first-release-dev-plan/00c-gap-ruling.md`
- Modify: `db/bootstrap/02_cluster_params.sql`
- Modify: dependent reviewed-golden/seed references only where their schema requires the new typed identifiers

**Interfaces:**
- Consumes: GitHub-only ADR-0027, four-pool ADR-0018/0019, the 350 GB WAL-spool equality, and `StateDomainDefinitionV1` typed fields.
- Produces: one unambiguous authority chain, executable Cargo commands, named purchase-order reverse guards/invariants/triggers, and a P340 capacity table whose arithmetic includes the 350 GB reservation plus a distinct archive-failure budget.

- [x] Correct all four `--locked--test` tokens to `--locked --test` and prove no malformed token remains.
- [x] Assign stable action, guard, invariant, and reverse-fact trigger IDs to the three purchase-order reverse edges; keep historical seed rows unchanged unless their declared schema has a matching typed field.
- [x] Remove Forgejo/Woodpecker from the active ADR-0022 authority mirror and mark GitHub Actions as the only current CI platform while retaining thin-adapter principles.
- [x] Recompute the P340 HDD policy as an exact nine-bucket vector `161061273600,64424509440,42949672960,375809638400,375809638400,375809638400,25769803776,8589934592,51539607552` bytes, with fourteen capacity classes and fourteen canonical selectors assigning every product-managed canonical `data_root` object and registered VSS extent exactly once. Prove checked bucket sum `1,481,763,717,120` plus non-borrowable `107,374,182,400` yields the nominal `1,589,137,899,520`-byte NTFS floor, then require `volume_total_bytes>=max(1,589,137,899,520,1,481,763,717,120+107,374,182,400+measured_unclassifiable_filesystem_allocation_bytes)` and the `2,000,000,000,000`-byte raw-device floor. The whole-volume collector may place only proven NTFS/BitLocker metadata in the measured scalar; metadata gets no class, `unclassified_allocation_bytes` must remain zero, and used bytes must equal class allocation plus measured metadata. Close the 60 GiB shared bucket at `20+2+2+1+1+34` GiB, freeze backup staging at `25,769,803,776`, writer spool default/range/hard max at `268,435,456` / `67,108,864..=2,147,483,648` / `2,147,483,648`, require aggregate admission before local limits and fail closed on unknown/duplicate/cross-bucket/unclassified allocation bytes, metadata catch-all class or arithmetic overflow. Freeze live-WAL hold/hard thresholds at `650/700 GiB`, archive-failure hold/hard thresholds at `300/350 GiB`, sampling at `<=30 s`, and prove measured peak WAL rate times `(sample + shutdown)` is `<50 GiB`.
- [x] Mark the installed 1 TB profile `production_eligible=false`; require every later RAID1 member independently to satisfy the 2 TB profile and prohibit reusing the old 1 TB disk as either member or backup target.
- [x] Update bootstrap/config comments and tests to the four-pool 37-connection topology.
- [x] Keep `db/bootstrap/02_cluster_params.sql` executable only for development/test/manual validation; document that G6 production renders the signed exact `postgresql.conf`, requires `postgresql.auto.conf` absent or empty with no effective overrides, and never runs the `ALTER SYSTEM` bootstrap.
- [x] Run exact state-semantic table/readback scans, `cargo xtask configdoc`, `cargo xtask archcheck`, and exact `rg` scans for the superseded phrases/arguments. Record the planned `cargo xtask f57 graph generate --check` as unimplemented rather than pretending it ran.

### Task 6: Full regression, security bypass review, and evidence closeout

**Files:**
- Modify only files implicated by a reproduced regression.

**Interfaces:**
- Consumes: Tasks 1–5.
- Produces: a clean focused test set, unchanged-or-narrowed known-red counts, and a fresh independent security verdict.

- [x] Run formatting, workspace compilation, focused unit/integration tests, CI negative tests, and all delivered xtask gates.
- [x] Run the full workspace test command and compare its three intentional repository-red tests against `.github/ci/known-red-baseline.tsv`; any new failure or count increase blocks completion.
- [x] Run the hardened baseline comparator and confirm malformed/partial evidence cannot return 0.
- [x] Dispatch a fresh read-only security reviewer with only the original findings and current diff; fix any demonstrated bypass and repeat one scoped review.
- [x] Inspect `git diff --check`, `git status`, and the final diff; report external-only checks separately and do not claim them as passed.

## Closeout Evidence (2026-09-01)

> Historical checkpoint, not evidence for a later tree. The subsequent deep-convergence audit found additional runtime, transaction-cleanup, CI, and document defects. Its execution and current verification are governed by `2026-09-01-deep-release-convergence-implementation.md` and the [2026-09-07 verification record](../reviews/2026-09-07-deep-release-convergence-verification.md). In particular, the historical command-manifest check below was only a partial availability check; it did not prove that every Cargo argument row was executable. A baseline-comparator exit of 0 means no new regression against registered failures, never a green CI or production admission.

The remediation scope is complete, but the product is not implemented and is not production-ready. The governing states remain `implementation_state=NOT_IMPLEMENTED` and `production_state=PRODUCTION_NOT_READY`.

- `cargo fmt --all -- --check`, `git diff --check`, workspace `cargo check --all-targets --locked`, and workspace Clippy with `-D warnings` passed.
- Workspace all-feature debug/release checks and all-feature Clippy with `-D warnings` passed. The final all-feature test set for runtime, KMS, core-server, job-worker, and ops-agent passed under normal host permissions, including the real loopback-socket test; its earlier sandbox-only `EPERM` was an execution-environment limitation, not a product test failure.
- The full workspace test run under normal host permissions had exactly three failing repository-truth tests: `codecheck`, `configdoc`, and `eventcatalog`. The hardened comparator returned 0 because all seven measured faces exactly matched the registered baseline: `0/0,0/0,1/2,1/12,1/304,1/117,1/3`.
- The negative CI fixture suite passed all 40 cases with zero unconstructible or unexpected cases; the command manifest proved all 11 stages are executable and document-equal.
- `archcheck` passed 19/19 rules; `sqlcheck` passed 15 rules over 69 migrations. The removed-key parser regression passed and now reads every old key before the full-width colon without treating replacement or explicitly pending keys as removed.
- The complete workspace cross-compiled for `x86_64-pc-windows-msvc`. This is a compile result only, not a Windows Server 2022 runtime or security certification.
- The nine buckets sum to `1,481,763,717,120`; adding the non-borrowable `107,374,182,400` yields `1,589,137,899,520`. One measured metadata byte raises the dynamic floor to `1,589,137,899,521`; the 60 GiB shared bucket independently sums to `64,424,509,440`.
- The sensitive self-hosted workflow now runs only protected `main` refs, pins checkout by full commit, removes persisted credentials, and declares read-only token permissions. Repository branch protection, runner-group allowlisting, runner reset, and an isolated ephemeral PR/L1 runner remain external prerequisites; the PR/L1 runner is not implemented.
- Default `KmsSecretProvider` failure is now fatal before pool construction in core-server, job-worker, and ops-agent. Three process-level regressions prove exit 78, `NOT_IMPLEMENTED`, ERROR severity, no false PASSED report, and no pool-construction attempt.
- The legacy data-KMS `master.key` path now defaults empty. Default/release builds have no disk constructor; only an explicit Unix `legacy-file` development/test debug feature retains the historical 0400 reader. `builtin` and `hsm` otherwise return stable `NOT_IMPLEMENTED` rather than silently falling back.
- Windows named-pipe listener replenishment now preserves an already connected stream when prebuild fails and recreates a missing pending instance on the next accept. Its two Windows-only regressions cross-compile, but their runtime result remains `EXTERNAL_NOT_RUN` on this macOS host.
- PowerShell was not installed on this review host. Unix behavior negatives and the static PowerShell safety-order gate passed, but PowerShell 5.1/7 execution remains `EXTERNAL_NOT_RUN`.
- Six live PostgreSQL tests remain intentionally ignored without `EP_TEST_PG_URL`; Windows named-pipe DACL/token behavior, the KMS-backed secret provider, real P340/UPS/disk probes, backup/restore drills, 72-hour load certification, signing lanes, and `cargo xtask f57 graph generate --check` remain unimplemented or external and must not be reported as passed.
