//! 连接抽象。`DbConn` 是本 crate 与「一条数据库连接」之间的唯一界面：
//! 真实路径是 sqlx 的 [`SqlxConn`]，纯逻辑单测路径是 `fake` 模块的假连接。
//!
//! 事务边界用裸 BEGIN/COMMIT/ROLLBACK 语句而不是驱动的 Transaction 句柄：
//! 端口 `Tx::as_any_mut` 要求句柄 `PgTx` 满足 `'static` 并拥有连接，驱动
//! 的事务句柄与连接互为借用，放进同一个所有者里会自引用。
//!
//! SQLSTATE 分类与错误码映射在本模块统一做：40001/40P01 可重试、
//! 23503 映射 `PLATFORM.DB.REFERENCED_ROW_MISSING`、写精度超限映射
//! `PLATFORM.DB.WRITE_SCALE_VIOLATION`，其余一律按内部错误处置。

use ep_foundation::error::codes::{
    PLATFORM_DB_REFERENCED_ROW_MISSING, PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED,
    PLATFORM_DB_WRITE_SCALE_VIOLATION, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::tx::IsolationKind;

/// 数据库返回错误的分类。重试判定与错误码映射都只认分类，不认具体文案。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DbErrorClass {
    /// 序列化失败 40001 与死锁 40P01：只对未产生外部可见副作用的事务重试。
    Retryable,
    /// 外键违约 23503：被引用行缺失。
    ReferencedRowMissing,
    /// 写入数值超出声明精度。
    WriteScale,
    /// 其余错误一律按基础设施故障处置，不重试。
    Other,
}

/// 序列化失败的 SQLSTATE。
pub const SQLSTATE_SERIALIZATION_FAILURE: &str = "40001";
/// 检测到死锁的 SQLSTATE。
pub const SQLSTATE_DEADLOCK_DETECTED: &str = "40P01";
/// 外键违约的 SQLSTATE。
pub const SQLSTATE_FOREIGN_KEY_VIOLATION: &str = "23503";
/// 数值超出范围的 SQLSTATE。写前断言与库端报错共用这一分类。
pub const SQLSTATE_NUMERIC_OUT_OF_RANGE: &str = "22003";

/// 一次数据库错误。携带分类所需的全部元数据：SQLSTATE、约束名、列名。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PgError {
    pub sqlstate: Option<String>,
    pub message: String,
    pub constraint: Option<String>,
    pub column: Option<String>,
}

impl PgError {
    pub fn class(&self) -> DbErrorClass {
        match self.sqlstate.as_deref() {
            Some(SQLSTATE_SERIALIZATION_FAILURE) | Some(SQLSTATE_DEADLOCK_DETECTED) => {
                DbErrorClass::Retryable
            }
            Some(SQLSTATE_FOREIGN_KEY_VIOLATION) => DbErrorClass::ReferencedRowMissing,
            Some(SQLSTATE_NUMERIC_OUT_OF_RANGE) => DbErrorClass::WriteScale,
            _ => DbErrorClass::Other,
        }
    }

    pub fn is_retryable(&self) -> bool {
        self.class() == DbErrorClass::Retryable
    }

    /// 统一映射为已登记的错误码。details（约束名、列名）随 message 携带，
    /// 供 23503 定位外键列与约束名。
    pub fn into_app_error(self) -> AppError {
        match self.class() {
            DbErrorClass::Retryable => {
                // message 尾部附 sqlstate：transact 的重试判定与重试指标的
                // sqlstate 标签都从这一处取，不再回读驱动错误。
                let sqlstate = self.sqlstate.unwrap_or_default();
                AppError::new(
                    PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED,
                    format!("事务因并发冲突未能完成 [sqlstate={sqlstate}]"),
                )
            }
            DbErrorClass::ReferencedRowMissing => {
                let constraint = self.constraint.unwrap_or_else(|| "未知约束".to_string());
                let column = self.column.unwrap_or_else(|| "未知列".to_string());
                AppError::new(
                    PLATFORM_DB_REFERENCED_ROW_MISSING,
                    format!("所引用的记录不存在 [constraint={constraint}, column={column}]"),
                )
            }
            DbErrorClass::WriteScale => AppError::new(
                PLATFORM_DB_WRITE_SCALE_VIOLATION,
                "写入的数值超出列声明的精度范围",
            ),
            DbErrorClass::Other => {
                AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "数据库返回了未预期的错误")
            }
        }
    }
}

