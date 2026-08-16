//! 守卫条件表达式的最小求值器。
//!
//! 需求逐字取自阶段 3 计划第 3.4.8 节：「守卫条件表达式：本阶段只交付最小求值器，
//! 支持字段引用（`vars.x`、`instance.state`）、比较（六种）、逻辑（与或非）、
//! 集合成员、空判定，以及一个不超过 12 个函数的白名单（长度、上取整、日期加减等）。
//! 表达式无副作用、无循环、求值步数上限 1000，超限返回 `VALIDATION`。
//! 该求值器只服务于流程守卫条件，不是 `RuleEvaluator` 的实现。」
//!
//! # 三条判错方向不对称的地方，各自的处置
//!
//! 一、**数字不用 `f64`**，用与基线第 3.5 节同源的 `rust_decimal::Decimal`。
//! 见 [`value`]：两笔差一分的大额金额在 `f64` 下按位相等，
//! 守卫会把更大的一笔判成不大于，无异常、无日志。
//!
//! 二、**空一律报错**，不折叠成布尔。见 [`eval`]：判假会产生一个有人看得见的故障，
//! 判真会产生一条与正常放行逐字相同的记录。
//!
//! 三、**深度闸门在解析期**，不靠求值步数。见 [`parse`]：栈溢出是 abort 不是 panic，
//! `Result` 接不住，而 1000 步的计数器在深嵌套输入上一次都走不到。
//!
//! # 本轮不交付什么
//!
//! 不交付表达式的**存储形态**（挂在 `definition jsonb` 的哪个键）——全卷无出处；
//! 不交付**调用点**（计划把守卫求值放在 job-worker 的单步事务里）；
//! 不交付 `RuleEvaluator` 与 `WasmComputePort` 两个端口——计划逐字说本求值器
//! 「不是 `RuleEvaluator` 的实现」，两者归阶段 13b。
//!
//! # 未覆盖（明写，不以「校验过了」的外观掩盖）
//!
//! 一、**「本 crate 不引 serde_json」这条没有机检承接。** 它是本模块数值精度的
//! 全部前提（见 [`value`]），但 `archcheck` 只判层位与环、不按 crate 比对依赖清单，
//! 一次 `cargo add serde_json` 就能把它推翻而六道门禁全绿。
//!
//! 二、**发布期没有承接方调 [`Guard::parse`]。** 本模块把能在发布期判的全部前移到
//! 解析（文法、函数名与元数、路径形态、深度、长度、数字字面量精度），
//! 但 `FlowDefinitionApplier::validate` 的签名里没有任何一处要求它校验守卫。
//! 不接的话，一条语法错的守卫会一直躺到某个实例走到那条边的那一刻才炸。
//!
//! 三、**变量名无法在解析期校验。** 全卷没有创建实例的端点，`vars` 的自由变量集合
//! 无出处，`vars.amout` 这类拼写错只能在求值期以「空操作数」报出来。
//!
//! 四、**求值失败之后实例往哪走，计划没写。** 计划第 3.4.8 节那张状态机表里
//! 没有任何一行的触发是「守卫求值失败」，按那张表这个情形无迁移可走。
//!
//! 五、**未经 [`func`] 的日期文本，格式无人校验。** 日期在本模块里是定宽
//! ISO-8601 文本；`date_add_days` 拒绝非定宽形态，但一个直接写
//! `vars.due > '2026-9-1'` 的守卫按字典序比较会得到错的答案且不报错。
//! 要根治须有变量 schema，而变量 schema 不存在。
//!
//! 六、**「白名单不超过 12 个」这条验收在本轮是恒真的**，本轮交付 3 个。
//!
//! 七、**变量的个数没有上限。** [`value::MAX_VAR_TEXT_BYTES`] 管住单个文本，
//! 管不住 `variables` 里有多少个键；本模块靠 [`value::VarLookup`] 按名取数
//! 把它挡在门外（只有被引用的键才会被取），但那要求调用方照着用。

pub mod eval;
pub mod func;
pub mod lex;
pub mod parse;
pub mod value;

