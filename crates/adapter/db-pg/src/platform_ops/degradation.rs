//! 降级窗口台账的 SQL 实现（裁定 A-26，02 计划 §3.5 表十二）。
//!
//! 表是 `platform_ops.degradation_windows`（第 104500 号迁移建表）。
//! 活动窗口的判据是哨兵取值 `closed_at = 'infinity'`：开窗时该列取表默认值，
//! 关窗即把它改写为当前时刻。唯一约束
//! `ux_degradation_windows_kind_scope_closed` 建在 kind、subject、两个 scope
//! 列与 closed_at 五者上，同一对象至多一条活动条目；重复开窗以 23505 拒绝，
//! 本模块把它映射为已登记的冲突类错误码。
//!
//! 端口 `DegradationLedger` 与其 kind 枚举在 ep-platform-obs；基线第 1.3 节
//! 允许 adapter 依赖 platform 端口 trait（tenancy 先例），本文件即台账一侧
//! 的落点。`ep_degradation_windows_open` gauge 按 IT-41 在 open/close 成功后
//! 刷新，取值与 `open_count` 一致——台账与指标双出，不得只有其一。

use std::sync::Arc;

use ep_foundation::error::codes::{
    PLATFORM_DB_MIGRATION_WINDOW_CONFLICT, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::id::Id;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
use ep_foundation::security::context::{RequestId, TraceId};
use ep_foundation::security::SecurityContext;
use ep_platform_obs::degradation::{DegradationLedger, WindowOpenRequest, WindowRef};
use ep_platform_obs::MetricsRegistry;

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

/// 唯一约束违约的 SQLSTATE。
const SQLSTATE_UNIQUE_VIOLATION: &str = "23505";

/// 开窗语句。`opened_at`、审计四列与 `closed_at` 哨兵都取表默认值，
/// 九个绑定参数覆盖其余必填列。
const OPEN_STMT: &str = "insert into platform_ops.degradation_windows \
     (id, kind, subject, scope_key, scope_legal_entity_id, scope_accounting_period_id, \
      basis, closing_condition, is_suppressible) \
     values ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

/// 关窗语句。定位五元组逐列可比（空值用 IS NOT DISTINCT FROM），
/// 只改活动条目；影响行数为零即窗口已关，按幂等约定不报错。
const CLOSE_STMT: &str = "update platform_ops.degradation_windows \
     set closed_at = now(), row_version = row_version + 1, updated_at = now() \
     where kind = $1 and subject is not distinct from $2 \
     and scope_legal_entity_id is not distinct from $3 \
     and scope_accounting_period_id is not distinct from $4 \
     and closed_at = 'infinity'";

/// 活动窗口计数：gauge 与巡检共用这一句。
const COUNT_STMT: &str =
    "select count(*) from platform_ops.degradation_windows where closed_at = 'infinity'";

/// 降级窗口台账的 PostgreSQL 实现。装配时绑定 Ops 池的工作单元
/// 与指标注册表；注册表用于开窗/关窗成功后刷新 gauge。
pub struct PgDegradationLedger {
    uow: Arc<PgUnitOfWork>,
    metrics: Arc<MetricsRegistry>,
}

impl PgDegradationLedger {
    pub fn new(uow: Arc<PgUnitOfWork>, metrics: Arc<MetricsRegistry>) -> Self {
        Self { uow, metrics }
    }

    /// 台账写入是系统行为，与法人目录同口径取系统上下文。
    fn system_ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("degradation-ledger").expect("固定取值长度合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值形态合法"),
        )
    }

    /// open/close 成功后的 gauge 刷新：先取计数再写指标，
    /// 保证与 `open_count` 一致（IT-41）。取数失败原样上抛；
    /// 指标名已在注册表登记，写入失败只可能是登记表被改坏，不吞。
    async fn refresh_gauge(&self) -> Result<(), AppError> {
        let count = self.open_count().await?;
        self.metrics
            .set_gauge("ep_degradation_windows_open", &[], count as f64)
            .expect("降级窗口 gauge 已在指标登记表注册");
        Ok(())
    }
}

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "降级台账写入必须在 PostgreSQL 事务内执行",
        )
    })
}

