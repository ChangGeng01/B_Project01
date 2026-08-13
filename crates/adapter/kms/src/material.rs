//! 密钥材料三类型：`KeyDomain`、`DataKey`、`BlindIndexKey`。
//!
//! 出处：02 计划第 4 节与规格报告第 5 节。列形与 `platform_core.key_domains`、
//! `platform_core.data_keys` 两张表一一对应（第 2 节表二、表三），本 crate 只承载
//! 材料与状态，落库由集成层承接。
//!
//! 安全纪律：私钥与数据密钥的明文材料一律不出载体——`BlindIndexKey` 不实现
//! `Debug`、`Display` 与 `Clone`；`DataKey` 只携带封包形态（`wrapped_key`），
//! 明文 DEK 只存在于 `BuiltinKmsBackend` 的进程内缓存，`Debug` 手工实现且
//! 不打印封包字节。

use ep_foundation::error::codes::PLATFORM_REQUEST_INVALID_PAYLOAD;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::kms::{KeyPurpose, KeyRef};
use ep_foundation::AppError;

/// 密钥域类别。首版只放行 `LEGAL_ENTITY`，`GROUP_SHARED` 预留不放行
/// （`ck_key_domains_kind`），故本枚举不设第二变体。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DomainKind {
    /// 法人独立密钥域。
    LegalEntity,
}

impl DomainKind {
    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            DomainKind::LegalEntity => "LEGAL_ENTITY",
        }
    }
}

/// 密钥域四态。六条合法迁移见 [`KeyDomainState::allows`] 的注释与
/// `super::builtin` 的迁移入口；`DESTROYED` 为终态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyDomainState {
    /// 建立中：域已登记，KEK 与四 purpose 的 DEK 尚在置备。
    Provisioning,
    /// 在用：可加密可解密。
    Active,
    /// 销毁计划已立：双人审批与销毁前核验已通过，允许回退。
    DestroyPlanned,
    /// 已销毁：终态，域内全部 DEK 已销毁且销毁证明三项齐备。
    Destroyed,
}

impl KeyDomainState {
    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            KeyDomainState::Provisioning => "PROVISIONING",
            KeyDomainState::Active => "ACTIVE",
            KeyDomainState::DestroyPlanned => "DESTROY_PLANNED",
            KeyDomainState::Destroyed => "DESTROYED",
        }
    }

    /// 六条合法迁移的判定面（规格报告第 5 节状态机表）：
    /// 一、无→PROVISIONING（登记入口，不经本函数）；
    /// 二、PROVISIONING→ACTIVE（KEK 可解引用 + 四 purpose 各一把 version=1 ACTIVE DEK）；
    /// 三、ACTIVE→ACTIVE 轮换（事务级建议锁互斥，真实 advisory lock 由 db-pg 承接）；
    /// 四、ACTIVE→DESTROY_PLANNED（核验通过 + 双人审批 + 重新认证，端口未装配一律拒绝）；
    /// 五、DESTROY_PLANNED→ACTIVE 可回退（写审计）；
    /// 六、DESTROY_PLANNED→DESTROYED（全 DEK DESTROYED + 销毁证明三项）。
    /// 其余迁移一律非法，返 BUSINESS_CONFLICT 类错误。
    pub const fn allows(self, next: KeyDomainState) -> bool {
        matches!(
            (self, next),
            (KeyDomainState::Provisioning, KeyDomainState::Active)
                | (KeyDomainState::Active, KeyDomainState::Active)
                | (KeyDomainState::Active, KeyDomainState::DestroyPlanned)
                | (KeyDomainState::DestroyPlanned, KeyDomainState::Active)
                | (KeyDomainState::DestroyPlanned, KeyDomainState::Destroyed)
        )
    }
}

/// 数据密钥四态，链路只进不退：ACTIVE→RETIRING→RETIRED→DESTROYED。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DataKeyState {
    /// 在用：参与新密文加密，也解旧密文。
    Active,
    /// 退用中：不再加密新密文，仍解旧密文。
    Retiring,
    /// 已退用：只解旧密文。
    Retired,
    /// 已销毁：引用它的密文一律解密失败。
    Destroyed,
}

impl DataKeyState {
    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            DataKeyState::Active => "ACTIVE",
            DataKeyState::Retiring => "RETIRING",
            DataKeyState::Retired => "RETIRED",
            DataKeyState::Destroyed => "DESTROYED",
        }
    }

    /// 链路只进不退：ACTIVE→RETIRING→RETIRED→DESTROYED，其余非法。
    pub const fn allows(self, next: DataKeyState) -> bool {
        matches!(
            (self, next),
            (DataKeyState::Active, DataKeyState::Retiring)
                | (DataKeyState::Retiring, DataKeyState::Retired)
                | (DataKeyState::Retired, DataKeyState::Destroyed)
        )
    }
}

/// 数据密钥算法，对应 `data_keys.algorithm` 的两取值。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DekAlgorithm {
    /// 信封加密用。
    Aes256Gcm,
    /// 盲索引用。
    HmacSha256,
}

