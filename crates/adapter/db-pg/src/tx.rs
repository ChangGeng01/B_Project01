//! `PgTx` 与 `PgUnitOfWork`。
//!
//! 声明位与实现位都在本 crate。技术基线第 1.4 节配套纪律第四条原文要求
//! 「声明位在 ep-adapter-db，实现落在 ep-adapter-db-pg」，实测该写法不成立，
//! 两条独立理由：
//!
//! 一、孤儿规则。`Tx` 定义在 ep-foundation、`PgTx` 若定义在 ep-adapter-db，
//!     则 `impl Tx for PgTx` 对本 crate 而言 trait 与类型双双是外部的，
//!     rustc 报 E0117，无法绕开。
//! 二、依赖方向。要在本 crate 引用 ep-adapter-db 声明的类型就必须依赖它，
//!     而第 1.3 节禁止项第五条禁止 adapter 之间互相依赖，`xtask archcheck`
//!     的 adapter-no-peer-adapter 规则会直接判失败。
//!
//! 两条冲突互相独立，只改其中一条都不够。本文件按「声明与实现同处本 crate」
//! 落地，ep-adapter-db 只保留与 PostgreSQL 无关的抽象。该偏离待裁定。

use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::port::tx::{
    BoxFuture, IsolationKind, SnapshotCtx, Tx, TxId, UnitOfWork,
};
use ep_foundation::security::SecurityContext;
use ep_foundation::Id;

pub struct PgTx {
    tx_id: TxId,
    isolation: IsolationKind,
    legal_entity_id: Id<LegalEntity>,
}

impl PgTx {
    pub fn new(tx_id: TxId, isolation: IsolationKind, legal_entity_id: Id<LegalEntity>) -> Self {
        Self { tx_id, isolation, legal_entity_id }
    }
}

impl Tx for PgTx {
    fn tx_id(&self) -> TxId {
        self.tx_id
    }

    fn isolation(&self) -> IsolationKind {
        self.isolation
    }

    fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.legal_entity_id
    }

    fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
        self
    }
}

pub struct PgSnapshot {
    snapshot_id: String,
    taken_at: chrono::DateTime<chrono::Utc>,
    legal_entity_id: Id<LegalEntity>,
}

impl SnapshotCtx for PgSnapshot {
    fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    fn taken_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.taken_at
    }

    fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.legal_entity_id
    }

    fn as_any(&self) -> &(dyn core::any::Any + Sync) {
        self
    }
}

/// 一个实例在装配时绑定一个池，不带池参数。
pub struct PgUnitOfWork {
    pool_name: &'static str,
}

impl PgUnitOfWork {
    pub fn new(pool_name: &'static str) -> Self {
        Self { pool_name }
    }

    pub fn pool_name(&self) -> &'static str {
        self.pool_name
    }
}

#[async_trait::async_trait]
impl UnitOfWork for PgUnitOfWork {
    async fn transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'t> FnOnce(&'t mut dyn Tx) -> BoxFuture<'t, Result<T, AppError>> + Send + 'static,
    {
        let mut tx = PgTx::new(
            TxId(uuid::Uuid::now_v7()),
            IsolationKind::ReadCommitted,
            ctx.legal_entity_id,
        );
        body(&mut tx).await
    }

    async fn snapshot_transact<T, F>(&self, ctx: &SecurityContext, body: F) -> Result<T, AppError>
    where
        T: Send + 'static,
        F: for<'s> FnOnce(&'s dyn SnapshotCtx) -> BoxFuture<'s, Result<T, AppError>> + Send + 'static,
    {
        let snapshot = PgSnapshot {
            snapshot_id: uuid::Uuid::now_v7().to_string(),
            taken_at: chrono::Utc::now(),
            legal_entity_id: ctx.legal_entity_id,
        };
        body(&snapshot).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
    use ep_foundation::security::context::{RequestId, TraceId};

    use super::*;

    /// 本文件的 future 从不 pend，忙轮询即可，无需引入运行时依赖。
    fn block_on<F: core::future::Future>(fut: F) -> F::Output {
        use core::task::{Context, Poll};
        let mut fut = Box::pin(fut);
        let mut cx = Context::from_waker(core::task::Waker::noop());
        loop {
            if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
                return v;
            }
        }
    }

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("0199aa11bb22cc33").expect("固定取值合法"),
            TraceId::new("0199aa11bb22cc330199aa11bb22cc33").expect("固定取值合法"),
        )
    }

    /// 冻结签名的可实现性与可调用性：`transact` 能在闭包里拿到 `&mut dyn Tx`。
    #[test]
    fn transact_hands_out_tx() {
        let uow = PgUnitOfWork::new("rw");
        let got = block_on(uow.transact(&ctx(), |tx| {
            let isolation = tx.isolation();
            Box::pin(async move { Ok(isolation) })
        }));
        assert_eq!(got.expect("事务体应成功"), IsolationKind::ReadCommitted);
    }

    /// 跨 crate 取具体句柄的唯一写法在本 crate 内可用。
    #[test]
    fn downcast_to_concrete_handle() {
        let uow = PgUnitOfWork::new("rw");
        let got = block_on(uow.transact(&ctx(), |tx| {
            let name = tx
                .as_any_mut()
                .downcast_mut::<PgTx>()
                .map(|pg| pg.tx_id())
                .expect("句柄必须是 PgTx");
            Box::pin(async move { Ok(name) })
        }));
        assert!(got.is_ok());
    }

    /// 快照分支同样可实现。
    #[test]
    fn snapshot_transact_hands_out_snapshot() {
        let uow = PgUnitOfWork::new("ro");
        let got = block_on(uow.snapshot_transact(&ctx(), |snap| {
            let id = snap.snapshot_id().to_string();
            Box::pin(async move { Ok(id) })
        }));
        assert!(!got.expect("快照体应成功").is_empty());
    }

    /// 空的 `Arc<[RoleCode]>` 与系统上下文的固定填充。
    #[test]
    fn system_context_uses_frozen_constants() {
        let c = ctx();
        assert_eq!(c.user_id.as_uuid(), SYSTEM_PRINCIPAL_ID);
        assert_eq!(c.device_id.as_str(), ep_foundation::principal::SYSTEM_DEVICE_ID);
        assert!(c.roles.is_empty());
        let _: &Arc<[_]> = &c.roles;
    }
}
