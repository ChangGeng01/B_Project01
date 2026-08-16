//! 自定义对象的落点、字段类型基线与索引基线。
//!
//! 规格第 7.4 章的两条硬边界：
//! 一、**自定义结构不直接修改核心业务表**；
//! 二、自定义对象与自定义字段采用**真实表加在线 DDL**，每个自定义对象对应独立物理表、
//! 自定义字段为该表的真实列；**不使用 EAV**，JSON 列只承载不需要索引与校验的附加属性。
//!
//! 第一条是本模块存在的主要理由。它写在规格里是一句话，
//! 落到代码里必须是一道**拿不到就过不去**的闸——否则日后某个 applier
//! 图省事往核心表上加一列，评审看不出来，而它会污染一张有 RLS 策略、
//! 有仅追加约束、有勾稽依赖的业务表。
//!
//! 落点由技术基线第 3.1 节定死：**低代码自定义对象的物理表一律建在 `ext`，
//! 不得建到业务 schema**；阶段 13 计划的 `platform_meta.ddl_plans.target_schema`
//! 逐字「固定 `'ext'`」。注意 `platform_meta` 装的是**元数据表**
//! （`custom_objects`、`custom_fields` 等），不是自定义对象的物理表，两者不是一处。

/// 全库 24 个 schema，逐名登记。取自技术基线第 3.1 节。
///
/// **逐名而不是按前缀判**：基线第 1.3 节禁止项第七条给 `archcheck` 的
/// db-pg-one-schema-per-file 规则定的就是
/// 「按第 3.1 节登记的 24 个 schema 名逐名判定，**不用前缀启发式**」。
/// 本模块沿用同一条纪律——前缀判会把 `platform_meta` 和 `platform_core` 归成一类，
/// 而这两者在本模块里的地位正好相反。
pub const ALL_SCHEMAS: [&str; 24] = [
    // 平台侧八个
    "platform_core",
    "platform_authz",
    "platform_meta",
    "platform_flow",
    "platform_audit",
    "platform_msg",
    "platform_file",
    "platform_ops",
    // 业务侧十五个
    "mdm",
    "crm",
    "cpq",
    "clm",
    "sales",
    "procure",
    "inventory",
    "costing",
    "project",
    "service",
    "finance",
    "ledger",
    "invoice",
    "portal",
    "reporting",
    // 低代码扩展一个
    "ext",
];

/// 自定义对象物理表的唯一落点。基线第 3.1 节与阶段 13 计划第 3.2 节都是这个取值。
pub const CUSTOM_OBJECT_SCHEMA: &str = "ext";

/// 已校验的落点。
///
/// 只有一个取值，且这不是「暂时只有一个」。做成类型而不是字符串，
/// 是为了让调用方**拿不到它就拼不出建表语句**。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TargetSchema;

impl TargetSchema {
    pub fn as_db_value(self) -> &'static str {
        CUSTOM_OBJECT_SCHEMA
    }
}

/// 落点校验失败的原因。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PlacementError {
    /// 目标是已登记的另一个 schema——平台侧或业务侧。
    RegisteredButNotExt { schema: String },
    /// 目标不在第 3.1 节登记的 24 个里。
    ///
    /// 与上一种**分开报**：前者是「想往业务表上加东西」，后者是
    /// 「schema 名拼错了或新建了没登记」。两者的下一步动作完全不同，
    /// 合成一条会让后一种被当成越权企图去查。
    NotRegistered { schema: String },
}

impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlacementError::RegisteredButNotExt { schema } => write!(
                f,
                "自定义对象的物理表不得建在 {schema}；规格第 7.4 章：自定义结构不直接修改核心业务表，\
                 基线第 3.1 节：一律建在 {CUSTOM_OBJECT_SCHEMA}"
            ),
            PlacementError::NotRegistered { schema } => write!(
                f,
                "{schema} 不在技术基线第 3.1 节登记的 24 个 schema 里；\
                 新增 schema 须先改基线第 3.1 节"
            ),
        }
    }
}

impl std::error::Error for PlacementError {}

