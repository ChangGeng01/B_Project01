//! 上传会话状态机与分片校验。取值逐行照抄阶段 3 计划第 3.4.7 节那张表。

/// 上传会话状态。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UploadState {
    Initiated,
    Uploading,
    Assembling,
    Scanning,
    Committed,
    Rejected,
    Aborted,
    Expired,
}

impl UploadState {
    pub fn as_db_value(self) -> &'static str {
        match self {
            UploadState::Initiated => "INITIATED",
            UploadState::Uploading => "UPLOADING",
            UploadState::Assembling => "ASSEMBLING",
            UploadState::Scanning => "SCANNING",
            UploadState::Committed => "COMMITTED",
            UploadState::Rejected => "REJECTED",
            UploadState::Aborted => "ABORTED",
            UploadState::Expired => "EXPIRED",
        }
    }

    /// 终态。`Committed` 与 `Rejected` 是判定结果，`Aborted` 与 `Expired` 是放弃。
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            UploadState::Committed
                | UploadState::Rejected
                | UploadState::Aborted
                | UploadState::Expired
        )
    }
}

/// 分片校验失败的原因。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PartError {
    /// 分片序号越界。序号是 **1 起**，不是 0 起——计划逐字「分片序号在 1..part_count」。
    OutOfRange { part_no: u32, part_count: u32 },
    /// 同序号重传但哈希与首次不一致。**这是最要紧的一条**：
    /// 允许它过去等于让重传悄悄替换掉一段已经校验过的正文，
    /// 而总哈希是在组装后才算的，替换发生在那之前就查不出来了。
    HashMismatchOnRetry {
        part_no: u32,
        first_hash: String,
        retry_hash: String,
    },
}

impl std::fmt::Display for PartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartError::OutOfRange {
                part_no,
                part_count,
            } => write!(f, "分片序号 {part_no} 越界，应在 1 至 {part_count} 之间"),
            PartError::HashMismatchOnRetry {
                part_no,
                first_hash,
                retry_hash,
            } => write!(
                f,
                "分片 {part_no} 重传的哈希 {retry_hash} 与首次 {first_hash} 不一致，拒绝覆盖"
            ),
        }
    }
}

impl std::error::Error for PartError {}

/// 校验一次分片写入。
///
/// `existing_hash` 是该序号此前已收分片的哈希，`None` 表示首次收到。
pub fn validate_part(
    part_no: u32,
    part_count: u32,
    incoming_hash: &str,
    existing_hash: Option<&str>,
) -> Result<(), PartError> {
    if part_no == 0 || part_no > part_count {
        return Err(PartError::OutOfRange {
            part_no,
            part_count,
        });
    }
    if let Some(first) = existing_hash {
        if first != incoming_hash {
            return Err(PartError::HashMismatchOnRetry {
                part_no,
                first_hash: first.to_string(),
                retry_hash: incoming_hash.to_string(),
            });
        }
    }
    Ok(())
}

/// 组装后的总校验。总哈希与总大小**两项都要判**，缺一不可。
///
/// 只判哈希不判大小，会放过一个哈希碰撞之外的更平常的情况：
/// 声明大小与实际不符说明分片清单本身就错了，而那时哈希可能恰好因为
/// 少收了一个空分片而仍然对得上。两项都判是廉价的双保险。
pub fn verify_assembled(
    actual_hash: &str,
    declared_hash: &str,
    actual_size: u64,
    declared_size: u64,
) -> bool {
    actual_hash == declared_hash && actual_size == declared_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_numbers_are_one_based() {
        assert!(validate_part(1, 3, "h", None).is_ok());
        assert!(validate_part(3, 3, "h", None).is_ok());
        // 0 号不存在——序号是 1 起。
        assert!(matches!(
            validate_part(0, 3, "h", None),
            Err(PartError::OutOfRange { .. })
        ));
        assert!(matches!(
            validate_part(4, 3, "h", None),
            Err(PartError::OutOfRange { .. })
        ));
    }

    /// 同序号重传哈希一致即放行——这是断点续传的正常情形。
    #[test]
    fn identical_retry_is_allowed() {
        assert!(validate_part(2, 3, "abc", Some("abc")).is_ok());
    }

    /// 本模块最要紧的一条：同序号重传哈希不一致必须拒。
    /// 放行等于让重传悄悄替换掉一段已校验过的正文，
    /// 而总哈希是在组装后才算的——替换发生在那之前就查不出来了。
    #[test]
    fn retry_with_a_different_hash_is_refused() {
        let err = validate_part(2, 3, "def", Some("abc")).expect_err("必须拒绝");
        match err {
            PartError::HashMismatchOnRetry {
                part_no,
                first_hash,
                retry_hash,
            } => {
                assert_eq!(part_no, 2);
                assert_eq!(first_hash, "abc");
                assert_eq!(retry_hash, "def");
            }
            other => panic!("应报重传哈希不一致，实为 {other}"),
        }
    }

    /// 总哈希与总大小两项都要判。
    #[test]
    fn assembly_check_needs_both_hash_and_size() {
        assert!(verify_assembled("h", "h", 100, 100));
        assert!(!verify_assembled("h", "other", 100, 100), "哈希不符应判否");
        assert!(!verify_assembled("h", "h", 100, 101), "大小不符应判否");
    }

    #[test]
    fn terminal_states() {
        for s in [
            UploadState::Committed,
            UploadState::Rejected,
            UploadState::Aborted,
            UploadState::Expired,
        ] {
            assert!(s.is_terminal(), "{s:?} 应为终态");
        }
        for s in [
            UploadState::Initiated,
            UploadState::Uploading,
            UploadState::Assembling,
            UploadState::Scanning,
        ] {
            assert!(!s.is_terminal(), "{s:?} 不该是终态");
        }
    }

    #[test]
    fn db_values() {
        assert_eq!(UploadState::Initiated.as_db_value(), "INITIATED");
        assert_eq!(UploadState::Committed.as_db_value(), "COMMITTED");
        assert_eq!(UploadState::Expired.as_db_value(), "EXPIRED");
    }
}
