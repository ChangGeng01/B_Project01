//! integration-gateway 的配置根结构。
//!
//! 它是八个进程里唯一有 egress 段的：出网只从这里发生。白名单项的格式在
//! 配置层就判，坏白名单让进程以 78 拒绝启动，而不是等到第一次出网才炸。

use ep_platform_runtime::config::{EgressCfg, IpcCfg, LogCfg, RuntimeCfg, SelfcheckCfg, TraceCfg};
use serde::Deserialize;

#[cfg(windows)]
pub const DEFAULTS: &str = r#"
[ipc]
socket_path = '\\.\pipe\ep-integ'
"#;

/// 非 Windows 值只服务于开发与测试；Windows Server 生产端固定使用 `ep-integ` 管道。
#[cfg(not(windows))]
pub const DEFAULTS: &str = r#"
[ipc]
socket_path = "/run/ep/ipc/integration.sock"
"#;

pub const SHUTDOWN_DRAIN_MS: u32 = 30_000;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct IntegrationConfig {
    pub ipc: IpcCfg,
    pub egress: EgressCfg,
    pub log: LogCfg,
    pub trace: TraceCfg,
    pub selfcheck: SelfcheckCfg,
    pub runtime: RuntimeCfg,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::ConfigLoader;

    fn load(extra: &str) -> Result<IntegrationConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS)
            .map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn integration_uses_only_its_fixed_ipc_endpoint_without_a_database_surface() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(
            cfg.ipc.socket_path.to_string_lossy(),
            ep_adapter_ipc::INTEGRATION_ENDPOINT
        );
        assert_eq!(cfg.ipc.max_frame_bytes, 1_048_576);
        assert!(
            cfg.egress.allowlist.is_empty(),
            "白名单默认为空，出网默认拒绝"
        );
        assert_eq!(cfg.egress.breaker.failure_threshold, 5);
        assert!(
            load("[db]\nport = 5432\n").is_err(),
            "出网网关必须拒绝整个 db 配置段"
        );
    }

    #[test]
    fn forbidden_tcp_metrics_and_secrets_sections_are_rejected() {
        assert!(
            load("[http]\nbind_addr = \"127.0.0.1:8082\"\n").is_err(),
            "integration-gateway 不得恢复 HTTP 监听"
        );
        assert!(
            load("[metrics]\nbind_addr = \"127.0.0.1:8082\"\n").is_err(),
            "指标只能经固定 IPC 快照方法读取"
        );
        assert!(
            load("[secrets]\nprovider = \"kms\"\n").is_err(),
            "零 KMS 网关必须拒绝整个 secrets 配置段"
        );
    }

    #[test]
    fn integration_ipc_endpoint_cannot_be_redirected() {
        let cfg = load("[ipc]\nsocket_path = \"/tmp/attacker.sock\"\n").unwrap();
        assert!(cfg
            .ipc
            .require_endpoint(ep_adapter_ipc::INTEGRATION_ENDPOINT)
            .is_err());
    }

    // 负样例断言的是白名单校验这条规则本身：坏项必须让配置层失败。
    #[test]
    fn a_malformed_allowlist_entry_fails_the_config_layer() {
        let err = load("[egress]\nallowlist = [\"http://plain.example.com\"]\n")
            .expect_err("明文 http 必须被拒");
        assert!(err.contains("https"), "{err}");
    }
}
