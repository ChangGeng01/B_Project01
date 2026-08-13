//! `IdempotencyStore` 的内存实现，只供集成测试使用。
//!
//! 边界写死：本实现不得进入 `apps/*` 的依赖图——它不触碰
//! `platform_msg.idempotency_keys`，不做事务内持久化，不承接生产判等；
//! 阶段 3a 的表实现才是唯一生产实现，CI 另断言全仓无第二套判等实现。
//!
//! 简化语义（与数据库实现的差异，测试引用时须知）：
//! 一、条目只在 `try_begin` 处登记；对未登记的键调 `finish` 不产生条目。
//! 二、同一键已登记但尚未 `finish` 时再次 `try_begin` 仍返 `FirstCall`，
//!     即本夹具不模拟并发在途去重；数据库实现由行锁承接该场景。
//! 三、键的定位取 `IdempotencyScope` 四字段全体，与事务句柄无关，
//!     `Tx` 参数在本实现中不被读取。

use std::collections::HashMap;
use std::sync::Mutex;

use ep_foundation::error::AppError;
use ep_foundation::port::db::{IdempotencyOutcome, IdempotencyScope, IdempotencyStore};
use ep_foundation::port::tx::Tx;

/// 内存幂等键存储。互斥锁只护一张哈希表，本夹具不做并发在途去重（见模块注释）。
#[derive(Default)]
pub struct InMemoryIdempotencyStore {
    entries: Mutex<HashMap<ScopeKey, Entry>>,
}

/// 作用域键：四字段全体参与定位，与端口的判等口径一致。
#[derive(Clone, PartialEq, Eq, Hash)]
struct ScopeKey {
    legal_entity: [u8; 16],
    user: [u8; 16],
    endpoint: String,
    key: [u8; 16],
}

#[derive(Clone)]
struct Entry {
    hash: [u8; 32],
    response: Option<(u16, Vec<u8>)>,
}

impl ScopeKey {
    fn of(scope: &IdempotencyScope) -> Self {
        Self {
            legal_entity: *scope.legal_entity_id.as_uuid().as_bytes(),
            user: *scope.user_id.as_uuid().as_bytes(),
            endpoint: scope.endpoint.clone(),
            key: *scope.key.as_bytes(),
        }
    }
}

impl InMemoryIdempotencyStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// 测试观察口：当前登记的条目数。
    pub fn entry_count(&self) -> usize {
        self.entries
            .lock()
            .expect("测试夹具的锁不承接投毒场景")
            .len()
    }
}

#[async_trait::async_trait]
impl IdempotencyStore for InMemoryIdempotencyStore {
    async fn try_begin(
        &self,
        _tx: &mut dyn Tx,
        scope: IdempotencyScope,
        request_hash: [u8; 32],
    ) -> Result<IdempotencyOutcome, AppError> {
        let key = ScopeKey::of(&scope);
        let mut entries = self.entries.lock().expect("测试夹具的锁不承接投毒场景");
        match entries.get(&key) {
            None => {
                entries.insert(
                    key,
                    Entry {
                        hash: request_hash,
                        response: None,
                    },
                );
                Ok(IdempotencyOutcome::FirstCall)
            }
            Some(entry) if entry.hash == request_hash => match &entry.response {
                Some((status, body)) => Ok(IdempotencyOutcome::Replay {
                    response_status: *status,
                    response_body: body.clone(),
                }),
                // 在途未完：内存夹具不模拟并发在途去重，语义见模块注释第二条。
                None => Ok(IdempotencyOutcome::FirstCall),
            },
            Some(_) => Ok(IdempotencyOutcome::PayloadMismatch),
        }
    }

    async fn finish(
        &self,
        _tx: &mut dyn Tx,
        scope: IdempotencyScope,
        response_status: u16,
        response_body: &[u8],
    ) -> Result<(), AppError> {
        let key = ScopeKey::of(&scope);
        let mut entries = self.entries.lock().expect("测试夹具的锁不承接投毒场景");
        if let Some(entry) = entries.get_mut(&key) {
            entry.response = Some((response_status, response_body.to_vec()));
        }
        // 对未登记的键调 finish 不产生条目：简化语义第一条。
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    use ep_foundation::id::marker::{LegalEntity, UserAccount};
    use ep_foundation::id::Id;
    use ep_foundation::port::tx::{IsolationKind, TxId};

    /// 最小执行器。ep-testkit 不依赖任何运行时（见工作区根 `Cargo.toml` 对
    /// tokio 的层位限制），而内存实现的未来体从不挂起，一次轮询即就绪。
    fn block_on<F: core::future::Future>(fut: F) -> F::Output {
        const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW, |_| {}, |_| {}, |_| {});
        const RAW: RawWaker = RawWaker::new(core::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(RAW) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("内存夹具的未来体不得挂起"),
        }
    }