pub use eval::{evaluate, EvalCtx};
pub use func::{Func, ALL_FUNCS, MAX_WHITELIST_FUNCS};
pub use lex::CmpOp;
pub use parse::{Node, PathRef};
pub use value::{GuardValue, NoVars, VarLookup, MAX_VAR_TEXT_BYTES};

use crate::state::InstanceState;

/// 源文本字节上限。计划对长度一个字没写。
///
/// 不设它就没有任何东西约束进入词法器的输入规模，而分词与建树发生在
/// 持 `FOR UPDATE` 行锁的单步事务内。它同时是「1000 步可达」的算术前提：
/// `in` 列表里最便宜的一个元素是 `1,` 两个字节，4096 字节能容下约两千个，
/// 故 1000 步在本上限之内取得到——两条上限不互相架空。
pub const MAX_SOURCE_BYTES: usize = 4096;

/// 嵌套深度上限。见 [`parse`] 的模块文档。
pub const MAX_DEPTH: u16 = 32;

/// 求值步数上限的默认值。计划第 3.4.8 节逐字 1000。
///
/// 类型取 `u16` 与计划第 3.7 节已登记的配置键
/// `EP__PLATFORM__FLOW__EXPRESSION_MAX_STEPS`（`u16`，默认 1000）一致，
/// 免得配置键落地时要做一次没有承接方的窄化转换。
/// 做成 [`Guard::evaluate`] 的入参而不是硬常量，是为了让配置键落地时不必改求值器。
pub const DEFAULT_MAX_STEPS: u16 = 1000;

/// 求值时实例可能处的状态。
///
/// 计划第 3.4.8 节逐字给出单步事务的动作顺序：「加载实例行并 `FOR UPDATE`，
/// 按 `definition_version` 加载定义，**求值守卫条件选出下一节点**，执行该节点，
/// 写 `process_steps`，更新实例状态与 `next_wake_at`……」——
/// 求值发生在**更新实例状态之前**，且在推进路径上。按状态机表能走到推进的来源态
/// 只有 `Running`（步骤成功且有后继）与 `Waiting`（唤醒，且它先迁到 `Running`）。
///
/// 于是 `instance.state` 的另外六个取值在生产上**恒假**：`CREATED` 只在首次派发
/// 之前存在而那一步无守卫；三个终态按状态机表没有出边；`COMPENSATING` 走的是
/// 按步号降序的补偿路径，不经守卫；`MANUAL_INTERVENTION` 恢复后先迁到 `RUNNING`。
///
/// 本模块**不在解析期拒绝那六个字面量**——计划只说 `instance.state` 是一个字段引用，
/// 没有限定取值域，替它限定就是自造规格。改为把事实暴露成
/// [`Guard::unreachable_state_literals`]，让发布期的校验方能报出来。
pub const GUARD_TIME_STATES: [InstanceState; 2] = [InstanceState::Running, InstanceState::Waiting];

/// 一条已解析的守卫表达式。
///
/// 拿不到它就求不了值——文法、白名单、深度与长度都在 [`Guard::parse`] 里判完。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Guard {
    root: Node,
}

impl Guard {
    pub fn parse(src: &str) -> Result<Self, GuardError> {
        Ok(Self {
            root: parse::parse(src)?,
        })
    }

    /// 求值。`max_steps` 通常取 [`DEFAULT_MAX_STEPS`]。
    pub fn evaluate(&self, ctx: &EvalCtx<'_>, max_steps: u16) -> Result<bool, GuardError> {
        eval::evaluate(&self.root, ctx, max_steps)
    }

    /// 本表达式引用了哪些 `vars` 键。
    ///
    /// 有它，调用方才能只取被引用的键而不是把整份 `variables` 转一遍——
    /// 见 [`value::VarLookup`] 的文档。
    pub fn referenced_vars(&self) -> Vec<String> {
        let mut out = Vec::new();
        walk(&self.root, &mut |n| {
            if let Node::Path {
                path: PathRef::Var(name),
                ..
            } = n
            {
                if !out.contains(name) {
                    out.push(name.clone());
                }
            }
        });
        out.sort();
        out
    }

