//! 客户 HSM 载体（PKCS#11），`hsm` feature 门控。
//!
//! 首版为占位实现：三配置键（`EP__KMS__HSM__PKCS11_MODULE`、
//! `EP__KMS__HSM__PKCS11_SLOT`、`EP__KMS__HSM__PKCS11_PIN_REF`）只做解析与
//! 错误路径，真实 PKCS#11 调用不接；`KmsBackend` 六方法一律按降级返回
//! 已登记的 `PLATFORM.SYSTEM.NOT_READY`，不新增错误码。
//!
//! PIN 只写机密引用（`secret://` 前缀），绝不落字面口令；配置解析失败
//! 属输入校验，返 `PLATFORM.REQUEST.INVALID_PAYLOAD`。

use ep_foundation::error::codes::{PLATFORM_REQUEST_INVALID_PAYLOAD, PLATFORM_SYSTEM_NOT_READY};
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::kms::{
    Aad, BlindIndex, CipherEnvelope, KeyDomainId, KeyPurpose, KeyRef, KmsBackend, Signature,
};
use ep_foundation::AppError;

use crate::cfg::{ENV_HSM_PKCS11_MODULE, ENV_HSM_PKCS11_PIN_REF, ENV_HSM_PKCS11_SLOT};

/// PKCS#11 三键配置。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HsmPkcs11Config {
    /// PKCS#11 模块动态库路径。
    pub module: String,
    /// 槽位标识。
    pub slot: String,
    /// PIN 的机密引用，形如 `secret://kms/hsm-pin`，不接受字面口令。
    pub pin_ref: String,
}

impl HsmPkcs11Config {
    /// 从环境变量解析。缺键、空值或 PIN 非引用形态一律拒绝。
    pub fn from_env() -> Result<HsmPkcs11Config, AppError> {
        Self::parse(
            std::env::var(ENV_HSM_PKCS11_MODULE).ok().as_deref(),
            std::env::var(ENV_HSM_PKCS11_SLOT).ok().as_deref(),
            std::env::var(ENV_HSM_PKCS11_PIN_REF).ok().as_deref(),
        )
    }

    /// 纯解析面，供测试与装配共用。
    pub fn parse(
        module: Option<&str>,
        slot: Option<&str>,
        pin_ref: Option<&str>,
    ) -> Result<HsmPkcs11Config, AppError> {
        let module = required(module, ENV_HSM_PKCS11_MODULE)?;
        let slot = required(slot, ENV_HSM_PKCS11_SLOT)?;
        let pin_ref = required(pin_ref, ENV_HSM_PKCS11_PIN_REF)?;
        if !pin_ref.starts_with("secret://") {
            return Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("{ENV_HSM_PKCS11_PIN_REF} 必须是 secret:// 机密引用，不接受字面口令"),
            ));
        }
        Ok(HsmPkcs11Config {
            module,
            slot,
            pin_ref,
        })
    }
}

fn required(raw: Option<&str>, key: &str) -> Result<String, AppError> {
    match raw.map(str::trim).filter(|v| !v.is_empty()) {
        Some(v) => Ok(v.to_string()),
        Option::None => Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("{key} 缺失或为空"),
        )),
    }
}

/// HSM 载体。配置在构造期校验完毕；运行期方法一律降级。
pub struct HsmKmsBackend {
    config: HsmPkcs11Config,
}

impl HsmKmsBackend {
    pub fn new(config: HsmPkcs11Config) -> HsmKmsBackend {
        HsmKmsBackend { config }
    }

    pub fn from_env() -> Result<HsmKmsBackend, AppError> {
        Ok(HsmKmsBackend::new(HsmPkcs11Config::from_env()?))
    }

    pub fn config(&self) -> &HsmPkcs11Config {
        &self.config
    }

    /// 全部方法共用的降级出口。
    fn degraded(operation: &str) -> AppError {
        AppError::new(
            PLATFORM_SYSTEM_NOT_READY,
            format!("HSM 载体未接入真实 PKCS#11 调用，{operation}按降级处理"),
        )
    }
}