/// 校验一次自定义对象的落点。
pub fn validate_placement(target_schema: &str) -> Result<TargetSchema, PlacementError> {
    if target_schema == CUSTOM_OBJECT_SCHEMA {
        return Ok(TargetSchema);
    }
    if ALL_SCHEMAS.contains(&target_schema) {
        return Err(PlacementError::RegisteredButNotExt {
            schema: target_schema.to_string(),
        });
    }
    Err(PlacementError::NotRegistered {
        schema: target_schema.to_string(),
    })
}

/// 公共能力基线内的字段类型。规格第 7.4 章逐字**十一种**。
///
/// 「首版在 PostgreSQL 16 上实现与认证」——本清单是**能力基线**不是 PostgreSQL 的类型全集，
/// 多出来的类型即便 PostgreSQL 支持也不在基线内，因为基线要能被别的数据库实现。
/// 规格同句收尾：**超出基线的类型与索引在发布前的影响分析中拒绝**。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldType {
    Integer,
    Decimal,
    Float,
    Boolean,
    String,
    Text,
    Date,
    Timestamp,
    Enum,
    Reference,
    /// JSON。**十一种里唯一不可索引不可校验的一种**，见 [`FieldType::indexable`]。
    Json,
}

impl FieldType {
    pub fn as_db_value(self) -> &'static str {
        match self {
            FieldType::Integer => "INTEGER",
            FieldType::Decimal => "DECIMAL",
            FieldType::Float => "FLOAT",
            FieldType::Boolean => "BOOLEAN",
            FieldType::String => "STRING",
            FieldType::Text => "TEXT",
            FieldType::Date => "DATE",
            FieldType::Timestamp => "TIMESTAMP",
            FieldType::Enum => "ENUM",
            FieldType::Reference => "REFERENCE",
            FieldType::Json => "JSON",
        }
    }

    pub const ALL: [FieldType; 11] = [
        FieldType::Integer,
        FieldType::Decimal,
        FieldType::Float,
        FieldType::Boolean,
        FieldType::String,
        FieldType::Text,
        FieldType::Date,
        FieldType::Timestamp,
        FieldType::Enum,
        FieldType::Reference,
        FieldType::Json,
    ];

    /// 该类型的列能否建索引与校验。
    ///
    /// 规格第 7.4 章两句话夹出同一个结论：
    /// 「**JSON 列只承载不需要索引与校验的附加属性**」，
    /// 以及索引基线「不使用函数索引、局部索引和 **JSON 路径索引**」。
    ///
    /// 这个方法**不是恒真的**——`Json` 返回 `false`。写下这一点是因为
    /// 一个恒真的判据等于没有判据：调用方照样会调它，门禁照样显示绿，
    /// 而第一个往 JSON 列上加唯一索引的自定义字段会在建表时才炸，
    /// 或者更糟，被 PostgreSQL 用表达式索引接住而绕过基线。
    pub fn indexable(self) -> bool {
        !matches!(self, FieldType::Json)
    }
}

/// 公共能力基线内的索引形态。规格第 7.4 章逐字三种。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexKind {
    /// 单列索引。
    SingleColumn,
    /// 复合索引。
    Composite,
    /// 唯一索引。
    Unique,
}

impl IndexKind {
    pub fn as_db_value(self) -> &'static str {
        match self {
            IndexKind::SingleColumn => "SINGLE_COLUMN",
            IndexKind::Composite => "COMPOSITE",
            IndexKind::Unique => "UNIQUE",
        }
    }

    pub const ALL: [IndexKind; 3] = [
        IndexKind::SingleColumn,
        IndexKind::Composite,
        IndexKind::Unique,
    ];
}

