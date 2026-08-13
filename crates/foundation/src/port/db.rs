//! 数据库能力端口。
//!
//! 公共能力基线描述（对应规格第 7.4 章的能力面，与具体数据库无关）：
//! 字段类型限定为标识、文本、时间、金额、数量、枚举取值等固定集合，
//! 金额、数量、单价、比率四类数值一律带精度后缀声明；
//! 索引只允许普通 B-tree 与唯一约束，禁止函数索引、部分索引与 JSON 路径索引；
//! 在线变更边界由迁移侧落实，锁等待上限 5 秒，执行上限 30 分钟。
//! 本描述是对业务侧的能力承诺；到具体数据库的类型与索引映射由 ep-adapter-db-pg 承接。
//!
//! 按裁定 F-01，原 ep-adapter-db 承载的端口 trait 与能力描述下沉本模块，
//! 与 `port::tx`、`port::search`、`port::doc` 三个端口模块并列：
//! `IdempotencyScope`、`IdempotencyOutcome`、`IdempotencyStore` 按 C-07 由阶段 2 补齐，
//! `MigrationWindowGuard` 按 B-03 由阶段 2 补齐，只读事务端口 `ReadOnlyTx` 由阶段 11 补齐。
//!
//! 本模块只放端口与能力描述。具体类型、取值与 SQL 侧映射一律落在 ep-adapter-db-pg，
//! 本模块不声明任何 `Pg` 前缀的类型，也不出现任何数据库专有语法。

use crate::error::AppError;
use crate::id::marker::{LegalEntity, UserAccount};
use crate::id::Id;

use super::tx::Tx;

/// 一次幂等请求的作用域，四字段共同定位一个幂等键。
///
/// 逐字出处是 02 计划第 6 节：同一法人、同一用户、同一端点、同一键值
/// 四者合起来唯一。请求头的存在性与 UUIDv7 合法性由阶段 1 的
/// `IdempotencyKeyHeaderGuard` 校验，不在本端口的职责之内。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdempotencyScope {
    pub legal_entity_id: Id<LegalEntity>,
    pub user_id: Id<UserAccount>,
    pub endpoint: String,
    pub key: uuid::Uuid,
}

/// 幂等判定的三态结论（F-01 只冻结三个变体名，判定实现归阶段 3a）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum IdempotencyOutcome {
    /// 首次调用：键位此前不存在，本次请求取得执行权。
    FirstCall,
    /// 重放：同一键、同一请求哈希此前已完成，直接回放当时定稿的响应。
    Replay {
        response_status: u16,
        response_body: Vec<u8>,
    },
    /// 载荷不符：同一键携带了不同的请求哈希。
    /// 调用方据此映射其已登记的 `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`。
    PayloadMismatch,
}

/// 幂等键存储端口（C-07）。职责三段中本端口只承担中间一段：
/// 定义契约，不校验请求头，不建表，不判等。
///
/// 事务内的持久化与并发在途去重由阶段 3a 在 `platform_msg.idempotency_keys` 上实现；
/// 两个方法的签名逐字取自 02 计划第 6 节，不得增删。
#[async_trait::async_trait]
pub trait IdempotencyStore: Send + Sync {
    /// 尝试占用一个幂等键，返回三态结论之一。
    async fn try_begin(
        &self,
        tx: &mut dyn Tx,
        scope: IdempotencyScope,
        request_hash: [u8; 32],
    ) -> Result<IdempotencyOutcome, AppError>;

    /// 把本次请求的响应定稿到幂等键上，供后续重放回放进。
    async fn finish(
        &self,
        tx: &mut dyn Tx,
        scope: IdempotencyScope,
        response_status: u16,
        response_body: &[u8],
    ) -> Result<(), AppError>;
}

/// 迁移窗口守卫（B-03）。在线 DDL 开始前由调用方出示一个处于 `OPEN`
/// 状态的迁移窗口，未持 `OPEN` 窗口一律返回
/// `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`（HTTP 409，分类 BUSINESS_CONFLICT，
/// 错误码常量见 [`crate::error::codes::PLATFORM_DB_MIGRATION_WINDOW_CLOSED`]，
/// 阶段 1 已登记，本端口不重复登记）。
///
/// 唯一实现类型为 ep-adapter-db-pg 的 `PgMigrationWindowGuard`，
/// 在 `apps/core-server/src/wiring/` 与 `apps/job-worker/src/wiring/` 两处注入；
/// `ep-platform-release` 不引用本 trait。
#[async_trait::async_trait]
pub trait MigrationWindowGuard: Send + Sync {
    /// 断言迁移窗口处于开启状态，否则返回窗口关闭错误。
    async fn assert_open(&self, tx: &mut dyn Tx) -> Result<(), AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::marker;

    #[test]
    fn scope_carries_four_fields() {
        let scope = IdempotencyScope {
            legal_entity_id: Id::<marker::LegalEntity>::from_uuid(uuid::Uuid::nil()),
            user_id: Id::<marker::UserAccount>::from_uuid(uuid::Uuid::nil()),
            endpoint: "POST /api/v1/platform/x".to_string(),
            key: uuid::Uuid::nil(),
        };
        assert_eq!(scope.endpoint, "POST /api/v1/platform/x");
        assert_eq!(scope.clone(), scope, "作用域可克隆且可判等");
    }

    #[test]
    fn outcome_has_exactly_three_variants() {
        // 三态名按 F-01 冻结；穷举匹配保证增删变体时本测试必须同步改写。
        for outcome in [
            IdempotencyOutcome::FirstCall,
            IdempotencyOutcome::Replay {
                response_status: 200,
                response_body: Vec::new(),
            },
            IdempotencyOutcome::PayloadMismatch,
        ] {
            match outcome {
                IdempotencyOutcome::FirstCall
                | IdempotencyOutcome::Replay { .. }
                | IdempotencyOutcome::PayloadMismatch => {}
            }
        }
    }

    /// 两个端口都必须对象安全：装配侧以 `Arc<dyn _>` 注入。
    #[test]
    fn ports_are_object_safe() {
        fn _idempotency(_x: std::sync::Arc<dyn IdempotencyStore>) {}
        fn _window(_x: std::sync::Arc<dyn MigrationWindowGuard>) {}
    }
}
