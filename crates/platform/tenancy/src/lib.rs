//! ep-platform-tenancy — 组织架构读取契约与部门层级闭包（A-04，阶段 2 交付）。
//!
//! 职责：集团、法人、组织、部门、岗位，以及安全上下文的建立与法人授权集合校验。
//! 本阶段交付两样：
//! 一、两个逐字冻结的读取契约 trait（[`LegalEntityDirectory`] 与
//!     [`DepartmentClosureQuery`]，签名出处 02 计划 §4.8）及其纯逻辑配套
//!     （闭包子树全量重写计划 [`plan_subtree_rewrite`]）；
//! 二、平台管理域九端点的能力常量（[`capability`]，A-20）。
//!
//! 依赖方向：本 crate 只依赖 ep-foundation；SQL 执行体按基线第 1.3 节
//! 落在 ep-adapter-db-pg 的 `platform_core` 仓储目录内（adapter 可依赖
//! platform 的端口 trait），platform 不依赖 adapter，
//! 由 `xtask archcheck` 的 platform-no-adapter 逐条断言。

pub mod capability;
pub mod closure;
pub mod directory;

pub use closure::{
    cap_by_depth, plan_subtree_rewrite, ClosureRow, DepartmentClosureQuery, SubtreeRewritePlan,
    MAX_ORG_DEPTH,
};
pub use directory::{LegalEntityDirectory, LegalEntityRef};
