# Deep Release Convergence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Deeply audit, correct, verify, commit, and push the complete current project change set without expanding unimplemented product scope or overstating production evidence.

**Architecture:** Treat the existing dirty workspace as one candidate release-convergence tree. Audit it through independent runtime/security, persistence/operability, and authority/documentation workstreams; accept only reproduced defects, repair them regression-first, then run one fresh upload gate over the exact staged tree. Delivery is a normal non-force push of `docs/spec-review-revisions` to its existing upstream.

**Tech Stack:** Rust 2021, Cargo, Tokio, Axum/Hyper, SQLx/PostgreSQL 16 contracts, Windows named pipes, Bash, Windows PowerShell 5.1, GitHub Actions, Markdown/TSV machine contracts, Git.

**Spec:** `docs/superpowers/specs/2026-09-01-deep-release-convergence-design.md`

## Global Constraints

- Keep `implementation_state=NOT_IMPLEMENTED` and `production_state=PRODUCTION_NOT_READY` until separately implemented and evidenced.
- Windows Server 2022 is the only production authority host; non-Windows execution is development evidence only.
- The installed 1 TB HDD remains permanently `production_eligible=false`.
- A production data device requires at least `2,000,000,000,000` raw bytes and the dynamic NTFS capacity floor.
- Do not implement the F-57 business product, production KMS provider, Windows pipe DACL/token authority, or any external certification in this pass.
- Default and production paths fail closed; no file-secret, network, identity-header, IPC, CI, or self-check fallback may turn missing authority into success.
- Preserve user-authored changes; do not reset, discard, force-push, merge, release, or deploy.
- Every demonstrated behavior defect receives a regression test that fails without the fix and passes with it.
- External Windows Server, PostgreSQL, P340, UPS, HDD, VSS, backup/restore, 72-hour load, and signing checks remain explicitly `EXTERNAL_NOT_RUN` when their environment is unavailable.

---

### Task 1: Freeze the Candidate Inventory and Upload Boundary

**Files:**
- Inspect: all paths reported by `git status --porcelain=v1 -uall`
- Inspect: `.gitignore`, `Cargo.toml`, and `Cargo.lock`
- Modify: this plan only to record final evidence and completed checkboxes

**Interfaces:**
- Consumes: local branch `docs/spec-review-revisions`, upstream `origin/docs/spec-review-revisions`, and design commit `bf568b8`.
- Produces: an exact candidate-file inventory and verified exclusion list for credentials, build products, local databases, editor state, and unrelated files.

- [ ] **Step 1: Capture repository identity and complete status**

```bash
git branch --show-current
git rev-parse --abbrev-ref --symbolic-full-name '@{upstream}'
git rev-parse --git-dir
git rev-parse --git-common-dir
git status --porcelain=v1 -uall
```

Expected: the branch/upstream equal the spec and this is a normal checkout.

- [ ] **Step 2: Enumerate the candidate**

```bash
git diff --name-status
git ls-files --others --exclude-standard
git diff --stat
```

Expected: every path belongs to code, SQL, CI, scripts, or project documentation in the approved scope.

- [ ] **Step 3: Scan for accidental secrets and generated artifacts without logging values**

```bash
rg -l --hidden -g '!.git/**' -g '!target/**' -g '!*.lock' '(BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY|AKIA[0-9A-Z]{16}|gh[pousr]_[A-Za-z0-9_]{20,}|xox[baprs]-[A-Za-z0-9-]{10,})' .
rg --files --hidden -g '!.git/**' -g '!target/**' | rg '(^|/)(\.env($|\.)|master\.key$|id_rsa$|id_ed25519$|.*\.(pfx|p12|key|pem|db|sqlite|bak|dump)$)'
```

Expected: no candidate contains a real credential or local data artifact. Documentation-only placeholders must be manually confirmed inert.

- [ ] **Step 4: Confirm no staged data predates the audit**

```bash
git diff --cached --name-status
```

Expected: empty output.

### Task 2: Audit Runtime Trust Boundaries and Failure Semantics

**Files:**
- Review/modify: `crates/platform/runtime/src/http/*.rs`
- Review/modify: `crates/platform/runtime/src/config/{secret,sections}.rs`
- Review/modify: `crates/platform/runtime/src/{serving,shutdown}.rs`
- Review/modify: `crates/platform/identity/src/{login,mfa}.rs`
- Review/modify: `crates/adapter/kms/src/*.rs`
- Review/modify: `crates/adapter/ipc/src/*.rs`
- Review/modify: `apps/{core-server,job-worker,ops-agent}/src/main.rs`
- Test: colocated unit tests and `apps/{core-server,job-worker,ops-agent}/tests/default_kms_fail_closed.rs`

