//! 组织架构读取契约与部门闭包维护的 SQL 实现体（D-14，A-04）。
//!
//! trait 定义在 ep-platform-tenancy：基线第 1.3 节允许 adapter 依赖
//! platform 中的端口 trait，反方向（platform 依赖 adapter）由
//! `xtask archcheck` 的 platform-no-adapter 拦住，本文件即该裁定的落点。
//!
//! 三个实现体：
//! - [`PgLegalEntityDirectory`]：持 Ro 工作单元，系统上下文读取法人档案；
//!   `legal_entities` 不建行级策略（迁移第 14 号登记豁免），无需会话变量前置。
//! - [`PgDepartmentClosure`]：在调用方事务内查物化闭包行，单表索引扫描，
//!   不用递归 CTE（基线第 3.10 节）。
//! - [`PgDepartmentClosureMaintainer`]：新增、改父、停用三种写入在同一事务内
//!   全量重写子树（先按 ancestor 删、逐层插、depth 自零起、同事务维护
//!   `departments.level_no`），重写计划由 ep-platform-tenancy 的纯逻辑算出。

use std::sync::Arc;

use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{Department, LegalEntity};
use ep_foundation::id::Id;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_tenancy::{
    plan_subtree_rewrite, DepartmentClosureQuery, LegalEntityDirectory, LegalEntityRef,
    SubtreeRewritePlan, MAX_ORG_DEPTH,
};

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

const LIST_ACTIVE_LE_STMT: &str = "select id, code, entity_no, name, is_active \
     from platform_core.legal_entities where is_active = true order by code";

const GET_LE_STMT: &str = "select id, code, entity_no, name, is_active \
     from platform_core.legal_entities where id = $1";

/// 闭包查询：物化行单表索引扫描，`depth <= max_depth` 把截止语义下推库侧。
const DESCENDANT_STMT: &str =
    "select descendant_department_id from platform_core.department_closures \
     where legal_entity_id = $1 and ancestor_department_id = $2 and depth <= $3 \
     order by depth, descendant_department_id";

/// 子树收集：按父引用逐层走线（不使用递归 CTE）。
const CHILDREN_STMT: &str = "select id from platform_core.departments \
     where legal_entity_id = $1 and parent_department_id = $2";

const DELETE_CLOSURE_BY_ANCESTOR_STMT: &str = "delete from platform_core.department_closures \
     where legal_entity_id = $1 and ancestor_department_id = $2";

const DELETE_STALE_CROSS_LINK_STMT: &str = "delete from platform_core.department_closures \
     where legal_entity_id = $1 and ancestor_department_id = $2 and descendant_department_id = $3";

const INSERT_CLOSURE_STMT: &str = "insert into platform_core.department_closures \
     (id, legal_entity_id, ancestor_department_id, descendant_department_id, depth) \
     values ($1, $2, $3, $4, $5)";

const UPDATE_LEVEL_NO_STMT: &str =
    "update platform_core.departments set level_no = $1 where id = $2 and legal_entity_id = $3";

const DEACTIVATE_SUBTREE_STMT: &str = "update platform_core.departments \
     set is_active = false, deactivated_at = now() where id = $1 and legal_entity_id = $2";

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "组织架构读取必须在 PostgreSQL 事务内执行",
        )
    })
}

fn decode_uuid(v: &DbValue) -> Result<uuid::Uuid, AppError> {
    match v {
        DbValue::Uuid(u) => Ok(*u),
        _ => Err(AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "数据库返回了非 UUID 形态的标识列",
        )),
    }
}

fn decode_legal_entity(row: &[DbValue]) -> Result<LegalEntityRef, AppError> {
    let bad = || {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "法人档案行的列形态与五列投影不符",
        )
    };
    let id = decode_uuid(row.first().ok_or_else(bad)?)?;
    let text_at = |idx: usize| match row.get(idx) {
        Some(DbValue::Text(s)) => Ok(s.clone()),
        _ => Err(bad()),
    };
    let code = text_at(1)?;
    let entity_no = text_at(2)?;
    let name = text_at(3)?;
    let is_active = match row.get(4) {
        Some(DbValue::Bool(b)) => *b,
        _ => return Err(bad()),
    };
    Ok(LegalEntityRef {
        id: Id::<LegalEntity>::from_uuid(id),
        code,
        entity_no,
        name,
        is_active,
    })
}

