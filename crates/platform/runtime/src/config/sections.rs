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
        Self {
            socket_path: PathBuf::from("/run/ep/ipc/core.sock"),
            max_frame_bytes: 1_048_576,
        }
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
            // 阶段 2 任务 #11 自 3000 提到 8000：五池满载下取连接的
            // 等待窗口对齐网关侧请求超时，避免 3s 误伤突发排队。
            acquire_timeout_ms: 8_000,
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
        Self {
            statement_ms,
            lock_ms: 3_000,
            idle_in_tx_ms: 15_000,
        }
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
        Self {
            work_mem_kb: 65_536,
            temp_file_limit_kb: 2_097_152,
        }
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
        Self {
            max_attempts: 3,
            backoff_ms: vec![50, 150, 450],
        }
    }
}

/// 连接预算（裁定 C-04）：resident 上限与突发上限。启动时五池规模
/// 求和校验，超限以退出码 78 拒启，校验本体在 ep-adapter-db-pg 的
/// `ConnectionBudget`。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbBudgetCfg {
    pub resident_max: u16,
    pub peak_max: u16,
}

impl Default for DbBudgetCfg {
    fn default() -> Self {
        Self {
            resident_max: 42,
            peak_max: 52,
        }
    }
}

/// 迁移预期版本台账的读取位置（阶段 2）。
#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbMigrationCfg {
    pub expected_versions_path: PathBuf,
}

impl Default for DbMigrationCfg {
    fn default() -> Self {
        Self {
            expected_versions_path: PathBuf::from("/etc/ep/migration-versions.toml"),
        }
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
    pub budget: DbBudgetCfg,
    pub migration: DbMigrationCfg,
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
            budget: DbBudgetCfg::default(),
            migration: DbMigrationCfg::default(),
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
        Self {
            level: "info".into(),
            debug_auto_off_minutes: 30,
        }
    }
}

/// KMS 后端配置（02 计划 §7）。只写引用与路径，不写密钥材料；
/// DEK 缓存两键与盲索引宽度键按裁定热生效，由载体在使用时读环境变量。
#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct KmsCfg {
    /// 取 `builtin` 或 `hsm`。
    pub backend: String,
    pub builtin: KmsBuiltinCfg,
    pub hsm: KmsHsmCfg,
}

impl Default for KmsCfg {
    fn default() -> Self {
        Self {
            backend: "builtin".into(),
            builtin: KmsBuiltinCfg::default(),
            hsm: KmsHsmCfg::default(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct KmsBuiltinCfg {
    /// 主密钥文件：权限必须 0400 且属主为本进程账户，否则拒启动。
    pub master_key_path: PathBuf,
}

impl Default for KmsBuiltinCfg {
    fn default() -> Self {
        Self {
            master_key_path: PathBuf::from("/var/lib/ep/kms/master.key"),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct KmsHsmCfg {
    pub pkcs11_module: String,
    pub slot: u32,
    pub pin_ref: SecretRef,
}

impl Default for KmsHsmCfg {
    fn default() -> Self {
        Self {
            pkcs11_module: String::new(),
            slot: 0,
            pin_ref: SecretRef::parse("secret://kms/hsm_pin#1").expect("内置默认必须自洽"),
        }
    }
}

/// 迁移窗口控制配置（02 计划 §7）。窗口 TTL 上限热生效。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct MigrationCfg {
    pub window_ttl_max_min: u32,
}

impl Default for MigrationCfg {
    fn default() -> Self {
        Self {
            window_ttl_max_min: 240,
        }
    }
}

/// 幂等键保留期（03 计划表 12）。过期行由保留期清理扫描按
/// `expires_at` 物理删除，core-server 与 job-worker 双进程生效；
/// 环境变量 `EP__PLATFORM__IDEMPOTENCY__RETENTION_DAYS`。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct IdempotencyCfg {
    pub retention_days: u32,
}

impl Default for IdempotencyCfg {
    fn default() -> Self {
        Self { retention_days: 7 }
    }
}

/// 平台内核配置段（03 计划 §3.7）。阶段 3a 只含幂等键保留期，
/// 后续段的配置项随其能力同批登记。
#[derive(Clone, Copy, Default, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct PlatformCfg {
    pub idempotency: IdempotencyCfg,
}

/// 授权快照重载轮询（04 计划 §4.2）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzSnapshotCfg {
    pub poll_interval_ms: u32,
}

impl Default for AuthzSnapshotCfg {
    fn default() -> Self {
        Self {
            poll_interval_ms: 2_000,
        }
    }
}

/// 授权判定辅助开关（04 计划 §4.1）。默认关闭即 bool 零值，故用派生。
#[derive(Clone, Copy, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzDecisionCfg {
    pub explain_enabled: bool,
}

/// 记录级范围编译（04 计划 §4.1 阶段三）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzScopeCfg {
    pub max_department_depth: u8,
    pub in_list_threshold: u16,
}

impl Default for AuthzScopeCfg {
    fn default() -> Self {
        Self {
            max_department_depth: 8,
            in_list_threshold: 200,
        }
    }
}

/// 敏感导出阈值（04 计划 §4.8）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzExportCfg {
    pub sensitive_row_threshold: u32,
}

