//! integration-gateway 的装配。
//!
//! `ep-adapter-db-pg` 不提供 `SqlProbe` 实现——全仓两处 `impl SqlProbe` 分别在
//! core-server 与 job-worker **各自的 `wiring/probes.rs`**，是那两个 app 自建的
//! `FoundationProbeAdapter`，建在 `PgDataFoundationCheck` 之上。
//!
//! 本进程今天不依赖 `ep-adapter-db-pg`、无池、无该适配器，故不注入，
//! 四项 SQL 自检对本进程报 NOT_APPLICABLE；零数据库边界不是未覆盖。
//!
//! **原注写的「与 core-server 同理」已不成立**：core-server 早已自建适配器并注入
//! （`apps/core-server/src/wiring/db.rs` 的 `sql_probe` 逐字「装配成功即 Some，
//! 自检随即产生实质判定」）。本进程与它不同理，是**尚无任何数据库装配**。
//! 该处更正见裁定 F-34。

use std::sync::Arc;

use ep_adapter_ipc::{IpcMethod, MethodTable};
use ep_platform_runtime::http::SystemState;
use ep_platform_runtime::selfcheck::SqlProbe;
use serde_json::{json, Value};

pub const METHODS: [&str; 2] = ["health.get.v1", "metrics.snapshot.v1"];

pub fn sql_probe() -> Option<Arc<dyn SqlProbe>> {
    None
}

fn require_empty_payload(payload: &Value) -> Result<(), String> {
    if payload.is_null() || payload.as_object().is_some_and(serde_json::Map::is_empty) {
        Ok(())
    } else {
        Err("该方法不接受请求字段".to_string())
    }
}

struct HealthGet {
    state: Arc<SystemState>,
}

#[async_trait::async_trait]
impl IpcMethod for HealthGet {
    async fn call(&self, payload: Value) -> Result<Value, String> {
        require_empty_payload(&payload)?;
        Ok(json!({
            "schema_version": 1,
            "process": self.state.process().name(),
            "version": self.state.build().version,
            "state": self.state.state().as_str(),
            "ready": self.state.is_serving(),
        }))
    }
}

struct MetricsSnapshot {
    state: Arc<SystemState>,
}

#[async_trait::async_trait]
impl IpcMethod for MetricsSnapshot {
    async fn call(&self, payload: Value) -> Result<Value, String> {
        require_empty_payload(&payload)?;
        Ok(json!({
            "schema_version": 1,
            "content_type": "text/plain; version=0.0.4; charset=utf-8",
            "body": self.state.metrics().encode_text(),
        }))
    }
}

pub fn method_table(state: Arc<SystemState>) -> MethodTable {
    MethodTable::new()
        .with(
            METHODS[0],
            Arc::new(HealthGet {
                state: state.clone(),
            }),
        )
        .with(METHODS[1], Arc::new(MetricsSnapshot { state }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_adapter_ipc::IpcRequest;
    use ep_platform_obs::log::{JsonLogger, Level};
    use ep_platform_obs::MetricsRegistry;
    use ep_platform_runtime::lifecycle::Lifecycle;
    use ep_platform_runtime::selfcheck::SelfCheckReport;
    use ep_platform_runtime::selfcheck::{baseline_registry, Outcome};
    use ep_platform_runtime::{BuildInfo, ProcessKind};

    fn state() -> Arc<SystemState> {
        SystemState::new(
            ProcessKind::IntegrationGateway,
            BuildInfo::current(),
            Lifecycle::new(ProcessKind::IntegrationGateway),
            SelfCheckReport {
                process: "integration-gateway",
                version: "test".into(),
                items: Vec::new(),
                overall: Outcome::Passed,
            },
            Arc::new(MetricsRegistry::new()),
            Arc::new(JsonLogger::new("integration-gateway", "test", Level::Info)),
        )
    }

    // 零数据库进程不得因未注入 SQL probe 被误报为 PENDING。
    #[tokio::test]
    async fn zero_database_process_yields_not_applicable() {
        let p = ProcessKind::IntegrationGateway;
        let report = baseline_registry(p, String::new(), 1_000, sql_probe(), None, None)
            .run_all(p, "0.1.0")
            .await;
        let item = report
            .items
            .iter()
            .find(|i| i.name == "database-reachable")
            .expect("项必须在报告里");
        assert_eq!(item.outcome, Outcome::NotApplicable);
    }

    #[test]
    fn method_table_is_the_documented_closed_set() {
        assert_eq!(method_table(state()).names(), METHODS);
    }

    #[tokio::test]
    async fn health_and_metrics_have_versioned_strict_shapes() {
        let table = method_table(state());
        let health = table
            .dispatch(&serde_json::to_vec(&IpcRequest::new(METHODS[0], Value::Null)).unwrap())
            .await;
        assert!(health.ok);
        let health = health.payload.unwrap();
        assert_eq!(health["schema_version"], 1);
        assert_eq!(health["process"], "integration-gateway");
        assert_eq!(health["ready"], false);

        let metrics = table
            .dispatch(&serde_json::to_vec(&IpcRequest::new(METHODS[1], json!({}))).unwrap())
            .await;
        assert!(metrics.ok);
        let metrics = metrics.payload.unwrap();
        assert_eq!(metrics["schema_version"], 1);
        assert_eq!(
            metrics["content_type"],
            "text/plain; version=0.0.4; charset=utf-8"
        );
        assert!(metrics["body"].is_string());
    }

    #[tokio::test]
    async fn health_and_metrics_reject_ignored_request_fields() {
        for method in METHODS {
            let response = method_table(state())
                .dispatch(
                    &serde_json::to_vec(&IpcRequest::new(method, json!({"ignored": true})))
                        .unwrap(),
                )
                .await;
            assert!(!response.ok, "{method} 不得静默忽略未知请求字段");
        }
    }
}
