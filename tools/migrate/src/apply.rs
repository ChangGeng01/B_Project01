//! `apply` 编排与 `status` 输出。
//!
//! apply 入口判定（裁定见 02 计划第 12 节偏离登记十四，判据在 `bootstrap.rs`）：
//! - 未出示 `--window-id` 且目标库无历史表 → 首装自举：以 ep_migrator 身份
//!   建历史表与窗口两表并自开一次性安装窗口，随后按正常流程执行全部迁移；
//! - 出示了 `--window-id` → 窗口闸（读库判 OPEN，退出码 3）先行，次序不变；
//! - 非空库且未出示窗口 → 正常比对路径：无待执行退出码 0，有待执行落 3。
//!
//! 正常流程顺序（计划 §3.3 与前置阶梯）：
//! 1. 窗口闸（读 migration_windows/migration_window_lock 判 OPEN，退出码 3）；
//! 2. 版本闸（期望版本清单 vs 目录目标版本，退出码 5；清单缺失判未覆盖不拦）；
//! 3. 扫描迁移目录，按文件版本号全序与历史表比对：
//!    - 已应用版本的文件校验和不符 → 退出码 4；
//!    - 已应用版本无对应文件 → 版本漂移 → 退出码 5；
//! 4. 逐个执行未应用文件：常规文件走事务执行器（每文件一事务），
//!    `concurrent/` 文件走非事务执行器；两路径共用历史表与校验和算法；
//! 5. 每成功一个文件，把版本号回写窗口行 applied_versions。
//!
//! 结构冲突记录（refinery i32 版本号 vs 14 位时间戳）见 `history.rs` 模块头。

use tokio_postgres::Client;

use crate::bootstrap;
use crate::cli::{Invocation, StatusFormat};
use crate::concurrent;
use crate::dbconn::apply_session_preamble;
use crate::exit::{MigrateExit, Outcome};
use crate::history::{
    align_history_owner_sql, create_history_table_sql, insert_history_sql, migration_checksum,
    scan_migrations, select_history_sql, select_max_version_sql, MigrationFile,
};
use crate::versions;
use crate::window;

fn db_failure(detail: String) -> Outcome {
    Outcome::Failed(
        MigrateExit::EnvSelfCheckFailed,
        format!("环境自检项 db-reachable 不通过：{detail}"),
    )
}

/// 历史表里的一行已应用记录。
pub struct AppliedRow {
    pub version: i64,
    pub name: String,
    pub checksum: String,
}

/// 历史表存在性探测：与 read_history 同一 to_regclass 口径，
/// 首装自举的触发判定（bootstrap.rs）据此区分空库与非空库。
pub async fn history_table_exists(client: &Client, inv: &Invocation) -> Result<bool, Outcome> {
    client
        .query_one(
            "SELECT to_regclass($1) IS NOT NULL",
            &[&format!("{}.{}", inv.history_schema, inv.history_table)],
        )
        .await
        .map_err(|e| db_failure(format!("探测历史表失败：{e}")))
        .map(|row| row.get(0))
}

pub async fn read_history(client: &Client, inv: &Invocation) -> Result<Vec<AppliedRow>, Outcome> {
    // 空库首装时历史表尚不存在：按空历史处理，不得报错拦住首个迁移。
    if !history_table_exists(client, inv).await? {
        return Ok(Vec::new());
    }
    let rows = client
        .query(
            &select_history_sql(&inv.history_schema, &inv.history_table),
            &[],
        )
        .await
        .map_err(|e| db_failure(format!("读取历史表失败（迁移未应用？）：{e}")))?;
    Ok(rows
        .iter()
        .map(|r| AppliedRow {
            version: r.get(0),
            name: r.get(1),
            checksum: r.get(3),
        })
        .collect())
}