impl Default for AuthzExportCfg {
    fn default() -> Self {
        Self {
            sensitive_row_threshold: 1_000,
        }
    }
}

/// 授权域配置段（阶段 4 任务 #22 登记五键）。四子段各有非零默认，
/// 段自身组合即四子段默认值，故用派生。
#[derive(Clone, Copy, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzCfg {
    pub snapshot: AuthzSnapshotCfg,
    pub decision: AuthzDecisionCfg,
    pub scope: AuthzScopeCfg,
    pub export: AuthzExportCfg,
}

/// 会话并发准入（阶段 4 任务 #22 登记四键）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AdmissionCfg {
    pub max_concurrent_users: u16,
    pub queue_max_len: u16,
    pub queue_wait_timeout_seconds: u8,
    pub active_window_seconds: u16,
}

impl Default for AdmissionCfg {
    fn default() -> Self {
        Self {
            max_concurrent_users: 20,
            queue_max_len: 40,
            queue_wait_timeout_seconds: 10,
            active_window_seconds: 60,
        }
    }
}

/// 口令策略与 Argon2id 参数（04 计划 §7，U-B-14 临时取值）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthPasswordCfg {
    pub min_length: u8,
    pub min_char_classes: u8,
    pub max_age_days: u16,
    pub history_size: u8,
    pub argon2: AuthArgon2Cfg,
}

impl Default for AuthPasswordCfg {
    fn default() -> Self {
        Self {
            min_length: 12,
            min_char_classes: 3,
            max_age_days: 90,
            history_size: 5,
            argon2: AuthArgon2Cfg::default(),
        }
    }
}

/// Argon2id 哈希参数：默认 65536 KiB、3 轮、单并行度。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthArgon2Cfg {
    pub memory_kib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl Default for AuthArgon2Cfg {
    fn default() -> Self {
        Self {
            memory_kib: 65_536,
            iterations: 3,
            parallelism: 1,
        }
    }
}

/// 登录锁定策略（U-B-14 临时取值：15 分钟窗口内 5 次失败锁 30 分钟）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthLockoutCfg {
    pub max_failures: u8,
    pub window_seconds: u32,
    pub duration_seconds: u32,
}

impl Default for AuthLockoutCfg {
    fn default() -> Self {
        Self {
            max_failures: 5,
            window_seconds: 900,
            duration_seconds: 1_800,
        }
    }
}

/// 会话策略：TTL、空闲超时、单用户上限与滑动续期写合并粒度。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthSessionCfg {
    pub ttl_seconds: u32,
    pub idle_timeout_seconds: u32,
    pub max_per_user: u8,
    pub sliding_write_granularity_seconds: u32,
}

