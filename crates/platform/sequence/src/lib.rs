//! ep-platform-sequence —— 单据编号与档案编码。
//!
//! 职责：按技术基线第 11.1 节的格式取号，格式为
//! `<类型码>-<法人码>-<YYYYMM>-<6 位流水>`，例如 `SO-01-202608-000123`。
//! 流水按法人、类型、年月三元组独立自增，位数溢出自动扩展。
//!
//! **取号在业务事务内完成，回滚即退号，因此不产生空号。** 这条是基线第 11.1 节的
//! 明确要求，也是本 crate 把数据库操作放在 [`port::NumberAllocator`] 之后、
//! 由调用方在自己的事务里驱动的原因——本 crate 自己不持有连接、不开事务。
//!
//! # 编辑本 crate 前必读：一条会伤到机检门禁的约束
//!
//! `xtask configdoc --check-doc-type-codes` 扫描 `crates/platform/sequence/src` 下
//! **全部字符串字面量**，凡形如 2 至 4 位大写字母者一律当作「本 crate 登记的类型码」，
//! 再与 `docs/data-dictionary.md` 第 5 节的登记表逐项比对。
//!
//! 因此**本 crate 内不得出现裸的 2 至 4 位大写字母字面量**，包括测试夹具与文档示例。
//! 需要造一个类型码时用 [`TypeCode::from_chars`]，它从字符构造、不经字面量。
//! 违反这条不会立刻报错——登记表当前 0 行，门禁处于「推迟」态而不进比对；
//! 但登记表一出现第一行，那些字面量就会被报成「有类型码 X，字典中没有登记」。
//! 这是一条埋着的雷，写在这里免得下一个人踩。

pub mod number;
pub mod port;
pub mod registry;

pub use number::{DocumentNumber, LegalEntityCode, PeriodKey, SequenceError, TypeCode};
pub use port::{AllocateRequest, Allocated, NumberAllocator};
pub use registry::{ScopeKind, TypeCodeRegistry, REGISTERED_TYPE_CODES};
