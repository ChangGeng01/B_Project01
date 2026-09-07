//! portal-gateway 的配置根结构。
//!
//! 没有 db 段：门户网关不建数据库连接，取数一律经 core-server 的受控能力 API。
//! 这是把「不建库连接」这条边界前移到类型层，配置里出现 db 段即启动失败。

use ep_platform_runtime::config::{
    HttpCfg, LogCfg, MetricsCfg, RuntimeCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:8090"

[metrics]
bind_addr = "127.0.0.1:8090"

"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct PortalConfig {
    pub http: HttpCfg,
    pub log: LogCfg,
    pub metrics: MetricsCfg,
    pub trace: TraceCfg,
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
    fn portal_listens_on_8090_without_a_configurable_internal_upstream() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8090");
    }

    // 负样例断言的是「门户不建库连接」这条边界本身。
    #[test]
    fn a_db_section_is_rejected() {
        let err = load("[db]\nhost = \"127.0.0.1\"\n").expect_err("portal-gateway 没有 db 段");
        assert!(err.contains("db"), "{err}");
    }

    #[test]
    fn obsolete_upstream_and_unused_secrets_sections_are_rejected() {
        assert!(
            load("[portal]\nupstream_base_url = \"http://127.0.0.1:8080\"\n").is_err(),
            "门户内部调用已固定为未来 ep-core IPC，不得恢复回环 HTTP"
        );
        assert!(
            load("[portal]\nrate_limit_rps = 20\n").is_err(),
            "旧单值限流键不得静默兼容"
        );
        assert!(
            load("[secrets]\nprovider = \"kms\"\n").is_err(),
            "零 KMS 门户不得接受无消费者的 secrets 段"
        );
    }
}