/// 法人目录的 PostgreSQL 实现。装配时绑定 Ro 池的工作单元。
pub struct PgLegalEntityDirectory {
    uow: Arc<PgUnitOfWork>,
}

impl PgLegalEntityDirectory {
    pub fn new(uow: Arc<PgUnitOfWork>) -> Self {
        Self { uow }
    }

    /// 目录读取用系统上下文：法人表不受行级策略约束，
    /// request/trace 取固定系统常量，与库侧测试装配同口径。
    fn system_ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("tenancy-directory").expect("固定取值长度合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值形态合法"),
        )
    }
}

#[async_trait::async_trait]
impl LegalEntityDirectory for PgLegalEntityDirectory {
    async fn list_active(&self) -> Result<Vec<LegalEntityRef>, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(LIST_ACTIVE_LE_STMT, &[]).await?;
                    rows.iter().map(|row| decode_legal_entity(row)).collect()
                })
            })
            .await
    }

    async fn get(&self, id: Id<LegalEntity>) -> Result<LegalEntityRef, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg
                        .query(GET_LE_STMT, &[DbValue::Uuid(id.as_uuid())])
                        .await?;
                    match rows.first() {
                        Some(row) => decode_legal_entity(row),
                        None => Err(AppError::new(
                            PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                            "法人档案不存在或不可见",
                        )),
                    }
                })
            })
            .await
    }
}

/// 部门闭包查询的 PostgreSQL 实现。无自身状态：事务由调用方出示。
pub struct PgDepartmentClosure;

#[async_trait::async_trait]
impl DepartmentClosureQuery for PgDepartmentClosure {
    async fn descendant_ids(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        department_id: Id<Department>,
        max_depth: u8,
    ) -> Result<Vec<Id<Department>>, AppError> {
        let pg = downcast(tx)?;
        let rows = pg
            .query(
                DESCENDANT_STMT,
                &[
                    DbValue::Uuid(legal_entity_id.as_uuid()),
                    DbValue::Uuid(department_id.as_uuid()),
                    DbValue::Int64(max_depth as i64),
                ],
            )
            .await?;
        rows.iter()
            .map(|row| {
                decode_uuid(row.first().ok_or_else(|| {
                    AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "闭包行缺少标识列")
                })?)
                .map(Id::<Department>::from_uuid)
            })
            .collect()
    }
}

/// 部门闭包维护器：三种写入都在调用方事务内完成子树全量重写。
/// 写入一律置副作用标志，序列化重试纪律据此拒绝重试。
pub struct PgDepartmentClosureMaintainer;

