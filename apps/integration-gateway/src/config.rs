//! integration-gateway 的配置根结构。
//!
//! 它是八个进程里唯一有 egress 段的：出网只从这里发生。白名单项的格式在
//! 配置层就判，坏白名单让进程以 78 拒绝启动，而不是等到第一次出网才炸。

use ep_platform_runtime::config::{
    DbCfg, EgressCfg, HttpCfg, LogCfg, MetricsCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:8082"

[metrics]
bind_addr = "127.0.0.1:8082"
"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct IntegrationConfig {
    pub http: HttpCfg,
    pub db: DbCfg,
    pub egress: EgressCfg,
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

    fn load(extra: &str) -> Result<IntegrationConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS).map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn integration_listens_on_8082_and_takes_the_integ_pool() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8082");
        assert_eq!(cfg.db.pool.integ_max, 5);
        assert!(cfg.egress.allowlist.is_empty(), "白名单默认为空，出网默认拒绝");
        assert_eq!(cfg.egress.breaker.failure_threshold, 5);
    }

    // 负样例断言的是白名单校验这条规则本身：坏项必须让配置层失败。
    #[test]
    fn a_malformed_allowlist_entry_fails_the_config_layer() {
        let err = load("[egress]\nallowlist = [\"http://plain.example.com\"]\n")
            .expect_err("明文 http 必须被拒");
        assert!(err.contains("https"), "{err}");
    }
}
