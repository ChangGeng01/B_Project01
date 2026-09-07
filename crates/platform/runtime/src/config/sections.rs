//! 配置分段。默认值的唯一出处是阶段 1 计划第 8 节的配置项表。
//!
//! 每段都开 `deny_unknown_fields`：未知键必须让进程以 78 退出，而不是被
//! 静默忽略——被忽略的键会让运维以为改生效了。
//! 分段而不是一个大结构，是因为八个进程各取所需：archive-writer 与
//! backup-writer 的根结构里根本没有 `db` 段，配置里出现 db 段即启动失败。

use std::net::{Ipv4Addr, Ipv6Addr};
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
    /// 只有直接 TCP 对端命中这些 CIDR 时才受理 X-Forwarded-For；默认空即不信任代理头。
    pub trusted_proxy_cidrs: Vec<crate::http::TrustedProxyNet>,
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
            trusted_proxy_cidrs: Vec::new(),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct IpcCfg {
    pub socket_path: PathBuf,
    #[serde(deserialize_with = "deserialize_fixed_ipc_frame_bytes")]
    pub max_frame_bytes: u32,
}

fn deserialize_fixed_ipc_frame_bytes<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u32::deserialize(deserializer)?;
    if value != 1_048_576 {
        return Err(serde::de::Error::custom(
            "ipc.max_frame_bytes 是协议常量，必须精确为 1048576",
        ));
    }
    Ok(value)
}

impl IpcCfg {
    /// 每个进程只允许冻结的单一端点。分层配置仍会显示该键的来源，但不能借覆盖层
    /// 把受控命名管道改成任意路径或第二个 endpoint。
    pub fn require_endpoint(&self, expected: &str) -> Result<(), String> {
        if self.socket_path == std::path::Path::new(expected) {
            Ok(())
        } else {
            Err(format!(
                "ipc.socket_path 必须精确为 {expected}，不接受覆盖值 {}",
                self.socket_path.display()
            ))
        }
    }
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
            ops_max: 2,
            // 阶段 2 任务 #11 自 3000 提到 8000：四池满载下取连接的
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

/// 四个具名池的超时；integration-gateway 零数据库，不保留 integ 配置。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbTimeoutCfg {
    pub rw: PoolTimeoutCfg,
    pub ro: PoolTimeoutCfg,
    pub worker: PoolTimeoutCfg,
    pub ops: PoolTimeoutCfg,
}

