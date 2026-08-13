//! `AuthzConfigWriteStore` 的 SQL 实现体（阶段 4 任务 #23，04 计划 §4.8）。
//!
//! 三个 AUTHZ 类 applier 的端口签名是同步的（发布执行事务内被调），
//! 而本 crate 的连接面一律 async：同步方法经 [`drive_pg`] 驱动同一
//! 事务句柄上的 async 语句完成。多线程运行时以 block_in_place 让出
//! 工作线程后 block_on；无运行时环境（单元测试）则以最小当前线程
//! 运行时驱动。FakeConn 的语句一律即时就绪，不产生真正挂起。
//!
//! 法人展开：行级策略按 `app.legal_entity_id` 等值过滤，单个事务
//! 只能写见一个法人的行，因此空法人数组不展开为全部法人，而取
//! 事务自身的法人上下文（与端口注释的"全部法人"差异已在任务 #23
//! 汇报登记；发布执行按法人逐事务驱动时两者等价）。
//!
//! 校验和口径：[`CHECKSUM_STMT`] 把七张配置表按固定序聚合为一段
//! 规范文本取 SHA-256，与第 27 号种子迁移的 `digest(...,'sha256')`
//! 同族；版本推进与快照载入共用这一条语句，保证两侧一致。

use std::future::Future;

use ep_foundation::error::codes::{
    PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_platform_authz::applier::{
    AuthzConfigWriteStore, AuthzPolicySpec, FieldGrantSpec, RoleSpec,
};
use ep_platform_authz::types::{FieldVisibility, PolicyEffect};

use crate::conn::DbValue;
use crate::platform_core::identity_accounts::{col_i64, col_text, col_uuid, shape_err};
use crate::tx::PgTx;

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "授权配置写入必须在 PostgreSQL 事务内执行",
        )
    })
}

/// 同步端口驱动 async 语句：多线程运行时让出工作线程后 block_on；
/// 无运行时（测试）用最小当前线程运行时；单线程运行时内调用属装配
/// 错误，直接报错而不嵌套运行时。
fn drive_pg<F: Future>(fut: F) -> Result<F::Output, AppError> {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => match handle.runtime_flavor() {
            tokio::runtime::RuntimeFlavor::MultiThread => {
                Ok(tokio::task::block_in_place(|| handle.block_on(fut)))
            }
            _ => Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "授权配置同步写不支持单线程运行时上下文",
            )),
        },
        Err(_) => {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|_| {
                    AppError::new(
                        PLATFORM_SYSTEM_INTERNAL_ERROR,
                        "构造最小运行时失败，授权配置写入无法执行",
                    )
                })?;
            Ok(rt.block_on(fut))
        }
    }
}

const SELECT_ROLE_ID_STMT: &str = "select id from platform_authz.roles \
     where legal_entity_id = $1 and code = $2";

const INSERT_ROLE_STMT: &str = "insert into platform_authz.roles \
     (id, legal_entity_id, code, name, is_portal_role, lifecycle_state, is_active) \
     values ($1, $2, $3, $4, $5, 'EFFECTIVE', true)";

const UPDATE_ROLE_STMT: &str = "update platform_authz.roles \
     set is_portal_role = $3, updated_at = now(), row_version = row_version + 1 \
     where legal_entity_id = $1 and code = $2";

const DELETE_ROLE_GRANTS_STMT: &str = "delete from platform_authz.role_permission_grants \
     where legal_entity_id = $1 and role_id = $2";

const INSERT_ROLE_GRANT_STMT: &str = "insert into platform_authz.role_permission_grants \
     (id, legal_entity_id, role_id, permission_item_code, action) \
     values ($1, $2, $3, $4, $5)";

const DELETE_ROLE_STMT: &str = "delete from platform_authz.roles \
     where legal_entity_id = $1 and code = $2";

