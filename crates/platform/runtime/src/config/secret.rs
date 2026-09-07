//! 机密的两个承载类型。
//!
//! 配置里只写引用，不写机密本身；解引用的结果包在 [`SecretString`] 里。
//! [`SecretString`] 故意不实现 Debug 与 Display——实现了就一定会有人
//! 在排障时把它打进日志，而日志是可轮转可外发的。

use std::fmt;

use serde::Deserialize;

/// `secret://<domain>/<name>#<version>` 形态的机密引用。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SecretRef(String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SecretRefError(&'static str);

impl fmt::Display for SecretRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "机密引用格式非法：{}", self.0)
    }
}

impl std::error::Error for SecretRefError {}

fn is_win32_device_basename(value: &str) -> bool {
    let basename = value
        .split('.')
        .next()
        .unwrap_or(value)
        .to_ascii_lowercase();
    matches!(basename.as_str(), "con" | "prn" | "aux" | "nul")
        || (basename.len() == 4
            && (basename.starts_with("com") || basename.starts_with("lpt"))
            && matches!(basename.as_bytes()[3], b'1'..=b'9'))
}

impl SecretRef {
    pub fn parse(raw: &str) -> Result<SecretRef, SecretRefError> {
        if raw.len() > 512 {
            return Err(SecretRefError("引用超过 512 bytes"));
        }
        let Some(rest) = raw.strip_prefix("secret://") else {
            // 不得把 raw 放进错误：这条分支最可能收到误填的明文口令。
            return Err(SecretRefError("缺 secret:// 前缀"));
        };
        let Some((path, version)) = rest.split_once('#') else {
            return Err(SecretRefError("缺 # 版本段"));
        };
        let components: Vec<_> = path.split('/').collect();
        let valid_component = |value: &str| {
            let bytes = value.as_bytes();
            !bytes.is_empty()
                && bytes.len() <= 64
                && (bytes[0].is_ascii_lowercase() || bytes[0].is_ascii_digit())
                && !value.ends_with('.')
                && value.bytes().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')
                })
                && !is_win32_device_basename(value)
        };
        let valid_version = (1..=10).contains(&version.len())
            && version.as_bytes()[0].is_ascii_digit()
            && version.as_bytes()[0] != b'0'
            && version.bytes().all(|b| b.is_ascii_digit())
            && version
                .parse::<u32>()
                .is_ok_and(|value| value <= i32::MAX as u32);
        if !(2..=8).contains(&components.len())
            || components.iter().any(|part| !valid_component(part))
            || !valid_version
        {
            return Err(SecretRefError("不是规范机密引用"));
        }
        Ok(SecretRef(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SecretRef {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        SecretRef::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// 解引用之后的机密取值。不实现 Clone、Debug、Display 与 Serialize。
///
/// ```compile_fail
/// use ep_platform_runtime::config::SecretString;
/// let secret = SecretString::new("sensitive");
/// let copied = secret.clone();
/// drop(copied);
/// ```
pub struct SecretString(zeroize::Zeroizing<String>);

impl SecretString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(zeroize::Zeroizing::new(value.into()))
    }

    /// 唯一的取出口。调用点应尽量靠近使用处，不做长期持有。
    pub fn expose(&self) -> &str {
        self.0.as_str()
    }

    #[cfg(all(feature = "legacy-file", debug_assertions))]
    fn expose_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[cfg(all(feature = "legacy-file", debug_assertions))]
mod legacy_file {
    use super::{SecretRef, SecretString};
    use std::fmt;

    /// 历史 development/test 文件 provider 的失败。错误故意只含稳定类别，
    /// 不携带秘密根或目标的绝对路径。
    #[cfg(all(feature = "legacy-file", debug_assertions))]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct LegacySecretError(&'static str);

    #[cfg(feature = "legacy-file")]
    impl fmt::Display for LegacySecretError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "legacy file secret 拒绝：{}", self.0)
        }
    }

    #[cfg(feature = "legacy-file")]
    impl std::error::Error for LegacySecretError {}

    #[cfg(feature = "legacy-file")]
    const LEGACY_SECRET_MAX_BYTES: u64 = 64 * 1024;

    /// 仅供显式 `legacy-file` debug/test 构建使用的共享 reader。
    ///
    /// 引用本身先经 [`SecretRef`] 规范化；这里再拒绝秘密根、父目录或目标文件中的
    /// symlink/reparse point，并在打开后按最终文件句柄核对仍位于 canonical 根内。
    /// 所有错误都脱敏，绝不把机器绝对路径带回启动日志。
    #[cfg(feature = "legacy-file")]
    pub fn resolve_legacy_file_secret(
        root: &std::path::Path,
        reference: &SecretRef,
    ) -> Result<SecretString, LegacySecretError> {
        use std::io::Read;

        if !root.is_absolute() {
            return Err(LegacySecretError("秘密根必须是绝对路径"));
        }
        reject_path_alias(root)?;
        let canonical_root =
            std::fs::canonicalize(root).map_err(|_| LegacySecretError("秘密根不存在或无法解析"))?;
        if !canonical_root.is_dir() {
            return Err(LegacySecretError("秘密根不是目录"));
        }

        let relative = reference
            .as_str()
            .strip_prefix("secret://")
            .ok_or(LegacySecretError("机密引用格式非法"))?;
        let target = canonical_root.join(relative);
        reject_path_components(&canonical_root, &target)?;

        let mut file = open_legacy_secret_file(&target)?;
        let metadata = file
            .metadata()
            .map_err(|_| LegacySecretError("无法读取机密文件属性"))?;
        if !metadata.is_file()
            || metadata_is_reparse_point(&metadata)
            || file_has_multiple_links(&file, &metadata)?
            || metadata.len() > LEGACY_SECRET_MAX_BYTES
        {
            return Err(LegacySecretError("机密目标必须是有界普通文件"));
        }

        let opened_path = final_path_from_handle(&file)?;
        if !path_is_same_or_child(&opened_path, &canonical_root) {
            return Err(LegacySecretError("打开后的机密目标逃逸秘密根"));
        }
        // 打开前后各检查一次，配合最终句柄 containment，拒绝常见的链接交换旁路。
        reject_path_components(&canonical_root, &target)?;

        let mut raw = SecretString::new(String::new());
        file.by_ref()
            .take(LEGACY_SECRET_MAX_BYTES + 1)
            .read_to_string(raw.expose_mut())
            .map_err(|_| LegacySecretError("机密文件不是有界 UTF-8 文本"))?;
        if raw.expose().len() as u64 > LEGACY_SECRET_MAX_BYTES {
            return Err(LegacySecretError("机密文件超过硬上限"));
        }
        let trimmed = raw.expose().trim();
        if trimmed.is_empty() {
            return Err(LegacySecretError("机密正文为空"));
        }
        Ok(SecretString::new(trimmed))
    }

    #[cfg(all(feature = "legacy-file", debug_assertions, unix))]
    fn open_legacy_secret_file(path: &std::path::Path) -> Result<std::fs::File, LegacySecretError> {
        use std::os::unix::fs::OpenOptionsExt;

        std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)
            .map_err(|_| LegacySecretError("机密文件不存在、不可读或为链接"))
    }

    #[cfg(all(feature = "legacy-file", windows))]
    fn open_legacy_secret_file(path: &std::path::Path) -> Result<std::fs::File, LegacySecretError> {
        use std::os::windows::fs::OpenOptionsExt;

        // FILE_FLAG_OPEN_REPARSE_POINT：打开链接本身而不是跟随最终 reparse target，
        // 随后的句柄 metadata 检查会拒绝该对象。
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(|_| LegacySecretError("机密文件不存在、不可读或为重解析点"))
    }

    #[cfg(feature = "legacy-file")]
    fn reject_path_alias(path: &std::path::Path) -> Result<(), LegacySecretError> {
        let metadata = std::fs::symlink_metadata(path)
            .map_err(|_| LegacySecretError("路径组成不存在或不可检查"))?;
        if metadata.file_type().is_symlink() || metadata_is_reparse_point(&metadata) {
            return Err(LegacySecretError("秘密路径不得包含链接或重解析点"));
        }
        Ok(())
    }

    #[cfg(feature = "legacy-file")]
    fn reject_path_components(
        root: &std::path::Path,
        target: &std::path::Path,
    ) -> Result<(), LegacySecretError> {
        let relative = target
            .strip_prefix(root)
            .map_err(|_| LegacySecretError("机密目标不在秘密根内"))?;
        let mut current = root.to_path_buf();
        reject_path_alias(&current)?;
        for component in relative.components() {
            current.push(component);
            reject_path_alias(&current)?;
        }
        Ok(())
    }

    #[cfg(all(feature = "legacy-file", windows))]
    fn metadata_is_reparse_point(metadata: &std::fs::Metadata) -> bool {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }

    #[cfg(all(feature = "legacy-file", not(windows)))]
    fn metadata_is_reparse_point(_metadata: &std::fs::Metadata) -> bool {
        false
    }

    #[cfg(all(feature = "legacy-file", unix))]
    fn file_has_multiple_links(
        _file: &std::fs::File,
        metadata: &std::fs::Metadata,
    ) -> Result<bool, LegacySecretError> {
        use std::os::unix::fs::MetadataExt;
        Ok(metadata.nlink() != 1)
    }

    #[cfg(all(feature = "legacy-file", windows))]
    fn file_has_multiple_links(
        file: &std::fs::File,
        _metadata: &std::fs::Metadata,
    ) -> Result<bool, LegacySecretError> {
        use std::ffi::c_void;
        use std::os::windows::io::AsRawHandle;

        #[repr(C)]
        struct FileTime {
            low_date_time: u32,
            high_date_time: u32,
        }

        #[repr(C)]
        struct ByHandleFileInformation {
            file_attributes: u32,
            creation_time: FileTime,
            last_access_time: FileTime,
            last_write_time: FileTime,
            volume_serial_number: u32,
            file_size_high: u32,
            file_size_low: u32,
            number_of_links: u32,
            file_index_high: u32,
            file_index_low: u32,
        }

        unsafe extern "system" {
            fn GetFileInformationByHandle(
                file: *mut c_void,
                information: *mut ByHandleFileInformation,
            ) -> i32;
        }

        // SAFETY: 该 Win32 输出结构允许全零初始化；handle 在调用期间由 file 持有。
        let mut information: ByHandleFileInformation = unsafe { std::mem::zeroed() };
        // SAFETY: handle 有效，information 指向完整可写结构。
        let ok = unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut information) };
        if ok == 0 {
            return Err(LegacySecretError("无法核验机密文件链接数"));
        }
        Ok(information.number_of_links != 1)
    }

    #[cfg(all(feature = "legacy-file", target_os = "macos"))]
    fn final_path_from_handle(
        file: &std::fs::File,
    ) -> Result<std::path::PathBuf, LegacySecretError> {
        use std::os::fd::AsRawFd;

        let mut bytes = vec![0i8; libc::PATH_MAX as usize];
        // SAFETY: `bytes` 是可写的 PATH_MAX 缓冲，fd 在调用期间由 `file` 持有。
        let result = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETPATH, bytes.as_mut_ptr()) };
        if result == -1 {
            return Err(LegacySecretError("无法核验已打开机密文件的最终路径"));
        }
        // SAFETY: F_GETPATH 成功时写入以 NUL 结尾的路径，缓冲区长度为 PATH_MAX。
        let path = unsafe { std::ffi::CStr::from_ptr(bytes.as_ptr()) }
            .to_str()
            .map_err(|_| LegacySecretError("最终路径不是 UTF-8"))?;
        Ok(std::path::PathBuf::from(path))
    }

    #[cfg(all(feature = "legacy-file", target_os = "linux"))]
    fn final_path_from_handle(
        file: &std::fs::File,
    ) -> Result<std::path::PathBuf, LegacySecretError> {
        use std::os::fd::AsRawFd;

        std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd()))
            .map_err(|_| LegacySecretError("无法核验已打开机密文件的最终路径"))
    }

    #[cfg(all(feature = "legacy-file", windows))]
    fn final_path_from_handle(
        file: &std::fs::File,
    ) -> Result<std::path::PathBuf, LegacySecretError> {
        use std::ffi::c_void;
        use std::os::windows::ffi::OsStringExt;
        use std::os::windows::io::AsRawHandle;

        unsafe extern "system" {
            fn GetFinalPathNameByHandleW(
                file: *mut c_void,
                path: *mut u16,
                path_len: u32,
                flags: u32,
            ) -> u32;
        }

        let mut wide = vec![0u16; 32_768];
        // SAFETY: handle 在调用期间有效，wide 是声明长度的可写 UTF-16 缓冲。
        let length = unsafe {
            GetFinalPathNameByHandleW(
                file.as_raw_handle(),
                wide.as_mut_ptr(),
                wide.len() as u32,
                0,
            )
        };
        if length == 0 || length as usize >= wide.len() {
            return Err(LegacySecretError("无法核验已打开机密文件的最终路径"));
        }
        // 保留 Windows 路径的原始 UTF-16（含可能的 unpaired surrogate），不得用
        // lossy 字符串把两个不同路径折叠成同一 containment spelling。默认 flags
        // 返回与 std::fs::canonicalize 同形的 `\\?\` DOS/UNC normalized 路径。
        Ok(std::path::PathBuf::from(std::ffi::OsString::from_wide(
            &wide[..length as usize],
        )))
    }

    #[cfg(all(
        feature = "legacy-file",
        unix,
        not(any(target_os = "macos", target_os = "linux"))
    ))]
    fn final_path_from_handle(
        _file: &std::fs::File,
    ) -> Result<std::path::PathBuf, LegacySecretError> {
        Err(LegacySecretError("当前平台不支持最终句柄路径核验"))
    }

    #[cfg(all(feature = "legacy-file", windows))]
    pub(super) fn path_is_same_or_child(
        candidate: &std::path::Path,
        root: &std::path::Path,
    ) -> bool {
        let candidate: Vec<_> = candidate.components().collect();
        let root: Vec<_> = root.components().collect();
        candidate.len() >= root.len()
            && candidate.iter().zip(root.iter()).all(|(left, right)| {
                left.as_os_str() == right.as_os_str()
                    || left
                        .as_os_str()
                        .to_str()
                        .zip(right.as_os_str().to_str())
                        .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
            })
    }

    #[cfg(all(feature = "legacy-file", not(windows)))]
    fn path_is_same_or_child(candidate: &std::path::Path, root: &std::path::Path) -> bool {
        candidate.starts_with(root)
    }
}

