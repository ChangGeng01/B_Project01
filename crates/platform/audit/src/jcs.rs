//! RFC 8785 JSON 规范化（JCS）。审计事件的哈希输入由它产生。
//!
//! 为什么必须自己实现而不是随手 `to_string()`：`serde_json` 的默认序列化
//! **不保证键序**，而哈希链的每一环都建立在「同一份事实产出同一串字节」上。
//! 键序一变，同一条事件的哈希就变，整条链在验证时断掉——而那时已经过去很久，查不回来。
//!
//! # 一处刻意的收紧：不能精确往返的数值一律拒绝
//!
//! JCS 的数值序列化走 ECMAScript 的 `Number::toString`，即 IEEE 754 双精度。
//! 双精度只能精确表示到 2^53（约 9.007×10^15）以内的整数，而本卷的金额是
//! `numeric(18,2)`，最大到 10^18 量级——**一个足够大的金额经 JCS 会被悄悄舍入**，
//! 于是哈希算的是舍入后的值，而库里存的是原值。链验证当时不会报错，
//! 因为两侧用的是同一个舍入过程；但任何一方换实现就对不上，而且它从一开始就算错了。
//!
//! 本实现因此在遇到无法精确往返的数值时**返回错误而不是舍入**
//! （[`JcsError::NumberNotExact`]）。正确用法是把金额一类以字符串承载进
//! `before` 与 `after`——这与计划第 3.4.2 节「`bytea` 与 `uuid` 一律以字符串承载」
//! 是同一条纪律的延伸，只是那一节没点名数值。**这一条是本实现登记的一处发现。**

use serde_json::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum JcsError {
    /// 数值无法经 IEEE 754 双精度精确往返，见模块文档。
    NumberNotExact { literal: String },
    /// JSON 中出现 NaN 或无穷。`serde_json` 默认不产出它们，
    /// 出现即说明上游用了非标准配置。
    NonFinite,
}

impl std::fmt::Display for JcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JcsError::NumberNotExact { literal } => write!(
                f,
                "数值 {literal} 无法经双精度精确往返，JCS 会将其舍入；\
                 请以字符串承载该值（金额、数量、单价、比率一律如此）"
            ),
            JcsError::NonFinite => f.write_str("JSON 中出现 NaN 或无穷，无法规范化"),
        }
    }
}

impl std::error::Error for JcsError {}

/// 把一个 JSON 值规范化为 JCS 字节串。
pub fn canonicalize(value: &Value) -> Result<Vec<u8>, JcsError> {
    let mut out = String::new();
    write_value(value, &mut out)?;
    Ok(out.into_bytes())
}

