//! 命令行解析。纯函数，不做任何 IO，便于逐条参数错误各配一个用例。
//!
//! 选项名取技术基线第 625 行的口径：生成器接受 `--seed` 与 `--scale`。
//! `--seed` 没有默认值且必须显式给出——若默认取时钟或取随机数，
//! 「同一 seed 两次字节一致」这条判据就会被一个看不见的入参悄悄绕过。

use std::path::PathBuf;

use crate::exit::Exit;
use crate::scale::{Scale, ScaleError, REGISTERED};

const USAGE_HEAD: &str = "\
ep-datagen — 基准数据集生成器。独立二进制，不属八进程。

用法：
  ep-datagen generate --scale=<档位> --seed=<u64> [--out=<路径>]
  ep-datagen help

选项：
  --scale=<档位>  样本档名。已交付 t0-min、t0 与 small 三档。
  --seed=<u64>    生成种子，必须显式给出，无默认值。
  --out=<路径>    写出目标；缺省写 stdout。两种写出的字节完全相同。
";

/// 用法文本。退出码一节由 `Exit::ALL` 生成而不是另抄一份，
/// 否则新增一个退出码时用法文本会静默过期。
pub fn usage() -> String {
    let mut out = String::from(USAGE_HEAD);
    out.push_str(&format!("\n已登记档位：{}\n", REGISTERED.join("、")));
    out.push_str("\n退出码：\n");
    for e in Exit::ALL {
        out.push_str(&format!("  {:<4}{}\n", e.code(), e.label()));
    }
    out
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Command {
    Help,
    Generate {
        scale: Scale,
        seed: u64,
        out: Option<PathBuf>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CliError {
    MissingSubcommand,
    UnknownSubcommand(String),
    UnknownArgument(String),
    MissingOption(&'static str),
    BadSeed(String),
    Scale(ScaleError),
}

impl CliError {
    /// 「未交付」与「参数错误」必须落到不同退出码：前者改命令行没用。
    pub const fn exit(&self) -> Exit {
        match self {
            CliError::Scale(ScaleError::NotDelivered { .. }) => Exit::NotDelivered,
            _ => Exit::Usage,
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::MissingSubcommand => write!(f, "缺子命令；可用：generate、help"),
            CliError::UnknownSubcommand(s) => write!(f, "未知子命令 {s}；可用：generate、help"),
            CliError::UnknownArgument(s) => write!(f, "未知参数 {s}"),
            CliError::MissingOption("--scale") => {
                write!(
                    f,
                    "缺必填选项 --scale；已登记档位：{}",
                    REGISTERED.join("、")
                )
            }
            CliError::MissingOption(name) => write!(f, "缺必填选项 {name}"),
            CliError::BadSeed(s) => write!(f, "--seed 取值 {s} 不是合法的 u64 十进制整数"),
            CliError::Scale(e) => write!(f, "{e}"),
        }
    }
}

/// 解析已剥去程序名的参数表。
pub fn parse(args: &[String]) -> Result<Command, CliError> {
    let Some(sub) = args.first().map(String::as_str) else {
        return Err(CliError::MissingSubcommand);
    };
    match sub {
        "help" | "--help" | "-h" => Ok(Command::Help),
        "generate" => parse_generate(&args[1..]),
        other => Err(CliError::UnknownSubcommand(other.to_string())),
    }
}

fn parse_generate(args: &[String]) -> Result<Command, CliError> {
    let mut scale_name: Option<String> = None;
    let mut seed_text: Option<String> = None;
    let mut out: Option<PathBuf> = None;

    for arg in args {
        if let Some(v) = arg.strip_prefix("--scale=") {
            scale_name = Some(v.to_string());
        } else if let Some(v) = arg.strip_prefix("--seed=") {
            seed_text = Some(v.to_string());
        } else if let Some(v) = arg.strip_prefix("--out=") {
            out = Some(PathBuf::from(v));
        } else {
            return Err(CliError::UnknownArgument(arg.clone()));
        }
    }

    let scale_name = scale_name.ok_or(CliError::MissingOption("--scale"))?;
    let seed_text = seed_text.ok_or(CliError::MissingOption("--seed"))?;
    let scale = Scale::parse(&scale_name).map_err(CliError::Scale)?;
    let seed = seed_text
        .parse::<u64>()
        .map_err(|_| CliError::BadSeed(seed_text.clone()))?;
    Ok(Command::Generate { scale, seed, out })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn generate_parses_all_options() {
        let cmd = parse(&args(&[
            "generate",
            "--scale=t0-min",
            "--seed=42",
            "--out=/tmp/x",
        ]));
        assert_eq!(
            cmd,
            Ok(Command::Generate {
                scale: Scale::T0Min,
                seed: 42,
                out: Some(PathBuf::from("/tmp/x")),
            })
        );
    }

    #[test]
    fn out_is_optional() {
        assert!(matches!(
            parse(&args(&["generate", "--scale=t0-min", "--seed=0"])),
            Ok(Command::Generate { out: None, .. })
        ));
    }

    /// 负样例：缺 seed 必须报错，不得回落到某个默认值。
    #[test]
    fn missing_seed_is_rejected() {
        let e = parse(&args(&["generate", "--scale=t0-min"])).unwrap_err();
        assert_eq!(e, CliError::MissingOption("--seed"));
        assert_eq!(e.exit(), Exit::Usage);
    }

    /// 负样例：缺 scale 必须报错，不得回落到默认档。
    #[test]
    fn missing_scale_is_rejected() {
        assert_eq!(
            parse(&args(&["generate", "--seed=1"])),
            Err(CliError::MissingOption("--scale"))
        );
    }

    /// 负样例：seed 不是 u64 必须报参数错误，不得截断或取 0。
    #[test]
    fn bad_seed_is_rejected() {
        for bad in ["", "-1", "1.5", "abc", "18446744073709551616"] {
            let got = parse(&args(&[
                "generate",
                "--scale=t0-min",
                &format!("--seed={bad}"),
            ]));
            assert_eq!(got, Err(CliError::BadSeed(bad.to_string())), "seed={bad}");
        }
    }

    /// 负样例：未交付的档位以 70 收尾，与参数错误的 2 区分。
    #[test]
    fn not_delivered_scale_maps_to_exit_70() {
        let e = parse(&args(&["generate", "--scale=default", "--seed=1"])).unwrap_err();
        assert_eq!(e.exit(), Exit::NotDelivered);
        assert_eq!(e.exit().code(), 70);
    }

    /// 负样例：未登记的档位是参数错误。
    #[test]
    fn unknown_scale_maps_to_exit_2() {
        let e = parse(&args(&["generate", "--scale=t0-mega", "--seed=1"])).unwrap_err();
        assert_eq!(e.exit().code(), 2);
    }

    /// D-09：`t0` 与 `small` 两档可解析并落到 generate 命令。
    #[test]
    fn t0_and_small_scales_parse_into_generate() {
        assert_eq!(
            parse(&args(&["generate", "--scale=t0", "--seed=1"])),
            Ok(Command::Generate {
                scale: Scale::T0,
                seed: 1,
                out: None,
            })
        );
        assert_eq!(
            parse(&args(&["generate", "--scale=small", "--seed=2"])),
            Ok(Command::Generate {
                scale: Scale::Small,
                seed: 2,
                out: None,
            })
        );
    }

    /// 负样例：多余参数不得被静默忽略。
    #[test]
    fn unknown_argument_is_rejected() {
        assert_eq!(
            parse(&args(&[
                "generate",
                "--scale=t0-min",
                "--seed=1",
                "--verbose"
            ])),
            Err(CliError::UnknownArgument("--verbose".to_string()))
        );
    }

    #[test]
    fn missing_and_unknown_subcommands_are_rejected() {
        assert_eq!(parse(&[]), Err(CliError::MissingSubcommand));
        assert_eq!(
            parse(&args(&["gen"])),
            Err(CliError::UnknownSubcommand("gen".to_string()))
        );
    }

    #[test]
    fn help_is_available_under_three_spellings() {
        for spelling in ["help", "--help", "-h"] {
            assert_eq!(parse(&args(&[spelling])), Ok(Command::Help));
        }
    }

    /// 用法文本必须印全每个退出码与每个已登记档位，避免文本与实现各说各话。
    #[test]
    fn usage_lists_every_exit_code_and_scale() {
        let text = usage();
        for e in Exit::ALL {
            assert!(
                text.contains(&format!("  {:<4}{}", e.code(), e.label())),
                "缺退出码 {}",
                e.code()
            );
        }
        for name in REGISTERED {
            assert!(text.contains(name), "缺档位 {name}");
        }
    }
}
