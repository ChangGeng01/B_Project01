//! ep-platform-meta —— 元数据、自定义对象与字段、在线 DDL 边界。
//!
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`ddl`] 在线变更边界与三项上限、[`custom`] 落点与类型索引基线、
//! [`identifier`] 对象码与派生标识符的长度约束。
//! 六个 `ConfigItemApplier`（`CUSTOM_OBJECT`、`CUSTOM_FIELD`、`CUSTOM_RELATION`、
//! `CUSTOM_INDEX`、`CUSTOM_VIEW`、`UI_LAYOUT`）按裁定归阶段 13b，本轮不做；
//! 在线 DDL 的实际执行与影响分析在适配层，本 crate 不碰数据库。
//!
//! # 三处判错方向的代价不对称，都取了保守侧
//!
//! 一、[`ddl::classify`] 用**白名单**：只有规格第 7.4 章逐字列出的四个操作判可在线，
//! **其余一律需停机窗口，包括本模块还不认识的新操作**。
//! 误判成可在线会在生产高峰锁住业务表——客户是二三十人的小公司，
//! 一张表锁五分钟就是全公司停工；反过来只是多要一个维护窗口。
//!
//! 二、[`custom::validate_placement`] 只放行 `ext` 一个落点，其余 23 个登记 schema
//! 逐名拒绝。规格第 7.4 章「自定义结构不直接修改核心业务表」是一句话，
//! 落到代码里必须是一道**拿不到就过不去的闸**——否则日后某个 applier
//! 图省事往核心表加一列，评审看不出来，而它污染的是一张有 RLS 策略、
//! 有仅追加约束、有勾稽依赖的业务表。
//!
//! 三、[`identifier::ObjectCode`] 的长度上限按**多对多派生名**反推而不是单对象，
//! 因为前者更紧（24 对 33）。取松的那个，两个自定义对象一建关系就派生出
//! 81 字节的索引名——PostgreSQL 不报错，截断到 63，元数据记的名字与库里的从此不是一个。

pub mod custom;
pub mod ddl;
pub mod identifier;

pub use custom::{
    validate_placement, FieldType, IndexKind, PlacementError, TargetSchema, ALL_SCHEMAS,
    CUSTOM_OBJECT_SCHEMA, FORBIDDEN_INDEX_FORMS,
};
pub use ddl::{
    classify, judge_online_run, DdlOperation, ExecutionMode, LimitExceeded, OnlineOutcome,
    MAX_LOCK_HOLD, MAX_MAINTENANCE_WINDOW, MAX_MIGRATION_DURATION, MINIMUM_ONLINE_CAPABILITY,
    ONLINE_OPERATIONS,
};
pub use identifier::{
    created_at_index_name, link_pair_index_name, CodeError, ObjectCode, MAX_OBJECT_CODE_LEN,
    PG_IDENTIFIER_LIMIT,
};
