# Deep Release Convergence Design

**Date:** 2026-09-01

**Branch:** `docs/spec-review-revisions`

**Decision:** Approved approach A — deeply converge the complete current change set, then commit and push the current branch.

## 1. Purpose

This pass prepares the existing architecture, documentation, runtime skeleton, database foundation, local-development controls, and CI evidence for the next formal development phase. It does not claim that the business product or its production controls are implemented.

The governing truth remains:

- `implementation_state=NOT_IMPLEMENTED`
- `production_state=PRODUCTION_NOT_READY`
- Windows Server 2022 is the only production authority host.
- The installed 1 TB HDD is permanently ineligible for production data. A production data device requires at least 2,000,000,000,000 raw bytes and must satisfy the dynamic NTFS capacity formula.

## 2. Scope

The convergence pass covers every tracked and untracked artifact already present in the current workspace that belongs to this project:

1. Rust runtime and adapters: startup, shutdown, identity propagation, request metadata, rate limiting, database pools and transaction cleanup, secret handling, KMS failure semantics, Windows IPC, metrics, and self-check behavior.
2. SQL and database authority: roles, grants, migrations, pool budgets, session cleanup, and live-PostgreSQL test contracts.
3. CI and developer controls: fail-closed known-red comparison, executable pipeline manifests, negative fixtures, Unix scripts, and PowerShell 5.1-compatible scripts.
4. Architecture and product documents: authority chain, status truth, Windows/P340 deployment profile, capacity arithmetic, plugin boundaries, and explicit external evidence.
5. Repository delivery: review the complete diff, commit every intended project artifact, push the existing branch to its configured upstream, and verify the remote branch contains the resulting commit.

## 3. Non-goals

This pass does not implement or certify:

- the F-57 business product;
- the production KMS-backed secret provider;
- Windows named-pipe DACL construction or bidirectional process-token validation;
- real Windows Server 2022 runtime behavior;
- real PostgreSQL, P340, UPS, HDD, VSS, backup/restore, 72-hour load, or signing evidence;
- a PR merge, production deployment, or release declaration.

Those capabilities must remain fail closed or explicitly marked `NOT_IMPLEMENTED` / `EXTERNAL_NOT_RUN` until their own implementation and evidence exist.

## 4. Convergence Strategy

The work uses four ordered layers:

### 4.1 Truth and boundary audit

Compare code, migrations, CI contracts, ADRs, plans, reviews, and configuration reference material. Resolve contradictions in favor of the registered authority chain. Reject wording that upgrades compilation, simulation, or static review into runtime or production evidence.

### 4.2 Failure and attack-path audit

Inspect all trust boundaries and privileged startup paths for fallback, fake-green, unbounded-cardinality, secret exposure, confused-deputy, partial-transaction, retry, shutdown, and Windows IPC recovery defects. A demonstrated defect receives the smallest coherent fix plus a regression test. Speculative product functionality is not added.

### 4.3 Consistency and operability audit

Check configuration defaults, feature combinations, Debug/Release behavior, Windows cross-compilation, local-development lifecycle scripts, database connection budgets, capacity arithmetic, and the exact known-red contract. Generated or machine-readable contracts must agree with their human-readable authority documents.

### 4.4 Delivery audit

Run fresh verification on the exact tree to be uploaded. Independent reviewers examine the final diff for correctness, security, and documentation/state conflicts. Critical and important findings block the upload. After a clean review, all intended files are committed on `docs/spec-review-revisions`, pushed without force, and verified against `origin/docs/spec-review-revisions`.

## 5. Verification Contract

The upload gate requires fresh evidence for all locally executable checks:

- formatting and whitespace checks;
- workspace Debug and Release compilation, including all features;
- strict workspace Clippy;
- focused and full test suites under normal host permissions when local sockets are required;
- negative CI fixtures and command-manifest validation;
- architecture, SQL, configuration, event-catalog, error-code, and known-red comparison gates;
- Windows MSVC cross-compilation;
- exact status, capacity, authority, removed-key, and forbidden-phrase scans;
- independent final code/security and documentation/architecture reviews.

Known-red checks may remain nonzero only when they exactly match the registered baseline and the outer comparator returns success. Any new failure, count increase, malformed evidence, secret disclosure, unexpected network or disk fallback, or state contradiction blocks commit and push.

## 6. Upload Contract

The current branch and upstream are fixed as:

- local: `docs/spec-review-revisions`
- remote: `origin/docs/spec-review-revisions`

Delivery will not rewrite history, force-push, merge to another branch, create a release, or deploy. The final verification commit may include all existing intended changes and the additional convergence fixes. After pushing, the local `HEAD`, upstream reference, and remote queried head must agree.

## 7. Success Criteria

The pass is complete only when:

1. no reproducible critical or important finding remains in the approved scope;
2. all locally executable upload gates have current passing evidence or exactly match the registered known-red baseline;
3. external-only evidence is named rather than implied;
4. the two governing state values and hardware eligibility rules remain consistent everywhere;
5. the full intended workspace is committed with no accidental secret, build output, or unrelated artifact;
6. the commit is present on `origin/docs/spec-review-revisions` and the local worktree is clean.
