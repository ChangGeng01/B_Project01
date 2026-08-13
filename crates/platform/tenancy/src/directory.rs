//! 法人档案读取契约（A-04，02 计划 §4.8）。
//!
//! 两个方法的签名逐字冻结，任何阶段不得改写。本模块只写端口语义：
//! 真实的 SQL 查询实现体落在 ep-adapter-db-pg 的 `platform_core` 仓储目录内
//! （基线第 1.3 节允许 adapter 依赖 platform 的端口 trait），
//! platform 不依赖 adapter，依赖方向由 `xtask archcheck` 断言。

use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;

/// 一条法人档案的最小读取投影。`legal_entities` 表不建行级安全策略
/// （迁移第 14 号登记豁免），故枚举与按主键读取不需要会话变量前置。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LegalEntityRef {
    pub id: Id<LegalEntity>,
    pub code: String,
    pub entity_no: String,
    pub name: String,
    pub is_active: bool,
}

/// 法人目录。阶段 3 枚举法人的唯一入口是 [`LegalEntityDirectory::list_active`]。
#[async_trait::async_trait]
pub trait LegalEntityDirectory: Send + Sync {
    async fn list_active(&self) -> Result<Vec<LegalEntityRef>, AppError>;
    async fn get(&self, id: Id<LegalEntity>) -> Result<LegalEntityRef, AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 装配侧以 `Arc<dyn _>` 注入，trait 必须对象安全。
    #[test]
    fn trait_is_object_safe() {
        fn _dir(_x: std::sync::Arc<dyn LegalEntityDirectory>) {}
    }

    #[test]
    fn ref_carries_five_fields() {
        let r = LegalEntityRef {
            id: Id::<LegalEntity>::from_uuid(uuid::Uuid::nil()),
            code: "LE01".to_string(),
            entity_no: "E-0001".to_string(),
            name: "示例法人".to_string(),
            is_active: true,
        };
        assert_eq!(r.clone(), r, "档案投影可克隆且可判等");
    }
}
