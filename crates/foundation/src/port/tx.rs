//! 事务与快照抽象。契约层的跨模块方法签名一律写 `&mut dyn Tx`。

use crate::error::AppError;
use crate::id::marker::LegalEntity;
use crate::id::Id;
use crate::security::context::SecurityContext;

pub type BoxFuture<'a, T> = core::pin::Pin<Box<dyn core::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TxId(pub uuid::Uuid);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsolationKind {
    ReadCommitted,
    RepeatableReadSnapshot,
}

pub trait Tx: Send {
    fn tx_id(&self) -> TxId;
    fn isolation(&self) -> IsolationKind;
    fn legal_entity_id(&self) -> Id<LegalEntity>;
    fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send);
}

pub trait SnapshotCtx: Sync {
    fn snapshot_id(&self) -> &str;
    fn taken_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn legal_entity_id(&self) -> Id<LegalEntity>;
    fn as_any(&self) -> &(dyn core::any::Any + Sync);
}

/// 含泛型方法，不满足对象安全：application crate 一律取泛型参数
/// `U: UnitOfWork`，不取 trait 对象。
///
/// 不带池参数，一个实例在装配时绑定一个池。
#[async_trait::async_trait]
pub trait UnitOfWork: Send + Sync + 'static {
    async fn transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static;

    async fn snapshot_transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'s> FnOnce(&'s dyn SnapshotCtx) -> BoxFuture<'s, Result<T, AppError>>
            + Send
            + 'static;
}
