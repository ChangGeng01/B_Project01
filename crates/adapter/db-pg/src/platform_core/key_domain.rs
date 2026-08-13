//! 密钥域与数据密钥的 SQL 存取（02 计划 §3.5 表二表三、§4.2 状态机载体）。
//!
//! 两表都带法人列并挂了 `apply_le_rls` 模板策略，会话变量缺失即默认拒绝：
//! 因此全部读写必须在携带目标法人会话上下文的事务内执行，跨法人枚举由
//! 调用方逐法人切换上下文完成（启动自检的缺域核验即此形态）。
//!
//! `wrapped_key` 是 bytea 列而 `DbValue` 暂无字节形态，绑定与解码一律经
//! `decode($n, 'hex')` 与 `encode(col, 'hex')` 走十六进制文本，封包字节
//! 本身是安全形态，可以出载体入库。
//!
//! 轮换的并发串行按 §6 锁策略落在事务内建议锁：
//! `pg_advisory_xact_lock(hashtextextended('key_domain:' || $1 || ':' || $2, 0))`，
//! 同域同 purpose 的第二个轮换在事务层面排队，进程内互斥由 KMS 载体另承担。

use std::sync::Arc;

use chrono::{DateTime, Utc};
use ep_foundation::error::codes::{
    PLATFORM_CONCURRENCY_STALE_VERSION, PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::error::AppError;
use ep_foundation::port::tx::{Tx, UnitOfWork};
use ep_foundation::security::SecurityContext;
use uuid::Uuid;

use crate::conn::DbValue;
use crate::tx::{PgTx, PgUnitOfWork};

/// 密钥域一行（A-01/A-02 的响应材料；`active_key_count` 由聚合列带出）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyDomainRow {
    pub id: Uuid,
    pub legal_entity_id: Uuid,
    pub domain_kind: String,
    pub state: String,
    pub kek_version: i32,
    pub provisioned_at: Option<DateTime<Utc>>,
    pub row_version: i64,
    pub active_key_count: i64,
}

/// 数据密钥一行。`wrapped_key_hex` 只供重新注入载体使用，
/// 任何对外响应都不得携带（02 计划 A-02 明令）。
/// `row_version` 供轮换时的旧版本乐观锁谓词使用，同样不对外。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataKeyRow {
    pub id: Uuid,
    pub purpose: String,
    pub security_level_scope: i16,
    pub version: i32,
    pub algorithm: String,
    pub wrapped_key_hex: String,
    pub wrap_kek_version: i32,
    pub state: String,
    pub activated_at: DateTime<Utc>,
    pub row_version: i64,
}

/// 数据密钥插入材料（provision 与 rotate 共用）。
pub struct DataKeyInsert {
    pub id: Uuid,
    pub key_domain_id: Uuid,
    pub purpose: &'static str,
    pub security_level_scope: u8,
    pub version: u16,
    pub algorithm: &'static str,
    pub wrapped_key_hex: String,
    pub wrap_kek_version: u32,
}

/// 轮换结果（A-04 响应的库侧材料）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RotationRows {
    pub new_data_key_id: Uuid,
    pub new_version: i32,
    pub retiring_data_key_id: Uuid,
}

const LIST_DOMAINS_STMT: &str = "select d.id, d.legal_entity_id, d.domain_kind, d.state, \
     d.kek_version, d.provisioned_at, d.row_version, \
     (select count(*) from platform_core.data_keys k \
      where k.key_domain_id = d.id and k.state = 'ACTIVE') \
     from platform_core.key_domains d order by d.created_at";

const GET_DOMAIN_STMT: &str = "select d.id, d.legal_entity_id, d.domain_kind, d.state, \
     d.kek_version, d.provisioned_at, d.row_version, \
     (select count(*) from platform_core.data_keys k \
      where k.key_domain_id = d.id and k.state = 'ACTIVE') \
     from platform_core.key_domains d where d.id = $1";

const LIST_KEYS_STMT: &str = "select id, purpose, security_level_scope, version, algorithm, \
     encode(wrapped_key, 'hex'), wrap_kek_version, state, activated_at, row_version \
     from platform_core.data_keys where key_domain_id = $1 order by purpose, version";

