//! `AuthzConfigVersionQuery` 的 SQL 实现体（阶段 4 任务 #23，04 计划 §2.3）。
//!
//! 快照重载的唯一路径是 core-server 轮询 `authz_config_versions` 的
//! EFFECTIVE 版本号。行级安全策略对运行角色强制生效（含 FORCE），
//! 单事务只能见事务法人上下文一个法人的行：本实现体的三个查询都
//! 只返回当前事务法人可见的行，跨法人聚合由 wiring 侧逐法人逐事务
//! 驱动完成（任务 #23 汇报登记的实现差异）。
//!
//! `load_entity` 的校验和取 [`crate::platform_authz::config_store::CHECKSUM_STMT`]
//! 的现算结果，与版本推进写入侧同一口径；`SnapshotReloader` 据此复核
//! 版本行的 checksum，不符即走降级窗口路径。

use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use ep_foundation::error::codes::{
    PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_platform_authz::snapshot::{AuthzConfigVersionQuery, EffectiveVersion, EntityAuthzData};
use ep_platform_authz::types::{
    Action, FieldVisibility, MaskStyle, ObjectScopeBinding, PolicyCondition, PolicyEffect,
};

use crate::conn::DbValue;
use crate::platform_authz::config_store::CHECKSUM_STMT;
use crate::platform_core::identity_accounts::{col_bool, col_i64, col_text, col_uuid, shape_err};
use crate::tx::PgTx;

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "授权快照查询必须在 PostgreSQL 事务内执行",
        )
    })
}

/// EFFECTIVE 版本行：RLS 过滤后仅见当前事务法人的行。
const EFFECTIVE_VERSIONS_STMT: &str = "select legal_entity_id, version_no, \
     encode(checksum, 'hex') from platform_authz.authz_config_versions \
     where state = 'EFFECTIVE' order by legal_entity_id";

const SNAPSHOT_ROLES_STMT: &str = "select code, is_portal_role \
     from platform_authz.roles where legal_entity_id = $1 \
     and lifecycle_state = 'EFFECTIVE' order by code";

const SNAPSHOT_GRANTS_STMT: &str = "select r.code, pi.object_type, g.action \
     from platform_authz.role_permission_grants g \
     join platform_authz.roles r on r.id = g.role_id \
     join platform_authz.permission_items pi on pi.code = g.permission_item_code \
     where g.legal_entity_id = $1 \
     order by r.code, pi.object_type, g.action";

const SNAPSHOT_POLICIES_STMT: &str = "select r.code, p.object_type, p.effect, \
     p.priority, p.condition::text from platform_authz.access_policies p \
     left join platform_authz.roles r on r.id = p.role_id \
     where p.legal_entity_id = $1 and p.lifecycle_state = 'EFFECTIVE' \
     order by p.priority, p.id";

const SNAPSHOT_FIELDS_STMT: &str = "select r.code, f.object_type, f.field_name, \
     f.visibility, f.mask_style from platform_authz.field_permissions f \
     join platform_authz.roles r on r.id = f.role_id \
     where f.legal_entity_id = $1 order by r.code, f.object_type, f.field_name";

/// 登记制表，无列法人、无 RLS 策略，任一法人上下文均可全量读取。
/// 表上不存在 valid_from/valid_to 两列，内存形态的对应两字段取 None。
const OBJECT_BINDINGS_STMT: &str = "select object_type, schema_name, table_name, \
     owner_user_col, owning_dept_col, project_col, customer_col, security_level_col \
     from platform_authz.object_scope_bindings order by object_type";

/// `AuthzConfigVersionQuery` 的 PostgreSQL 实现。无状态，读取一律在
/// 调用方事务内执行，不自行开事务。
#[derive(Default)]
pub struct PgAuthzConfigVersionQuery;

impl PgAuthzConfigVersionQuery {
    pub fn new() -> Self {
        Self
    }
}

fn opt_text(row: &[DbValue], idx: usize) -> Result<Option<Arc<str>>, AppError> {
    match &row[idx] {
        DbValue::Text(s) => Ok(Some(Arc::from(s.as_str()))),
        DbValue::Null => Ok(None),
        _ => Err(shape_err("可选文本列形态非法")),
    }
}

