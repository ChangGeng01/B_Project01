//! 单个选项的取值校验与落位。从 `cli.rs` 拆出来，是为了让「命令行长什么样」
//! 与「每个取值合不合法」两件事各占一个文件，改一处不必读另一处。
//!
//! 这里只判形态，不判语义：连接串只看协议前缀，不解析、不连接；标识符只看
//! PostgreSQL 的无引号形态。语义判定要么在前置阶梯里（见 `preflight.rs`），
//! 要么归阶段 2。

use std::path::PathBuf;

use crate::cli::{err, Invocation, ParseError, StatusFormat, MAX_TTL_MINUTES};
use crate::manifest::is_wellformed_sha256;

/// PostgreSQL 标识符形态：小写字母或下划线开头，其后小写字母、数字、下划线，
/// 长度不超过 63。大写与引号形态一律拒绝，避免把带引号标识符引进迁移路径。
fn is_ident(s: &str) -> bool {
    if s.is_empty() || s.len() > 63 {
        return false;
    }
    let mut bytes = s.bytes();
    let first = bytes.next().unwrap_or(b'0');
    let head_ok = first == b'_' || first.is_ascii_lowercase();
    head_ok && bytes.all(|b| b == b'_' || b.is_ascii_lowercase() || b.is_ascii_digit())
}

/// 连接串形态：只判协议前缀与非空主体，不解析也不连接。
fn is_pg_url(s: &str) -> bool {
    let body = s
        .strip_prefix("postgresql://")
        .or_else(|| s.strip_prefix("postgres://"));
    matches!(body, Some(rest) if !rest.is_empty())
}

/// 把一个已确认属于本子命令的选项落进 `Invocation`。
pub fn apply_option(inv: &mut Invocation, key: &str, value: &str) -> Result<(), ParseError> {
    match key {
        "db-url" => {
            if !is_pg_url(value) {
                return err(
                    "--db-url 形态不合法：必须是 postgresql:// 或 postgres:// 开头的非空连接串。",
                );
            }
            inv.db_url = Some(value.to_string());
        }
        "migrations-dir" => {
            if value.is_empty() {
                return err("--migrations-dir 不得为空。");
            }
            inv.migrations_dir = PathBuf::from(value);
        }
        "history-schema" => {
            if !is_ident(value) {
                return err(format!("--history-schema 取值 {value} 不是合法标识符。"));
            }
            inv.history_schema = value.to_string();
        }
        "history-table" => {
            if !is_ident(value) {
                return err(format!("--history-table 取值 {value} 不是合法标识符。"));
            }
            inv.history_table = value.to_string();
        }
        "window-id" => {
            if value.is_empty() {
                return err("--window-id 不得为空。");
            }
            inv.window_id = Some(value.to_string());
        }
        "expect-tool-version" => {
            if value.is_empty() {
                return err("--expect-tool-version 不得为空。");
            }
            inv.expect_tool_version = Some(value.to_string());
        }
        "expect-manifest-sha256" => {
            if !is_wellformed_sha256(value) {
                return err("--expect-manifest-sha256 必须是 64 位小写十六进制。");
            }
            inv.expect_manifest_sha256 = Some(value.to_string());
        }
        "format" => {
            inv.format = match value {
                "text" => StatusFormat::Text,
                "json" => StatusFormat::Json,
                "manifest" => StatusFormat::Manifest,
                other => return err(format!("--format 取值 {other} 未知；可用 text、json、manifest。")),
            };
        }
        "ttl-minutes" => {
            let n: u32 = value
                .parse()
                .map_err(|_| ParseError(format!("--ttl-minutes 取值 {value} 不是非负整数。")))?;
            if n == 0 || n > MAX_TTL_MINUTES {
                return err(format!(
                    "--ttl-minutes 取值 {n} 越界；允许 1 至 {MAX_TTL_MINUTES}。"
                ));
            }
            inv.ttl_minutes = n;
        }
        "reason" => {
            if value.is_empty() {
                return err("--reason 不得为空。");
            }
            inv.reason = Some(value.to_string());
        }
        "schema" => {
            if !is_ident(value) {
                return err(format!("--schema 取值 {value} 不是合法标识符。"));
            }
            inv.rls_schema = Some(value.to_string());
        }
        "table" => {
            if !is_ident(value) {
                return err(format!("--table 取值 {value} 不是合法标识符。"));
            }
            inv.rls_table = Some(value.to_string());
        }
        "out" => {
            if value.is_empty() {
                return err("--out 不得为空。");
            }
            inv.out = Some(PathBuf::from(value));
        }
        other => return err(format!("未知选项 --{other}。")),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ident_form_is_postgres_unquoted_lowercase() {
        assert!(is_ident("platform_core"));
        assert!(is_ident("_x9"));
        assert!(!is_ident("Platform"));
        assert!(!is_ident("9x"));
        assert!(!is_ident(""));
        assert!(!is_ident(&"a".repeat(64)));
        assert!(!is_ident("a-b"));
        assert!(!is_ident("a\"b"));
    }

    #[test]
    fn pg_url_form_requires_scheme_and_body() {
        assert!(is_pg_url("postgres://h/ep"));
        assert!(is_pg_url("postgresql://h/ep"));
        assert!(!is_pg_url("postgres://"));
        assert!(!is_pg_url("mysql://h/ep"));
        assert!(!is_pg_url(""));
    }
}
