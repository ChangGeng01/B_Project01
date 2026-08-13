//! `concurrent/` 非事务执行器（计划 §3.3 逐字口径）。
//!
//! `CREATE INDEX CONCURRENTLY` 不能在事务块内执行，本执行器以自动提交模式
//! 逐条执行，只做四件事：读文件、算校验和、执行、写历史。
//! 风险点是中途失败留下失效索引，因此：
//! - 执行前先 `DROP INDEX IF EXISTS` 同名对象；
//! - 执行后校验 `pg_index.indisvalid`，无效即报错并要求人工清理；
//! - 成功后按历史表同一结构插入一行（version、name、applied_on、checksum），
//!   校验和经 [`crate::history::migration_checksum`]，与事务执行器严格一致。
//!
//! 结构冲突记录见 `history.rs` 模块头：本执行器与事务执行器共同替代
//! refinery Runner，历史表形态与其逐项对齐。

use tokio_postgres::Client;

use crate::exit::{MigrateExit, Outcome};
use crate::history::{
    align_history_owner_sql, create_history_table_sql, insert_history_sql, migration_checksum,
    MigrationFile,
};

fn db_failure(detail: String) -> Outcome {
    Outcome::Failed(
        MigrateExit::EnvSelfCheckFailed,
        format!("环境自检项 db-reachable 不通过：{detail}"),
    )
}

