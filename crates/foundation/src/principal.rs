//! 系统主体常量。
//!
//! 取值选用全零前缀加版本位 7 与变体位 8 的保留形态：符合 UUIDv7 的
//! 版本与变体校验，同时不可能与 IdGen 生成的任何值碰撞。
//! 凡在种子迁移或系统上下文写 created_by 与 updated_by 的，一律引用该常量。

pub const SYSTEM_PRINCIPAL_ID: uuid::Uuid =
    uuid::uuid!("00000000-0000-7000-8000-000000000001");
pub const SYSTEM_DEVICE_ID: &str = "SYSTEM";