/// 规格逐字点名**不使用**的三种索引。
///
/// 单列出来是为了让「被拒的到底是哪三种」可断言。三种都是 PostgreSQL 支持、
/// 别的数据库未必支持的形态——基线拒它们不是因为它们不好，
/// 是因为基线要能被别的数据库实现。
pub const FORBIDDEN_INDEX_FORMS: [&str; 3] = ["FUNCTIONAL", "PARTIAL", "JSON_PATH"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twenty_four_schemas_are_registered() {
        assert_eq!(ALL_SCHEMAS.len(), 24, "改这张表必须先改技术基线第 3.1 节");
        let mut sorted = ALL_SCHEMAS;
        sorted.sort_unstable();
        let mut deduped = sorted.to_vec();
        deduped.dedup();
        assert_eq!(deduped.len(), 24, "登记表里有重复名");
    }

    #[test]
    fn custom_objects_land_in_ext() {
        assert_eq!(validate_placement("ext"), Ok(TargetSchema));
        assert_eq!(TargetSchema.as_db_value(), "ext");
    }

    /// 本模块的要害：`ext` 之外的 23 个 schema 一个都不许。
    /// 规格里这是一句话，落到代码里必须是一道拿不到就过不去的闸——
    /// 否则日后某个 applier 图省事往核心表加一列，评审看不出来，
    /// 而它污染的是一张有 RLS 策略、有仅追加约束、有勾稽依赖的业务表。
    #[test]
    fn every_other_registered_schema_is_refused() {
        let refused: Vec<&str> = ALL_SCHEMAS
            .iter()
            .copied()
            .filter(|s| validate_placement(s).is_err())
            .collect();
        assert_eq!(refused.len(), 23, "24 个里应恰好拒 23 个，实拒 {refused:?}");
        for s in refused {
            assert!(matches!(
                validate_placement(s),
                Err(PlacementError::RegisteredButNotExt { .. })
            ));
        }
    }

    /// `platform_meta` 装的是元数据表，不是自定义对象的物理表——
    /// 这一条最容易写错，因为本 crate 自己就叫 meta。
    #[test]
    fn platform_meta_is_not_where_physical_tables_go() {
        assert!(matches!(
            validate_placement("platform_meta"),
            Err(PlacementError::RegisteredButNotExt { .. })
        ));
    }

    /// 未登记的 schema 与「登记了但不是 ext」分开报：
    /// 前者是名字拼错或新建没登记，后者是想往业务表上加东西，下一步动作不同。
    #[test]
    fn unregistered_schemas_get_their_own_reason() {
        for s in ["public", "pg_catalog", "ext2", "随便什么"] {
            assert!(
                matches!(
                    validate_placement(s),
                    Err(PlacementError::NotRegistered { .. })
                ),
                "{s} 未登记，应报 NotRegistered"
            );
        }
    }

    #[test]
    fn error_messages_cite_the_rule() {
        let msg = validate_placement("finance")
            .expect_err("应拒绝")
            .to_string();
        assert!(
            msg.contains("不直接修改核心业务表") && msg.contains("ext"),
            "错误文案要点出规则出处与正确落点，实为 {msg}"
        );
    }

    #[test]
    fn field_types_are_the_eleven_named_in_the_spec() {
        assert_eq!(FieldType::ALL.len(), 11, "改这张表必须先改规格第 7.4 章");
    }

    /// `indexable` 不是恒真的：JSON 是十一种里唯一不可索引的。
    /// 一个恒真的判据等于没有判据——调用方照样调它、门禁照样绿，
    /// 而第一个往 JSON 列上加唯一索引的自定义字段要么在建表时才炸，
    /// 要么被 PostgreSQL 用表达式索引接住，绕过基线。
    #[test]
    fn json_is_the_one_field_type_that_cannot_be_indexed() {
        let not_indexable: Vec<FieldType> = FieldType::ALL
            .iter()
            .copied()
            .filter(|t| !t.indexable())
            .collect();
        assert_eq!(
            not_indexable,
            vec![FieldType::Json],
            "规格第 7.4 章：JSON 列只承载不需要索引与校验的附加属性"
        );
    }

    #[test]
    fn index_kinds_are_the_three_named_in_the_spec() {
        assert_eq!(IndexKind::ALL.len(), 3, "改这张表必须先改规格第 7.4 章");
        assert_eq!(FORBIDDEN_INDEX_FORMS.len(), 3);
        for forbidden in FORBIDDEN_INDEX_FORMS {
            assert!(
                !IndexKind::ALL.iter().any(|k| k.as_db_value() == forbidden),
                "{forbidden} 被规格点名不使用，不得出现在基线内"
            );
        }
    }

    #[test]
    fn db_values() {
        assert_eq!(FieldType::Reference.as_db_value(), "REFERENCE");
        assert_eq!(FieldType::Json.as_db_value(), "JSON");
        assert_eq!(IndexKind::Unique.as_db_value(), "UNIQUE");
    }
}
