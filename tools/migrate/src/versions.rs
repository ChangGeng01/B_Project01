//! 库侧期望版本清单（schema-history-version-matched 闸的纯逻辑段）。
//!
//! 清单路径取环境变量 `EP__DB__MIGRATION__EXPECTED_VERSIONS_PATH`，
//! 缺省 `/etc/ep/migration-versions.toml`（计划 §7 逐字）。
//! 文件不存在不算失败——该闸判「未覆盖」；存在但解析不了才是环境自检失败。

use std::path::{Path, PathBuf};

/// 清单文件默认路径（计划 §7 表逐字）。
pub const DEFAULT_EXPECTED_VERSIONS_PATH: &str = "/etc/ep/migration-versions.toml";

/// 解析清单路径：环境变量优先，缺省取默认路径。
pub fn expected_versions_path(env_path: Option<&str>) -> PathBuf {
    env_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_EXPECTED_VERSIONS_PATH))
}

/// 读清单并取出期望版本。
/// - 文件不存在 → Ok(None)，闸判未覆盖；
/// - 存在但解析失败 → Err，由调用方落 78；
/// - 解析成功 → Ok(Some(version))。
pub fn load_expected_version(path: &Path) -> Result<Option<i64>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("期望版本清单 {} 不可读：{e}", path.display()))?;
    parse_expected_version(&text).map(Some).ok_or_else(|| {
        format!(
            "期望版本清单 {} 缺少 expected_version 键，或取值不是整数/十进制字符串",
            path.display()
        )
    })
}

/// 解析清单文本。接受 `expected_version = 20260901121000`
/// 或 `expected_version = "20260901121000"` 两种形态。
pub fn parse_expected_version(text: &str) -> Option<i64> {
    let value: toml::Value = toml::from_str(text).ok()?;
    let entry = value.get("expected_version")?;
    match entry {
        toml::Value::Integer(n) => Some(*n),
        toml::Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

/// 闸判定纯函数：期望版本与实际版本（库侧或目录目标版本）比对。
/// None 期望 → 未覆盖（返回 None）；相等 → Some(Ok(()))；不等 → Some(Err(说明))。
pub fn judge(expected: Option<i64>, actual: i64) -> Option<Result<(), String>> {
    expected.map(|want| {
        if want == actual {
            Ok(())
        } else {
            Err(format!(
                "版本不一致：期望版本清单给出 {want}，实际版本为 {actual}"
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_overrides_default_path() {
        assert_eq!(
            expected_versions_path(Some("/tmp/x.toml")),
            PathBuf::from("/tmp/x.toml")
        );
        assert_eq!(
            expected_versions_path(None),
            PathBuf::from(DEFAULT_EXPECTED_VERSIONS_PATH)
        );
    }

    #[test]
    fn parse_integer_and_string_forms() {
        assert_eq!(
            parse_expected_version("expected_version = 20260901121000"),
            Some(20260901121000)
        );
        assert_eq!(
            parse_expected_version("expected_version = \"20260901121000\""),
            Some(20260901121000)
        );
        assert_eq!(parse_expected_version("other = 1"), None);
        assert_eq!(parse_expected_version("expected_version = true"), None);
        assert_eq!(parse_expected_version("not toml ["), None);
    }

    #[test]
    fn missing_file_is_not_covered_not_failure() {
        let p = std::env::temp_dir().join("ep-migrate-no-such-versions.toml");
        let _ = std::fs::remove_file(&p);
        assert_eq!(load_expected_version(&p), Ok(None));
    }

    #[test]
    fn judge_three_ways() {
        assert_eq!(judge(None, 5), None, "无期望即未覆盖");
        assert_eq!(judge(Some(5), 5), Some(Ok(())));
        let bad = judge(Some(5), 6).expect("有期望且不等须给说明");
        assert!(bad.is_err());
    }
}
