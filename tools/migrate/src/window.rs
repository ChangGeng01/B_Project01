//! 迁移窗口闸与开窗（判据与计划 §5 的 A-09 端点一致）。
//!
//! - apply 前置闸：对 `migration_window_lock` 单例行取 `SELECT ... FOR UPDATE`
//!   串行化（基线第 3.10 节禁部分唯一索引，窗口唯一性靠行锁），
//!   再查 `migration_windows` 是否有一个未过期的 `OPEN` 窗口且其 id 等于
//!   调用方出示的 `--window-id`。任一不满足即退出码 3，
//!   错误码 `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`（HTTP 409，BUSINESS_CONFLICT）。
//! - open-window：请求要素 approval_ref（双人审批，缺失不可开窗）、reason、
//!   ttl_minutes（默认 60、上限 240，CLI 闸已冻）。CLI 选项白名单阶段 1 冻结，
//!   不含 approval-ref 选项，故 approval_ref 由环境变量
//!   `EP__DB__MIGRATION__APPROVAL_REF` 出示，缺失落 78。
//!   已有 OPEN 窗口时拒绝（对应端点 409 语义），过期的 OPEN 窗口先按
//!   EXPIRED 关闭再开新窗。开窗写审计由审计阶段在端点侧提供，本工具只写窗口行。

use tokio_postgres::Client;

use crate::cli::APPROVAL_REF_ENV;
use crate::exit::{MigrateExit, Outcome};

/// 系统主体：开窗行的 opened_by 一律取 ep_foundation::SYSTEM_PRINCIPAL_ID（A-02）。
/// 首装自举窗口行（见 bootstrap.rs）同取此值。
pub(crate) const SYSTEM_PRINCIPAL: &str = "00000000-0000-7000-8000-000000000001";

const LOCK_ROW_STMT: &str =
    "SELECT id FROM platform_core.migration_window_lock WHERE id = 1 FOR UPDATE";

const OPEN_WINDOWS_STMT: &str = "SELECT id::text, (expires_at > now()) AS live \
     FROM platform_core.migration_windows WHERE state = 'OPEN'";

fn window_closed(detail: &str) -> Outcome {
    Outcome::Failed(
        MigrateExit::MigrationWindowClosed,
        format!(
            "迁移窗口未打开（PLATFORM.DB.MIGRATION_WINDOW_CLOSED）：{detail}\n\
             窗口由 ep-migrate open-window 开启，登记在 platform_core.migration_windows。"
        ),
    )
}

fn db_failure(detail: String) -> Outcome {
    Outcome::Failed(
        MigrateExit::EnvSelfCheckFailed,
        format!("环境自检项 db-reachable 不通过：{detail}"),
    )
}

/// 窗口闸的纯判定：给定 (id, live) 行集合与出示的窗口 id，给出结论。
/// 抽成纯函数是为了无活库也能测试判定逻辑。
pub fn judge_open(windows: &[(String, bool)], presented: &str) -> Result<(), String> {
    let live: Vec<&str> = windows
        .iter()
        .filter(|(_, live)| *live)
        .map(|(id, _)| id.as_str())
        .collect();
    match live.len() {
        0 => Err("库中没有未过期的 OPEN 窗口".to_string()),
        1 if live[0] == presented => Ok(()),
        1 => Err(format!(
            "库中 OPEN 窗口是 {}，与出示的 {presented} 不符",
            live[0]
        )),
        n => Err(format!("库中同时存在 {n} 个未过期 OPEN 窗口，数据已损坏")),
    }
}

/// apply 前置窗口闸。成功返回窗口 id（后续回写 applied_versions 用）。
pub async fn assert_open(client: &mut Client, presented: &str) -> Result<String, Outcome> {
    let tx = client
        .transaction()
        .await
        .map_err(|e| db_failure(format!("开启窗口闸事务失败：{e}")))?;
    tx.execute(LOCK_ROW_STMT, &[])
        .await
        .map_err(|e| db_failure(format!("窗口锁表不可用（迁移未应用？）：{e}")))?;
    let rows = tx
        .query(OPEN_WINDOWS_STMT, &[])
        .await
        .map_err(|e| db_failure(format!("读取迁移窗口失败：{e}")))?;
    let windows: Vec<(String, bool)> = rows
        .iter()
        .map(|r| (r.get::<_, String>(0), r.get::<_, bool>(1)))
        .collect();
    tx.commit()
        .await
        .map_err(|e| db_failure(format!("窗口闸事务收尾失败：{e}")))?;
    judge_open(&windows, presented)
        .map(|()| presented.to_string())
        .map_err(|detail| window_closed(&detail))
}

/// approval_ref 出示判定（A-09：双人审批引用缺失不可开窗）。纯函数，无活库可测。
pub fn check_approval_ref(env_approval_ref: Option<&str>) -> Result<String, String> {
    match env_approval_ref {
        Some(r) if !r.trim().is_empty() => Ok(r.trim().to_string()),
        _ => Err(format!(
            "环境自检项 approval-ref-presented 不通过：开窗需要双人审批引用，\
             请以环境变量 {APPROVAL_REF_ENV} 出示（A-09：approval_ref 缺失不可开窗）。"
        )),
    }
}