const DELETE_POLICIES_STMT: &str =
    "delete from platform_authz.access_policies where legal_entity_id = $1";
const DELETE_SOD_RULES_STMT: &str =
    "delete from platform_authz.sod_rules where legal_entity_id = $1";
const DELETE_CHAIN_NODES_STMT: &str =
    "delete from platform_authz.approval_chain_nodes where legal_entity_id = $1";
const DELETE_CHAINS_STMT: &str =
    "delete from platform_authz.approval_chains where legal_entity_id = $1";

const INSERT_POLICY_STMT: &str = "insert into platform_authz.access_policies \
     (id, legal_entity_id, role_id, object_type, effect, priority, condition, lifecycle_state) \
     values ($1, $2, $3, $4, $5, $6, $7::jsonb, 'EFFECTIVE')";

/// 角色互斥规则违例的错误码以运行期 violation_error 的同款映射为准。
const INSERT_SOD_RULE_STMT: &str = "insert into platform_authz.sod_rules \
     (id, legal_entity_id, rule_code, rule_kind, left_ref, right_ref, enforcement, message_code) \
     values ($1, $2, $3, 'ROLE_EXCLUSION', $4, $5, 'BLOCK', 'PLATFORM.SOD.DUTY_CONFLICT')";

/// scenario 无规格字段承载，取固定登记场景字面量；name 暂与 code 同值。
const INSERT_CHAIN_STMT: &str = "insert into platform_authz.approval_chains \
     (id, legal_entity_id, code, name, scenario, version_no, lifecycle_state, is_active) \
     values ($1, $2, $3, $4, 'CONFIG_RELEASE', 1, 'EFFECTIVE', true)";

/// 审批人集合目前仅存首个审批人引用（approver_ref）：完整审批人集合
/// 的承载属审批引擎本体（阶段 3b），本文件不越权扩列。
const INSERT_CHAIN_NODE_STMT: &str = "insert into platform_authz.approval_chain_nodes \
     (id, legal_entity_id, approval_chain_id, node_no, approver_kind, approver_ref, quorum) \
     values ($1, $2, $3, $4, 'ROLE', $5, $6)";

const INSERT_FIELD_GRANT_STMT: &str = "insert into platform_authz.field_permissions \
     (id, legal_entity_id, role_id, object_type, field_name, visibility, mask_style) \
     values ($1, $2, $3, $4, $5, $6, $7) \
     on conflict on constraint ux_field_permissions_le_id_role_id_obj_type_field_name \
     do update set visibility = excluded.visibility, mask_style = excluded.mask_style, \
     updated_at = now(), row_version = platform_authz.field_permissions.row_version + 1";

const DELETE_FIELD_GRANT_STMT: &str = "delete from platform_authz.field_permissions \
     where legal_entity_id = $1 and object_type = $3 and field_name = $4 \
     and role_id = (select id from platform_authz.roles \
         where legal_entity_id = $1 and code = $2)";

const MAX_VERSION_STMT: &str = "select coalesce(max(version_no), 0) \
     from platform_authz.authz_config_versions where legal_entity_id = $1";

const INSERT_VERSION_STMT: &str = "insert into platform_authz.authz_config_versions \
     (id, legal_entity_id, version_no, state, checksum) \
     values ($1, $2, $3, 'EFFECTIVE', decode($4, 'hex'))";