#[async_trait::async_trait]
impl DegradationLedger for PgDegradationLedger {
    async fn open(&self, req: WindowOpenRequest) -> Result<(), AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let params = [
                        DbValue::Uuid(uuid::Uuid::now_v7()),
                        DbValue::Text(req.kind.as_str().to_string()),
                        match req.subject {
                            Some(s) => DbValue::Text(s),
                            None => DbValue::Null,
                        },
                        DbValue::Text(req.scope_key),
                        match req.scope_legal_entity_id {
                            Some(id) => DbValue::Uuid(id.as_uuid()),
                            None => DbValue::Null,
                        },
                        match req.scope_accounting_period_id {
                            Some(id) => DbValue::Uuid(id.as_uuid()),
                            None => DbValue::Null,
                        },
                        DbValue::Text(req.basis),
                        DbValue::Text(req.closing_condition),
                        DbValue::Bool(req.is_suppressible),
                    ];
                    if let Err(app_err) = pg.execute(OPEN_STMT, &params).await {
                        // PgTx::execute 已把 PgError 映射为 AppError 并留存原始错误；
                        // 重复开窗按冲突类错误码拒绝，其余原样上抛。
                        let is_duplicate = pg.last_pg_error.as_ref().is_some_and(|e| {
                            e.sqlstate.as_deref() == Some(SQLSTATE_UNIQUE_VIOLATION)
                                && e.constraint
                                    .as_deref()
                                    .is_some_and(|c| c.contains("ux_degradation_windows"))
                        });
                        if is_duplicate {
                            return Err(AppError::new(
                                PLATFORM_DB_MIGRATION_WINDOW_CONFLICT,
                                "同一对象的降级窗口已登记，不得重复开窗",
                            ));
                        }
                        return Err(app_err);
                    }
                    Ok(())
                })
            })
            .await?;
        self.refresh_gauge().await
    }

    async fn close(&self, window: WindowRef) -> Result<(), AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let params = [
                        DbValue::Text(window.kind.as_str().to_string()),
                        match window.subject {
                            Some(s) => DbValue::Text(s),
                            None => DbValue::Null,
                        },
                        match window.scope_legal_entity_id {
                            Some(id) => DbValue::Uuid(id.as_uuid()),
                            None => DbValue::Null,
                        },
                        match window.scope_accounting_period_id {
                            Some(id) => DbValue::Uuid(id.as_uuid()),
                            None => DbValue::Null,
                        },
                    ];
                    pg.execute(CLOSE_STMT, &params).await?;
                    Ok(())
                })
            })
            .await?;
        self.refresh_gauge().await
    }

    async fn open_count(&self) -> Result<usize, AppError> {
        let ctx = Self::system_ctx();
        self.uow
            .transact(&ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(COUNT_STMT, &[]).await?;
                    let row = rows.first().ok_or_else(|| {
                        AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "降级台账计数查询未返回行")
                    })?;
                    match row.first() {
                        Some(DbValue::Int64(n)) => usize::try_from(*n).map_err(|_| {
                            AppError::new(
                                PLATFORM_SYSTEM_INTERNAL_ERROR,
                                "降级台账计数超出可表达范围",
                            )
                        }),
                        _ => Err(AppError::new(
                            PLATFORM_SYSTEM_INTERNAL_ERROR,
                            "降级台账计数列形态不符",
                        )),
                    }
                })
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conn::PgError;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;
    use crate::retry::RetryPolicy;
    use ep_platform_obs::degradation::DegradationKind;

    fn ledger_with(conn: FakeConn) -> (PgDegradationLedger, Arc<MetricsRegistry>) {
        let metrics = Arc::new(MetricsRegistry::new());
        let uow = Arc::new(PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "ops",
            RetryPolicy::standard(),
            Arc::new(NoopDbMetrics),
        ));
        (PgDegradationLedger::new(uow, metrics.clone()), metrics)
    }

    fn req() -> WindowOpenRequest {
        WindowOpenRequest {
            kind: DegradationKind::PortNotImplemented,
            subject: Some("platform.key_domain.provisioning".into()),
            scope_key: "legal-entity:x".into(),
            scope_legal_entity_id: None,
            scope_accounting_period_id: None,
            basis: "测试依据".into(),
            closing_condition: "测试关窗条件".into(),
            is_suppressible: false,
        }
    }

    #[tokio::test]
    async fn open_then_refresh_sets_the_gauge_from_count() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 1; // insert 成功
        conn.push_rows(vec![vec![DbValue::Int64(1)]]); // refresh 的计数查询
        let (ledger, registry) = ledger_with(conn);
        ledger.open(req()).await.expect("开窗成功");
        let text = registry.encode_text();
        assert!(
            text.contains("ep_degradation_windows_open 1"),
            "gauge 必须刷新为计数取值：{text}"
        );
    }

    #[tokio::test]
    async fn duplicate_open_maps_unique_violation_to_conflict_code() {
        let mut conn = FakeConn::new();
        conn.fail_next(PgError {
            sqlstate: Some(SQLSTATE_UNIQUE_VIOLATION.into()),
            message: "duplicate key".into(),
            constraint: Some("ux_degradation_windows_kind_scope_closed".into()),
            column: None,
        });
        let (ledger, _) = ledger_with(conn);
        let err = ledger.open(req()).await.expect_err("重复开窗必须被拒");
        assert_eq!(err.code, PLATFORM_DB_MIGRATION_WINDOW_CONFLICT);
    }

    #[tokio::test]
    async fn close_is_idempotent_and_refreshes_gauge() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0; // 无活动窗口：幂等视为已完成
        conn.push_rows(vec![vec![DbValue::Int64(0)]]);
        let (ledger, registry) = ledger_with(conn);
        let window = WindowRef {
            kind: DegradationKind::PortNotImplemented,
            subject: Some("platform.key_domain.provisioning".into()),
            scope_legal_entity_id: None,
            scope_accounting_period_id: None,
        };
        ledger.close(window).await.expect("关窗幂等");
        assert!(registry
            .encode_text()
            .contains("ep_degradation_windows_open 0"));
    }

    #[tokio::test]
    async fn open_count_decodes_int8_column() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![DbValue::Int64(3)]]);
        let (ledger, _) = ledger_with(conn);
        assert_eq!(ledger.open_count().await.expect("计数可解码"), 3);
    }
}
