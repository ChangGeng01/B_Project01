//! 求值与步数计量。
//!
//! # 空值语义：既不是二值，也不是三值，是报错
//!
//! 计划对空一个字没写。三种选法：
//!
//! **二值**（空参与比较判假）在 `not` 下自相矛盾：`vars.x` 缺失时
//! `vars.x > 10` 为假，`not (vars.x > 10)` 为真——即「既不大于，也不是不大于」
//! 两条里必有一条被判成真。这是一条永远不会报错的错。
//!
//! **三值**把 unknown 一路传到顶层之后，仍然要在最后一刻折叠成一个布尔
//! 才能选出下一节点；那次折叠是整条链上最不可见的一次静默判定，
//! 等于把同一个决定推迟到最难查的地方。
//!
//! **本实现取第三条：空进比较、集合成员或函数实参，一律返回错误**，不返回布尔。
//! 空只在 `is null` / `is not null` 两个算子里合法——这正是计划把「空判定」
//! 单列为一个算子的用处。
//!
//! 判错两个方向的代价不对称，而且**恒真那一侧贵在事后不可区分**：
//! 判假会让实例选不出下一节点、停在原地、最终触及运行约束转人工干预——
//! 那是一个有工单、有告警、有当事人的故障。判真会让不该过的审批被放行，
//! 而落库记录与一次正常放行**逐字相同**，事后只能靠回头重算业务后果才能发现。
//! 报错则把「不知道」变成一个带变量名与字节偏移的事件，落在那一步事务上。
//!
//! # 短路是这套严格语义能用起来的前提
//!
//! `and` / `or` 从左到右短路，被短路的一侧**不求值也不计步**。
//! 于是可选变量的正确写法是 `vars.discount is null or vars.discount < 100`——
//! 它在 discount 缺失时返回真而不是报错。缺了短路，严格空语义就不可用。

use super::func::Func;
use super::lex::CmpOp;
use super::parse::{Node, PathRef};
use super::value::{GuardValue, VarLookup, MAX_VAR_TEXT_BYTES};
use super::GuardError;
use crate::state::InstanceState;

/// 求值所需的上下文。
pub struct EvalCtx<'a> {
    pub vars: &'a dyn VarLookup,
    /// 求值发生在推进路径上，实例此刻的状态。
    pub instance_state: InstanceState,
}

/// 一次求值的计量。
struct Meter {
    used: u16,
    max: u16,
}

impl Meter {
    /// 计一步。**在下降进该节点之前计**，不是之后——
    /// 之后计会让最后一层节点白嫖一步，边界就与「一步是什么」的定义对不上。
    fn tick(&mut self, at: usize) -> Result<(), GuardError> {
        self.used = self.used.saturating_add(1);
        if self.used > self.max {
            return Err(GuardError::StepLimitExceeded { max: self.max, at });
        }
        Ok(())
    }
}

/// 求值。返回一个布尔——守卫的用处就是选一条边。
///
/// # 一步是什么
///
/// 逐条钉死，否则 1000 是一个不可判的数：
///
/// 一、**每个在本次求值中被真正进入的语法树节点计一步**，在下降进它之前计；
/// 二、被短路掉的子树进不去，计 **0** 步；
/// 三、`in` 的列表**每检查一个元素计一步**，命中后其余元素不再检查，计 0 步；
/// 四、函数调用按调用节点计一步，不按实参的规模加计——
///     实参本身是节点，各自计各自的。
pub fn evaluate(node: &Node, ctx: &EvalCtx<'_>, max_steps: u16) -> Result<bool, GuardError> {
    let mut meter = Meter {
        used: 0,
        max: max_steps,
    };
    let v = eval_node(node, ctx, &mut meter)?;
    match v {
        GuardValue::Bool(b) => Ok(b),
        other => Err(GuardError::NotABoolean {
            got: other.type_name(),
        }),
    }
}

