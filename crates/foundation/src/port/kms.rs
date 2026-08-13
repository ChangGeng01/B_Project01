//! 密钥管理端口。
//!
//! 按裁定 F-04，`KmsBackend` 与其调用词汇落本模块：六个方法为
//! `wrap`、`unwrap`、`derive_blind_key`、`sign`、`verify`、`health`，
//! 连同八个词汇类型构成端口面九项；内置 KMS 与客户 HSM 两种载体的实现
//! 落在 ep-adapter-kms，本模块不声明任何载体类型，私钥与数据密钥材料一律不出载体。
//!
//! 端口下沉本模块的理由：ep-platform-release 与 ep-platform-audit 需要命名该 trait，
//! 而基线第 1.3 节允许项只准 platform 依赖 foundation 与其他 platform；
//! 端口停在 ep-adapter-kms 会构成 platform 反向依赖 adapter。
//!
//! 签名算法全卷固定 ECDSA P-256，因此端口不带算法参数；
//! `verify` 取 `Result<bool, AppError>`，`false` 表示验签不通过，
//! 由调用方映射到其已登记的错误码，本阶段不因此新增错误码。
//!
//! `derive_blind_key` 只冻结三参数形态，返回宽度是待决项，不随本批冻结：
//! `BlindIndex` 现取 `[u8; 16]`，而 02 计划第 11 节假设三要求
//! `finance.cash_accounts` 走确需唯一路径时取完整 32 字节，且第 7 节
//! `EP__CRYPTO__BLIND_INDEX__BYTES` 允许取 16 或 32——三处合一之前，
//! 任何阶段不得把 16 字节当作已冻结结论。
//!
//! `CipherText`、`KeyDomainId`、`BlindIndex` 三者不实现 `Debug` 与 `Display`
//! 的明文形态；`CipherText` 的 `Debug` 输出固定为 `CipherText(len=N)`，
//! `CipherEnvelope` 承载密文，同样只做长度脱敏。

use crate::error::AppError;
use crate::id::marker::LegalEntity;
use crate::id::Id;

/// 数据密钥的用途，四变体与 `platform_core.data_keys.purpose` 的取值一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyPurpose {
    /// 行内敏感字段的信封加密。
    Field,
    /// 受治理盲索引的派生。
    BlindIndex,
    /// 附件加密。
    Attachment,
    /// 归档加密。
    Archive,
}

impl KeyPurpose {
    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            KeyPurpose::Field => "FIELD",
            KeyPurpose::BlindIndex => "BLIND_INDEX",
            KeyPurpose::Attachment => "ATTACHMENT",
            KeyPurpose::Archive => "ARCHIVE",
        }
    }
}

/// 密文字节。不实现 `Display`；`Debug` 固定输出长度，不泄露内容。
#[derive(Clone, PartialEq, Eq)]
pub struct CipherText(Vec<u8>);

impl CipherText {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl core::fmt::Debug for CipherText {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CipherText(len={})", self.0.len())
    }
}

/// 密钥域标识。不实现明文 `Debug` 与 `Display`。
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KeyDomainId(pub uuid::Uuid);

/// 盲索引值。现取 `[u8; 16]`，该宽度是待决项（见模块注释），不随本批冻结。
/// 不实现明文 `Debug` 与 `Display`。
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BlindIndex([u8; 16]);

