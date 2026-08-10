//! backup-writer 的配置根结构。
//!
//! **没有 db 段**：backup-writer 在本阶段就不持有运行期应用账号，
//! 按规格第 7.7 章只持 REPLICATION 属性，配置里出现 db 段即启动失败。
//! 这是把账号边界前移到类型层——写在文档里的边界，运行时无人执行。

use ep_platform_runtime::config::{IpcCfg, LogCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, SpoolCfg, TraceCfg};
use serde::Deserialize;

pub const DEFAULTS: &str = r#"
[ipc]
socket_path = "/run/ep/ipc/core.sock"

[spool]
dir = "/var/lib/ep/backup-writer/spool"
"#;

/// 无监听进程的停机 drain 上限。没有 http 段就没有 `http.shutdown_drain_ms`，
/// 取值与该键的默认值一致，两处若要变必须一起变。
pub const SHUTDOWN_DRAIN_MS: u32 = 30_000;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct BackupWriterConfig {
    pub ipc: IpcCfg,
    pub spool: SpoolCfg,
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

    fn load(extra: &str) -> Result<BackupWriterConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS).map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn writer_takes_core_socket_and_its_own_spool() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.ipc.socket_path.to_string_lossy(), "/run/ep/ipc/core.sock");
        assert_eq!(cfg.spool.dir.to_string_lossy(), "/var/lib/ep/backup-writer/spool");
        assert_eq!(cfg.spool.max_bytes, 268_435_456);
    }

    // 负样例断言的是账号边界这条规则本身：配置里出现 db 段必须启动失败。
    #[test]
    fn a_db_section_is_rejected() {
        let err = load("[db]\nhost = \"127.0.0.1\"\n").expect_err("backup-writer 没有 db 段");
        assert!(err.contains("db"), "{err}");
        // 只写 db 段里的一个键同样要被拒，不能靠「段名恰好叫 db」才拦得住。
        assert!(load("[db.pool]\nrw_max = 20\n").is_err());
    }

    #[test]
    fn an_http_section_is_rejected() {
        assert!(load("[http]\nbind_addr = \"127.0.0.1:9000\"\n").is_err(), "backup-writer 无监听");
    }
}