fn action_of(raw: &str) -> Result<Action, AppError> {
    Action::ALL
        .iter()
        .copied()
        .find(|a| a.as_str() == raw)
        .ok_or_else(|| {
            AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("授权动作取值未登记：{raw}"),
            )
        })
}

fn visibility_of(visibility: &str, mask_style: Option<&str>) -> Result<FieldVisibility, AppError> {
    match visibility {
        "HIDDEN" => Ok(FieldVisibility::Hidden),
        "READ" => Ok(FieldVisibility::Read),
        "WRITE" => Ok(FieldVisibility::Write),
        "MASKED" => {
            let style = match mask_style.unwrap_or("FULL") {
                "FULL" => MaskStyle::Full,
                "KEEP_LAST_4" => MaskStyle::KeepLast4,
                "KEEP_DOMAIN" => MaskStyle::KeepDomain,
                other => {
                    return Err(AppError::new(
                        PLATFORM_REQUEST_INVALID_PAYLOAD,
                        format!("掩码样式取值未登记：{other}"),
                    ))
                }
            };
            Ok(FieldVisibility::Masked(style))
        }
        other => Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("字段可见性取值未登记：{other}"),
        )),
    }
}

async fn load_checksum(pg: &mut PgTx, le: uuid::Uuid) -> Result<Arc<str>, AppError> {
    let rows = pg.query(CHECKSUM_STMT, &[DbValue::Uuid(le)]).await?;
    let row = rows
        .first()
        .ok_or_else(|| shape_err("校验和查询未返回行"))?;
    Ok(Arc::from(col_text(row, 0)?.as_str()))
}

async fn load_roles(
    pg: &mut PgTx,
    le: uuid::Uuid,
    portal_roles: &mut BTreeSet<Arc<str>>,
) -> Result<(), AppError> {
    let rows = pg.query(SNAPSHOT_ROLES_STMT, &[DbValue::Uuid(le)]).await?;
    for row in &rows {
        let code = col_text(row, 0)?;
        if col_bool(row, 1)? {
            portal_roles.insert(Arc::from(code.as_str()));
        }
    }
    Ok(())
}

async fn load_role_grants(
    pg: &mut PgTx,
    le: uuid::Uuid,
    grants: &mut HashMap<(Arc<str>, Arc<str>), BTreeSet<Action>>,
) -> Result<(), AppError> {
    let rows = pg.query(SNAPSHOT_GRANTS_STMT, &[DbValue::Uuid(le)]).await?;
    for row in &rows {
        let role = Arc::from(col_text(row, 0)?.as_str());
        let object_type = Arc::from(col_text(row, 1)?.as_str());
        let action = action_of(&col_text(row, 2)?)?;
        grants
            .entry((role, object_type))
            .or_default()
            .insert(action);
    }
    Ok(())
}

async fn load_policies(
    pg: &mut PgTx,
    le: uuid::Uuid,
    data: &mut EntityAuthzData,
) -> Result<(), AppError> {
    let rows = pg
        .query(SNAPSHOT_POLICIES_STMT, &[DbValue::Uuid(le)])
        .await?;
    for row in &rows {
        let role_code = match &row[0] {
            DbValue::Text(s) => Some(Arc::from(s.as_str())),
            DbValue::Null => None,
            _ => return Err(shape_err("策略角色列形态非法")),
        };
        let effect = match col_text(row, 2)?.as_str() {
            "ALLOW" => PolicyEffect::Allow,
            "DENY" => PolicyEffect::Deny,
            other => {
                return Err(AppError::new(
                    PLATFORM_REQUEST_INVALID_PAYLOAD,
                    format!("策略效果取值未登记：{other}"),
                ))
            }
        };
        let priority = i32::try_from(col_i64(row, 3)?).map_err(|_| shape_err("策略优先级溢出"))?;
        let condition =
            serde_json::from_str::<PolicyCondition>(&col_text(row, 4)?).map_err(|_| {
                AppError::new(PLATFORM_REQUEST_INVALID_PAYLOAD, "访问策略条件反序列化失败")
            })?;
        data.policies
            .push(ep_platform_authz::snapshot::AccessPolicyEntry {
                role_code,
                object_type: Arc::from(col_text(row, 1)?.as_str()),
                effect,
                priority,
                condition,
            });
    }
    Ok(())
}

