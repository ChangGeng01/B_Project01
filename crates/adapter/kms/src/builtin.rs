//! 内置 KMS 载体（`EP__KMS__BACKEND=builtin`）。
//!
//! 主密钥取 master.key（32 字节随机、不二次加密，假设一），DEK 封包与
//! 信封加密均走 AES-256-GCM；签名固定 ECDSA P-256，签名密钥由主密钥经
//! HKDF 按 `KeyRef` 确定性派生，私钥不出载体。
//!
//! 密钥域状态机六条合法迁移的入口都在本模块（`register_domain` 对应
//! 无→PROVISIONING，其余五条各占一个方法），非法迁移一律返
//! `PLATFORM.KEY_DOMAIN.TRANSITION_INVALID`（BUSINESS_CONFLICT）。
//!
//! ACTIVE→ACTIVE 轮换的互斥在本 crate 内表达为进程内非阻塞锁（try-lock）：
//! 在途即返 `PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS`。跨进程的事务级
//! 建议锁语义（`pg_advisory_xact_lock(hashtextextended('key_domain:'||id||':'||purpose, 0))`）
//! 由 db-pg 侧承接，本处只保证接口参数与返回形态与之对齐。
//!
//! 销毁前核验五步（枚举受保护列、枚举未加密表、逐项补足措施、保留义务与
//! 备份处置核验、缺失即失败）依赖 `sensitive_field_registry` 与 `pg_class`，
//! 落在集成层执行；本模块以 [`DestroyApproval`] 凭据承接其结果，缺项一律拒绝。

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Nonce};
use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_OBJECT_FORBIDDEN, PLATFORM_CRYPTO_DECRYPT_FAILED,
    PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED, PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED, PLATFORM_KEY_DOMAIN_ROTATION_IN_PROGRESS,
    PLATFORM_KEY_DOMAIN_TRANSITION_INVALID, PLATFORM_REQUEST_INVALID_PAYLOAD,
    PLATFORM_SYSTEM_INTERNAL_ERROR,
};
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::kms::{
    Aad, BlindIndex, CipherEnvelope, KeyDomainId, KeyPurpose, KeyRef, KmsBackend, Signature,
};
use ep_foundation::AppError;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::cache::{wall_clock, Clock, DekCache};
use crate::cfg::{
    parse_blind_index_bytes, DekCacheCfg, ENV_BLIND_INDEX_BYTES, ENV_DEK_CACHE_MAX_ENTRIES,
    ENV_DEK_CACHE_TTL_S,
};
use crate::envelope::{self, PLAINTEXT_MAX};
use crate::masterkey::{load_master_key, MasterKey};
use crate::material::{
    DataKey, DataKeyState, DekAlgorithm, DestroyApproval, DestroyEvidence, DomainKind, KeyDomain,
    KeyDomainState,
};
use crate::normalize::{normalize, Normalization};

/// DEK 封包的 AAD：固定字面量即可，封包与信封是两层独立认证。
const DEK_WRAP_AAD: &[u8] = b"ep/kms/dek-wrap/v1";
/// 签名密钥派生的 HKDF salt。
const SIGN_KEY_SALT: &[u8] = b"ep/kms/sign/v1";
/// 盲索引密钥派生只冻结 info 段：`HKDF-SHA256(dek, info = schema.table.column)`。
const HEALTH_CHECK_KEYREF: &str = "kms://builtin/selftest";

/// 轮换结果，对应 02 计划 A-04 的响应形态（新版本号与旧版本 RETIRING）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RotationReport {
    pub new_version: u16,
    pub new_data_key_id: uuid::Uuid,
    pub retiring_data_key_id: uuid::Uuid,
}

#[derive(Default)]
struct Registry {
    domains: HashMap<uuid::Uuid, KeyDomain>,
    data_keys: HashMap<uuid::Uuid, DataKey>,
    /// 轮换建议锁的进程内表达：(域, purpose) 在途即互斥。
    rotation_locks: HashSet<(uuid::Uuid, &'static str)>,
}

/// 内置 KMS 载体。只实现 `KmsBackend` 一个公开 trait；密钥域与数据密钥的
/// 管理面以固有方法提供，不声明端口之外的任何公开 trait。
pub struct BuiltinKmsBackend {
    master: MasterKey,
    registry: Mutex<Registry>,
    cache: DekCache,
}

impl BuiltinKmsBackend {
    /// 生产入口：按路径读 master.key，权限与属主校验失败即拒启动。
    pub fn new(master_key_path: &std::path::Path) -> Result<BuiltinKmsBackend, AppError> {
        let master = load_master_key(master_key_path)?;
        Ok(Self::from_master(master, wall_clock()))
    }

    /// 测试与装配入口：直接注入主密钥与时钟，不碰文件系统。
    pub(crate) fn from_master(master: MasterKey, clock: Clock) -> BuiltinKmsBackend {
        BuiltinKmsBackend {
            master,
            registry: Mutex::new(Registry::default()),
            cache: DekCache::new(clock),
        }
    }

    // —— 配置热生效：两缓存键与盲索引宽度键每次使用时重读。——

    fn cache_cfg_hot() -> DekCacheCfg {
        DekCacheCfg::parse(
            std::env::var(ENV_DEK_CACHE_MAX_ENTRIES).ok().as_deref(),
            std::env::var(ENV_DEK_CACHE_TTL_S).ok().as_deref(),
        )
    }

    fn blind_width_hot() -> Result<usize, AppError> {
        parse_blind_index_bytes(std::env::var(ENV_BLIND_INDEX_BYTES).ok().as_deref())
    }

    // —— 密钥域状态机六条迁移。——

    /// 迁移一：无→PROVISIONING。新域一律从建立中起步；同法人同 kind
    /// 已有域即拒绝（法人存在且 is_active 由集成层在调用前核验）。
    pub fn register_domain(&self, domain: KeyDomain) -> Result<(), AppError> {
        let mut reg = self.lock_registry();
        if domain.state() != KeyDomainState::Provisioning {
            return Err(transition_invalid(&format!(
                "新域必须以 PROVISIONING 起步，实为 {}",
                domain.state().as_str()
            )));
        }
        let occupied = reg.domains.values().any(|d| {
            d.legal_entity_id() == domain.legal_entity_id()
                && d.domain_kind() == domain.domain_kind()
        });
        if occupied {
            return Err(transition_invalid(
                "同法人同 domain_kind 已有密钥域，不得重复建立",
            ));
        }
        reg.domains.insert(domain.id(), domain);
        Ok(())
    }

