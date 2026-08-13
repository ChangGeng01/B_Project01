//! 确定性夹具：把「同一入参两次产出字节一致」这条判据做成一个可复用的判定。
//!
//! 为什么单列一个夹具而不是让每个调用点各写一个 `assert_eq!`：判据的反面
//! ——「产出与入参无关」——同样要被断言，否则一个常量生成器也能让字节一致平凡成立。
//! 两个方向合在一处，调用点就不会只写一半。
//!
//! 本模块不依赖任何生成器，只依赖调用方给的闭包，因此 `ep-datagen` 与后续阶段的
//! 任何产物都能用同一套判定，不必各自造一份。

/// 一次确定性判定的结论。`Differed` 携带首个不同字节的下标，便于定位漂移点。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Determinism {
    /// 两次产出逐字节相同。
    Identical { len: usize },
    /// 两次产出不同。
    Differed {
        first_diff_at: usize,
        left_len: usize,
        right_len: usize,
    },
}

impl Determinism {
    pub fn is_identical(&self) -> bool {
        matches!(self, Determinism::Identical { .. })
    }
}

/// 用同一入参跑两次并比对字节。不做断言，只给结论，便于正反两个方向复用。
pub fn compare_twice<I, F>(input: I, produce: F) -> Determinism
where
    I: Clone,
    F: Fn(I) -> Vec<u8>,
{
    let left = produce(input.clone());
    let right = produce(input);
    classify(&left, &right)
}

fn classify(left: &[u8], right: &[u8]) -> Determinism {
    match left.iter().zip(right).position(|(a, b)| a != b) {
        None if left.len() == right.len() => Determinism::Identical { len: left.len() },
        None => Determinism::Differed {
            first_diff_at: left.len().min(right.len()),
            left_len: left.len(),
            right_len: right.len(),
        },
        Some(at) => Determinism::Differed {
            first_diff_at: at,
            left_len: left.len(),
            right_len: right.len(),
        },
    }
}

/// 判定失败时给出的说明。调用方通常直接 `panic!` 这段文字。
pub fn describe(what: &str, outcome: &Determinism) -> String {
    match outcome {
        Determinism::Identical { len } => format!("{what}：两次产出一致，共 {len} 字节"),
        Determinism::Differed { first_diff_at, left_len, right_len } => format!(
            "{what}：两次产出不一致，首个不同字节在第 {first_diff_at} 位（长度 {left_len} 与 {right_len}）"
        ),
    }
}

/// 断言 `produce` 对同一入参确定，且对不同入参不退化为常量。
///
/// 两条一起断言的理由见模块注释：只断言前者时，一个忽略入参的常量生成器同样通过。
///
/// # Panics
/// 同一入参两次产出不同，或两个不同入参产出相同字节时 panic。
pub fn assert_deterministic_and_input_sensitive<I, F>(what: &str, a: I, b: I, produce: F)
where
    I: Clone,
    F: Fn(I) -> Vec<u8>,
{
    let outcome = compare_twice(a.clone(), &produce);
    assert!(outcome.is_identical(), "{}", describe(what, &outcome));

    let left = produce(a);
    let right = produce(b);
    assert_ne!(
        left, right,
        "{what}：两个不同入参产出了同一批字节，产出与入参无关则确定性判据平凡成立"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deterministic(n: u8) -> Vec<u8> {
        vec![n; 4]
    }

    #[test]
    fn identical_output_is_recognised() {
        let outcome = compare_twice(1u8, deterministic);
        assert_eq!(outcome, Determinism::Identical { len: 4 });
        assert!(describe("样例", &outcome).contains("一致"));
    }

    /// 负样例：产出随调用次数变化时必须判为不一致，断言的是比对规则本身。
    #[test]
    fn drifting_producer_is_caught() {
        use std::cell::Cell;
        let calls = Cell::new(0u8);
        let outcome = compare_twice(0u8, |_| {
            calls.set(calls.get() + 1);
            vec![calls.get()]
        });
        assert_eq!(
            outcome,
            Determinism::Differed {
                first_diff_at: 0,
                left_len: 1,
                right_len: 1
            }
        );
        assert!(describe("样例", &outcome).contains("不一致"));
    }

    /// 负样例：长度不同也必须判为不一致，不得因前缀相同就放行。
    #[test]
    fn length_mismatch_is_caught() {
        assert_eq!(
            classify(&[1, 2], &[1, 2, 3]),
            Determinism::Differed {
                first_diff_at: 2,
                left_len: 2,
                right_len: 3
            }
        );
    }

    #[test]
    fn deterministic_and_sensitive_producer_passes() {
        assert_deterministic_and_input_sensitive("样例", 1u8, 2u8, deterministic);
    }

    /// 负样例：忽略入参的常量生成器必须被拦下，这是本夹具存在的理由。
    #[test]
    #[should_panic(expected = "产出与入参无关")]
    fn constant_producer_is_rejected() {
        assert_deterministic_and_input_sensitive("常量生成器", 1u8, 2u8, |_| vec![7, 7, 7]);
    }
}