fn eval_node(node: &Node, ctx: &EvalCtx<'_>, m: &mut Meter) -> Result<GuardValue, GuardError> {
    m.tick(node_at(node))?;
    match node {
        Node::Lit(v) => Ok(v.clone()),
        Node::Path { path, at } => read_path(path, ctx, *at),
        Node::Not(inner) => match eval_node(inner, ctx, m)? {
            GuardValue::Bool(b) => Ok(GuardValue::Bool(!b)),
            other => Err(GuardError::NotABoolean {
                got: other.type_name(),
            }),
        },
        // 短路：左侧为假时右侧**不进入**，故计 0 步。
        Node::And(a, b) => {
            if !as_bool(eval_node(a, ctx, m)?)? {
                return Ok(GuardValue::Bool(false));
            }
            Ok(GuardValue::Bool(as_bool(eval_node(b, ctx, m)?)?))
        }
        Node::Or(a, b) => {
            if as_bool(eval_node(a, ctx, m)?)? {
                return Ok(GuardValue::Bool(true));
            }
            Ok(GuardValue::Bool(as_bool(eval_node(b, ctx, m)?)?))
        }
        Node::Cmp { op, at, lhs, rhs } => {
            let l = eval_node(lhs, ctx, m)?;
            let r = eval_node(rhs, ctx, m)?;
            compare(*op, &l, &r, *at).map(GuardValue::Bool)
        }
        Node::In { at, lhs, set } => {
            let l = eval_node(lhs, ctx, m)?;
            if l.is_null() {
                return Err(GuardError::NullOperand {
                    at: *at,
                    what: "集合成员判定的左操作数",
                });
            }
            for candidate in set {
                // 每检查一个元素计一步，命中即停。
                m.tick(*at)?;
                if candidate.type_name() != l.type_name() {
                    return Err(GuardError::TypeMismatch {
                        at: *at,
                        left: l.type_name(),
                        right: candidate.type_name(),
                    });
                }
                if candidate == &l {
                    return Ok(GuardValue::Bool(true));
                }
            }
            Ok(GuardValue::Bool(false))
        }
        Node::IsNull {
            negated, operand, ..
        } => {
            let v = eval_node(operand, ctx, m)?;
            Ok(GuardValue::Bool(v.is_null() != *negated))
        }
        Node::Call { func, at, args } => {
            let mut vals = Vec::with_capacity(args.len());
            for a in args {
                let v = eval_node(a, ctx, m)?;
                if v.is_null() {
                    return Err(GuardError::NullOperand {
                        at: *at,
                        what: "函数实参",
                    });
                }
                vals.push(v);
            }
            check_text_sizes(func, &vals, *at)?;
            func.call(&vals, *at)
        }
    }
}

fn as_bool(v: GuardValue) -> Result<bool, GuardError> {
    match v {
        GuardValue::Bool(b) => Ok(b),
        other => Err(GuardError::NotABoolean {
            got: other.type_name(),
        }),
    }
}

fn check_text_sizes(func: &Func, args: &[GuardValue], at: usize) -> Result<(), GuardError> {
    for a in args {
        if let GuardValue::Text(s) = a {
            if s.len() > MAX_VAR_TEXT_BYTES {
                return Err(GuardError::TextTooLong {
                    at,
                    func: func.name(),
                    len: s.len(),
                    max: MAX_VAR_TEXT_BYTES,
                });
            }
        }
    }
    Ok(())
}

fn read_path(path: &PathRef, ctx: &EvalCtx<'_>, at: usize) -> Result<GuardValue, GuardError> {
    match path {
        PathRef::Var(name) => Ok(ctx.vars.get(name).unwrap_or(GuardValue::Null)),
        PathRef::InstanceState => {
            let _ = at;
            Ok(GuardValue::Text(
                ctx.instance_state.as_db_value().to_string(),
            ))
        }
    }
}

/// 比较。
///
/// **跨类型是错误，不是假。** 判假会让 `vars.qty > '10'`（右侧误写成文本）
/// 静默走另一条边；报错会让写它的人当场知道。
///
/// **布尔只能判等与不等。** `true < false` 没有业务含义，
/// 给它一个次序等于替作者发明一个他没写的口径。
///
/// 文本按**字节序**比较，与 `LC_COLLATE 'C'` 的库侧默认排序同源
/// （见 ADR-0003）；两处不同源会让同一条判据在应用层与在 SQL 里给出不同答案。
fn compare(op: CmpOp, l: &GuardValue, r: &GuardValue, at: usize) -> Result<bool, GuardError> {
    if l.is_null() || r.is_null() {
        return Err(GuardError::NullOperand {
            at,
            what: "比较的操作数",
        });
    }
    if l.type_name() != r.type_name() {
        return Err(GuardError::TypeMismatch {
            at,
            left: l.type_name(),
            right: r.type_name(),
        });
    }
    let ordering = match (l, r) {
        (GuardValue::Number(a), GuardValue::Number(b)) => a.partial_cmp(b),
        (GuardValue::Text(a), GuardValue::Text(b)) => Some(a.as_bytes().cmp(b.as_bytes())),
        (GuardValue::Bool(a), GuardValue::Bool(b)) => {
            if matches!(op, CmpOp::Eq | CmpOp::Ne) {
                Some(a.cmp(b))
            } else {
                return Err(GuardError::BoolNotOrdered {
                    at,
                    op: op.as_source(),
                });
            }
        }
        _ => None,
    };
    let Some(ord) = ordering else {
        return Err(GuardError::TypeMismatch {
            at,
            left: l.type_name(),
            right: r.type_name(),
        });
    };
    Ok(match op {
        CmpOp::Eq => ord.is_eq(),
        CmpOp::Ne => ord.is_ne(),
        CmpOp::Lt => ord.is_lt(),
        CmpOp::Le => ord.is_le(),
        CmpOp::Gt => ord.is_gt(),
        CmpOp::Ge => ord.is_ge(),
    })
}

