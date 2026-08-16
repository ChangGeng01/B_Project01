//! 分段键：按法人与 Asia/Shanghai 自然日分段。
//!
//! 分段键算错的后果是链分叉——同一条链的事件被算进两个段，
//! 两个段各自看着都连贯，合起来却少了一截。因此这一层单列并单测，
//! 不混在追加算法里。

use chrono::{DateTime, Datelike, FixedOffset, Utc};

/// 分段时区。技术基线第 9.4 节逐字取 Asia/Shanghai。
///
/// 取固定偏移而不是时区数据库：Asia/Shanghai 自 1991 年起不再有夏令时，
/// 而本系统的审计事件不会早于部署日。用固定偏移可以让这一层不依赖时区数据，
/// 也就不会因为时区数据库版本不同而在两台机器上算出不同的分段键——
/// 那正是这类分段最难查的一种故障。
pub const SHANGHAI_OFFSET_SECONDS: i32 = 8 * 3600;

/// 段键。`event_day` 是 Asia/Shanghai 自然日。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SegmentKey {
    pub legal_entity_id: String,
    /// 形如 `2026-08-17`。取字符串而不是日期类型，是为了让它与库里
    /// `date` 列的文本形态逐字一致，避免两侧各自格式化时出现分叉。
    pub event_day: String,
}

impl SegmentKey {
    /// 由发生时刻算分段键。
    ///
    /// 注意时区换算的方向：`occurred_at` 是带时区的时刻，先换到东八区再取日期。
    /// 直接取 UTC 日期会让每天 16:00 到 24:00（UTC）之间的事件掉进前一天的段里，
    /// 那正是一天里业务最密集的时段。
    pub fn from_occurred_at(legal_entity_id: &str, occurred_at: DateTime<Utc>) -> Self {
        let offset =
            FixedOffset::east_opt(SHANGHAI_OFFSET_SECONDS).expect("东八区偏移是常量，构造不会失败");
        let local = occurred_at.with_timezone(&offset);
        Self {
            legal_entity_id: legal_entity_id.to_string(),
            event_day: format!(
                "{:04}-{:02}-{:02}",
                local.year(),
                local.month(),
                local.day()
            ),
        }
    }
}

/// 把一批事件按 `event_day` 升序分组。
///
/// 升序是死锁防线，不是排版偏好：计划第 3.4.2 节第 2 步逐字要求
/// 「若跨越两个自然日，按 `event_day` 升序依次处理，避免与另一事务反向加锁形成死锁」。
/// 两个事务若一个按升序、一个按降序取段锁，跨日的那一刻就会互相等。
pub fn group_by_day_ascending<T>(items: Vec<(SegmentKey, T)>) -> Vec<(SegmentKey, Vec<T>)> {
    let mut buckets: std::collections::BTreeMap<SegmentKey, Vec<T>> =
        std::collections::BTreeMap::new();
    for (k, v) in items {
        buckets.entry(k).or_default().push(v);
    }
    buckets.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn utc(y: i32, m: u32, d: u32, h: u32, mi: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, mi, 0)
            .single()
            .expect("夹具时刻应唯一")
    }

    /// 东八区当天 08:00 对应 UTC 前一天 00:00——分段键必须取东八区那一天。
    #[test]
    fn day_is_taken_in_shanghai_not_utc() {
        // UTC 2026-08-16 23:30 = 东八区 2026-08-17 07:30
        let k = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 16, 23, 30));
        assert_eq!(k.event_day, "2026-08-17", "应取东八区那一天");
    }

    /// 边界两侧各一条：东八区 00:00 的前一秒与后一秒分属两天。
    #[test]
    fn midnight_boundary_splits_exactly() {
        // UTC 15:59:00 → 东八区 23:59，仍是 16 日
        let before = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 16, 15, 59));
        // UTC 16:00:00 → 东八区次日 00:00
        let after = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 16, 16, 0));
        assert_eq!(before.event_day, "2026-08-16");
        assert_eq!(after.event_day, "2026-08-17");
    }

    #[test]
    fn day_is_zero_padded() {
        let k = SegmentKey::from_occurred_at("le-1", utc(2026, 1, 5, 4, 0));
        assert_eq!(
            k.event_day, "2026-01-05",
            "月与日都要补零，与 date 的文本形态一致"
        );
    }

    #[test]
    fn different_entities_never_share_a_segment() {
        let a = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 17, 1, 0));
        let b = SegmentKey::from_occurred_at("le-2", utc(2026, 8, 17, 1, 0));
        assert_ne!(a, b, "法人不同即不同段");
    }

    /// 分组必须按 event_day 升序——这是死锁防线。
    #[test]
    fn grouping_is_ascending_by_day() {
        let later = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 17, 1, 0));
        let earlier = SegmentKey::from_occurred_at("le-1", utc(2026, 8, 15, 1, 0));
        // 故意把晚的放前面。
        let grouped = group_by_day_ascending(vec![
            (later.clone(), "b"),
            (earlier.clone(), "a"),
            (later.clone(), "c"),
        ]);
        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped[0].0, earlier, "早的那一天必须排在前面");
        assert_eq!(grouped[1].0, later);
        assert_eq!(grouped[1].1, vec!["b", "c"], "同段内保持写入顺序");
    }
}