impl Default for AuthSessionCfg {
    fn default() -> Self {
        Self {
            ttl_seconds: 28_800,
            idle_timeout_seconds: 1_800,
            max_per_user: 3,
            sliding_write_granularity_seconds: 60,
        }
    }
}

/// 重新认证与登录二段挑战窗口（基线第 5.6 节 5 分钟）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthReauthCfg {
    pub ttl_seconds: u32,
    pub max_failures: u8,
}

impl Default for AuthReauthCfg {
    fn default() -> Self {
        Self {
            ttl_seconds: 300,
            max_failures: 3,
        }
    }
}

/// TOTP 判码窗口：前后各 skew_steps 个 30 秒窗。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthTotpCfg {
    pub skew_steps: u8,
}

impl Default for AuthTotpCfg {
    fn default() -> Self {
        Self { skew_steps: 1 }
    }
}

/// WebAuthn：RP_ID 与 ORIGINS 必填，缺失即启动自检失败，无默认。
#[derive(Clone, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct AuthWebauthnCfg {
    pub rp_id: String,
    pub origins: Vec<String>,
}

/// X509_CERT 第一因子信任锚引用（形如 secret://pki/client_ca#1）。
#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthX509Cfg {
    pub trust_anchor_ref: String,
}

impl Default for AuthX509Cfg {
    fn default() -> Self {
        Self {
            trust_anchor_ref: "secret://pki/client_ca#1".to_string(),
        }
    }
}

/// 应急账号：单次启用上限与闲置轮换天数（规格第 12.1 章）。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct AuthBreakglassCfg {
    pub max_session_seconds: u32,
    pub idle_rotation_days: u16,
}

impl Default for AuthBreakglassCfg {
    fn default() -> Self {
        Self {
            max_session_seconds: 28_800,
            idle_rotation_days: 365,
        }
    }
}

/// 身份域配置段（阶段 4 任务 #21 登记，04 计划 §7 的 EP__AUTH__* 全键）。
/// 除 webauthn 两键必填无默认外，其余键各有启动默认。
#[derive(Clone, Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct AuthCfg {
    pub password: AuthPasswordCfg,
    pub lockout: AuthLockoutCfg,
    pub session: AuthSessionCfg,
    pub reauth: AuthReauthCfg,
    pub totp: AuthTotpCfg,
    pub webauthn: AuthWebauthnCfg,
    pub x509: AuthX509Cfg,
    pub breakglass: AuthBreakglassCfg,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct MetricsCfg {
    pub enabled: bool,
    pub bind_addr: String,
}

