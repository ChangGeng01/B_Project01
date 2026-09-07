//! 历史 master.key 载体与内存主密钥类型。
//!
//! F-57 默认与发布构建只编译内存类型，不编译磁盘加载入口。只有显式
//! `legacy-master-key-file` 的 Unix development/test debug 构建保留 0400/属主/长度
//! 校验，用于迁移期测试；它不是生产 provider。

#[cfg(all(feature = "legacy-master-key-file", debug_assertions, unix))]
use ep_foundation::error::codes::PLATFORM_SYSTEM_NOT_READY;
#[cfg(all(feature = "legacy-master-key-file", debug_assertions, unix))]
use ep_foundation::AppError;

/// 主密钥定长 32 字节（AES-256 键）。
pub const MASTER_KEY_LEN: usize = 32;

/// 主密钥字节。不实现 `Debug`、`Display` 与 `Clone`，`Drop` 时清零。
pub struct MasterKey {
    bytes: [u8; MASTER_KEY_LEN],
}

impl MasterKey {
    // 默认构建没有磁盘构造入口；载体单测只通过 crate 内存入口构造。
    #[cfg(any(test, all(feature = "legacy-master-key-file", debug_assertions, unix)))]
    pub(crate) fn new(bytes: [u8; MASTER_KEY_LEN]) -> Self {
        Self { bytes }
    }

    /// 只限载体内部取用，不出载体。
    pub(crate) fn bytes(&self) -> &[u8; MASTER_KEY_LEN] {
        &self.bytes
    }
}

impl Drop for MasterKey {
    fn drop(&mut self) {
        self.bytes.fill(0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

/// 启动校验的纯判定面，供加载器与单元测试共用：
/// 权限必须恰为 0400，属主必须等于期望 uid，长度必须恰为 32 字节。
#[cfg(all(feature = "legacy-master-key-file", debug_assertions, unix))]
pub fn verify_master_key_metadata(
    mode_bits: u32,
    owner_uid: u32,
    expected_uid: u32,
    len: usize,
) -> Result<(), AppError> {
    let deny = |why: String| {
        AppError::new(
            PLATFORM_SYSTEM_NOT_READY,
            format!("master.key 拒启动：{why}"),
        )
    };
    if mode_bits & 0o777 != 0o400 {
        return Err(deny(format!("权限 {:04o} 不是 0400", mode_bits & 0o777)));
    }
    if owner_uid != expected_uid {
        return Err(deny(format!(
            "属主 {owner_uid} 与本进程账户 {expected_uid} 不符"
        )));
    }
    if len != MASTER_KEY_LEN {
        return Err(deny(format!("长度 {len} 不是 {MASTER_KEY_LEN} 字节")));
    }
    Ok(())
}

/// 读取并校验历史 master.key。属主校验取进程 uid。
#[cfg(all(feature = "legacy-master-key-file", debug_assertions, unix))]
pub fn load_master_key(path: &std::path::Path) -> Result<MasterKey, AppError> {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::metadata(path).map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_NOT_READY,
            format!("master.key 拒启动：元数据不可读取（{:?}）", e.kind()),
        )
    })?;
    // 期望属主为本进程账户。
    let expected_uid = unsafe { libc::getuid() };
    verify_master_key_metadata(meta.mode(), meta.uid(), expected_uid, meta.len() as usize)?;
    let bytes = std::fs::read(path).map_err(|e| {
        AppError::new(
            PLATFORM_SYSTEM_NOT_READY,
            format!("master.key 拒启动：读文件失败（{e}）"),
        )
    })?;
    let arr: [u8; MASTER_KEY_LEN] = bytes
        .as_slice()
        .try_into()
        .expect("长度已在元数据层校验为 32");
    Ok(MasterKey::new(arr))
}

#[cfg(all(test, feature = "legacy-master-key-file", debug_assertions, unix))]
mod tests {
    use super::*;

    fn tmp_master(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("ep-kms-test-{}-{}", std::process::id(), name));
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("master.key")
    }

    #[test]
    fn metadata_gate_matrix() {
        // 合格：0400、属主相符、32 字节。
        assert!(verify_master_key_metadata(0o100400, 1000, 1000, 32).is_ok());
        // 权限放宽即拒。
        assert!(verify_master_key_metadata(0o100440, 1000, 1000, 32).is_err());
        assert!(verify_master_key_metadata(0o100600, 1000, 1000, 32).is_err());
        assert!(verify_master_key_metadata(0o100644, 1000, 1000, 32).is_err());
        // 属主不符即拒。
        assert!(verify_master_key_metadata(0o100400, 999, 1000, 32).is_err());
        // 长度不符即拒。
        assert!(verify_master_key_metadata(0o100400, 1000, 1000, 31).is_err());
        let err = verify_master_key_metadata(0o100644, 1000, 1000, 32).unwrap_err();
        assert_eq!(err.code, PLATFORM_SYSTEM_NOT_READY);
    }

    #[test]
    fn load_rejects_wrong_permissions_on_disk() {
        use std::os::unix::fs::PermissionsExt;
        let path = tmp_master("perm");
        std::fs::write(&path, [7u8; 32]).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(load_master_key(&path).is_err());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_accepts_strict_permissions_on_disk() {
        use std::os::unix::fs::PermissionsExt;
        let path = tmp_master("ok");
        std::fs::write(&path, [9u8; 32]).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400)).unwrap();
        let key = load_master_key(&path).expect("0400 且属主为本进程账户应通过");
        assert_eq!(key.bytes(), &[9u8; 32]);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_rejects_wrong_length_on_disk() {
        use std::os::unix::fs::PermissionsExt;
        let path = tmp_master("len");
        std::fs::write(&path, [1u8; 16]).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400)).unwrap();
        assert!(load_master_key(&path).is_err());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn load_rejects_missing_file() {
        let path = std::env::temp_dir().join("ep-kms-test-no-such-dir/master.key");
        assert!(load_master_key(&path).is_err());
    }

    #[test]
    fn missing_legacy_master_key_diagnostics_redact_secret_path() {
        let path = std::path::Path::new("/private/tmp/ABSOLUTE_MASTER_KEY_MARKER/missing.key");
        let error = match load_master_key(path) {
            Err(error) => error,
            Ok(_) => panic!("missing key must fail closed"),
        };
        assert_eq!(error.code, PLATFORM_SYSTEM_NOT_READY);
        for rendered in [error.to_string(), format!("{error:?}")] {
            assert!(
                !rendered.contains("ABSOLUTE_MASTER_KEY_MARKER"),
                "{rendered}"
            );
            assert!(!rendered.contains("/private/tmp"), "{rendered}");
        }
    }
}