**Interfaces:**
- Consumes: `SecurityContext`, trusted-proxy configuration, `SecretString`, KMS backend configuration, service startup wiring, and Windows IPC listener state.
- Produces: bounded request identity metadata, redacted secrets, fatal missing-authority startup, and recoverable-but-fail-closed IPC listener behavior.

- [ ] **Step 1: Verify identity and request-metadata invariants**

Confirm these exact behaviors: inbound `x-ep-*` headers are removed; only `SecurityContext` authorizes requests; forwarded addresses require a trusted transport peer; untrusted method/source inputs cannot grow metric or limiter cardinality without bound; one server-generated request/trace identity reaches response, log, and envelope.

```bash
cargo test -p ep-platform-runtime http:: --all-features --locked
cargo test -p core-server platform::middleware:: --all-features --locked
```

Expected: all selected tests pass.

- [ ] **Step 2: Verify KMS build shape and fatal startup semantics**

```bash
cargo check -p ep-adapter-kms --locked
cargo check --release -p ep-adapter-kms --locked
cargo check -p ep-adapter-kms --features legacy-master-key-file --locked
cargo check --release -p ep-adapter-kms --features legacy-master-key-file --locked
cargo test -p core-server -p job-worker -p ops-agent --test default_kms_fail_closed --locked
cargo test --release -p core-server -p job-worker -p ops-agent --features legacy-file --test default_kms_fail_closed --locked
```

Expected: every command returns 0; each process observes exit 78, `ERROR`, and `NOT_IMPLEMENTED`, without `PASSED` or pool construction.

- [ ] **Step 3: Verify redaction and release rejection**

```bash
cargo test -p ep-platform-runtime config::secret::tests --all-features --locked
cargo test --release -p ep-platform-runtime --features legacy-file config::secret::tests --locked
cargo test --release -p core-server -p job-worker -p ops-agent --features legacy-file release_shape_has_no_legacy_file_reader --locked
```

Expected: all tests pass; diagnostics reveal neither secret contents nor absolute secret paths.

- [ ] **Step 4: Verify Windows IPC recovery without claiming Windows runtime evidence**

```bash
cargo test -p ep-adapter-ipc --lib --locked
cargo check -p ep-adapter-ipc --tests --target x86_64-pc-windows-msvc --locked
```

Expected: host tests pass and the Windows-only listener tests compile. Actual Windows execution remains `EXTERNAL_NOT_RUN`.

- [ ] **Step 5: Close reproduced defects regression-first**

For each failed invariant, add one narrowly named test beside the owning module, observe the unfixed failure, apply the smallest fix at the same authority boundary, rerun the focused test, then rerun Steps 1–4. Do not add business endpoints or replace a deliberate `NOT_IMPLEMENTED` result.

### Task 3: Audit Persistence, Concurrency, and P340 Operability

**Files:**
- Review/modify: `crates/adapter/db-pg/src/{budget,pool,tx,retry}.rs`
- Review/modify: `crates/adapter/db-pg/tests/live_pg.rs`
- Review/modify: `apps/{core-server,job-worker,ops-agent}/src/wiring/db.rs`
- Review/modify: `db/bootstrap/*.sql` and `db/migrations/**/*.sql`
- Review/modify: `docs/superpowers/specs/2026-08-23-f57-windows-p340-production-profile.md`

**Interfaces:**
- Consumes: four pools `Rw`, `Ro`, `Worker`, `Ops`; tenant session GUCs; retry policy; nine capacity buckets and fourteen selectors.
- Produces: application pool budget 37, PostgreSQL ceiling 52/64, rollback-before-clear semantics, and exact HDD admission arithmetic.

- [ ] **Step 1: Prove the four-pool budget and reject stale pools**

```bash
cargo test -p ep-adapter-db-pg budget:: --locked
cargo test -p core-server wiring::db:: --locked
cargo test -p job-worker wiring::db:: --locked
cargo test -p ops-agent wiring::db:: --locked
rg -n 'PoolKind::Integ|integ_pool|integration_pool' crates apps docs db
```

Expected: tests pass; any text match is historical, not active code/configuration.

- [ ] **Step 2: Prove failed transactions cannot leak tenant state**

```bash
cargo test -p ep-adapter-db-pg pool:: --locked
cargo test -p ep-adapter-db-pg tx:: --locked
cargo test -p ep-adapter-db-pg retry:: --locked
```

Expected: cleanup performs `ROLLBACK` before clearing all session GUCs; either cleanup failure discards the connection; unknown or non-idempotent work is never retried.

