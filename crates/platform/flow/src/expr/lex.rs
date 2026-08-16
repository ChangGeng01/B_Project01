//! 守卫表达式的词法。
//!
//! 表面形态取**中缀文本**，字符串字面量用**单引号**。
//!
//! # 为什么不是 JSON AST
//!
//! 决定性的理由只有一条：**JSON AST 会把守卫自己的数字字面量在解析那一刻变成 `f64`。**
//! `{"op":">","lit":70368744177664.02}` 里的那个数在 `from_str` 返回之前
//! 就已经是 `70368744177664.015625`——**阈值本身被污染**，
//! 这比变量被污染更隐蔽，因为阈值是流程作者亲手写下的，他会认为那就是原文。
//!
//! 要在 JSON 形态下躲开这一条只有两条路：把每个数字退化成 JSON 字符串
//! （那时「免解析」的好处已经没了，仍要为每个字符串写十进制解析器，只是外面多裹一层），
//! 或者手写一个不丢精度的 JSON 解析器（那比手写中缀词法器加递归下降**更长**：
//! JSON 要处理对象、数组、转义与 Unicode 码点，中缀只要处理七类记号和四层优先级）。
//!
//! 附带一条：表达式存在 `definition jsonb` 里，双引号字面量会被转义成 `\"`，
//! 在库里和在报错文案里都不可读；单引号让存储形态与作者写下的形态逐字相同。

use super::{GuardError, MAX_SOURCE_BYTES};
use crate::expr::value::GuardValue;

/// 一个记号及其在源文本中的字节偏移。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    /// 起始字节偏移。错误要能指到位置。
    pub at: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenKind {
    /// `vars` / `instance` / 函数名等标识符。
    Ident(String),
    /// 十进制数字字面量。**以文本形态过词法，到语法层才转 `Decimal`**——
    /// 词法器不碰数值语义。
    Number(String),
    /// 单引号字符串的内容，`''` 已还原为一个单引号。
    Text(String),
    /// 关键字。
    Keyword(Keyword),
    /// 比较运算符。
    Cmp(CmpOp),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Dot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Keyword {
    And,
    Or,
    Not,
    In,
    Is,
    Null,
    True,
    False,
}

impl Keyword {
    fn from_ident(s: &str) -> Option<Self> {
        Some(match s {
            "and" => Keyword::And,
            "or" => Keyword::Or,
            "not" => Keyword::Not,
            "in" => Keyword::In,
            "is" => Keyword::Is,
            "null" => Keyword::Null,
            "true" => Keyword::True,
            "false" => Keyword::False,
            _ => return None,
        })
    }
}

/// 六种比较。计划第 3.4.8 节逐字「比较（六种）」。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    pub const ALL: [CmpOp; 6] = [
        CmpOp::Eq,
        CmpOp::Ne,
        CmpOp::Lt,
        CmpOp::Le,
        CmpOp::Gt,
        CmpOp::Ge,
    ];

    pub fn as_source(self) -> &'static str {
        match self {
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
        }
    }
}

