//! core-server 的配置根结构。
//!
//! 段的取舍即约束：core-server 持有 rw 与 ro 两个池，因此有 db 段；
//! 它不出网，因此没有 egress 段，配置里出现 egress 段即启动失败。

use ep_platform_runtime::config::{
    DbCfg, HttpCfg, IpcCfg, LogCfg, MetricsCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

/// 进程固定默认层。内置默认由各段的 Default 承载，这里只写按进程固定的键。
pub const DEFAULTS: &str = r#"
[http]
bind_addr = "127.0.0.1:8080"

[ipc]
socket_path = "/run/ep/ipc/core.sock"

[metrics]
bind_addr = "127.0.0.1:8080"
"#;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct CoreConfig {
    pub http: HttpCfg,
    pub ipc: IpcCfg,
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

    fn load(extra: &str) -> Result<CoreConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS).map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn process_defaults_pin_the_core_port_and_socket() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8080");
        assert_eq!(cfg.ipc.socket_path.to_string_lossy(), "/run/ep/ipc/core.sock");
        assert_eq!(cfg.http.concurrency_limit, 20);
        assert_eq!(cfg.db.pool.rw_max, 20);
        assert_eq!(cfg.db.pool.ro_max, 10);
    }

    // 负样例断言的是 core-server 不出网这条边界本身。
    #[test]
    fn an_egress_section_is_rejected() {
        let err = load("[egress]\nallowlist = []\n").expect_err("core-server 没有 egress 段");
        assert!(err.contains("egress"), "{err}");
    }
}
