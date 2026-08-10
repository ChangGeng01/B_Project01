//! 阶段 1 登记的错误码全集，共十三条。
//!
//! 取值、分类、HTTP 码与可重试性的唯一出处是阶段 1 计划第 6 节的登记表，
//! 与 `docs/error-codes.md` 由 `xtask errorcodes` 逐项比对，重复码或缺失码即构建失败。
//! 其中裁定 C-24 点名的七条由本阶段独家登记，阶段 3a 与阶段 4 不得重复登记。
//!
//! 本阶段只登记不返回的码在下表的 `stage` 列注明返回方，它们在本阶段没有调用点，
//! 这不是遗漏——登记先于实现，是为了让码在跨阶段唯一。

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thirteen_codes_no_duplicates() {
        assert_eq!(REGISTERED.len(), 13, "阶段 1 退出条件 7 定为十三条");
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
            assert!(want.contains(&r.http), "{} 的 {} 与 {:?} 不配套", r.code.0, r.http, r.category);
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
            assert!(REGISTERED.iter().any(|r| r.code.0 == want), "C-24 点名的 {want} 缺失");
        }
    }
}