/// 分词。
///
/// **长度闸门在这里，在分词之前。** 计划对源文本长度一个字没写；
/// 不设它就没有任何东西约束进入词法器的输入规模，
/// 一条十兆的守卫会在持锁事务内做一次十兆的分词与建树。
pub fn tokenize(src: &str) -> Result<Vec<Token>, GuardError> {
    if src.len() > MAX_SOURCE_BYTES {
        return Err(GuardError::SourceTooLong {
            len: src.len(),
            max: MAX_SOURCE_BYTES,
        });
    }
    let b = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        let at = i;
        match c {
            b'(' => {
                out.push(Token {
                    kind: TokenKind::LParen,
                    at,
                });
                i += 1;
            }
            b')' => {
                out.push(Token {
                    kind: TokenKind::RParen,
                    at,
                });
                i += 1;
            }
            b'[' => {
                out.push(Token {
                    kind: TokenKind::LBracket,
                    at,
                });
                i += 1;
            }
            b']' => {
                out.push(Token {
                    kind: TokenKind::RBracket,
                    at,
                });
                i += 1;
            }
            b',' => {
                out.push(Token {
                    kind: TokenKind::Comma,
                    at,
                });
                i += 1;
            }
            b'.' => {
                out.push(Token {
                    kind: TokenKind::Dot,
                    at,
                });
                i += 1;
            }
            b'=' | b'!' | b'<' | b'>' => {
                let two = b.get(i + 1) == Some(&b'=');
                let op = match (c, two) {
                    (b'=', true) => CmpOp::Eq,
                    (b'!', true) => CmpOp::Ne,
                    (b'<', true) => CmpOp::Le,
                    (b'>', true) => CmpOp::Ge,
                    (b'<', false) => CmpOp::Lt,
                    (b'>', false) => CmpOp::Gt,
                    // 单个 `=` 与单个 `!` 都不是合法运算符。**不把 `=` 当成 `==`**：
                    // 那是在替作者猜意图，而猜错的方向是静默改变一条守卫的语义。
                    _ => return Err(GuardError::UnexpectedChar { at, ch: c as char }),
                };
                out.push(Token {
                    kind: TokenKind::Cmp(op),
                    at,
                });
                i += if two { 2 } else { 1 };
            }
            b'\'' => {
                let (text, next) = scan_text(b, i)?;
                out.push(Token {
                    kind: TokenKind::Text(text),
                    at,
                });
                i = next;
            }
            b'0'..=b'9' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_digit() || b[i] == b'.') {
                    i += 1;
                }
                let raw = &src[start..i];
                // 形状在这里就判死：不接受指数形式、不接受多个小数点、不接受前导符号。
                // 取值合法性（能否转成 Decimal）在语法层判，那里能给出更好的上下文。
                if raw.matches('.').count() > 1 || raw.ends_with('.') {
                    return Err(GuardError::MalformedNumber { at: start });
                }
                out.push(Token {
                    kind: TokenKind::Number(raw.to_string()),
                    at,
                });
            }
            _ if c.is_ascii_alphabetic() || c == b'_' => {
                let start = i;
                while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                    i += 1;
                }
                let word = &src[start..i];
                let kind = match Keyword::from_ident(word) {
                    Some(k) => TokenKind::Keyword(k),
                    None => TokenKind::Ident(word.to_string()),
                };
                out.push(Token { kind, at });
            }
            _ => {
                // 非 ASCII 字节落在这里。按字符报，不按字节报——
                // 报一个半个 UTF-8 序列的字节值对写表达式的人毫无用处。
                let ch = src[at..].chars().next().unwrap_or('\u{fffd}');
                return Err(GuardError::UnexpectedChar { at, ch });
            }
        }
    }
    Ok(out)
}

/// 扫一个单引号字符串。`''` 表示一个单引号。
fn scan_text(b: &[u8], open: usize) -> Result<(String, usize), GuardError> {
    let mut i = open + 1;
    let mut buf = Vec::new();
    while i < b.len() {
        if b[i] == b'\'' {
            if b.get(i + 1) == Some(&b'\'') {
                buf.push(b'\'');
                i += 2;
                continue;
            }
            let s = String::from_utf8(buf).map_err(|_| GuardError::MalformedText { at: open })?;
            return Ok((s, i + 1));
        }
        buf.push(b[i]);
        i += 1;
    }
    Err(GuardError::UnterminatedText { at: open })
}

