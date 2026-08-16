//! 错误码全集：阶段 1 十三条，加阶段 2 任务 #12（ep-adapter-kms）登记的八条，
//! 加阶段 2 任务 #11（ep-adapter-db-pg）登记的三条，加阶段 2 任务 #14
//! （集成 B）登记的十条，加阶段 3a 任务 #18 登记的一条，加阶段 4 任务 #22
//! 登记的二十三条，加阶段 3a 任务 #3（ep-platform-sequence）登记的一条，共五十九条。
//!
//! 取值、分类、HTTP 码与可重试性的唯一出处是阶段 1 计划第 6 节的登记表，
//! 与 `docs/error-codes.md` 由 `xtask errorcodes` 逐项比对，重复码或缺失码即构建失败。
//! 其中裁定 C-24 点名的七条由阶段 1 独家登记，阶段 3a 与阶段 4 不得重复登记。
//!
//! 阶段 2 新增八条的出处：02 计划第 5 节错误码表的 KEY_DOMAIN 与 CRYPTO 两段，
//! 由任务 #12 先登记后实现；`TRANSITION_INVALID` 不在计划 20 码清单之内，
//! 是密钥域状态机非法迁移的专用码（分类按本文件第 1 节对 BUSINESS_CONFLICT
//! 的定义含「状态机非法迁移」），拟登记文案见 docs/error-codes.md 第 3 节。
//!
//! 阶段 2 任务 #11 新增三条的出处：02 计划第 4.7 节重试算法与第 3.8 节
//! 命名与精度：`SERIALIZATION_RETRY_EXHAUSTED`（40001/40P01 重试用尽或
//! 副作用置位）、`REFERENCED_ROW_MISSING`（SQLSTATE 23503 统一映射）、
//! `WRITE_SCALE_VIOLATION`（写入超出声明精度，IT-27）。
//!
//! 阶段 2 任务 #14 新增十条的出处：02 计划第 5 节 20 码清单的差额——
//! DB 段九条（`RLS_CONTEXT_MISSING`、`LEGAL_ENTITY_MISMATCH`、`POOL_EXHAUSTED`、
//! `STATEMENT_TIMEOUT`、`LOCK_TIMEOUT`、`MIGRATION_VERSION_MISMATCH`、
//! `MIGRATION_WINDOW_CONFLICT`、`APPEND_ONLY_VIOLATION`、`ROW_VERSION_NOT_BUMPED`）
//! 与 `SENSITIVE_FIELD.NOT_REGISTERED`（敏感字段未登记即加密写入被拒，IT-35）。
//!
//! 阶段 3a 任务 #18 新增一条的出处：03 计划表 12 注与新增决定六：
//! `IDEMPOTENCY.IN_PROGRESS`（同一幂等键已有请求在途，并发去重以冲突拒绝；
//! 不占用 `IdempotencyOutcome` 变体，由 `IdempotencyStore::try_begin` 以 Err 返回）。
//!
//! 阶段 4 任务 #22 新增二十三条的出处：04 计划 §6.5 错误码清单（扣除裁定
//! C-24 已由阶段 1 独家登记的七码）：AUTHN 段九条、AUTHZ 段六条、SOD 段
//! 两条、REAUTH 与 APPROVAL 各一条、HIGH_RISK_REQUEST 一条、USER_ACCOUNT
//! 段三条；`RATE_LIMITED` 归类 INFRASTRUCTURE 取其登记的 503（限流 429
//! 的运行时映射在封套层，登记口径与第 1 节分类表一致）。
//! 裁定 C-24 点名的七码由阶段 1 独家登记，本批不重复登记。

use super::ErrorCode;

/// 错误码的分类。取值域与技术基线第 5 节的统一封套一致。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Category {
    Validation,
    PermissionDenied,
    BusinessConflict,
    Infrastructure,
}

impl Category {
    pub const fn as_str(self) -> &'static str {
        match self {
            Category::Validation => "VALIDATION",
            Category::PermissionDenied => "PERMISSION_DENIED",
            Category::BusinessConflict => "BUSINESS_CONFLICT",
            Category::Infrastructure => "INFRASTRUCTURE",
        }
    }
}