async fn load_field_grants(
    pg: &mut PgTx,
    le: uuid::Uuid,
    data: &mut EntityAuthzData,
) -> Result<(), AppError> {
    let rows = pg.query(SNAPSHOT_FIELDS_STMT, &[DbValue::Uuid(le)]).await?;
    for row in &rows {
        let mask_style = match &row[4] {
            DbValue::Text(s) => Some(s.as_str()),
            DbValue::Null => None,
            _ => return Err(shape_err("掩码样式列形态非法")),
        };
        data.field_grants
            .push(ep_platform_authz::snapshot::FieldGrantEntry {
                role_code: Arc::from(col_text(row, 0)?.as_str()),
                object_type: Arc::from(col_text(row, 1)?.as_str()),
                field_name: Arc::from(col_text(row, 2)?.as_str()),
                visibility: visibility_of(&col_text(row, 3)?, mask_style)?,
            });
    }
    Ok(())
}

#[async_trait::async_trait]
impl AuthzConfigVersionQuery for PgAuthzConfigVersionQuery {
    async fn effective_versions(&self, tx: &mut dyn Tx) -> Result<Vec<EffectiveVersion>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg.query(EFFECTIVE_VERSIONS_STMT, &[]).await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            let n = col_i64(row, 1)?;
            out.push(EffectiveVersion {
                legal_entity_id: Id::<LegalEntity>::from_uuid(col_uuid(row, 0)?),
                version_no: u64::try_from(n).map_err(|_| shape_err("授权配置版本号为负"))?,
                checksum: Arc::from(col_text(row, 2)?.as_str()),
            });
        }
        Ok(out)
    }

    async fn load_entity(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        version_no: u64,
    ) -> Result<EntityAuthzData, AppError> {
        let pg = downcast(tx)?;
        let le = legal_entity_id.as_uuid();
        let mut data = EntityAuthzData {
            version_no,
            checksum: load_checksum(pg, le).await?,
            ..Default::default()
        };
        load_roles(pg, le, &mut data.portal_roles).await?;
        load_role_grants(pg, le, &mut data.role_grants).await?;
        load_policies(pg, le, &mut data).await?;
        load_field_grants(pg, le, &mut data).await?;
        Ok(data)
    }

    async fn object_bindings(&self, tx: &mut dyn Tx) -> Result<Vec<ObjectScopeBinding>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg.query(OBJECT_BINDINGS_STMT, &[]).await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in &rows {
            out.push(ObjectScopeBinding {
                object_type: Arc::from(col_text(row, 0)?.as_str()),
                schema_name: Arc::from(col_text(row, 1)?.as_str()),
                table_name: Arc::from(col_text(row, 2)?.as_str()),
                owner_user_col: opt_text(row, 3)?,
                owning_dept_col: opt_text(row, 4)?,
                project_col: opt_text(row, 5)?,
                customer_col: opt_text(row, 6)?,
                security_level_col: Arc::from(col_text(row, 7)?.as_str()),
                valid_from_col: None,
                valid_to_col: None,
            });
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc as StdArc;

    use ep_foundation::port::tx::{IsolationKind, TxId};

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    fn tx_over(conn: FakeConn) -> PgTx {
        PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
            conn: Some(Box::new(conn)),
            pool_label: "ro",
            metrics: StdArc::new(NoopDbMetrics),
            side_effect: false,
            last_pg_error: None,
        }
    }

    #[test]
    fn checksum_is_shared_with_version_bump() {
        assert!(
            CHECKSUM_STMT.contains("'sha256'"),
            "快照复核与版本推进共用同一条校验和语句"
        );
    }

    #[tokio::test]
    async fn effective_versions_decodes_hex_checksum() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![
            DbValue::Uuid(uuid::Uuid::from_u128(2)),
            DbValue::Int64(7),
            DbValue::Text("ab".repeat(32)),
        ]]);
        let mut tx = tx_over(conn);
        let q = PgAuthzConfigVersionQuery::new();
        let versions = q.effective_versions(&mut tx).await.expect("查询可完成");
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].version_no, 7);
        assert_eq!(versions[0].checksum.as_ref(), "ab".repeat(32));
    }

    #[tokio::test]
    async fn load_entity_assembles_role_grants_and_policies() {
        let mut conn = FakeConn::new();
        // 校验和、角色、授予、策略、字段五个查询按序应答。
        conn.push_rows(vec![vec![DbValue::Text("cd".repeat(32))]]);
        conn.push_rows(vec![vec![
            DbValue::Text("ADMIN".to_string()),
            DbValue::Bool(true),
        ]]);
        conn.push_rows(vec![vec![
            DbValue::Text("ADMIN".to_string()),
            DbValue::Text("platform.user_accounts".to_string()),
            DbValue::Text("VIEW".to_string()),
        ]]);
        conn.push_rows(vec![vec![
            DbValue::Null,
            DbValue::Text("platform.user_accounts".to_string()),
            DbValue::Text("DENY".to_string()),
            DbValue::Int64(1),
            DbValue::Text("{\"clauses\":[]}".to_string()),
        ]]);
        conn.push_rows(vec![vec![
            DbValue::Text("ADMIN".to_string()),
            DbValue::Text("platform.user_accounts".to_string()),
            DbValue::Text("login_name".to_string()),
            DbValue::Text("MASKED".to_string()),
            DbValue::Text("KEEP_LAST_4".to_string()),
        ]]);
        let mut tx = tx_over(conn);
        let q = PgAuthzConfigVersionQuery::new();
        let data = q
            .load_entity(&mut tx, Id::from_uuid(uuid::Uuid::from_u128(2)), 7)
            .await
            .expect("载入可完成");
        assert_eq!(data.version_no, 7);
        assert!(data.portal_roles.contains("ADMIN"));
        let key = (Arc::from("ADMIN"), Arc::from("platform.user_accounts"));
        assert!(data.role_grants[&key].contains(&Action::View));
        assert_eq!(data.policies.len(), 1);
        assert_eq!(data.policies[0].effect, PolicyEffect::Deny);
        assert!(matches!(
            data.field_grants[0].visibility,
            FieldVisibility::Masked(MaskStyle::KeepLast4)
        ));
    }

    #[tokio::test]
    async fn object_bindings_have_no_validity_columns() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![
            DbValue::Text("platform.user_accounts".to_string()),
            DbValue::Text("platform_core".to_string()),
            DbValue::Text("user_accounts".to_string()),
            DbValue::Text("id".to_string()),
            DbValue::Null,
            DbValue::Null,
            DbValue::Null,
            DbValue::Text("security_level".to_string()),
        ]]);
        let mut tx = tx_over(conn);
        let q = PgAuthzConfigVersionQuery::new();
        let bindings = q.object_bindings(&mut tx).await.expect("查询可完成");
        assert_eq!(bindings.len(), 1);
        assert!(bindings[0].valid_from_col.is_none());
        assert_eq!(bindings[0].owner_user_col.as_deref(), Some("id"));
    }

    #[tokio::test]
    async fn unknown_action_literal_is_rejected() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Text("cd".repeat(32))]]);
        conn.push_rows(Vec::new());
        conn.push_rows(vec![vec![
            DbValue::Text("ADMIN".to_string()),
            DbValue::Text("platform.user_accounts".to_string()),
            DbValue::Text("FROBNICATE".to_string()),
        ]]);
        let mut tx = tx_over(conn);
        let q = PgAuthzConfigVersionQuery::new();
        let err = q
            .load_entity(&mut tx, Id::from_uuid(uuid::Uuid::from_u128(2)), 7)
            .await
            .expect_err("未登记动作必须被拒");
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
    }
}
