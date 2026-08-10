//! plugin-host 的配置根结构。
//!
//! 没有 http 段也没有 db 段：本进程零监听端口、零数据库连接，只有 IPC 服务端。
//! `wasmtime` 与 `wasmtime-wasi` 两个依赖本阶段一律不登记，也不留默认关闭的
//! feature 与编译缓存目录约定，由阶段 13b 在交付宿主时一次引入。

use ep_platform_runtime::config::{
    IpcCfg, LogCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[ipc]
socket_path = "/run/ep/ipc/plugin.sock"
"#;

/// 无监听进程的停机 drain 上限。没有 http 段就没有 `http.shutdown_drain_ms`，
/// 取值与该键的默认值一致，两处若要变必须一起变。
pub const SHUTDOWN_DRAIN_MS: u32 = 30_000;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct PluginHostConfig {
    pub ipc: IpcCfg,
    pub log: LogCfg,
    pub trace: TraceCfg,
    pub secrets: SecretsCfg,
    pub selfcheck: SelfcheckCfg,
    pub runtime: RuntimeCfg,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::ConfigLoader;

    fn load(extra: &str) -> Result<PluginHostConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS).map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn plugin_host_listens_only_on_its_socket() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.ipc.socket_path.to_string_lossy(), "/run/ep/ipc/plugin.sock");
        assert_eq!(cfg.ipc.max_frame_bytes, 1_048_576);
    }

    // 负样例断言的是「零监听端口、零数据库连接」这条边界本身。
    #[test]
    fn http_and_db_sections_are_rejected() {
        assert!(load("[http]\nbind_addr = \"127.0.0.1:9000\"\n").is_err(), "plugin-host 没有 http 段");
        assert!(load("[db]\nhost = \"127.0.0.1\"\n").is_err(), "plugin-host 没有 db 段");
    }
}
