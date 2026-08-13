//! `gen-rls`：按第 3.6 节模板生成行级安全策略语句，不连库。
//!
//! 全库的行级安全策略只经 `platform_core.apply_le_rls` 一份模板生成
//! （2 号迁移 V20260901090500 定义）。本命令把该模板对 (schema, table)
//! 的展开逐字印出，供迁移作者核对与评审，不在生成过程中触达数据库。

/// 模板来源的迁移文件名（唯一出处，供生成文本自注）。
pub const TEMPLATE_SOURCE: &str =
    "db/migrations/platform_core/V20260901090500__platform_core_conventions.sql";

/// 会话变量名：策略 qual 与 with check 都读它（模板逐字）。
pub const SESSION_VAR: &str = "app.legal_entity_id";

/// 生成策略语句文本。schema 与 table 已经 CLI 标识符闸校验为小写无引号形态。
pub fn render(schema: &str, table: &str) -> String {
    let policy = format!("rls_{table}_le");
    let qual =
        format!("(legal_entity_id = nullif(current_setting('{SESSION_VAR}', true), '')::uuid)");
    format!(
        "-- ep-migrate gen-rls 生成：{schema}.{table} 的行级安全策略语句。\n\
         -- 模板唯一出处：platform_core.apply_le_rls（{TEMPLATE_SOURCE}）。\n\
         -- 迁移中不得手写策略；库侧执行路径是调用模板函数：\n\
         --   select platform_core.apply_le_rls('{schema}', '{table}');\n\
         -- 以下为模板展开的等价语句，供核对：\n\
         alter table {schema}.{table} enable row level security;\n\
         alter table {schema}.{table} force row level security;\n\
         drop policy if exists {policy} on {schema}.{table};\n\
         create policy {policy} on {schema}.{table}\n\
         \x20 using {qual}\n\
         \x20 with check {qual};\n"
    )
}

/// gen-rls 主流程：有 --out 写文件，否则进报告正文（由 main 印到 stdout）。
pub fn run(schema: &str, table: &str, out: Option<&std::path::Path>) -> crate::exit::Outcome {
    let text = render(schema, table);
    match out {
        Some(path) => match std::fs::write(path, &text) {
            Ok(()) => crate::exit::Outcome::Done(format!(
                "策略语句已写入 {}（{schema}.{table}）。",
                path.display()
            )),
            Err(e) => crate::exit::Outcome::Failed(
                crate::exit::MigrateExit::EnvSelfCheckFailed,
                format!(
                    "环境自检项 out-writable 不通过：无法写入 {}：{e}",
                    path.display()
                ),
            ),
        },
        None => crate::exit::Outcome::Done(text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_matches_template_semantics() {
        let text = render("mdm", "parties");
        assert!(text.contains("alter table mdm.parties enable row level security;"));
        assert!(text.contains("alter table mdm.parties force row level security;"));
        assert!(text.contains("drop policy if exists rls_parties_le on mdm.parties;"));
        assert!(text.contains("create policy rls_parties_le on mdm.parties"));
        assert!(text.contains("current_setting('app.legal_entity_id', true)"));
        assert!(
            text.contains("using") && text.contains("with check"),
            "qual 与 with check 都必须出现"
        );
        assert!(text.contains("apply_le_rls"), "必须自注模板出处");
    }

    #[test]
    fn run_without_out_returns_text() {
        match run("mdm", "parties", None) {
            crate::exit::Outcome::Done(text) => assert!(text.contains("create policy")),
            _ => panic!("无 --out 时应直接返回正文"),
        }
    }

    #[test]
    fn run_with_out_writes_file() {
        let path = std::env::temp_dir().join(format!(
            "ep-migrate-genrls-{}-{}.sql",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default()
        ));
        let out = run("crm", "leads", Some(&path));
        assert_eq!(out.exit(), crate::exit::MigrateExit::Success);
        let written = std::fs::read_to_string(&path).expect("文件已写");
        assert!(written.contains("rls_leads_le"));
        std::fs::remove_file(&path).ok();
    }
}
