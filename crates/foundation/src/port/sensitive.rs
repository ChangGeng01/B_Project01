//! 敏感字段解密位点端口。
//!
//! 全库唯一的敏感字段解密位点：行内字段级加密的列只有在字段级授权为
//! READ 或 WRITE、且安全上下文密级不低于字段密级时，才经本端口解密输出；
//! MASKED 与 HIDDEN 形态一律不解密（04 计划 §4.1 字段级判定的冻结口径）。
//!
//! 端口下沉 foundation 的理由与 [`crate::port::kms`] 同构：消费方
//! ep-platform-authz 不得依赖 adapter 层，实现载体基于
//! [`crate::port::kms::KmsBackend`] 由两个 apps 的 wiring 目录注入（F-04）。
//! 本模块不声明任何载体类型，明文材料不出载体。

use std::sync::Arc;

use crate::error::AppError;
use crate::id::marker::LegalEntity;
use crate::id::Id;
use crate::port::kms::CipherEnvelope;

/// 一次字段解密请求。AAD 由载体按当前行重构（16 字节法人标识、
/// `schema.table.column` 与 16 字节行标识三段拼接），调用方只给定位信息。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldDecryptRequest {
    /// 行所属法人，参与 AAD 重构。
    pub legal_entity_id: Id<LegalEntity>,
    /// 对象类型，`<module>.<table>` 小写下划线形态。
    pub object_type: Arc<str>,
    /// 字段名，即物理列名。
    pub field_name: Arc<str>,
    /// 行主键，参与 AAD 重构。
    pub row_id: uuid::Uuid,
    /// 该列的自描述密文信封。
    pub envelope: CipherEnvelope,
}

/// 敏感字段解密端口。对象安全，装配时以 `Arc<dyn SensitiveFieldDecryptor>`
/// 注入；解密失败的错误码由载体按其登记的 CRYPTO 段映射。
#[async_trait::async_trait]
pub trait SensitiveFieldDecryptor: Send + Sync {
    /// 解密单个字段，返回明文字节。
    async fn decrypt_field(&self, request: FieldDecryptRequest) -> Result<Vec<u8>, AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端口必须对象安全：装配以 `Arc<dyn _>` 注入。
    #[test]
    fn decryptor_is_object_safe() {
        fn _assert(_x: std::sync::Arc<dyn SensitiveFieldDecryptor>) {}
    }

    #[test]
    fn request_carries_aad_locator_fields() {
        let req = FieldDecryptRequest {
            legal_entity_id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            object_type: Arc::from("finance.cash_accounts"),
            field_name: Arc::from("bank_account_no"),
            row_id: uuid::Uuid::from_u128(2),
            envelope: CipherEnvelope::new(vec![1, 2, 3]),
        };
        assert_eq!(req.object_type.as_ref(), "finance.cash_accounts");
        assert_eq!(req.envelope.as_bytes().len(), 3);
    }
}
