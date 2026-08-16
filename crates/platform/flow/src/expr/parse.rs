//! 语法树与递归下降解析。
//!
//! # 深度闸门为什么在这里，以及它拦的是求值步数拦不住的东西
//!
//! 计划逐字是「表达式无副作用、无循环、**求值**步数上限 1000」——
//! 计数器锚在求值阶段。而 `((((…))))` 在**解析**阶段就把栈打穿了，
//! 那时语法树还没建出来，计数器一次都没自增过。
//!
//! Rust 的栈溢出由 guard page 触发，打印后 **abort**：不是 panic，
//! `catch_unwind` 抓不到、`Result` 接不住、`#[should_panic]` 接不住，进程整个死掉。
//! 而守卫表达式是管理员在低代码界面上写的。
//! 只落 1000 步不落深度上限，那条 `steps > max` 的判断在这类输入上一次都走不到——
//! 是一个**永远达不到的上限**。
//!
//! # 一个深度计数器同时管住三件事
//!
//! 关键是 `and` / `or` 按**右结合**建树而不是左结合循环。两者语义完全相同
//! （与、或都可结合，且短路都是从左到右），但右结合让
//! **解析递归深度、语法树高度、求值递归深度三者同阶**，
//! 于是一个 [`MAX_DEPTH`] 就同时管住：
//!
//! 一、括号嵌套 `((((…))))`；
//! 二、`not not not …` 链；
//! 三、`a and b and c and …` 链——**左结合循环拦不住这一条**：
//!     循环产出的是左偏树，树高等于合取项数，与括号深度无关，
//!     求值器按树递归时栈深度就是合取项数，而四百个合取项只有约八百步，
//!     远在 1000 之内。这是设计上最容易漏的一条。
//!
//! 深度只在**结构性下降**处自增（进括号、链尾递归、`not` 递归、函数实参），
//! 不在 or→and→not→cmp→primary 这一串固定的优先级下降上自增——
//! 否则一条 `a > 1` 就吃掉五格，[`MAX_DEPTH`] 会变成一个与嵌套无关的数。

use super::func::Func;
use super::lex::{number_value, tokenize, CmpOp, Keyword, Token, TokenKind};
use super::value::GuardValue;
use super::{GuardError, MAX_DEPTH};

/// 变量引用的两种形态。计划逐字「字段引用（`vars.x`、`instance.state`）」。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PathRef {
    /// `vars.<ident>`。**只允许单段**：`vars.a.b` 是解析错误，不是未实现的 todo。
    /// 变量没有 schema，多段路径的每一段都无从校验。
    Var(String),
    /// `instance.state`。`instance.` 下没有第二个合法路径。
    InstanceState,
}