- [ ] **Step 3: Verify SQL and optional live PostgreSQL evidence**

```bash
cargo xtask sqlcheck
cargo test -p ep-adapter-db-pg --test live_pg --locked
```

Expected: static SQL checks pass. Six environment-dependent tests stay ignored unless `EP_TEST_PG_URL` names an approved disposable PostgreSQL 16 instance; otherwise record `EXTERNAL_NOT_RUN`.

- [ ] **Step 4: Recompute capacity truth independently**

```bash
awk 'BEGIN { buckets=161061273600+64424509440+42949672960+375809638400+375809638400+375809638400+25769803776+8589934592+51539607552; reserve=107374182400; shared=20*1073741824+2*1073741824+2*1073741824+1*1073741824+1*1073741824+34*1073741824; print buckets, buckets+reserve, shared }'
```

Expected:

```text
1481763717120 1589137899520 64424509440
```

The profile must also require `raw_device_bytes >= 2,000,000,000,000` and the documented dynamic NTFS formula.

- [ ] **Step 5: Close reproduced persistence defects regression-first**

Add the failed assertion to the owning Rust or shell test, observe failure, correct the single source of truth, update dependent mirrors, and rerun Steps 1–4.

### Task 4: Audit CI, Developer Controls, and Authority Documents

**Files:**
- Review/modify: `.github/workflows/ci.yml`
- Review/modify: `.github/ci/**`
- Review/modify: `scripts/dev-*`, `scripts/tests/**`
- Review/modify: `xtask/src/{ci,configdoc,sqlcheck,main}.rs` and `xtask/tests/**`
- Review/modify: `README.md`, `docs/adr/**`, `docs/config-reference.md`, `docs/ci-pipeline.md`, `docs/threat-model.md`, and `docs/superpowers/{plans,reviews,specs}/**`

**Interfaces:**
- Consumes: 11-stage/19-command manifest, seven-face known-red baseline, state register, GitHub-only CI decision, and P340 profile.
- Produces: fail-closed CI evidence, idempotent developer controls, and one contradiction-free authority chain.

- [ ] **Step 1: Run CI harness and local-control negatives**

```bash
bash .github/ci/tests/run-negative.sh
bash .github/ci/verify-pipeline-commands.sh
bash scripts/tests/dev-controls-negative.sh
```

Expected: 40/40 CI negatives pass with zero unconstructible cases; 11 stages/19 commands are document-equal; Unix controls pass.

- [ ] **Step 2: Validate PowerShell without overstating coverage**

When `pwsh` exists:

```bash
pwsh -NoProfile -File scripts/tests/dev-controls-negative.ps1
```

When absent, rely only on the static PowerShell checks in the preceding shell suite and record PowerShell runtime as `EXTERNAL_NOT_RUN`.

- [ ] **Step 3: Measure every delivered authority face**

```bash
cargo xtask archcheck
cargo xtask sqlcheck
cargo xtask codecheck
cargo xtask errorcodes
cargo xtask configdoc
cargo xtask configdoc --check-doc-type-codes
cargo xtask eventcatalog
```

Expected registered vector:

```text
archcheck 0/0
sqlcheck 0/0
codecheck 1/2
errorcodes 1/12
configdoc 1/304
eventcatalog 1/117
doc-type-codes exit 3; 43 documented codes, 0 code constants, 1 uncovered item
```

Narrowing requires an evidence-backed baseline update; widening or unmeasurable output blocks upload.

- [ ] **Step 4: Scan state and superseded authority**

```bash
rg -n 'implementation_state=|production_state=' README.md docs
rg -n '2GB_CMR|2 GB HDD|raw_device_bytes[^\n]*2,000,000,000([^0-9]|$)' README.md docs db crates apps
rg -n 'Forgejo|Woodpecker' README.md docs .github
rg -n -- '--locked--test|PoolKind::Integ|integration\.pool|portal\.upstream' README.md docs crates apps .github scripts db
```

Expected: active truth uses only `NOT_IMPLEMENTED` / `PRODUCTION_NOT_READY`; no 2 GB typo or active five-pool/config authority remains; Forgejo/Woodpecker matches are historical only.

- [ ] **Step 5: Close reproduced machine-contract conflicts at their owner**

Update the machine owner first and its mirrors second. Add a negative fixture for parser/comparator defects and a focused Rust test for xtask defects. Rerun Steps 1–4 after each correction set.

### Task 5: Run the Fresh Full Upload Gate

**Files:**
- Inspect: complete workspace
- Modify: only files implicated by a failing gate