/// 目录全序与历史表的比对判定（纯函数，无活库可测）。
/// 返回 (待执行文件, 报告行)。
pub fn plan_pending(
    files: &[MigrationFile],
    applied: &[AppliedRow],
) -> Result<(Vec<usize>, Vec<String>), Outcome> {
    let mut notes = Vec::new();
    let max_applied = applied.iter().map(|a| a.version).max();
    for file in files {
        match applied.iter().find(|a| a.version == file.version) {
            Some(row) => {
                let actual = migration_checksum(&file.name, file.version, &read_sql_or_empty(file));
                if row.checksum != actual.to_string() {
                    return Err(Outcome::Failed(
                        MigrateExit::ChecksumMismatch,
                        format!(
                            "校验和不符：版本 {} 的已应用记录校验和为 {}，\
                             目录文件 {} 实算为 {actual}。迁移文件在应用后被改动过。",
                            file.version,
                            row.checksum,
                            file.path.display()
                        ),
                    ));
                }
            }
            None => {
                if let Some(max) = max_applied {
                    if file.version < max {
                        return Err(Outcome::Failed(
                            MigrateExit::VersionMismatch,
                            format!(
                                "版本不一致：文件版本 {} 早于已应用的最高版本 {max}，\
                                 迁移全序已漂移（{}）。",
                                file.version,
                                file.path.display()
                            ),
                        ));
                    }
                }
                notes.push(format!("待执行 V{} {}", file.version, file.name));
            }
        }
    }
    // 已应用但目录中无对应文件：漂移。
    for row in applied {
        if !files.iter().any(|f| f.version == row.version) {
            return Err(Outcome::Failed(
                MigrateExit::VersionMismatch,
                format!(
                    "版本不一致：历史表有版本 {} 的记录（{}），目录中却无对应迁移文件。",
                    row.version, row.name
                ),
            ));
        }
    }
    let pending_idx: Vec<usize> = files
        .iter()
        .enumerate()
        .filter(|(_, f)| !applied.iter().any(|a| a.version == f.version))
        .map(|(i, _)| i)
        .collect();
    Ok((pending_idx, notes))
}

fn read_sql_or_empty(file: &MigrationFile) -> String {
    std::fs::read_to_string(&file.path).unwrap_or_default()
}

/// apply 主流程。
pub async fn run_apply(
    client: &mut Client,
    inv: &Invocation,
    env_versions_path: Option<&str>,
) -> Result<Outcome, Outcome> {
    apply_session_preamble(client).await?;

    // 一、入口判定：先探历史表定入口（空库首装自举见 bootstrap.rs）。
    let history_exists = history_table_exists(client, inv).await?;
    let entry = bootstrap::decide_entry(history_exists, inv.window_id.is_some());

    // 二、窗口闸：出示窗口的调用维持原次序，先读库判 OPEN 再谈其余；
    // 空库上出示的窗口不可能存在，由本闸以库侧事实拒绝，自举不接管该路。
    let presented_window = if entry == bootstrap::ApplyEntry::PresentedWindow {
        Some(
            window::assert_open(
                client,
                inv.window_id
                    .as_deref()
                    .expect("PresentedWindow 入口表示 window_id 已出示"),
            )
            .await?,
        )
    } else {
        None
    };

    // 三、扫描与历史比对。
    let files = scan_migrations(&inv.migrations_dir).map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!("环境自检项 migrations-dir-readable 不通过：{e}"),
        )
    })?;
    let applied = read_history(client, inv).await?;

    // 四、版本闸：期望版本清单 vs 目录目标版本（目录全序的最高版本）。
    let target_version = files.last().map(|f| f.version).unwrap_or_default();
    let versions_path = versions::expected_versions_path(env_versions_path);
    let expected = versions::load_expected_version(&versions_path).map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!("环境自检项 versions-file-parseable 不通过：{e}"),
        )
    })?;
    let mut version_note = format!(
        "schema-history-version-matched 未覆盖：期望版本清单 {} 不存在。",
        versions_path.display()
    );
    if let Some(judgement) = versions::judge(expected, target_version) {
        judgement.map_err(|msg| Outcome::Failed(MigrateExit::VersionMismatch, msg))?;
        version_note =
            format!("schema-history-version-matched 通过：期望版本 {target_version} 与清单一致。");
    }

    let (pending_idx, _notes) = plan_pending(&files, &applied)?;
    if pending_idx.is_empty() && entry != bootstrap::ApplyEntry::Bootstrap {
        // 非自举入口且无待执行：正常比对路径的完成态（自举后重复 apply 落这里）。
        return Ok(Outcome::Done(format!(
            "无待执行迁移：目录 {} 共 {} 个文件全部已应用，库版本 {target_version}。\n{version_note}",
            inv.migrations_dir.display(),
            files.len()
        )));
    }

    // 五、落定窗口：自举开窗 / 已出示窗口 / 非空库未出示窗口落 3。
    let (window_id, bootstrap_note) = match entry {
        bootstrap::ApplyEntry::Bootstrap => {
            let id =
                bootstrap::run_bootstrap(client, &inv.history_schema, &inv.history_table).await?;
            (
                id.clone(),
                format!(
                    "首装自举：目标库无 {}.{}，已以 ep_migrator 身份建历史表与迁移窗口两表，\
                     并自开一次性安装窗口 {id}（审批引用 {}），到期按既有窗口生命周期机制自动关闭。",
                    inv.history_schema,
                    inv.history_table,
                    bootstrap::INITIAL_INSTALL_APPROVAL_REF
                ),
            )
        }
        bootstrap::ApplyEntry::PresentedWindow => (
            presented_window.expect("PresentedWindow 入口在上面已取得窗口 id"),
            String::new(),
        ),
        bootstrap::ApplyEntry::Bare => {
            return Err(Outcome::Failed(
                MigrateExit::MigrationWindowClosed,
                format!(
                    "迁移窗口未打开（PLATFORM.DB.MIGRATION_WINDOW_CLOSED）：目标库已有历史表\
                     且尚有 {} 个待执行迁移，apply 必须以 --window-id 出示一个已打开的迁移窗口。\n\
                     窗口由 ep-migrate open-window 开启，登记在 platform_core.migration_windows；\
                     空库首装无需出示窗口，由首装自举承担（本库历史表已存在，不属于首装）。",
                    pending_idx.len()
                ),
            ));
        }
    };

    // 六、按全序逐个执行。
    let mut done = Vec::new();
    for idx in pending_idx {
        let file = &files[idx];
        let sql = std::fs::read_to_string(&file.path).map_err(|e| {
            Outcome::Failed(
                MigrateExit::EnvSelfCheckFailed,
                format!(
                    "环境自检项 migrations-dir-readable 不通过：迁移文件 {} 不可读：{e}",
                    file.path.display()
                ),
            )
        })?;
        if file.concurrent {
            concurrent::apply_one(client, file, &inv.history_schema, &inv.history_table).await?;
        } else {
            apply_transactional(client, file, &sql, &inv.history_schema, &inv.history_table)
                .await?;
        }
        window::append_applied_version(client, &window_id, file.version).await?;
        done.push(format!(
            "已执行 V{} {}（{}）",
            file.version,
            file.name,
            if file.concurrent {
                "concurrent 路径"
            } else {
                "事务路径"
            }
        ));
    }

    let bootstrap_note = if bootstrap_note.is_empty() {
        String::new()
    } else {
        format!("{bootstrap_note}\n")
    };
    Ok(Outcome::Done(format!(
        "apply 完成：本次执行 {} 个迁移，库版本 {}。\n{}{}\n{}",
        done.len(),
        target_version,
        bootstrap_note,
        done.join("\n"),
        version_note
    )))
}