    /// 迁移二：PROVISIONING→ACTIVE。前置：KEK 可解引用，四 purpose 各一把
    /// version=1 的 ACTIVE DEK。
    pub fn activate_domain(&self, domain_id: uuid::Uuid) -> Result<(), AppError> {
        let mut reg = self.lock_registry();
        // 先只读核验（状态与 KEK），避免与后置的可变借用冲突。
        let (state, kek_ref) = match reg.domains.get(&domain_id) {
            Option::None => {
                return Err(AppError::new(
                    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                    format!("密钥域 {domain_id} 不存在"),
                ))
            }
            Some(d) => (d.state(), d.kek_ref().as_str().to_string()),
        };
        if state != KeyDomainState::Provisioning {
            return Err(transition_invalid(&format!(
                "只有 PROVISIONING 域可激活，当前 {}",
                state.as_str()
            )));
        }
        // KEK 可解引用：内置载体只认 kms://builtin/ 前缀。
        if !kek_ref.starts_with("kms://builtin/") {
            return Err(AppError::new(
                PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                format!("KEK 引用 {kek_ref} 在内置载体不可解引用"),
            ));
        }
        for purpose in [
            KeyPurpose::Field,
            KeyPurpose::BlindIndex,
            KeyPurpose::Attachment,
            KeyPurpose::Archive,
        ] {
            let ok = reg.data_keys.values().any(|k| {
                k.key_domain_id() == domain_id
                    && k.purpose() == purpose
                    && k.version() == 1
                    && k.state() == DataKeyState::Active
            });
            if !ok {
                return Err(AppError::new(
                    PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                    format!(
                        "purpose {} 缺 version=1 的 ACTIVE 数据密钥",
                        purpose.as_str()
                    ),
                ));
            }
        }
        reg.domains
            .get_mut(&domain_id)
            .expect("刚定位")
            .set_state(KeyDomainState::Active);
        Ok(())
    }

    /// 迁移三：ACTIVE→ACTIVE 轮换。进程内非阻塞互斥，在途返 409 语义的
    /// `ROTATION_IN_PROGRESS`；事务级建议锁由 db-pg 承接。
    pub fn rotate(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
    ) -> Result<RotationReport, AppError> {
        // 建议锁：try-lock 语义，拿不到即在途。
        {
            let mut reg = self.lock_registry();
            let domain = reg.domains.get(&domain_id).ok_or_else(|| {
                AppError::new(
                    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                    format!("密钥域 {domain_id} 不存在"),
                )
            })?;
            if domain.state() != KeyDomainState::Active {
                return Err(transition_invalid(&format!(
                    "只有 ACTIVE 域可轮换，当前 {}",
                    domain.state().as_str()
                )));
            }
            if !reg.rotation_locks.insert((domain_id, purpose.as_str())) {
                return Err(AppError::new(
                    PLATFORM_KEY_DOMAIN_ROTATION_IN_PROGRESS,
                    format!("域 {domain_id} 的 {} 已有轮换在途", purpose.as_str()),
                ));
            }
        }
        // 锁内工作：新版本上位，旧 ACTIVE 置 RETIRING。任何失败都要放锁。
        let outcome = (|| {
            let retiring_id = self.current_active_dek(domain_id, purpose)?.0;
            let new_id = self.generate_data_key_inner(domain_id, purpose)?;
            let mut reg = self.lock_registry();
            let new_version = reg.data_keys.get(&new_id).expect("刚生成").version();
            reg.data_keys
                .get_mut(&retiring_id)
                .expect("刚定位")
                .set_state(DataKeyState::Retiring);
            Ok(RotationReport {
                new_version,
                new_data_key_id: new_id,
                retiring_data_key_id: retiring_id,
            })
        })();
        self.lock_registry()
            .rotation_locks
            .remove(&(domain_id, purpose.as_str()));
        outcome
    }

    /// 迁移四：ACTIVE→DESTROY_PLANNED。核验报告、双人审批、重新认证三项齐备
    /// 才放行；审批与重认证端口未装配时集成层不得出具凭据，见缺项一律拒绝。
    pub fn plan_destroy(
        &self,
        domain_id: uuid::Uuid,
        approval: &DestroyApproval,
    ) -> Result<(), AppError> {
        if approval.precheck_report_ref.is_empty() {
            return Err(AppError::new(
                PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED,
                "销毁前核验五步报告缺失，不得立项销毁",
            ));
        }
        if approval.approval_ref.is_empty() || approval.reauth_ref.is_empty() {
            // 端口未装配一律拒绝（02 计划 A-05，403 语义）。
            return Err(AppError::new(
                PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
                "双人审批或重新认证凭据缺失，端口未装配时一律拒绝",
            ));
        }
        let mut reg = self.lock_registry();
        let domain = reg.domains.get_mut(&domain_id).ok_or_else(|| {
            AppError::new(
                PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                format!("密钥域 {domain_id} 不存在"),
            )
        })?;
        if domain.state() != KeyDomainState::Active {
            return Err(transition_invalid(&format!(
                "只有 ACTIVE 域可立项销毁，当前 {}",
                domain.state().as_str()
            )));
        }
        domain.set_state(KeyDomainState::DestroyPlanned);
        Ok(())
    }

