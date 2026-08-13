//! 八个进程共用的命令行形态。
//!
//! 只有四个开关。进程不接受自由参数：运行期可变的东西一律走配置，
//! 命令行只是配置的最后一层。

use crate::config::ConfigSources;
use crate::process::ProcessKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cli {
    /// `--check`：执行全部自检项，输出 JSON 报告后退出，不监听端口。
    pub check: bool,
    pub help: bool,
    pub sources: ConfigSourcesSpec,
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ConfigSourcesSpec {
    pub file: Option<String>,
    pub dir: Option<String>,
    pub sets: Vec<String>,
}

pub const USAGE: &str =
    "用法: <进程> [--check] [--config <文件>] [--config-dir <目录>] [--set <键路径>=<取值>]...";

impl Cli {
    /// 解析失败返回可直接打印的中文说明。未知开关一律报错，
    /// 不忽略——被忽略的开关会让运维以为参数生效了。
    pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Cli, String> {
        let mut cli = Cli {
            check: false,
            help: false,
            sources: ConfigSourcesSpec::default(),
        };
        let mut it = args.into_iter();
        while let Some(arg) = it.next() {
            match arg.as_str() {
                "--check" => cli.check = true,
                "--help" | "-h" => cli.help = true,
                "--config" => {
                    cli.sources.file =
                        Some(it.next().ok_or_else(|| "--config 缺参数".to_string())?);
                }
                "--config-dir" => {
                    cli.sources.dir =
                        Some(it.next().ok_or_else(|| "--config-dir 缺参数".to_string())?);
                }
                "--set" => {
                    let kv = it.next().ok_or_else(|| "--set 缺参数".to_string())?;
                    if !kv.contains('=') {
                        return Err(format!("--set 的参数形态必须是 <键路径>=<取值>，实际 {kv}"));
                    }
                    cli.sources.sets.push(kv);
                }
                other => return Err(format!("未知参数 {other}\n{USAGE}")),
            }
        }
        Ok(cli)
    }

    pub fn from_env() -> Result<Cli, String> {
        Cli::parse(std::env::args().skip(1))
    }

    pub fn config_sources(&self, process: ProcessKind) -> ConfigSources {
        let mut s = ConfigSources::defaults_for(process);
        if let Some(f) = &self.sources.file {
            s.file = f.into();
        }
        if let Some(d) = &self.sources.dir {
            s.dir = d.into();
        }
        s.sets = self.sources.sets.clone();
        s
    }

    /// 已生效的配置来源描述，写进 `config-parsed` 的 detail。
    pub fn layers_description(&self, sources: &ConfigSources) -> String {
        format!(
            "内置默认 + 进程默认 + {} + {} + 环境变量 {}__* + 命令行 {} 项",
            sources.file.display(),
            sources.dir.display(),
            sources.env_prefix,
            sources.sets.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Cli, String> {
        Cli::parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn no_argument_means_serve_not_check() {
        let cli = parse(&[]).unwrap();
        assert!(!cli.check);
    }

    #[test]
    fn check_and_config_switches_are_accepted() {
        let cli = parse(&["--check", "--config", "/tmp/a.toml", "--set", "db.host=x"]).unwrap();
        assert!(cli.check);
        assert_eq!(cli.sources.file.as_deref(), Some("/tmp/a.toml"));
        assert_eq!(cli.sources.sets, ["db.host=x"]);
    }

    // 负样例断言的是参数解析这条规则本身：未知开关与缺参数都不得被忽略。
    #[test]
    fn unknown_switch_is_rejected() {
        assert!(parse(&["--force"]).is_err());
        assert!(parse(&["--config"]).is_err(), "缺参数必须报错");
        assert!(parse(&["--set", "db.host"]).is_err(), "--set 必须带等号");
    }

    #[test]
    fn config_paths_default_to_the_process_name() {
        let cli = parse(&[]).unwrap();
        let s = cli.config_sources(ProcessKind::JobWorker);
        assert_eq!(s.file.to_string_lossy(), "/etc/ep/job-worker.toml");
        assert_eq!(s.dir.to_string_lossy(), "/etc/ep/job-worker.conf.d");
    }

    #[test]
    fn explicit_paths_win_over_the_defaults() {
        let cli = parse(&["--config", "/x.toml", "--config-dir", "/x.d"]).unwrap();
        let s = cli.config_sources(ProcessKind::CoreServer);
        assert_eq!(s.file.to_string_lossy(), "/x.toml");
        assert_eq!(s.dir.to_string_lossy(), "/x.d");
    }
}
