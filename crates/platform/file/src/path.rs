//! 正文落盘路径的构造。
//!
//! 形状取自阶段 3 计划第 3.4.7 节：
//! `<legal_entity_id>/<security_level>/<yyyy>/<mm>/<version_id>`。
//!
//! 这一层单列并单测的理由只有一个：**路径是由外部输入拼出来的**。
//! 法人标识与版本标识来自请求，若不校验就直接拼，一个带 `..` 的取值
//! 就能把正文写到附件根之外——那不是「附件功能有 bug」，那是任意文件写入。
//!
//! 因此本模块**不接受任何字符串**，只接受已校验过形状的片段，
//! 且把校验做成构造的必经之路：拿不到 [`PathSegment`] 就拼不出路径。

/// 一个已校验的路径片段。
///
/// 只允许 ASCII 字母、数字、连字符与下划线。这比「拒绝 `..`」严得多，
/// 而严是对的：黑名单要穷举所有危险形态（`..`、`.`、绝对路径、盘符、
/// 反斜杠、空字节、Unicode 同形字），白名单只需要说清什么是安全的。
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct PathSegment(String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SegmentError {
    Empty,
    /// 含不允许的字符。把违规字符点出来，否则调用方只知道「不合法」。
    IllegalChar {
        found: char,
    },
    TooLong {
        len: usize,
        max: usize,
    },
}

impl std::fmt::Display for SegmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegmentError::Empty => f.write_str("路径片段不得为空"),
            SegmentError::IllegalChar { found } => write!(
                f,
                "路径片段含不允许的字符 {found:?}；只允许 ASCII 字母、数字、连字符与下划线"
            ),
            SegmentError::TooLong { len, max } => {
                write!(f, "路径片段长度 {len} 超过上限 {max}")
            }
        }
    }
}

impl std::error::Error for SegmentError {}

/// 单个片段的长度上限。取值不来自规格——规格只给了整条路径的形状。
/// 定 64 是因为该位置上真正会出现的取值是 UUID（36）与两位月份，
/// 留一倍余量足够；上限的作用是挡住把一整段正文塞进路径这类形态。
pub const MAX_SEGMENT_LEN: usize = 64;

impl PathSegment {
    pub fn parse(s: &str) -> Result<Self, SegmentError> {
        if s.is_empty() {
            return Err(SegmentError::Empty);
        }
        if s.len() > MAX_SEGMENT_LEN {
            return Err(SegmentError::TooLong {
                len: s.len(),
                max: MAX_SEGMENT_LEN,
            });
        }
        if let Some(bad) = s
            .chars()
            .find(|c| !(c.is_ascii_alphanumeric() || *c == '-' || *c == '_'))
        {
            return Err(SegmentError::IllegalChar { found: bad });
        }
        Ok(Self(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 正文的相对落盘路径。
///
/// 返回的是**相对路径**，不含附件根——拼根是调用方的事，
/// 本模块拿不到根就不可能拼出一个绝对路径，这是又一层。
///
/// 分隔符固定用 `/`：它同时是 URL 与 Windows API 都认的形式，
/// 而混用 `\` 会让同一份正文在两处产生两个不同的键。
pub fn storage_path(
    legal_entity_id: &PathSegment,
    security_level: u8,
    year: u16,
    month: u8,
    version_id: &PathSegment,
) -> String {
    format!(
        "{}/{}/{:04}/{:02}/{}",
        legal_entity_id.as_str(),
        security_level,
        year,
        month,
        version_id.as_str()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(s: &str) -> PathSegment {
        PathSegment::parse(s).expect("夹具片段应合法")
    }

    #[test]
    fn path_shape_matches_the_plan() {
        let p = storage_path(&seg("le-01"), 20, 2026, 8, &seg("v-abc"));
        assert_eq!(p, "le-01/20/2026/08/v-abc");
    }

    #[test]
    fn month_is_zero_padded() {
        let p = storage_path(&seg("le"), 10, 2026, 1, &seg("v"));
        assert!(p.contains("/2026/01/"), "月份要补零，实际 {p}");
    }

    /// 白名单挡住的不只是 `..`——逐个点名最常见的几种越界形态。
    /// 用白名单而不是黑名单：黑名单要穷举所有危险形态，白名单只需说清什么安全。
    #[test]
    fn traversal_and_separators_are_all_refused() {
        for bad in [
            "..", "../etc", "a/b", "a\\b", "/abs", "C:", "a.b", "a b", "中文",
        ] {
            assert!(
                PathSegment::parse(bad).is_err(),
                "{bad:?} 必须被拒绝，否则可写到附件根之外"
            );
        }
    }

    /// 空字节是最容易被漏的一种——某些底层 API 会在它那里截断，
    /// 于是校验看到的是长串、实际打开的是短串。
    #[test]
    fn nul_byte_is_refused() {
        assert!(PathSegment::parse("a\0b").is_err());
    }

    #[test]
    fn empty_and_overlong_are_refused() {
        assert!(matches!(PathSegment::parse(""), Err(SegmentError::Empty)));
        let long = "a".repeat(MAX_SEGMENT_LEN + 1);
        assert!(matches!(
            PathSegment::parse(&long),
            Err(SegmentError::TooLong { .. })
        ));
        // 恰好等于上限应通过——上限是「不得超过」。
        assert!(PathSegment::parse(&"a".repeat(MAX_SEGMENT_LEN)).is_ok());
    }

    /// 真正会出现的取值必须过得去：UUID 与两位月份。
    #[test]
    fn real_world_values_pass() {
        assert!(PathSegment::parse("018f3a2b-1c4d-7e8f-9a0b-1c2d3e4f5a6b").is_ok());
        assert!(PathSegment::parse("le_01").is_ok());
    }

    /// 违规字符要被点名，否则调用方只知道「不合法」、不知道哪里不合法。
    #[test]
    fn illegal_char_is_named() {
        match PathSegment::parse("a/b") {
            Err(SegmentError::IllegalChar { found }) => assert_eq!(found, '/'),
            other => panic!("应点名违规字符，实为 {other:?}"),
        }
    }
}