**Interfaces:**
- Consumes: Tasks 1–4 with no unresolved critical/important finding.
- Produces: fresh evidence for the exact tree to stage.

- [ ] **Step 1: Verify format, whitespace, Debug compilation, and lint**

```bash
cargo fmt --all -- --check
git diff --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Expected: all return 0.

- [ ] **Step 2: Verify all-feature and Release shapes**

```bash
cargo check --workspace --all-targets --all-features --locked
cargo check --workspace --all-targets --all-features --release --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Expected: all return 0.

- [ ] **Step 3: Run focused all-feature host tests under normal socket permissions**

```bash
cargo test -p ep-platform-runtime -p ep-adapter-kms -p core-server -p job-worker -p ops-agent --all-features --locked
cargo test -p ep-adapter-db-pg -p ep-adapter-ipc -p ep-platform-identity --locked
```

Expected: all executable tests pass. A sandbox `EPERM` from a real listener test must be rerun with normal host permissions, never waived.

- [ ] **Step 4: Measure full-workspace known red**

```bash
cargo test --workspace --no-fail-fast --locked
bash .github/ci/compare-red-baseline.sh
```

Expected: exactly three intentional repository-truth failures; comparator vector `0/0,0/0,1/2,1/12,1/304,1/117,1/3`; comparator exit 0.

- [ ] **Step 5: Cross-compile for Windows**

```bash
cargo check --workspace --all-targets --target x86_64-pc-windows-msvc --locked
cargo check --release -p core-server -p job-worker -p ops-agent --features legacy-file --target x86_64-pc-windows-msvc --locked
```

Expected: both return 0; this is compile evidence only.

### Task 6: Independent Final Review and Finding Closure

**Files:**
- Review: complete diff from pre-convergence upstream to candidate HEAD/worktree
- Modify: only paths implicated by validated findings

**Interfaces:**
- Consumes: Task 5 candidate.
- Produces: independent runtime/security, persistence/operability, and documentation/authority verdicts without unresolved critical/important findings.

- [ ] **Step 1: Dispatch three disjoint reviewers**

Reviewer A covers Rust trust boundaries, secrets, startup, HTTP identity, IPC, shutdown, and denial of service. Reviewer B covers transactions, pools, SQL grants, retry/idempotency, CI scripts, and cross-platform operability. Reviewer C covers authority precedence, state truth, P340 capacity, external-evidence labels, omissions, and contradictions.

- [ ] **Step 2: Reproduce every blocking finding**

Require file, line, input/state sequence, violated contract, and observable consequence. Reject preference-only refactors or proposals that implement a non-goal.

- [ ] **Step 3: Fix validated findings regression-first**

Rerun Task 2, 3, or 4 according to ownership, then rerun the complete Task 5 gate because every fix invalidates earlier evidence.

- [ ] **Step 4: Obtain final no-blocker verdicts**

Expected: all reviewers report no unresolved critical/important issue and list external evidence separately.

### Task 7: Stage, Commit, Push, and Verify Remote State

**Files:**
- Stage: every intended project path from Task 1
- Exclude: credentials, local data, build output, unrelated artifacts

**Interfaces:**
- Consumes: freshly verified and reviewed tree.
- Produces: final convergence commit on `docs/spec-review-revisions`, matching upstream head, clean worktree.

- [ ] **Step 1: Refresh upstream without changing local history**

```bash
git fetch origin docs/spec-review-revisions
git rev-list --left-right --count origin/docs/spec-review-revisions...HEAD
```

Expected: remote-ahead count is zero. If nonzero, stop and reconcile without force.

- [ ] **Step 2: Stage and inspect the full candidate**

```bash
git add -A
git diff --cached --check
git diff --cached --name-status
git diff --cached --stat
```

Expected: every intended path is present; no secret, build output, local data, or unrelated file is staged.

- [ ] **Step 3: Commit the verified candidate**

```bash
git commit -m "fix: converge runtime security and authority contracts"
```

Expected: one commit contains the full intended candidate.

- [ ] **Step 4: Re-run immutable-tree smoke evidence**

```bash
git diff --check HEAD^ HEAD
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features --locked
bash .github/ci/compare-red-baseline.sh
```

Expected: all return 0 and no post-commit mutation exists.

- [ ] **Step 5: Push normally and prove the remote head**

```bash
git push origin docs/spec-review-revisions
git ls-remote --heads origin refs/heads/docs/spec-review-revisions
git rev-parse HEAD
git status --short --branch
```

Expected: remote object ID equals local `HEAD`; branch tracks upstream; worktree is clean. Do not create a PR, merge, release, or deploy.
