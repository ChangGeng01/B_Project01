//! portal-gateway 的配置根结构。
//!
//! 没有 db 段：门户网关不建数据库连接，取数一律经 core-server 的受控能力 API。
//! 这是把「不建库连接」这条边界前移到类型层，配置里出现 db 段即启动失败。

use ep_platform_runtime::config::{
    HttpCfg, LogCfg, MetricsCfg, PortalCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:8090"

[metrics]
bind_addr = "127.0.0.1:8090"

[portal]
upstream_base_url = "http://127.0.0.1:8080"
"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct PortalConfig {
    pub http: HttpCfg,
    pub portal: PortalCfg,
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

    fn load(extra: &str) -> Result<PortalConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS)
            .map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn portal_listens_on_8090_and_points_at_core() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8090");
        assert_eq!(cfg.portal.upstream_base_url, "http://127.0.0.1:8080");
        assert_eq!(cfg.portal.rate_limit_rps, 20);
    }

    // 负样例断言的是「门户不建库连接」这条边界本身。
    #[test]
    fn a_db_section_is_rejected() {
        let err = load("[db]\nhost = \"127.0.0.1\"\n").expect_err("portal-gateway 没有 db 段");
        assert!(err.contains("db"), "{err}");
    }
}