const DOMAIN_OF_KIND_STMT: &str = "select d.id, d.legal_entity_id, d.domain_kind, d.state, \
     d.kek_version, d.provisioned_at, d.row_version, \
     (select count(*) from platform_core.data_keys k \
      where k.key_domain_id = d.id and k.state = 'ACTIVE') \
     from platform_core.key_domains d where d.domain_kind = $1";

const COUNT_DOMAINS_STMT: &str = "select count(*) from platform_core.key_domains";

const INSERT_DOMAIN_STMT: &str = "insert into platform_core.key_domains \
     (id, legal_entity_id, domain_kind, state, kek_ref, kek_version) \
     values ($1, $2, $3, 'PROVISIONING', $4, 1)";

const INSERT_KEY_STMT: &str = "insert into platform_core.data_keys \
     (id, legal_entity_id, key_domain_id, purpose, security_level_scope, version, \
      algorithm, wrapped_key, wrap_kek_version, state) \
     values ($1, $2, $3, $4, $5, $6, $7, decode($8, 'hex'), $9, 'ACTIVE')";

const ACTIVATE_DOMAIN_STMT: &str = "update platform_core.key_domains \
     set state = 'ACTIVE', provisioned_at = now(), \
         row_version = row_version + 1, updated_at = now() \
     where id = $1 and state = 'PROVISIONING' and row_version = $2";

const ROTATION_LOCK_STMT: &str =
    "select pg_advisory_xact_lock(hashtextextended('key_domain:' || $1 || ':' || $2, 0))";

const CURRENT_ACTIVE_KEY_STMT: &str = "select id, version from platform_core.data_keys \
     where key_domain_id = $1 and purpose = $2 and state = 'ACTIVE' \
     order by version desc limit 1";

const RETIRE_KEY_STMT: &str = "update platform_core.data_keys \
     set state = 'RETIRING', retiring_at = now(), \
         row_version = row_version + 1, updated_at = now() \
     where id = $1 and state = 'ACTIVE' and row_version = $2";

const SET_DOMAIN_STATE_STMT: &str = "update platform_core.key_domains \
     set state = $2, row_version = row_version + 1, updated_at = now() \
     where id = $1 and state = $3 and row_version = $4";

/// 密钥域与数据密钥的 PostgreSQL 存取层。装配时绑定 Rw 池工作单元。
pub struct PgKeyDomainStore {
    uow: Arc<PgUnitOfWork>,
}

impl PgKeyDomainStore {
    pub fn new(uow: Arc<PgUnitOfWork>) -> Self {
        Self { uow }
    }