    /// 占位事务句柄：内存实现不读取事务，只要求端口签名被满足。
    struct DummyTx {
        le: Id<LegalEntity>,
    }

    impl Tx for DummyTx {
        fn tx_id(&self) -> TxId {
            TxId(uuid::Uuid::nil())
        }
        fn isolation(&self) -> IsolationKind {
            IsolationKind::ReadCommitted
        }
        fn legal_entity_id(&self) -> Id<LegalEntity> {
            self.le
        }
        fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
            self
        }
    }

    fn scope(le_byte: u8, endpoint: &str, key: uuid::Uuid) -> IdempotencyScope {
        IdempotencyScope {
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(le_byte as u128)),
            user_id: Id::<UserAccount>::from_uuid(uuid::Uuid::from_u128(2)),
            endpoint: endpoint.to_string(),
            key,
        }
    }

    const HASH_A: [u8; 32] = [1; 32];
    const HASH_B: [u8; 32] = [2; 32];

    /// 三态判定的主线：首次 FirstCall、同哈希 Replay、异哈希 PayloadMismatch。
    #[test]
    fn three_states_first_replay_mismatch() {
        let store = InMemoryIdempotencyStore::new();
        let mut tx = DummyTx {
            le: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
        };
        let key = uuid::Uuid::from_u128(9);

        let first = block_on(store.try_begin(&mut tx, scope(1, "POST /x", key), HASH_A))
            .expect("内存实现不返错");
        assert_eq!(first, IdempotencyOutcome::FirstCall);

        block_on(store.finish(&mut tx, scope(1, "POST /x", key), 201, b"created"))
            .expect("finish 不返错");

        let replay = block_on(store.try_begin(&mut tx, scope(1, "POST /x", key), HASH_A))
            .expect("内存实现不返错");
        assert_eq!(
            replay,
            IdempotencyOutcome::Replay {
                response_status: 201,
                response_body: b"created".to_vec()
            },
            "重放必须回放定稿的响应"
        );

        let mismatch = block_on(store.try_begin(&mut tx, scope(1, "POST /x", key), HASH_B))
            .expect("内存实现不返错");
        assert_eq!(mismatch, IdempotencyOutcome::PayloadMismatch);
    }

    /// 作用域四字段全体参与定位：换一个端点或键值即视为新键。
    #[test]
    fn distinct_scopes_are_distinct_keys() {
        let store = InMemoryIdempotencyStore::new();
        let mut tx = DummyTx {
            le: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
        };
        let key = uuid::Uuid::from_u128(9);

        for s in [
            scope(1, "POST /a", key),
            scope(1, "POST /b", key),
            scope(1, "POST /a", uuid::Uuid::from_u128(10)),
            scope(2, "POST /a", key),
        ] {
            let outcome = block_on(store.try_begin(&mut tx, s, HASH_A)).expect("内存实现不返错");
            assert_eq!(outcome, IdempotencyOutcome::FirstCall);
        }
        assert_eq!(store.entry_count(), 4);
    }

    /// 未 finish 的在途键再次 try_begin 仍返 FirstCall（简化语义第二条）。
    #[test]
    fn in_flight_key_is_not_deduplicated() {
        let store = InMemoryIdempotencyStore::new();
        let mut tx = DummyTx {
            le: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
        };
        let s = scope(1, "POST /x", uuid::Uuid::from_u128(3));
        let first = block_on(store.try_begin(&mut tx, s.clone(), HASH_A)).expect("不返错");
        let again = block_on(store.try_begin(&mut tx, s, HASH_A)).expect("不返错");
        assert_eq!(first, IdempotencyOutcome::FirstCall);
        assert_eq!(again, IdempotencyOutcome::FirstCall);
    }

    /// 对未登记的键调 finish 不产生条目（简化语义第一条）。
    #[test]
    fn finish_without_begin_is_a_noop() {
        let store = InMemoryIdempotencyStore::new();
        let mut tx = DummyTx {
            le: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(1)),
        };
        let s = scope(1, "POST /x", uuid::Uuid::from_u128(4));
        block_on(store.finish(&mut tx, s.clone(), 200, b"ok")).expect("不返错");
        assert_eq!(store.entry_count(), 0);
        let outcome = block_on(store.try_begin(&mut tx, s, HASH_A)).expect("不返错");
        assert_eq!(outcome, IdempotencyOutcome::FirstCall);
    }
}
