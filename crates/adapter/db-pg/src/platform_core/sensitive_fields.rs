//! 敏感字段登记表的只读查询（A-07，02 计划 §3.5 表四）。
//!
//! 登记表不带法人列、不建行级安全策略（第 14 号迁移登记豁免）。
//! 响应不含任何样例值——本模块只取登记元数据，样例值自始不存在于本表。

use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR;
use ep_foundation::error::AppError;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::security::SecurityContext;

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

/// 敏感字段登记行（对外只暴露这五列元数据）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SensitiveFieldRow {
    pub schema_name: String,
    pub table_name: String,
    pub column_name: String,
    pub category: String,
    pub is_field_encrypted: bool,
}

/// 三个过滤条件逐位可选，命中即等值收窄。
#[derive(Clone, Default, Debug)]
pub struct SensitiveFieldFilter {
    pub schema_name: Option<String>,
    pub table_name: Option<String>,
    pub category: Option<String>,
}

const BASE_STMT: &str = "select schema_name, table_name, column_name, category, \
     is_field_encrypted from platform_core.sensitive_field_registry";

/// 敏感字段登记表的 PostgreSQL 只读取数层。装配时绑定 Rw 池工作单元。
pub struct PgSensitiveFieldRegistry {
    uow: Arc<PgUnitOfWork>,
}

impl PgSensitiveFieldRegistry {
    pub fn new(uow: Arc<PgUnitOfWork>) -> Self {
        Self { uow }
    }

    /// 按过滤条件查询登记行，按三元组排序保证输出稳定。
    /// 过滤值经参数化绑定下发，不拼接进语句文本。
    pub async fn list(
        &self,
        ctx: &SecurityContext,
        filter: SensitiveFieldFilter,
    ) -> Result<Vec<SensitiveFieldRow>, AppError> {
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    // 过滤条件逐个追加：语句骨架固定，绑定参数按出现序编号。
                    let mut stmt = BASE_STMT.to_string();
                    let mut params: Vec<DbValue> = Vec::new();
                    let mut clauses: Vec<String> = Vec::new();
                    if let Some(v) = filter.schema_name {
                        params.push(DbValue::Text(v));
                        clauses.push(format!("schema_name = ${}", params.len()));
                    }
                    if let Some(v) = filter.table_name {
                        params.push(DbValue::Text(v));
                        clauses.push(format!("table_name = ${}", params.len()));
                    }
                    if let Some(v) = filter.category {
                        params.push(DbValue::Text(v));
                        clauses.push(format!("category = ${}", params.len()));
                    }
                    if !clauses.is_empty() {
                        stmt.push_str(" where ");
                        stmt.push_str(&clauses.join(" and "));
                    }
                    stmt.push_str(" order by schema_name, table_name, column_name");
                    let rows = pg.query(&stmt, &params).await?;
                    rows.iter()
                        .map(|row| {
                            Ok(SensitiveFieldRow {
                                schema_name: text_of(row.first())?,
                                table_name: text_of(row.get(1))?,
                                column_name: text_of(row.get(2))?,
                                category: text_of(row.get(3))?,
                                is_field_encrypted: bool_of(row.get(4))?,
                            })
                        })
                        .collect()
                })
            })
            .await
    }
}

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "敏感字段查询必须在 PostgreSQL 事务内执行",
        )
    })
}

fn shape(what: &'static str) -> AppError {
    AppError::new(
        PLATFORM_SYSTEM_INTERNAL_ERROR,
        format!("敏感字段登记行形态不符：{what}"),
    )
}

fn text_of(value: Option<&DbValue>) -> Result<String, AppError> {
    match value {
        Some(DbValue::Text(s)) => Ok(s.clone()),
        _ => Err(shape("文本列")),
    }
}

fn bool_of(value: Option<&DbValue>) -> Result<bool, AppError> {
    match value {
        Some(DbValue::Bool(b)) => Ok(*b),
        _ => Err(shape("布尔列")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeConn;
    use crate::metrics::NoopDbMetrics;
    use crate::retry::RetryPolicy;
    use ep_foundation::id::Id;
    use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
    use ep_foundation::security::context::{RequestId, TraceId};

    fn registry_with(conn: FakeConn) -> PgSensitiveFieldRegistry {
        let uow = Arc::new(PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "rw",
            RetryPolicy::standard(),
            Arc::new(NoopDbMetrics),
        ));
        PgSensitiveFieldRegistry::new(uow)
    }

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("sensitive-tests").expect("固定取值合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值合法"),
        )
    }

    #[tokio::test]
    async fn rows_decode_five_metadata_columns() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![vec![
            DbValue::Text("mdm".into()),
            DbValue::Text("customers".into()),
            DbValue::Text("phone_no".into()),
            DbValue::Text("CONTACT".into()),
            DbValue::Bool(true),
        ]]);
        let reg = registry_with(conn);
        let rows = reg
            .list(&ctx(), SensitiveFieldFilter::default())
            .await
            .expect("解码成功");
        assert_eq!(rows.len(), 1);
        assert!(rows[0].is_field_encrypted);
    }
}
