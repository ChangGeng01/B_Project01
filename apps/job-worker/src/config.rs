//! job-worker 的配置根结构。
//!
//! 有 db 段（worker 池上限 5），没有 ipc 段：本阶段的 job-worker 既不做
//! IPC 服务端也不做客户端，配置里出现 ipc 段即启动失败。

use ep_platform_runtime::config::{
    DbCfg, HttpCfg, LogCfg, MetricsCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:8081"

[metrics]
bind_addr = "127.0.0.1:8081"

[db]
user = "ep_app_rw"
"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct WorkerConfig {
    pub http: HttpCfg,
    pub db: DbCfg,
    pub log: LogCfg,
    pub metrics: MetricsCfg,
    pub trace: TraceCfg,
    pub secrets: SecretsCfg,
    pub selfcheck: SelfcheckCfg,
    pub runtime: RuntimeCfg,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::ConfigLoader;

    fn load(extra: &str) -> Result<WorkerConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS).map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn worker_listens_on_8081_and_takes_the_worker_pool() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8081");
        assert_eq!(cfg.db.pool.worker_max, 5);
        assert_eq!(cfg.db.timeout.worker.statement_ms, 300_000);
    }

    // 负样例断言的是段的取舍这条边界本身。
    #[test]
    fn an_ipc_section_is_rejected() {
        let err = load("[ipc]\nsocket_path = \"/run/ep/ipc/x.sock\"\n").expect_err("job-worker 没有 ipc 段");
        assert!(err.contains("ipc"), "{err}");
    }
}
