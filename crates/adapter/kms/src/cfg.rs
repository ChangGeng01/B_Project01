//! 配置键解析（02 计划第 7 节与规格报告第 5 节配置面）。
//!
//! 本 crate 只依赖 foundation，不经 runtime 配置段；键值直接取自环境变量，
//! 语义与 `EP__` 前缀体系一致。标注「热生效」的键在每次使用时重读，
//! 改键不重启进程。
//!
//! 注意：`EP__KMS__*` 与 `EP__CRYPTO__BLIND_INDEX__BYTES` 在 runtime 的
//! `sections.rs` 尚无对应段（规格报告第 8 节待决六），集成任务对齐时以本文件
//! 的键名与默认值为实现侧出处。

use ep_foundation::error::codes::PLATFORM_REQUEST_INVALID_PAYLOAD;
use ep_foundation::AppError;

/// 载体选择键：builtin（默认）或 hsm。
pub const ENV_BACKEND: &str = "EP__KMS__BACKEND";
/// master.key 路径键。
pub const ENV_MASTER_KEY_PATH: &str = "EP__KMS__BUILTIN__MASTER_KEY_PATH";
/// master.key 默认路径。
pub const DEFAULT_MASTER_KEY_PATH: &str = "/var/lib/ep/kms/master.key";
/// DEK 缓存条数上限键（热生效）。
pub const ENV_DEK_CACHE_MAX_ENTRIES: &str = "EP__KMS__DEK_CACHE__MAX_ENTRIES";
/// DEK 缓存存活秒数键（热生效）。
pub const ENV_DEK_CACHE_TTL_S: &str = "EP__KMS__DEK_CACHE__TTL_S";
/// DEK 缓存条数默认。
pub const DEK_CACHE_MAX_ENTRIES_DEFAULT: usize = 512;
/// DEK 缓存存活秒数默认。
pub const DEK_CACHE_TTL_S_DEFAULT: u64 = 300;
/// 盲索引字节宽度键（热生效）。
pub const ENV_BLIND_INDEX_BYTES: &str = "EP__CRYPTO__BLIND_INDEX__BYTES";
/// 盲索引字节宽度默认。
pub const BLIND_INDEX_BYTES_DEFAULT: usize = 16;
/// HSM PKCS#11 模块路径键。
pub const ENV_HSM_PKCS11_MODULE: &str = "EP__KMS__HSM__PKCS11_MODULE";
/// HSM 槽位键。
pub const ENV_HSM_PKCS11_SLOT: &str = "EP__KMS__HSM__PKCS11_SLOT";
/// HSM PIN 引用键：只写机密引用，不落字面 PIN。
pub const ENV_HSM_PKCS11_PIN_REF: &str = "EP__KMS__HSM__PKCS11_PIN_REF";

/// 载体二选一。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KmsBackendKind {
    /// 内置 KMS：master.key 主密钥加进程内 DEK 缓存。
    Builtin,
    /// 客户 HSM：PKCS#11 载体，`hsm` feature 门控。
    Hsm,
}

/// 解析载体选择。缺省取 builtin；取值不在白名单返校验错误。
pub fn parse_backend_kind(raw: Option<&str>) -> Result<KmsBackendKind, AppError> {
    match raw {
        None | Some("") | Some("builtin") => Ok(KmsBackendKind::Builtin),
        Some("hsm") => Ok(KmsBackendKind::Hsm),
        Some(other) => Err(AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("EP__KMS__BACKEND 取值 {other} 不在白名单 builtin|hsm"),
        )),
    }
}

/// 盲索引字节宽度的纯解析面。**宽度是待决项，不得视为冻结结论**：
/// 端口 `BlindIndex` 现取 16 字节，02 计划第 11 节假设三要求例外列取 32，
/// 两处合一之前本函数只认 16 与 32 两取值。
pub fn parse_blind_index_bytes(raw: Option<&str>) -> Result<usize, AppError> {
    match raw {
        None | Some("") => Ok(BLIND_INDEX_BYTES_DEFAULT),
        Some(v) => match v.parse::<usize>() {
            Ok(n @ (16 | 32)) => Ok(n),
            _ => Err(AppError::new(
                PLATFORM_REQUEST_INVALID_PAYLOAD,
                format!("{ENV_BLIND_INDEX_BYTES} 取值 {v} 非法，只认 16 或 32"),
            )),
        },
    }
}

/// DEK 缓存配置的纯解析面，供热生效读取与测试共用。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DekCacheCfg {
    pub max_entries: usize,
    pub ttl_s: u64,
}

impl DekCacheCfg {
    pub const fn defaults() -> DekCacheCfg {
        DekCacheCfg {
            max_entries: DEK_CACHE_MAX_ENTRIES_DEFAULT,
            ttl_s: DEK_CACHE_TTL_S_DEFAULT,
        }
    }

    /// 非法取值回落默认并留注记，不因一个缓存参数拒绝服务。
    pub fn parse(max_entries: Option<&str>, ttl_s: Option<&str>) -> DekCacheCfg {
        DekCacheCfg {
            max_entries: max_entries
                .and_then(|v| v.parse().ok())
                .filter(|n: &usize| *n > 0)
                .unwrap_or(DEK_CACHE_MAX_ENTRIES_DEFAULT),
            ttl_s: ttl_s
                .and_then(|v| v.parse().ok())
                .filter(|n: &u64| *n > 0)
                .unwrap_or(DEK_CACHE_TTL_S_DEFAULT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_kind_whitelist() {
        assert_eq!(parse_backend_kind(None).unwrap(), KmsBackendKind::Builtin);
        assert_eq!(
            parse_backend_kind(Some("builtin")).unwrap(),
            KmsBackendKind::Builtin
        );
        assert_eq!(
            parse_backend_kind(Some("hsm")).unwrap(),
            KmsBackendKind::Hsm
        );
        assert!(parse_backend_kind(Some("aws")).is_err());
    }

    #[test]
    fn blind_index_bytes_only_sixteen_or_thirty_two() {
        assert_eq!(parse_blind_index_bytes(None).unwrap(), 16);
        assert_eq!(parse_blind_index_bytes(Some("16")).unwrap(), 16);
        assert_eq!(parse_blind_index_bytes(Some("32")).unwrap(), 32);
        assert!(parse_blind_index_bytes(Some("24")).is_err());
        assert!(parse_blind_index_bytes(Some("abc")).is_err());
    }

    #[test]
    fn dek_cache_cfg_falls_back_on_garbage() {
        let cfg = DekCacheCfg::parse(Some("64"), Some("60"));
        assert_eq!(cfg.max_entries, 64);
        assert_eq!(cfg.ttl_s, 60);
        let cfg = DekCacheCfg::parse(Some("垃圾"), Some("0"));
        assert_eq!(cfg, DekCacheCfg::defaults());
    }
}