impl DekAlgorithm {
    /// 数据库侧登记的字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            DekAlgorithm::Aes256Gcm => "AES_256_GCM",
            DekAlgorithm::HmacSha256 => "HMAC_SHA256",
        }
    }

    /// 按 purpose 取既定算法：FIELD/ATTACHMENT/ARCHIVE 为 AES-256-GCM，
    /// BLIND_INDEX 为 HMAC-SHA256。
    pub const fn for_purpose(purpose: KeyPurpose) -> DekAlgorithm {
        match purpose {
            KeyPurpose::BlindIndex => DekAlgorithm::HmacSha256,
            KeyPurpose::Field | KeyPurpose::Attachment | KeyPurpose::Archive => {
                DekAlgorithm::Aes256Gcm
            }
        }
    }
}

/// 密钥域。对应 `platform_core.key_domains` 的材料列；时间戳四列由落库层维护，
/// 本类型不承载。不含密钥材料，可以 `Clone` 与 `Debug`。
#[derive(Clone, Debug)]
pub struct KeyDomain {
    id: uuid::Uuid,
    legal_entity_id: Id<LegalEntity>,
    domain_kind: DomainKind,
    state: KeyDomainState,
    kek_ref: KeyRef,
    kek_version: u32,
}

impl KeyDomain {
    /// 无→PROVISIONING 的登记形态：新域一律从建立中起步。
    /// `kek_version` 自 1 起（`ck > 0`）。
    pub fn new_provisioning(
        id: uuid::Uuid,
        legal_entity_id: Id<LegalEntity>,
        domain_kind: DomainKind,
        kek_ref: KeyRef,
    ) -> Self {
        Self {
            id,
            legal_entity_id,
            domain_kind,
            state: KeyDomainState::Provisioning,
            kek_ref,
            kek_version: 1,
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn legal_entity_id(&self) -> Id<LegalEntity> {
        self.legal_entity_id
    }

    pub fn domain_kind(&self) -> DomainKind {
        self.domain_kind
    }

    pub fn state(&self) -> KeyDomainState {
        self.state
    }

    pub fn kek_ref(&self) -> &KeyRef {
        &self.kek_ref
    }

    pub fn kek_version(&self) -> u32 {
        self.kek_version
    }

    /// 仅供载体内部状态机改写状态，不对外开放任意赋值。
    pub(crate) fn set_state(&mut self, state: KeyDomainState) {
        self.state = state;
    }
}

/// 数据密钥的持久化形态。对应 `platform_core.data_keys`；只携带封包字节
/// （`wrapped_key`），明文材料不出载体。不实现 `Clone`，`Debug` 手工实现
/// 且不打印封包字节。
pub struct DataKey {
    id: uuid::Uuid,
    key_domain_id: uuid::Uuid,
    purpose: KeyPurpose,
    security_level_scope: u8,
    version: u16,
    algorithm: DekAlgorithm,
    wrapped_key: Vec<u8>,
    wrap_kek_version: u32,
    state: DataKeyState,
}

impl DataKey {
    /// 构造。`version` 必须大于 0，`security_level_scope` 取 10/20/30/40 之一，
    /// 由载体登记入口统一断言。
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        id: uuid::Uuid,
        key_domain_id: uuid::Uuid,
        purpose: KeyPurpose,
        security_level_scope: u8,
        version: u16,
        algorithm: DekAlgorithm,
        wrapped_key: Vec<u8>,
        wrap_kek_version: u32,
        state: DataKeyState,
    ) -> Self {
        Self {
            id,
            key_domain_id,
            purpose,
            security_level_scope,
            version,
            algorithm,
            wrapped_key,
            wrap_kek_version,
            state,
        }
    }

    /// 从落库记录重建（集成层读 `platform_core.data_keys` 后注入）。
    /// 封包字节是安全形态，可以出载体入库；明文材料仍不出载体。
    #[allow(clippy::too_many_arguments)]
    pub fn from_record(
        id: uuid::Uuid,
        key_domain_id: uuid::Uuid,
        purpose: KeyPurpose,
        security_level_scope: u8,
        version: u16,
        algorithm: DekAlgorithm,
        wrapped_key: Vec<u8>,
        wrap_kek_version: u32,
        state: DataKeyState,
    ) -> Result<DataKey, AppError> {
        if version == 0 {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "data_keys.version 必须大于 0",
            ));
        }
        if !matches!(security_level_scope, 10 | 20 | 30 | 40) {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("security_level_scope 取值 {security_level_scope} 不在 10|20|30|40"),
            ));
        }
        if wrapped_key.is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "wrapped_key 不得为空",
            ));
        }
        Ok(Self::new(
            id,
            key_domain_id,
            purpose,
            security_level_scope,
            version,
            algorithm,
            wrapped_key,
            wrap_kek_version,
            state,
        ))
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn key_domain_id(&self) -> uuid::Uuid {
        self.key_domain_id
    }

    pub fn purpose(&self) -> KeyPurpose {
        self.purpose
    }

    pub fn security_level_scope(&self) -> u8 {
        self.security_level_scope
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn algorithm(&self) -> DekAlgorithm {
        self.algorithm
    }

    /// 封包字节（安全形态，非明文），供集成层落库。
    pub fn wrapped_key_bytes(&self) -> &[u8] {
        &self.wrapped_key
    }

    pub fn wrap_kek_version(&self) -> u32 {
        self.wrap_kek_version
    }

    pub fn state(&self) -> DataKeyState {
        self.state
    }

    /// 仅供载体内部状态机改写状态。
    pub(crate) fn set_state(&mut self, state: DataKeyState) {
        self.state = state;
    }
}

