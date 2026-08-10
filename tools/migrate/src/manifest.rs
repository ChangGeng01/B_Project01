//! 迁移清单哈希。
//!
//! 归一化规则取自阶段 1 计划第 7 节自检项 `migration-version-matched` 的算法
//! 描述「统一 LF、去行尾空白」。这里把「同样归一化」写死成一条可执行的定义，
//! 免得目录侧与库侧两处各自理解一遍——阶段 2 计划的 R-01 已经点名两套执行器
//! 的校验和算法必须严格一致，两处措辞不同即是分歧的起点。
//!
//! 冻结定义：
//! 一、递归收集目录下的全部普通文件，按相对路径的字节序升序排列；空目录不
//!     贡献任何输入，因此本阶段 24 个空目录算出的就是空串的 SHA-256。
//! 二、每个文件按 UTF-8 读入，`\r\n` 与孤立 `\r` 一律换成 `\n`。
//! 三、每行去掉行尾的空格与制表符。
//! 四、去掉文件末尾的全部空行；若剩余非空，补一个 `\n` 收尾。
//! 五、按序把各文件归一化后的内容首尾相接，对拼接结果取 SHA-256。
//!     文件名不进哈希输入，与计划原文「对全部文件做同样归一化后计算」一致。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::sha256::sha256_hex;

/// 清单计算失败的原因。一律显式向上传递，不吞、不降级为「算作空清单」。
#[derive(Debug)]
pub enum ManifestError {
    NotADirectory(PathBuf),
    Io(PathBuf, io::Error),
    NotUtf8(PathBuf),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::NotADirectory(p) => {
                write!(f, "迁移目录不存在或不是目录：{}", p.display())
            }
            ManifestError::Io(p, e) => write!(f, "读取 {} 失败：{e}", p.display()),
            ManifestError::NotUtf8(p) => write!(
                f,
                "{} 不是 UTF-8 文本，无法按归一化规则计算清单哈希",
                p.display()
            ),
        }
    }
}

/// 把一个文件的内容按冻结规则归一化。
fn normalize(raw: &str) -> String {
    let unified = raw.replace("\r\n", "\n").replace('\r', "\n");
    let trimmed: Vec<&str> = unified
        .split('\n')
        .map(|line| line.trim_end_matches([' ', '\t']))
        .collect();
    let mut end = trimmed.len();
    while end > 0 && trimmed[end - 1].is_empty() {
        end -= 1;
    }
    if end == 0 {
        return String::new();
    }
    let mut out = trimmed[..end].join("\n");
    out.push('\n');
    out
}

/// 递归收集目录下全部普通文件的相对路径，按字节序升序。
fn collect_files(root: &Path, dir: &Path, acc: &mut Vec<PathBuf>) -> Result<(), ManifestError> {
    let entries = fs::read_dir(dir).map_err(|e| ManifestError::Io(dir.to_path_buf(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| ManifestError::Io(dir.to_path_buf(), e))?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(|e| ManifestError::Io(path.clone(), e))?;
        if meta.is_dir() {
            collect_files(root, &path, acc)?;
        } else {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_path_buf();
            acc.push(rel);
        }
    }
    Ok(())
}

/// 算出迁移目录的清单哈希，返回 64 位小写十六进制。
pub fn manifest_sha256(root: &Path) -> Result<String, ManifestError> {
    if !root.is_dir() {
        return Err(ManifestError::NotADirectory(root.to_path_buf()));
    }
    let mut rels = Vec::new();
    collect_files(root, root, &mut rels)?;
    rels.sort();

    let mut input = String::new();
    for rel in &rels {
        let full = root.join(rel);
        let bytes = fs::read(&full).map_err(|e| ManifestError::Io(full.clone(), e))?;
        let text = String::from_utf8(bytes).map_err(|_| ManifestError::NotUtf8(full.clone()))?;
        input.push_str(&normalize(&text));
    }
    Ok(sha256_hex(input.as_bytes()))
}

/// 期望值的形态判定：64 位小写十六进制。形态不合法属参数错误，不属校验和不符。
pub fn is_wellformed_sha256(v: &str) -> bool {
    v.len() == 64 && v.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmpdir(tag: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "ep-migrate-manifest-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        fs::create_dir_all(&p).expect("建临时目录");
        p
    }

    #[test]
    fn normalization_unifies_line_endings_and_trailing_blanks() {
        assert_eq!(normalize("a  \r\nb\t\r\n\n\n"), "a\nb\n");
        assert_eq!(normalize("a\rb"), "a\nb\n");
        assert_eq!(normalize("   \n\t\n"), "");
    }

    #[test]
    fn empty_dirs_hash_to_empty_input() {
        let root = tmpdir("empty");
        fs::create_dir_all(root.join("platform_core")).expect("建子目录");
        fs::create_dir_all(root.join("mdm")).expect("建子目录");
        assert_eq!(
            manifest_sha256(&root).expect("可计算"),
            sha256_hex(b""),
            "空目录不贡献输入，清单为空集"
        );
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn tampering_one_file_changes_the_hash() {
        let root = tmpdir("tamper");
        fs::create_dir_all(root.join("mdm")).expect("建子目录");
        fs::write(root.join("mdm/V001__a.sql"), "-- rollback: drop table t;\n").expect("写文件");
        let before = manifest_sha256(&root).expect("可计算");
        fs::write(root.join("mdm/V001__a.sql"), "-- rollback: drop table u;\n").expect("改文件");
        let after = manifest_sha256(&root).expect("可计算");
        assert_ne!(before, after, "篡改一个文件后清单哈希必须变化");
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn line_ending_only_change_does_not_change_the_hash() {
        let root = tmpdir("crlf");
        fs::write(root.join("a.sql"), "select 1;\nselect 2;\n").expect("写文件");
        let lf = manifest_sha256(&root).expect("可计算");
        fs::write(root.join("a.sql"), "select 1;  \r\nselect 2;\r\n\r\n").expect("写文件");
        let crlf = manifest_sha256(&root).expect("可计算");
        assert_eq!(lf, crlf, "归一化后行尾差异不得影响清单哈希");
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn missing_dir_is_an_error_not_an_empty_manifest() {
        let root = tmpdir("missing").join("nope");
        let err = manifest_sha256(&root).expect_err("不存在的目录必须报错");
        assert!(matches!(err, ManifestError::NotADirectory(_)));
    }

    #[test]
    fn non_utf8_file_is_an_error_not_skipped() {
        let root = tmpdir("binary");
        fs::write(root.join("bad.sql"), [0xffu8, 0xfe, 0x00]).expect("写文件");
        let err = manifest_sha256(&root).expect_err("非 UTF-8 必须报错");
        assert!(matches!(err, ManifestError::NotUtf8(_)));
        fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn wellformed_check_rejects_uppercase_and_short_values() {
        assert!(is_wellformed_sha256(&sha256_hex(b"")));
        assert!(!is_wellformed_sha256(&sha256_hex(b"").to_uppercase()));
        assert!(!is_wellformed_sha256("deadbeef"));
        assert!(!is_wellformed_sha256(""));
    }
}
