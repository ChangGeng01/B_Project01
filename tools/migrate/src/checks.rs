//! `check` 子命令：执行 db/checks/ 的十三个编号合规断言（计划 §3.9）。
//!
//! 每个编号脚本是一条（或 UNION ALL 连接的）查询，返回 0 行即通过；
//! 非 0 行列出违规对象。不编号的 append_only_consistency.sql 归 xtask sqlcheck，
//! 不在本命令的执行范围。发现违规或脚本读不到都落 78：六码里没有「库不合规」
//! 专属码，78 的语义「本机器不具备执行前提」覆盖「库侧状态未达合规」。

use std::path::{Path, PathBuf};

use tokio_postgres::Client;

use crate::exit::{MigrateExit, Outcome};

/// 编号脚本的固定编号与文件名前缀（阶段 2 交付物 D-04 冻结）。
pub const NUMBERED_CHECKS: [&str; 13] = [
    "01_common_columns.sql",
    "02_rls_enabled.sql",
    "03_rls_conformance.sql",
    "04_time_column_types.sql",
    "05_numeric_precision.sql",
    "06_naming.sql",
    "07_identifier_length.sql",
    "08_no_forbidden_objects.sql",
    "09_sql_hygiene.sql",
    "10_baseline_indexes.sql",
    "11_sensitive_field_encryption.sql",
    "12_collation_conformance.sql",
    "13_unpoliced_registry.sql",
];

/// check 脚本目录的默认相对路径。
pub const DEFAULT_CHECKS_DIR: &str = "db/checks";

/// 目录闸（纯判定，无活库可测）：脚本目录不存在即环境自检失败，不判通过。
pub fn dir_gate(checks_dir: &Path) -> Result<(), Outcome> {
    if checks_dir.is_dir() {
        Ok(())
    } else {
        Err(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 checks-dir-readable 不通过：断言脚本目录 {} 不存在或不是目录。",
                checks_dir.display()
            ),
        ))
    }
}

/// check 主流程：逐个脚本执行，汇总后一次性报告。
pub async fn run_check(client: &Client, checks_dir: &Path) -> Result<Outcome, Outcome> {
    if let Err(outcome) = dir_gate(checks_dir) {
        return Ok(outcome);
    }
    let mut passed = Vec::new();
    let mut failed: Vec<String> = Vec::new();
    for name in NUMBERED_CHECKS {
        match run_one(client, checks_dir, name).await {
            Ok(rows) if rows.is_empty() => {
                passed.push(name.to_string());
            }
            Ok(rows) => {
                failed.push(format!(
                    "  {name} 返回 {} 行违规：\n{}",
                    rows.len(),
                    rows.iter()
                        .take(20)
                        .map(|r| format!("    {r}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
            }
            Err(outcome) => return Ok(outcome),
        }
    }
    if failed.is_empty() {
        Ok(Outcome::Done(format!(
            "check 通过：十三个编号断言全部返回 0 行。\n{}",
            passed
                .iter()
                .map(|n| format!("  通过 {n}"))
                .collect::<Vec<_>>()
                .join("\n")
        )))
    } else {
        Ok(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 db-compliant 不通过：{} 个编号断言发现违规：\n{}",
                failed.len(),
                failed.join("\n")
            ),
        ))
    }
}

/// 执行单个脚本，返回每行的文本形态（列以竖线分隔）。
async fn run_one(client: &Client, dir: &Path, name: &str) -> Result<Vec<String>, Outcome> {
    let path: PathBuf = dir.join(name);
    let sql = std::fs::read_to_string(&path).map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 checks-dir-readable 不通过：断言脚本 {} 不可读：{e}",
                path.display()
            ),
        )
    })?;
    let messages = client.simple_query(&sql).await.map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!("环境自检项 db-reachable 不通过：断言脚本 {name} 执行失败：{e}"),
        )
    })?;
    let mut rows = Vec::new();
    for message in messages {
        if let tokio_postgres::SimpleQueryMessage::Row(row) = message {
            let mut cols = Vec::new();
            for i in 0..row.len() {
                cols.push(row.get(i).unwrap_or("NULL").to_string());
            }
            rows.push(cols.join(" | "));
        }
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_checks_are_the_frozen_thirteen() {
        assert_eq!(NUMBERED_CHECKS.len(), 13);
        for (i, name) in NUMBERED_CHECKS.iter().enumerate() {
            let expect_prefix = format!("{:02}_", i + 1);
            assert!(
                name.starts_with(&expect_prefix),
                "第 {} 个脚本必须以 {expect_prefix} 开头：{name}",
                i + 1
            );
        }
        assert!(
            !NUMBERED_CHECKS.contains(&"append_only_consistency.sql"),
            "不编号脚本归 xtask sqlcheck，不归本命令"
        );
    }

    #[test]
    fn missing_checks_dir_is_env_selfcheck_not_pass() {
        // 无活库纪律：读不到被测对象不得判通过。目录闸先于任何数据库动作。
        let missing = std::env::temp_dir().join("ep-migrate-no-such-checks-dir");
        let _ = std::fs::remove_dir_all(&missing);
        let out = dir_gate(&missing).expect_err("目录缺失必须拦");
        assert_eq!(out.exit(), MigrateExit::EnvSelfCheckFailed);
    }
}
