//! ops-agent 数据库指标到统一注册表的桥接。

use std::sync::Arc;

use ep_adapter_db_pg::DbMetrics;
use ep_platform_obs::MetricsRegistry;

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
    fn bridge_forwards_ops_pool_events() {
        let registry = Arc::new(MetricsRegistry::new());
        let bridge = ObsDbMetrics::new(registry.clone());
        bridge.pool_connections("ops", 2);
        bridge.statement_observed("ops", "select", 0.01);
        bridge.tx_retry("ops", "40001");
        let text = registry.encode_text();
        assert!(
            text.contains(r#"ep_db_pool_connections{pool="ops"} 2"#),
            "{text}"
        );
        assert!(text.contains("ep_db_statement_duration_seconds_bucket"));
        assert!(text.contains(r#"sqlstate="40001""#));
    }
}
