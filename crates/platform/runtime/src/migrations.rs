//! 二进制内嵌的迁移文件清单（A-08 与 `migration-version-matched` 的运行期入口）。
//!
//! 清单由本 crate 的 build.rs 在构建时从 `db/migrations/` 扫描生成并经
//! `EP_MIGRATION_FILE_LIST` 注入：逐条 `schema\u{1F}version\u{1F}name\u{1F}T|C`，
//! 条间以 `\u{1E}` 分隔，按版本号升序。运行期两处使用：
//! 一、SQL 探针据此为历史表行回填 schema 归属（历史表本身无 schema 列）；
//! 二、A-08 端点据此推导每条记录的执行路径与 `expected_version_by_binary`。

use std::sync::OnceLock;

/// 内嵌清单的一条：与 build.rs 的输出字段一一对应。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EmbeddedMigration {
    pub schema: String,
    pub version: u64,
    pub name: String,
    /// 是否位于 `concurrent/` 子目录：true 即非事务执行路径。
    pub concurrent: bool,
}

static LIST: OnceLock<Vec<EmbeddedMigration>> = OnceLock::new();

/// 内嵌迁移文件清单，按版本号升序。构建时目录缺席则为空切片。
pub fn embedded_migrations() -> &'static [EmbeddedMigration] {
    LIST.get_or_init(|| parse(env!("EP_MIGRATION_FILE_LIST")))
}

/// 二进制的期望版本号：内嵌清单的最大版本号。清单为空返回 None。
pub fn expected_version_by_binary() -> Option<u64> {
    embedded_migrations().iter().map(|e| e.version).max()
}

/// 按版本号与名称回填 schema 归属。历史表无 schema 列，取值只能来自
/// 构建时的目录结构；清单之外的行返回 None，由调用方如实上报不一致。
pub fn schema_of(version: u64, name: &str) -> Option<&'static str> {
    embedded_migrations()
        .iter()
        .find(|e| e.version == version && e.name == name)
        .map(|e| e.schema.as_str())
}

fn parse(raw: &str) -> Vec<EmbeddedMigration> {
    if raw.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for record in raw.split('\u{1E}') {
        let mut fields = record.split('\u{1F}');
        let (Some(schema), Some(version), Some(name), Some(via)) =
            (fields.next(), fields.next(), fields.next(), fields.next())
        else {
            // 清单形态由本 crate 的 build.rs 独家生成，字段缺失即构建缺陷。
            panic!("EP_MIGRATION_FILE_LIST 形态损坏：{record}");
        };
        out.push(EmbeddedMigration {
            schema: schema.to_string(),
            version: version.parse().expect("清单中的版本号必须是十进制整数"),
            name: name.to_string(),
            concurrent: via == "C",
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage2_migrations_are_embedded_and_ordered() {
        let list = embedded_migrations();
        assert!(!list.is_empty(), "阶段 2 已交付迁移文件，内嵌清单不得为空");
        for pair in list.windows(2) {
            assert!(
                pair[0].version < pair[1].version,
                "清单必须按版本号严格升序且全局唯一"
            );
        }
        assert_eq!(
            expected_version_by_binary(),
            Some(list.last().expect("非空").version)
        );
    }

    #[test]
    fn schema_of_finds_embedded_entries_and_rejects_others() {
        let first = &embedded_migrations()[0];
        assert_eq!(
            schema_of(first.version, &first.name),
            Some(first.schema.as_str())
        );
        assert_eq!(schema_of(first.version, "不存在的名称"), None);
        assert_eq!(schema_of(0, &first.name), None);
    }

    #[test]
    fn parse_round_trips_the_record_form() {
        let parsed = parse("platform_core\u{1F}10\u{1F}a\u{1F}T\u{1E}mdm\u{1F}20\u{1F}b\u{1F}C");
        assert_eq!(parsed.len(), 2);
        assert!(!parsed[0].concurrent);
        assert!(parsed[1].concurrent);
        assert!(parse("").is_empty(), "空清单对应构建时目录缺席");
    }

    #[test]
    #[should_panic(expected = "形态损坏")]
    fn malformed_record_is_a_build_defect_not_silenced() {
        parse("platform_core\u{1F}10");
    }
}