/// 一条错误码的登记项。四列与登记表一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Registered {
    pub code: ErrorCode,
    pub category: Category,
    pub http: u16,
    pub retryable: bool,
}

macro_rules! codes {
    ($($konst:ident => $code:literal, $cat:ident, $http:literal, $retry:literal;)*) => {
        $(pub const $konst: ErrorCode = ErrorCode($code);)*

        /// 全集。`xtask errorcodes` 以本表为代码侧的唯一出处。
        pub const REGISTERED: &[Registered] = &[
            $(Registered {
                code: $konst,
                category: Category::$cat,
                http: $http,
                retryable: $retry,
            },)*
        ];
    };
}

codes! {
    PLATFORM_SYSTEM_NOT_READY => "PLATFORM.SYSTEM.NOT_READY", Infrastructure, 503, true;
    PLATFORM_SYSTEM_SYNC_TIMEOUT => "PLATFORM.SYSTEM.SYNC_TIMEOUT", Infrastructure, 503, true;
    PLATFORM_SYSTEM_INTERNAL_ERROR => "PLATFORM.SYSTEM.INTERNAL_ERROR", Infrastructure, 503, true;
    PLATFORM_REQUEST_INVALID_PAYLOAD => "PLATFORM.REQUEST.INVALID_PAYLOAD", Validation, 400, false;
    PLATFORM_REQUEST_HEADER_MISSING => "PLATFORM.REQUEST.HEADER_MISSING", Validation, 400, false;
    PLATFORM_ROUTE_NOT_FOUND => "PLATFORM.ROUTE.NOT_FOUND", PermissionDenied, 404, false;
    PLATFORM_IDEMPOTENCY_KEY_REQUIRED => "PLATFORM.IDEMPOTENCY.KEY_REQUIRED", Validation, 400, false;
    PLATFORM_CAPACITY_CONCURRENCY_LIMIT => "PLATFORM.CAPACITY.CONCURRENCY_LIMIT", Infrastructure, 503, true;
    PLATFORM_IDEMPOTENCY_PAYLOAD_MISMATCH => "PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH", BusinessConflict, 409, false;
    PLATFORM_CONCURRENCY_STALE_VERSION => "PLATFORM.CONCURRENCY.STALE_VERSION", BusinessConflict, 409, false;
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED => "PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED", PermissionDenied, 404, false;
    PLATFORM_AUTHZ_OBJECT_FORBIDDEN => "PLATFORM.AUTHZ.OBJECT_FORBIDDEN", PermissionDenied, 403, false;
    PLATFORM_DB_MIGRATION_WINDOW_CLOSED => "PLATFORM.DB.MIGRATION_WINDOW_CLOSED", BusinessConflict, 409, false;
    PLATFORM_SEQUENCE_TYPE_CODE_NOT_REGISTERED => "PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED", Validation, 400, false;
    PLATFORM_KEY_DOMAIN_NOT_PROVISIONED => "PLATFORM.KEY_DOMAIN.NOT_PROVISIONED", Infrastructure, 503, true;
    PLATFORM_KEY_DOMAIN_KEY_UNAVAILABLE => "PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE", Infrastructure, 503, true;
    PLATFORM_KEY_DOMAIN_ROTATION_IN_PROGRESS => "PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS", BusinessConflict, 409, false;
    PLATFORM_KEY_DOMAIN_DESTROY_PRECHECK_FAILED => "PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED", BusinessConflict, 409, false;
    PLATFORM_KEY_DOMAIN_TRANSITION_INVALID => "PLATFORM.KEY_DOMAIN.TRANSITION_INVALID", BusinessConflict, 409, false;
    PLATFORM_CRYPTO_DECRYPT_FAILED => "PLATFORM.CRYPTO.DECRYPT_FAILED", Infrastructure, 503, true;
    PLATFORM_CRYPTO_AAD_MISMATCH => "PLATFORM.CRYPTO.AAD_MISMATCH", BusinessConflict, 409, false;
    PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID => "PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID", Validation, 400, false;
    PLATFORM_DB_SERIALIZATION_RETRY_EXHAUSTED => "PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED", Infrastructure, 503, true;
    PLATFORM_DB_REFERENCED_ROW_MISSING => "PLATFORM.DB.REFERENCED_ROW_MISSING", Validation, 400, false;
    PLATFORM_DB_WRITE_SCALE_VIOLATION => "PLATFORM.DB.WRITE_SCALE_VIOLATION", Validation, 400, false;
    PLATFORM_DB_RLS_CONTEXT_MISSING => "PLATFORM.DB.RLS_CONTEXT_MISSING", Infrastructure, 503, true;
    PLATFORM_DB_LEGAL_ENTITY_MISMATCH => "PLATFORM.DB.LEGAL_ENTITY_MISMATCH", PermissionDenied, 403, false;
    PLATFORM_DB_POOL_EXHAUSTED => "PLATFORM.DB.POOL_EXHAUSTED", Infrastructure, 503, true;
    PLATFORM_DB_STATEMENT_TIMEOUT => "PLATFORM.DB.STATEMENT_TIMEOUT", Infrastructure, 503, true;
    PLATFORM_DB_LOCK_TIMEOUT => "PLATFORM.DB.LOCK_TIMEOUT", BusinessConflict, 409, false;
    PLATFORM_DB_MIGRATION_VERSION_MISMATCH => "PLATFORM.DB.MIGRATION_VERSION_MISMATCH", Infrastructure, 503, true;
    PLATFORM_DB_MIGRATION_WINDOW_CONFLICT => "PLATFORM.DB.MIGRATION_WINDOW_CONFLICT", BusinessConflict, 409, false;
    PLATFORM_DB_APPEND_ONLY_VIOLATION => "PLATFORM.DB.APPEND_ONLY_VIOLATION", BusinessConflict, 409, false;
    PLATFORM_DB_ROW_VERSION_NOT_BUMPED => "PLATFORM.DB.ROW_VERSION_NOT_BUMPED", BusinessConflict, 409, false;
    PLATFORM_SENSITIVE_FIELD_NOT_REGISTERED => "PLATFORM.SENSITIVE_FIELD.NOT_REGISTERED", Validation, 400, false;
    PLATFORM_IDEMPOTENCY_IN_PROGRESS => "PLATFORM.IDEMPOTENCY.IN_PROGRESS", BusinessConflict, 409, false;
    PLATFORM_AUTHN_CREDENTIAL_INVALID => "PLATFORM.AUTHN.CREDENTIAL_INVALID", PermissionDenied, 403, false;
    PLATFORM_AUTHN_ACCOUNT_LOCKED => "PLATFORM.AUTHN.ACCOUNT_LOCKED", BusinessConflict, 409, false;
    PLATFORM_AUTHN_ACCOUNT_INACTIVE => "PLATFORM.AUTHN.ACCOUNT_INACTIVE", BusinessConflict, 409, false;
    PLATFORM_AUTHN_MFA_REQUIRED => "PLATFORM.AUTHN.MFA_REQUIRED", BusinessConflict, 409, false;
    PLATFORM_AUTHN_MFA_INVALID => "PLATFORM.AUTHN.MFA_INVALID", Validation, 400, false;
    PLATFORM_AUTHN_MFA_CHALLENGE_EXPIRED => "PLATFORM.AUTHN.MFA_CHALLENGE_EXPIRED", BusinessConflict, 409, false;
    PLATFORM_AUTHN_MFA_LAST_FACTOR_FORBIDDEN => "PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN", BusinessConflict, 409, false;
    PLATFORM_AUTHN_DEVICE_NOT_REGISTERED => "PLATFORM.AUTHN.DEVICE_NOT_REGISTERED", PermissionDenied, 403, false;
    PLATFORM_AUTHN_RATE_LIMITED => "PLATFORM.AUTHN.RATE_LIMITED", Infrastructure, 503, true;
    PLATFORM_AUTHZ_LEGAL_ENTITY_NOT_GRANTED => "PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED", PermissionDenied, 403, false;
    PLATFORM_AUTHZ_ISOLATION_CONTROL_FORBIDDEN => "PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN", PermissionDenied, 403, false;
    PLATFORM_AUTHZ_DIRECT_DB_ACCESS_FORBIDDEN => "PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN", PermissionDenied, 403, false;
    PLATFORM_AUTHZ_PERMISSION_ITEM_UNKNOWN => "PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN", Validation, 400, false;
    PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN => "PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN", Validation, 400, false;
    PLATFORM_AUTHZ_INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN => "PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN", PermissionDenied, 403, false;
    PLATFORM_SOD_DUTY_CONFLICT => "PLATFORM.SOD.DUTY_CONFLICT", BusinessConflict, 409, false;
    PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN => "PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN", BusinessConflict, 409, false;
    PLATFORM_REAUTH_TOKEN_ALREADY_CONSUMED => "PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED", BusinessConflict, 409, false;
    PLATFORM_APPROVAL_NODE_HAS_NO_APPROVER => "PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER", BusinessConflict, 409, false;
    PLATFORM_HIGH_RISK_REQUEST_CLIENT_NOT_ALLOWED => "PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED", PermissionDenied, 403, false;
    PLATFORM_USER_ACCOUNT_BATCH_PARTIAL_FAILED => "PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED", BusinessConflict, 409, false;
    PLATFORM_USER_ACCOUNT_MFA_ENROLLMENT_REQUIRED => "PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED", BusinessConflict, 409, false;
    PLATFORM_USER_ACCOUNT_PENDING_APPROVAL_TASKS => "PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS", BusinessConflict, 409, false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_count_matches_headcount() {
        // 阶段 1 十三条 + 阶段 2 任务 #12 八条 + 阶段 2 任务 #11 三条
        // + 阶段 2 任务 #14 十条 + 阶段 3a 任务 #18 一条 + 阶段 4 任务 #22
        // 二十三条 + 阶段 3a 任务 #3 一条，共五十九条。
        assert_eq!(REGISTERED.len(), 59, "计数与登记批次不符");
        let mut seen: Vec<&str> = REGISTERED.iter().map(|r| r.code.0).collect();
        seen.sort_unstable();
        let before = seen.len();
        seen.dedup();
        assert_eq!(seen.len(), before, "重复码即构建失败");
    }

    /// 码形态固定为 `PLATFORM.<段>.<名>` 的三段点分大写。
    #[test]
    fn code_shape_is_three_segment_upper() {
        for r in REGISTERED {
            let parts: Vec<&str> = r.code.0.split('.').collect();
            assert_eq!(parts.len(), 3, "{} 不是三段", r.code.0);
            assert_eq!(parts[0], "PLATFORM", "阶段 1 只登记 PLATFORM 段");
            for p in parts {
                assert!(
                    p.chars().all(|c| c.is_ascii_uppercase() || c == '_'),
                    "{} 含非大写下划线字符",
                    r.code.0
                );
            }
        }
    }

    /// 分类与 HTTP 码的配套关系不得自相矛盾。
    #[test]
    fn category_and_http_agree() {
        for r in REGISTERED {
            let want: &[u16] = match r.category {
                Category::Validation => &[400],
                Category::PermissionDenied => &[403, 404],
                Category::BusinessConflict => &[409],
                Category::Infrastructure => &[503],
            };
            assert!(
                want.contains(&r.http),
                "{} 的 {} 与 {:?} 不配套",
                r.code.0,
                r.http,
                r.category
            );
            // 只有基础设施类可重试，其余一律否——重试一个校验失败没有意义。
            assert_eq!(
                r.retryable,
                r.category == Category::Infrastructure,
                "{} 的可重试性与分类不符",
                r.code.0
            );
        }
    }

    /// 裁定 C-24 点名的七条必须齐备。
    #[test]
    fn c24_seven_are_registered() {
        const C24: [&str; 7] = [
            "PLATFORM.IDEMPOTENCY.KEY_REQUIRED",
            "PLATFORM.CAPACITY.CONCURRENCY_LIMIT",
            "PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH",
            "PLATFORM.CONCURRENCY.STALE_VERSION",
            "PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED",
            "PLATFORM.AUTHZ.OBJECT_FORBIDDEN",
            "PLATFORM.DB.MIGRATION_WINDOW_CLOSED",
        ];
        for want in C24 {
            assert!(
                REGISTERED.iter().any(|r| r.code.0 == want),
                "C-24 点名的 {want} 缺失"
            );
        }
    }
}
