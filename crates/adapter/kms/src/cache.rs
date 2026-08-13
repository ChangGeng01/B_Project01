//! DEK 进程内缓存（02 计划第 4.3 节「进程内缓存，未命中读 data_keys 经
//! KmsBackend::unwrap 解封」的承接点）。
//!
//! 上限取 `EP__KMS__DEK_CACHE__MAX_ENTRIES`（默认 512），存活取
//! `EP__KMS__DEK_CACHE__TTL_S`（默认 300），两键热生效——每次取放都重读配置。
//! 时钟以函数注入，TTL 行为可在单元测试中推进。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::cfg::DekCacheCfg;

/// 注入式时钟。生产取 `Instant::now`，测试取可控假时钟。
pub type Clock = Arc<dyn Fn() -> Instant + Send + Sync>;

/// 实时时钟。
pub fn wall_clock() -> Clock {
    Arc::new(Instant::now)
}

struct CacheEntry {
    material: [u8; 32],
    cached_at: Instant,
}

/// DEK 明文材料的进程内缓存。材料只在载体内流动：条目不提供公开访问器。
pub(crate) struct DekCache {
    entries: Mutex<HashMap<uuid::Uuid, CacheEntry>>,
    clock: Clock,
}

impl DekCache {
    pub(crate) fn new(clock: Clock) -> DekCache {
        DekCache {
            entries: Mutex::new(HashMap::new()),
            clock,
        }
    }

    /// 命中且未过期返材料副本；过期条目就地清除。
    pub(crate) fn get(&self, id: uuid::Uuid, cfg: DekCacheCfg) -> Option<[u8; 32]> {
        let now = (self.clock)();
        let mut entries = self.entries.lock().expect("缓存锁不得投毒");
        let expired = entries
            .get(&id)
            .is_some_and(|e| now.duration_since(e.cached_at) >= Duration::from_secs(cfg.ttl_s));
        if expired {
            entries.remove(&id);
            return Option::None;
        }
        entries.get(&id).map(|e| e.material)
    }

    /// 写入并按上限驱逐：超限先逐最旧条目，保证条数不破 `max_entries`。
    pub(crate) fn put(&self, id: uuid::Uuid, material: [u8; 32], cfg: DekCacheCfg) {
        let now = (self.clock)();
        let mut entries = self.entries.lock().expect("缓存锁不得投毒");
        if !entries.contains_key(&id) && entries.len() >= cfg.max_entries {
            let oldest = entries
                .iter()
                .min_by_key(|(_, e)| e.cached_at)
                .map(|(k, _)| *k);
            if let Some(victim) = oldest {
                entries.remove(&victim);
            }
        }
        entries.insert(
            id,
            CacheEntry {
                material,
                cached_at: now,
            },
        );
    }

    /// 销毁数据密钥时逐出条目，防止已销毁 DEK 的材料滞留缓存。
    pub(crate) fn evict(&self, id: uuid::Uuid) {
        self.entries.lock().expect("缓存锁不得投毒").remove(&id);
    }

    /// 当前条数，供测试与自检观察。
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.lock().expect("缓存锁不得投毒").len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;

    /// 可推进的假时钟。
    fn fake_clock() -> (Clock, Arc<StdMutex<Instant>>) {
        let t0 = Instant::now();
        let holder = Arc::new(StdMutex::new(t0));
        let h = holder.clone();
        let clock: Clock = Arc::new(move || *h.lock().unwrap());
        (clock, holder)
    }

    fn advance(holder: &StdMutex<Instant>, secs: u64) {
        *holder.lock().unwrap() += Duration::from_secs(secs);
    }

    #[test]
    fn hit_within_ttl_and_miss_after() {
        let (clock, holder) = fake_clock();
        let cache = DekCache::new(clock);
        let cfg = DekCacheCfg {
            max_entries: 8,
            ttl_s: 300,
        };
        let id = uuid::Uuid::from_u128(1);
        cache.put(id, [3; 32], cfg);
        assert_eq!(cache.get(id, cfg), Some([3; 32]));
        // 恰好到 TTL 边界即过期（>= 判据）。
        advance(&holder, 300);
        assert_eq!(cache.get(id, cfg), Option::None);
    }

    #[test]
    fn eviction_respects_max_entries() {
        let (clock, holder) = fake_clock();
        let cache = DekCache::new(clock);
        let cfg = DekCacheCfg {
            max_entries: 2,
            ttl_s: 300,
        };
        let a = uuid::Uuid::from_u128(1);
        let b = uuid::Uuid::from_u128(2);
        let c = uuid::Uuid::from_u128(3);
        cache.put(a, [1; 32], cfg);
        advance(&holder, 1);
        cache.put(b, [2; 32], cfg);
        advance(&holder, 1);
        // 第三次写入驱逐最旧的 a。
        cache.put(c, [3; 32], cfg);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(a, cfg), Option::None);
        assert_eq!(cache.get(b, cfg), Some([2; 32]));
        assert_eq!(cache.get(c, cfg), Some([3; 32]));
    }

    #[test]
    fn ttl_hot_reload_changes_behavior() {
        let (clock, holder) = fake_clock();
        let cache = DekCache::new(clock);
        let id = uuid::Uuid::from_u128(9);
        cache.put(
            id,
            [8; 32],
            DekCacheCfg {
                max_entries: 8,
                ttl_s: 300,
            },
        );
        advance(&holder, 60);
        // 热生效：把 TTL 读成 30 秒，60 秒前的条目即刻过期。
        let tightened = DekCacheCfg {
            max_entries: 8,
            ttl_s: 30,
        };
        assert_eq!(cache.get(id, tightened), Option::None);
    }
}