    /// 本表达式里与 `instance.state` 比较的、在求值时刻**取不到**的状态字面量。
    ///
    /// 返回非空即说明写这条守卫的人理解错了求值时机——那条分支永远不成立。
    /// 见 [`GUARD_TIME_STATES`]。
    pub fn unreachable_state_literals(&self) -> Vec<String> {
        let reachable: Vec<&str> = GUARD_TIME_STATES.iter().map(|s| s.as_db_value()).collect();
        let mut out: Vec<String> = Vec::new();
        let mut push = |lit: &GuardValue| {
            if let GuardValue::Text(s) = lit {
                if !reachable.contains(&s.as_str()) && !out.contains(s) {
                    out.push(s.clone());
                }
            }
        };
        walk(&self.root, &mut |n| match n {
            Node::Cmp { lhs, rhs, .. } => {
                if is_state_path(lhs) {
                    if let Node::Lit(v) = rhs.as_ref() {
                        push(v);
                    }
                }
                if is_state_path(rhs) {
                    if let Node::Lit(v) = lhs.as_ref() {
                        push(v);
                    }
                }
            }
            Node::In { lhs, set, .. } if is_state_path(lhs) => {
                for v in set {
                    push(v);
                }
            }
            _ => {}
        });
        out.sort();
        out
    }
}

fn is_state_path(n: &Node) -> bool {
    matches!(
        n,
        Node::Path {
            path: PathRef::InstanceState,
            ..
        }
    )
}

fn walk(node: &Node, f: &mut impl FnMut(&Node)) {
    f(node);
    match node {
        Node::Lit(_) | Node::Path { .. } => {}
        Node::Not(a) | Node::IsNull { operand: a, .. } | Node::In { lhs: a, .. } => walk(a, f),
        Node::And(a, b) | Node::Or(a, b) | Node::Cmp { lhs: a, rhs: b, .. } => {
            walk(a, f);
            walk(b, f);
        }
        Node::Call { args, .. } => {
            for a in args {
                walk(a, f);
            }
        }
    }
}

/// 守卫表达式被拒的原因。
///
/// 全部映射到同一个错误码 `PLATFORM.FLOW.GUARD_EXPRESSION_INVALID`
/// （分类 `VALIDATION`，与计划第 3.4.8 节逐字「超限返回 `VALIDATION`」一致）；
/// 具体理由由变体承载，不为每一种拒绝各登记一个码。
///
/// # 一条纪律：错误文案里不出现操作数的取值
///
/// 守卫是按金额、按薪资这类东西分流的，而错误文案会落进日志与工单，
/// 读得到它的人未必读得到那个字段。因此类型不符只报**类型**，
/// 空操作数只报**变量名与位置**——变量名是管理员自己写下的，不是数据。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GuardError {
    SourceTooLong {
        len: usize,
        max: usize,
    },
    EmptyExpression,
    UnexpectedChar {
        at: usize,
        ch: char,
    },
    MalformedNumber {
        at: usize,
    },
    MalformedText {
        at: usize,
    },
    UnterminatedText {
        at: usize,
    },
    Expected {
        what: &'static str,
        at: usize,
    },
    TrailingTokens {
        at: usize,
    },
    TooDeep {
        max: u16,
        at: usize,
    },
    NestedPath {
        at: usize,
    },
    UnknownPath {
        at: usize,
        head: &'static str,
        seg: String,
    },
    UnknownFunction {
        at: usize,
        name: String,
    },
    WrongArity {
        at: usize,
        name: &'static str,
        want: usize,
        got: usize,
    },
    ChainedComparison {
        at: usize,
    },
    EmptySet {
        at: usize,
    },
    NullInSet {
        at: usize,
    },
    MixedSetTypes {
        at: usize,
        first: &'static str,
        found: &'static str,
    },
    /// 空进了比较、集合成员或函数实参。见 [`eval`] 的模块文档。
    NullOperand {
        at: usize,
        what: &'static str,
    },
    TypeMismatch {
        at: usize,
        left: &'static str,
        right: &'static str,
    },
    BoolNotOrdered {
        at: usize,
        op: &'static str,
    },
    NotABoolean {
        got: &'static str,
    },
    NotAWholeNumber {
        at: usize,
        func: &'static str,
    },
    BadDateFormat {
        at: usize,
    },
    DateOutOfRange {
        at: usize,
    },
    BadArgumentType {
        at: usize,
        func: &'static str,
        got: Vec<&'static str>,
    },
    TextTooLong {
        at: usize,
        func: &'static str,
        len: usize,
        max: usize,
    },
    StepLimitExceeded {
        max: u16,
        at: usize,
    },
}

