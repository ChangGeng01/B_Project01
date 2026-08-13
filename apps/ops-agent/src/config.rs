//! ops-agent 的配置根结构。
//!
//! 两个端口：`metrics.bind_addr` 是 9101 的指标口，`http.bind_addr` 是 9102 的
//! 健康聚合口。抓取目标不做配置键——阶段 1 计划第 8 节的配置项表里没有这一项，
//! 目标地址就是各进程按进程固定的监听地址，见 `targets.rs`。

use ep_platform_runtime::config::{
    DbCfg, HttpCfg, LogCfg, MetricsCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:9102"

[metrics]
bind_addr = "127.0.0.1:9101"

[db]
user = "ep_ops_ro"
"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct OpsConfig {
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

    fn load(extra: &str) -> Result<OpsConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS)
            .map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn ops_agent_takes_two_ports_and_the_read_only_account() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:9102");
        assert_eq!(cfg.metrics.bind_addr, "127.0.0.1:9101");
        assert_eq!(cfg.db.user, "ep_ops_ro", "运维台只读账号");
        assert_eq!(cfg.db.pool.ops_max, 2);
    }

    // 负样例断言的是「不新增配置键」这条纪律本身：抓取目标不进配置。
    #[test]
    fn a_scrape_targets_key_is_rejected() {
        let err = load("[ops]\ntargets = []\n").expect_err("配置项表里没有 ops 段");
        assert!(err.contains("ops"), "{err}");
    }
}
