//! ep-datagen — 基准数据集生成器。独立二进制，不属于八进程。
//!
//! 阶段 1 交付骨架与 `t0-min` 最小样本档：一个法人、一个客户、一个产品。
//! 同一 seed 两次生成结果必须字节一致，该判据由 `t0_min` 与 `record` 两处的用例守。
//!
//! 本文件只做「参数 → 产出 → 写出」的接线，判定逻辑一律在子模块里，
//! 使每条判定都能在不起进程的前提下被测到。

mod cli;
mod exit;
mod record;
mod rng;
mod scale;
mod t0_min;
mod uuid7;

use std::io::Write as _;
use std::path::Path;
use std::process::ExitCode;

use cli::Command;
use exit::Exit;
use scale::Scale;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match cli::parse(&args) {
        Ok(Command::Help) => {
            print!("{}", cli::usage());
            Exit::Ok
        }
        Ok(Command::Generate { scale, seed, out }) => generate(scale, seed, out.as_deref()),
        Err(e) => {
            eprintln!("{e}");
            fail(e.exit())
        }
    };
    ExitCode::from(code.code())
}

fn generate(scale: Scale, seed: u64, out: Option<&Path>) -> Exit {
    let dataset = match scale {
        Scale::T0Min => t0_min::build(seed),
    };

    // 形状校验放在写出之前：形状不符的样本档一个字节都不该落地。
    if let Err(violations) = t0_min::verify(&dataset) {
        eprintln!("样本档 {} 形状不符（{} 处）：", scale.as_str(), violations.len());
        for v in &violations {
            eprintln!("  {v}");
        }
        return fail(Exit::ShapeViolation);
    }

    let bytes = match dataset.encode() {
        Ok(b) => b,
        Err(e) => {
            // 编码失败是生成器自身的缺陷，不是调用方的参数问题，必须显式报出来。
            eprintln!("样本档编码失败：{e}");
            return fail(Exit::ShapeViolation);
        }
    };
    match write_out(&bytes, out) {
        Ok(()) => Exit::Ok,
        Err(e) => {
            let target = out.map(|p| p.display().to_string()).unwrap_or_else(|| "stdout".into());
            eprintln!("写出 {target} 失败：{e}");
            fail(Exit::IoError)
        }
    }
}

/// 失败收尾：把退出码与其含义一并印到 stderr，再把它交回调用链。
///
/// 单点收口是为了不出现「印了错误但返回 0」这条路径。
fn fail(exit: Exit) -> Exit {
    debug_assert_ne!(exit, Exit::Ok, "fail 不接受成功态");
    eprintln!("退出码 {}（{}）。", exit.code(), exit.label());
    exit
}

fn write_out(bytes: &[u8], out: Option<&Path>) -> std::io::Result<()> {
    match out {
        Some(path) => std::fs::write(path, bytes),
        None => {
            // 显式 flush：stdout 的缓冲错误若只在进程析构时发生，会以退出码 0 收尾。
            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            lock.write_all(bytes)?;
            lock.flush()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 写文件与写 stdout 必须是同一批字节：判据只认字节，不认写出方式。
    #[test]
    fn file_output_matches_encoded_bytes() {
        let expected = t0_min::build(11).encode().unwrap();
        let path = std::env::temp_dir().join("ep-datagen-t0-min-11.dataset");
        assert_eq!(generate(Scale::T0Min, 11, Some(&path)), Exit::Ok);
        let written = std::fs::read(&path).expect("样本档应已写出");
        assert_eq!(written, expected);
        let _ = std::fs::remove_file(&path);
    }

    /// 负样例：写不进去的路径必须以 74 收尾，不得当成功。
    #[test]
    fn unwritable_target_reports_io_error() {
        let path = Path::new("/this-path-does-not-exist-ep-datagen/out.dataset");
        let got = generate(Scale::T0Min, 1, Some(path));
        assert_eq!(got, Exit::IoError);
        assert_eq!(got.code(), 74);
    }
}