/// 规范校验和：七张配置表按固定序聚合为文本后取 SHA-256 十六进制。
/// 版本推进写入与快照载入复核共用本语句（snapshot_query 同 crate 复用）。
pub(crate) const CHECKSUM_STMT: &str = "select encode(digest( \
     'authz_config;legal_entity=' || $1::text \
     || coalesce((select string_agg(';role=' || code || ':' || is_portal_role::text \
             || ':' || coalesce(duty_class, ''), '' order by code) \
         from platform_authz.roles where legal_entity_id = $1), '') \
     || coalesce((select string_agg(';grant=' || r.code || '|' || g.permission_item_code \
             || '|' || g.action, '' order by r.code, g.permission_item_code, g.action) \
         from platform_authz.role_permission_grants g \
         join platform_authz.roles r on r.id = g.role_id \
         where g.legal_entity_id = $1), '') \
     || coalesce((select string_agg(';policy=' || coalesce(r.code, '*') || '|' || p.object_type \
             || '|' || p.effect || '|' || p.priority::text || '|' || p.condition::text, \
             '' order by p.object_type, p.priority, p.id) \
         from platform_authz.access_policies p \
         left join platform_authz.roles r on r.id = p.role_id \
         where p.legal_entity_id = $1), '') \
     || coalesce((select string_agg(';sod=' || rule_code || '|' || rule_kind || '|' \
             || coalesce(left_ref, '') || '|' || coalesce(right_ref, '') || '|' \
             || enforcement || '|' || message_code, '' order by rule_code) \
         from platform_authz.sod_rules where legal_entity_id = $1), '') \
     || coalesce((select string_agg(';chain=' || code || '|' || scenario || '|' \
             || version_no::text, '' order by code, version_no) \
         from platform_authz.approval_chains where legal_entity_id = $1), '') \
     || coalesce((select string_agg(';node=' || c.code || '|' || n.node_no::text || '|' \
             || n.approver_kind || '|' || coalesce(n.role_code, '') || '|' \
             || coalesce(n.approver_ref::text, '') || '|' || n.quorum::text, \
             '' order by c.code, n.node_no) \
         from platform_authz.approval_chain_nodes n \
         join platform_authz.approval_chains c on c.id = n.approval_chain_id \
         where n.legal_entity_id = $1), '') \
     || coalesce((select string_agg(';field=' || r.code || '|' || f.object_type || '|' \
             || f.field_name || '|' || f.visibility || '|' || coalesce(f.mask_style, ''), \
             '' order by r.code, f.object_type, f.field_name) \
         from platform_authz.field_permissions f \
         join platform_authz.roles r on r.id = f.role_id \
         where f.legal_entity_id = $1), '') \
     , 'sha256'), 'hex')";

/// `AuthzConfigWriteStore` 的 PostgreSQL 实现。
#[derive(Default)]
pub struct PgAuthzConfigWriteStore;

impl PgAuthzConfigWriteStore {
    pub fn new() -> Self {
        Self
    }
}

/// 空法人数组取事务自身法人：RLS 等值过滤下单事务只能写见一个法人。
fn targets(pg: &PgTx, requested: &[Id<LegalEntity>]) -> Vec<Id<LegalEntity>> {
    if requested.is_empty() {
        vec![pg.legal_entity_id()]
    } else {
        requested.to_vec()
    }
}

async fn role_id_of(pg: &mut PgTx, le: uuid::Uuid, code: &str) -> Result<uuid::Uuid, AppError> {
    let rows = pg
        .query(
            SELECT_ROLE_ID_STMT,
            &[DbValue::Uuid(le), DbValue::Text(code.to_string())],
        )
        .await?;
    match rows.first() {
        Some(row) => col_uuid(row, 0),
        None => Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("角色 {code} 不存在，授权配置写入被拒"),
        )),
    }
}

