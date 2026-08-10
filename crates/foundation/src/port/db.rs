//! 数据库能力端口。
//!
//! 阶段 1 只建空文件。按裁定 F-01，原 ep-adapter-db 承载的端口 trait 与
//! 能力描述下沉本模块，与 `port::tx`、`port::search`、`port::doc` 三个端口模块并列：
//! `IdempotencyStore` 与 `IdempotencyScope`、`IdempotencyOutcome` 按 C-07 由阶段 2 补齐，
//! `MigrationWindowGuard` 按 B-03 由阶段 2 补齐，只读事务端口 `ReadOnlyTx` 由阶段 11 补齐，
//! 规格第 7.4 章公共能力基线的字段类型与索引种类的能力描述由阶段 2 补齐。
//!
//! 本模块只放端口与能力描述。具体类型、取值与 SQL 侧映射一律落在 ep-adapter-db-pg，
//! 本模块不声明任何 `Pg` 前缀的类型。
