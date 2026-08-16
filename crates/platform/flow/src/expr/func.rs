//! 函数白名单。
//!
//! 计划逐字：「以及一个**不超过 12 个**函数的白名单（长度、上取整、日期加减等）」。
//!
//! # 本轮交付三个，不凑到十二个
//!
//! 「不超过 12」是**上限**，少交付合规，凭空发明函数不合规。
//! 计划点名的只有三类——长度、上取整、日期加减——本轮就交付这三个。
//! 每多一个，就是一个没有出处的语义要在后面十年里被人当成规格。
//!
//! **顺带说清一件事：正因为只有三个，「白名单不超过 12 个」这条验收在本轮是恒真的**，
//! 不能算作已验收项。要验它，得等白名单真的长到接近十二个的那一天。

use super::value::GuardValue;
use super::GuardError;
use chrono::{Duration, NaiveDate};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

/// 白名单函数。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Func {
    /// `len(文本) -> 数字`。计划逐字点名「长度」。
    Len,
    /// `ceil(数字) -> 数字`。计划逐字点名「上取整」。
    Ceil,
    /// `date_add_days(文本, 数字) -> 文本`。计划逐字点名「日期加减」。
    ///
    /// **一个函数覆盖加与减**，天数为负即减。不另设 `date_sub_days`——
    /// 它写不出任何 `date_add_days(d, -n)` 写不出的东西，多一个就是凑数。
    DateAddDays,
}

/// 白名单全集。计划的上限是 12，本轮 3。
pub const ALL_FUNCS: [Func; 3] = [Func::Len, Func::Ceil, Func::DateAddDays];

/// 计划给的白名单上限。
pub const MAX_WHITELIST_FUNCS: usize = 12;

/// 日期的定宽形态。**只认 `YYYY-MM-DD` 十个字节**。
pub const DATE_LEN: usize = 10;

impl Func {
    pub fn name(self) -> &'static str {
        match self {
            Func::Len => "len",
            Func::Ceil => "ceil",
            Func::DateAddDays => "date_add_days",
        }
    }

    pub fn arity(self) -> usize {
        match self {
            Func::Len | Func::Ceil => 1,
            Func::DateAddDays => 2,
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        ALL_FUNCS.iter().copied().find(|f| f.name() == s)
    }

    /// 调用。实参已由求值器求好，且已保证**没有空值**——
    /// 空进函数是求值器那一层的错误，不到这里。
    pub fn call(self, args: &[GuardValue], at: usize) -> Result<GuardValue, GuardError> {
        match (self, args) {
            (Func::Len, [GuardValue::Text(s)]) => {
                // **按 Unicode 标量计数**，不是字节数、不是字素簇。
                // 字节数会让同一个汉字算三，字素簇要一张会随 Unicode 版本变的表；
                // 标量数是唯一一个既稳定又与人的直觉不太远的口径。
                let n = s.chars().count();
                Ok(GuardValue::Number(Decimal::from(n as i64)))
            }
            (Func::Ceil, [GuardValue::Number(d)]) => Ok(GuardValue::Number(d.ceil())),
            (Func::DateAddDays, [GuardValue::Text(d), GuardValue::Number(n)]) => {
                let base = parse_date(d, at)?;
                let days = n.is_integer().then(|| n.to_i64()).flatten().ok_or(
                    GuardError::NotAWholeNumber {
                        at,
                        func: self.name(),
                    },
                )?;
                let shifted = base
                    .checked_add_signed(Duration::days(days))
                    .ok_or(GuardError::DateOutOfRange { at })?;
                Ok(GuardValue::Text(shifted.format("%Y-%m-%d").to_string()))
            }
            // 类型不符。**错误里只报类型不报取值**，见 [`super`] 的纪律说明。
            (f, given) => Err(GuardError::BadArgumentType {
                at,
                func: f.name(),
                got: given.iter().map(|v| v.type_name()).collect(),
            }),
        }
    }
}