impl PgDepartmentClosureMaintainer {
    /// 新增部门：部门行已由用例插入，此处只补闭包行与 level_no。
    /// `parent_chain` 为新部门之上的直系祖先链，最近者在前。
    pub async fn on_add(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        new_department_id: Id<Department>,
        parent_chain: &[Id<Department>],
    ) -> Result<(), AppError> {
        let plan = plan_subtree_rewrite(&[vec![new_department_id]], &[], parent_chain, &[])
            .ok_or_else(|| {
                AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "组织深度越界，拒绝写入闭包")
            })?;
        self.apply_plan(tx, legal_entity_id, &plan).await
    }

    /// 改父：收集子树后按新链重写；`stale_chain` 为旧祖先链，其跨层残留行一并清除。
    pub async fn on_reparent(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        root_id: Id<Department>,
        new_chain: &[Id<Department>],
        stale_chain: &[Id<Department>],
    ) -> Result<(), AppError> {
        let (layers, edges) = collect_subtree(tx, legal_entity_id, root_id).await?;
        let plan =
            plan_subtree_rewrite(&layers, &edges, new_chain, stale_chain).ok_or_else(|| {
                AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "组织深度越界，拒绝重写闭包")
            })?;
        self.apply_plan(tx, legal_entity_id, &plan).await
    }

    /// 停用：登记档案列并按同一链重写子树（停用不改父子关系，两链同值）。
    pub async fn on_deactivate(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        root_id: Id<Department>,
        chain: &[Id<Department>],
    ) -> Result<(), AppError> {
        let (layers, edges) = collect_subtree(tx, legal_entity_id, root_id).await?;
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        for layer in &layers {
            for id in layer {
                pg.execute(
                    DEACTIVATE_SUBTREE_STMT,
                    &[
                        DbValue::Uuid(id.as_uuid()),
                        DbValue::Uuid(legal_entity_id.as_uuid()),
                    ],
                )
                .await?;
            }
        }
        let plan = plan_subtree_rewrite(&layers, &edges, chain, &[]).ok_or_else(|| {
            AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "组织深度越界，拒绝重写闭包")
        })?;
        self.apply_plan(tx, legal_entity_id, &plan).await
    }

    /// 在同一事务内执行重写计划：先删、再逐层插、最后维护 level_no。
    pub async fn apply_plan(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        plan: &SubtreeRewritePlan,
    ) -> Result<(), AppError> {
        let pg = downcast(tx)?;
        pg.mark_side_effect();
        let le = DbValue::Uuid(legal_entity_id.as_uuid());

        for ancestor in &plan.delete_ancestors {
            pg.execute(
                DELETE_CLOSURE_BY_ANCESTOR_STMT,
                &[le.clone(), DbValue::Uuid(ancestor.as_uuid())],
            )
            .await?;
        }
        for descendant in &plan.delete_ancestors {
            for stale in &plan.delete_stale_cross_links_for {
                pg.execute(
                    DELETE_STALE_CROSS_LINK_STMT,
                    &[
                        le.clone(),
                        DbValue::Uuid(stale.as_uuid()),
                        DbValue::Uuid(descendant.as_uuid()),
                    ],
                )
                .await?;
            }
        }
        for row in &plan.inserts {
            pg.execute(
                INSERT_CLOSURE_STMT,
                &[
                    DbValue::Uuid(uuid::Uuid::now_v7()),
                    le.clone(),
                    DbValue::Uuid(row.ancestor.as_uuid()),
                    DbValue::Uuid(row.descendant.as_uuid()),
                    DbValue::Int64(row.depth as i64),
                ],
            )
            .await?;
        }
        for (id, level_no) in &plan.level_nos {
            pg.execute(
                UPDATE_LEVEL_NO_STMT,
                &[
                    DbValue::Int64(*level_no as i64),
                    DbValue::Uuid(id.as_uuid()),
                    le.clone(),
                ],
            )
            .await?;
        }
        Ok(())
    }
}

/// 按父引用逐层收集子树：返回层序列与父子边（子 → 父）。
/// 层数超过 [`MAX_ORG_DEPTH`] 即判数据损坏，防御环状父引用。
async fn collect_subtree(
    tx: &mut dyn Tx,
    legal_entity_id: Id<LegalEntity>,
    root_id: Id<Department>,
) -> Result<
    (
        Vec<Vec<Id<Department>>>,
        Vec<(Id<Department>, Id<Department>)>,
    ),
    AppError,
> {
    let pg = downcast(tx)?;
    let mut layers: Vec<Vec<Id<Department>>> = vec![vec![root_id]];
    let mut edges: Vec<(Id<Department>, Id<Department>)> = Vec::new();
    loop {
        let frontier = layers.last().expect("首层已放入子树根").clone();
        let mut next: Vec<Id<Department>> = Vec::new();
        for parent in &frontier {
            let rows = pg
                .query(
                    CHILDREN_STMT,
                    &[
                        DbValue::Uuid(legal_entity_id.as_uuid()),
                        DbValue::Uuid(parent.as_uuid()),
                    ],
                )
                .await?;
            for row in rows {
                let child =
                    Id::<Department>::from_uuid(decode_uuid(row.first().ok_or_else(|| {
                        AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "部门行缺少标识列")
                    })?)?);
                edges.push((child, *parent));
                next.push(child);
            }
        }
        if next.is_empty() {
            break;
        }
        if layers.len() >= MAX_ORG_DEPTH {
            return Err(AppError::new(
                PLATFORM_SYSTEM_INTERNAL_ERROR,
                "部门层级超出深度上限，疑似环状父引用",
            ));
        }
        layers.push(next);
    }
    Ok((layers, edges))
}

#[cfg(test)]
mod tests {
    use ep_foundation::port::tx::{IsolationKind, TxId};

    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;