#[cfg(all(feature = "legacy-file", debug_assertions))]
pub use legacy_file::{resolve_legacy_file_secret, LegacySecretError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn well_formed_reference_is_accepted() {
        assert_eq!(
            SecretRef::parse("secret://db/app_rw#1").unwrap().as_str(),
            "secret://db/app_rw#1"
        );
    }

    // 负样例断言的是「配置里不得出现明文机密」这条规则本身：
    // 任何不带 secret:// 的取值都不得被接受为机密引用。
    #[test]
    fn plaintext_password_is_rejected() {
        assert!(SecretRef::parse("hunter2").is_err());
        assert!(SecretRef::parse("secret://db/app_rw").is_err(), "缺版本段");
        assert!(SecretRef::parse("secret://app_rw#1").is_err(), "缺域");
        assert!(SecretRef::parse("secret://db/app_rw#").is_err(), "版本为空");
    }

    #[test]
    fn invalid_reference_errors_never_echo_the_supplied_secret() {
        for raw in [
            "ULTRA_DISTINCT_PLAINTEXT_PASSWORD_7f83",
            "secret://db/ULTRA_DISTINCT_SECRET_7f83#1",
            "secret://db/ultra_distinct_secret_7f83",
        ] {
            let error = SecretRef::parse(raw).expect_err("非法引用必须拒绝");
            let rendered = error.to_string();
            assert!(!rendered.contains(raw), "错误不得回显原始输入：{rendered}");
            assert!(
                !rendered.contains("7f83"),
                "错误不得泄露输入片段：{rendered}"
            );
        }
    }

    #[test]
    fn reference_cannot_escape_or_alias_the_secret_root() {
        for invalid in [
            "secret:////etc/passwd#1",
            "secret://../db/app_rw#1",
            "secret://db/../kms/hsm_pin#1",
            "secret://db/.#1",
            "secret://db//app_rw#1",
            "secret://db\\app_rw#1",
            "secret://db/app_rw#1#2",
            "secret://db/app_rw#v/1",
            "secret://db/app_rw#1\nnext",
        ] {
            assert!(SecretRef::parse(invalid).is_err(), "必须拒绝 {invalid:?}");
        }
        assert!(SecretRef::parse("secret://kms/totp/user-42#3").is_ok());
    }

    #[test]
    fn reference_requires_one_canonical_ascii_spelling() {
        for invalid in [
            "secret://DB/app_rw#1",
            "secret://db/App_rw#1",
            "secret://db/.hidden#1",
            "secret://db/app_rw.#1",
            "secret://db/app_rw #1",
            "secret://db/app_rw#0",
            "secret://db/app_rw#01",
            "secret://db/app_rw#2147483648",
            "secret://db/app_rw#1.",
            "secret://a/b/c/d/e/f/g/h/i#1",
        ] {
            assert!(SecretRef::parse(invalid).is_err(), "必须拒绝 {invalid:?}");
        }

        assert!(SecretRef::parse("secret://provider/esign/api.v2#2147483647").is_ok());
    }

    #[test]
    fn reference_rejects_win32_device_name_aliases() {
        for invalid in [
            "secret://con/credential#1",
            "secret://db/prn#1",
            "secret://db/aux.txt#1",
            "secret://db/nul.json#1",
            "secret://db/com1#1",
            "secret://db/com9.bin#1",
            "secret://db/lpt1#1",
            "secret://db/lpt9.txt#1",
            "secret://db/CON.txt#1",
        ] {
            assert!(SecretRef::parse(invalid).is_err(), "必须拒绝 {invalid:?}");
        }

        assert!(SecretRef::parse("secret://db/com0#1").is_ok());
        assert!(SecretRef::parse("secret://db/com10#1").is_ok());
        assert!(SecretRef::parse("secret://db/console#1").is_ok());
    }

    #[cfg(all(feature = "legacy-file", debug_assertions))]
    #[test]
    fn legacy_file_resolver_returns_a_secret_without_exposing_its_absolute_path() {
        let root = std::env::temp_dir().join(format!(
            "ep-legacy-secret-{}-{}",
            std::process::id(),
            uuid::Uuid::now_v7()
        ));
        std::fs::create_dir_all(root.join("db")).unwrap();
        std::fs::write(root.join("db/app_rw#1"), "s3cret\n").unwrap();
        let reference = SecretRef::parse("secret://db/app_rw#1").unwrap();

        let value = resolve_legacy_file_secret(&root, &reference).expect("受控普通文件应可读取");
        assert_eq!(value.expose(), "s3cret");

        let missing = SecretRef::parse("secret://db/missing#1").unwrap();
        let error = match resolve_legacy_file_secret(&root, &missing) {
            Err(error) => error,
            Ok(_) => panic!("缺失文件必须失败"),
        };
        assert!(!error.to_string().contains(&root.display().to_string()));
        assert!(!error
            .to_string()
            .contains(std::env::temp_dir().to_string_lossy().as_ref()));
        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(all(feature = "legacy-file", debug_assertions, unix))]
    #[test]
    fn legacy_file_resolver_rejects_symlinked_roots_parents_and_files() {
        use std::os::unix::fs::symlink;

        let base = std::env::temp_dir().join(format!(
            "ep-legacy-secret-link-{}-{}",
            std::process::id(),
            uuid::Uuid::now_v7()
        ));
        let root = base.join("root");
        let outside = base.join("outside");
        std::fs::create_dir_all(root.join("db")).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("value"), "escaped").unwrap();
        std::fs::write(outside.join("app_rw#1"), "escaped").unwrap();
        let reference = SecretRef::parse("secret://db/app_rw#1").unwrap();

        std::fs::hard_link(outside.join("value"), root.join("db/app_rw#1")).unwrap();
        assert!(resolve_legacy_file_secret(&root, &reference).is_err());
        std::fs::remove_file(root.join("db/app_rw#1")).unwrap();

        symlink(outside.join("value"), root.join("db/app_rw#1")).unwrap();
        assert!(resolve_legacy_file_secret(&root, &reference).is_err());
        std::fs::remove_file(root.join("db/app_rw#1")).unwrap();

        std::fs::remove_dir(root.join("db")).unwrap();
        symlink(&outside, root.join("db")).unwrap();
        assert!(resolve_legacy_file_secret(&root, &reference).is_err());

        let root_link = base.join("root-link");
        symlink(&root, &root_link).unwrap();
        assert!(resolve_legacy_file_secret(&root_link, &reference).is_err());
        let _ = std::fs::remove_dir_all(base);
    }

    #[cfg(all(feature = "legacy-file", debug_assertions, windows))]
    #[test]
    fn windows_final_handle_containment_uses_verbatim_components_and_ascii_case_folding() {
        assert!(super::legacy_file::path_is_same_or_child(
            std::path::Path::new(r"\\?\C:\Secrets\db\app_rw#1"),
            std::path::Path::new(r"\\?\c:\secrets"),
        ));
        assert!(!super::legacy_file::path_is_same_or_child(
            std::path::Path::new(r"\\?\C:\Secrets-escape\db\app_rw#1"),
            std::path::Path::new(r"\\?\C:\Secrets"),
        ));
    }
}