async fn upsert_role_for_le(
    pg: &mut PgTx,
    le: uuid::Uuid,
    spec: &RoleSpec,
) -> Result<(), AppError> {
    let existing = pg
        .query(
            SELECT_ROLE_ID_STMT,
            &[DbValue::Uuid(le), DbValue::Text(spec.role_code.clone())],
        )
        .await?;
    let role_id = match existing.first() {
        Some(row) => {
            let id = col_uuid(row, 0)?;
            pg.execute(
                UPDATE_ROLE_STMT,
                &[
                    DbValue::Uuid(le),
                    DbValue::Text(spec.role_code.clone()),
                    DbValue::Bool(spec.is_portal_role),
                ],
            )
            .await?;
            id
        }
        None => {
            let id = uuid::Uuid::now_v7();
            pg.execute(
                INSERT_ROLE_STMT,
                &[
                    DbValue::Uuid(id),
                    DbValue::Uuid(le),
                    DbValue::Text(spec.role_code.clone()),
                    DbValue::Text(spec.role_code.clone()),
                    DbValue::Bool(spec.is_portal_role),
                ],
            )
            .await?;
            id
        }
    };
    pg.execute(
        DELETE_ROLE_GRANTS_STMT,
        &[DbValue::Uuid(le), DbValue::Uuid(role_id)],
    )
    .await?;
    for grant in &spec.grants {
        for action in &grant.actions {
            pg.execute(
                INSERT_ROLE_GRANT_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    DbValue::Uuid(le),
                    DbValue::Uuid(role_id),
                    DbValue::Text(grant.permission_item_code.clone()),
                    DbValue::Text(action.as_str().to_string()),
                ],
            )
            .await?;
        }
    }
    Ok(())
}

async fn replace_policy_domain_for_le(
    pg: &mut PgTx,
    le: uuid::Uuid,
    spec: &AuthzPolicySpec,
) -> Result<(), AppError> {
    for stmt in [
        DELETE_POLICIES_STMT,
        DELETE_SOD_RULES_STMT,
        DELETE_CHAIN_NODES_STMT,
        DELETE_CHAINS_STMT,
    ] {
        pg.execute(stmt, &[DbValue::Uuid(le)]).await?;
    }
    for policy in &spec.policies {
        let role_id = match &policy.role_code {
            Some(code) => Some(role_id_of(pg, le, code).await?),
            None => None,
        };
        let condition = serde_json::to_string(&policy.condition).map_err(|_| {
            AppError::new(PLATFORM_REQUEST_INVALID_PAYLOAD, "访问策略条件序列化失败")
        })?;
        pg.execute(
            INSERT_POLICY_STMT,
            &[
                DbValue::Uuid(uuid::Uuid::now_v7()),
                DbValue::Uuid(le),
                role_id.map_or(DbValue::Null, DbValue::Uuid),
                DbValue::Text(policy.object_type.clone()),
                DbValue::Text(effect_literal(policy.effect).to_string()),
                DbValue::Int64(i64::from(policy.priority)),
                DbValue::Text(condition),
            ],
        )
        .await?;
    }
    for rule in &spec.sod_rules {
        pg.execute(
            INSERT_SOD_RULE_STMT,
            &[
                DbValue::Uuid(uuid::Uuid::now_v7()),
                DbValue::Uuid(le),
                DbValue::Text(rule.rule_code.clone()),
                DbValue::Text(rule.role_a.clone()),
                DbValue::Text(rule.role_b.clone()),
            ],
        )
        .await?;
    }
    for chain in &spec.approval_chains {
        let chain_id = uuid::Uuid::now_v7();
        pg.execute(
            INSERT_CHAIN_STMT,
            &[
                DbValue::Uuid(chain_id),
                DbValue::Uuid(le),
                DbValue::Text(chain.chain_code.clone()),
                DbValue::Text(chain.chain_code.clone()),
            ],
        )
        .await?;
        for node in &chain.nodes {
            pg.execute(
                INSERT_CHAIN_NODE_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    DbValue::Uuid(le),
                    DbValue::Uuid(chain_id),
                    DbValue::Int64(i64::from(node.node_seq)),
                    node.approver_user_ids
                        .first()
                        .map_or(DbValue::Null, |u| DbValue::Uuid(*u)),
                    DbValue::Int64(i64::from(node.quorum)),
                ],
            )
            .await?;
        }
    }
    Ok(())
}