impl BlindIndex {
    pub const fn new(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

/// 附加认证数据（AAD）。由调用方按当前行重构：16 字节法人标识、
/// `schema.table.column` 与 16 字节行标识三段拼接，拼装规则见 02 计划第 4.3 节。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Aad(Vec<u8>);

impl Aad {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// 密钥引用，形如 `kms://builtin/le/<uuid>` 或 `kms://hsm/slot0/le/<uuid>`。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyRef(String);

impl KeyRef {
    pub fn new(reference: impl Into<String>) -> Self {
        Self(reference.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// ECDSA P-256 签名字节。算法全卷固定，故签名本身不携带算法标识。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Signature(Vec<u8>);

impl Signature {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// 自描述密文信封（EPC1 布局，见 02 计划第 4.3 节）。承载密文，
/// `Debug` 与 `CipherText` 同样只做长度脱敏。
#[derive(Clone, PartialEq, Eq)]
pub struct CipherEnvelope(Vec<u8>);

impl CipherEnvelope {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl core::fmt::Debug for CipherEnvelope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CipherEnvelope(len={})", self.0.len())
    }
}

/// 密钥管理后端端口（F-04）。六个方法中 `wrap`、`unwrap`、`sign`、
/// `verify`、`health` 五者的参数与返回类型冻结，任何阶段不得改写；
/// `derive_blind_key` 只冻结三参数形态。trait 无泛型方法，对象安全，
/// 装配时以 `Arc<dyn KmsBackend>` 注入 `apps/core-server/src/wiring/`
/// 与 `apps/job-worker/src/wiring/` 两个目录。
#[async_trait::async_trait]
pub trait KmsBackend: Send + Sync + 'static {
    /// 用指定域与用途的当前 ACTIVE 数据密钥做信封加密。
    async fn wrap(
        &self,
        domain: KeyDomainId,
        purpose: KeyPurpose,
        aad: &Aad,
        plaintext: &[u8],
    ) -> Result<CipherEnvelope, AppError>;

    /// 按信封内的密钥引用解封，AAD 由调用方按当前行重构。
    async fn unwrap(
        &self,
        domain: KeyDomainId,
        aad: &Aad,
        envelope: &CipherEnvelope,
    ) -> Result<Vec<u8>, AppError>;

    /// 派生盲索引密钥并产出盲索引值（B-04 唯一入口）。
    /// 只冻结三参数形态；返回宽度是待决项，见模块注释。
    async fn derive_blind_key(
        &self,
        legal_entity_id: Id<LegalEntity>,
        column_fqn: &str,
        plaintext: &[u8],
    ) -> Result<BlindIndex, AppError>;

    /// 以 ECDSA P-256 签名，算法全卷固定，端口不带算法参数。
    async fn sign(&self, key: &KeyRef, payload: &[u8]) -> Result<Signature, AppError>;

    /// 验签。`false` 表示验签不通过，由调用方映射其已登记错误码。
    async fn verify(
        &self,
        key: &KeyRef,
        payload: &[u8],
        signature: &Signature,
    ) -> Result<bool, AppError>;

    /// 后端健康探测。
    async fn health(&self) -> Result<(), AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cipher_text_debug_hides_content() {
        let ct = CipherText::new(vec![0xAB, 0xCD, 0xEF]);
        let shown = format!("{:?}", ct);
        assert_eq!(shown, "CipherText(len=3)");
        assert!(
            !shown.contains("ab") && !shown.contains("171"),
            "不得泄露字节取值"
        );
        assert_eq!(ct.len(), 3);
        assert!(!ct.is_empty());
        assert_eq!(ct.as_bytes(), &[0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn envelope_debug_only_shows_length() {
        let env = CipherEnvelope::new(vec![1, 2, 3, 4]);
        assert_eq!(format!("{:?}", env), "CipherEnvelope(len=4)");
        assert_eq!(env.as_bytes().len(), 4);
        assert_eq!(env.into_bytes(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn key_domain_id_construct_and_compare() {
        // 本类型刻意不实现 Debug，故用布尔断言而非 assert_eq!。
        let a = KeyDomainId(uuid::Uuid::nil());
        let b = KeyDomainId(uuid::Uuid::nil());
        assert!(a == b);
        assert!(a != KeyDomainId(uuid::Uuid::from_u128(1)));
    }

    #[test]
    fn blind_index_width_is_sixteen_for_now() {
        // 宽度 16 是待决项而非冻结结论，注释见模块头；本测试只固化现状。
        let idx = BlindIndex::new([7; 16]);
        assert_eq!(idx.as_bytes(), &[7; 16]);
        assert_eq!(core::mem::size_of::<BlindIndex>(), 16);
    }

    #[test]
    fn key_purpose_literals_match_data_keys() {
        assert_eq!(KeyPurpose::Field.as_str(), "FIELD");
        assert_eq!(KeyPurpose::BlindIndex.as_str(), "BLIND_INDEX");
        assert_eq!(KeyPurpose::Attachment.as_str(), "ATTACHMENT");
        assert_eq!(KeyPurpose::Archive.as_str(), "ARCHIVE");
    }

    #[test]
    fn aad_keyref_signature_construct() {
        let aad = Aad::new(vec![9; 48]);
        assert_eq!(aad.as_bytes().len(), 48);
        let key = KeyRef::new("kms://builtin/le/00000000-0000-7000-8000-000000000001");
        assert!(key.as_str().starts_with("kms://"));
        let sig = Signature::new(vec![1, 2]);
        assert_eq!(sig.as_bytes(), &[1, 2]);
    }

    /// 端口必须对象安全：装配以 `Arc<dyn KmsBackend>` 注入。
    #[test]
    fn backend_is_object_safe() {
        fn _assert(_x: std::sync::Arc<dyn KmsBackend>) {}
    }
}
