//! ep-adapter-kms — KMS 载体层：EPC1 信封加密、受治理盲索引、密钥域状态机
//! 与两个载体后端（内置 builtin / 客户 HSM，后者经 `hsm` feature 门控）。
//!
//! 出处：02 计划第 4 节（密钥域、信封、盲索引、销毁核验）与规格报告第 5 节。
//! 只实现 foundation 冻结的 [`ep_foundation::port::kms::KmsBackend`] 一个公开
//! trait；密钥域与数据密钥的管理面以载体固有方法提供，不声明端口之外的
//! 任何公开 trait。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节（adapter 只依赖 foundation 与第三方）。

pub mod builtin;
pub mod cfg;
pub mod envelope;
pub mod masterkey;
pub mod material;
pub mod normalize;

pub(crate) mod cache;

#[cfg(feature = "hsm")]
pub mod hsm;

// —— 关键公开面的再导出，供装配层按 crate 根取用。——

pub use builtin::{BuiltinKmsBackend, RotationReport};
pub use envelope::aad_for_row;
pub use masterkey::{load_master_key, verify_master_key_metadata, MasterKey, MASTER_KEY_LEN};
pub use material::{
    DataKey, DataKeyState, DekAlgorithm, DestroyApproval, DestroyEvidence, DomainKind, KeyDomain,
    KeyDomainState,
};
pub use normalize::{normalize, Normalization};

#[cfg(feature = "hsm")]
pub use hsm::{HsmKmsBackend, HsmPkcs11Config};