impl GuardError {
    pub fn error_code(&self) -> &'static str {
        ep_foundation::error::codes::PLATFORM_FLOW_GUARD_EXPRESSION_INVALID.0
    }
}

impl std::fmt::Display for GuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardError::SourceTooLong { len, max } => {
                write!(f, "守卫表达式长度 {len} 字节超过上限 {max}")
            }
            GuardError::EmptyExpression => f.write_str("守卫表达式为空"),
            GuardError::UnexpectedChar { at, ch } => {
                write!(f, "第 {at} 字节处出现不允许的字符 {ch:?}")
            }
            GuardError::MalformedNumber { at } => write!(
                f,
                "第 {at} 字节处的数字写法不合法；只接受十进制形态，不接受指数与前导符号"
            ),
            GuardError::MalformedText { at } => write!(f, "第 {at} 字节处的字符串不是合法 UTF-8"),
            GuardError::UnterminatedText { at } => {
                write!(f, "第 {at} 字节处的字符串没有闭合的单引号")
            }
            GuardError::Expected { what, at } => write!(f, "第 {at} 字节处：{what}"),
            GuardError::TrailingTokens { at } => {
                write!(f, "第 {at} 字节处还有多余内容，表达式已在此之前结束")
            }
            GuardError::TooDeep { max, at } => write!(
                f,
                "第 {at} 字节处嵌套超过 {max} 层；括号、not 链、and 链与 or 链共用这一个上限"
            ),
            GuardError::NestedPath { at } => write!(
                f,
                "第 {at} 字节处：字段引用只支持单段，vars.a.b 这类写法不受支持"
            ),
            GuardError::UnknownPath { at, head, seg } => {
                write!(f, "第 {at} 字节处：{head} 下没有名为 {seg} 的字段")
            }
            GuardError::UnknownFunction { at, name } => write!(
                f,
                "第 {at} 字节处：{name} 不在函数白名单内；白名单为 {}",
                ALL_FUNCS.map(|x| x.name()).join("、")
            ),
            GuardError::WrongArity {
                at,
                name,
                want,
                got,
            } => write!(
                f,
                "第 {at} 字节处：{name} 需要 {want} 个实参，实为 {got} 个"
            ),
            GuardError::ChainedComparison { at } => write!(
                f,
                "第 {at} 字节处：比较不可串联；a < b < c 请写成 a < b and b < c"
            ),
            GuardError::EmptySet { at } => {
                write!(f, "第 {at} 字节处：集合不得为空，空集合的成员判定恒假")
            }
            GuardError::NullInSet { at } => {
                write!(f, "第 {at} 字节处：集合里不得出现 null，空判定请用 is null")
            }
            GuardError::MixedSetTypes { at, first, found } => write!(
                f,
                "第 {at} 字节处：集合元素类型不齐，先是{first}又出现{found}"
            ),
            GuardError::NullOperand { at, what } => {
                write!(f, "第 {at} 字节处：{what}为空；请先用 is null 判定")
            }
            GuardError::TypeMismatch { at, left, right } => {
                write!(f, "第 {at} 字节处：{left}与{right}之间不作比较")
            }
            GuardError::BoolNotOrdered { at, op } => {
                write!(f, "第 {at} 字节处：布尔之间不支持 {op}，只支持 == 与 !=")
            }
            GuardError::NotABoolean { got } => {
                write!(f, "守卫表达式的结果必须是布尔，实为{got}")
            }
            GuardError::NotAWholeNumber { at, func } => {
                write!(f, "第 {at} 字节处：{func} 的天数必须是整数")
            }
            GuardError::BadDateFormat { at } => write!(
                f,
                "第 {at} 字节处：日期必须写成定宽的 YYYY-MM-DD，且必须是真实存在的日期"
            ),
            GuardError::DateOutOfRange { at } => {
                write!(f, "第 {at} 字节处：日期加减的结果超出可表示范围")
            }
            GuardError::BadArgumentType { at, func, got } => write!(
                f,
                "第 {at} 字节处：{func} 不接受这样的实参类型（{}）",
                got.join("、")
            ),
            GuardError::TextTooLong { at, func, len, max } => write!(
                f,
                "第 {at} 字节处：{func} 的文本实参 {len} 字节超过上限 {max}"
            ),
            GuardError::StepLimitExceeded { max, at } => {
                write!(f, "第 {at} 字节处：求值步数超过上限 {max}")
            }
        }
    }
}