/// 参数与结果共用的取值形态。金额不在此列：金额一律以最小单位整数
/// （numeric(18,2) 对应分）经 [`assert_money_minor_units`] 断言后再写。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DbValue {
    Text(String),
    Int64(i64),
    Uuid(uuid::Uuid),
    Bool(bool),
    Timestamp(chrono::DateTime<chrono::Utc>),
    Date(chrono::NaiveDate),
    Null,
}

/// numeric(18,2) 以分承载时的上限：总值不超过 10^18 - 1 分。
pub const MONEY_MINOR_UNITS_MAX: i64 = 999_999_999_999_999_999;

/// 金额写前断言（IT-27 的运行期对应物）。超精度先返
/// `PLATFORM.DB.WRITE_SCALE_VIOLATION`，不让数据库端才报错。
pub fn assert_money_minor_units(minor: i64) -> Result<(), PgError> {
    if minor.abs() > MONEY_MINOR_UNITS_MAX {
        return Err(PgError {
            sqlstate: Some(SQLSTATE_NUMERIC_OUT_OF_RANGE.to_string()),
            message: format!("金额 {minor} 分超出 numeric(18,2) 的可表达范围"),
            constraint: None,
            column: None,
        });
    }
    Ok(())
}

/// 一条连接的最小界面。事务边界是显式语句，调用方（`PgUnitOfWork`）
/// 负责 begin 与 commit/rollback 的配对，连接自身不隐含任何事务状态。
#[async_trait::async_trait]
pub trait DbConn: Send {
    /// 执行一条语句，返回影响行数。
    async fn execute(&mut self, sql: &str, params: &[DbValue]) -> Result<u64, PgError>;

    /// 执行一条查询，按行按列返回取值。
    async fn query(&mut self, sql: &str, params: &[DbValue]) -> Result<Vec<Vec<DbValue>>, PgError>;

    /// 开启事务。`read_only` 为真时附加 READ ONLY（快照事务用）。
    async fn begin(&mut self, isolation: IsolationKind, read_only: bool) -> Result<(), PgError>;

    async fn commit(&mut self) -> Result<(), PgError>;

    async fn rollback(&mut self) -> Result<(), PgError>;

    /// 是否处于未结束事务中。连接归还前的断言依据：
    /// 事务外 `transaction_isolation` 的取值为 `read uncommitted`。
    async fn in_transaction(&mut self) -> Result<bool, PgError>;
}

/// sqlx 连接的具体实现。拥有 `PoolConnection`，随 `PgTx` 移动。
pub struct SqlxConn {
    inner: sqlx::pool::PoolConnection<sqlx::Postgres>,
}

impl SqlxConn {
    pub fn new(inner: sqlx::pool::PoolConnection<sqlx::Postgres>) -> Self {
        Self { inner }
    }
}

fn to_pg_error(err: sqlx::Error) -> PgError {
    match &err {
        sqlx::Error::Database(db) => {
            // PostgreSQL 16 实态：23503 的主消息只带表名与约束名，
            // `Key (<列>)=(<值>) is not present in table …` 段在 detail 字段；
            // 驱动 trait 不暴露 detail，下探到 PgDatabaseError 取，并入 message
            // 使列名可提取，details 同时携带约束名与列名。
            let detail = db
                .try_downcast_ref::<sqlx::postgres::PgDatabaseError>()
                .and_then(|pg| pg.detail())
                .map(str::to_string);
            let message = match &detail {
                Some(d) if !d.is_empty() => format!("{} [detail: {d}]", db.message()),
                _ => db.message().to_string(),
            };
            PgError {
                sqlstate: db.code().map(|c| c.to_string()),
                column: extract_fk_column(&message),
                message,
                constraint: db.constraint().map(str::to_string),
            }
        }
        other => PgError {
            sqlstate: None,
            message: other.to_string(),
            constraint: None,
            column: None,
        },
    }
}

