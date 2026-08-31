//! `platform_authz` 六张用户维度表与 authz_config_versions 的读取面：
//! 会话建立时读授权集合一次冻结进 SecurityContext（04 §4.3）。
//!
//! 生效窗判定一律以 `CURRENT_DATE` 在库侧比较（granted/effective
//! from-to），多行聚合以 `string_agg` 折为单值文本再在 Rust 侧
//! 拆分去重（DbValue 无数组变体）。法人目录枚举经注入的
//! `LegalEntityDirectory`（platform_core 的 SQL 归 tenancy 仓储），
//! 本文件不直接引用 platform_core 基表（archcheck 一 schema 一文件）。

use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{
    Customer, Department, LegalEntity, Position, Project, UserAccount,
};
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;
use ep_foundation::security::context::DutyClass;
use ep_platform_identity::ports::{UserAuthzQuery, UserAuthzSet};
use ep_platform_tenancy::directory::LegalEntityDirectory;

use crate::conn::DbValue;
use crate::platform_core::identity_accounts::{col_i64, col_text, col_uuid, shape_err};
use crate::tx::PgTx;

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "授权读取必须在 PostgreSQL 事务内执行",
        )
    })
}

/// 有效角色授予：授予窗口命中今日且角色 lifecycle 为 ACTIVE；
/// duty_class 逐行聚合为逗号串（NULL 职责被 string_agg 自然跳过）。
const ROLE_GRANTS_STMT: &str = "select r.code, r.duty_class \
     from platform_authz.user_role_grants g \
     join platform_authz.roles r on r.id = g.role_id \
     where g.user_id = $1 \
     and g.effective_from <= CURRENT_DATE \
     and (g.effective_to is null or g.effective_to >= CURRENT_DATE) \
     and r.lifecycle_state = 'EFFECTIVE' and r.is_active \
     order by r.code";
// F-83 更正：原过滤是 `lifecycle_state = 'ACTIVE'`，而该列的 CHECK 只允许
// DRAFT/PENDING_RELEASE/EFFECTIVE/SUPERSEDED/RETIRED 五态（数据字典 8.3 同款）——
// 'ACTIVE' 根本不是合法取值，条件恒为空集：每个真实用户的 roles 与 duty_classes
// 都是空集，ABAC/RBAC 与职责分离拿到的是「此人无任何角色」。方向是 fail-closed
// （全拒），但等于身份域上线即全体不可用。生效态是 EFFECTIVE；is_active 是独立的
// 停用开关（字典逐字「默认 true」），一并纳入。

/// 第三支判据：所授角色是否挂六类高风险操作权限项之一。
const HIGH_RISK_STMT: &str = "select 1 \
     from platform_authz.user_role_grants g \
     join platform_authz.role_permission_grants p on p.role_id = g.role_id \
     where g.user_id = $1 \
     and g.effective_from <= CURRENT_DATE \
     and (g.effective_to is null or g.effective_to >= CURRENT_DATE) \
     and p.permission_item_code in ('platform.contract_effective', 'platform.payment', \
         'platform.invoice_issue', 'platform.ledger_posting', 'platform.period_close', \
         'platform.sensitive_export') \
     limit 1";

const LEGAL_ENTITY_GRANTS_STMT: &str = "select distinct legal_entity_id \
     from platform_authz.user_legal_entity_grants \
     where user_id = $1 \
     and granted_from <= CURRENT_DATE \
     and (granted_to is null or granted_to >= CURRENT_DATE)";

const ORG_ASSIGNMENTS_STMT: &str = "select distinct department_id, position_id \
     from platform_authz.user_org_assignments \
     where user_id = $1 \
     and effective_from <= CURRENT_DATE \
     and (effective_to is null or effective_to >= CURRENT_DATE)";

const SCOPE_GRANTS_STMT: &str = "select scope_kind, object_type, scope_ref_id \
     from platform_authz.user_scope_grants \
     where user_id = $1 \
     and effective_from <= CURRENT_DATE \
     and (effective_to is null or effective_to >= CURRENT_DATE)";

const DATA_SCOPE_TAGS_STMT: &str = "select array_to_string(g.data_scope_tags, ',') \
     from platform_authz.user_role_grants g \
     where g.user_id = $1 \
     and g.effective_from <= CURRENT_DATE \
     and (g.effective_to is null or g.effective_to >= CURRENT_DATE)";