    /// A-01：当前会话法人的密钥域列表（RLS 已按会话变量收窄）。
    pub async fn list_for_entity(
        &self,
        ctx: &SecurityContext,
    ) -> Result<Vec<KeyDomainRow>, AppError> {
        self.uow
            .transact(ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(LIST_DOMAINS_STMT, &[]).await?;
                    rows.iter().map(|row| domain_row_of(row)).collect()
                })
            })
            .await
    }

    /// A-02：单个域与其全部数据密钥。RLS 不可见时返回 None，
    /// 由端点映射为 NOT_FOUND_OR_DENIED。
    pub async fn get(
        &self,
        ctx: &SecurityContext,
        id: Uuid,
    ) -> Result<Option<(KeyDomainRow, Vec<DataKeyRow>)>, AppError> {
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(GET_DOMAIN_STMT, &[DbValue::Uuid(id)]).await?;
                    let Some(row) = rows.first() else {
                        return Ok(None);
                    };
                    let domain = domain_row_of(row)?;
                    let keys = pg
                        .query(LIST_KEYS_STMT, &[DbValue::Uuid(id)])
                        .await?
                        .iter()
                        .map(|row| key_row_of(row))
                        .collect::<Result<Vec<_>, AppError>>()?;
                    Ok(Some((domain, keys)))
                })
            })
            .await
    }

    /// A-03 幂等前置：当前法人同 kind 的既有域。
    pub async fn domain_of_kind(
        &self,
        ctx: &SecurityContext,
        kind: &str,
    ) -> Result<Option<KeyDomainRow>, AppError> {
        let kind = kind.to_string();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg
                        .query(DOMAIN_OF_KIND_STMT, &[DbValue::Text(kind)])
                        .await?;
                    rows.first().map(|row| domain_row_of(row)).transpose()
                })
            })
            .await
    }

    /// 启动自检第二段：当前会话法人是否已有密钥域（逐法人切换上下文调用）。
    pub async fn has_any_domain(&self, ctx: &SecurityContext) -> Result<bool, AppError> {
        self.uow
            .transact(ctx, |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let rows = pg.query(COUNT_DOMAINS_STMT, &[]).await?;
                    Ok(count_of(rows.first().and_then(|r| r.first()))? > 0)
                })
            })
            .await
    }

    /// A-03 落库：PROVISIONING 域与首批 DEK 一次事务写入。
    /// 域行插入触碰 ux（同法人同 kind）时按既登记冲突上抛。
    pub async fn insert_provisioning(
        &self,
        ctx: &SecurityContext,
        domain_id: Uuid,
        legal_entity_id: Uuid,
        domain_kind: &str,
        kek_ref: &str,
        keys: Vec<DataKeyInsert>,
    ) -> Result<(), AppError> {
        let kind = domain_kind.to_string();
        let kek = kek_ref.to_string();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    pg.execute(
                        INSERT_DOMAIN_STMT,
                        &[
                            DbValue::Uuid(domain_id),
                            DbValue::Uuid(legal_entity_id),
                            DbValue::Text(kind),
                            DbValue::Text(kek),
                        ],
                    )
                    .await?;
                    for k in keys {
                        pg.execute(
                            INSERT_KEY_STMT,
                            &[
                                DbValue::Uuid(k.id),
                                DbValue::Uuid(legal_entity_id),
                                DbValue::Uuid(k.key_domain_id),
                                DbValue::Text(k.purpose.to_string()),
                                DbValue::Int64(i64::from(k.security_level_scope)),
                                DbValue::Int64(i64::from(k.version)),
                                DbValue::Text(k.algorithm.to_string()),
                                DbValue::Text(k.wrapped_key_hex),
                                DbValue::Int64(i64::from(k.wrap_kek_version)),
                            ],
                        )
                        .await?;
                    }
                    Ok(())
                })
            })
            .await
    }

    /// PROVISIONING→ACTIVE：乐观锁谓词不中即并发冲突。
    pub async fn activate_domain(
        &self,
        ctx: &SecurityContext,
        id: Uuid,
        row_version: i64,
    ) -> Result<(), AppError> {
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let affected = pg
                        .execute(
                            ACTIVATE_DOMAIN_STMT,
                            &[DbValue::Uuid(id), DbValue::Int64(row_version)],
                        )
                        .await?;
                    if affected == 0 {
                        return Err(AppError::new(
                            PLATFORM_CONCURRENCY_STALE_VERSION,
                            "密钥域激活时行版本已变化",
                        ));
                    }
                    Ok(())
                })
            })
            .await
    }

    /// A-04：事务级建议锁内完成新版本插入与旧版本 RETIRING。
    /// 旧行状态或行版本不中即并发冲突，按 STALE_VERSION 上抛。
    pub async fn rotate(
        &self,
        ctx: &SecurityContext,
        domain_id: Uuid,
        legal_entity_id: Uuid,
        purpose: &'static str,
        new_key: DataKeyInsert,
        retiring_row_version: i64,
    ) -> Result<RotationRows, AppError> {
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    pg.query(
                        ROTATION_LOCK_STMT,
                        &[
                            DbValue::Text(domain_id.to_string()),
                            DbValue::Text(purpose.to_string()),
                        ],
                    )
                    .await?;
                    let rows = pg
                        .query(
                            CURRENT_ACTIVE_KEY_STMT,
                            &[DbValue::Uuid(domain_id), DbValue::Text(purpose.to_string())],
                        )
                        .await?;
                    let retiring = rows.first().ok_or_else(|| {
                        AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "轮换时缺少在役数据密钥")
                    })?;
                    let retiring_id = uuid_of(retiring.first())?;
                    pg.execute(
                        RETIRE_KEY_STMT,
                        &[
                            DbValue::Uuid(retiring_id),
                            DbValue::Int64(retiring_row_version),
                        ],
                    )
                    .await
                    .and_then(|affected| {
                        if affected == 0 {
                            Err(AppError::new(
                                PLATFORM_CONCURRENCY_STALE_VERSION,
                                "轮换时在役密钥行版本已变化",
                            ))
                        } else {
                            Ok(())
                        }
                    })?;
                    pg.execute(
                        INSERT_KEY_STMT,
                        &[
                            DbValue::Uuid(new_key.id),
                            DbValue::Uuid(legal_entity_id),
                            DbValue::Uuid(new_key.key_domain_id),
                            DbValue::Text(new_key.purpose.to_string()),
                            DbValue::Int64(i64::from(new_key.security_level_scope)),
                            DbValue::Int64(i64::from(new_key.version)),
                            DbValue::Text(new_key.algorithm.to_string()),
                            DbValue::Text(new_key.wrapped_key_hex),
                            DbValue::Int64(i64::from(new_key.wrap_kek_version)),
                        ],
                    )
                    .await?;
                    Ok(RotationRows {
                        new_data_key_id: new_key.id,
                        new_version: i32::from(new_key.version),
                        retiring_data_key_id: retiring_id,
                    })
                })
            })
            .await
    }

    /// A-05/A-06：域状态迁移（ACTIVE↔DESTROY_PLANNED），乐观锁谓词守护。
    /// `from` 为期望的当前状态，不中即并发冲突或状态已迁移。
    pub async fn set_domain_state(
        &self,
        ctx: &SecurityContext,
        id: Uuid,
        from: &str,
        to: &str,
        row_version: i64,
    ) -> Result<(), AppError> {
        let from = from.to_string();
        let to = to.to_string();
        self.uow
            .transact(ctx, move |tx| {
                Box::pin(async move {
                    let pg = downcast(tx)?;
                    let affected = pg
                        .execute(
                            SET_DOMAIN_STATE_STMT,
                            &[
                                DbValue::Uuid(id),
                                DbValue::Text(to),
                                DbValue::Text(from),
                                DbValue::Int64(row_version),
                            ],
                        )
                        .await?;
                    if affected == 0 {
                        return Err(AppError::new(
                            PLATFORM_CONCURRENCY_STALE_VERSION,
                            "密钥域状态迁移未命中期望形态",
                        ));
                    }
                    Ok(())
                })
            })
            .await
    }
}

