//! 数据库连接串解析与连接建立。
//!
//! ## 连接串入口口径对齐（规格报告第 8 项）
//!
//! 本工具阶段 1 冻结的入口是 `--db-url` / `EP__DB__URL`（直连串）；计划 §7 的
//! 配置键是 `EP__DB__DSN`，且只写机密引用（`secret://...`）。两处的对齐方式：
//! 1. 优先级 `--db-url` > `EP__DB__URL` > `EP__DB__DSN`；
//! 2. `EP__DB__DSN` 的 `secret://` 引用语义由运行时配置层（ep-platform-runtime）
//!    解析后把真实连接串注入 `EP__DB__URL`；本工具是一次性运维二进制，不链接
//!    机密库，遇到尚未解析的 `secret://` 引用一律落 78 并写明该口径；
//! 3. `EP__DB__DSN` 若直接给出 postgres:// 直连串（非引用形态），同样受理。

use tokio_postgres::{Client, NoTls};

use crate::cli::{Invocation, DB_DSN_ENV, DB_URL_ENV};
use crate::exit::{MigrateExit, Outcome};

/// 迁移会话固定两条设置（计划 §3.3 逐字）：锁等待 5 秒、单语句 30 分钟。
pub const SESSION_PREAMBLE: [&str; 2] =
    ["SET lock_timeout = '5s'", "SET statement_timeout = '30min'"];

/// 解析连接串：命令行优先，其次 `EP__DB__URL`，最后 `EP__DB__DSN`。
/// `secret://` 引用形态落 78——口径见模块头。
pub fn resolve_db_url(
    inv: &Invocation,
    env_db_url: Option<&str>,
    env_db_dsn: Option<&str>,
) -> Result<String, Outcome> {
    let picked = inv
        .db_url
        .clone()
        .or_else(|| env_db_url.map(|s| s.to_string()))
        .or_else(|| env_db_dsn.map(|s| s.to_string()));
    match picked {
        None => Err(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 db-url-resolvable 不通过：子命令 {} 需要数据库连接串，\
                 命令行未给 --db-url，环境变量 {DB_URL_ENV} 与 {DB_DSN_ENV} 也都未设置。",
                inv.sub.name()
            ),
        )),
        Some(url) if url.starts_with("secret://") => Err(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!(
                "环境自检项 db-url-resolvable 不通过：连接串入口给的是机密引用 {url}。\
                 secret:// 引用由运行时配置层解析后注入 {DB_URL_ENV}；\
                 本工具是一次性运维二进制，不链接机密库，请出示直连串。"
            ),
        )),
        Some(url) => Ok(url),
    }
}

/// 建立连接。连不上落 78（本机器不具备执行前提），并把底层错误原样带出。
pub async fn connect(url: &str) -> Result<Client, Outcome> {
    match tokio_postgres::connect(url, NoTls).await {
        Ok((client, connection)) => {
            tokio::spawn(async move {
                // 连接的 IO 循环必须被驱动；一次性命令收尾时随进程退出。
                if let Err(e) = connection.await {
                    eprintln!("数据库连接通道异常：{e}");
                }
            });
            Ok(client)
        }
        Err(e) => Err(Outcome::Failed(
            MigrateExit::EnvSelfCheckFailed,
            format!("环境自检项 db-reachable 不通过：无法建立数据库连接：{e}"),
        )),
    }
}

/// 给一个会话固定迁移会话两条设置。
pub async fn apply_session_preamble(client: &Client) -> Result<(), Outcome> {
    for stmt in SESSION_PREAMBLE {
        client.batch_execute(stmt).await.map_err(|e| {
            Outcome::Failed(
                MigrateExit::EnvSelfCheckFailed,
                format!("环境自检项 db-reachable 不通过：会话设置失败：{e}"),
            )
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{parse, Parsed};

    fn inv(argv: &[&str]) -> Invocation {
        let args: Vec<String> = argv.iter().map(|s| s.to_string()).collect();
        match parse(&args).expect("这组参数应当可解析") {
            Parsed::Run(i) => *i,
            Parsed::Print(_) => panic!("这组参数不该走用法分支"),
        }
    }

    #[test]
    fn cli_beats_env_url_beats_dsn() {
        let i = inv(&["status", "--db-url=postgres://cli/ep"]);
        let got = resolve_db_url(&i, Some("postgres://env/ep"), Some("postgres://dsn/ep"))
            .expect("可解析");
        assert_eq!(got, "postgres://cli/ep");

        let i = inv(&["status"]);
        let got = resolve_db_url(&i, Some("postgres://env/ep"), Some("postgres://dsn/ep"))
            .expect("可解析");
        assert_eq!(got, "postgres://env/ep");

        let got = resolve_db_url(&i, None, Some("postgres://dsn/ep")).expect("可解析");
        assert_eq!(got, "postgres://dsn/ep");
    }

    #[test]
    fn secret_reference_is_env_selfcheck_failure() {
        let i = inv(&["status"]);
        let out = resolve_db_url(&i, None, Some("secret://db/app_rw#1")).expect_err("引用须拒");
        assert_eq!(out.exit(), MigrateExit::EnvSelfCheckFailed);
        match out {
            Outcome::Failed(_, msg) => assert!(msg.contains("secret://"), "{msg}"),
            Outcome::Done(_) => panic!(),
        }
    }

    #[test]
    fn nothing_available_is_env_selfcheck_failure() {
        let i = inv(&["status"]);
        let out = resolve_db_url(&i, None, None).expect_err("全缺须拒");
        assert_eq!(out.exit(), MigrateExit::EnvSelfCheckFailed);
    }

    #[test]
    fn session_preamble_is_plan_verbatim() {
        assert_eq!(SESSION_PREAMBLE[0], "SET lock_timeout = '5s'");
        assert_eq!(SESSION_PREAMBLE[1], "SET statement_timeout = '30min'");
    }
}
