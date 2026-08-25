# Database Migration Catalog

> **F-57 Status: `REGISTRY_PENDING_REBASELINE`.** Existing versions and paths remain collision-protected reservations, but the old `FROZEN — direct-development` conclusion is superseded. `docs/f57-migration-baseline.v1.tsv` now freezes the 78-row pre-F57 baseline/absence partition and its current SHA-256; G0 Task 1 validates it against this 388-row catalog and the 310-row legacy disposition seed. G0 Task 6 is the only authorized future rebaseline mutation: it may correct exactly three unpublished draft SQL files in place, prove the 69-file executable baseline on disposable PostgreSQL 16, generate the apply manifest, and only then promote those three catalog rows. No new F57 SQL may be created before `G0_BOOTSTRAP_GREEN`.

This file is the single source of truth for concrete database migration versions and paths. The global runner orders every schema directory by the 14-digit `YYYYMMDDHHMMSS` version.

Governance rules:

- `EXISTING` rows correspond one-for-one to files already present under `db/migrations/` and already matching the frozen target; their versions, slugs, paths, and contents are immutable.
- `PLANNED` rows normally reserve a version, slug, and owner path for implementation, with the SQL file absent until implementation. Narrow direct-development exception: `20260901091500`、`20260901092000`、`20261012090500` files already exist but are stale pre-release drafts; their path/version stay fixed while their contents must be revised in the first implementation batch, and they remain PLANNED until fresh-PostgreSQL `pg_catalog` verification passes. Catalog prose or file presence alone can never promote them to EXISTING.
- `docs/f57-migration-baseline.v1.tsv` partitions exactly 78 catalog rows as 66 `APPLY_IMMUTABLE_BASELINE`, three `REWRITE_THEN_APPLY`, seven `ABSENT_SUPERSEDED_BY_F57`, and two `ABSENT_DEFERRED_WITH_INTERFACE`; its SHA-256 is `52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd`. Together with the 310-row legacy seed this exact-joins all 388 pre-F57 catalog rows. The 66 immutable + three corrected drafts are the only executable baseline; all other 319 pre-F57 `PLANNED` rows remain absent.
- Before G0 Task 6, the only legal state is this file's current `66 EXISTING + 322 PLANNED = 388`, the three drafts at registered preimage hashes, and no apply manifest. After Task 6, the only legal state is `69 EXISTING + 319 PLANNED = 388`, all three drafts at apply-manifest postimage hashes, and a deterministic apply manifest bound by the clean-candidate G0 receipt. Any mixed catalog/byte/manifest state is invalid.
- After this development-readiness freeze, new migrations append after the last reserved version `20261024090800` with a valid later Gregorian timestamp. The stage-local catch-up slots already named below—including `20261017093630`, `20261020090130`, `20261023092500`, and `20261023092600`—and the F-55 block `20261024090000`–`20261024090800` are part of this freeze, not permission to insert another historical slot. Never reuse a version or slug, and never renumber an existing or reserved row.
- A placeholder such as `V<YYYYMMDDHHMMSS>__schema_action.sql` is illustrative only and is not a reservation.
- Cross-schema work is stored under the owner of the primary object being created or changed. A later stage that appends rows to a shared registry uses a stage-specific slug; it never reuses an earlier backfill filename.
- F-57 Task 1 不得重新判断 310 个缺失的旧 `PLANNED` 行。它们的版本、路径、固定处置、唯一聚合 owner task、F-57 替代路径和映射规则已逐行冻结在 [`f57-legacy-migration-disposition.seed.tsv`](f57-legacy-migration-disposition.seed.tsv)；该种子必须与本目录 exact-join 为 310/310，未知、重复、缺失、额外或手工分配一律失败。本文在 Task 1 实际执行前仍诚实保留旧状态文字，种子只关闭分配歧义，不伪称迁移已重分类或 SQL 已实现。

Current pre-G0 catalog cardinality: **66 EXISTING + 322 PLANNED = 388 total**. Planned post-G0 Task 6 cardinality: **69 EXISTING + 319 PLANNED = 388 total**.

## Existing migrations and named pre-created target revisions

