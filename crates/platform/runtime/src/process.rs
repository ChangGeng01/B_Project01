//! 八个进程的身份。
//!
//! 进程名、进程序号与是否持有常规数据库连接三件事必须在一处定死：
//! 关联编号的十万段划分靠序号，SQL 类自检项判 NotApplicable 靠后者，
//! 分散到各进程各写一份必然漂移。

/// 八个进程。crate 名与进程名、systemd 单元名、cgroup slice 名一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProcessKind {
    CoreServer,
    JobWorker,
    PortalGateway,
    IntegrationGateway,
    PluginHost,
    OpsAgent,
    ArchiveWriter,
    BackupWriter,
}

pub const ALL_PROCESSES: [ProcessKind; 8] = [
    ProcessKind::CoreServer,
    ProcessKind::JobWorker,
    ProcessKind::PortalGateway,
    ProcessKind::IntegrationGateway,
    ProcessKind::PluginHost,
    ProcessKind::OpsAgent,
    ProcessKind::ArchiveWriter,
    ProcessKind::BackupWriter,
];

impl ProcessKind {
    pub const fn name(self) -> &'static str {
        match self {
            ProcessKind::CoreServer => "core-server",
            ProcessKind::JobWorker => "job-worker",
            ProcessKind::PortalGateway => "portal-gateway",
            ProcessKind::IntegrationGateway => "integration-gateway",
            ProcessKind::PluginHost => "plugin-host",
            ProcessKind::OpsAgent => "ops-agent",
            ProcessKind::ArchiveWriter => "archive-writer",
            ProcessKind::BackupWriter => "backup-writer",
        }
    }

    /// 阶段 1 计划第 5.3 节固定的进程序号，关联编号按序号各占一个十万段。
    pub const fn ordinal(self) -> u32 {
        match self {
            ProcessKind::CoreServer => 1,
            ProcessKind::JobWorker => 2,
            ProcessKind::PortalGateway => 3,
            ProcessKind::IntegrationGateway => 4,
            ProcessKind::PluginHost => 5,
            ProcessKind::OpsAgent => 6,
            ProcessKind::ArchiveWriter => 7,
            ProcessKind::BackupWriter => 8,
        }
    }

    /// 是否持有常规数据库连接。为假的四个进程对全部 SQL 类自检项标 NotApplicable。
    pub const fn holds_sql_session(self) -> bool {
        match self {
            ProcessKind::CoreServer
            | ProcessKind::JobWorker
            | ProcessKind::IntegrationGateway
            | ProcessKind::OpsAgent => true,
            ProcessKind::PortalGateway
            | ProcessKind::PluginHost
            | ProcessKind::ArchiveWriter
            | ProcessKind::BackupWriter => false,
        }
    }

    pub fn parse(name: &str) -> Option<ProcessKind> {
        ALL_PROCESSES.into_iter().find(|p| p.name() == name)
    }
}

/// 构建标识。取值来自构建期环境变量，运行期只读。
#[derive(Clone, Copy, Debug)]
pub struct BuildInfo {
    pub version: &'static str,
    pub git_commit: &'static str,
    pub source_date_epoch: &'static str,
    pub migration_manifest_sha256: &'static str,
}

impl BuildInfo {
    /// 未注入的构建信息取 `unknown` 而不是空串：空串在指标标签上与「没这个标签」
    /// 难以分辨，`unknown` 一眼可见是未注入。
    pub const fn current() -> BuildInfo {
        BuildInfo {
            version: env!("CARGO_PKG_VERSION"),
            git_commit: match option_env!("EP_GIT_COMMIT") {
                Some(v) => v,
                None => "unknown",
            },
            source_date_epoch: match option_env!("SOURCE_DATE_EPOCH") {
                Some(v) => v,
                None => "unknown",
            },
            migration_manifest_sha256: env!("EP_MIGRATION_MANIFEST_SHA256"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinals_are_one_to_eight_without_gap() {
        let mut seen: Vec<u32> = ALL_PROCESSES.iter().map(|p| p.ordinal()).collect();
        seen.sort_unstable();
        assert_eq!(seen, (1..=8).collect::<Vec<_>>());
    }

    #[test]
    fn names_round_trip() {
        for p in ALL_PROCESSES {
            assert_eq!(ProcessKind::parse(p.name()), Some(p));
        }
        assert_eq!(ProcessKind::parse("core"), None, "进程名不做前缀匹配");
    }

    #[test]
    fn exactly_four_processes_hold_sql_sessions() {
        let holders: Vec<&str> = ALL_PROCESSES
            .into_iter()
            .filter(|p| p.holds_sql_session())
            .map(|p| p.name())
            .collect();
        assert_eq!(
            holders,
            [
                "core-server",
                "job-worker",
                "integration-gateway",
                "ops-agent"
            ]
        );
    }
}