#[async_trait::async_trait]
impl KmsBackend for HsmKmsBackend {
    async fn wrap(
        &self,
        _domain: KeyDomainId,
        _purpose: KeyPurpose,
        _aad: &Aad,
        _plaintext: &[u8],
    ) -> Result<CipherEnvelope, AppError> {
        Err(Self::degraded("信封加密"))
    }

    async fn unwrap(
        &self,
        _domain: KeyDomainId,
        _aad: &Aad,
        _envelope: &CipherEnvelope,
    ) -> Result<Vec<u8>, AppError> {
        Err(Self::degraded("信封解密"))
    }

    async fn derive_blind_key(
        &self,
        _legal_entity_id: Id<LegalEntity>,
        _column_fqn: &str,
        _plaintext: &[u8],
    ) -> Result<BlindIndex, AppError> {
        Err(Self::degraded("盲索引派生"))
    }

    async fn sign(&self, _key: &KeyRef, _payload: &[u8]) -> Result<Signature, AppError> {
        Err(Self::degraded("签名"))
    }

    async fn verify(
        &self,
        _key: &KeyRef,
        _payload: &[u8],
        _signature: &Signature,
    ) -> Result<bool, AppError> {
        Err(Self::degraded("验签"))
    }

    async fn health(&self) -> Result<(), AppError> {
        Err(Self::degraded("健康自检"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    struct NowWake;
    impl Wake for NowWake {
        fn wake(self: Arc<Self>) {}
    }

    /// 降级路径的 future 首次 poll 即就绪，单次驱动即可取结果，
    /// 不为一个占位载体引入运行时依赖。
    fn poll_once<F: Future>(fut: F) -> F::Output {
        let waker = Waker::from(Arc::new(NowWake));
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(out) => out,
            Poll::Pending => panic!("降级 future 不应挂起"),
        }
    }

    fn backend() -> HsmKmsBackend {
        HsmKmsBackend::new(
            HsmPkcs11Config::parse(
                Some("/usr/lib/pkcs11.so"),
                Some("0"),
                Some("secret://kms/hsm-pin"),
            )
            .expect("测试配置合法"),
        )
    }

    #[test]
    fn config_parse_paths() {
        let ok = HsmPkcs11Config::parse(
            Some("/usr/lib/pkcs11.so"),
            Some("0"),
            Some("secret://kms/hsm-pin"),
        )
        .expect("三键齐备且 PIN 为引用");
        assert_eq!(ok.slot, "0");
        // 缺键拒。
        assert!(HsmPkcs11Config::parse(Option::None, Some("0"), Some("secret://x")).is_err());
        // 空值拒。
        assert!(HsmPkcs11Config::parse(Some("  "), Some("0"), Some("secret://x")).is_err());
        // 字面口令拒。
        let err = HsmPkcs11Config::parse(Some("m"), Some("0"), Some("123456")).unwrap_err();
        assert_eq!(err.code, PLATFORM_REQUEST_INVALID_PAYLOAD);
    }

    #[test]
    fn all_six_methods_degrade_to_not_ready() {
        let be = backend();
        let domain = KeyDomainId(uuid::Uuid::nil());
        let aad = Aad::new(vec![0; 48]);
        let env = CipherEnvelope::new(vec![0; 51]);
        let le = Id::<LegalEntity>::from_uuid(uuid::Uuid::nil());
        let key = KeyRef::new("kms://hsm/slot0/le/x");
        let sig = Signature::new(vec![0; 64]);
        let blind_err = match poll_once(be.derive_blind_key(le, "s.t.c", b"x")) {
            Err(e) => e,
            Ok(_) => panic!("降级路径不应成功"),
        };
        let results: Vec<AppError> = vec![
            poll_once(be.wrap(domain, KeyPurpose::Field, &aad, b"x")).unwrap_err(),
            poll_once(be.unwrap(domain, &aad, &env)).unwrap_err(),
            blind_err,
            poll_once(be.sign(&key, b"x")).unwrap_err(),
            poll_once(be.verify(&key, b"x", &sig)).unwrap_err(),
            poll_once(be.health()).unwrap_err(),
        ];
        assert_eq!(results.len(), 6);
        for e in &results {
            assert_eq!(e.code, PLATFORM_SYSTEM_NOT_READY);
        }
    }
}