fn write_value(v: &Value, out: &mut String) -> Result<(), JcsError> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(n) => out.push_str(&number_literal(n)?),
        Value::String(s) => write_string(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_value(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            // JCS 要求按键的 UTF-16 码元序排序。Rust 的 `str` 序是 UTF-8 字节序，
            // 两者在基本多文种平面内一致，在辅助平面上不同——那里 UTF-8 排在
            // U+E000..U+FFFF 之后，而 UTF-16 因代理对排在之前。
            // 审计事件的固定列名落在基本多文种平面内，但 before 与 after 是任意 jsonb，
            // 键名可能来自自定义字段，因此这里按 UTF-16 码元序排，不图省事用默认序。
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
            out.push('{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_string(k, out);
                out.push(':');
                write_value(&map[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

/// JCS 的字符串转义：只转义必须转义的，其余原样输出（含非 ASCII）。
fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// 双精度能精确表示的最大整数。超出即拒绝。
const MAX_EXACT_INT: u64 = 1u64 << 53;

/// 数值字面量。整数按十进制输出；小数与超出精确范围者一律拒绝。
fn number_literal(n: &serde_json::Number) -> Result<String, JcsError> {
    if let Some(i) = n.as_i64() {
        if i.unsigned_abs() > MAX_EXACT_INT {
            return Err(JcsError::NumberNotExact {
                literal: n.to_string(),
            });
        }
        return Ok(i.to_string());
    }
    if let Some(u) = n.as_u64() {
        if u > MAX_EXACT_INT {
            return Err(JcsError::NumberNotExact {
                literal: n.to_string(),
            });
        }
        return Ok(u.to_string());
    }
    match n.as_f64() {
        None => Err(JcsError::NonFinite),
        Some(f) if !f.is_finite() => Err(JcsError::NonFinite),
        // 小数一律拒绝：本卷的金额、数量、单价、比率四类都带精度声明，
        // 以双精度承载它们本身就是错的，不该在这里被默许。
        Some(_) => Err(JcsError::NumberNotExact {
            literal: n.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn c(v: &Value) -> String {
        String::from_utf8(canonicalize(v).expect("应可规范化")).expect("应为 UTF-8")
    }

    #[test]
    fn object_keys_are_sorted() {
        assert_eq!(
            c(&json!({"b": 1, "a": 2, "C": 3})),
            r#"{"C":3,"a":2,"b":1}"#
        );
    }

    /// 同一份事实、不同键序，必须产出同一串字节。这是哈希链成立的前提。
    #[test]
    fn key_order_does_not_change_the_bytes() {
        let a: Value = serde_json::from_str(r#"{"x":1,"y":{"q":"1","p":"2"}}"#).expect("解析");
        let b: Value = serde_json::from_str(r#"{"y":{"p":"2","q":"1"},"x":1}"#).expect("解析");
        assert_eq!(canonicalize(&a), canonicalize(&b));
    }

    #[test]
    fn no_whitespace_and_nested_shapes() {
        assert_eq!(
            c(&json!({"a":[1,2,{"b":null}]})),
            r#"{"a":[1,2,{"b":null}]}"#
        );
        assert_eq!(c(&json!([])), "[]");
        assert_eq!(c(&json!({})), "{}");
        assert_eq!(
            c(&json!({"t": true, "f": false})),
            r#"{"f":false,"t":true}"#
        );
    }

    #[test]
    fn control_chars_use_short_escapes() {
        assert_eq!(c(&json!("a\nb\tc\"d\\e")), r#""a\nb\tc\"d\\e""#);
        // 没有短转义的控制字符走 uXXXX 长转义。
        // 这一行的期望值我第一次写成了空串——实现是对的，断言写错了。
        // 留这条注释免得下次有人照着错的期望去改实现。
        assert_eq!(c(&json!("\u{1}")), "\"\\u0001\"");
        // 非 ASCII 原样输出，不转成 uXXXX 形态。
        assert_eq!(c(&json!("法人")), "\"法人\"");
    }

    /// 本实现登记的那处发现：超出双精度精确范围的整数不得被悄悄舍入。
    #[test]
    fn oversized_numbers_are_rejected_not_rounded() {
        let big: Value = serde_json::from_str("9007199254740993").expect("解析");
        assert!(matches!(
            canonicalize(&big),
            Err(JcsError::NumberNotExact { .. })
        ));
        // 边界内的照常输出。
        assert_eq!(c(&json!(9_007_199_254_740_992i64)), "9007199254740992");
    }

    /// 小数一律拒绝：金额与数量必须以字符串承载。
    #[test]
    fn fractional_numbers_are_rejected() {
        let amount: Value = serde_json::from_str("1234.56").expect("解析");
        assert!(matches!(
            canonicalize(&amount),
            Err(JcsError::NumberNotExact { .. })
        ));
        // 正确用法：以字符串承载。
        assert_eq!(c(&json!({"amount": "1234.56"})), r#"{"amount":"1234.56"}"#);
    }

    /// 辅助平面的键必须按 UTF-16 序排，不按 UTF-8 序。
    /// 这两个序在这里给出相反的结论，正好把用错的实现照出来。
    #[test]
    fn keys_in_supplementary_plane_sort_by_utf16() {
        let v = json!({"\u{10000}": 1, "\u{ffff}": 2});
        let s = c(&v);
        let i_supp = s.find('\u{10000}').expect("含辅助平面键");
        let i_bmp = s.find('\u{ffff}').expect("含基本平面键");
        assert!(i_supp < i_bmp, "应按 UTF-16 序排，实际 {s:?}");
    }
}