impl std::error::Error for GuardError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn g(src: &str) -> Guard {
        Guard::parse(src).unwrap_or_else(|e| panic!("{src:?} 应能解析，实为 {e}"))
    }

    /// 引用了哪些变量要报得全、去重、有序——调用方据此只取被引用的键，
    /// 报漏一个就会在求值时变成一次「空操作数」，而那是一个查起来像业务问题的错。
    #[test]
    fn referenced_vars_are_complete_deduplicated_and_sorted() {
        let vars = g("vars.b > 1 and (vars.a in [1,2] or len(vars.b) > 0)").referenced_vars();
        assert_eq!(vars, vec!["a".to_string(), "b".to_string()]);
        assert!(g("instance.state == 'RUNNING'")
            .referenced_vars()
            .is_empty());
    }

    /// 求值时刻取不到的状态字面量要能被报出来。
    ///
    /// 计划把守卫求值放在**更新实例状态之前**的推进路径上，
    /// 能走到那里的来源态只有 `RUNNING` 与 `WAITING`；
    /// 另外六个取值写进守卫就是一条永远不成立的分支——本卷在清的正是这种东西。
    #[test]
    fn state_literals_that_can_never_hold_are_reported() {
        assert_eq!(
            g("instance.state == 'COMPLETED'").unreachable_state_literals(),
            vec!["COMPLETED".to_string()]
        );
        // 写在左边也要认出来。
        assert_eq!(
            g("'CANCELLED' == instance.state").unreachable_state_literals(),
            vec!["CANCELLED".to_string()]
        );
        // 集合成员里的也要认出来，且只报取不到的那些。
        assert_eq!(
            g("instance.state in ['RUNNING', 'COMPENSATING']").unreachable_state_literals(),
            vec!["COMPENSATING".to_string()]
        );
    }

    /// 取得到的两个不得被误报——误报会把一条正确的守卫说成写错了。
    #[test]
    fn reachable_state_literals_are_not_reported() {
        assert_eq!(GUARD_TIME_STATES.len(), 2);
        for s in ["RUNNING", "WAITING"] {
            let src = format!("instance.state == '{s}'");
            assert!(
                g(&src).unreachable_state_literals().is_empty(),
                "{s} 在求值时刻取得到，不该被报"
            );
        }
        // 与 instance.state 无关的文本字面量一概不看。
        assert!(g("vars.x == 'COMPLETED'")
            .unreachable_state_literals()
            .is_empty());
    }

    /// 三条上限互不架空：1000 步要在 4096 字节与 32 层深度之内取得到。
    /// 一个取不到的上限就是没有上限。
    #[test]
    fn the_three_limits_do_not_cancel_each_other_out() {
        // 走满 1000 步的最省字节的写法：一个 998 元素的整数集合。
        let src = format!(
            "vars.n in [{}]",
            vec!["1"; DEFAULT_MAX_STEPS as usize - 2].join(",")
        );
        assert!(
            src.len() < MAX_SOURCE_BYTES,
            "走满步数的表达式必须写得进长度上限"
        );
        assert!(Guard::parse(&src).is_ok(), "它还必须在深度上限之内");
    }
}