fn downcast(tx: &mut dyn Tx) -> Result<&mut PgTx, AppError> {
    tx.as_any_mut().downcast_mut::<PgTx>().ok_or_else(|| {
        AppError::new(
            PLATFORM_SYSTEM_INTERNAL_ERROR,
            "密钥域存取必须在 PostgreSQL 事务内执行",
        )
    })
}

fn domain_row_of(row: &[DbValue]) -> Result<KeyDomainRow, AppError> {
    Ok(KeyDomainRow {
        id: uuid_of(row.first())?,
        legal_entity_id: uuid_of(row.get(1))?,
        domain_kind: text_of(row.get(2))?,
        state: text_of(row.get(3))?,
        kek_version: i32_of(row.get(4))?,
        provisioned_at: ts_of(row.get(5)).ok().flatten(),
        row_version: i64_of(row.get(6))?,
        active_key_count: i64_of(row.get(7))?,
    })
}

fn key_row_of(row: &[DbValue]) -> Result<DataKeyRow, AppError> {
    Ok(DataKeyRow {
        id: uuid_of(row.first())?,
        purpose: text_of(row.get(1))?,
        security_level_scope: i32_of(row.get(2))? as i16,
        version: i32_of(row.get(3))?,
        algorithm: text_of(row.get(4))?,
        wrapped_key_hex: text_of(row.get(5))?,
        wrap_kek_version: i32_of(row.get(6))?,
        state: text_of(row.get(7))?,
        activated_at: ts_of(row.get(8))?.ok_or_else(|| {
            AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, "数据密钥激活时刻列缺失")
        })?,
        row_version: i64_of(row.get(9))?,
    })
}

fn count_of(value: Option<&DbValue>) -> Result<i64, AppError> {
    i64_of(value).map_err(|_| shape("计数列"))
}

fn shape(what: &'static str) -> AppError {
    AppError::new(
        PLATFORM_SYSTEM_INTERNAL_ERROR,
        format!("密钥域读取结果形态不符：{what}"),
    )
}