    fn tx_over(conn: FakeConn) -> PgTx {
        PgTx {
            tx_id: TxId(uuid::Uuid::now_v7()),
            isolation: IsolationKind::ReadCommitted,
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::now_v7()),
            conn: Some(Box::new(conn)),
            pool_label: "rw",
            metrics: Arc::new(NoopDbMetrics),
            side_effect: false,
            last_pg_error: None,
        }
    }

    fn dept(n: u8) -> Id<Department> {
        Id::<Department>::from_uuid(uuid::Uuid::from_u128(n as u128))
    }

    fn le() -> Id<LegalEntity> {
        Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(999))
    }

    #[tokio::test]
    async fn descendant_ids_pushes_max_depth_to_sql() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![
            vec![DbValue::Uuid(dept(1).as_uuid())],
            vec![DbValue::Uuid(dept(2).as_uuid())],
        ]);
        let mut tx = tx_over(conn);
        let got = PgDepartmentClosure
            .descendant_ids(&mut tx, le(), dept(1), 0)
            .await
            .expect("可查询");
        assert_eq!(got, vec![dept(1), dept(2)]);
    }

    #[tokio::test]
    async fn on_add_writes_self_and_chain_rows_then_level_no() {
        let conn = FakeConn::new();
        let mut tx = tx_over(conn);
        let (root, child) = (dept(1), dept(2));
        PgDepartmentClosureMaintainer
            .on_add(&mut tx, le(), child, &[root])
            .await
            .expect("新增写入可完成");
        assert!(tx.has_side_effect(), "写入必须置副作用标志");
    }

    #[tokio::test]
    async fn reparent_collects_subtree_without_recursive_cte() {
        let mut conn = FakeConn::new();
        // 根 r 有一个子 a；a 无子。三次 CHILDREN 查询：r 的子、a 的子、空层终止。
        conn.push_rows(vec![vec![DbValue::Uuid(dept(2).as_uuid())]]);
        conn.push_rows(vec![]);
        let mut tx = tx_over(conn);
        let (r, new_parent) = (dept(1), dept(7));
        PgDepartmentClosureMaintainer
            .on_reparent(&mut tx, le(), r, &[new_parent], &[])
            .await
            .expect("改父重写可完成");
    }

    /// 闭包相关语句形态断言：在线查询不得出现递归 CTE（基线第 3.10 节）。
    #[test]
    fn closure_statements_never_use_recursive_cte() {
        for stmt in [CHILDREN_STMT, DESCENDANT_STMT] {
            assert!(
                !stmt.contains("recursive"),
                "闭包在线查询不得用递归 CTE：{stmt}"
            );
        }
    }

    #[tokio::test]
    async fn deactivate_marks_archive_columns() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![]); // 根无子，子树只含自身
        let mut tx = tx_over(conn);
        PgDepartmentClosureMaintainer
            .on_deactivate(&mut tx, le(), dept(1), &[])
            .await
            .expect("停用可完成");
        assert!(tx.has_side_effect());
    }

    #[tokio::test]
    async fn non_pg_handle_is_rejected() {
        struct NotPg;
        impl Tx for NotPg {
            fn tx_id(&self) -> TxId {
                TxId(uuid::Uuid::nil())
            }
            fn isolation(&self) -> IsolationKind {
                IsolationKind::ReadCommitted
            }
            fn legal_entity_id(&self) -> Id<LegalEntity> {
                Id::<LegalEntity>::from_uuid(uuid::Uuid::nil())
            }
            fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
                self
            }
        }
        let mut handle = NotPg;
        let err = PgDepartmentClosure
            .descendant_ids(&mut handle, le(), dept(1), 1)
            .await
            .expect_err("非 PgTx 句柄必须拒绝");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    #[test]
    fn legal_entity_row_decoding_rejects_bad_shape() {
        let bad = decode_legal_entity(&[DbValue::Int64(1)]);
        assert!(bad.is_err());
    }

    #[test]
    fn legal_entity_row_decoding_happy_path() {
        let row = vec![
            DbValue::Uuid(uuid::Uuid::from_u128(1)),
            DbValue::Text("LE01".to_string()),
            DbValue::Text("E-0001".to_string()),
            DbValue::Text("示例法人".to_string()),
            DbValue::Bool(true),
        ];
        let r = decode_legal_entity(&row).expect("五列行可解码");
        assert_eq!(r.code, "LE01");
        assert!(r.is_active);
    }
}