const SNAPSHOT_VERSION_STMT: &str = "select version_no \
     from platform_authz.authz_config_versions \
     where legal_entity_id = $1 and state = 'EFFECTIVE' \
     order by version_no desc limit 1";

const DUTY_CLASSES_STMT: &str = "select distinct r.duty_class \
     from platform_authz.user_role_grants g \
     join platform_authz.roles r on r.id = g.role_id \
     where g.user_id = $1 \
     and g.effective_from <= CURRENT_DATE \
     and (g.effective_to is null or g.effective_to >= CURRENT_DATE) \
     and r.lifecycle_state = 'EFFECTIVE' and r.is_active and r.duty_class is not null"; // F-83 同上

/// 未结审批待办：未决、待重认证、审批中与已批未执行诸态。
const OPEN_HIGH_RISK_STMT: &str = "select count(*)::int8 \
     from platform_authz.high_risk_requests \
     where initiator_user_id = $1 \
     and status in ('PENDING_INITIATION', 'PENDING_REAUTH', 'REAUTH_FAILED', 'LOCKED', \
         'REAUTH_PASSED', 'IN_APPROVAL', 'APPROVED')";

const PROBE_LE_GRANT_STMT: &str = "select 1 \
     from platform_authz.user_legal_entity_grants \
     where user_id = $1 and legal_entity_id = $2 \
     and granted_from <= CURRENT_DATE \
     and (granted_to is null or granted_to >= CURRENT_DATE) \
     limit 1";

/// `UserAuthzQuery` 的 PostgreSQL 实现。法人目录枚举经注入的
/// [`LegalEntityDirectory`]（platform_core 的 SQL 归 tenancy 仓储）。
pub struct PgUserAuthzQuery {
    directory: Arc<dyn LegalEntityDirectory>,
}

impl PgUserAuthzQuery {
    pub fn new(directory: Arc<dyn LegalEntityDirectory>) -> Self {
        Self { directory }
    }
}

#[async_trait::async_trait]
impl UserAuthzQuery for PgUserAuthzQuery {
    async fn load_user_authz(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        home_legal_entity_id: Id<LegalEntity>,
    ) -> Result<UserAuthzSet, AppError> {
        let pg = downcast(tx)?;
        let uid = user_id.as_uuid();
        let mut set = UserAuthzSet::default();
        let role_rows = pg.query(ROLE_GRANTS_STMT, &[DbValue::Uuid(uid)]).await?;
        for row in &role_rows {
            set.role_codes.push(col_text(row, 0)?);
            if let Some(DbValue::Text(dc)) = row.get(1) {
                if let Some(class) = parse_duty_class(dc) {
                    if !set.duty_classes.contains(&class) {
                        set.duty_classes.push(class);
                    }
                }
            }
        }
        set.has_high_risk_permission = !pg
            .query(HIGH_RISK_STMT, &[DbValue::Uuid(uid)])
            .await?
            .is_empty();
        let le_rows = pg
            .query(LEGAL_ENTITY_GRANTS_STMT, &[DbValue::Uuid(uid)])
            .await?;
        for row in &le_rows {
            set.legal_entity_ids
                .push(Id::<LegalEntity>::from_uuid(col_uuid(row, 0)?));
        }
        load_org_assignments(pg, uid, &mut set).await?;
        load_scope_grants(pg, uid, &mut set).await?;
        load_data_scope_tags(pg, uid, &mut set).await?;
        set.snapshot_version = load_snapshot_version(pg, home_legal_entity_id.as_uuid()).await?;
        Ok(set)
    }

    async fn user_duty_classes(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<Vec<DutyClass>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(DUTY_CLASSES_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        let mut out = Vec::new();
        for row in &rows {
            if let Some(DbValue::Text(dc)) = row.first() {
                if let Some(class) = parse_duty_class(dc) {
                    if !out.contains(&class) {
                        out.push(class);
                    }
                }
            }
        }
        Ok(out)
    }

    async fn count_open_high_risk_requests(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
    ) -> Result<u64, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(OPEN_HIGH_RISK_STMT, &[DbValue::Uuid(user_id.as_uuid())])
            .await?;
        let row = rows
            .first()
            .ok_or_else(|| shape_err("高风险待办计数查询未返回行"))?;
        let n = col_i64(row, 0)?;
        u64::try_from(n).map_err(|_| shape_err("高风险待办计数为负"))
    }

