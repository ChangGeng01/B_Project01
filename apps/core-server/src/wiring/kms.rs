//! 密钥后端的装配（02 计划 §7：`EP__KMS__BACKEND` 取 builtin 或 hsm）。
//!
//! 首版只装配内置 KMS：主密钥文件权限 0400 且属主为本进程账户的校验
//! 在 `BuiltinKmsBackend::new` 内部完成，不合规即拒。`hsm` 取值待
//! 客户提供硬件密码机后按同一配置位点切换（02 计划 §11 预留三），
//! 本阶段配置写 `hsm` 一律视为装配失败——不以半个实现顶位。

use std::path::Path;
use std::sync::Arc;

use ep_adapter_kms::BuiltinKmsBackend;
use ep_platform_runtime::config::KmsCfg;

/// 按配置构造密钥后端。失败原因以文本上抛，由调用方决定退出或降级。
pub fn build_kms_backend(
    kms: &KmsCfg,
    _secrets_dir: &Path,
) -> Result<Arc<BuiltinKmsBackend>, String> {
    match kms.backend.as_str() {
        "builtin" => BuiltinKmsBackend::new(&kms.builtin.master_key_path)
            .map(Arc::new)
            .map_err(|e| format!("内置密钥后端装配失败：{}", e.message)),
        // hsm 载体按 feature 门控交付；未启用 feature 即装配失败，
        // 绝不回落 builtin——静默回落会让主密钥保护形态与配置声明不符。
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
}