/// 事务执行器：每个迁移一个事务（refinery 默认行为），
/// 会话两条固定设置在事务内重申，历史行与迁移 SQL 同事务落库。
async fn apply_transactional(
    client: &mut Client,
    file: &MigrationFile,
    sql: &str,
    history_schema: &str,
    history_table: &str,
) -> Result<(), Outcome> {
    let tx = client
        .transaction()
        .await
        .map_err(|e| db_failure(format!("开启迁移事务失败：{e}")))?;
    for stmt in crate::dbconn::SESSION_PREAMBLE {
        tx.batch_execute(stmt)
            .await
            .map_err(|e| db_failure(format!("迁移会话设置失败：{e}")))?;
    }
    tx.batch_execute(sql).await.map_err(|e| {
        Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 db-reachable 不通过：迁移 V{}（{}）执行失败：{e}",
                file.version, file.name
            ),
        )
    })?;
    tx.batch_execute(&create_history_table_sql(history_schema, history_table))
        .await
        .map_err(|e| db_failure(format!("建历史表失败：{e}")))?;
    tx.batch_execute(&align_history_owner_sql(history_schema, history_table))
        .await
        .map_err(|e| db_failure(format!("历史表属主归位失败：{e}")))?;
    let checksum = migration_checksum(&file.name, file.version, sql).to_string();
    let applied_on = chrono::Utc::now().to_rfc3339();
    tx.execute(
        &insert_history_sql(history_schema, history_table),
        &[&file.version, &file.name, &applied_on, &checksum],
    )
    .await
    .map_err(|e| db_failure(format!("写历史表失败：{e}")))?;
    tx.commit()
        .await
        .map_err(|e| db_failure(format!("提交迁移事务失败：{e}")))?;
    Ok(())
}