fn node_at(node: &Node) -> usize {
    match node {
        Node::Path { at, .. }
        | Node::Cmp { at, .. }
        | Node::In { at, .. }
        | Node::IsNull { at, .. }
        | Node::Call { at, .. } => *at,
        Node::Lit(_) | Node::Not(_) | Node::And(..) | Node::Or(..) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Guard, DEFAULT_MAX_STEPS};
    use std::collections::BTreeMap;

    fn vars(pairs: &[(&str, GuardValue)]) -> BTreeMap<String, GuardValue> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    fn num(s: &str) -> GuardValue {
        GuardValue::number(s).expect("夹具数字应合法")
    }

    fn run(src: &str, v: &BTreeMap<String, GuardValue>) -> Result<bool, GuardError> {
        run_with(src, v, InstanceState::Running, DEFAULT_MAX_STEPS)
    }

    fn run_with(
        src: &str,
        v: &BTreeMap<String, GuardValue>,
        state: InstanceState,
        max: u16,
    ) -> Result<bool, GuardError> {
        let g = Guard::parse(src).unwrap_or_else(|e| panic!("{src:?} 应能解析，实为 {e}"));
        g.evaluate(
            &EvalCtx {
                vars: v,
                instance_state: state,
            },
            max,
        )
    }

    /// 六种比较各走一遍。
    #[test]
    fn all_six_comparisons_work() {
        let v = vars(&[("a", num("10"))]);
        for (src, want) in [
            ("vars.a == 10", true),
            ("vars.a != 10", false),
            ("vars.a < 11", true),
            ("vars.a <= 10", true),
            ("vars.a > 9", true),
            ("vars.a >= 11", false),
        ] {
            assert_eq!(run(src, &v), Ok(want), "{src}");
        }
    }

    /// 端到端的金额算例：本模块存在的头号理由。
    /// 两笔差一分的大额金额，`f64` 承载下按位相等而这里必须分得开。
    #[test]
    fn a_one_cent_difference_survives_the_whole_pipeline() {
        let v = vars(&[("amount", num("70368744177664.02"))]);
        assert_eq!(
            run("vars.amount > 70368744177664.01", &v),
            Ok(true),
            "更大的一笔必须判成更大；f64 承载下这里会返回 false"
        );
        let v2 = vars(&[("amount", num("9999999999999999.99"))]);
        assert_eq!(
            run("vars.amount >= 10000000000000000.00", &v2),
            Ok(false),
            "f64 承载下这笔会被舍到 1e16 从而判成成立"
        );
    }

    /// 空进比较是错误，不是假。二值语义会让
    /// `vars.x > 10` 与 `not (vars.x > 10)` 里必有一条被判成真。
    #[test]
    fn null_in_a_comparison_is_an_error_not_false() {
        let empty = vars(&[]);
        assert!(matches!(
            run("vars.x > 10", &empty),
            Err(GuardError::NullOperand { .. })
        ));
        // 取反也报错，而不是变成真——这就是二值语义自相矛盾的那一对。
        assert!(matches!(
            run("not (vars.x > 10)", &empty),
            Err(GuardError::NullOperand { .. })
        ));
    }

    /// 空进集合成员判定同样报错。这一条最容易漏——
    /// 比较那条判到了，`in` 那条忘了判，缺陷原样留着。
    #[test]
    fn null_in_a_set_membership_is_an_error() {
        assert!(matches!(
            run("vars.x in [1, 2, 3]", &vars(&[])),
            Err(GuardError::NullOperand { .. })
        ));
    }

    /// 空进函数实参同样报错。同上，第三条容易漏的路径。
    #[test]
    fn null_as_a_function_argument_is_an_error() {
        assert!(matches!(
            run("len(vars.x) > 0", &vars(&[])),
            Err(GuardError::NullOperand { .. })
        ));
    }

    /// 空只在两个空判定算子里合法。
    #[test]
    fn is_null_is_the_only_place_null_is_welcome() {
        let empty = vars(&[]);
        assert_eq!(run("vars.x is null", &empty), Ok(true));
        assert_eq!(run("vars.x is not null", &empty), Ok(false));
        let some = vars(&[("x", num("1"))]);
        assert_eq!(run("vars.x is null", &some), Ok(false));
        assert_eq!(run("vars.x is not null", &some), Ok(true));
    }

    /// 短路是严格空语义能用起来的前提：可选变量的正确写法在缺失时返回真而不报错。
    #[test]
    fn short_circuit_makes_optional_variables_writable() {
        let empty = vars(&[]);
        assert_eq!(
            run("vars.discount is null or vars.discount < 100", &empty),
            Ok(true)
        );
        let some = vars(&[("discount", num("50"))]);
        assert_eq!(
            run("vars.discount is null or vars.discount < 100", &some),
            Ok(true)
        );
        let big = vars(&[("discount", num("500"))]);
        assert_eq!(
            run("vars.discount is null or vars.discount < 100", &big),
            Ok(false)
        );
    }

    /// `and` 与 `or` 都要短路，且**被短路的一侧连错误都不产生**。
    /// 只测其中一个的话，另一个急求值的实现照样全绿。
    #[test]
    fn both_and_and_or_short_circuit_past_errors() {
        let empty = vars(&[]);
        assert_eq!(run("false and vars.x > 1", &empty), Ok(false), "and 未短路");
        assert_eq!(run("true or vars.x > 1", &empty), Ok(true), "or 未短路");
        // 反向：没被短路掉的那一侧必须照常报错。
        assert!(run("true and vars.x > 1", &empty).is_err());
        assert!(run("false or vars.x > 1", &empty).is_err());
    }

    /// 被短路的一侧**不计步**。这条与上一条不是重复：
    /// 上一条测的是错误不产生，这一条测的是步数不增加——
    /// 一个「求值右侧但吞掉错误」的实现能过上一条，过不了这一条。
    #[test]
    fn the_short_circuited_side_costs_no_steps() {
        let v = vars(&[]);
        // `true and true` 要 3 步：And 一步，两个字面量各一步。
        assert!(matches!(
            run_with("true and true", &v, InstanceState::Running, 2),
            Err(GuardError::StepLimitExceeded { .. })
        ));
        // `false and true` 只走 2 步——右侧进不去。
        assert_eq!(
            run_with("false and true", &v, InstanceState::Running, 2),
            Ok(false),
            "右侧被短路后仍计步，说明短路只做了一半"
        );
    }

    /// 步数上限**可达**，且边界精确。
    /// 一个永远达不到的上限就是没有上限——本卷已在八档退避上栽过一次。
    ///
    /// `vars.n in [1,1,…]`：In 节点一步、左操作数一步、每检查一个元素一步，
    /// 故 N 个元素恰好 N+2 步。
    #[test]
    fn the_step_limit_is_reachable_and_its_boundary_is_exact() {
        let v = vars(&[("n", num("999"))]); // 不命中任何元素，逐个检查到底
        let set = |n: usize| format!("vars.n in [{}]", vec!["1"; n].join(","));

        let exact = set(DEFAULT_MAX_STEPS as usize - 2); // 998 个元素 = 1000 步
        assert_eq!(
            run(&exact, &v),
            Ok(false),
            "恰好用满 {DEFAULT_MAX_STEPS} 步应通过——上限是「不得超过」"
        );

        let over = set(DEFAULT_MAX_STEPS as usize - 1); // 999 个元素 = 1001 步
        assert!(
            matches!(run(&over, &v), Err(GuardError::StepLimitExceeded { .. })),
            "多一个元素就该超限"
        );

        // 这条源文本必须在长度上限之内，否则两条上限互相架空、
        // 「1000 步可达」只是算术上成立而实际取不到。
        assert!(over.len() < crate::expr::MAX_SOURCE_BYTES);
    }

    /// 计步不止在 `in` 一条路上。只在集合那一维测的话，
    /// 一个「只给集合元素计费、别的节点一律零费」的实现照样全绿。
    #[test]
    fn steps_are_counted_outside_of_set_membership_too() {
        let v = vars(&[("a", num("1"))]);
        // Cmp 一步、Path 一步、字面量一步 = 3 步。
        assert!(matches!(
            run_with("vars.a == 1", &v, InstanceState::Running, 2),
            Err(GuardError::StepLimitExceeded { .. })
        ));
        assert_eq!(
            run_with("vars.a == 1", &v, InstanceState::Running, 3),
            Ok(true)
        );
    }

    /// 跨类型是错误，不是假。判假会让右侧误写成文本的守卫静默走另一条边。
    #[test]
    fn cross_type_comparison_is_an_error_not_false() {
        let v = vars(&[("qty", num("10"))]);
        assert!(matches!(
            run("vars.qty > '10'", &v),
            Err(GuardError::TypeMismatch { .. })
        ));
    }

    /// 布尔之间没有次序。给它一个次序等于替作者发明一个他没写的口径。
    #[test]
    fn booleans_are_not_ordered() {
        let v = vars(&[("f", GuardValue::Bool(true))]);
        assert!(matches!(
            run("vars.f < false", &v),
            Err(GuardError::BoolNotOrdered { .. })
        ));
        assert_eq!(run("vars.f == true", &v), Ok(true));
        assert_eq!(run("vars.f != false", &v), Ok(true));
    }

    /// `instance.state` 读的是上下文里的实例状态，与 `as_db_value` 同源。
    #[test]
    fn instance_state_reads_the_context() {
        let v = vars(&[]);
        assert_eq!(
            run_with(
                "instance.state == 'RUNNING'",
                &v,
                InstanceState::Running,
                100
            ),
            Ok(true)
        );
        assert_eq!(
            run_with(
                "instance.state == 'RUNNING'",
                &v,
                InstanceState::Waiting,
                100
            ),
            Ok(false)
        );
        assert_eq!(
            run_with(
                "instance.state in ['RUNNING', 'WAITING']",
                &v,
                InstanceState::Waiting,
                100
            ),
            Ok(true)
        );
    }

    /// 顶层结果必须是布尔。`vars.x` 单独作为守卫、或 `len(vars.s)` 单独作为守卫，
    /// 都不是一个能选边的东西。
    #[test]
    fn the_top_level_result_must_be_a_boolean() {
        let v = vars(&[("s", GuardValue::text("abc"))]);
        assert!(matches!(
            run("len(vars.s)", &v),
            Err(GuardError::NotABoolean { got: "数字" })
        ));
    }

    /// 错误文案不得携带操作数取值——守卫按薪资、按金额分流，
    /// 而错误文案会落进日志与工单，读得到它的人未必读得到那个字段。
    #[test]
    fn error_messages_never_carry_operand_values() {
        let v = vars(&[("salary", num("987654321"))]);
        let err = run("vars.salary > '0'", &v).expect_err("跨类型应报错");
        let msg = err.to_string();
        assert!(
            msg.contains("数字") && msg.contains("文本"),
            "应报类型，实为 {msg}"
        );
        assert!(!msg.contains("987654321"), "不得携带取值，实为 {msg}");
    }

    /// 全部拒绝理由映射到同一个已登记的错误码。
    #[test]
    fn every_rejection_carries_the_registered_code() {
        let v = vars(&[]);
        let errs = [
            run("vars.x > 1", &v).expect_err("空操作数"),
            run_with("true and true", &v, InstanceState::Running, 1).expect_err("步数超限"),
        ];
        for e in errs {
            assert_eq!(e.error_code(), "PLATFORM.FLOW.GUARD_EXPRESSION_INVALID");
        }
    }

    /// 函数在求值里走通，且日期比较靠定宽文本的字典序成立。
    #[test]
    fn functions_evaluate_end_to_end() {
        let v = vars(&[
            ("s", GuardValue::text("中文字")),
            ("start", GuardValue::text("2026-01-31")),
            ("due", GuardValue::text("2026-03-01")),
        ]);
        assert_eq!(run("len(vars.s) == 3", &v), Ok(true));
        assert_eq!(run("ceil(1.2) == 2", &v), Ok(true));
        assert_eq!(run("vars.due > date_add_days(vars.start, 1)", &v), Ok(true));
        assert_eq!(
            run("date_add_days(vars.start, 1) == '2026-02-01'", &v),
            Ok(true)
        );
    }
}