/// 把一个数字记号转成值。放在这里是为了让「数只从十进制文本来」这一条只有一个入口。
pub fn number_value(raw: &str, at: usize) -> Result<GuardValue, GuardError> {
    GuardValue::number(raw).ok_or(GuardError::MalformedNumber { at })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        tokenize(src)
            .expect("夹具应能分词")
            .into_iter()
            .map(|t| t.kind)
            .collect()
    }

    #[test]
    fn six_comparison_operators_all_lex() {
        assert_eq!(CmpOp::ALL.len(), 6, "计划逐字「比较（六种）」");
        for op in CmpOp::ALL {
            let src = format!("1 {} 2", op.as_source());
            let k = kinds(&src);
            assert_eq!(k[1], TokenKind::Cmp(op), "{} 未被正确分词", op.as_source());
        }
        // 六个源文本形态互异——写重了会让两个运算符指向同一个记号。
        let mut forms: Vec<&str> = CmpOp::ALL.iter().map(|o| o.as_source()).collect();
        forms.sort_unstable();
        forms.dedup();
        assert_eq!(forms.len(), 6);
    }

    /// `<` 与 `<=` 必须分得开——最长匹配。
    #[test]
    fn longest_match_wins() {
        assert_eq!(kinds("a <= b")[1], TokenKind::Cmp(CmpOp::Le));
        assert_eq!(kinds("a < b")[1], TokenKind::Cmp(CmpOp::Lt));
        assert_eq!(kinds("a >= b")[1], TokenKind::Cmp(CmpOp::Ge));
        assert_eq!(kinds("a > b")[1], TokenKind::Cmp(CmpOp::Gt));
    }

    /// 单个 `=` 不当成 `==`。替作者猜意图，猜错的方向是静默改变一条守卫的语义。
    #[test]
    fn a_single_equals_is_not_an_equality_operator() {
        assert!(matches!(
            tokenize("a = b"),
            Err(GuardError::UnexpectedChar { ch: '=', .. })
        ));
        assert!(matches!(
            tokenize("a ! b"),
            Err(GuardError::UnexpectedChar { ch: '!', .. })
        ));
    }

    #[test]
    fn keywords_are_not_identifiers() {
        for (w, k) in [
            ("and", Keyword::And),
            ("or", Keyword::Or),
            ("not", Keyword::Not),
            ("in", Keyword::In),
            ("is", Keyword::Is),
            ("null", Keyword::Null),
            ("true", Keyword::True),
            ("false", Keyword::False),
        ] {
            assert_eq!(kinds(w), vec![TokenKind::Keyword(k)]);
        }
        // 前缀相同的标识符不得被误认成关键字。
        assert_eq!(kinds("android"), vec![TokenKind::Ident("android".into())]);
        assert_eq!(kinds("nothing"), vec![TokenKind::Ident("nothing".into())]);
    }

    /// 单引号字符串，`''` 还原成一个单引号。
    #[test]
    fn text_literals_and_escaped_quotes() {
        assert_eq!(kinds("'abc'"), vec![TokenKind::Text("abc".into())]);
        assert_eq!(kinds("'it''s'"), vec![TokenKind::Text("it's".into())]);
        assert_eq!(kinds("''"), vec![TokenKind::Text(String::new())]);
        assert_eq!(kinds("'中文'"), vec![TokenKind::Text("中文".into())]);
    }

    #[test]
    fn unterminated_text_is_refused() {
        assert!(matches!(
            tokenize("'abc"),
            Err(GuardError::UnterminatedText { at: 0 })
        ));
    }

    /// 数字保持文本形态过词法——**词法器不碰数值语义**。
    /// 这一条守的是「精度在哪一步丢」这件事只有一个可查的地方。
    #[test]
    fn numbers_stay_textual_through_lexing() {
        assert_eq!(
            kinds("70368744177664.02"),
            vec![TokenKind::Number("70368744177664.02".into())]
        );
    }

    #[test]
    fn malformed_numbers_are_refused_at_lexing() {
        for bad in ["1.2.3", "1."] {
            assert!(
                matches!(tokenize(bad), Err(GuardError::MalformedNumber { .. })),
                "{bad} 应在词法期被拒"
            );
        }
    }

    /// 指数形式不被接受：`1e10` 分词成标识符相邻，语法层会拒。
    /// 这里断言的是词法器**没有**悄悄支持它。
    #[test]
    fn exponent_form_is_not_a_number() {
        let k = kinds("1e10");
        assert_eq!(k[0], TokenKind::Number("1".into()));
        assert_eq!(k[1], TokenKind::Ident("e10".into()));
    }

    /// 源文本长度闸门在分词之前，不是分词之后。
    #[test]
    fn the_length_gate_runs_before_lexing() {
        let over = "1".repeat(MAX_SOURCE_BYTES + 1);
        assert!(matches!(
            tokenize(&over),
            Err(GuardError::SourceTooLong { .. })
        ));
        // 恰好等于上限应通过——上限是「不得超过」。
        let at = "1".repeat(MAX_SOURCE_BYTES);
        assert!(tokenize(&at).is_ok());
    }

    /// 非 ASCII 按字符报位置，不报半个 UTF-8 序列的字节值。
    #[test]
    fn non_ascii_is_reported_as_a_character() {
        match tokenize("a ＝ b") {
            Err(GuardError::UnexpectedChar { ch, .. }) => assert_eq!(ch, '＝'),
            other => panic!("应报非法字符，实为 {other:?}"),
        }
    }
}