/// open-window：与 A-09 端点判据一致。成功返回报告文本（含窗口 id 与 expires_at）。
pub async fn open_window(
    client: &mut Client,
    ttl_minutes: u32,
    reason: &str,
    env_approval_ref: Option<&str>,
) -> Result<String, Outcome> {
    let approval_ref = check_approval_ref(env_approval_ref)
        .map_err(|msg| Outcome::Failed(MigrateExit::EnvSelfCheckFailed, msg))?;

    let tx = client
        .transaction()
        .await
        .map_err(|e| db_failure(format!("开启开窗事务失败：{e}")))?;
    tx.execute(LOCK_ROW_STMT, &[])
        .await
        .map_err(|e| db_failure(format!("窗口锁表不可用（迁移未应用？）：{e}")))?;

    // 过期的 OPEN 窗口先按 EXPIRED 关闭（row_version 递增是乐观锁守卫要求）。
    // UUID 参数以文本绑定并经 ::text::uuid 两步显式转型：tokio_postgres 无法把
    // &str 直接序列化为 uuid 类型参数，单步 ::uuid 会在客户端侧报序列化失败。
    tx.execute(
        "UPDATE platform_core.migration_windows SET state = 'CLOSED', closed_at = now(), \
         closed_by = $1::text::uuid, close_kind = 'EXPIRED', row_version = row_version + 1, \
         updated_at = now(), updated_by = $1::text::uuid \
         WHERE state = 'OPEN' AND expires_at <= now()",
        &[&SYSTEM_PRINCIPAL],
    )
    .await
    .map_err(|e| db_failure(format!("关闭过期窗口失败：{e}")))?;

    let still_open: i64 = tx
        .query_one(
            "SELECT count(*) FROM platform_core.migration_windows WHERE state = 'OPEN'",
            &[],
        )
        .await
        .map_err(|e| db_failure(format!("清点 OPEN 窗口失败：{e}")))?
        .get(0);
    if still_open > 0 {
        tx.rollback()
            .await
            .map_err(|e| db_failure(format!("回滚开窗事务失败：{e}")))?;
        return Err(window_closed(
            "已有 OPEN 窗口，开窗请求被拒（A-09 的 409 语义）",
        ));
    }

    let window_id = uuid::Uuid::now_v7();
    let row = tx
        .query_one(
            "INSERT INTO platform_core.migration_windows \
             (id, state, approval_ref, reason, opened_by, opened_at, expires_at) \
             VALUES ($1::text::uuid, 'OPEN', $2, $3, $4::text::uuid, now(), now() + make_interval(mins => $5)) \
             RETURNING id::text, expires_at::text",
            &[
                &window_id.to_string(),
                &approval_ref,
                &reason,
                &SYSTEM_PRINCIPAL,
                &(ttl_minutes as i32),
            ],
        )
        .await
        .map_err(|e| db_failure(format!("写入迁移窗口失败：{e}")))?;
    tx.commit()
        .await
        .map_err(|e| db_failure(format!("提交开窗事务失败：{e}")))?;

    Ok(format!(
        "迁移窗口已开启。\n窗口 id：{}\n到期时刻：{}\nttl：{ttl_minutes} 分钟\n审批引用：{approval_ref}",
        row.get::<_, String>(0),
        row.get::<_, String>(1)
    ))
}

/// 把本次 apply 应用过的版本号回写到窗口行的 applied_versions。
pub async fn append_applied_version(
    client: &Client,
    window_id: &str,
    version: i64,
) -> Result<(), Outcome> {
    let id: uuid::Uuid = window_id
        .parse()
        .map_err(|_| window_closed(&format!("窗口 id {window_id} 不是 UUID 形态")))?;
    client
        .execute(
            "UPDATE platform_core.migration_windows \
             SET applied_versions = array_append(applied_versions, $1), \
                 row_version = row_version + 1, updated_at = now(), updated_by = $2::text::uuid \
             WHERE id = $3::text::uuid AND state = 'OPEN'",
            &[&version.to_string(), &SYSTEM_PRINCIPAL, &id.to_string()],
        )
        .await
        .map_err(|e| db_failure(format!("回写 applied_versions 失败：{e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_requires_live_window_matching_presented_id() {
        assert!(judge_open(&[("w-1".into(), true)], "w-1").is_ok());
        assert!(judge_open(&[], "w-1").is_err(), "无窗口必拒");
        assert!(
            judge_open(&[("w-2".into(), true)], "w-1").is_err(),
            "不符必拒"
        );
        assert!(
            judge_open(&[("w-1".into(), false)], "w-1").is_err(),
            "过期窗口不算 OPEN"
        );
        assert!(
            judge_open(&[("a".into(), true), ("b".into(), true)], "a").is_err(),
            "两个未过期 OPEN 是数据损坏"
        );
    }

    #[test]
    fn approval_ref_missing_or_blank_is_rejected() {
        assert!(check_approval_ref(None).is_err(), "缺失必拒");
        assert!(check_approval_ref(Some("")).is_err(), "空串必拒");
        assert!(check_approval_ref(Some("   ")).is_err(), "空白必拒");
        assert_eq!(
            check_approval_ref(Some(" APR-001 ")).as_deref(),
            Ok("APR-001"),
            "有效引用去首尾空白后受理"
        );
    }
}