/// 从外键违约文本提取列名。PostgreSQL 的形态是
/// `Key (<列>)=(<值>) is not present in table …`（23503 时在 detail 字段，
/// 已并入 message）。段不一定在文本开头，按子串定位 `Key (` 再截取。
fn extract_fk_column(message: &str) -> Option<String> {
    let rest = message.split("Key (").nth(1)?;
    let cols = rest.split(')').next()?.trim();
    // 复合外键取首列；多列形态用逗号分隔，与消息原文保持一致。
    let first = cols.split(',').next()?.trim();
    (!first.is_empty()).then(|| first.to_string())
}

/// 按参数个数分发到定长绑定的 execute。参数化语句最多九个参数
/// （降级台账开窗语句九参为上限），覆盖本 crate 全部已知调用点；
/// 超出即属实现缺陷，直接 panic。
async fn sqlx_execute(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
    sql: &str,
    params: &[DbValue],
) -> Result<u64, PgError> {
    use sqlx::Executor;
    let mut q = sqlx::query(sql);
    for p in params {
        q = bind_value(q, p);
    }
    let done = conn.as_mut().execute(q).await.map_err(to_pg_error)?;
    Ok(done.rows_affected())
}

fn bind_value<'q>(
    q: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    v: &'q DbValue,
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match v {
        DbValue::Text(s) => q.bind(s.clone()),
        DbValue::Int64(n) => q.bind(*n),
        DbValue::Uuid(u) => q.bind(*u),
        DbValue::Bool(b) => q.bind(*b),
        DbValue::Timestamp(t) => q.bind(*t),
        DbValue::Date(d) => q.bind(*d),
        DbValue::Null => q.bind(Option::<String>::None),
    }
}

/// 单列文本解码。专用类型之外的列一律按文本形态取。
fn decode_text_column(row: &sqlx::postgres::PgRow, idx: usize) -> Result<DbValue, sqlx::Error> {
    use sqlx::Row;
    let v: Option<String> = row.try_get(idx)?;
    Ok(match v {
        Some(s) => DbValue::Text(s),
        None => DbValue::Null,
    })
}

fn decode_row(row: sqlx::postgres::PgRow) -> Result<Vec<DbValue>, sqlx::Error> {
    use sqlx::{Column, Row, TypeInfo};
    let mut out = Vec::with_capacity(row.len());
    for idx in 0..row.len() {
        let ty = row.column(idx).type_info().name().to_string();
        let value = match ty.as_str() {
            "INT8" => row
                .try_get::<Option<i64>, _>(idx)?
                .map_or(DbValue::Null, DbValue::Int64),
            // int4/int2 必须各有分支：落到 decode_text_column 会以 ColumnDecode 失败，
            // 而 ColumnDecode 不是 Database 错误、`sqlstate` 为 None，最终被映射成
            // `PLATFORM.SYSTEM.INTERNAL_ERROR`，看不出真实原因。两者都归一到 Int64，
            // 调用方拿到的仍是同一个变体（F-80）。
            "INT4" => row
                .try_get::<Option<i32>, _>(idx)?
                .map_or(DbValue::Null, |v| DbValue::Int64(i64::from(v))),
            "INT2" => row
                .try_get::<Option<i16>, _>(idx)?
                .map_or(DbValue::Null, |v| DbValue::Int64(i64::from(v))),
            "BOOL" => row
                .try_get::<Option<bool>, _>(idx)?
                .map_or(DbValue::Null, DbValue::Bool),
            "UUID" => row
                .try_get::<Option<uuid::Uuid>, _>(idx)?
                .map_or(DbValue::Null, DbValue::Uuid),
            "TIMESTAMPTZ" => row
                .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>(idx)?
                .map_or(DbValue::Null, DbValue::Timestamp),
            "DATE" => row
                .try_get::<Option<chrono::NaiveDate>, _>(idx)?
                .map_or(DbValue::Null, DbValue::Date),
            _ => decode_text_column(&row, idx)?,
        };
        out.push(value);
    }
    Ok(out)
}

