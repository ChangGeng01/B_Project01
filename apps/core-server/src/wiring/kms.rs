//! 密钥后端的装配（02 计划 §7：`EP__KMS__BACKEND` 取 builtin 或 hsm）。
//!
//! F-57 生产需要 TPM/HSM/KMS non-exportable wrapping handle；该 provider 尚未交付，
//! 因而默认 `builtin` 与 `hsm` 都稳定失败关闭。历史 POSIX master.key 只允许显式
//! `legacy-file` development/test debug 构建，默认与发布构建不编译磁盘读取入口。

use std::path::Path;
use std::sync::Arc;

use ep_adapter_kms::BuiltinKmsBackend;
use ep_platform_runtime::config::KmsCfg;

/// 按配置构造密钥后端。失败原因以文本上抛，由调用方决定退出或降级。
pub fn build_kms_backend(
    kms: &KmsCfg,
    _secrets_dir: &Path,
) -> Result<Arc<BuiltinKmsBackend>, String> {
    if !kms.builtin.master_key_path.as_os_str().is_empty() {
        #[cfg(all(feature = "legacy-file", debug_assertions, unix))]
        if kms.backend == "builtin" {
            return BuiltinKmsBackend::new(&kms.builtin.master_key_path)
                .map(Arc::new)
                .map_err(|e| format!("历史开发密钥后端装配失败：{}", e.message));
        }
        return Err(
            "已废弃的 kms.builtin.master_key_path 只允许显式 legacy-file development/test debug 构建"
                .into(),
        );
    }

    match kms.backend.as_str() {
        "builtin" => Err(
            "NOT_IMPLEMENTED：F-57 non-exportable wrapping-handle KMS 尚未交付，禁止回退普通 master.key"
                .into(),
        ),
        "hsm" => Err("NOT_IMPLEMENTED：F-57 HSM wrapping-handle provider 尚未交付".into()),
        other => Err(format!("密钥后端 {other} 在本构建中不可用")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_runtime::config::{KmsBuiltinCfg, KmsCfg};

    // 负样例断言的是「未知后端名不得回落」这条规则本身。
    #[test]
    fn an_unknown_backend_is_a_failure_not_a_fallback() {
        let kms = KmsCfg {
            backend: "cloud".into(),
            ..KmsCfg::default()
        };
        assert!(build_kms_backend(&kms, Path::new("/tmp")).is_err());
    }

    #[test]
    fn a_missing_master_key_file_is_a_failure() {
        let kms = KmsCfg {
            builtin: KmsBuiltinCfg {
                master_key_path: Path::new("/nonexistent/master.key").to_path_buf(),
            },
            ..KmsCfg::default()
        };
        assert!(build_kms_backend(&kms, Path::new("/tmp")).is_err());
    }

    #[test]
    fn default_builtin_backend_is_explicitly_not_implemented() {
        let error = match build_kms_backend(&KmsCfg::default(), Path::new("/tmp")) {
            Err(error) => error,
            Ok(_) => panic!("F-57 non-exportable wrapping handle 尚未交付，默认不得读普通文件"),
        };
        assert!(error.contains("NOT_IMPLEMENTED"), "{error}");
    }

    #[cfg(all(feature = "legacy-file", debug_assertions, unix))]
    #[test]
    fn missing_legacy_master_key_path_is_redacted_through_assembly() {
        let kms = KmsCfg {
            builtin: KmsBuiltinCfg {
                master_key_path: Path::new("/private/tmp/ABSOLUTE_MASTER_KEY_MARKER/missing.key")
                    .to_path_buf(),
            },
            ..KmsCfg::default()
        };
        let error = match build_kms_backend(&kms, Path::new("/tmp")) {
            Err(error) => error,
            Ok(_) => panic!("missing key must fail closed"),
        };
        assert!(!error.contains("ABSOLUTE_MASTER_KEY_MARKER"), "{error}");
        assert!(!error.contains("/private/tmp"), "{error}");
    }
}
