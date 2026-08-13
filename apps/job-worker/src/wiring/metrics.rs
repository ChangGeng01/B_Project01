//! 数据库指标到统一登记表的桥接（02 计划 §7.2：桥接归装配侧）。
//!
//! `ep-adapter-db-pg` 只声明三类事件的出口 trait [`DbMetrics`]，三项指标名
//! 已在 ep-platform-obs 注册表登记（阶段 1 注册，本阶段填充）。本桥接只做
//! 转发：指标写入不得把数据路径带死，注册表侧的写入失败在这里就地吞掉，
//! 失败原因由注册表自身的不变量守护（名字未登记即编程错误，另由单测拦截）。

use std::sync::Arc;

use ep_adapter_db_pg::DbMetrics;
use ep_platform_obs::MetricsRegistry;

/// 把数据库侧三类指标事件转发到 obs 注册表的桥接器。
pub struct ObsDbMetrics {
    registry: Arc<MetricsRegistry>,
}

impl ObsDbMetrics {
    pub fn new(registry: Arc<MetricsRegistry>) -> Self {
        Self { registry }
    }
}

impl DbMetrics for ObsDbMetrics {
    fn pool_connections(&self, pool: &'static str, count: u32) {
        let _ = self.registry.set_gauge(
            "ep_db_pool_connections",
            &[("pool", pool)],
            f64::from(count),
        );
    }

    fn statement_observed(&self, pool: &'static str, kind: &'static str, seconds: f64) {
        let _ = self.registry.observe(
            "ep_db_statement_duration_seconds",
            &[("pool", pool), ("statement_kind", kind)],
            seconds,
        );
    }

    fn tx_retry(&self, pool: &'static str, sqlstate: &'static str) {
        let _ = self.registry.inc_counter(
            "ep_db_tx_retries_total",
            &[("pool", pool), ("sqlstate", sqlstate)],
            1.0,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_forwards_all_three_events_into_the_registry() {
        let registry = Arc::new(MetricsRegistry::new());
        let bridge = ObsDbMetrics::new(registry.clone());
        bridge.pool_connections("rw", 3);
        bridge.statement_observed("rw", "select", 0.02);
        bridge.tx_retry("worker", "40001");
        let text = registry.encode_text();
        assert!(
            text.contains(r#"ep_db_pool_connections{pool="rw"} 3"#),
            "{text}"
        );
        assert!(
            text.contains("ep_db_statement_duration_seconds_bucket"),
            "{text}"
        );
        assert!(text.contains(r#"sqlstate="40001""#), "{text}");
    }
}