async fn bump_version_for_le(pg: &mut PgTx, le: uuid::Uuid) -> Result<(), AppError> {
    let rows = pg.query(MAX_VERSION_STMT, &[DbValue::Uuid(le)]).await?;
    let row = rows
        .first()
        .ok_or_else(|| shape_err("版本计数查询未返回行"))?;
    let current = col_i64(row, 0)?;
    let next = current
        .checked_add(1)
        .ok_or_else(|| shape_err("授权配置版本号溢出"))?;
    let checksum_rows = pg.query(CHECKSUM_STMT, &[DbValue::Uuid(le)]).await?;
    let checksum_row = checksum_rows
        .first()
        .ok_or_else(|| shape_err("校验和查询未返回行"))?;
    let checksum = col_text(checksum_row, 0)?;
    pg.execute(
        INSERT_VERSION_STMT,
        &[
            DbValue::Uuid(uuid::Uuid::now_v7()),
            DbValue::Uuid(le),
            DbValue::Int64(next),
            DbValue::Text(checksum),
        ],
    )
    .await?;
    Ok(())
}

impl AuthzConfigWriteStore for PgAuthzConfigWriteStore {
    fn upsert_role(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &RoleSpec,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        let spec = spec.clone();
        drive_pg(async move {
            for le in les {
                upsert_role_for_le(pg, le.as_uuid(), &spec).await?;
            }
            Ok(())
        })?
    }

    fn delete_role(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        role_code: &str,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        let code = role_code.to_string();
        drive_pg(async move {
            for le in les {
                let rows = pg
                    .query(
                        SELECT_ROLE_ID_STMT,
                        &[DbValue::Uuid(le.as_uuid()), DbValue::Text(code.clone())],
                    )
                    .await?;
                if let Some(row) = rows.first() {
                    let role_id = col_uuid(row, 0)?;
                    pg.execute(
                        DELETE_ROLE_GRANTS_STMT,
                        &[DbValue::Uuid(le.as_uuid()), DbValue::Uuid(role_id)],
                    )
                    .await?;
                }
                pg.execute(
                    DELETE_ROLE_STMT,
                    &[DbValue::Uuid(le.as_uuid()), DbValue::Text(code.clone())],
                )
                .await?;
            }
            Ok(())
        })?
    }

    fn replace_policy_domain(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &AuthzPolicySpec,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        let spec = spec.clone();
        drive_pg(async move {
            for le in les {
                replace_policy_domain_for_le(pg, le.as_uuid(), &spec).await?;
            }
            Ok(())
        })?
    }

    fn upsert_field_grant(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &FieldGrantSpec,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        let spec = spec.clone();
        drive_pg(async move {
            for le in les {
                let role_id = role_id_of(pg, le.as_uuid(), &spec.role_code).await?;
                let mask_style = mask_style_of(visibility_literal(spec.visibility));
                pg.execute(
                    INSERT_FIELD_GRANT_STMT,
                    &[
                        DbValue::Uuid(uuid::Uuid::now_v7()),
                        DbValue::Uuid(le.as_uuid()),
                        DbValue::Uuid(role_id),
                        DbValue::Text(spec.object_type.clone()),
                        DbValue::Text(spec.field_name.clone()),
                        DbValue::Text(visibility_literal(spec.visibility).to_string()),
                        mask_style,
                    ],
                )
                .await?;
            }
            Ok(())
        })?
    }

    fn delete_field_grant(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
        spec: &FieldGrantSpec,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        let spec = spec.clone();
        drive_pg(async move {
            for le in les {
                pg.execute(
                    DELETE_FIELD_GRANT_STMT,
                    &[
                        DbValue::Uuid(le.as_uuid()),
                        DbValue::Text(spec.role_code.clone()),
                        DbValue::Text(spec.object_type.clone()),
                        DbValue::Text(spec.field_name.clone()),
                    ],
                )
                .await?;
            }
            Ok(())
        })?
    }