/// 语法树节点。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Node {
    Lit(GuardValue),
    Path {
        path: PathRef,
        at: usize,
    },
    Not(Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Cmp {
        op: CmpOp,
        at: usize,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    In {
        at: usize,
        lhs: Box<Node>,
        set: Vec<GuardValue>,
    },
    IsNull {
        at: usize,
        negated: bool,
        operand: Box<Node>,
    },
    Call {
        func: Func,
        at: usize,
        args: Vec<Node>,
    },
}

/// 解析一条守卫表达式。
pub fn parse(src: &str) -> Result<Node, GuardError> {
    let tokens = tokenize(src)?;
    if tokens.is_empty() {
        return Err(GuardError::EmptyExpression);
    }
    let mut p = Parser {
        tokens: &tokens,
        pos: 0,
    };
    let node = p.parse_or(0)?;
    if p.pos < p.tokens.len() {
        return Err(GuardError::TrailingTokens {
            at: p.tokens[p.pos].at,
        });
    }
    Ok(node)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(|t| &t.kind)
    }

    fn peek_at(&self) -> usize {
        self.tokens
            .get(self.pos)
            .map(|t| t.at)
            .unwrap_or_else(|| self.tokens.last().map(|t| t.at).unwrap_or(0))
    }

    fn eat(&mut self, want: &TokenKind) -> bool {
        if self.peek() == Some(want) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, want: &TokenKind, what: &'static str) -> Result<(), GuardError> {
        if self.eat(want) {
            Ok(())
        } else {
            Err(GuardError::Expected {
                what,
                at: self.peek_at(),
            })
        }
    }

    fn guard_depth(&self, depth: u16) -> Result<(), GuardError> {
        if depth > MAX_DEPTH {
            return Err(GuardError::TooDeep {
                max: MAX_DEPTH,
                at: self.peek_at(),
            });
        }
        Ok(())
    }

    /// `or := and ('or' or)?` —— **右结合**，见模块文档。
    fn parse_or(&mut self, depth: u16) -> Result<Node, GuardError> {
        self.guard_depth(depth)?;
        let lhs = self.parse_and(depth)?;
        if self.eat(&TokenKind::Keyword(Keyword::Or)) {
            let rhs = self.parse_or(depth + 1)?;
            return Ok(Node::Or(Box::new(lhs), Box::new(rhs)));
        }
        Ok(lhs)
    }

    /// `and := not ('and' and)?` —— 右结合。
    fn parse_and(&mut self, depth: u16) -> Result<Node, GuardError> {
        self.guard_depth(depth)?;
        let lhs = self.parse_not(depth)?;
        if self.eat(&TokenKind::Keyword(Keyword::And)) {
            let rhs = self.parse_and(depth + 1)?;
            return Ok(Node::And(Box::new(lhs), Box::new(rhs)));
        }
        Ok(lhs)
    }

    /// `not := 'not' not | cmp`。`not` 比比较**松**，故 `not a == b` 是 `not (a == b)`。
    fn parse_not(&mut self, depth: u16) -> Result<Node, GuardError> {
        self.guard_depth(depth)?;
        if self.eat(&TokenKind::Keyword(Keyword::Not)) {
            let inner = self.parse_not(depth + 1)?;
            return Ok(Node::Not(Box::new(inner)));
        }
        self.parse_cmp(depth)
    }

    /// `cmp := primary (cmpop primary | 'in' '[' lits ']' | 'is' ['not'] 'null')?`
    ///
    /// **不可链式**：`a < b < c` 是解析错误。链式比较在别的语言里语义各不相同
    /// （有的是 `(a<b)<c`，有的是区间），写下它的人一定有一个具体的期待，
    /// 而这三种期待里至少两种会被静默判成另一种。
    fn parse_cmp(&mut self, depth: u16) -> Result<Node, GuardError> {
        let lhs = self.parse_primary(depth)?;
        let at = self.peek_at();
        if let Some(TokenKind::Cmp(op)) = self.peek() {
            let op = *op;
            self.pos += 1;
            let rhs = self.parse_primary(depth)?;
            if matches!(self.peek(), Some(TokenKind::Cmp(_))) {
                return Err(GuardError::ChainedComparison { at: self.peek_at() });
            }
            return Ok(Node::Cmp {
                op,
                at,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
        if self.eat(&TokenKind::Keyword(Keyword::In)) {
            let set = self.parse_literal_set()?;
            return Ok(Node::In {
                at,
                lhs: Box::new(lhs),
                set,
            });
        }
        if self.eat(&TokenKind::Keyword(Keyword::Is)) {
            let negated = self.eat(&TokenKind::Keyword(Keyword::Not));
            self.expect(&TokenKind::Keyword(Keyword::Null), "is 之后应为 null")?;
            return Ok(Node::IsNull {
                at,
                negated,
                operand: Box::new(lhs),
            });
        }
        Ok(lhs)
    }

    /// `in` 右侧只收**同类字面量**，不收表达式、不收变量、不收 `null`。
    ///
    /// 不收变量是因为变量里取不出集合；不收 `null` 是因为空判定只有
    /// `is null` 一条路——`x in [null]` 会让「空」有第二种问法，而两种问法
    /// 在空语义上必然不一致。
    fn parse_literal_set(&mut self) -> Result<Vec<GuardValue>, GuardError> {
        self.expect(&TokenKind::LBracket, "in 之后应为 [")?;
        let mut set: Vec<GuardValue> = Vec::new();
        if self.eat(&TokenKind::RBracket) {
            return Err(GuardError::EmptySet { at: self.peek_at() });
        }
        loop {
            let at = self.peek_at();
            let v = self.parse_set_literal(at)?;
            if let Some(first) = set.first() {
                if first.type_name() != v.type_name() {
                    return Err(GuardError::MixedSetTypes {
                        at,
                        first: first.type_name(),
                        found: v.type_name(),
                    });
                }
            }
            set.push(v);
            if self.eat(&TokenKind::Comma) {
                continue;
            }
            self.expect(&TokenKind::RBracket, "集合应以 ] 收尾")?;
            return Ok(set);
        }
    }

    fn parse_set_literal(&mut self, at: usize) -> Result<GuardValue, GuardError> {
        match self.peek().cloned() {
            Some(TokenKind::Number(raw)) => {
                self.pos += 1;
                number_value(&raw, at)
            }
            Some(TokenKind::Text(s)) => {
                self.pos += 1;
                Ok(GuardValue::Text(s))
            }
            Some(TokenKind::Keyword(Keyword::True)) => {
                self.pos += 1;
                Ok(GuardValue::Bool(true))
            }
            Some(TokenKind::Keyword(Keyword::False)) => {
                self.pos += 1;
                Ok(GuardValue::Bool(false))
            }
            Some(TokenKind::Keyword(Keyword::Null)) => Err(GuardError::NullInSet { at }),
            _ => Err(GuardError::Expected {
                what: "集合元素应为字面量",
                at,
            }),
        }
    }

    fn parse_primary(&mut self, depth: u16) -> Result<Node, GuardError> {
        self.guard_depth(depth)?;
        let at = self.peek_at();
        match self.peek().cloned() {
            Some(TokenKind::LParen) => {
                self.pos += 1;
                let inner = self.parse_or(depth + 1)?;
                self.expect(&TokenKind::RParen, "括号未闭合")?;
                Ok(inner)
            }
            Some(TokenKind::Number(raw)) => {
                self.pos += 1;
                Ok(Node::Lit(number_value(&raw, at)?))
            }
            Some(TokenKind::Text(s)) => {
                self.pos += 1;
                Ok(Node::Lit(GuardValue::Text(s)))
            }
            Some(TokenKind::Keyword(Keyword::True)) => {
                self.pos += 1;
                Ok(Node::Lit(GuardValue::Bool(true)))
            }
            Some(TokenKind::Keyword(Keyword::False)) => {
                self.pos += 1;
                Ok(Node::Lit(GuardValue::Bool(false)))
            }
            Some(TokenKind::Keyword(Keyword::Null)) => {
                self.pos += 1;
                Ok(Node::Lit(GuardValue::Null))
            }
            Some(TokenKind::Ident(name)) => {
                self.pos += 1;
                if self.peek() == Some(&TokenKind::LParen) {
                    return self.parse_call(&name, at, depth);
                }
                self.parse_path(&name, at)
            }
            _ => Err(GuardError::Expected {
                what: "此处应为一个取值",
                at,
            }),
        }
    }

    fn parse_path(&mut self, head: &str, at: usize) -> Result<Node, GuardError> {
        self.expect(&TokenKind::Dot, "字段引用应形如 vars.x 或 instance.state")?;
        let seg_at = self.peek_at();
        let Some(TokenKind::Ident(seg)) = self.peek().cloned() else {
            return Err(GuardError::Expected {
                what: "点号之后应为字段名",
                at: seg_at,
            });
        };
        self.pos += 1;
        // 多段路径在这里被拒。`vars.a.b` 是解析错误，不是未实现。
        if self.peek() == Some(&TokenKind::Dot) {
            return Err(GuardError::NestedPath { at: self.peek_at() });
        }
        match head {
            "vars" => Ok(Node::Path {
                path: PathRef::Var(seg),
                at,
            }),
            "instance" if seg == "state" => Ok(Node::Path {
                path: PathRef::InstanceState,
                at,
            }),
            "instance" => Err(GuardError::UnknownPath {
                at,
                head: "instance",
                seg,
            }),
            _ => Err(GuardError::UnknownPath {
                at,
                head: "（未知前缀）",
                seg,
            }),
        }
    }

    fn parse_call(&mut self, name: &str, at: usize, depth: u16) -> Result<Node, GuardError> {
        let Some(func) = Func::from_name(name) else {
            return Err(GuardError::UnknownFunction {
                at,
                name: name.to_string(),
            });
        };
        self.expect(&TokenKind::LParen, "函数调用应带括号")?;
        let mut args = Vec::new();
        if !self.eat(&TokenKind::RParen) {
            loop {
                args.push(self.parse_or(depth + 1)?);
                if self.eat(&TokenKind::Comma) {
                    continue;
                }
                self.expect(&TokenKind::RParen, "实参列表未闭合")?;
                break;
            }
        }
        if args.len() != func.arity() {
            return Err(GuardError::WrongArity {
                at,
                name: func.name(),
                want: func.arity(),
                got: args.len(),
            });
        }
        Ok(Node::Call { func, at, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(src: &str) -> Node {
        parse(src).unwrap_or_else(|e| panic!("{src:?} 应能解析，实为 {e}"))
    }

    #[test]
    fn field_references_take_the_two_forms_the_plan_names() {
        assert!(matches!(
            ok("vars.amount"),
            Node::Path { path: PathRef::Var(ref n), .. } if n == "amount"
        ));
        assert!(matches!(
            ok("instance.state"),
            Node::Path {
                path: PathRef::InstanceState,
                ..
            }
        ));
    }

    /// `vars.a.b` 是解析错误，不是未实现的 todo。
    /// 变量没有 schema，多段路径的每一段都无从校验。
    #[test]
    fn nested_paths_are_a_parse_error() {
        assert!(matches!(
            parse("vars.a.b"),
            Err(GuardError::NestedPath { .. })
        ));
    }

    /// `instance.` 下只有 `state` 一条路。
    #[test]
    fn instance_has_exactly_one_path() {
        assert!(matches!(
            parse("instance.foo"),
            Err(GuardError::UnknownPath { .. })
        ));
        assert!(matches!(
            parse("other.state"),
            Err(GuardError::UnknownPath { .. })
        ));
    }

    /// `not` 比比较松：`not a == b` 是 `not (a == b)`，与 SQL 一致。
    #[test]
    fn not_binds_looser_than_comparison() {
        let n = ok("not vars.a == 1");
        assert!(matches!(n, Node::Not(inner) if matches!(*inner, Node::Cmp { .. })));
    }

    /// `and` 比 `or` 紧：`a or b and c` 是 `a or (b and c)`。
    #[test]
    fn and_binds_tighter_than_or() {
        let n = ok("true or false and false");
        let Node::Or(_, rhs) = n else {
            panic!("顶层应是 or")
        };
        assert!(matches!(*rhs, Node::And(..)), "右侧应是 and");
    }

    /// 链式比较是错误。`a < b < c` 在不同语言里语义各不相同，
    /// 写下它的人一定有一个具体期待，而三种期待里至少两种会被静默判成另一种。
    #[test]
    fn chained_comparison_is_refused() {
        assert!(matches!(
            parse("1 < 2 < 3"),
            Err(GuardError::ChainedComparison { .. })
        ));
    }

    /// `and` / `or` 按右结合建树。这是深度闸门能同时管住链长的前提——
    /// 左结合循环产出左偏树，树高等于链长而括号深度为零，闸门拦不到。
    #[test]
    fn chains_are_right_associative_so_depth_tracks_chain_length() {
        let n = ok("true and true and true");
        let Node::And(_, rhs) = n else {
            panic!("顶层应是 and")
        };
        assert!(matches!(*rhs, Node::And(..)), "右侧应仍是 and，即右结合");
    }

    /// 深度闸门对**四条**路径同时生效。少任何一条都会留下一条能把栈打穿的输入。
    #[test]
    fn the_depth_gate_covers_every_nesting_construct() {
        let over = MAX_DEPTH as usize + 2;
        let cases = [
            (
                "括号",
                format!("{}true{}", "(".repeat(over), ")".repeat(over)),
            ),
            ("not 链", format!("{}true", "not ".repeat(over))),
            ("and 链", vec!["true"; over].join(" and ")),
            ("or 链", vec!["true"; over].join(" or ")),
        ];
        for (what, src) in cases {
            assert!(
                matches!(parse(&src), Err(GuardError::TooDeep { .. })),
                "{what} 未被深度闸门拦住——这条输入能把进程 abort"
            );
        }
    }

    /// 上限之内必须过得去，否则闸门就成了一条把合法表达式也拦掉的判据。
    #[test]
    fn just_inside_the_depth_limit_still_parses() {
        let n = MAX_DEPTH as usize;
        assert!(parse(&format!("{}true{}", "(".repeat(n), ")".repeat(n))).is_ok());
        assert!(parse(&vec!["true"; n].join(" and ")).is_ok());
    }

    /// 集合元素必须同类。混类型集合的成员判定要么恒假要么要一套隐式转换，
    /// 两条都是静默的。
    #[test]
    fn mixed_type_sets_are_refused() {
        assert!(matches!(
            parse("vars.x in [1, 'a']"),
            Err(GuardError::MixedSetTypes { .. })
        ));
        assert!(parse("vars.x in [1, 2, 3]").is_ok());
        assert!(parse("vars.x in ['a', 'b']").is_ok());
    }

    /// 空集合与集合里的 `null` 都拒。
    /// 空集合的成员判定恒假；`x in [null]` 会让「空」有第二种问法。
    #[test]
    fn empty_sets_and_null_in_sets_are_refused() {
        assert!(matches!(
            parse("vars.x in []"),
            Err(GuardError::EmptySet { .. })
        ));
        assert!(matches!(
            parse("vars.x in [null]"),
            Err(GuardError::NullInSet { .. })
        ));
    }

    #[test]
    fn is_null_and_is_not_null_both_parse() {
        assert!(matches!(
            ok("vars.x is null"),
            Node::IsNull { negated: false, .. }
        ));
        assert!(matches!(
            ok("vars.x is not null"),
            Node::IsNull { negated: true, .. }
        ));
    }

    /// 未登记的函数名一律拒——白名单不是「先解析再说」。
    #[test]
    fn unknown_functions_are_refused_at_parse_time() {
        assert!(matches!(
            parse("upper(vars.x)"),
            Err(GuardError::UnknownFunction { .. })
        ));
    }

    /// 元数在解析期就判，不留到求值期。
    #[test]
    fn wrong_arity_is_a_parse_error() {
        assert!(matches!(
            parse("len(vars.a, vars.b)"),
            Err(GuardError::WrongArity { .. })
        ));
        assert!(matches!(parse("len()"), Err(GuardError::WrongArity { .. })));
    }

    #[test]
    fn empty_and_trailing_garbage_are_refused() {
        assert!(matches!(parse("   "), Err(GuardError::EmptyExpression)));
        assert!(matches!(
            parse("true true"),
            Err(GuardError::TrailingTokens { .. })
        ));
        assert!(matches!(parse("(true"), Err(GuardError::Expected { .. })));
    }
}