/// status：输出历史表单版本；--format=json 出 JSON；--format=manifest 出制品清单。
pub async fn run_status(client: &Client, inv: &Invocation) -> Result<Outcome, Outcome> {
    let table = format!("{}.{}", inv.history_schema, inv.history_table);
    let row = client
        .query_one(
            &select_max_version_sql(&inv.history_schema, &inv.history_table),
            &[],
        )
        .await
        .map_err(|e| db_failure(format!("读取历史表 {table} 失败（迁移未应用？）：{e}")))?;
    let max: Option<i64> = row.get(0);
    let count_rows = client
        .query_one(&format!("SELECT count(*) FROM {table}"), &[])
        .await
        .map_err(|e| db_failure(format!("清点历史表 {table} 失败：{e}")))?;
    let count: i64 = count_rows.get(0);

    match inv.format {
        StatusFormat::Json => Ok(Outcome::Done(match max {
            Some(v) => format!(
                "{{\"history_table\":\"{table}\",\"version\":{v},\"applied_count\":{count}}}"
            ),
            None => {
                format!("{{\"history_table\":\"{table}\",\"version\":null,\"applied_count\":0}}")
            }
        })),
        StatusFormat::Manifest => {
            let manifest = crate::manifest::manifest_sha256(&inv.migrations_dir).map_err(|e| {
                Outcome::Failed(
                    MigrateExit::EnvSelfCheckFailed,
                    format!("环境自检项 migrations-dir-readable 不通过：{e}"),
                )
            })?;
            Ok(Outcome::Done(format!(
                "制品清单：\n迁移目录：{}\n清单 sha256：{manifest}\n历史表：{table}\n单一版本：{}",
                inv.migrations_dir.display(),
                max.map(|v| v.to_string()).unwrap_or_else(|| "空库".into())
            )))
        }
        StatusFormat::Text => Ok(Outcome::Done(match max {
            Some(v) => format!("{table} 单一版本：{v}（已应用 {count} 个迁移）"),
            None => format!("{table} 单一版本：空库，尚无已应用迁移"),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn file(version: i64, name: &str, concurrent: bool) -> MigrationFile {
        MigrationFile {
            version,
            name: name.to_string(),
            path: PathBuf::from(format!("/nonexistent/V{version}__{name}.sql")),
            concurrent,
        }
    }

    fn applied(version: i64, name: &str, checksum: &str) -> AppliedRow {
        AppliedRow {
            version,
            name: name.to_string(),
            checksum: checksum.to_string(),
        }
    }

    /// 计划里 plan_pending 读文件算校验和；无活库测试用真实临时文件。
    fn real_file(dir: &std::path::Path, version: i64, name: &str, sql: &str) -> MigrationFile {
        let schema_dir = dir.join("s");
        std::fs::create_dir_all(&schema_dir).unwrap();
        let path = schema_dir.join(format!("V{version}__{name}.sql"));
        std::fs::write(&path, sql).unwrap();
        MigrationFile {
            version,
            name: name.to_string(),
            path,
            concurrent: false,
        }
    }

    #[test]
    fn empty_history_means_all_pending() {
        let dir = std::env::temp_dir().join(format!(
            "ep-migrate-plan-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let files = vec![
            real_file(&dir, 1, "a", "select 1;"),
            real_file(&dir, 2, "b", "select 2;"),
        ];
        let (pending, _) = plan_pending(&files, &[]).expect("可比对");
        assert_eq!(pending, vec![0, 1]);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn checksum_mismatch_is_exit_4() {
        let dir = std::env::temp_dir().join(format!(
            "ep-migrate-sum-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let files = vec![real_file(&dir, 1, "a", "select 1;")];
        let applied = vec![applied(1, "a", "12345")];
        let out = plan_pending(&files, &applied).expect_err("校验和不符必须拦");
        assert_eq!(out.exit(), MigrateExit::ChecksumMismatch);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn matching_checksum_is_skipped() {
        let dir = std::env::temp_dir().join(format!(
            "ep-migrate-ok-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let sql = "select 1;";
        let files = vec![real_file(&dir, 1, "a", sql)];
        let sum = migration_checksum("a", 1, sql).to_string();
        let applied = vec![applied(1, "a", &sum)];
        let (pending, _) = plan_pending(&files, &applied).expect("已应用应跳过");
        assert!(pending.is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn out_of_order_file_is_exit_5() {
        let dir = std::env::temp_dir().join(format!(
            "ep-migrate-oo-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let files = vec![
            real_file(&dir, 1, "a", "select 1;"),
            real_file(&dir, 3, "c", "select 3;"),
        ];
        let sum3 = migration_checksum("c", 3, "select 3;").to_string();
        // 库里已应用到版本 5，目录里还有未应用的版本 1 与 3 → 全序漂移。
        let applied = vec![applied(3, "c", &sum3), applied(5, "e", "x")];
        let mut files2 = files.clone();
        files2.push(file(5, "e", false));
        // 版本 5 无对应文件 → 也是漂移；两路都落 5。
        let out = plan_pending(&files2, &applied).expect_err("漂移必须拦");
        assert_eq!(out.exit(), MigrateExit::VersionMismatch);
        std::fs::remove_dir_all(&dir).ok();
    }
}