    fn bump_authz_config_version(
        &self,
        tx: &mut dyn Tx,
        legal_entity_ids: &[Id<LegalEntity>],
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        let les = targets(pg, legal_entity_ids);
        drive_pg(async move {
            for le in les {
                bump_version_for_le(pg, le.as_uuid()).await?;
            }
            Ok(())
        })?
    }
}

/// MASKED 缺省掩码风格取 FULL（U-B-06 临时取值）；其余可见性无掩码。
fn mask_style_of(visibility: &str) -> DbValue {
    if visibility == "MASKED" {
        DbValue::Text("FULL".to_string())
    } else {
        DbValue::Null
    }
}

/// 策略效果的落库字面量（与 access_policies.effect 的 CHECK 取值一致）。
fn effect_literal(effect: PolicyEffect) -> &'static str {
    match effect {
        PolicyEffect::Allow => "ALLOW",
        PolicyEffect::Deny => "DENY",
    }
}

/// 字段可见性的落库字面量（与 field_permissions.visibility 的 CHECK 一致）。
fn visibility_literal(visibility: FieldVisibility) -> &'static str {
    match visibility {
        FieldVisibility::Hidden => "HIDDEN",
        FieldVisibility::Masked(_) => "MASKED",
        FieldVisibility::Read => "READ",
        FieldVisibility::Write => "WRITE",
    }
}

#[cfg(test)]
mod tests {
    use ep_foundation::port::tx::{IsolationKind, TxId};
    use ep_platform_authz::applier::{RoleGrantSpec, RoleSpec};
    use ep_platform_authz::types::Action;
    use std::sync::Arc;

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    fn tx_over(conn: FakeConn) -> PgTx {
        PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
            conn: Some(Box::new(conn)),
            pool_label: "rw",
            metrics: Arc::new(NoopDbMetrics),
            side_effect: false,
            last_pg_error: None,
        }
    }

    #[test]
    fn empty_legal_entity_list_falls_back_to_tx_entity() {
        let conn = FakeConn::new();
        let tx = tx_over(conn);
        let got = targets(&tx, &[]);
        assert_eq!(
            got,
            vec![Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1))]
        );
    }

    #[test]
    fn upsert_role_inserts_then_replaces_grants() {
        let mut conn = FakeConn::new();
        // 角色存在性查询（无行）→ 插入角色 → 清授予 → 插两条授予。
        conn.push_rows(Vec::new());
        let mut tx = tx_over(conn);
        let store = PgAuthzConfigWriteStore::new();
        let spec = RoleSpec {
            role_code: "OPS_ADMIN".to_string(),
            is_portal_role: false,
            grants: vec![RoleGrantSpec {
                permission_item_code: "platform.roles".to_string(),
                actions: vec![Action::View, Action::Create],
            }],
        };
        store
            .upsert_role(&mut tx, &[], &spec)
            .expect("无运行时环境下以最小运行时驱动");
    }

    #[test]
    fn bump_version_computes_checksum_before_insert() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(3)]]);
        conn.push_rows(vec![vec![DbValue::Text("ab".repeat(32))]]);
        let mut tx = tx_over(conn);
        let store = PgAuthzConfigWriteStore::new();
        store
            .bump_authz_config_version(&mut tx, &[])
            .expect("版本推进可完成");
        assert!(CHECKSUM_STMT.contains("'sha256'"));
        assert!(INSERT_VERSION_STMT.contains("'EFFECTIVE'"));
    }

    #[test]
    fn checksum_statement_covers_all_seven_tables() {
        for table in [
            "platform_authz.roles",
            "platform_authz.role_permission_grants",
            "platform_authz.access_policies",
            "platform_authz.sod_rules",
            "platform_authz.approval_chains",
            "platform_authz.approval_chain_nodes",
            "platform_authz.field_permissions",
        ] {
            assert!(CHECKSUM_STMT.contains(table), "规范校验和须覆盖 {table}");
        }
    }
}