    async fn installed_legal_entities(
        &self,
        _tx: &mut dyn Tx,
    ) -> Result<Vec<Id<LegalEntity>>, AppError> {
        // 枚举源归法人目录（tenancy 仓储的 platform_core SQL），此处只投影 id。
        let refs = self.directory.list_active().await?;
        Ok(refs.into_iter().map(|r| r.id).collect())
    }

    async fn probe_legal_entity_grant(
        &self,
        tx: &mut dyn Tx,
        user_id: Id<UserAccount>,
        legal_entity_id: Id<LegalEntity>,
    ) -> Result<bool, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                PROBE_LE_GRANT_STMT,
                &[
                    DbValue::Uuid(user_id.as_uuid()),
                    DbValue::Uuid(legal_entity_id.as_uuid()),
                ],
            )
            .await?;
        Ok(!rows.is_empty())
    }
}

async fn load_org_assignments(
    pg: &mut PgTx,
    uid: uuid::Uuid,
    set: &mut UserAuthzSet,
) -> Result<(), AppError> {
    let rows = pg
        .query(ORG_ASSIGNMENTS_STMT, &[DbValue::Uuid(uid)])
        .await?;
    for row in &rows {
        set.department_ids
            .push(Id::<Department>::from_uuid(col_uuid(row, 0)?));
        set.position_ids
            .push(Id::<Position>::from_uuid(col_uuid(row, 1)?));
    }
    Ok(())
}

async fn load_scope_grants(
    pg: &mut PgTx,
    uid: uuid::Uuid,
    set: &mut UserAuthzSet,
) -> Result<(), AppError> {
    let rows = pg.query(SCOPE_GRANTS_STMT, &[DbValue::Uuid(uid)]).await?;
    for row in &rows {
        let kind = col_text(row, 0)?;
        let ref_id = col_uuid(row, 2)?;
        match kind.as_str() {
            "PROJECT" => set.project_ids.push(Id::<Project>::from_uuid(ref_id)),
            "CUSTOMER" => set.customer_ids.push(Id::<Customer>::from_uuid(ref_id)),
            "RECORD" => {
                let object_type = match row.get(1) {
                    Some(DbValue::Text(s)) => s.clone(),
                    _ => return Err(shape_err("记录级共享缺 object_type")),
                };
                set.record_shares.push((object_type, ref_id));
            }
            _ => return Err(shape_err("范围授予种类字面量非法")),
        }
    }
    Ok(())
}

async fn load_data_scope_tags(
    pg: &mut PgTx,
    uid: uuid::Uuid,
    set: &mut UserAuthzSet,
) -> Result<(), AppError> {
    let rows = pg
        .query(DATA_SCOPE_TAGS_STMT, &[DbValue::Uuid(uid)])
        .await?;
    for row in &rows {
        if let Some(DbValue::Text(joined)) = row.first() {
            for tag in joined.split(',') {
                if !tag.is_empty() && !set.data_scope_tags.iter().any(|t| t == tag) {
                    set.data_scope_tags.push(tag.to_string());
                }
            }
        }
    }
    Ok(())
}

/// 快照版本取属籍法人 EFFECTIVE 的最大 version_no；无配置取 0。
async fn load_snapshot_version(pg: &mut PgTx, home: uuid::Uuid) -> Result<u64, AppError> {
    let rows = pg
        .query(SNAPSHOT_VERSION_STMT, &[DbValue::Uuid(home)])
        .await?;
    match rows.first() {
        Some(row) => {
            let n = col_i64(row, 0)?;
            u64::try_from(n).map_err(|_| shape_err("授权配置版本号为负"))
        }
        None => Ok(0),
    }
}

