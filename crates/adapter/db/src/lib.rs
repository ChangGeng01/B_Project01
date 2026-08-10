//! ep-adapter-db — 数据库适配抽象层。
//!
//! 职责：连接池抽象、公共能力基线的类型与索引映射，不含任何 PostgreSQL 专有语法。
//!
//! 待裁定（阶段 1 实测）：技术基线第 1.4 节配套纪律第四条要求
//! `PgTx` 与 `PgUnitOfWork` 的「声明位在本 crate、实现落在 ep-adapter-db-pg」。
//! 该写法在 Rust 中不成立，见 `crates/adapter/db-pg/src/tx.rs` 的说明。
//! 本 crate 暂不声明这两个类型。