/// 从 concurrent 迁移文件的 SQL 中解析索引名与目标 schema。
/// 期望形态：`create index concurrently <name> on <schema>.<table> ...`，
/// 允许大小写混杂与 `if not exists`。解析不出即拒绝执行——宁可不做，
/// 不留一个 DROP/校验对不上号的索引。
pub fn parse_concurrent_index(sql: &str) -> Option<(String, String)> {
    // 去注释行，压平空白，按词元解析。
    let body: String = sql
        .lines()
        .filter(|l| !l.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join(" ");
    let words: Vec<String> = body
        .split_whitespace()
        .map(|t| t.to_ascii_lowercase())
        .collect();
    let mut saw_create_index_concurrently = false;
    let mut index_name: Option<String> = None;
    let mut schema: Option<String> = None;
    let mut i = 0;
    while i < words.len() {
        let w = &words[i];
        if !saw_create_index_concurrently
            && w == "create"
            && words.get(i + 1).map(String::as_str) == Some("index")
            && words.get(i + 2).map(String::as_str) == Some("concurrently")
        {
            saw_create_index_concurrently = true;
            i += 3;
            if words.get(i).map(String::as_str) == Some("if")
                && words.get(i + 1).map(String::as_str) == Some("not")
                && words.get(i + 2).map(String::as_str) == Some("exists")
            {
                i += 3;
            }
            continue;
        }
        if saw_create_index_concurrently && index_name.is_none() {
            index_name = Some(strip_sql_punct(w));
            i += 1;
            continue;
        }
        if saw_create_index_concurrently && index_name.is_some() && w == "on" && schema.is_none() {
            let target = words.get(i + 1).map(|t| strip_sql_punct(t));
            if let Some(target) = target {
                if let Some((s, _table)) = target.split_once('.') {
                    schema = Some(s.to_string());
                }
            }
            break;
        }
        i += 1;
    }
    match (index_name, schema) {
        (Some(name), Some(schema)) if !name.is_empty() && !schema.is_empty() => {
            Some((name, schema))
        }
        _ => None,
    }
}

fn strip_sql_punct(t: &str) -> String {
    t.trim_matches(|c: char| c == '(' || c == ')' || c == ';' || c == ',')
        .to_string()
}

/// 执行一个 concurrent 迁移文件并写历史。自动提交模式：
/// 每条语句单独经 simple query 通道执行，不落事务块。
pub async fn apply_one(
    client: &Client,
    file: &MigrationFile,
    history_schema: &str,
    history_table: &str,
) -> Result<(), Outcome> {
    let sql = std::fs::read_to_string(&file.path).map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 migrations-dir-readable 不通过：迁移文件 {} 不可读：{e}",
                file.path.display()
            ),
        )
    })?;
    let (index_name, index_schema) = parse_concurrent_index(&sql).ok_or_else(|| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 concurrent-form 不通过：{} 解析不出 \
                 create index concurrently <名> on <schema>.<表> 形态，拒绝执行",
                file.path.display()
            ),
        )
    })?;

    // 一、清场：同名失效索引先删，避免中途失败留下的残骸挡住重建。
    client
        .batch_execute(&format!("DROP INDEX IF EXISTS {index_schema}.{index_name}"))
        .await
        .map_err(|e| {
            db_failure(format!(
                "清理同名索引 {index_schema}.{index_name} 失败：{e}"
            ))
        })?;

    // 二、执行：simple query 单语句即自动提交，不在事务块内。
    let messages = client.simple_query(&sql).await.map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "concurrent 迁移 V{} 执行失败（索引可能残留，需人工清理）：{e}",
                file.version
            ),
        )
    })?;
    let _ = messages;

    // 三、校验 indisvalid：无效即报错并要求人工清理。
    let row = client
        .query_one(
            "SELECT i.indisvalid FROM pg_index i \
             JOIN pg_class c ON c.oid = i.indexrelid \
             JOIN pg_namespace n ON n.oid = c.relnamespace \
             WHERE n.nspname = $1 AND c.relname = $2",
            &[&index_schema, &index_name],
        )
        .await
        .map_err(|e| db_failure(format!("校验 pg_index.indisvalid 失败：{e}")))?;
    if !row.get::<_, bool>(0) {
        return Err(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 index-valid 不通过：索引 {index_schema}.{index_name} 建立后 \
                 pg_index.indisvalid 为假，请人工 DROP INDEX 后重跑 apply。"
            ),
        ));
    }

    // 四、写历史：与事务执行器同一结构、同一校验和算法。
    client
        .batch_execute(&create_history_table_sql(history_schema, history_table))
        .await
        .map_err(|e| db_failure(format!("建历史表失败：{e}")))?;
    client
        .batch_execute(&align_history_owner_sql(history_schema, history_table))
        .await
        .map_err(|e| db_failure(format!("历史表属主归位失败：{e}")))?;
    let checksum = migration_checksum(&file.name, file.version, &sql).to_string();
    let applied_on = chrono::Utc::now().to_rfc3339();
    client
        .execute(
            &insert_history_sql(history_schema, history_table),
            &[&file.version, &file.name, &applied_on, &checksum],
        )
        .await
        .map_err(|e| {
            db_failure(format!(
                "concurrent 迁移 V{} 已执行但写历史表失败，需人工补记或回清：{e}",
                file.version
            ))
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_typical_concurrent_index() {
        let sql = "-- rollback: drop index mdm.ix_parties_le_created_at;\n\
                   CREATE INDEX CONCURRENTLY ix_parties_le_created_at \
                   ON mdm.parties (legal_entity_id, created_at);";
        let (name, schema) = parse_concurrent_index(sql).expect("典型形态可解析");
        assert_eq!(name, "ix_parties_le_created_at");
        assert_eq!(schema, "mdm");
    }

    #[test]
    fn parse_accepts_if_not_exists_and_lowercase() {
        let sql = "create index concurrently if not exists ix_a on crm.a (b)";
        assert_eq!(
            parse_concurrent_index(sql),
            Some(("ix_a".to_string(), "crm".to_string()))
        );
    }

    #[test]
    fn parse_rejects_non_concurrent_or_shapeless() {
        assert!(parse_concurrent_index("create index ix_a on s.t (c)").is_none());
        assert!(parse_concurrent_index("select 1").is_none());
        assert!(parse_concurrent_index("").is_none());
    }
}