    /// 迁移五：DESTROY_PLANNED→ACTIVE 可回退，`audit_ref` 为审计记录引用。
    pub fn restore_from_destroy_plan(
        &self,
        domain_id: uuid::Uuid,
        audit_ref: &str,
    ) -> Result<(), AppError> {
        if audit_ref.is_empty() {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "回退必须携带审计记录引用",
            ));
        }
        let mut reg = self.lock_registry();
        let domain = reg.domains.get_mut(&domain_id).ok_or_else(|| {
            AppError::new(
                PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                format!("密钥域 {domain_id} 不存在"),
            )
        })?;
        if domain.state() != KeyDomainState::DestroyPlanned {
            return Err(transition_invalid(&format!(
                "只有 DESTROY_PLANNED 域可回退，当前 {}",
                domain.state().as_str()
            )));
        }
        domain.set_state(KeyDomainState::Active);
        Ok(())
    }

    /// 迁移六：DESTROY_PLANNED→DESTROYED。全 DEK DESTROYED 且销毁证明三项
    /// 齐备；`DESTROYED` 为终态。
    pub fn destroy(
        &self,
        domain_id: uuid::Uuid,
        evidence: &DestroyEvidence,
    ) -> Result<(), AppError> {
        if evidence.unreadable_scope_ref.is_empty()
            || evidence.readable_scope_ref.is_empty()
            || evidence.remediation_ref.is_empty()
        {
            return Err(AppError::new(
                PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED,
                "销毁证明三项（不可读范围、仍可读范围、补足措施）缺一不得销毁",
            ));
        }
        let mut reg = self.lock_registry();
        // 先只读核验状态与 DEK 链，避免与后置的可变借用冲突。
        let state = match reg.domains.get(&domain_id) {
            Option::None => {
                return Err(AppError::new(
                    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                    format!("密钥域 {domain_id} 不存在"),
                ))
            }
            Some(d) => d.state(),
        };
        if state != KeyDomainState::DestroyPlanned {
            return Err(transition_invalid(&format!(
                "只有 DESTROY_PLANNED 域可销毁，当前 {}",
                state.as_str()
            )));
        }
        let all_destroyed = reg
            .data_keys
            .values()
            .filter(|k| k.key_domain_id() == domain_id)
            .all(|k| k.state() == DataKeyState::Destroyed);
        if !all_destroyed {
            return Err(transition_invalid("域内尚有未销毁的数据密钥"));
        }
        reg.domains
            .get_mut(&domain_id)
            .expect("刚定位")
            .set_state(KeyDomainState::Destroyed);
        Ok(())
    }

    /// 域快照（只读，供装配与测试观察）。
    pub fn domain_snapshot(&self, domain_id: uuid::Uuid) -> Option<KeyDomain> {
        self.lock_registry().domains.get(&domain_id).cloned()
    }

    // —— 数据密钥登记与四态链路。——

    /// 生成新数据密钥：随机 32 字节，主密钥封包，版本取 (域, purpose) 现存
    /// 最大版本加一。明文材料自生成起不出载体，返回其标识。
    pub fn generate_data_key(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
        security_level_scope: u8,
    ) -> Result<uuid::Uuid, AppError> {
        self.validate_scope(security_level_scope)?;
        {
            let reg = self.lock_registry();
            let domain = reg.domains.get(&domain_id).ok_or_else(|| {
                AppError::new(
                    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                    format!("密钥域 {domain_id} 不存在"),
                )
            })?;
            if !matches!(
                domain.state(),
                KeyDomainState::Provisioning | KeyDomainState::Active
            ) {
                return Err(transition_invalid(&format!(
                    "域状态 {} 下不得新增数据密钥",
                    domain.state().as_str()
                )));
            }
        }
        self.generate_data_key_inner_with(domain_id, purpose, security_level_scope)
    }

    /// 生成与进程内注册表无关的封包密钥材料（集成层以库为基准直接
    /// 轮换时使用：进程重启后内存注册表与库断档，新版本号与状态链
    /// 只能由库侧推导）。只走主密钥封包，不登记不入缓存；
    /// 返回（数据密钥标识，封包字节）。
    pub fn generate_detached_data_key(&self) -> (uuid::Uuid, Vec<u8>) {
        let dek: [u8; 32] = rand::random();
        let wrapped = self.wrap_dek_internal(&dek);
        (uuid::Uuid::now_v7(), wrapped)
    }

    /// 从落库记录注入数据密钥（集成层读库后装配）。
    pub fn ingest_data_key(&self, data_key: DataKey) -> Result<(), AppError> {
        let mut reg = self.lock_registry();
        if reg.data_keys.contains_key(&data_key.id()) {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "数据密钥标识重复注入",
            ));
        }
        reg.data_keys.insert(data_key.id(), data_key);
        Ok(())
    }

    /// 取封包字节（安全形态），供集成层落库。
    pub fn wrapped_key_of(&self, data_key_id: uuid::Uuid) -> Option<Vec<u8>> {
        self.lock_registry()
            .data_keys
            .get(&data_key_id)
            .map(|k| k.wrapped_key_bytes().to_vec())
    }

    /// 数据密钥状态位，供集成层与测试观察。
    pub fn data_key_state_of(&self, data_key_id: uuid::Uuid) -> Option<DataKeyState> {
        self.lock_registry()
            .data_keys
            .get(&data_key_id)
            .map(|k| k.state())
    }

    /// ACTIVE→RETIRING。
    pub fn retire_data_key(&self, data_key_id: uuid::Uuid) -> Result<(), AppError> {
        self.move_dek_state(data_key_id, DataKeyState::Retiring)
    }

    /// RETIRING→RETIRED。
    pub fn release_data_key(&self, data_key_id: uuid::Uuid) -> Result<(), AppError> {
        self.move_dek_state(data_key_id, DataKeyState::Retired)
    }

    /// RETIRED→DESTROYED。销毁后同步逐出缓存，引用该 DEK 的密文自此解密必败。
    pub fn destroy_data_key(&self, data_key_id: uuid::Uuid) -> Result<(), AppError> {
        self.move_dek_state(data_key_id, DataKeyState::Destroyed)?;
        self.cache.evict(data_key_id);
        Ok(())
    }

    fn move_dek_state(&self, data_key_id: uuid::Uuid, next: DataKeyState) -> Result<(), AppError> {
        let mut reg = self.lock_registry();
        let dk = reg.data_keys.get_mut(&data_key_id).ok_or_else(|| {
            AppError::new(
                PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                format!("数据密钥 {data_key_id} 不存在"),
            )
        })?;
        if !dk.state().allows(next) {
            return Err(transition_invalid(&format!(
                "数据密钥状态机非法迁移 {}→{}",
                dk.state().as_str(),
                next.as_str()
            )));
        }
        dk.set_state(next);
        Ok(())
    }

    // —— 盲索引（B-04 唯一入口的承接）。——

    /// 派生盲索引密钥材料：`HKDF-SHA256(dek, info = schema.table.column)`，
    /// DEK 取该法人 BLIND_INDEX 用途的当前 ACTIVE 密钥。
    pub fn derive_blind_key_material(
        &self,
        legal_entity_id: Id<LegalEntity>,
        column_fqn: &str,
    ) -> Result<crate::material::BlindIndexKey, AppError> {
        let domain_id = self.domain_for_legal_entity(legal_entity_id)?;
        let (dek_id, _) = self.current_active_dek(domain_id, KeyPurpose::BlindIndex)?;
        let dek = self.dek_material(dek_id)?;
        let hk = Hkdf::<Sha256>::new(Option::None, &dek);
        let mut blind = [0u8; 32];
        hk.expand(column_fqn.as_bytes(), &mut blind)
            .map_err(|_| internal_error("HKDF 展开失败"))?;
        Ok(crate::material::BlindIndexKey::new(blind))
    }

    /// 盲索引值：`HMAC-SHA256(blind_key, normalize(value))` 前 N 字节。
    /// N 取 `EP__CRYPTO__BLIND_INDEX__BYTES`（热生效，16 或 32）——
    /// **宽度是待决项，不得视为冻结结论**（02 计划第 8 节待决一）。
    pub fn blind_index(
        &self,
        legal_entity_id: Id<LegalEntity>,
        column_fqn: &str,
        value: &str,
        normalization: Normalization,
    ) -> Result<Vec<u8>, AppError> {
        let width = Self::blind_width_hot()?;
        let blind_key = self.derive_blind_key_material(legal_entity_id, column_fqn)?;
        let mut mac =
            <Hmac<Sha256> as Mac>::new_from_slice(blind_key.bytes()).expect("HMAC 接受任意长密钥");
        mac.update(&normalize(value, normalization));
        let full = mac.finalize().into_bytes();
        Ok(full[..width].to_vec())
    }

    // —— 密码学内务。——

    fn lock_registry(&self) -> std::sync::MutexGuard<'_, Registry> {
        self.registry.lock().expect("密钥域注册表锁不得投毒")
    }

    fn validate_scope(&self, scope: u8) -> Result<(), AppError> {
        if matches!(scope, 10 | 20 | 30 | 40) {
            Ok(())
        } else {
            Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("security_level_scope 取值 {scope} 不在 10|20|30|40"),
            ))
        }
    }

    fn generate_data_key_inner(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
    ) -> Result<uuid::Uuid, AppError> {
        // 轮换沿用域内该 purpose 现行密级范围。
        let scope = {
            let reg = self.lock_registry();
            reg.data_keys
                .values()
                .filter(|k| k.key_domain_id() == domain_id && k.purpose() == purpose)
                .map(|k| k.security_level_scope())
                .next()
                .unwrap_or(40)
        };
        self.generate_data_key_inner_with(domain_id, purpose, scope)
    }

    fn generate_data_key_inner_with(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
        scope: u8,
    ) -> Result<uuid::Uuid, AppError> {
        let dek: [u8; 32] = rand::random();
        let wrapped = self.wrap_dek_internal(&dek);
        let id = uuid::Uuid::now_v7();
        let mut reg = self.lock_registry();
        let version = reg
            .data_keys
            .values()
            .filter(|k| k.key_domain_id() == domain_id && k.purpose() == purpose)
            .map(|k| k.version())
            .max()
            .unwrap_or(0)
            + 1;
        let kek_version = reg
            .domains
            .get(&domain_id)
            .map(|d| d.kek_version())
            .unwrap_or(1);
        let record = DataKey::new(
            id,
            domain_id,
            purpose,
            scope,
            version,
            DekAlgorithm::for_purpose(purpose),
            wrapped,
            kek_version,
            DataKeyState::Active,
        );
        reg.data_keys.insert(id, record);
        Ok(id)
    }

    /// 定位域内某 purpose 的当前 ACTIVE DEK（按唯一约束定位后取最高版本）。
    fn current_active_dek(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
    ) -> Result<(uuid::Uuid, u16), AppError> {
        let reg = self.lock_registry();
        reg.data_keys
            .values()
            .filter(|k| {
                k.key_domain_id() == domain_id
                    && k.purpose() == purpose
                    && k.state() == DataKeyState::Active
            })
            .max_by_key(|k| k.version())
            .map(|k| (k.id(), k.version()))
            .ok_or_else(|| {
                AppError::new(
                    PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                    format!(
                        "域 {domain_id} 缺 {} 用途的 ACTIVE 数据密钥",
                        purpose.as_str()
                    ),
                )
            })
    }

    /// 按法人定位 LEGAL_ENTITY 域的在用域。
    fn domain_for_legal_entity(
        &self,
        legal_entity_id: Id<LegalEntity>,
    ) -> Result<uuid::Uuid, AppError> {
        let reg = self.lock_registry();
        let domain = reg
            .domains
            .values()
            .find(|d| {
                d.legal_entity_id() == legal_entity_id && d.domain_kind() == DomainKind::LegalEntity
            })
            .ok_or_else(|| {
                AppError::new(
                    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                    format!("法人 {legal_entity_id} 尚无密钥域"),
                )
            })?;
        if domain.state() != KeyDomainState::Active {
            return Err(AppError::new(
                PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                format!(
                    "法人 {legal_entity_id} 的密钥域处于 {}，不可派生",
                    domain.state().as_str()
                ),
            ));
        }
        Ok(domain.id())
    }

    /// 取域状态并校验其处于 ACTIVE，返 (DEK 标识, 版本)。
    fn active_dek_in_active_domain(
        &self,
        domain_id: uuid::Uuid,
        purpose: KeyPurpose,
    ) -> Result<(uuid::Uuid, u16), AppError> {
        {
            let reg = self.lock_registry();
            match reg.domains.get(&domain_id) {
                Option::None => {
                    return Err(AppError::new(
                        PLATFORM_KEY_DOMAIN_NOT_PROVISIONED,
                        format!("密钥域 {domain_id} 不存在"),
                    ))
                }
                Some(d) if d.state() != KeyDomainState::Active => {
                    return Err(AppError::new(
                        PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                        format!(
                            "密钥域 {domain_id} 处于 {}，不提供加解密",
                            d.state().as_str()
                        ),
                    ))
                }
                Some(_) => {}
            }
        }
        self.current_active_dek(domain_id, purpose)
    }

    /// 主密钥封包 DEK：nonce(12) || AES-256-GCM 密文加标签。
    fn wrap_dek_internal(&self, dek: &[u8; 32]) -> Vec<u8> {
        let nonce_bytes: [u8; 12] = rand::random();
        let cipher = Aes256Gcm::new_from_slice(self.master.bytes()).expect("主密钥定长 32");
        let ct = cipher
            .encrypt(
                Nonce::from_slice(&nonce_bytes),
                Payload {
                    msg: dek,
                    aad: DEK_WRAP_AAD,
                },
            )
            .expect("封包加密不得失败");
        let mut out = Vec::with_capacity(12 + ct.len());
        out.extend_from_slice(&nonce_bytes);
        out.extend_from_slice(&ct);
        out
    }

    /// 解封 DEK。任何失败按密钥不可用上报，不泄露失败细节。
    fn unwrap_dek_internal(&self, wrapped: &[u8]) -> Result<[u8; 32], AppError> {
        let unavailable =
            || AppError::new(PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE, "数据密钥封包解封失败");
        if wrapped.len() != 12 + 32 + 16 {
            return Err(unavailable());
        }
        let cipher = Aes256Gcm::new_from_slice(self.master.bytes()).expect("主密钥定长 32");
        let pt = cipher
            .decrypt(
                Nonce::from_slice(&wrapped[..12]),
                Payload {
                    msg: &wrapped[12..],
                    aad: DEK_WRAP_AAD,
                },
            )
            .map_err(|_| unavailable())?;
        Ok(pt.as_slice().try_into().expect("封包内明文定长 32"))
    }

    /// DEK 明文材料：先查进程内缓存（热生效的 512/300s），未命中再解封。
    fn dek_material(&self, dek_id: uuid::Uuid) -> Result<[u8; 32], AppError> {
        let cfg = Self::cache_cfg_hot();
        if let Some(hit) = self.cache.get(dek_id, cfg) {
            return Ok(hit);
        }
        let wrapped = {
            let reg = self.lock_registry();
            reg.data_keys
                .get(&dek_id)
                .map(|k| k.wrapped_key_bytes().to_vec())
                .ok_or_else(|| {
                    AppError::new(
                        PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                        format!("数据密钥 {dek_id} 不存在"),
                    )
                })?
        };
        let material = self.unwrap_dek_internal(&wrapped)?;
        self.cache.put(dek_id, material, cfg);
        Ok(material)
    }

    /// 按 KeyRef 确定性派生 P-256 签名密钥：HKDF(master, salt, keyref||counter)，
    /// 遇到非法标量顺延计数器。私钥只在载体内生成与消亡。
    fn signing_key_for(&self, key_ref: &KeyRef) -> Result<p256::ecdsa::SigningKey, AppError> {
        let hk = Hkdf::<Sha256>::new(Some(SIGN_KEY_SALT), self.master.bytes());
        for counter in 0u8..32 {
            let mut info = key_ref.as_str().as_bytes().to_vec();
            info.push(counter);
            let mut okm = [0u8; 32];
            if hk.expand(&info, &mut okm).is_err() {
                continue;
            }
            if let Ok(key) = p256::ecdsa::SigningKey::from_slice(&okm) {
                return Ok(key);
            }
        }
        Err(internal_error("签名密钥派生失败"))
    }
}

