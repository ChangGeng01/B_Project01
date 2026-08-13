//! core-server 的配置根结构。
//!
//! 段的取舍即约束：core-server 持有 rw 与 ro 两个池，因此有 db 段；
//! 它不出网，因此没有 egress 段，配置里出现 egress 段即启动失败。

use ep_platform_runtime::config::{
    AdmissionCfg, AuthCfg, AuthzCfg, DbCfg, HttpCfg, IpcCfg, KmsCfg, LogCfg, MetricsCfg,
    MigrationCfg, PlatformCfg, RuntimeCfg, SecretsCfg, SelfcheckCfg, TraceCfg,
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
    pub kms: KmsCfg,
    pub migration: MigrationCfg,
    /// 平台内核段（阶段 3a）：幂等键保留期，写端点的幂等定稿行过期清理取用。
    pub platform: PlatformCfg,
    /// 授权域段（阶段 4）：快照轮询、判定开关、范围编译与导出阈值五键。
    pub authz: AuthzCfg,
    /// 会话准入段（阶段 4）：并发上限、队列与等待、活跃窗口四键。
    pub admission: AdmissionCfg,
    /// 身份域段（阶段 4 任务 #21）：口令/锁定/会话/重认证/TOTP/
    /// WebAuthn/X509/应急八子段，EP__AUTH__* 前缀。
    pub auth: AuthCfg,
    pub selfcheck: SelfcheckCfg,
    pub runtime: RuntimeCfg,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::ConfigLoader;

    fn load(extra: &str) -> Result<CoreConfig, String> {
        let mut l = ConfigLoader::new();
        l.layer_str("defaults", DEFAULTS)
            .map_err(|e| e.to_string())?;
        l.layer_str("test", extra).map_err(|e| e.to_string())?;
        l.finish().map_err(|e| e.to_string())
    }

    #[test]
    fn process_defaults_pin_the_core_port_and_socket() {
        let cfg = load("").expect("默认层必须自洽");
        assert_eq!(cfg.http.bind_addr, "127.0.0.1:8080");
        assert_eq!(
            cfg.ipc.socket_path.to_string_lossy(),
            "/run/ep/ipc/core.sock"
        );
        assert_eq!(cfg.http.concurrency_limit, 20);
        assert_eq!(cfg.db.pool.rw_max, 20);
        assert_eq!(cfg.db.pool.ro_max, 10);
        assert_eq!(cfg.kms.backend, "builtin");
        assert_eq!(cfg.migration.window_ttl_max_min, 240);
        assert_eq!(cfg.platform.idempotency.retention_days, 7);
        assert_eq!(cfg.authz.snapshot.poll_interval_ms, 2_000);
        assert_eq!(cfg.authz.scope.max_department_depth, 8);
        assert_eq!(cfg.admission.max_concurrent_users, 20);
        assert_eq!(cfg.admission.queue_max_len, 40);
        assert_eq!(cfg.auth.password.min_length, 12);
        assert_eq!(cfg.auth.password.argon2.memory_kib, 65_536);
        assert_eq!(cfg.auth.session.max_per_user, 3);
        assert_eq!(cfg.auth.breakglass.max_session_seconds, 28_800);
    }

    // 负样例断言的是 core-server 不出网这条边界本身。
    #[test]
    fn an_egress_section_is_rejected() {
        let err = load("[egress]\nallowlist = []\n").expect_err("core-server 没有 egress 段");
        assert!(err.contains("egress"), "{err}");
    }
}
