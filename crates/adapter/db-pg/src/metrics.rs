//! 指标出口。db-pg 按 archcheck 纪律只依赖 ep-foundation，不得依赖
//! ep-platform-obs，因此本模块只声明出口 trait：三项指标的填充动作
//! 经 trait 出 crate，真实注册表（`ep_db_pool_connections`、
//! `ep_db_statement_duration_seconds`、`ep_db_tx_retries_total`）由
//! 装配侧桥接到 ep-platform-obs 的 MetricsRegistry。
//!
//! 标签取值域：pool 取五个具名池标签；statement_kind 取语句首词小写；
//! sqlstate 取 40001 与 40P01 两值。三者的基数都是有界集合，
//! 与注册表的标签基数纪律一致。

use std::sync::Mutex;

/// 数据库侧三类指标事件的出口。实现方必须无阻塞、不失败：
/// 指标写入不得把数据路径带死。
pub trait DbMetrics: Send + Sync {
    /// 具名池当前连接数（gauge）。
    fn pool_connections(&self, pool: &'static str, count: u32);

    /// 一条语句的执行时长（histogram）。
    fn statement_observed(&self, pool: &'static str, kind: &'static str, seconds: f64);

    /// 一次事务重试（counter）。
    fn tx_retry(&self, pool: &'static str, sqlstate: &'static str);
}

/// 未装配指标时的默认出口：全部丢弃。
#[derive(Default, Clone, Copy)]
pub struct NoopDbMetrics;

impl DbMetrics for NoopDbMetrics {
    fn pool_connections(&self, _pool: &'static str, _count: u32) {}
    fn statement_observed(&self, _pool: &'static str, _kind: &'static str, _seconds: f64) {}
    fn tx_retry(&self, _pool: &'static str, _sqlstate: &'static str) {}
}

/// 记录式出口。纯逻辑单测用它断言指标事件的产生时机与标签取值。
#[derive(Default)]
pub struct RecordingDbMetrics {
    pub gauges: Mutex<Vec<(&'static str, u32)>>,
    pub observations: Mutex<Vec<(&'static str, &'static str, f64)>>,
    pub retries: Mutex<Vec<(&'static str, &'static str)>>,
}

impl RecordingDbMetrics {
    pub fn new() -> Self {
        Self::default()
    }
}

fn unlock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    // 与注册表同策：指标锁中毒不吞事实，恢复内层值继续记。
    m.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl DbMetrics for RecordingDbMetrics {
    fn pool_connections(&self, pool: &'static str, count: u32) {
        unlock(&self.gauges).push((pool, count));
    }

    fn statement_observed(&self, pool: &'static str, kind: &'static str, seconds: f64) {
        unlock(&self.observations).push((pool, kind, seconds));
    }

    fn tx_retry(&self, pool: &'static str, sqlstate: &'static str) {
        unlock(&self.retries).push((pool, sqlstate));
    }
}

/// 语句类别：取 SQL 首词的小写形态。取值域有界，供
/// `ep_db_statement_duration_seconds` 的 statement_kind 标签使用。
pub fn statement_kind(sql: &str) -> &'static str {
    let first = sql.split_whitespace().next().unwrap_or("");
    match first.to_ascii_lowercase().as_str() {
        "select" => "select",
        "insert" => "insert",
        "update" => "update",
        "delete" => "delete",
        "begin" => "begin",
        "commit" => "commit",
        "rollback" => "rollback",
        "set" => "set",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_kind_is_the_lowercased_first_word() {
        assert_eq!(statement_kind("select 1"), "select");
        assert_eq!(statement_kind("  INSERT INTO t VALUES (1)"), "insert");
        assert_eq!(
            statement_kind("with cte as (select 1) select * from cte"),
            "other"
        );
        assert_eq!(statement_kind(""), "other");
    }

    #[test]
    fn recording_sink_keeps_every_event() {
        let rec = RecordingDbMetrics::new();
        rec.pool_connections("rw", 3);
        rec.statement_observed("rw", "select", 0.02);
        rec.tx_retry("rw", "40001");
        assert_eq!(unlock(&rec.gauges).len(), 1);
        assert_eq!(unlock(&rec.observations).len(), 1);
        assert_eq!(unlock(&rec.retries).as_slice(), &[("rw", "40001")]);
    }
}
