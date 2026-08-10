//! 探针 schema `ci_probe` 与探针表 `ci_probe.probe_records`。
//!
//! 出处：阶段 1 计划第 4.4 节。它的唯一用途是让 `rls_matrix` 的八类越权断言在阶段 1
//! 就有被测对象。它不出现在 `db/migrations/` 下，不进任何交付制品，
//! `xtask sqlcheck` 的 SQL-030 断言 `ci_probe` 字样不出现在生产迁移目录中。
//!
//! 按裁定 B-01，本模块整体带 `#[cfg(feature = "ci-probe")]`，feature 名固定为 `ci-probe`，
//! 在 `testkit/Cargo.toml` 中声明且默认关闭；发布制品中不得出现该 feature，
//! 判据由阶段 14 的发布门禁项 `RG-CI-PROBE-ABSENT` 承担。
//!
//! 本阶段只交付 DDL 语句文本，不交付执行它的连接：工作区内尚无数据库客户端
//! （`ep-adapter-db-pg` 为空壳，引导脚本属阶段 2）。这里不提供任何「执行成功」的返回值，
//! 以免出现一条没连过库却报通过的路径——是否执行过由调用方自己判定。

/// 探针 schema 名。`rls_matrix` 与 `xtask sqlcheck` 的 SQL-030 共用这一个字面量。
pub const SCHEMA: &str = "ci_probe";

/// 探针表全名。
pub const TABLE: &str = "ci_probe.probe_records";

/// 三条基线索引名，与阶段 1 计划第 4.4 节的索引一节逐字一致。
pub const INDEX_NAMES: [&str; 3] = [
    "pk_probe_records",
    "ix_probe_records_legal_entity_id_created_at",
    "ux_probe_records_legal_entity_id_doc_no",
];

/// 三条 CHECK 约束名，同上。
pub const CHECK_NAMES: [&str; 3] = [
    "ck_probe_records_security_level",
    "ck_probe_records_status",
    "ck_probe_records_note_len",
];

const CREATE_SCHEMA: &str = "create schema if not exists ci_probe;";

const CREATE_TABLE: &str = "\
create table ci_probe.probe_records (
  id uuid not null,
  legal_entity_id uuid not null,
  security_level smallint not null default 20,
  data_scope_tags text[] not null default '{}',
  row_version bigint not null default 1,
  created_at timestamptz not null default now(),
  created_by uuid not null,
  updated_at timestamptz not null default now(),
  updated_by uuid not null,
  doc_no text not null,
  status text not null,
  note text null,
  constraint pk_probe_records primary key (id),
  constraint ck_probe_records_security_level check (security_level in (10,20,30,40)),
  constraint ck_probe_records_status check (status in ('DRAFT','EFFECTIVE','VOID')),
  constraint ck_probe_records_note_len check (note is null or length(note) <= 2000)
);";

const CREATE_INDEXES: &str = "\
create index ix_probe_records_legal_entity_id_created_at
  on ci_probe.probe_records (legal_entity_id, created_at);
create unique index ux_probe_records_legal_entity_id_doc_no
  on ci_probe.probe_records (legal_entity_id, doc_no);";

/// RLS 策略，按阶段 1 计划第 4.4 节的模板原样照抄，不写变体。
const ENABLE_RLS: &str = "\
alter table ci_probe.probe_records enable row level security;
alter table ci_probe.probe_records force row level security;
create policy rls_probe_records_le on ci_probe.probe_records
  using (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid)
  with check (legal_entity_id = nullif(current_setting('app.legal_entity_id', true), '')::uuid);";

/// 建探针 schema、表、索引与 RLS 策略的 DDL，按必须执行的先后顺序给出。
pub fn ddl_statements() -> Vec<&'static str> {
    vec![CREATE_SCHEMA, CREATE_TABLE, CREATE_INDEXES, ENABLE_RLS]
}

/// 拆库用 DDL。临时测试库用完即弃，但同一进程内反复建库时需要它。
pub fn drop_statements() -> Vec<&'static str> {
    vec!["drop schema if exists ci_probe cascade;"]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_ddl() -> String {
        ddl_statements().join("\n")
    }

    /// 九个公共列必须齐备且顺序与数据字典第 2 节一致。
    #[test]
    fn common_columns_appear_in_dictionary_order() {
        let expected = [
            "id uuid",
            "legal_entity_id uuid",
            "security_level smallint",
            "data_scope_tags text[]",
            "row_version bigint",
            "created_at timestamptz",
            "created_by uuid",
            "updated_at timestamptz",
            "updated_by uuid",
        ];
        let mut at = 0usize;
        for col in expected {
            let found = CREATE_TABLE[at..]
                .find(col)
                .unwrap_or_else(|| panic!("建表语句缺列或列序不符：{col}"));
            at += found + col.len();
        }
    }

    /// 单据类附加列 doc_no 与 status 必须在，且 status 的取值域是三态。
    #[test]
    fn document_columns_and_status_domain_are_present() {
        assert!(CREATE_TABLE.contains("doc_no text not null"));
        assert!(CREATE_TABLE.contains("status text not null"));
        assert!(CREATE_TABLE.contains("in ('DRAFT','EFFECTIVE','VOID')"));
    }

    /// 三条索引名与三条约束名必须逐条出现在 DDL 中。
    #[test]
    fn every_registered_name_appears_in_ddl() {
        let ddl = all_ddl();
        for name in INDEX_NAMES.iter().chain(CHECK_NAMES.iter()) {
            assert!(ddl.contains(name), "DDL 中缺 {name}");
        }
    }

    /// RLS 必须同时 enable 与 force：只 enable 时表属主仍绕过策略，
    /// 那会让八类越权断言在属主连接下全部平凡通过。
    #[test]
    fn rls_is_both_enabled_and_forced() {
        assert!(ENABLE_RLS.contains("enable row level security"));
        assert!(ENABLE_RLS.contains("force row level security"));
    }

    /// 策略的 using 与 with check 两侧必须都在，缺 with check 时写入不受隔离。
    #[test]
    fn policy_guards_both_read_and_write() {
        assert!(ENABLE_RLS.contains("using (legal_entity_id ="));
        assert!(ENABLE_RLS.contains("with check (legal_entity_id ="));
    }

    /// 语句顺序不可换：先建 schema 再建表，索引与 RLS 在表之后。
    #[test]
    fn statement_order_is_executable() {
        let stmts = ddl_statements();
        assert!(stmts[0].contains("create schema"));
        assert!(stmts[1].contains("create table ci_probe.probe_records"));
        assert!(stmts[2].contains("create index"));
        assert!(stmts[3].contains("row level security"));
    }

    #[test]
    fn drop_is_available_for_reruns() {
        assert!(drop_statements()[0].contains("drop schema if exists ci_probe"));
    }

    /// schema 与表名两处字面量必须同源，避免各写各的。
    #[test]
    fn table_name_is_qualified_by_schema() {
        assert_eq!(TABLE, format!("{SCHEMA}.probe_records"));
        assert!(CREATE_TABLE.contains(TABLE));
    }
}