impl Default for MetricsCfg {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_addr: "127.0.0.1:8080".into(),
        }
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
        Self {
            sample_ratio: 0.1,
            otlp_enabled: false,
            otlp_endpoint: None,
        }
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
        Self {
            dir: PathBuf::from("/var/lib/ep/secrets"),
            provider: SecretsProvider::File,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct SelfcheckCfg {
    pub clock_skew_max_ms: u32,
}

impl Default for SelfcheckCfg {
    fn default() -> Self {
        Self {
            clock_skew_max_ms: 1_000,
        }
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
        Self {
            worker_threads: 0,
            blocking_threads: 32,
        }
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
        Self {
            failure_threshold: 5,
            open_ms: 30_000,
            half_open_probes: 1,
        }
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
        Self {
            dir: PathBuf::from("/var/lib/ep/spool"),
            max_bytes: 268_435_456,
        }
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
        Self {
            upstream_base_url: "http://127.0.0.1:8080".into(),
            rate_limit_rps: 20,
        }
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
    fn db_section_stage2_additions_match_the_ruling() {
        let d = DbCfg::default();
        assert_eq!(d.budget.resident_max, 42, "裁定 C-04 常驻上限");
        assert_eq!(d.budget.peak_max, 52, "裁定 C-04 突发上限");
        assert_eq!(d.pool.acquire_timeout_ms, 8_000);
        assert_eq!(
            d.migration.expected_versions_path,
            PathBuf::from("/etc/ep/migration-versions.toml")
        );
    }

    /// 阶段 3a：幂等键保留期默认 7 天，未知键照例拒收。
    #[test]
    fn platform_idempotency_retention_defaults_to_seven_days() {
        let p = PlatformCfg::default();
        assert_eq!(p.idempotency.retention_days, 7);
        let parsed: PlatformCfg = toml::from_str("[idempotency]\nretention_days = 14\n").unwrap();
        assert_eq!(parsed.idempotency.retention_days, 14);
        assert!(toml::from_str::<PlatformCfg>("[idempotency]\nretention_day = 1\n").is_err());
    }

    /// 阶段 4：授权与准入两段默认值逐键固化，未知键照例拒收。
    #[test]
    fn authz_and_admission_defaults_match_the_registration() {
        let a = AuthzCfg::default();
        assert_eq!(a.snapshot.poll_interval_ms, 2_000);
        assert!(!a.decision.explain_enabled);
        assert_eq!(a.scope.max_department_depth, 8);
        assert_eq!(a.scope.in_list_threshold, 200);
        assert_eq!(a.export.sensitive_row_threshold, 1_000);
        let m = AdmissionCfg::default();
        assert_eq!(m.max_concurrent_users, 20);
        assert_eq!(m.queue_max_len, 40);
        assert_eq!(m.queue_wait_timeout_seconds, 10);
        assert_eq!(m.active_window_seconds, 60);
        assert!(toml::from_str::<AuthzCfg>("[snapshot]\npoll_interval = 1\n").is_err());
    }

    /// 阶段 4：身份域段默认值逐键固化（04 计划 §7 的 EP__AUTH__* 表）。
    #[test]
    fn auth_defaults_match_the_registration() {
        let a = AuthCfg::default();
        assert_eq!(a.password.min_length, 12);
        assert_eq!(a.password.min_char_classes, 3);
        assert_eq!(a.password.max_age_days, 90);
        assert_eq!(a.password.history_size, 5);
        assert_eq!(a.password.argon2.memory_kib, 65_536);
        assert_eq!(a.password.argon2.iterations, 3);
        assert_eq!(a.password.argon2.parallelism, 1);
        assert_eq!(a.lockout.max_failures, 5);
        assert_eq!(a.lockout.window_seconds, 900);
        assert_eq!(a.lockout.duration_seconds, 1_800);
        assert_eq!(a.session.ttl_seconds, 28_800);
        assert_eq!(a.session.idle_timeout_seconds, 1_800);
        assert_eq!(a.session.max_per_user, 3);
        assert_eq!(a.session.sliding_write_granularity_seconds, 60);
        assert_eq!(a.reauth.ttl_seconds, 300);
        assert_eq!(a.reauth.max_failures, 3);
        assert_eq!(a.totp.skew_steps, 1);
        assert!(a.webauthn.rp_id.is_empty(), "RP_ID 无默认，必填");
        assert!(a.webauthn.origins.is_empty(), "ORIGINS 无默认，必填");
        assert_eq!(a.x509.trust_anchor_ref, "secret://pki/client_ca#1");
        assert_eq!(a.breakglass.max_session_seconds, 28_800);
        assert_eq!(a.breakglass.idle_rotation_days, 365);
        assert!(toml::from_str::<AuthCfg>("[password]\nmin_len = 1\n").is_err());
    }

    #[test]
    fn unknown_key_inside_a_section_is_rejected() {
        let err = toml::from_str::<HttpCfg>("bind_addr = \"127.0.0.1:1\"\nbind_addrs = \"x\"\n")
            .expect_err("未知键必须被拒");
        assert!(
            err.to_string().contains("bind_addrs"),
            "错误消息要能定位到键：{err}"
        );
    }

    #[test]
    fn egress_allowlist_entries_are_validated_at_config_time() {
        let ok: EgressCfg =
            toml::from_str("allowlist = [\"https://esign.example.com:443\"]").unwrap();
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