| Version | Path | Owner / phase | Status |
|---|---|---|---|
| `20260901090000` | `db/migrations/platform_core/V20260901090000__platform_core_create_schema.sql` | `platform_core` / Stage 2 schema bootstrap | EXISTING |
| `20260901090500` | `db/migrations/platform_core/V20260901090500__platform_core_conventions.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901091000` | `db/migrations/platform_core/V20260901091000__platform_core_legal_entities.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901091500` | `db/migrations/platform_core/V20260901091500__platform_core_key_domains.sql` | `platform_core` / Stage 2 target revision；物理文件已存在但仍是过期草案，首批开发必须原路径修订且 fresh 验证前不得宣称完成。目标保持 `kek_ref/kek_version NOT NULL`，`kek_version` CHECK=1..2,147,483,647；首版 locator exact=`kms://ep/v1/deploy/<lowercase-deployment-uuid>/domain/<lowercase-key-domain-uuid>/kek/1`，不编码 provider。行内 CHECK 只能验证 grammar、embedded domain UUID=本行 id、尾段=本行 kek_version；deployment UUID 与签名 manifest 的相等由 `KeyDomainProvisioner`/bootstrap/Stage14 应用证据验证，不可伪称 SQL CHECK 能查外部值。Rust u32 超 i32::MAX 在 cast 前拒绝。PROVISIONING locator 非 KMS 存在证明；状态/FK/唯一键必须齐备。零域才 NOT_PROVISIONED；行已 PROVISIONING 后供给失败统一 KEY_UNAVAILABLE | PLANNED |
| `20260901092000` | `db/migrations/platform_core/V20260901092000__platform_core_data_keys.sql` | `platform_core` / Stage 2 pre-release target rewrite；物理文件已存在但仍是未应用的过期 `PLANNED` 草案，首批开发须在 registry rebaseline 后原路径改写并 fresh 验证，这不是改写已应用迁移历史。目标只用复合 FK `(legal_entity_id,key_domain_id)->key_domains(legal_entity_id,id) RESTRICT`；purpose↔algorithm 为 FIELD/ATTACHMENT/ARCHIVE→AES_256_GCM、BLIND_INDEX→HMAC_SHA256；`version` CHECK=1..65535，current=65535 轮换以 TRANSITION_INVALID 失败；四态时间 shape 保持精确。删除单数旧列，exact 新增八个 NOT NULL 列 `operational_wrapped_key,operational_wrap_key_version,operational_recipient_ref,recovery_wrapped_key,recovery_wrap_key_version,recovery_recipient_ref,wrap_context_generation,wrap_envelope_version`；两份 bytes 正长度，四个 version/generation 均正且不超过 i32::MAX，两个 canonical recipient 非空且不同。每份信封分别绑定 deployment/legal entity/purpose/data-key id/version/context generation/recipient/envelope version；任一正确路径恢复同一 DEK，不是 2-of-2。DEFERRABLE graph 保证每 tuple 至多一 ACTIVE、domain ACTIVE 时 exact 16。激活事务插 16 rows、状态推进并写含双信封摘要/binding 的唯一 activation event；STANDARD null/INITIAL non-null，按 purpose/scope 排序。日常 readback 只经 operational 路径；recovery 只经离线 `PIV_SHAMIR_2_OF_3_V1`（固定 3 份 share、任意 2 份重构）；A-04 每 purpose 一次轮换四 scope | PLANNED |
| `20260901092500` | `db/migrations/platform_core/V20260901092500__platform_core_sensitive_field_registry.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901093000` | `db/migrations/platform_core/V20260901093000__platform_core_append_only_registry.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901093500` | `db/migrations/platform_core/V20260901093500__platform_core_migration_windows.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901094000` | `db/migrations/platform_core/V20260901094000__platform_core_enterprise_groups.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901094500` | `db/migrations/platform_core/V20260901094500__platform_core_organizations.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901095000` | `db/migrations/platform_core/V20260901095000__platform_core_departments.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901095500` | `db/migrations/platform_core/V20260901095500__platform_core_positions.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901100000` | `db/migrations/platform_core/V20260901100000__platform_core_department_closures.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901100200` | `db/migrations/platform_core/V20260901100200__platform_core_unpoliced_table_registry.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901100500` | `db/migrations/platform_core/V20260901100500__platform_core_grants.sql` | `platform_core` / Stage 2 data foundation | EXISTING |
| `20260901101000` | `db/migrations/platform_authz/V20260901101000__platform_authz_create_schema.sql` | `platform_authz` / Stage 2 schema bootstrap | EXISTING |
| `20260901101500` | `db/migrations/platform_meta/V20260901101500__platform_meta_create_schema.sql` | `platform_meta` / Stage 2 schema bootstrap | EXISTING |
| `20260901102000` | `db/migrations/platform_flow/V20260901102000__platform_flow_create_schema.sql` | `platform_flow` / Stage 2 schema bootstrap | EXISTING |
| `20260901102500` | `db/migrations/platform_audit/V20260901102500__platform_audit_create_schema.sql` | `platform_audit` / Stage 2 schema bootstrap | EXISTING |
| `20260901103000` | `db/migrations/platform_msg/V20260901103000__platform_msg_create_schema.sql` | `platform_msg` / Stage 2 schema bootstrap | EXISTING |
| `20260901103500` | `db/migrations/platform_file/V20260901103500__platform_file_create_schema.sql` | `platform_file` / Stage 2 schema bootstrap | EXISTING |
| `20260901104000` | `db/migrations/platform_ops/V20260901104000__platform_ops_create_schema.sql` | `platform_ops` / Stage 2 schema bootstrap | EXISTING |
| `20260901104500` | `db/migrations/platform_ops/V20260901104500__platform_ops_create_degradation_windows.sql` | `platform_ops` / Stage 2 data foundation | EXISTING |
| `20260901105000` | `db/migrations/ext/V20260901105000__ext_create_schema.sql` | `ext` / Stage 2 schema bootstrap | EXISTING |
| `20260901110000` | `db/migrations/mdm/V20260901110000__mdm_create_schema.sql` | `mdm` / Stage 2 schema bootstrap | EXISTING |
| `20260901110500` | `db/migrations/crm/V20260901110500__crm_create_schema.sql` | `crm` / Stage 2 schema bootstrap | EXISTING |
| `20260901111000` | `db/migrations/cpq/V20260901111000__cpq_create_schema.sql` | `cpq` / Stage 2 schema bootstrap | EXISTING |
| `20260901111500` | `db/migrations/clm/V20260901111500__clm_create_schema.sql` | `clm` / Stage 2 schema bootstrap | EXISTING |
| `20260901112000` | `db/migrations/sales/V20260901112000__sales_create_schema.sql` | `sales` / Stage 2 schema bootstrap | EXISTING |
| `20260901112500` | `db/migrations/procure/V20260901112500__procure_create_schema.sql` | `procure` / Stage 2 schema bootstrap | EXISTING |
| `20260901113000` | `db/migrations/inventory/V20260901113000__inventory_create_schema.sql` | `inventory` / Stage 2 schema bootstrap | EXISTING |
| `20260901113500` | `db/migrations/costing/V20260901113500__costing_create_schema.sql` | `costing` / Stage 2 schema bootstrap | EXISTING |
| `20260901114000` | `db/migrations/project/V20260901114000__project_create_schema.sql` | `project` / Stage 2 schema bootstrap | EXISTING |
| `20260901114500` | `db/migrations/service/V20260901114500__service_create_schema.sql` | `service` / Stage 2 schema bootstrap | EXISTING |
| `20260901115000` | `db/migrations/finance/V20260901115000__finance_create_schema.sql` | `finance` / Stage 2 schema bootstrap | EXISTING |
| `20260901115500` | `db/migrations/ledger/V20260901115500__ledger_create_schema.sql` | `ledger` / Stage 2 schema bootstrap | EXISTING |
| `20260901120000` | `db/migrations/invoice/V20260901120000__invoice_create_schema.sql` | `invoice` / Stage 2 schema bootstrap | EXISTING |
| `20260901120500` | `db/migrations/portal/V20260901120500__portal_create_schema.sql` | `portal` / Stage 2 schema bootstrap | EXISTING |
| `20260901121000` | `db/migrations/reporting/V20260901121000__reporting_create_schema.sql` | `reporting` / Stage 2 schema bootstrap | EXISTING |
| `20260915090000` | `db/migrations/platform_msg/V20260915090000__platform_msg_create_idempotency_keys.sql` | `platform_msg` / Stage 3a platform kernel | EXISTING |
| `20261012090000` | `db/migrations/platform_core/V20261012090000__platform_core_identity_user_accounts.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012090500` | `db/migrations/platform_core/V20261012090500__platform_core_identity_user_credentials.sql` | `platform_core` / Stage 4 target revision；物理文件仍含过期 `secret_ref`，首批开发必须原路径修订：删除该列，新增 `secret_enc bytea null`、`secret_key_ref text null`、`last_used_counter bigint null`；TOTP only exact one-of、counter NULL→严格单调、FIELD/L40 EPC1 与 pseudo-column AAD 按数据字典 §6.14。fresh `pg_catalog` 证明旧列不存在及新列/类型/CHECK 全齐前保持 PLANNED | PLANNED |
| `20261012091000` | `db/migrations/platform_core/V20261012091000__platform_core_identity_user_password_history.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012091500` | `db/migrations/platform_core/V20261012091500__platform_core_identity_user_devices.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012092000` | `db/migrations/platform_core/V20261012092000__platform_core_identity_sessions.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012092500` | `db/migrations/platform_core/V20261012092500__platform_core_identity_reauth_challenges.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012093000` | `db/migrations/platform_core/V20261012093000__platform_core_identity_login_attempts.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012093500` | `db/migrations/platform_core/V20261012093500__platform_core_identity_account_lockouts.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012094000` | `db/migrations/platform_core/V20261012094000__platform_core_identity_breakglass_activations.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012094500` | `db/migrations/platform_core/V20261012094500__platform_core_backfill_system_principal_account.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |
| `20261012100000` | `db/migrations/platform_authz/V20261012100000__platform_authz_permission_items.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012100500` | `db/migrations/platform_authz/V20261012100500__platform_authz_object_scope_bindings.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012101000` | `db/migrations/platform_authz/V20261012101000__platform_authz_roles.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012101500` | `db/migrations/platform_authz/V20261012101500__platform_authz_role_permission_grants.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012102000` | `db/migrations/platform_authz/V20261012102000__platform_authz_access_policies.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012102500` | `db/migrations/platform_authz/V20261012102500__platform_authz_field_permissions.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012103500` | `db/migrations/platform_authz/V20261012103500__platform_authz_user_legal_entity_grants.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012104000` | `db/migrations/platform_authz/V20261012104000__platform_authz_user_role_grants.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012104500` | `db/migrations/platform_authz/V20261012104500__platform_authz_user_org_assignments.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012105000` | `db/migrations/platform_authz/V20261012105000__platform_authz_user_scope_grants.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012105500` | `db/migrations/platform_authz/V20261012105500__platform_authz_sod_rules.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012110000` | `db/migrations/platform_authz/V20261012110000__platform_authz_approval_chains.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012110500` | `db/migrations/platform_authz/V20261012110500__platform_authz_approval_chain_nodes.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012111000` | `db/migrations/platform_authz/V20261012111000__platform_authz_high_risk_requests.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012111500` | `db/migrations/platform_authz/V20261012111500__platform_authz_authz_config_versions.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012112000` | `db/migrations/platform_authz/V20261012112000__platform_authz_backfill_permission_item_seed.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012112500` | `db/migrations/platform_authz/V20261012112500__platform_authz_backfill_admin_duty_roles.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012113000` | `db/migrations/platform_authz/V20261012113000__platform_authz_backfill_default_sod_rules.sql` | `platform_authz` / Stage 4 identity and authorization | EXISTING |
| `20261012113500` | `db/migrations/platform_core/V20261012113500__platform_core_backfill_unpoliced_table_registry.sql` | `platform_core` / Stage 4 identity and authorization | EXISTING |

## Planned reservations

| Version | Path | Owner / phase | Status |
|---|---|---|---|
| `20261012114000` | `db/migrations/platform_core/V20261012114000__platform_core_alter_reauth_challenges_dual_kind.sql` | `platform_core` / Stage 4 identity and authorization catch-up | PLANNED |
| `20261012114500` | `db/migrations/platform_core/V20261012114500__platform_core_add_identity_foreign_keys.sql` | `platform_core` / Stage 4 identity and authorization catch-up | PLANNED |
| `20261012115000` | `db/migrations/platform_core/V20261012115000__platform_core_drop_user_accounts_supplier_ref_id.sql` | `platform_core` / Stage 4 identity and authorization catch-up | PLANNED |
| `20261012115500` | `db/migrations/platform_authz/V20261012115500__platform_authz_add_missing_foreign_keys.sql` | `platform_authz` / Stage 4 identity and authorization catch-up | PLANNED |
| `20261012120000` | `db/migrations/platform_authz/V20261012120000__platform_authz_alter_approval_scenario_constraints.sql` | `platform_authz` / Stage 4 approval scenario catch-up; 37-value closed set including EXTENSION_ENABLE | PLANNED |
| `20261012120500` | `db/migrations/platform_authz/V20261012120500__platform_authz_backfill_default_approval_chains.sql` | `platform_authz` / Stage 4 default approval chain catch-up | PLANNED |
| `20261013090000` | `db/migrations/platform_core/V20261013090000__platform_core_create_number_sequences.sql` | `platform_core` / Stage 3b platform foundation | PLANNED |
| `20261013090100` | `db/migrations/platform_core/V20261013090100__platform_core_create_module_registrations.sql` | `platform_core` / Stage 3b F-56 module projection；`module_registrations` 自有列精确为 `module_code,display_name,install_state,installed_at,state_changed_at,package_id,package_code,package_version_major,package_version_minor,package_version_patch,package_payload_sha256,package_signature,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,enabled_at,disabled_at,last_transition_reason`，另加可更新公共列；digest raw32，signer CHECK=`spki-sha256:<64 lowerhex>`，`module_contract_version between 1 and 2147483647`（installed 态；Rust u32 入库前 checked conversion），NOT_INSTALLED 空投影/installed 完整投影，source item unique，FK defer=`20261013093300`，零 executable/path/url/attachment 列。fresh 同一迁移以 SYSTEM、row_version=1 原子 seed 恰 15 行且全部 NOT_INSTALLED/其余投影 null：`00000000-0000-7000-8000-000000000601/mdm/主数据管理`、`00000000-0000-7000-8000-000000000602/crm/客户关系管理`、`00000000-0000-7000-8000-000000000603/cpq/配置、定价与报价`、`00000000-0000-7000-8000-000000000604/clm/合同生命周期管理`、`00000000-0000-7000-8000-000000000605/sales/销售与订单`、`00000000-0000-7000-8000-000000000606/procure/采购管理`、`00000000-0000-7000-8000-000000000607/inventory/库存管理`、`00000000-0000-7000-8000-000000000608/costing/成本管理`、`00000000-0000-7000-8000-000000000609/project/项目管理`、`00000000-0000-7000-8000-000000000610/service/售后服务`、`00000000-0000-7000-8000-000000000611/finance/收付款与往来`、`00000000-0000-7000-8000-000000000612/ledger/总账与结账`、`00000000-0000-7000-8000-000000000613/invoice/发票管理`、`00000000-0000-7000-8000-000000000614/portal/供应商门户`、`00000000-0000-7000-8000-000000000615/reporting/报表与分析`；不可删/改 catalog 或加第16行。15 descriptor/schema 与签名 `product-modules.v1.jcs` 是文件/构建证据，不为其加列或迁移 | PLANNED |
| `20261013090200` | `db/migrations/platform_core/V20261013090200__platform_core_create_license_grants.sql` | `platform_core` / Stage 3b F-56 license projection；`license_grants.id=grant_id`，自有列精确为 `license_no,deployment_id,governance_legal_entity_id,issued_to,license_kind,issued_at,valid_from,valid_to,maintenance_valid_to,legal_entity_scope,legal_entity_ids,legal_entity_limit,named_user_limit,registered_device_limit,module_codes,entitlement_codes,payload_sha256,signature,signer_subject,trust_bundle_sha256,supersedes_grant_id,superseded_at,current_slot,last_trusted_at,revoked_at,revocation_id,revocation_issued_at,revocation_reason_code,revocation_payload_sha256,revocation_signature,revocation_signer_subject,grant_source_config_package_id,grant_source_config_item_id,revocation_source_config_package_id,revocation_source_config_item_id`，另加可更新公共列；`governance_legal_entity_id -> legal_entities(id) ON DELETE RESTRICT`，LIST CHECK 必含该值；digest raw32，两个 signer CHECK=`spki-sha256:<64 lowerhex>`；CHECK=kind/date/scope/limit/signature/current/revocation shape，unique=current slot/revocation id/两 source item，自 FK supersedes RESTRICT，source/治理图 defer=`20261013093300`；license_no 非唯一、零 current 合法且首张接受后恰一、无持久化 status/usage snapshot；trust digest 是首次接受摘要，不增 revocation/module 摘要列或 usage 表。固定 advisory lock、TrustedClockV1/readiness+special+target cadence≤240s、240-second slot checkpoint CAS、CRL与全部 RELEASED special exact-set CAB 重验属于 applier/运行门禁 | PLANNED |
| `20261013090300` | `db/migrations/platform_core/V20261013090300__platform_core_create_feature_flags.sql` | `platform_core` / Stage 3b platform foundation | PLANNED |
| `20261013090400` | `db/migrations/platform_meta/V20261013090400__platform_meta_create_config_packages.sql` | `platform_meta` / Stage 3b config packages；终态基础列/六态与审批证据一次建齐，`signer_subject` 非空时 CHECK=`spki-sha256:<64 lowerhex>`，display DN 不持久化为身份；Stage13 只扩 autotest/十一态 | PLANNED |
| `20261013090500` | `db/migrations/platform_meta/V20261013090500__platform_meta_create_config_package_items.sql` | `platform_meta` / Stage 3b config items；Rust `ItemKind::ALL` 与 `ck_config_package_items_item_kind` 同序恰为18（前16加 `LICENSE_GRANT`、`MODULE_PACKAGE`）；随建表增加 `accepted_trust_bundle_sha256 bytea NULL` 与 null-or-32/普通项恒空行内 CHECK，特殊项只收 IMPORTED 单项 ADD/null-before/empty-entity-scope；item hash 保持 ADD/MODIFY=SHA256(JCS(after_spec))、REMOVE=SHA256(JCS(before_spec)) 且禁 null；本迁移不建 `UNIQUE(config_package_id,id)`，终态20由既定 `20261022090500` 追加 MCP 两项 | PLANNED |
| `20261013090600` | `db/migrations/platform_meta/V20261013090600__platform_meta_create_config_release_orders.sql` | `platform_meta` / Stage 3b platform foundation | PLANNED |
| `20261013090700` | `db/migrations/platform_flow/V20261013090700__platform_flow_create_process_definitions.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013090800` | `db/migrations/platform_flow/V20261013090800__platform_flow_create_process_instances.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013090900` | `db/migrations/platform_flow/V20261013090900__platform_flow_create_process_steps.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013091000` | `db/migrations/platform_flow/V20261013091000__platform_flow_create_process_tasks.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013091100` | `db/migrations/platform_flow/V20261013091100__platform_flow_create_process_timers.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013091200` | `db/migrations/platform_flow/V20261013091200__platform_flow_create_process_compensations.sql` | `platform_flow` / Stage 3b platform foundation | PLANNED |
| `20261013091300` | `db/migrations/platform_audit/V20261013091300__platform_audit_create_audit_segments.sql` | `platform_audit` / Stage 3b platform foundation | PLANNED |
| `20261013091400` | `db/migrations/platform_audit/V20261013091400__platform_audit_create_audit_events.sql` | `platform_audit` / Stage 3b platform foundation | PLANNED |
| `20261013091500` | `db/migrations/platform_audit/V20261013091500__platform_audit_create_audit_anchors.sql` | `platform_audit` / Stage 3b platform foundation | PLANNED |
| `20261013091600` | `db/migrations/platform_audit/V20261013091600__platform_audit_create_audit_verifications.sql` | `platform_audit` / Stage 3b platform foundation | PLANNED |
| `20261013091700` | `db/migrations/platform_msg/V20261013091700__platform_msg_create_outbox_events.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013091800` | `db/migrations/platform_msg/V20261013091800__platform_msg_create_inbox_consumptions.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013091900` | `db/migrations/platform_msg/V20261013091900__platform_msg_create_dead_letters.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092000` | `db/migrations/platform_msg/V20261013092000__platform_msg_create_notification_templates.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092100` | `db/migrations/platform_msg/V20261013092100__platform_msg_create_notifications.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092200` | `db/migrations/platform_msg/V20261013092200__platform_msg_create_notification_deliveries.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092300` | `db/migrations/platform_msg/V20261013092300__platform_msg_create_push_registrations.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092400` | `db/migrations/platform_msg/V20261013092400__platform_msg_create_ops_views.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092500` | `db/migrations/platform_msg/V20261013092500__platform_msg_backfill_append_only_registry.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092600` | `db/migrations/platform_msg/V20261013092600__platform_msg_backfill_sensitive_field_registry.sql` | `platform_msg` / Stage 3b platform foundation | PLANNED |
| `20261013092700` | `db/migrations/platform_file/V20261013092700__platform_file_create_attachment_objects.sql` | `platform_file` / Stage 3b platform foundation | PLANNED |
| `20261013092800` | `db/migrations/platform_file/V20261013092800__platform_file_create_attachment_versions.sql` | `platform_file` / Stage 3b；`attachment_versions.dek_ref text not null` 保存 `DataKeyHandleV1::canonical_ref()`，wire exact `data-key://<lowercase-data-key-uuid>#<u16非零无前导零版本>`；同版本 EPA1 全 chunk 固定同一 pinned ref，续传/Range 只以 ExactRef 重开，禁止读取时漂到 current | PLANNED |
| `20261013092900` | `db/migrations/platform_file/V20261013092900__platform_file_create_upload_sessions.sql` | `platform_file` / Stage 3b platform foundation | PLANNED |
| `20261013093000` | `db/migrations/platform_file/V20261013093000__platform_file_create_upload_parts.sql` | `platform_file` / Stage 3b platform foundation | PLANNED |
| `20261013093100` | `db/migrations/platform_file/V20261013093100__platform_file_create_scan_results.sql` | `platform_file` / Stage 3b platform foundation | PLANNED |
| `20261013093200` | `db/migrations/platform_file/V20261013093200__platform_file_create_watermark_views.sql` | `platform_file` / Stage 3b platform foundation | PLANNED |
| `20261013093300` | `db/migrations/platform_core/V20261013093300__platform_core_backfill_stage03_unpoliced_table_registry.sql` | `platform_core` / Stage 3b；仍幂等登记 module_registrations/license_grants/feature_flags/config_packages/config_package_items/config_release_orders 六张部署级表；本迁移才给 `config_package_items` 加 `UNIQUE(config_package_id,id)`，再精确添加 module/grant/revocation 各 package+同包 item 共六条 `ON DELETE RESTRICT` source FK。安装 DEFERRABLE INITIALLY DEFERRED F-56 graph 到 `config_packages/config_package_items/module_registrations/license_grants/legal_entities` 五表：COMMIT 强制 ordinary摘要恒NULL、special未RELEASED=NULL/RELEASED=32、非空不可改清、grant摘要等source；同 deployment 最早 RELEASED grant 唯一冻结治理法人、后继相等、LIST包含、PENDING_APPROVAL 及以后 special approval 法人唯一派生且治理法人不得停用；全部 RELEASED MODULE_PACKAGE history 中 `package_id -> exact inner` 与 `(module_code,package_code,semver) -> 同package_id/exact inner` 两映射一一。三项 source-item unique 与 revocation source 空形状由090100/090200建立，不新增表/列/迁移号。迁移全成后 fresh-production `ep-migrate apply` 三参数只读/写 canonical lowercase deployment UUID 子目录固定根 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\` 的三固定文件并按 exact PROTECTED DACL/safe-handle 运行；bootstrap 事务在角色绑定前先建 SYSTEM/CONFIG_OPERATOR/SECURITY_APPROVER 恰三条同法人 active `user_legal_entity_grants`，receipt/audit 绑定三 id/exact mapping。数据库只提交 signed `key_domain_id`、exact `/kek/1` locator 的 PROVISIONING 域；core readiness 前为 exact 16 tuple 生成双 recipient 信封并完成 operational readback 后同事务 ACTIVE 与唯一 `platform.key_domain.activated.v1`（INITIAL_GOVERNANCE + bootstrap_id，exact KEK fingerprint/16 rows 双信封 payload），receipt 无 KMS sidecar；这属于部署门禁而非新迁移 | PLANNED |
| `20261013093400` | `db/migrations/platform_core/V20261013093400__platform_core_create_impact_assessments.sql` | `platform_core` / Stage 3b platform foundation | PLANNED |
| `20261013093500` | `db/migrations/platform_core/V20261013093500__platform_core_create_impact_disposition_items.sql` | `platform_core` / Stage 3b platform foundation | PLANNED |
| `20261013093600` | `db/migrations/platform_flow/V20261013093600__platform_flow_create_approval_command_snapshots.sql` | `platform_flow` / Stage 3b high-confidential approval snapshots | PLANNED |
| `20261013093700` | `db/migrations/platform_flow/V20261013093700__platform_flow_backfill_sensitive_field_registry.sql` | `platform_flow` / Stage 3b high-confidential approval snapshots | PLANNED |
| `20261014090000` | `db/migrations/mdm/V20261014090000__mdm_create_uoms.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090100` | `db/migrations/mdm/V20261014090100__mdm_create_classification_items.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090200` | `db/migrations/mdm/V20261014090200__mdm_create_customers.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090300` | `db/migrations/mdm/V20261014090300__mdm_create_products.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090400` | `db/migrations/mdm/V20261014090400__mdm_enable_rls_t0.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090500` | `db/migrations/mdm/V20261014090500__mdm_create_materials.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090600` | `db/migrations/mdm/V20261014090600__mdm_create_customer_contacts.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090700` | `db/migrations/mdm/V20261014090700__mdm_create_customer_addresses.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090800` | `db/migrations/mdm/V20261014090800__mdm_create_customer_invoice_profiles.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014090900` | `db/migrations/mdm/V20261014090900__mdm_create_suppliers.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091000` | `db/migrations/mdm/V20261014091000__mdm_create_supplier_contacts.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091100` | `db/migrations/mdm/V20261014091100__mdm_create_supplier_payment_profiles.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091200` | `db/migrations/mdm/V20261014091200__mdm_create_supplier_qualifications.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091300` | `db/migrations/mdm/V20261014091300__mdm_create_supplier_price_records.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091400` | `db/migrations/mdm/V20261014091400__mdm_create_supplier_leadtime_records.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091500` | `db/migrations/mdm/V20261014091500__mdm_create_supplier_risk_records.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091600` | `db/migrations/mdm/V20261014091600__mdm_create_product_material_links.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091700` | `db/migrations/mdm/V20261014091700__mdm_create_change_requests.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091800` | `db/migrations/mdm/V20261014091800__mdm_create_record_versions.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014091900` | `db/migrations/mdm/V20261014091900__mdm_create_import_batches.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092000` | `db/migrations/mdm/V20261014092000__mdm_create_import_batch_rows.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092100` | `db/migrations/mdm/V20261014092100__mdm_create_export_jobs.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092200` | `db/migrations/mdm/V20261014092200__mdm_create_attachment_link_tables.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092300` | `db/migrations/mdm/V20261014092300__mdm_enable_rls_rest.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092400` | `db/migrations/mdm/V20261014092400__mdm_create_lookup_indexes.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092500` | `db/migrations/mdm/V20261014092500__mdm_backfill_sensitive_field_registry.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092600` | `db/migrations/mdm/V20261014092600__mdm_create_dataset_views.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092700` | `db/migrations/mdm/V20261014092700__mdm_create_warehouses.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092800` | `db/migrations/mdm/V20261014092800__mdm_enable_rls_warehouses.sql` | `mdm` / Stage 5 master data | PLANNED |
| `20261014092900` | `db/migrations/cpq/V20261014092900__cpq_create_price_lists.sql` | `cpq` / Stage 5 master data | PLANNED |
| `20261014093000` | `db/migrations/cpq/V20261014093000__cpq_create_price_list_lines.sql` | `cpq` / Stage 5 master data | PLANNED |
| `20261014093100` | `db/migrations/cpq/V20261014093100__cpq_create_price_list_customer_links.sql` | `cpq` / Stage 5 master data | PLANNED |
| `20261014093200` | `db/migrations/cpq/V20261014093200__cpq_enable_rls.sql` | `cpq` / Stage 5 master data | PLANNED |
| `20261014093300` | `db/migrations/cpq/V20261014093300__cpq_create_lookup_indexes.sql` | `cpq` / Stage 5 master data | PLANNED |
| `20261015090000` | `db/migrations/ledger/V20261015090000__ledger_create_accounts.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090100` | `db/migrations/ledger/V20261015090100__ledger_create_accounting_periods.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090110` | `db/migrations/platform_core/V20261015090110__platform_core_create_recon_check_definitions.sql` | `platform_core` / Stage 9a reconciliation foundation | PLANNED |
| `20261015090120` | `db/migrations/platform_core/V20261015090120__platform_core_create_recon_runs.sql` | `platform_core` / Stage 9a reconciliation foundation | PLANNED |
| `20261015090130` | `db/migrations/platform_core/V20261015090130__platform_core_create_recon_discrepancies.sql` | `platform_core` / Stage 9a reconciliation foundation | PLANNED |
| `20261015090200` | `db/migrations/ledger/V20261015090200__ledger_create_event_account_bindings.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090300` | `db/migrations/ledger/V20261015090300__ledger_create_opening_balance_batches.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090400` | `db/migrations/ledger/V20261015090400__ledger_create_opening_balance_batch_lines.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090500` | `db/migrations/ledger/V20261015090500__ledger_create_vouchers.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090600` | `db/migrations/ledger/V20261015090600__ledger_create_voucher_lines.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090700` | `db/migrations/ledger/V20261015090700__ledger_create_correction_vouchers.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090800` | `db/migrations/ledger/V20261015090800__ledger_create_correction_voucher_lines.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015090900` | `db/migrations/ledger/V20261015090900__ledger_create_account_period_balances.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015091300` | `db/migrations/ledger/V20261015091300__ledger_create_posting_trigger_event_types.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015091400` | `db/migrations/ledger/V20261015091400__ledger_create_ledger_views.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015091500` | `db/migrations/ledger/V20261015091500__ledger_backfill_posting_trigger_event_types.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015091600` | `db/migrations/ledger/V20261015091600__ledger_create_dataset_views.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261015091700` | `db/migrations/ledger/V20261015091700__ledger_backfill_append_only_registry.sql` | `ledger` / Stage 9a ledger foundation | PLANNED |
| `20261016090000` | `db/migrations/inventory/V20261016090000__inventory_create_stock_movements.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090100` | `db/migrations/inventory/V20261016090100__inventory_create_stock_qty_entries.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090200` | `db/migrations/inventory/V20261016090200__inventory_create_stock_value_entries.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090300` | `db/migrations/inventory/V20261016090300__inventory_create_variance_splits.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090400` | `db/migrations/inventory/V20261016090400__inventory_create_stock_qty_balances.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090500` | `db/migrations/inventory/V20261016090500__inventory_create_stock_value_balances.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090600` | `db/migrations/inventory/V20261016090600__inventory_create_variance_coverage_balances.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090700` | `db/migrations/inventory/V20261016090700__inventory_create_serial_states.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090800` | `db/migrations/inventory/V20261016090800__inventory_create_movement_serials.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016090900` | `db/migrations/inventory/V20261016090900__inventory_create_replenishment_policies.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016091000` | `db/migrations/inventory/concurrent/V20261016091000__inventory_create_report_indexes.sql` | `inventory` / Stage 8 inventory concurrent index | PLANNED |
| `20261016091100` | `db/migrations/inventory/V20261016091100__inventory_backfill_append_only_registry.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261016091200` | `db/migrations/inventory/V20261016091200__inventory_create_dataset_views.sql` | `inventory` / Stage 8 inventory | PLANNED |
| `20261017090000` | `db/migrations/cpq/V20261017090000__cpq_create_price_authorities.sql` | `cpq` / Stage 6 contract and sales | PLANNED |
| `20261017090100` | `db/migrations/clm/V20261017090100__clm_create_contract_types.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090200` | `db/migrations/clm/V20261017090200__clm_create_contract_templates.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090300` | `db/migrations/clm/V20261017090300__clm_create_clauses.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090400` | `db/migrations/clm/V20261017090400__clm_create_contracts.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090500` | `db/migrations/clm/V20261017090500__clm_create_contract_lines.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090600` | `db/migrations/clm/V20261017090600__clm_create_contract_terms.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090700` | `db/migrations/clm/V20261017090700__clm_create_contract_milestones.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090800` | `db/migrations/clm/V20261017090800__clm_create_contract_obligations.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017090900` | `db/migrations/clm/V20261017090900__clm_create_contract_payment_schedules.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091000` | `db/migrations/clm/V20261017091000__clm_create_contract_attachments.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091100` | `db/migrations/clm/V20261017091100__clm_create_contract_annotations.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091200` | `db/migrations/clm/V20261017091200__clm_create_contract_versions.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091300` | `db/migrations/clm/V20261017091300__clm_create_contract_approvals.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091400` | `db/migrations/clm/V20261017091400__clm_create_signature_requests.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091500` | `db/migrations/clm/V20261017091500__clm_create_signature_events.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091600` | `db/migrations/clm/V20261017091600__clm_create_seal_usages.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091700` | `db/migrations/clm/V20261017091700__clm_create_contract_derivations.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091800` | `db/migrations/clm/V20261017091800__clm_create_contract_validations.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017091900` | `db/migrations/clm/V20261017091900__clm_create_contract_merge_links.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017092000` | `db/migrations/clm/V20261017092000__clm_create_reminder_views.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017092100` | `db/migrations/clm/V20261017092100__clm_create_dataset_views.sql` | `clm` / Stage 6 contract and sales | PLANNED |
| `20261017092200` | `db/migrations/sales/V20261017092200__sales_create_credit_policies.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092300` | `db/migrations/sales/V20261017092300__sales_create_customer_credit_controls.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092400` | `db/migrations/sales/V20261017092400__sales_create_sales_orders.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092500` | `db/migrations/sales/V20261017092500__sales_create_sales_order_lines.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092600` | `db/migrations/sales/V20261017092600__sales_create_delivery_schedules.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092700` | `db/migrations/sales/V20261017092700__sales_create_delivery_confirmations.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092800` | `db/migrations/sales/V20261017092800__sales_create_delivery_confirmation_lines.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017092900` | `db/migrations/sales/V20261017092900__sales_create_sales_order_versions.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093000` | `db/migrations/sales/V20261017093000__sales_create_sales_order_changes.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093100` | `db/migrations/sales/V20261017093100__sales_create_sales_returns.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093200` | `db/migrations/sales/V20261017093200__sales_create_return_line_delivery_links.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093300` | `db/migrations/sales/V20261017093300__sales_create_exchange_links.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093400` | `db/migrations/sales/V20261017093400__sales_create_order_validations.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093500` | `db/migrations/sales/V20261017093500__sales_create_credit_exposure_view.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093600` | `db/migrations/sales/V20261017093600__sales_create_dataset_views.sql` | `sales` / Stage 6 contract and sales | PLANNED |
| `20261017093630` | `db/migrations/sales/V20261017093630__sales_backfill_append_only_registry.sql` | `sales` / Stage 6 return capture allocation registry | PLANNED |
| `20261017093700` | `db/migrations/clm/V20261017093700__clm_add_cross_schema_foreign_keys.sql` | `clm` / Stage 6 contract and sales catch-up | PLANNED |
| `20261018090000` | `db/migrations/procure/V20261018090000__procure_create_supplier_admissions.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090100` | `db/migrations/procure/V20261018090100__procure_create_supplier_quality_records.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090200` | `db/migrations/procure/V20261018090200__procure_create_purchase_requisitions.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090300` | `db/migrations/procure/V20261018090300__procure_create_purchase_orders.sql` | `procure` / Stage 7 procurement and portal；Stage 14 REVERSE 复用 audit 表，固定 owner action `PROCURE_PURCHASE_ORDER_MIGRATION_REVERSED`，无新增 DDL | PLANNED |
| `20261018090400` | `db/migrations/procure/V20261018090400__procure_create_purchase_order_lines.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090500` | `db/migrations/procure/V20261018090500__procure_create_purchase_order_line_batches.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090600` | `db/migrations/procure/V20261018090600__procure_create_purchase_order_payment_plans.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090700` | `db/migrations/procure/V20261018090700__procure_create_goods_receipts.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090800` | `db/migrations/procure/V20261018090800__procure_create_goods_receipt_lines.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018090900` | `db/migrations/procure/V20261018090900__procure_create_goods_receipt_line_serials.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091000` | `db/migrations/procure/V20261018091000__procure_create_goods_receipt_line_costings.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091100` | `db/migrations/procure/V20261018091100__procure_create_receipt_rejections.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091200` | `db/migrations/procure/V20261018091200__procure_create_purchase_returns.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091300` | `db/migrations/procure/V20261018091300__procure_create_purchase_return_lines.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091400` | `db/migrations/procure/V20261018091400__procure_create_purchase_return_line_serials.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091500` | `db/migrations/procure/V20261018091500__procure_create_payment_requests.sql` | `procure` / Stage 7 procurement and portal；Stage 14 REVERSE 复用 audit 表，固定 owner action `PROCURE_PAYMENT_REQUEST_MIGRATION_REVERSED`，无新增 DDL | PLANNED |
| `20261018091600` | `db/migrations/procure/V20261018091600__procure_create_payment_request_lines.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091700` | `db/migrations/procure/V20261018091700__procure_create_payable_reservations.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091800` | `db/migrations/procure/V20261018091800__procure_create_purchase_order_attachments.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018091900` | `db/migrations/procure/V20261018091900__procure_create_goods_receipt_attachments.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018092000` | `db/migrations/procure/V20261018092000__procure_create_purchase_return_attachments.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018092100` | `db/migrations/procure/V20261018092100__procure_create_payment_request_attachments.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018092200` | `db/migrations/procure/V20261018092200__procure_create_receipt_rejection_attachments.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018092300` | `db/migrations/procure/V20261018092300__procure_backfill_append_only_registry.sql` | `procure` / Stage 7 procurement and portal | PLANNED |
| `20261018092400` | `db/migrations/portal/V20261018092400__portal_create_supplier_portal_users.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018092500` | `db/migrations/portal/V20261018092500__portal_create_delivery_notices.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018092600` | `db/migrations/portal/V20261018092600__portal_create_delivery_notice_lines.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018092700` | `db/migrations/portal/V20261018092700__portal_create_delivery_notice_line_serials.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018092800` | `db/migrations/portal/V20261018092800__portal_create_delivery_notice_attachments.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018092900` | `db/migrations/portal/V20261018092900__portal_create_supplier_invoice_uploads.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018093000` | `db/migrations/portal/V20261018093000__portal_create_supplier_invoice_upload_lines.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018093100` | `db/migrations/portal/V20261018093100__portal_create_supplier_invoice_upload_attachments.sql` | `portal` / Stage 7 procurement and portal | PLANNED |
| `20261018093200` | `db/migrations/procure/V20261018093200__procure_add_portal_foreign_keys.sql` | `procure` / Stage 7 procurement and portal catch-up | PLANNED |
| `20261019090000` | `db/migrations/invoice/V20261019090000__invoice_create_tax_rate_options.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090100` | `db/migrations/invoice/V20261019090100__invoice_create_invoice_applications.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090200` | `db/migrations/invoice/V20261019090200__invoice_create_invoice_application_link_tables.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090300` | `db/migrations/invoice/V20261019090300__invoice_create_invoice_number_registry.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090400` | `db/migrations/invoice/V20261019090400__invoice_create_sales_invoices.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090500` | `db/migrations/invoice/V20261019090500__invoice_create_sales_invoice_lines.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090600` | `db/migrations/invoice/V20261019090600__invoice_create_invoice_reversals.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090700` | `db/migrations/invoice/V20261019090700__invoice_create_invoice_reversal_lines.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090800` | `db/migrations/invoice/V20261019090800__invoice_create_purchase_invoices.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090830` | `db/migrations/portal/V20261019090830__portal_add_invoice_foreign_keys.sql` | `portal` / Stage 10 invoice late-bound foreign keys | PLANNED |
| `20261019090900` | `db/migrations/invoice/V20261019090900__invoice_create_purchase_invoice_lines.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019090910` | `db/migrations/inventory/V20261019090910__inventory_add_invoice_foreign_keys.sql` | `inventory` / Stage 10 deferred invoice late-bound foreign keys | PLANNED |
| `20261019090930` | `db/migrations/procure/V20261019090930__procure_add_invoice_foreign_keys.sql` | `procure` / Stage 10 invoice late-bound foreign keys | PLANNED |
| `20261019091000` | `db/migrations/invoice/V20261019091000__invoice_create_invoice_receipt_plan_links.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091100` | `db/migrations/invoice/V20261019091100__invoice_create_invoice_import_batches.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091200` | `db/migrations/invoice/V20261019091200__invoice_create_attachment_link_tables.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091300` | `db/migrations/invoice/V20261019091300__invoice_enable_row_level_security.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091400` | `db/migrations/invoice/concurrent/V20261019091400__invoice_create_indexes.sql` | `invoice` / Stage 10 invoice concurrent index | PLANNED |
| `20261019091500` | `db/migrations/invoice/V20261019091500__invoice_backfill_seed_tax_rate_options.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091600` | `db/migrations/invoice/V20261019091600__invoice_create_dataset_views.sql` | `invoice` / Stage 10 invoice and finance | PLANNED |
| `20261019091700` | `db/migrations/finance/V20261019091700__finance_create_aging_bucket_definitions.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019091800` | `db/migrations/finance/V20261019091800__finance_create_cash_accounts.sql` | `finance` / Stage 10 invoice and finance；Stage 14 REVERSE 复用 audit 表，固定 owner action `FINANCE_CASH_ACCOUNT_MIGRATION_REVERSED`，无新增 DDL | PLANNED |
| `20261019091900` | `db/migrations/finance/V20261019091900__finance_create_receivable_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092000` | `db/migrations/finance/V20261019092000__finance_create_payable_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092100` | `db/migrations/finance/V20261019092100__finance_create_advance_receipt_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092200` | `db/migrations/finance/V20261019092200__finance_create_advance_payment_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092300` | `db/migrations/finance/V20261019092300__finance_create_unbilled_ar_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092400` | `db/migrations/finance/V20261019092400__finance_create_overbilling_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092430` | `db/migrations/invoice/V20261019092430__invoice_add_finance_foreign_keys.sql` | `invoice` / Stage 10 deferred invoice-finance foreign keys | PLANNED |
| `20261019092500` | `db/migrations/finance/V20261019092500__finance_create_overbilling_settlements.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092600` | `db/migrations/finance/V20261019092600__finance_create_receipts.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092700` | `db/migrations/finance/V20261019092700__finance_create_payments.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092800` | `db/migrations/finance/V20261019092800__finance_create_refunds.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019092900` | `db/migrations/finance/V20261019092900__finance_create_refund_source_payment_links.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093000` | `db/migrations/finance/V20261019093000__finance_create_cash_document_reversals.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093100` | `db/migrations/finance/V20261019093100__finance_create_settlement_link_tables.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093130` | `db/migrations/finance/V20261019093130__finance_add_deferred_foreign_keys.sql` | `finance` / Stage 10 deferred settlement foreign keys | PLANNED |
| `20261019093200` | `db/migrations/finance/V20261019093200__finance_create_cash_ledger_entries.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093300` | `db/migrations/finance/V20261019093300__finance_create_attachment_link_tables.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093400` | `db/migrations/finance/V20261019093400__finance_enable_row_level_security.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093500` | `db/migrations/finance/concurrent/V20261019093500__finance_create_indexes.sql` | `finance` / Stage 10 finance concurrent index | PLANNED |
| `20261019093600` | `db/migrations/finance/V20261019093600__finance_create_reconciliation_views.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093700` | `db/migrations/finance/V20261019093700__finance_backfill_seed_aging_buckets.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093800` | `db/migrations/finance/V20261019093800__finance_create_dataset_views.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019093900` | `db/migrations/finance/V20261019093900__finance_backfill_append_only_registry.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261019094000` | `db/migrations/finance/V20261019094000__finance_backfill_sensitive_field_registry.sql` | `finance` / Stage 10 invoice and finance | PLANNED |
| `20261020090000` | `db/migrations/costing/V20261020090000__costing_create_cost_entries.sql` | `costing` / Stage 11 costing and reporting | PLANNED |
| `20261020090100` | `db/migrations/costing/V20261020090100__costing_create_revenue_entries.sql` | `costing` / Stage 11 costing and reporting | PLANNED |
| `20261020090130` | `db/migrations/sales/V20261020090130__sales_add_costing_capture_foreign_keys.sql` | `sales` / Stage 11 return capture catch-up | PLANNED |
| `20261020090200` | `db/migrations/costing/V20261020090200__costing_create_dataset_views.sql` | `costing` / Stage 11 costing and reporting | PLANNED |
| `20261020090300` | `db/migrations/costing/V20261020090300__costing_grant_analyst_ro.sql` | `costing` / Stage 11 costing and reporting | PLANNED |
| `20261020090400` | `db/migrations/reporting/V20261020090400__reporting_backfill_seed_datasets.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020090500` | `db/migrations/reporting/V20261020090500__reporting_backfill_seed_dataset_fields.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020090600` | `db/migrations/reporting/V20261020090600__reporting_create_datasets.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020090700` | `db/migrations/reporting/V20261020090700__reporting_create_dataset_fields.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020090800` | `db/migrations/reporting/V20261020090800__reporting_create_report_objects.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020090900` | `db/migrations/reporting/V20261020090900__reporting_create_report_object_versions.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091000` | `db/migrations/reporting/V20261020091000__reporting_create_report_object_publications.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091100` | `db/migrations/reporting/V20261020091100__reporting_create_report_object_dependencies.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091200` | `db/migrations/reporting/V20261020091200__reporting_create_aging_bucket_profiles.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091300` | `db/migrations/reporting/V20261020091300__reporting_create_aging_bucket_lines.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091400` | `db/migrations/reporting/V20261020091400__reporting_create_render_tasks.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091500` | `db/migrations/reporting/V20261020091500__reporting_grant_analyst_ro.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091600` | `db/migrations/reporting/V20261020091600__reporting_backfill_migrate_aging_buckets_from_finance.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091700` | `db/migrations/reporting/V20261020091700__reporting_drop_finance_aging_bucket_definitions.sql` | `reporting` / Stage 11 costing and reporting | PLANNED |
| `20261020091800` | `db/migrations/platform_core/V20261020091800__platform_core_backfill_stage11_unpoliced_table_registry.sql` | `platform_core` / Stage 11 costing and reporting | PLANNED |
| `20261020091900` | `db/migrations/ledger/V20261020091900__ledger_create_close_serialization_slots.sql` | `ledger` / Stage 9b；slot generated active key、非部分证据候选键与后置图已拆/无 active 指针时的 RESTRICT down | PLANNED |
| `20261020092000` | `db/migrations/ledger/V20261020092000__ledger_create_period_close_requests.sql` | `ledger` / Stage 9b；独立取消证据、request generated keys、request↔slot/PASSED↔CLOSED 双向 deferred 长 FK、三表状态图与严格 13→12→11 可恢复 down | PLANNED |
| `20261020092100` | `db/migrations/ledger/V20261020092100__ledger_create_year_end_closings.sql` | `ledger` / Stage 9b；五态/末期/失败证据、期间身份 guard、0/1/2 年结凭证与余额 deferred 图、可恢复 down | PLANNED |
| `20261021090000` | `db/migrations/project/V20261021090000__project_create_projects.sql` | `project` / Stage 12；建 projects + APPEND_ONLY project_migration_corrections、同法人 root FK/唯一键/shape；down detach→registry delete→drop correction→drop root，project 最终效果图随 090100 安装 | PLANNED |
| `20261021090030` | `db/migrations/procure/V20261021090030__procure_add_project_foreign_keys.sql` | `procure` / Stage 12 project late-bound foreign keys | PLANNED |
| `20261021090040` | `db/migrations/costing/V20261021090040__costing_add_project_foreign_keys.sql` | `costing` / Stage 12 project late-bound foreign keys | PLANNED |
| `20261021090100` | `db/migrations/project/V20261021090100__project_create_project_tasks.sql` | `project` / Stage 12；建 tasks 并安装 project_migration_corrections 的 DEFERRABLE 根/task 最终效果图；down 先删图再 drop tasks | PLANNED |
| `20261021090200` | `db/migrations/project/V20261021090200__project_create_task_requisition_links.sql` | `project` / Stage 12 project and service；含 task/link 双表 `DEFERRABLE INITIALLY DEFERRED` 状态—基数约束触发器 | PLANNED |
| `20261021090300` | `db/migrations/project/V20261021090300__project_create_attachment_links.sql` | `project` / Stage 12 project and service | PLANNED |
| `20261021090400` | `db/migrations/project/V20261021090400__project_create_dataset_views.sql` | `project` / Stage 12 project and service | PLANNED |
| `20261021090500` | `db/migrations/service/V20261021090500__service_create_dictionaries.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261021090600` | `db/migrations/service/V20261021090600__service_create_equipment_records.sql` | `service` / Stage 12；建 equipment_records + APPEND_ONLY equipment_migration_corrections、root/状态字典 FK 与 DEFERRABLE 终态图；down 先 detach/删 registry/图再 drop correction/root | PLANNED |
| `20261021090700` | `db/migrations/service/V20261021090700__service_create_customer_complaints.sql` | `service` / Stage 12；建 customer_complaints + APPEND_ONLY customer_complaint_migration_corrections、root FK 与 DEFERRABLE 终态图；down 先 detach/删 registry/图再 drop correction/root | PLANNED |
| `20261021090800` | `db/migrations/service/V20261021090800__service_create_work_orders.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261021090900` | `db/migrations/service/V20261021090900__service_create_work_order_lines.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261021091000` | `db/migrations/service/V20261021091000__service_create_work_order_logs.sql` | `service` / Stage 12 project and service；含 APPEND_ONLY registry 与统一 guard attach | PLANNED |
| `20261021091100` | `db/migrations/service/V20261021091100__service_create_reminder_policies.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261021091200` | `db/migrations/service/V20261021091200__service_create_attachment_links.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261021091300` | `db/migrations/service/V20261021091300__service_backfill_seed_dictionaries.sql` | `service` / Stage 12 project and service | PLANNED |
| `20261022090000` | `db/migrations/platform_meta/V20261022090000__platform_meta_custom_object_model.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090100` | `db/migrations/platform_meta/V20261022090100__platform_meta_ddl_plan.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090200` | `db/migrations/platform_meta/V20261022090200__platform_meta_ui_layouts.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090300` | `db/migrations/platform_meta/V20261022090300__platform_meta_client_capability_values.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090400` | `db/migrations/platform_meta/V20261022090400__platform_meta_backfill_capability_matrix.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090500` | `db/migrations/platform_meta/V20261022090500__platform_meta_alter_config_package.sql` | `platform_meta` / Stage 13 clients and low-code；基于 Stage 3 同序 Rust/DB 18（前16加 `LICENSE_GRANT`、`MODULE_PACKAGE`），尾部追加 F-55 `MCP_CONNECTOR`、`MCP_MANIFEST_VERSION`，同批把 Rust `ItemKind::ALL` 与 DB CHECK 扩到终态20；同时按既定范围补 autotest 列/十一态；不重复 Stage 3 审批列或 `accepted_trust_bundle_sha256`，不创建只属于093300的 `UNIQUE(config_package_id,id)` | PLANNED |
| `20261022090600` | `db/migrations/platform_meta/V20261022090600__platform_meta_config_release.sql` | `platform_meta` / Stage 13 clients and low-code；除 config release 表/状态机外，同一迁移按 Stage 13 §3.4 两张 exact catalog 幂等 seed 恰 30 个 permission item（id `...0320`–`...0349`）与恰 12 个 object-scope binding（id `...0520`–`...0531`）。permission 固定 code/id/allowed_actions/object_type 逐行映射且 `module_code=platform,function_point=code,description=NULL`；binding 固定 object/id/table 逐行映射、`schema_name=platform_meta`、四 scope anchor NULL、`security_level_col=security_level`。`ON CONFLICT DO NOTHING` 后重读并断言全字段 exact equal，缺/多/漂移均失败，不 seed 任何 role grant；fresh gate 还须证明固定 route `(permission,Action)` registry exact-set 与真实 table/column 存在 | PLANNED |
| `20261022090700` | `db/migrations/platform_meta/V20261022090700__platform_meta_backfill_release_mutex_row.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090800` | `db/migrations/platform_meta/V20261022090800__platform_meta_brand_profiles.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022090900` | `db/migrations/platform_meta/V20261022090900__platform_meta_client_releases.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022091000` | `db/migrations/platform_meta/V20261022091000__platform_meta_extensions.sql` | `platform_meta` / Stage 13 extensions, immutable artifact identity, EXTENSION_ENABLE approval evidence and deferred grant graph | PLANNED |
| `20261022091100` | `db/migrations/platform_meta/V20261022091100__platform_meta_extension_invocations.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022091200` | `db/migrations/platform_meta/V20261022091200__platform_meta_client_bootstrap_dispatches.sql` | `platform_meta` / Stage 13 clients and low-code | PLANNED |
| `20261022091300` | `db/migrations/platform_core/V20261022091300__platform_core_backfill_stage13_unpoliced_table_registry.sql` | `platform_core` / Stage 13 clients and low-code | PLANNED |
| `20261023090000` | `db/migrations/platform_ops/V20261023090000__platform_ops_backfill_singletons.sql` | `platform_ops` / Stage 14 compatibility no-op; singleton rows are inserted by their 090700/090900 create-table migrations because this version executes first | PLANNED |
| `20261023090100` | `db/migrations/platform_ops/V20261023090100__platform_ops_deployment_records.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090200` | `db/migrations/platform_ops/V20261023090200__platform_ops_offsite_sinks.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090300` | `db/migrations/platform_ops/V20261023090300__platform_ops_extend_degradation_windows.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090350` | `db/migrations/platform_ops/concurrent/V20261023090350__platform_ops_add_degradation_window_indexes.sql` | `platform_ops` / Stage 14 operations concurrent index | PLANNED |
| `20261023090400` | `db/migrations/platform_ops/V20261023090400__platform_ops_writeout_runs.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090500` | `db/migrations/platform_ops/V20261023090500__platform_ops_attachment_watermarks.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090600` | `db/migrations/platform_ops/V20261023090600__platform_ops_backup_sets.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090700` | `db/migrations/platform_ops/V20261023090700__platform_ops_backup_runner_slot.sql` | `platform_ops` / Stage 14 backup runner singleton table and seed row | PLANNED |
| `20261023090800` | `db/migrations/platform_ops/V20261023090800__platform_ops_backup_verifications.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023090900` | `db/migrations/platform_ops/V20261023090900__platform_ops_archive_channel.sql` | `platform_ops` / Stage 14 archive singleton table and seed row | PLANNED |
| `20261023091000` | `db/migrations/platform_ops/V20261023091000__platform_ops_archive_channel_transitions.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091100` | `db/migrations/platform_ops/V20261023091100__platform_ops_replication_reports.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091200` | `db/migrations/platform_ops/V20261023091200__platform_ops_wal_retention_samples.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091300` | `db/migrations/platform_ops/V20261023091300__platform_ops_capacity_samples.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091400` | `db/migrations/platform_ops/V20261023091400__platform_ops_key_recovery_materials.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091500` | `db/migrations/platform_ops/V20261023091500__platform_ops_key_recovery_verifications.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091600` | `db/migrations/platform_ops/V20261023091600__platform_ops_recovery_drills.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091700` | `db/migrations/platform_ops/V20261023091700__platform_ops_alert_suppressions.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091800` | `db/migrations/platform_ops/V20261023091800__platform_ops_data_migration_batches.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023091900` | `db/migrations/platform_ops/V20261023091900__platform_ops_data_migration_records.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023092000` | `db/migrations/platform_ops/V20261023092000__platform_ops_data_migration_reconciliations.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023092100` | `db/migrations/platform_ops/V20261023092100__platform_ops_data_migration_known_differences.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023092200` | `db/migrations/platform_ops/V20261023092200__platform_ops_views.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023092300` | `db/migrations/platform_ops/V20261023092300__platform_ops_grants_ops_ro.sql` | `platform_ops` / Stage 14 operations | PLANNED |
| `20261023092400` | `db/migrations/platform_core/V20261023092400__platform_core_backfill_stage14_unpoliced_table_registry.sql` | `platform_core` / Stage 14 operations | PLANNED |
| `20261023092500` | `db/migrations/platform_ops/V20261023092500__platform_ops_harden_backup_evidence_graph.sql` | `platform_ops` / Stage 14 seven-state backup/writeout graph, complete verification set, typed archive after-image replay, recovery shape, append-only and disposal evidence hardening；atomically expands `degradation_windows.kind` CHECK from the Stage 2 exact 3-value baseline to the final 21-value set including `LEGAL_ENTITY_KEY_DOMAIN_UNAVAILABLE`, and replaces the unsuppressible CHECK with the final exact 5-value set | PLANNED |
| `20261023092600` | `db/migrations/platform_ops/V20261023092600__platform_ops_harden_data_migration_evidence_graph.sql` | `platform_ops` / Stage 14 DATA_MIGRATION reauth/content-version approval plus effective role-grant evidence, writer receipt, reconciliation and cutover/reversal graph hardening | PLANNED |
| `20261023092700` | `db/migrations/clm/V20261023092700__clm_harden_contract_economic_graph.sql` | `clm` / Stage 6 deferred contract header-line-version-payment economic graph | PLANNED |
| `20261023092800` | `db/migrations/sales/V20261023092800__sales_harden_order_delivery_economic_graph.sql` | `sales` / Stage 6 source-version, schedule, delivery allocation and open-amount deferred graph | PLANNED |
| `20261024090000` | `db/migrations/platform_ops/V20261024090000__platform_ops_create_ai_model_packages.sql` | `platform_ops` / F-55 local AI；depends=`20261023092800`；object=部署级 `ai_model_packages`、安装收据、security level、状态/唯一/不可变 guard；rollback=生产只停用 AI 路由/活动包并保留表与历史，禁止 DROP；evidence=`RG-AI-CONTAINMENT-GREEN`,`RG-AI-RESOURCE-CERTIFIED`,`RG-LICENSE-MODULE-LIFECYCLE-GREEN`，且与 090800 同发布批 | PLANNED |
| `20261024090100` | `db/migrations/platform_meta/V20261024090100__platform_meta_create_mcp_connectors.sql` | `platform_meta` / F-55 MCP；depends=`20261022090500` 的终态20值 Rust/DB ItemKind 与 `20261024090000` 全序前驱；object=`mcp_connectors`、RLS、同法人 FK、状态 guard；rollback=生产停 inbound/outbound 路由并保留 connector/history，禁止 DROP；evidence=`RG-MCP-CONFORMANCE-GREEN`,`RG-MCP-CONTAINMENT-GREEN`,`RG-LICENSE-MODULE-LIFECYCLE-GREEN` | PLANNED |
| `20261024090200` | `db/migrations/platform_meta/V20261024090200__platform_meta_create_mcp_manifest_versions.sql` | `platform_meta` / F-55 MCP；depends=`20261024090100`；object=immutable manifest versions、签名 key、附件/安装收据、active slot 与 shape trigger；rollback=生产停用 active manifest/路由并保留版本与附件引用，禁止 DROP；evidence=`RG-MCP-CONFORMANCE-GREEN`,`RG-MCP-CONTAINMENT-GREEN`,`RG-LICENSE-MODULE-LIFECYCLE-GREEN` | PLANNED |
| `20261024090300` | `db/migrations/platform_authz/V20261024090300__platform_authz_create_mcp_human_grants.sql` | `platform_authz` / F-55 MCP；depends=`20261024090100`,`20261024090200`；object=`mcp_human_grants`、last-proof counter、RLS、祖先 FK、受理计数状态 trigger，并幂等 seed permission `...0310`–`...0314`/binding `...0504`–`...0508`、逐字段断言且零自动 role grant；rollback=生产撤路由/显式授权但保留 grants/权限历史，禁止 DROP；evidence=`RG-MCP-CONFORMANCE-GREEN`,`RG-MCP-CONTAINMENT-GREEN`,`RG-LICENSE-MODULE-LIFECYCLE-GREEN` | PLANNED |
| `20261024090400` | `db/migrations/platform_meta/V20261024090400__platform_meta_add_server_admin_capability_rows.sql` | `platform_meta` / F-55 ServerAdmin；depends=`20261024090300`；object=client CHECK 加 `server_admin`、回填 18 行并冻结 90-cell hash；rollback=生产停 ServerAdmin 路由并保留 client 值/矩阵行，禁止删行或收窄 CHECK；evidence=`RG-SERVER-ADMIN-MATRIX-90-GREEN`,`RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` | PLANNED |
| `20261024090500` | `db/migrations/platform_core/V20261024090500__platform_core_add_server_admin_client_kind.sql` | `platform_core` / F-55 ServerAdmin；depends=`20261024090400`；object=`user_devices.client` 等持久化 client CHECK 只加 `server_admin`，MCP 继续使用来源 device 例外；rollback=生产停 ServerAdmin 路由并保留已写 client 历史，禁止收窄 CHECK；evidence=`RG-SERVER-ADMIN-MATRIX-90-GREEN`,`RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` | PLANNED |
| `20261024090600` | `db/migrations/platform_audit/V20261024090600__platform_audit_add_server_admin_and_mcp_clients.sql` | `platform_audit` / F-55 ServerAdmin/MCP；depends=`20261024090500`；object=audit client CHECK 加 `server_admin`、`mcp` 至终态九值，并同步序列化证据；rollback=生产停相应路由但保留不可变审计与扩展 CHECK，禁止删历史；evidence=`RG-SERVER-ADMIN-MATRIX-90-GREEN`,`RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` | PLANNED |
| `20261024090700` | `db/migrations/platform_ops/V20261024090700__platform_ops_add_deployment_carrier.sql` | `platform_ops` / F-55 deployment carrier；depends=`20261024090600`；object=`deployment_records` 十四个 nullable carrier/evidence 列、legacy-or-full CHECK、新行/current guard、签名 policy/evidence digest；rollback=生产只 supersede/停 carrier 路由，保留列与 revision 历史，禁止 DROP/原地补 legacy；evidence=`RG-SERVER-ADMIN-MATRIX-90-GREEN`,`RG-DEPLOYMENT-CARRIER-EVIDENCE-GREEN` | PLANNED |
| `20261024090800` | `db/migrations/platform_core/V20261024090800__platform_core_backfill_f55_unpoliced_table_registry.sql` | `platform_core` / F-55 local AI；depends=`20261024090000` 且与其同发布批；object=为 `platform_ops.ai_model_packages` 幂等登记 `SAME_FOR_ALL_ENTITIES` consumer/理由并逐字段断言；rollback=生产保留登记与表历史，只停 AI 路由，禁止 DELETE/DROP；evidence=`RG-AI-CONTAINMENT-GREEN`,`RG-AI-RESOURCE-CERTIFIED`,`RG-LICENSE-MODULE-LIFECYCLE-GREEN` | PLANNED |

## Fresh target-state verification gate

目录状态不是数据库完成证据。实现批必须把全部迁移应用到全新 PostgreSQL 16，再直接查询 `pg_catalog.pg_attribute`、`pg_constraint`、`pg_index`、`pg_trigger` 与 `pg_proc`；只读本目录、SQL 文本、ORM schema 或 migration history 行均不得放行。以下三条预创建迁移在所有断言逐项通过前持续为 PLANNED：

- `20260901091500`：证明 locator grammar CHECK、embedded domain id 与本行 id、尾段与 `kek_version` 的行内一致性、`kek_version between 1 and 2147483647`、状态时间形状、法人 FK/候选键/同 kind 唯一键；应用集成证据另证明 deployment id 对签名 manifest 的绑定及 u32 超界在 SQL cast 前拒绝，明确该项不是 SQL CHECK。
- `20260901092000`：证明复合 key-domain FK、purpose↔algorithm、data-key version 1..65535、四态时间 shape、tuple 唯一与双表 deferred trigger；证明上述 exact 八列存在且全为 NOT NULL，历史单数列不存在；两份 wrapped bytes 正长度，两个 wrap-key version、`wrap_context_generation`、`wrap_envelope_version` 均在 1..i32::MAX，两个 recipient ref canonical/非空/逐字不同。负例含 data-key version 0/65536、ref 前导零、current=65535 轮换、任一 wrap/version/generation 0 或 i32::MAX+1、空信封、空/相同 recipient、错 deployment/legal entity/purpose/data-key id/version/context generation/recipient/envelope version、缺任一信封和错误 state/purpose 组合。合法激活提交 exact 16 rows + 双信封摘要/binding + PROVISIONING→ACTIVE + 唯一 activation event；缺/多/错序/错 source 在 COMMIT 失败，日常 readback 只能使用 operational recipient，`PIV_SHAMIR_2_OF_3_V1` 的三种有效双 share 组合均可离线恢复、任一单 share 失败，PROVISIONING 后供给失败只映射 KEY_UNAVAILABLE。
- `20261012090500`：证明 `secret_ref` 列不存在，`secret_enc bytea`、`secret_key_ref text`、`last_used_counter bigint` 三列存在且可空，TOTP exact one-of 与非 TOTP 全空 CHECK 存在；集成测试再证明 FIELD/L40 EPC1 pseudo-column AAD、counter NULL→单调增加与重放拒绝。

同一 fresh gate 还必须查询 092800 的 canonical `dek_ref` 列、090500/Stage13-090500 的 ItemKind 18→20 阶段形状、093300 的父候选键/六条 source FK/五表 deferred graph，并运行 bootstrap 三条法人授权、activation audit 与 F-56 special acceptance audit 的 exact-payload/幂等负例。任何 catalog 声明与 `pg_catalog` 不等、named stale 文件尚未修订或 fresh apply 失败，整批停止且不得生成 release evidence。

F-56 的 fresh gate 还必须对 `20261013093300` 直接构造终态：所有 `LICENSE_GRANT|MODULE_PACKAGE` special 首次 RELEASE 后 package status 永久恰为 RELEASED，多份 special RELEASED 同时存在合法，且轮换/验收扫描全部 special RELEASED history。直接 SQL 尝试 special `RELEASED→SUPERSEDED|ROLLED_BACK`、RELEASED 摘要为空、非 RELEASED 摘要非空、清理已接受摘要，必须全在 COMMIT 被同一 deferred graph 拒绝；普通包 `RELEASED→SUPERSEDED` 仍为正例。同一 gate 还必须证明 090100 `module_contract_version` 与 descriptor/product/package parser 统一只收 `1..=2147483647`，0 与 2147483648 在入库/cast 前拒绝。

Stage 13 fresh gate 必须对 090600 重算 id `...0320`..`...0349` 恰 30 行 permission 和 `...0520`..`...0531` 恰 12 行 binding 的全字段 exact-set，验证每条固定 route 的 `(permission,Action)` 与目录 allowed_actions 一致、每个 schema/table/security-level 列真实存在，并断言零自动 role grant。任一缺失、额外、重复、ID/code/action/object/table 漂移或 `ON CONFLICT` 静默掩盖都不得放行。