/// 严格解析 `YYYY-MM-DD`。
///
/// **不接受 `2026-9-1` 这类非定宽写法**，即便 chrono 认得。定宽是本模块
/// 把日期当文本承载的前提：`YYYY-MM-DD` 零填充时字典序等于日历序，
/// 于是 `vars.due > date_add_days(vars.start, 30)` 直接用文本比较就是对的。
/// 一旦放进非定宽形态，`'2026-9-1' < '2026-10-01'` 按字典序是 **false**——
/// 错，而且不报错。
fn parse_date(s: &str, at: usize) -> Result<NaiveDate, GuardError> {
    if s.len() != DATE_LEN {
        return Err(GuardError::BadDateFormat { at });
    }
    let b = s.as_bytes();
    let shape_ok = b[..4].iter().all(u8::is_ascii_digit)
        && b[4] == b'-'
        && b[5..7].iter().all(u8::is_ascii_digit)
        && b[7] == b'-'
        && b[8..].iter().all(u8::is_ascii_digit);
    if !shape_ok {
        return Err(GuardError::BadDateFormat { at });
    }
    NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| GuardError::BadDateFormat { at })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(s: &str) -> GuardValue {
        GuardValue::number(s).expect("夹具数字应合法")
    }

    /// 白名单只有计划点名的三个，且全部有出处。
    #[test]
    fn the_whitelist_is_exactly_the_three_the_plan_names() {
        assert_eq!(ALL_FUNCS.len(), 3, "多一个就要先在计划里给它出处");
        assert!(ALL_FUNCS.len() <= MAX_WHITELIST_FUNCS);
        let mut names: Vec<&str> = ALL_FUNCS.iter().map(|f| f.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), 3, "函数名必须互异");
        assert_eq!(names, vec!["ceil", "date_add_days", "len"]);
    }

    #[test]
    fn names_round_trip() {
        for f in ALL_FUNCS {
            assert_eq!(Func::from_name(f.name()), Some(f));
        }
        assert_eq!(Func::from_name("upper"), None);
        assert_eq!(Func::from_name("LEN"), None, "白名单区分大小写");
    }

    /// `len` 按 Unicode 标量计数。字节数会让一个汉字算三。
    #[test]
    fn len_counts_unicode_scalars_not_bytes() {
        assert_eq!(Func::Len.call(&[GuardValue::text("abc")], 0), Ok(n("3")));
        assert_eq!(Func::Len.call(&[GuardValue::text("中文字")], 0), Ok(n("3")));
        assert_eq!(Func::Len.call(&[GuardValue::text("")], 0), Ok(n("0")));
        // 一个补充平面字符是一个标量。
        assert_eq!(Func::Len.call(&[GuardValue::text("𝄞")], 0), Ok(n("1")));
    }

    #[test]
    fn ceil_rounds_toward_positive_infinity() {
        assert_eq!(Func::Ceil.call(&[n("1.2")], 0), Ok(n("2")));
        assert_eq!(Func::Ceil.call(&[n("2")], 0), Ok(n("2")), "整数不动");
        // 负数向零方向取整——这一条最容易写反。
        assert_eq!(Func::Ceil.call(&[n("-1.5")], 0), Ok(n("-1")));
    }

    #[test]
    fn date_add_days_does_real_calendar_arithmetic() {
        let d = |s: &str| GuardValue::text(s);
        // 跨月。
        assert_eq!(
            Func::DateAddDays.call(&[d("2026-01-31"), n("1")], 0),
            Ok(d("2026-02-01"))
        );
        // 闰年二月——按字符串拼会算错的那一天。
        assert_eq!(
            Func::DateAddDays.call(&[d("2028-02-28"), n("1")], 0),
            Ok(d("2028-02-29"))
        );
        assert_eq!(
            Func::DateAddDays.call(&[d("2026-02-28"), n("1")], 0),
            Ok(d("2026-03-01"))
        );
        // 负数即减，不另设 date_sub_days。
        assert_eq!(
            Func::DateAddDays.call(&[d("2026-03-01"), n("-1")], 0),
            Ok(d("2026-02-28"))
        );
    }

    /// 非定宽日期一律拒。定宽是「日期当文本比较」这条口径成立的前提：
    /// `'2026-9-1' < '2026-10-01'` 按字典序是 false，错且不报错。
    #[test]
    fn non_fixed_width_dates_are_refused() {
        for bad in [
            "2026-9-1",
            "2026-01-1",
            "26-01-01",
            "2026/01/01",
            "2026-01-01 ",
        ] {
            assert!(
                matches!(
                    Func::DateAddDays.call(&[GuardValue::text(bad), n("1")], 0),
                    Err(GuardError::BadDateFormat { .. })
                ),
                "{bad:?} 不是定宽 ISO-8601，应拒绝"
            );
        }
        // 形状对但日子不存在的也要拒。
        assert!(matches!(
            Func::DateAddDays.call(&[GuardValue::text("2026-02-30"), n("1")], 0),
            Err(GuardError::BadDateFormat { .. })
        ));
    }

    /// 天数必须是整数。`date_add_days(d, 1.5)` 要么被静默截断要么被静默舍入，
    /// 两条都是「错了不会当场报错」。
    #[test]
    fn fractional_days_are_refused() {
        assert!(matches!(
            Func::DateAddDays.call(&[GuardValue::text("2026-01-01"), n("1.5")], 0),
            Err(GuardError::NotAWholeNumber { .. })
        ));
        // 1.0 是整数，应通过。
        assert!(Func::DateAddDays
            .call(&[GuardValue::text("2026-01-01"), n("1.0")], 0)
            .is_ok());
    }

    /// 类型不符要报**类型**，不得把实参取值写进错误——
    /// 守卫是按金额、按薪资分流的，错误文案会落进日志与工单。
    #[test]
    fn type_errors_report_types_never_values() {
        let err = Func::Len
            .call(&[n("123456789")], 0)
            .expect_err("数字不能取长度");
        let msg = err.to_string();
        assert!(msg.contains("数字"), "应报类型，实为 {msg}");
        assert!(
            !msg.contains("123456789"),
            "错误文案不得携带实参取值，实为 {msg}"
        );
    }

    /// 日期越界要报错而不是回绕。
    #[test]
    fn date_overflow_is_an_error_not_a_wrap() {
        assert!(matches!(
            Func::DateAddDays.call(&[GuardValue::text("2026-01-01"), n("99999999")], 0),
            Err(GuardError::DateOutOfRange { .. })
        ));
    }
}