fn uuid_of(value: Option<&DbValue>) -> Result<Uuid, AppError> {
    match value {
        Some(DbValue::Uuid(u)) => Ok(*u),
        _ => Err(shape("uuid 列")),
    }
}

fn text_of(value: Option<&DbValue>) -> Result<String, AppError> {
    match value {
        Some(DbValue::Text(s)) => Ok(s.clone()),
        _ => Err(shape("文本列")),
    }
}

fn i32_of(value: Option<&DbValue>) -> Result<i32, AppError> {
    match value {
        Some(DbValue::Int64(n)) => i32::try_from(*n).map_err(|_| shape("整型列越界")),
        _ => Err(shape("整型列")),
    }
}

fn i64_of(value: Option<&DbValue>) -> Result<i64, AppError> {
    match value {
        Some(DbValue::Int64(n)) => Ok(*n),
        _ => Err(shape("整型列")),
    }
}

fn ts_of(value: Option<&DbValue>) -> Result<Option<DateTime<Utc>>, AppError> {
    match value {
        Some(DbValue::Timestamp(t)) => Ok(Some(*t)),
        Some(DbValue::Null) | None => Ok(None),
        _ => Err(shape("时刻列")),
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

    fn store_with(conn: FakeConn) -> PgKeyDomainStore {
        let uow = Arc::new(PgUnitOfWork::with_fake_conns(
            vec![Box::new(conn)],
            "rw",
            RetryPolicy::standard(),
            Arc::new(NoopDbMetrics),
        ));
        PgKeyDomainStore::new(uow)
    }

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("key-domain-tests").expect("固定取值合法"),
            TraceId::new(&"0".repeat(32)).expect("固定取值合法"),
        )
    }

    fn domain_db_row() -> Vec<DbValue> {
        vec![
            DbValue::Uuid(Uuid::from_u128(1)),
            DbValue::Uuid(Uuid::from_u128(2)),
            DbValue::Text("LEGAL_ENTITY".into()),
            DbValue::Text("ACTIVE".into()),
            DbValue::Int64(1),
            DbValue::Null,
            DbValue::Int64(1),
            DbValue::Int64(4),
        ]
    }

    #[tokio::test]
    async fn list_decodes_domain_rows_with_active_key_count() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![domain_db_row()]);
        let store = store_with(conn);
        let rows = store.list_for_entity(&ctx()).await.expect("解码成功");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].state, "ACTIVE");
        assert_eq!(rows[0].active_key_count, 4);
    }

    #[tokio::test]
    async fn get_returns_none_when_rls_hides_the_row() {
        let conn = FakeConn::new(); // 空结果集
        let store = store_with(conn);
        assert!(store
            .get(&ctx(), Uuid::from_u128(9))
            .await
            .expect("查询成功")
            .is_none());
    }

    #[tokio::test]
    async fn activate_with_stale_row_version_is_a_conflict() {
        let mut conn = FakeConn::new();
        conn.execute_affected = 0;
        let store = store_with(conn);
        let err = store
            .activate_domain(&ctx(), Uuid::from_u128(1), 99)
            .await
            .expect_err("行版本不中必须拒绝");
        assert_eq!(err.code, PLATFORM_CONCURRENCY_STALE_VERSION);
    }

    #[tokio::test]
    async fn rotate_requires_an_active_key_to_retire() {
        let mut conn = FakeConn::new();
        conn.push_rows(vec![]); // 建议锁行
        conn.push_rows(vec![]); // 无在役密钥
        let store = store_with(conn);
        let insert = DataKeyInsert {
            id: Uuid::from_u128(7),
            key_domain_id: Uuid::from_u128(1),
            purpose: "FIELD",
            security_level_scope: 40,
            version: 2,
            algorithm: "AES_256_GCM",
            wrapped_key_hex: "00ff".into(),
            wrap_kek_version: 1,
        };
        let err = store
            .rotate(
                &ctx(),
                Uuid::from_u128(1),
                Uuid::from_u128(2),
                "FIELD",
                insert,
                1,
            )
            .await
            .expect_err("缺在役密钥必须拒绝");
        assert_eq!(err.code, PLATFORM_SYSTEM_INTERNAL_ERROR);
    }
}