fn parse_duty_class(raw: &str) -> Option<DutyClass> {
    match raw {
        "SYSTEM" => Some(DutyClass::System),
        "DATA" => Some(DutyClass::Data),
        "SECURITY" => Some(DutyClass::Security),
        "AUDIT" => Some(DutyClass::Audit),
        "KEY" => Some(DutyClass::Key),
        "CONFIG" => Some(DutyClass::Config),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::port::tx::{IsolationKind, TxId};
    use ep_platform_identity::config::HIGH_RISK_PERMISSION_ITEMS;
    use ep_platform_tenancy::directory::LegalEntityRef;

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    /// 测试用法人目录占位：本文件的测试不走枚举面。
    struct StubDirectory;

    #[async_trait::async_trait]
    impl LegalEntityDirectory for StubDirectory {
        async fn list_active(&self) -> Result<Vec<LegalEntityRef>, AppError> {
            Ok(Vec::new())
        }
        async fn get(&self, _id: Id<LegalEntity>) -> Result<LegalEntityRef, AppError> {
            Err(AppError::new(
                ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
                "测试占位不提供单法人取数",
            ))
        }
    }

    fn q() -> PgUserAuthzQuery {
        PgUserAuthzQuery::new(Arc::new(StubDirectory))
    }

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
    fn window_predicates_use_current_date_everywhere() {
        for stmt in [
            ROLE_GRANTS_STMT,
            HIGH_RISK_STMT,
            LEGAL_ENTITY_GRANTS_STMT,
            ORG_ASSIGNMENTS_STMT,
            SCOPE_GRANTS_STMT,
            DATA_SCOPE_TAGS_STMT,
            DUTY_CLASSES_STMT,
            PROBE_LE_GRANT_STMT,
        ] {
            assert!(
                stmt.contains("CURRENT_DATE"),
                "生效窗以库侧今日比较：{stmt}"
            );
        }
        // F-83：钉住合法取值。'ACTIVE' 不在该列的 CHECK 五态里，曾使本条件恒为空集。
        assert!(ROLE_GRANTS_STMT.contains("r.lifecycle_state = 'EFFECTIVE'"));
        assert!(!ROLE_GRANTS_STMT.contains("'ACTIVE'"), "'ACTIVE' 不是 lifecycle_state 的合法取值");
    }

    #[test]
    fn high_risk_probe_lists_the_six_seed_items() {
        for item in HIGH_RISK_PERMISSION_ITEMS {
            assert!(
                HIGH_RISK_STMT.contains(item),
                "六类高风险权限项必须逐一入判：{item}"
            );
        }
    }

    #[tokio::test]
    async fn load_user_authz_aggregates_role_duty_and_grants() {
        let mut conn = FakeConn::new();
        // 角色行、高风险行、法人授予、组织指派、范围授予、标签、版本号。
        conn.push_rows(vec![
            vec![
                DbValue::Text("ADMIN".to_string()),
                DbValue::Text("SECURITY".to_string()),
            ],
            vec![DbValue::Text("ADMIN".to_string()), DbValue::Null],
        ]);
        conn.push_rows(vec![vec![DbValue::Int64(1)]]);
        conn.push_rows(vec![vec![DbValue::Uuid(uuid::Uuid::from_u128(2))]]);
        conn.push_rows(vec![vec![
            DbValue::Uuid(uuid::Uuid::from_u128(3)),
            DbValue::Uuid(uuid::Uuid::from_u128(4)),
        ]]);
        conn.push_rows(vec![vec![
            DbValue::Text("PROJECT".to_string()),
            DbValue::Null,
            DbValue::Uuid(uuid::Uuid::from_u128(5)),
        ]]);
        conn.push_rows(vec![vec![DbValue::Text("fin,hr".to_string())]]);
        conn.push_rows(vec![vec![DbValue::Int64(7)]]);
        let mut tx = tx_over(conn);
        let set = q()
            .load_user_authz(
                &mut tx,
                Id::from_uuid(uuid::Uuid::from_u128(7)),
                Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            )
            .await
            .expect("读取可完成");
        assert_eq!(set.role_codes, vec!["ADMIN", "ADMIN"]);
        assert_eq!(set.duty_classes, vec![DutyClass::Security]);
        assert!(set.has_high_risk_permission);
        assert_eq!(set.legal_entity_ids.len(), 1);
        assert_eq!(set.project_ids.len(), 1);
        assert_eq!(set.data_scope_tags, vec!["fin", "hr"]);
        assert_eq!(set.snapshot_version, 7);
    }

    #[tokio::test]
    async fn open_high_risk_count_decodes_int8() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(2)]]);
        let mut tx = tx_over(conn);
        let n = q()
            .count_open_high_risk_requests(&mut tx, Id::from_uuid(uuid::Uuid::from_u128(7)))
            .await
            .expect("计数可完成");
        assert_eq!(n, 2);
        assert!(OPEN_HIGH_RISK_STMT.contains("'IN_APPROVAL'"));
    }

    #[tokio::test]
    async fn probe_is_existence_check_only() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(1)]]);
        let mut tx = tx_over(conn);
        let ok = q()
            .probe_legal_entity_grant(
                &mut tx,
                Id::from_uuid(uuid::Uuid::from_u128(7)),
                Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(2)),
            )
            .await
            .expect("探测可完成");
        assert!(ok);
        assert!(PROBE_LE_GRANT_STMT.contains("limit 1"));
    }
}
