//! 配置分段。默认值的唯一出处是阶段 1 计划第 8 节的配置项表。
//!
//! 每段都开 `deny_unknown_fields`：未知键必须让进程以 78 退出，而不是被
//! 静默忽略——被忽略的键会让运维以为改生效了。
//! 分段而不是一个大结构，是因为八个进程各取所需：archive-writer 与
//! backup-writer 的根结构里根本没有 `db` 段，配置里出现 db 段即启动失败。

use std::path::PathBuf;

use serde::Deserialize;

use super::secret::SecretRef;

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct HttpCfg {
    pub bind_addr: String,
    pub max_body_bytes: u64,
    pub request_timeout_ms: u32,
    pub shutdown_drain_ms: u32,
    pub concurrency_limit: u16,
    pub concurrency_wait_ms: u32,
}

impl Default for HttpCfg {
    fn default() -> Self {
        Self {
            // 按进程固定，由各进程的内置默认层覆盖；此处取 core 的取值。
            bind_addr: "127.0.0.1:8080".into(),
            max_body_bytes: 1_048_576,
            request_timeout_ms: 8_000,
            shutdown_drain_ms: 30_000,
            concurrency_limit: 20,
            concurrency_wait_ms: 10_000,
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct IpcCfg {
    pub socket_path: PathBuf,
    pub max_frame_bytes: u32,
}

impl Default for IpcCfg {
    fn default() -> Self {
        Self { socket_path: PathBuf::from("/run/ep/ipc/core.sock"), max_frame_bytes: 1_048_576 }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbPoolCfg {
    pub rw_max: u16,
    pub ro_max: u16,
    pub worker_max: u16,
    pub integ_max: u16,
    pub ops_max: u16,
    pub acquire_timeout_ms: u32,
    pub max_lifetime_s: u32,
    pub idle_timeout_s: u32,
}

impl Default for DbPoolCfg {
    fn default() -> Self {
        Self {
            rw_max: 20,
            ro_max: 10,
            worker_max: 5,
            integ_max: 5,
            ops_max: 2,
            acquire_timeout_ms: 3_000,
            max_lifetime_s: 1_800,
            idle_timeout_s: 300,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct PoolTimeoutCfg {
    pub statement_ms: u32,
    pub lock_ms: u32,
    pub idle_in_tx_ms: u32,
}

impl PoolTimeoutCfg {
    const fn with_statement(statement_ms: u32) -> Self {
        Self { statement_ms, lock_ms: 3_000, idle_in_tx_ms: 15_000 }
    }
}

impl Default for PoolTimeoutCfg {
    fn default() -> Self {
        Self::with_statement(10_000)
    }
}

/// 五个具名池的超时，取值见阶段 1 计划第 7.2 节。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbTimeoutCfg {
    pub rw: PoolTimeoutCfg,
    pub ro: PoolTimeoutCfg,
    pub worker: PoolTimeoutCfg,
    pub integ: PoolTimeoutCfg,
    pub ops: PoolTimeoutCfg,
}

impl Default for DbTimeoutCfg {
    fn default() -> Self {
        Self {
            rw: PoolTimeoutCfg::with_statement(10_000),
            ro: PoolTimeoutCfg::with_statement(60_000),
            worker: PoolTimeoutCfg::with_statement(300_000),
            integ: PoolTimeoutCfg::with_statement(10_000),
            ops: PoolTimeoutCfg::with_statement(5_000),
        }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbRoCfg {
    pub work_mem_kb: u32,
    pub temp_file_limit_kb: u32,
}

impl Default for DbRoCfg {
    fn default() -> Self {
        Self { work_mem_kb: 65_536, temp_file_limit_kb: 2_097_152 }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbRetryCfg {
    pub max_attempts: u8,
    pub backoff_ms: Vec<u32>,
}

impl Default for DbRetryCfg {
    fn default() -> Self {
        Self { max_attempts: 3, backoff_ms: vec![50, 150, 450] }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbCfg {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password_ref: SecretRef,
    pub pool: DbPoolCfg,
    pub timeout: DbTimeoutCfg,
    pub ro: DbRoCfg,
    pub retry: DbRetryCfg,
}

impl Default for DbCfg {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 5432,
            database: "ep".into(),
            user: "ep_app_rw".into(),
            password_ref: SecretRef::parse("secret://db/app_rw#1").expect("内置默认必须自洽"),
            pool: DbPoolCfg::default(),
            timeout: DbTimeoutCfg::default(),
            ro: DbRoCfg::default(),
            retry: DbRetryCfg::default(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct LogCfg {
    pub level: String,
    pub debug_auto_off_minutes: u16,
}

impl Default for LogCfg {
    fn default() -> Self {
        Self { level: "info".into(), debug_auto_off_minutes: 30 }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct MetricsCfg {
    pub enabled: bool,
    pub bind_addr: String,
}

impl Default for MetricsCfg {
    fn default() -> Self {
        Self { enabled: true, bind_addr: "127.0.0.1:8080".into() }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct TraceCfg {
    pub sample_ratio: f32,
    pub otlp_enabled: bool,
    pub otlp_endpoint: Option<String>,
}

impl Default for TraceCfg {
    fn default() -> Self {
        Self { sample_ratio: 0.1, otlp_enabled: false, otlp_endpoint: None }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SecretsProvider {
    File,
    Kms,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct SecretsCfg {
    pub dir: PathBuf,
    pub provider: SecretsProvider,
}

impl Default for SecretsCfg {
    fn default() -> Self {
        Self { dir: PathBuf::from("/var/lib/ep/secrets"), provider: SecretsProvider::File }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct SelfcheckCfg {
    pub clock_skew_max_ms: u32,
}

impl Default for SelfcheckCfg {
    fn default() -> Self {
        Self { clock_skew_max_ms: 1_000 }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct RuntimeCfg {
    /// 0 表示按 cgroup CPU 配额推导。
    pub worker_threads: u16,
    pub blocking_threads: u16,
}

impl Default for RuntimeCfg {
    fn default() -> Self {
        Self { worker_threads: 0, blocking_threads: 32 }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct BreakerCfg {
    pub failure_threshold: u16,
    pub open_ms: u32,
    pub half_open_probes: u8,
}

impl Default for BreakerCfg {
    fn default() -> Self {
        Self { failure_threshold: 5, open_ms: 30_000, half_open_probes: 1 }
    }
}

/// 出网白名单的一条。形态是 `<scheme>://<host>[:<port>]`，scheme 只允许 https。
///
/// 做成校验型 newtype 而不是裸字符串，是为了让白名单的格式错误在配置层就变成
/// 启动失败（退出码 78），而不是等到第一次出网时才在运行期炸开。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EgressTarget(String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EgressTargetError(String);

impl std::fmt::Display for EgressTargetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "出网白名单项非法：{}", self.0)
    }
}

impl std::error::Error for EgressTargetError {}

impl EgressTarget {
    pub fn parse(raw: &str) -> Result<EgressTarget, EgressTargetError> {
        let Some(rest) = raw.strip_prefix("https://") else {
            return Err(EgressTargetError(format!("{raw} 必须以 https:// 开头")));
        };
        if rest.contains('/') {
            return Err(EgressTargetError(format!("{raw} 只写主机与端口，不写路径")));
        }
        let (host, port) = match rest.split_once(':') {
            Some((h, p)) => (h, Some(p)),
            None => (rest, None),
        };
        if host.is_empty() || host.contains('*') {
            return Err(EgressTargetError(format!("{raw} 的主机为空或含通配符")));
        }
        if let Some(p) = port {
            if p.parse::<u16>().is_err() {
                return Err(EgressTargetError(format!("{raw} 的端口不是 1..=65535")));
            }
        }
        Ok(EgressTarget(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for EgressTarget {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        EgressTarget::parse(&raw).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct EgressCfg {
    pub allowlist: Vec<EgressTarget>,
    pub connect_timeout_ms: u32,
    pub request_timeout_ms: u32,
    pub ca_bundle_path: PathBuf,
    pub breaker: BreakerCfg,
}

impl Default for EgressCfg {
    fn default() -> Self {
        Self {
            allowlist: Vec::new(),
            connect_timeout_ms: 3_000,
            request_timeout_ms: 15_000,
            ca_bundle_path: PathBuf::from("/etc/ep/ca/esign-ca.pem"),
            breaker: BreakerCfg::default(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct SpoolCfg {
    pub dir: PathBuf,
    pub max_bytes: u64,
}

impl Default for SpoolCfg {
    fn default() -> Self {
        Self { dir: PathBuf::from("/var/lib/ep/spool"), max_bytes: 268_435_456 }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct PortalCfg {
    pub upstream_base_url: String,
    pub rate_limit_rps: u16,
}

impl Default for PortalCfg {
    fn default() -> Self {
        Self { upstream_base_url: "http://127.0.0.1:8080".into(), rate_limit_rps: 20 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_statement_timeouts_match_the_five_named_pools() {
        let t = DbTimeoutCfg::default();
        assert_eq!(t.rw.statement_ms, 10_000);
        assert_eq!(t.ro.statement_ms, 60_000);
        assert_eq!(t.worker.statement_ms, 300_000);
        assert_eq!(t.integ.statement_ms, 10_000);
        assert_eq!(t.ops.statement_ms, 5_000);
        for p in [t.rw, t.ro, t.worker, t.integ, t.ops] {
            assert_eq!(p.lock_ms, 3_000);
            assert_eq!(p.idle_in_tx_ms, 15_000);
        }
    }

    #[test]
    fn unknown_key_inside_a_section_is_rejected() {
        let err = toml::from_str::<HttpCfg>("bind_addr = \"127.0.0.1:1\"\nbind_addrs = \"x\"\n")
            .expect_err("未知键必须被拒");
        assert!(err.to_string().contains("bind_addrs"), "错误消息要能定位到键：{err}");
    }

    #[test]
    fn egress_allowlist_entries_are_validated_at_config_time() {
        let ok: EgressCfg = toml::from_str("allowlist = [\"https://esign.example.com:443\"]").unwrap();
        assert_eq!(ok.allowlist[0].as_str(), "https://esign.example.com:443");
    }

    // 负样例断言的是白名单形态这条规则本身：明文、通配、带路径、坏端口都要拒。
    #[test]
    fn malformed_allowlist_entries_are_rejected() {
        for bad in [
            "http://esign.example.com",
            "https://*.example.com",
            "https://esign.example.com/callback",
            "https://esign.example.com:70000",
            "esign.example.com",
        ] {
            assert!(
                toml::from_str::<EgressCfg>(&format!("allowlist = [\"{bad}\"]")).is_err(),
                "{bad} 应被拒"
            );
        }
    }

    #[test]
    fn password_ref_must_be_a_reference_not_a_literal() {
        assert!(toml::from_str::<DbCfg>("password_ref = \"hunter2\"").is_err());
        assert!(toml::from_str::<DbCfg>("password_ref = \"secret://db/app_rw#2\"").is_ok());
    }
}