#[async_trait::async_trait]
impl DbConn for SqlxConn {
    async fn execute(&mut self, sql: &str, params: &[DbValue]) -> Result<u64, PgError> {
        sqlx_execute(&mut self.inner, sql, params).await
    }

    async fn query(&mut self, sql: &str, params: &[DbValue]) -> Result<Vec<Vec<DbValue>>, PgError> {
        let mut q = sqlx::query(sql);
        for p in params {
            q = bind_value(q, p);
        }
        let rows = sqlx::Executor::fetch_all(self.inner.as_mut(), q)
            .await
            .map_err(to_pg_error)?;
        rows.into_iter()
            .map(|r| decode_row(r).map_err(to_pg_error))
            .collect()
    }

    async fn begin(&mut self, isolation: IsolationKind, read_only: bool) -> Result<(), PgError> {
        let mut stmt = match isolation {
            IsolationKind::ReadCommitted => "begin".to_string(),
            IsolationKind::RepeatableReadSnapshot => {
                "begin isolation level repeatable read".to_string()
            }
        };
        if read_only {
            stmt.push_str(" read only");
        }
        sqlx_execute(&mut self.inner, &stmt, &[]).await.map(|_| ())
    }

    async fn commit(&mut self) -> Result<(), PgError> {
        sqlx_execute(&mut self.inner, "commit", &[])
            .await
            .map(|_| ())
    }

    async fn rollback(&mut self) -> Result<(), PgError> {
        sqlx_execute(&mut self.inner, "rollback", &[])
            .await
            .map(|_| ())
    }

    async fn in_transaction(&mut self) -> Result<bool, PgError> {
        use sqlx::Row;
        let row = sqlx::query("select current_setting('transaction_isolation')")
            .fetch_one(self.inner.as_mut())
            .await
            .map_err(to_pg_error)?;
        let level: String = row.try_get(0).map_err(to_pg_error)?;
        Ok(level != "read uncommitted")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn err(sqlstate: &str) -> PgError {
        PgError {
            sqlstate: Some(sqlstate.to_string()),
            message: "冲突".to_string(),
            constraint: None,
            column: None,
        }
    }

    #[test]
    fn retryable_sqlstates_are_exactly_40001_and_40p01() {
        assert!(err("40001").is_retryable());
        assert!(err("40P01").is_retryable());
        assert!(!err("23503").is_retryable());
        assert!(!err("22023").is_retryable());
        assert!(
            !PgError {
                sqlstate: None,
                message: String::new(),
                constraint: None,
                column: None
            }
            .is_retryable(),
            "没有 SQLSTATE 的连接层错误不得被当成可重试"
        );
    }

    #[test]
    fn foreign_key_violation_maps_to_referenced_row_missing_with_details() {
        let e = PgError {
            sqlstate: Some("23503".to_string()),
            message: "fk".to_string(),
            constraint: Some("fk_orders_legal_entity_id".to_string()),
            column: Some("legal_entity_id".to_string()),
        };
        let app = e.into_app_error();
        assert_eq!(app.code, PLATFORM_DB_REFERENCED_ROW_MISSING);
        assert!(
            app.message.contains("fk_orders_legal_entity_id"),
            "details 带约束名"
        );
        assert!(app.message.contains("legal_entity_id"), "details 带外键列");
    }

    #[test]
    fn retryable_maps_to_serialization_retry_exhausted_with_sqlstate_marker() {
        let app = err("40001").into_app_error();
        assert_eq!(app.code, PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED);
        assert!(
            app.message.contains("40001"),
            "sqlstate 标记供重试判定与指标"
        );
    }

    #[test]
    fn other_errors_map_to_internal_error() {
        let app = err("22023").into_app_error();
        assert_eq!(app.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }

    #[test]
    fn money_minor_units_boundary() {
        assert!(
            assert_money_minor_units(MONEY_MINOR_UNITS_MAX).is_ok(),
            "上限本身合法"
        );
        assert!(assert_money_minor_units(-MONEY_MINOR_UNITS_MAX).is_ok());
        let e = assert_money_minor_units(MONEY_MINOR_UNITS_MAX + 1).unwrap_err();
        assert_eq!(e.into_app_error().code, PLATFORM_DB_WRITE_SCALE_VIOLATION);
    }
}