fn transition_invalid(detail: &str) -> AppError {
    AppError::new(
        PLATFORM_KEY_DOMAIN_TRANSITION_INVALID,
        format!("密钥域状态机非法迁移：{detail}"),
    )
}

fn internal_error(detail: &str) -> AppError {
    AppError::new(PLATFORM_SYSTEM_INTERNAL_ERROR, detail.to_string())
}

#[async_trait::async_trait]
impl KmsBackend for BuiltinKmsBackend {
    /// EPC1 信封加密四步：取当前 ACTIVE DEK → 12 字节随机 nonce →
    /// AES-256-GCM → 拼装。空串仍加密；明文超 1MB 拒绝走附件通道。
    async fn wrap(
        &self,
        domain: KeyDomainId,
        purpose: KeyPurpose,
        aad: &Aad,
        plaintext: &[u8],
    ) -> Result<CipherEnvelope, AppError> {
        if plaintext.len() > PLAINTEXT_MAX {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("明文 {} 字节超过 1MB 上限，请改走附件通道", plaintext.len()),
            ));
        }
        let (dek_id, version) = self.active_dek_in_active_domain(domain.0, purpose)?;
        let material = self.dek_material(dek_id)?;
        let nonce_bytes: [u8; 12] = rand::random();
        let cipher = Aes256Gcm::new_from_slice(&material).expect("DEK 定长 32");
        let ct = cipher
            .encrypt(
                Nonce::from_slice(&nonce_bytes),
                Payload {
                    msg: plaintext,
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| envelope::aad_mismatch())?;
        Ok(CipherEnvelope::new(envelope::assemble(
            dek_id,
            version,
            &nonce_bytes,
            &ct,
        )))
    }

    /// 解密：校验魔数长度；DEK 已销毁返 `DECRYPT_FAILED`（带 incident_no）；
    /// 标签失败返 `AAD_MISMATCH`。
    async fn unwrap(
        &self,
        domain: KeyDomainId,
        aad: &Aad,
        envelope_bytes: &CipherEnvelope,
    ) -> Result<Vec<u8>, AppError> {
        let parsed = envelope::parse(envelope_bytes.as_bytes())?;
        {
            let reg = self.lock_registry();
            let dk = reg.data_keys.get(&parsed.dek_id).ok_or_else(|| {
                AppError::new(
                    PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                    format!("信封引用的数据密钥 {} 不存在", parsed.dek_id),
                )
            })?;
            if dk.key_domain_id() != domain.0 {
                return Err(AppError::new(
                    PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE,
                    "信封引用的数据密钥不属于当前密钥域",
                ));
            }
            if dk.state() == DataKeyState::Destroyed {
                // 关联编号入 message，供事故追踪；材料细节不外露。
                let incident_no = uuid::Uuid::now_v7();
                return Err(AppError::new(
                    PLATFORM_CRYPTO_DECRYPT_FAILED,
                    format!("引用的数据密钥已销毁，解密拒绝；incident_no={incident_no}"),
                ));
            }
        }
        let material = self.dek_material(parsed.dek_id)?;
        let cipher = Aes256Gcm::new_from_slice(&material).expect("DEK 定长 32");
        cipher
            .decrypt(
                Nonce::from_slice(parsed.nonce),
                Payload {
                    msg: parsed.ciphertext,
                    aad: aad.as_bytes(),
                },
            )
            .map_err(|_| envelope::aad_mismatch())
    }

    /// 盲索引（B-04 唯一入口）。归一化取登记默认 TRIM_NFKC；
    /// 端口 `BlindIndex` 现取 16 字节，配置宽度 32 的例外列改用固有方法
    /// [`BuiltinKmsBackend::blind_index`]——宽度是待决项，见模块注释。
    async fn derive_blind_key(
        &self,
        legal_entity_id: Id<LegalEntity>,
        column_fqn: &str,
        plaintext: &[u8],
    ) -> Result<BlindIndex, AppError> {
        let value = core::str::from_utf8(plaintext).map_err(|_| {
            AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                "盲索引取值必须是 UTF-8 文本",
            )
        })?;
        let full = self.blind_index(legal_entity_id, column_fqn, value, Normalization::TrimNfkc)?;
        let truncated: [u8; 16] = full[..16].try_into().expect("盲索引宽度最小 16 字节");
        Ok(BlindIndex::new(truncated))
    }

    /// ECDSA P-256 签名，算法全卷固定。
    async fn sign(&self, key: &KeyRef, payload: &[u8]) -> Result<Signature, AppError> {
        use p256::ecdsa::signature::Signer;
        let signing_key = self.signing_key_for(key)?;
        let signature: p256::ecdsa::Signature = signing_key.sign(payload);
        Ok(Signature::new(signature.to_bytes().to_vec()))
    }

    /// 验签。签名形态非法或验不过一律 `Ok(false)`，由调用方映射其错误码。
    async fn verify(
        &self,
        key: &KeyRef,
        payload: &[u8],
        signature: &Signature,
    ) -> Result<bool, AppError> {
        use p256::ecdsa::signature::Verifier;
        let signing_key = self.signing_key_for(key)?;
        let verifying_key = p256::ecdsa::VerifyingKey::from(&signing_key);
        let Ok(parsed) = p256::ecdsa::Signature::from_slice(signature.as_bytes()) else {
            return Ok(false);
        };
        Ok(verifying_key.verify(payload, &parsed).is_ok())
    }

    /// 自检：AEAD 往返加签名往返。
    async fn health(&self) -> Result<(), AppError> {
        // 一、AEAD 往返：临时随机密钥，密文必须可解回。
        let key_bytes: [u8; 32] = rand::random();
        let cipher = Aes256Gcm::new_from_slice(&key_bytes).expect("临时密钥定长 32");
        let nonce = [0u8; 12];
        let ct = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: b"ep-kms-selftest",
                    aad: b"health",
                },
            )
            .map_err(|_| internal_error("自检加密失败"))?;
        let pt = cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &ct,
                    aad: b"health",
                },
            )
            .map_err(|_| internal_error("自检解密失败"))?;
        if pt != b"ep-kms-selftest" {
            return Err(internal_error("自检明文不一致"));
        }
        // 二、签名往返：固定自检 KeyRef。
        let key = KeyRef::new(HEALTH_CHECK_KEYREF);
        let signature = self.sign(&key, b"ep-kms-selftest").await?;
        if !self.verify(&key, b"ep-kms-selftest", &signature).await? {
            return Err(internal_error("自检验签失败"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_foundation::error::codes::{
        PLATFORM_CRYPTO_AAD_MISMATCH, PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID,
        PLATFORM_CRYPTO_DECRYPT_FAILED, PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED,
        PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE, PLATFORM_KEY_DOMAIN_TRANSITION_INVALID,
    };
    use std::future::Future;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    struct NowWake;
    impl Wake for NowWake {
        fn wake(self: Arc<Self>) {}
    }

    /// 本载体的 async 方法内部无挂起点，首次 poll 即就绪；
    /// 单次驱动即可，不为测试引入运行时。
    fn poll_once<F: Future>(fut: F) -> F::Output {
        let waker = Waker::from(Arc::new(NowWake));
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(out) => out,
            Poll::Pending => panic!("载体 future 不应挂起"),
        }
    }

    fn backend() -> BuiltinKmsBackend {
        BuiltinKmsBackend::from_master(MasterKey::new([7; 32]), wall_clock())
    }

    fn le() -> Id<LegalEntity> {
        Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(0x1111))
    }

    const FQN: &str = "mdm.customers.phone_no";

    fn row_aad(row: uuid::Uuid) -> Aad {
        envelope::aad_for_row(le(), FQN, row)
    }

    /// 建一个已激活的法人密钥域：四 purpose 各一把 version=1 的 ACTIVE DEK。
    fn provisioned() -> (BuiltinKmsBackend, uuid::Uuid) {
        let be = backend();
        let domain_id = uuid::Uuid::from_u128(0xD001);
        be.register_domain(KeyDomain::new_provisioning(
            domain_id,
            le(),
            DomainKind::LegalEntity,
            KeyRef::new("kms://builtin/master"),
        ))
        .expect("登记域");
        for p in [
            KeyPurpose::Field,
            KeyPurpose::BlindIndex,
            KeyPurpose::Attachment,
            KeyPurpose::Archive,
        ] {
            be.generate_data_key(domain_id, p, 40).expect("生成 DEK");
        }
        be.activate_domain(domain_id).expect("激活域");
        (be, domain_id)
    }

    fn dek_id_of(env: &CipherEnvelope) -> uuid::Uuid {
        uuid::Uuid::from_bytes(env.as_bytes()[5..21].try_into().expect("区间定长 16"))
    }

    // —— 一、信封往返：含 AAD 篡改失败、魔数错误、空串、超 1MB。——

    #[test]
    fn envelope_roundtrip_and_aad_tamper_fails() {
        let (be, domain) = provisioned();
        let aad = row_aad(uuid::Uuid::from_u128(0xA1));
        let env =
            poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"secret")).unwrap();
        // 头部字节逐字校验：魔数、算法、DEK 版本大端。
        assert_eq!(&env.as_bytes()[0..4], b"EPC1");
        assert_eq!(env.as_bytes()[4], 0x01);
        assert_eq!(&env.as_bytes()[21..23], &[0x00, 0x01]);
        let pt = poll_once(be.unwrap(KeyDomainId(domain), &aad, &env)).unwrap();
        assert_eq!(pt, b"secret");
        // AAD 篡改（行标识不符）必败。
        let wrong_row = row_aad(uuid::Uuid::from_u128(0xA2));
        let err = poll_once(be.unwrap(KeyDomainId(domain), &wrong_row, &env)).unwrap_err();
        assert_eq!(err.code, PLATFORM_CRYPTO_AAD_MISMATCH);
        // 空串仍加密：最短信封 51 字节，可解回空明文。
        let env0 = poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"")).unwrap();
        assert_eq!(env0.as_bytes().len(), 51);
        assert!(poll_once(be.unwrap(KeyDomainId(domain), &aad, &env0))
            .unwrap()
            .is_empty());
        // 魔数错误：格式非法。
        let bad_magic = CipherEnvelope::new(vec![0u8; 64]);
        let err = poll_once(be.unwrap(KeyDomainId(domain), &aad, &bad_magic)).unwrap_err();
        assert_eq!(err.code, PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID);
        // 明文超 1MB 拒。
        let err = poll_once(be.wrap(
            KeyDomainId(domain),
            KeyPurpose::Field,
            &aad,
            &vec![0u8; PLAINTEXT_MAX + 1],
        ))
        .unwrap_err();
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
    }

    #[test]
    fn destroyed_dek_rejects_decrypt_and_retired_still_decrypts() {
        let (be, domain) = provisioned();
        let aad = row_aad(uuid::Uuid::from_u128(0xB1));
        let env =
            poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"keep")).unwrap();
        let dek_id = dek_id_of(&env);
        // 四态链前进：RETIRING 与 RETIRED 仍解旧密文。
        be.retire_data_key(dek_id).unwrap();
        assert!(poll_once(be.unwrap(KeyDomainId(domain), &aad, &env)).is_ok());
        be.release_data_key(dek_id).unwrap();
        assert!(poll_once(be.unwrap(KeyDomainId(domain), &aad, &env)).is_ok());
        // DESTROYED 后解密拒绝，错误码归 DECRYPT_FAILED。
        be.destroy_data_key(dek_id).unwrap();
        let err = poll_once(be.unwrap(KeyDomainId(domain), &aad, &env)).unwrap_err();
        assert_eq!(err.code, PLATFORM_CRYPTO_DECRYPT_FAILED);
        assert!(err.message.contains("incident_no="));
    }

    // —— 二、盲索引：确定性与归一化分支。——

    #[test]
    fn blind_index_determinism_and_normalization_branches() {
        let (be, _) = provisioned();
        // 确定性：同输入必同输出。
        let a = be
            .blind_index(le(), FQN, "13800138000", Normalization::DigitsOnly)
            .unwrap();
        let again = be
            .blind_index(le(), FQN, "13800138000", Normalization::DigitsOnly)
            .unwrap();
        assert_eq!(a, again);
        assert_eq!(a.len(), 16);
        // DIGITS_ONLY：非数字字符不参与索引。
        let dashed = be
            .blind_index(le(), FQN, "138-0013-8000", Normalization::DigitsOnly)
            .unwrap();
        assert_eq!(a, dashed);
        // TRIM_NFKC_LOWER：首尾空白与大小写不参与。
        let lower1 = be
            .blind_index(le(), FQN, "  Héllo ", Normalization::TrimNfkcLower)
            .unwrap();
        let lower2 = be
            .blind_index(le(), FQN, "héllo", Normalization::TrimNfkcLower)
            .unwrap();
        assert_eq!(lower1, lower2);
        // NONE：大小写敏感。
        let raw1 = be
            .blind_index(le(), FQN, "Hello", Normalization::None)
            .unwrap();
        let raw2 = be
            .blind_index(le(), FQN, "hello", Normalization::None)
            .unwrap();
        assert_ne!(raw1, raw2);
        // 列名入派生链：不同列不同索引。
        let other = be
            .blind_index(
                le(),
                "mdm.customers.email",
                "13800138000",
                Normalization::DigitsOnly,
            )
            .unwrap();
        assert_ne!(a, other);
        // 端口方法：UTF-8 校验 + 默认 TRIM_NFKC + 恒取前 16 字节。
        let via_port = poll_once(be.derive_blind_key(le(), FQN, " value ".as_bytes())).unwrap();
        assert_eq!(via_port.as_bytes().len(), 16);
        let err = match poll_once(be.derive_blind_key(le(), FQN, &[0xFF, 0xFE])) {
            Err(e) => e,
            Ok(_) => panic!("非 UTF-8 取值应被拒绝"),
        };
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
    }

    // —— 三、状态机：合法与非法迁移。——

    #[test]
    fn state_machine_legal_and_illegal_transitions() {
        let be = backend();
        let domain_id = uuid::Uuid::from_u128(0xD002);
        be.register_domain(KeyDomain::new_provisioning(
            domain_id,
            le(),
            DomainKind::LegalEntity,
            KeyRef::new("kms://builtin/master"),
        ))
        .unwrap();
        // 同法人同 kind 重复登记拒。
        let dup = KeyDomain::new_provisioning(
            uuid::Uuid::from_u128(0xD003),
            le(),
            DomainKind::LegalEntity,
            KeyRef::new("kms://builtin/master"),
        );
        let err = be.register_domain(dup).unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_TRANSITION_INVALID);
        // DEK 未齐备不得激活。
        be.generate_data_key(domain_id, KeyPurpose::Field, 40)
            .unwrap();
        let err = be.activate_domain(domain_id).unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE);
        for p in [
            KeyPurpose::BlindIndex,
            KeyPurpose::Attachment,
            KeyPurpose::Archive,
        ] {
            be.generate_data_key(domain_id, p, 40).unwrap();
        }
        // 非法：PROVISIONING 域不得立项销毁。
        let err = be
            .plan_destroy(
                domain_id,
                &DestroyApproval {
                    precheck_report_ref: "audit://precheck/1".to_string(),
                    approval_ref: "audit://approval/1".to_string(),
                    reauth_ref: "audit://reauth/1".to_string(),
                },
            )
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_TRANSITION_INVALID);
        be.activate_domain(domain_id).unwrap();
        // ACTIVE→ACTIVE 轮换：新版本上位，旧密钥转 RETIRING。
        let report = be.rotate(domain_id, KeyPurpose::Field).unwrap();
        assert_eq!(report.new_version, 2);
        assert_eq!(
            be.data_key_state_of(report.retiring_data_key_id),
            Some(DataKeyState::Retiring)
        );
        // 核验报告缺失拒立项。
        let err = be
            .plan_destroy(
                domain_id,
                &DestroyApproval {
                    precheck_report_ref: String::new(),
                    approval_ref: "x".to_string(),
                    reauth_ref: "x".to_string(),
                },
            )
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED);
        // 凭据齐备放行；DESTROY_PLANNED 下不得生成新 DEK。
        be.plan_destroy(
            domain_id,
            &DestroyApproval {
                precheck_report_ref: "audit://precheck/1".to_string(),
                approval_ref: "audit://approval/1".to_string(),
                reauth_ref: "audit://reauth/1".to_string(),
            },
        )
        .unwrap();
        let err = be
            .generate_data_key(domain_id, KeyPurpose::Field, 40)
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_TRANSITION_INVALID);
        // 回退必须带审计引用；回退后可再立项。
        let err = be.restore_from_destroy_plan(domain_id, "").unwrap_err();
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
        be.restore_from_destroy_plan(domain_id, "audit://restore/1")
            .unwrap();
        assert_eq!(
            be.domain_snapshot(domain_id).unwrap().state(),
            KeyDomainState::Active
        );
    }

    #[test]
    fn destroy_requires_all_deks_destroyed_and_evidence() {
        let (be, domain) = provisioned();
        be.plan_destroy(
            domain,
            &DestroyApproval {
                precheck_report_ref: "audit://precheck/1".to_string(),
                approval_ref: "audit://approval/1".to_string(),
                reauth_ref: "audit://reauth/1".to_string(),
            },
        )
        .unwrap();
        let evidence = DestroyEvidence {
            unreadable_scope_ref: "audit://unreadable".to_string(),
            readable_scope_ref: "audit://readable".to_string(),
            remediation_ref: "audit://remediation".to_string(),
        };
        // DEK 未全销毁拒。
        let err = be.destroy(domain, &evidence).unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_TRANSITION_INVALID);
        // 逐把销毁域内全部 DEK。
        let ids: Vec<uuid::Uuid> = {
            let reg = be.lock_registry();
            reg.data_keys
                .values()
                .filter(|k| k.key_domain_id() == domain)
                .map(|k| k.id())
                .collect()
        };
        for id in ids {
            be.retire_data_key(id).unwrap();
            be.release_data_key(id).unwrap();
            be.destroy_data_key(id).unwrap();
        }
        // 销毁证明缺项拒。
        let err = be
            .destroy(
                domain,
                &DestroyEvidence {
                    unreadable_scope_ref: String::new(),
                    readable_scope_ref: "x".to_string(),
                    remediation_ref: "x".to_string(),
                },
            )
            .unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED);
        be.destroy(domain, &evidence).unwrap();
        assert_eq!(
            be.domain_snapshot(domain).unwrap().state(),
            KeyDomainState::Destroyed
        );
        // DESTROYED 域不再提供加密。
        let aad = row_aad(uuid::Uuid::from_u128(0xC1));
        let err =
            poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"x")).unwrap_err();
        assert_eq!(err.code, PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE);
    }

    // —— 四、签名与自检。——

    #[test]
    fn sign_verify_roundtrip_and_health() {
        let be = backend();
        let key = KeyRef::new("kms://builtin/le/test");
        let sig = poll_once(be.sign(&key, b"payload")).unwrap();
        assert!(poll_once(be.verify(&key, b"payload", &sig)).unwrap());
        assert!(!poll_once(be.verify(&key, b"other", &sig)).unwrap());
        // 签名形态非法：验不过而非报错。
        assert!(!poll_once(be.verify(&key, b"payload", &Signature::new(vec![0; 3]))).unwrap());
        poll_once(be.health()).unwrap();
    }

    // —— 五、缓存行为在 builtin 面上的表现（TTL 细节见 cache 模块单测）。——

    #[test]
    fn dek_cache_hit_then_reunwrap_after_rotation() {
        let (be, domain) = provisioned();
        let aad = row_aad(uuid::Uuid::from_u128(0xE1));
        let env = poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"old")).unwrap();
        let old_dek = dek_id_of(&env);
        // 轮换后旧密文仍可解（旧 DEK 转 RETIRING，继续解旧密文）。
        let report = be.rotate(domain, KeyPurpose::Field).unwrap();
        assert_eq!(report.retiring_data_key_id, old_dek);
        assert_eq!(
            poll_once(be.unwrap(KeyDomainId(domain), &aad, &env)).unwrap(),
            b"old"
        );
        // 新密文引用新 DEK，版本号 2。
        let env2 =
            poll_once(be.wrap(KeyDomainId(domain), KeyPurpose::Field, &aad, b"new")).unwrap();
        assert_ne!(dek_id_of(&env2), old_dek);
        assert_eq!(&env2.as_bytes()[21..23], &[0x00, 0x02]);
    }
}
