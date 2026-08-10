//! 配置模型与分层加载。
//!
//! 结构体分段而不是一整块：八个进程各取所需的段，段的缺席本身就是一条约束。

pub mod loader;
pub mod sections;
pub mod secret;

pub use loader::{ConfigError, ConfigLoader};
pub use secret::{SecretRef, SecretString};
pub use sections::{
    BreakerCfg, DbCfg, EgressTarget, DbPoolCfg, DbRetryCfg, DbRoCfg, DbTimeoutCfg, EgressCfg, HttpCfg, IpcCfg,
    LogCfg, MetricsCfg, PoolTimeoutCfg, PortalCfg, RuntimeCfg, SecretsCfg, SecretsProvider,
    SelfcheckCfg, SpoolCfg, TraceCfg,
};

use std::path::PathBuf;

use crate::process::ProcessKind;

/// 一个进程的配置来源。命令行只认这四个开关，不认自由参数。
#[derive(Clone, Debug)]
pub struct ConfigSources {
    pub file: PathBuf,
    pub dir: PathBuf,
    pub env_prefix: String,
    pub sets: Vec<String>,
}

impl ConfigSources {
    pub fn defaults_for(process: ProcessKind) -> Self {
        Self {
            file: PathBuf::from(format!("/etc/ep/{}.toml", process.name())),
            dir: PathBuf::from(format!("/etc/ep/{}.conf.d", process.name())),
            env_prefix: "EP".to_string(),
            sets: Vec::new(),
        }
    }
}

/// 按五层顺序加载：内置默认、进程固定默认、主配置、片段目录、环境变量、命令行。
///
/// 内置默认由各段的 `Default` 实现承载，因此这里的第一层是进程固定默认。
pub fn load<T: serde::de::DeserializeOwned>(
    process_defaults: &str,
    sources: &ConfigSources,
) -> Result<T, ConfigError> {
    let mut loader = ConfigLoader::new();
    loader.layer_str("process-defaults", process_defaults)?;
    loader.layer_file(&sources.file)?;
    loader.layer_dir(&sources.dir)?;
    loader.layer_env(&sources.env_prefix, std::env::vars())?;
    loader.layer_cli(&sources.sets)?;
    loader.finish()
}
