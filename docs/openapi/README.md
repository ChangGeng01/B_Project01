# Legacy/compatibility OpenAPI input registry

> **F-65 状态注（照 `metrics-catalog.md:136` 样板）**：本节描述的 checker **尚未实现**（全仓 `xtask/` 内 `openapi` 命中 0，CI 登记表 17 条命令无一涉及）；能力状态 `UNVERIFIED`——**不阻塞按本表开发，但在 checker 真实存在并返回 0 之前，不得声称本表的逐字段一致性已被机器验证**。承接与转绿判据随 G0 生成式权威批次定。

This table is the machine-exact registry only for legacy/compatibility inputs under `docs/openapi/`; it is not the global F-57 OpenAPI authority. Values are literals, not prose: the checker parses each present historical/input YAML's `info.title`、`info.version` and four `info.x-*` extensions and requires exact equality with its row. The [F-57 authority and supersession register](../superpowers/reviews/2026-08-23-f57-authority-supersession-register.md) is the semantic authority/status map.

The only current F-57 OpenAPI machine authorities are graph-generated at G0 under:

```text
docs/generated/f57/openapi/control-center.v1.yaml
docs/generated/f57/openapi/employee-api.v1.yaml
docs/generated/f57/openapi/portal.v1.yaml
```

They share the CapabilityGraph digest and generator version and may never be hand-edited. `SUPERSEDED_PLANNED_PATH` with `ABSENT` is a permanent negative assertion: the old path must never be created or activated. A premature old-path file, missing generated authority after G0, second schema truth, unknown/extra YAML, or graph/projection drift fails closed.

| File | Presence | `info.title` | `info.version` | `x-f57-status` | `x-source-authority` | `x-planned-implementation-tasks` | `x-implementation-state` | Activation owner |
|---|---|---|---|---|---|---|---|---|
| `ai-admin.v1.yaml` | `PRESENT` | `Enterprise Platform AI Model Administration API — F-55 exact surface` | `1.0.0-f55` | `HISTORICAL_SUPERSEDED` | `F-55-section-5.3-historical-input` | `["F57-15","F57-16"]` | `NOT_IMPLEMENTED` | `NONE` |
| `ai-reporting.v1.yaml` | `PRESENT` | `Enterprise Platform Local AI Reporting API — F-55 exact surface` | `1.0.0-f55` | `HISTORICAL_INPUT_LOCAL_AI_DEFERRED` | `F-55-historical-input` | `["F57-14","F57-15"]` | `NOT_IMPLEMENTED_LOCAL_MODEL_DEFERRED` | `NONE` |
| `finance.v1.yaml` | `PRESENT` | `Enterprise Platform Finance API — Stage 10 full surface` | `1.0.0-stage10` | `CURRENT_SUBJECT_INPUT_PENDING_EXACT_REBASELINE` | `F-50-current-subject-input` | `["F57-20"]` | `NOT_IMPLEMENTED_AS_FULL_F57_MACHINE_CONTRACT` | `F57-20` |
| `invoice.v1.yaml` | `PRESENT` | `Enterprise Platform Invoice API — Stage 10 full surface` | `1.0.0-stage10` | `CURRENT_SUBJECT_INPUT_PENDING_EXACT_REBASELINE` | `F-50-current-subject-input` | `["F57-20"]` | `NOT_IMPLEMENTED_AS_FULL_F57_MACHINE_CONTRACT` | `F57-20` |
| `ledger.v1.yaml` | `PRESENT` | `Enterprise Platform Ledger API — F-50 affected surface` | `1.0.0-f50` | `CURRENT_SUBJECT_INPUT_PENDING_EXACT_REBASELINE` | `F-50-current-subject-input` | `["F57-20"]` | `NOT_IMPLEMENTED_AS_FULL_F57_MACHINE_CONTRACT` | `F57-20` |
| `mcp-management.v1.yaml` | `PRESENT` | `Enterprise Platform MCP Management and Inbound API — F-55 exact surface` | `1.0.0-f55` | `HISTORICAL_SUPERSEDED` | `F-55-section-4-historical-input` | `["F57-14","F57-16"]` | `NOT_IMPLEMENTED` | `NONE` |
| `portal.v1.yaml` | `PRESENT` | `Enterprise Platform Portal API — F-50 affected surface` | `1.0.0-f50` | `CURRENT_SUBJECT_INPUT_PENDING_EXACT_REBASELINE` | `F-50-current-subject-input` | `["F57-22"]` | `NOT_IMPLEMENTED_AS_FULL_F57_MACHINE_CONTRACT` | `F57-22` |
| `control-center.v1.yaml` | `ABSENT` | `Enterprise Platform Control Center API` | `1.0.0-f57` | `SUPERSEDED_PLANNED_PATH` | `F57_G0_CAPABILITY_GRAPH` | `[]` | `NEVER_CREATE_USE_GENERATED_AUTHORITY` | `NONE` |
| `employee-api.v1.yaml` | `ABSENT` | `Enterprise Platform Employee API` | `1.0.0-f57` | `SUPERSEDED_PLANNED_PATH` | `F57_G0_CAPABILITY_GRAPH` | `[]` | `NEVER_CREATE_USE_GENERATED_AUTHORITY` | `NONE` |

Generated API versions are derived only from the CapabilityGraph and generator compatibility policy. G3–G6 modify graph nodes and regenerate the same three paths; no historical task number or `docs/openapi` row can create, version, or activate a second F-57 API.