impl Default for DbTimeoutCfg {
    fn default() -> Self {
        Self {
            rw: PoolTimeoutCfg::with_statement(10_000),
            ro: PoolTimeoutCfg::with_statement(60_000),
            worker: PoolTimeoutCfg::with_statement(300_000),
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

#[derive(Clone, Debug)]
pub struct DbRetryCfg {
    pub max_attempts: u8,
    pub backoff_ms: Vec<u32>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DbRetryCfgWire {
    #[serde(default = "default_db_retry_max_attempts")]
    max_attempts: u8,
    #[serde(default = "default_db_retry_backoff_ms")]
    backoff_ms: Vec<u32>,
}

const fn default_db_retry_max_attempts() -> u8 {
    3
}

fn default_db_retry_backoff_ms() -> Vec<u32> {
    vec![50, 150, 450]
}

impl<'de> Deserialize<'de> for DbRetryCfg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = DbRetryCfgWire::deserialize(deserializer)?;
        if wire.max_attempts != default_db_retry_max_attempts()
            || wire.backoff_ms.as_slice() != [50, 150, 450]
        {
            return Err(serde::de::Error::custom(
                "db.retry 必须精确为 max_attempts=3、backoff_ms=[50,150,450]",
            ));
        }
        Ok(Self {
            max_attempts: wire.max_attempts,
            backoff_ms: wire.backoff_ms,
        })
    }
}

impl Default for DbRetryCfg {
    fn default() -> Self {
        Self {
            max_attempts: default_db_retry_max_attempts(),
            backoff_ms: default_db_retry_backoff_ms(),
        }
    }
}

/// 当前 P340 连接预算种子。签名的全机 budget generation/digest 尚未实现前，
/// 三个持池进程必须逐项接受 `20/10/5/2 + 10 + 5 = 52`，不能从各自配置文件
/// 独立漂移；任何不等值或求和超限均以退出码 78 拒启。
#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct DbBudgetCfg {
    pub resident_max: u16,
    pub temporary_max: u16,
    pub peak_max: u16,
}

impl Default for DbBudgetCfg {
    fn default() -> Self {
        Self {
            resident_max: 37,
            temporary_max: 10,
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
    /// core-server Ro 池的独立只读账号；其他进程不解引用。
    pub ro_user: String,
    pub ro_password_ref: SecretRef,
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
            ro_user: "ep_analyst_ro".into(),
            ro_password_ref: SecretRef::parse("secret://db/analyst_ro#1")
                .expect("内置默认必须自洽"),
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
    /// 已废弃的阶段 2 文件入口。F-57 默认必须为空；非空只允许显式
    /// `legacy-file` development/test debug 构建处理，默认/发布构建拒绝。
    pub master_key_path: PathBuf,
}

impl Default for KmsBuiltinCfg {
    fn default() -> Self {
        Self {
            master_key_path: PathBuf::new(),
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
    pub trust_anchor_ref: SecretRef,
}

impl Default for AuthX509Cfg {
    fn default() -> Self {
        Self {
            trust_anchor_ref: SecretRef::parse("secret://pki/client_ca#1")
                .expect("内置默认必须自洽"),
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
    Kms,
    /// 历史 development/test reader。默认/发布构建不编译本枚举值，
    /// 因而 `provider=file` 会在配置反序列化阶段直接失败。
    #[cfg(feature = "legacy-file")]
    File,
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
            // 生产唯一允许的声明值。KmsSecretProvider 尚未交付时，消费方
            // 必须以 NOT_IMPLEMENTED 拒启，不能暗中回退历史明文 reader。
            provider: SecretsProvider::Kms,
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
        if raw.is_empty() || !raw.is_ascii() || raw.chars().any(char::is_whitespace) {
            return Err(EgressTargetError(
                "白名单项必须是非空、无空白的 ASCII HTTPS origin".into(),
            ));
        }
        let Some(rest) = raw.strip_prefix("https://") else {
            return Err(EgressTargetError(format!("{raw} 必须以 https:// 开头")));
        };
        if rest.contains(['/', '?', '#', '@', '\\']) {
            return Err(EgressTargetError(format!(
                "{raw} 只允许主机与可选端口，不允许路径、查询、片段、用户信息或反斜线"
            )));
        }
        let (host, port) = if rest.starts_with('[') {
            let Some(close) = rest.find(']') else {
                return Err(EgressTargetError(format!("{raw} 的 IPv6 缺少右方括号")));
            };
            let host = &rest[..=close];
            let suffix = &rest[close + 1..];
            let port = if suffix.is_empty() {
                None
            } else if let Some(value) = suffix.strip_prefix(':') {
                Some(value)
            } else {
                return Err(EgressTargetError(format!("{raw} 的 IPv6 主机后缀非法")));
            };
            (host, port)
        } else {
            match rest.rsplit_once(':') {
                Some((h, p)) if !h.contains(':') => (h, Some(p)),
                Some(_) => {
                    return Err(EgressTargetError(format!("{raw} 的 IPv6 必须使用方括号")));
                }
                None => (rest, None),
            }
        };

        validate_egress_host(host)
            .map_err(|detail| EgressTargetError(format!("{raw}：{detail}")))?;

        if port == Some("443") {
            return Err(EgressTargetError(format!(
                "{raw} 不得显式写 HTTPS 默认端口 443"
            )));
        }
        if let Some(p) = port {
            if p.is_empty() || p.starts_with('+') || (p.len() > 1 && p.starts_with('0')) {
                return Err(EgressTargetError(format!("{raw} 的端口不是规范十进制")));
            }
            if !matches!(p.parse::<u16>(), Ok(1..=u16::MAX)) {
                return Err(EgressTargetError(format!("{raw} 的端口不是 1..=65535")));
            }
        }
        Ok(EgressTarget(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn validate_egress_host(host: &str) -> Result<(), &'static str> {
    if host.is_empty() || host.contains('*') {
        return Err("主机为空或含通配符");
    }

    if let Some(inner) = host.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
        let address: Ipv6Addr = inner.parse().map_err(|_| "方括号内不是合法 IPv6")?;
        if inner != address.to_string() {
            return Err("IPv6 必须使用小写压缩规范形");
        }
        return Ok(());
    }

    if let Ok(address) = host.parse::<Ipv4Addr>() {
        return if host == address.to_string() {
            Ok(())
        } else {
            Err("IPv4 必须使用规范十进制形式")
        };
    }

    // WHATWG URL 与部分 HTTP 客户端会把 `127.1`、`0177.0.0.1`、
    // `2130706433`、`0x7f000001` 等旧式数字主机解释为 IPv4。若在这里把它们当
    // DNS 名保存，审阅者看到的白名单与客户端实际连接的地址就可能分叉。
    if host.split('.').all(is_legacy_ipv4_number) {
        return Err("疑似旧式数字 IPv4；必须使用四段规范十进制 IPv4");
    }

    if host.len() > 253 || host.ends_with('.') || host.bytes().any(|b| b.is_ascii_uppercase()) {
        return Err("DNS 主机名必须不超过 253 字节、小写且不带末尾点");
    }
    if host.split('.').any(|label| {
        label.is_empty()
            || label.len() > 63
            || !label
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || label.starts_with('-')
            || label.ends_with('-')
    }) {
        return Err("DNS 主机名标签必须为 1..=63 字节并只含小写字母、数字或内部连字符");
    }
    Ok(())
}

fn is_legacy_ipv4_number(label: &str) -> bool {
    !label.is_empty()
        && (label.bytes().all(|b| b.is_ascii_digit())
            || label
                .strip_prefix("0x")
                .is_some_and(|hex| !hex.is_empty() && hex.bytes().all(|b| b.is_ascii_hexdigit())))
}

impl<'de> Deserialize<'de> for EgressTarget {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        EgressTarget::parse(&raw).map_err(serde::de::Error::custom)
    }
}

fn deserialize_egress_allowlist<'de, D>(deserializer: D) -> Result<Vec<EgressTarget>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let values = Vec::<EgressTarget>::deserialize(deserializer)?;
    for pair in values.windows(2) {
        if pair[0].as_str() >= pair[1].as_str() {
            return Err(serde::de::Error::custom(format!(
                "egress.allowlist 必须按规范 origin 字节严格递增且无重复：{} / {}",
                pair[0].as_str(),
                pair[1].as_str()
            )));
        }
    }
    Ok(values)
}

#[derive(Clone, Deserialize, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct EgressCfg {
    #[serde(deserialize_with = "deserialize_egress_allowlist")]
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
    #[serde(deserialize_with = "deserialize_report_spool_max_bytes")]
    pub max_bytes: u64,
}

/// P340 容量包络为 archive-writer 与 backup-writer 各预留的单目录硬上限。
/// 默认仍保持 256 MiB；签名配置可以收紧或放大，但不得越过这条 2 GiB 物理预算。
pub const REPORT_SPOOL_HARD_MAX_BYTES: u64 = 2_147_483_648;
pub const REPORT_SPOOL_MIN_BYTES: u64 = 67_108_864;

fn deserialize_report_spool_max_bytes<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    if (REPORT_SPOOL_MIN_BYTES..=REPORT_SPOOL_HARD_MAX_BYTES).contains(&value) {
        Ok(value)
    } else {
        Err(serde::de::Error::custom(format!(
            "spool.max_bytes 必须在 {REPORT_SPOOL_MIN_BYTES}..={REPORT_SPOOL_HARD_MAX_BYTES} 字节内"
        )))
    }
}

impl Default for SpoolCfg {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("/var/lib/ep/spool"),
            max_bytes: 268_435_456,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipc_frame_limit_is_a_fixed_protocol_constant() {
        let exact: IpcCfg = toml::from_str("max_frame_bytes = 1048576\n").unwrap();
        assert_eq!(exact.max_frame_bytes, 1_048_576);
        for invalid in [0, 1, 1_048_575, 1_048_577, u32::MAX] {
            assert!(
                toml::from_str::<IpcCfg>(&format!("max_frame_bytes = {invalid}\n")).is_err(),
                "IPC 帧上限不得偏离协议常量：{invalid}"
            );
        }
    }

    #[test]
    fn pool_statement_timeouts_match_the_four_named_pools() {
        let t = DbTimeoutCfg::default();
        assert_eq!(t.rw.statement_ms, 10_000);
        assert_eq!(t.ro.statement_ms, 60_000);
        assert_eq!(t.worker.statement_ms, 300_000);
        assert_eq!(t.ops.statement_ms, 5_000);
        for p in [t.rw, t.ro, t.worker, t.ops] {
            assert_eq!(p.lock_ms, 3_000);
            assert_eq!(p.idle_in_tx_ms, 15_000);
        }
    }

    #[test]
    fn retry_policy_rejects_every_non_frozen_shape_at_config_parse_time() {
        let exact: DbRetryCfg = toml::from_str("max_attempts = 3\nbackoff_ms = [50, 150, 450]\n")
            .expect("冻结的重试策略必须可解析");
        assert_eq!(exact.max_attempts, 3);
        assert_eq!(exact.backoff_ms, [50, 150, 450]);

        for invalid in [
            "max_attempts = 0\nbackoff_ms = [50, 150, 450]\n",
            "max_attempts = 255\nbackoff_ms = [50, 150, 450]\n",
            "max_attempts = 3\nbackoff_ms = []\n",
            "max_attempts = 3\nbackoff_ms = [50, 150]\n",
            "max_attempts = 3\nbackoff_ms = [50, 150, 450, 900]\n",
            "max_attempts = 3\nbackoff_ms = [50, 150, 65536]\n",
            "max_attempts = 3\nbackoff_ms = [50, 150, 451]\n",
        ] {
            assert!(
                toml::from_str::<DbRetryCfg>(invalid).is_err(),
                "非冻结策略必须拒绝：{invalid}"
            );
        }
    }

    #[test]
    fn db_section_stage2_additions_match_the_ruling() {
        let d = DbCfg::default();
        assert_eq!(d.budget.resident_max, 37, "当前 P340 四池常驻测量种子");
        assert_eq!(d.budget.temporary_max, 10, "裁定 C-04 临时上限");
        assert_eq!(d.budget.peak_max, 52, "裁定 C-04 突发上限");
        assert_eq!(d.ro_user, "ep_analyst_ro");
        assert_eq!(d.ro_password_ref.as_str(), "secret://db/analyst_ro#1");
        assert_eq!(d.pool.acquire_timeout_ms, 8_000);
        assert_eq!(
            d.migration.expected_versions_path,
            PathBuf::from("/etc/ep/migration-versions.toml")
        );
    }

    #[test]
    fn obsolete_integration_pool_keys_are_rejected() {
        assert!(toml::from_str::<DbPoolCfg>("integ_max = 5").is_err());
        assert!(toml::from_str::<DbTimeoutCfg>("[integ]\nstatement_ms = 10000\n").is_err());
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
        assert_eq!(a.x509.trust_anchor_ref.as_str(), "secret://pki/client_ca#1");
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
        let ok: EgressCfg = toml::from_str(
            "allowlist = [\"https://192.0.2.10:8443\", \"https://[2001:db8::1]:8443\", \"https://esign.example.com\"]",
        )
        .unwrap();
        assert_eq!(ok.allowlist[0].as_str(), "https://192.0.2.10:8443");
        assert_eq!(ok.allowlist[1].as_str(), "https://[2001:db8::1]:8443");
        assert_eq!(ok.allowlist[2].as_str(), "https://esign.example.com");

        for bad_array in [
            "allowlist = [\"https://b.example.com\", \"https://a.example.com\"]",
            "allowlist = [\"https://a.example.com\", \"https://a.example.com\"]",
        ] {
            assert!(
                toml::from_str::<EgressCfg>(bad_array).is_err(),
                "白名单必须严格排序且去重：{bad_array}"
            );
        }
    }

    #[test]
    fn report_spool_size_is_bounded_by_the_p340_capacity_bucket() {
        assert_eq!(SpoolCfg::default().max_bytes, 268_435_456);
        let at_limit: SpoolCfg =
            toml::from_str(&format!("max_bytes = {REPORT_SPOOL_HARD_MAX_BYTES}"))
                .expect("2 GiB equality is the hard maximum");
        assert_eq!(at_limit.max_bytes, REPORT_SPOOL_HARD_MAX_BYTES);
        let at_minimum: SpoolCfg = toml::from_str(&format!("max_bytes = {REPORT_SPOOL_MIN_BYTES}"))
            .expect("64 MiB equality leaves the fixed critical reserve");
        assert_eq!(at_minimum.max_bytes, REPORT_SPOOL_MIN_BYTES);
        assert!(toml::from_str::<SpoolCfg>("max_bytes = 0").is_err());
        assert!(toml::from_str::<SpoolCfg>("max_bytes = 67108863").is_err());
        assert!(toml::from_str::<SpoolCfg>("max_bytes = 2147483649").is_err());
    }

    // 负样例断言的是白名单形态这条规则本身：明文、通配、带路径、坏端口都要拒。
    #[test]
    fn malformed_allowlist_entries_are_rejected() {
        for bad in [
            "http://esign.example.com",
            "https://*.example.com",
            "https://esign.example.com/callback",
            "https://esign.example.com:70000",
            "https://esign.example.com:0",
            "https://esign.example.com:0443",
            "https://esign.example.com:443",
            "https://user@esign.example.com",
            "https://esign.example.com?x=1",
            "https://esign.example.com#fragment",
            "https://esign.example.com\\callback",
            "https://A.example.com",
            "https://bad_name.example.com",
            "https://example..com",
            "https://example.com.",
            "https://-example.com",
            "https://2001:db8::1",
            "https://[2001:0db8::1]",
            "https://127.1",
            "https://0177.0.0.1",
            "https://2130706433",
            "https://0x7f000001",
            "https://esign.example.com ",
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

    #[test]
    fn secret_provider_defaults_to_kms_never_legacy_file() {
        assert_eq!(SecretsCfg::default().provider, SecretsProvider::Kms);
    }

    #[test]
    fn builtin_kms_default_has_no_legacy_master_key_file() {
        assert!(KmsCfg::default()
            .builtin
            .master_key_path
            .as_os_str()
            .is_empty());
    }

    #[cfg(not(feature = "legacy-file"))]
    #[test]
    fn default_build_rejects_file_provider_at_config_parse_time() {
        assert!(toml::from_str::<SecretsCfg>("provider = \"file\"").is_err());
    }
}