impl core::fmt::Debug for DataKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // 刻意不打印 wrapped_key：封包字节也是密钥材料，只露长度。
        write!(
            f,
            "DataKey(id={}, purpose={}, version={}, state={}, wrapped_len={})",
            self.id,
            self.purpose.as_str(),
            self.version,
            self.state.as_str(),
            self.wrapped_key.len()
        )
    }
}

/// 盲索引密钥材料：`HKDF-SHA256(dek, info = schema.table.column)` 的输出。
/// 明文材料不出载体：不实现 `Debug`、`Display` 与 `Clone`，`Drop` 时清零。
pub struct BlindIndexKey {
    bytes: [u8; 32],
}

impl BlindIndexKey {
    pub(crate) fn new(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// 只限载体内部取用。
    pub(crate) fn bytes(&self) -> &[u8; 32] {
        &self.bytes
    }
}

impl Drop for BlindIndexKey {
    fn drop(&mut self) {
        // 材料清零，避免明文密钥滞留堆内存。
        self.bytes.fill(0);
        // 阻止编译器把清零当死代码消除。
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

/// ACTIVE→DESTROY_PLANNED 的准入凭据（规格报告第 5 节迁移四）：
/// 销毁前核验报告、双人审批记录、重新认证凭据三项齐备才放行；
/// 审批与重新认证的验证端口未装配时，集成层不得出具本凭据，
/// 载体见缺项一律拒绝（02 计划 A-05：端口未装配时一律 403）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestroyApproval {
    /// 销毁前核验五步报告引用（缺则 DESTROY_PRECHECK_FAILED）。
    pub precheck_report_ref: String,
    /// 双人审批记录引用（缺则视为端口未装配，一律拒绝）。
    pub approval_ref: String,
    /// 重新认证凭据引用（缺则视为端口未装配，一律拒绝）。
    pub reauth_ref: String,
}

/// DESTROY_PLANNED→DESTROYED 的销毁证明三项（02 计划第 7.8/12.4 章）：
/// 不可读范围清单、仍可读范围清单与逐项补足措施，任一缺失不得销毁。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestroyEvidence {
    /// 受字段级密钥保护列（不可读范围）清单引用。
    pub unreadable_scope_ref: String,
    /// 未加密业务表（仍可读范围）清单引用。
    pub readable_scope_ref: String,
    /// 逐项补足措施（PHYSICAL_DELETE/ANONYMIZE/RETAIN 三选一）引用。
    pub remediation_ref: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_state_literals() {
        assert_eq!(KeyDomainState::Provisioning.as_str(), "PROVISIONING");
        assert_eq!(KeyDomainState::Active.as_str(), "ACTIVE");
        assert_eq!(KeyDomainState::DestroyPlanned.as_str(), "DESTROY_PLANNED");
        assert_eq!(KeyDomainState::Destroyed.as_str(), "DESTROYED");
        assert_eq!(DomainKind::LegalEntity.as_str(), "LEGAL_ENTITY");
    }

    #[test]
    fn dek_algorithm_follows_purpose() {
        assert_eq!(
            DekAlgorithm::for_purpose(KeyPurpose::Field),
            DekAlgorithm::Aes256Gcm
        );
        assert_eq!(
            DekAlgorithm::for_purpose(KeyPurpose::BlindIndex),
            DekAlgorithm::HmacSha256
        );
        assert_eq!(DekAlgorithm::Aes256Gcm.as_str(), "AES_256_GCM");
        assert_eq!(DekAlgorithm::HmacSha256.as_str(), "HMAC_SHA256");
    }

    #[test]
    fn data_key_debug_hides_wrapped_bytes() {
        let dk = DataKey::new(
            uuid::Uuid::nil(),
            uuid::Uuid::nil(),
            KeyPurpose::Field,
            40,
            1,
            DekAlgorithm::Aes256Gcm,
            vec![0xAB; 60],
            1,
            DataKeyState::Active,
        );
        let shown = format!("{:?}", dk);
        assert!(shown.contains("wrapped_len=60"));
        assert!(!shown.contains("ab"), "封包字节不得出现在 Debug 输出");
    }
}
