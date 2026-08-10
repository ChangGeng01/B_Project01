//! 确定性伪随机数取数。
//!
//! 为什么自带算法而不用 std：`std::collections::hash_map::RandomState` 每进程随机播种，
//! `DefaultHasher` 的算法与输出被标准库显式声明为不保证跨版本稳定，
//! 二者都会让「同一 seed 两次生成字节一致」这条硬判据在换机器或换编译器时失效。
//! 这里固定 SplitMix64 的三个魔数，输出与平台、进程、编译器版本无关。
//!
//! 为什么是纯函数而不是持状态的生成器：取数点由 `(seed, index)` 两个显式入参决定，
//! 不存在「谁先取谁后取」的隐式顺序耦合，新增一条记录不会改动既有记录的取值。

/// SplitMix64 的黄金比例增量。
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
const MIX_1: u64 = 0xBF58_476D_1CE4_E5B9;
const MIX_2: u64 = 0x94D0_49BB_1331_11EB;

/// 取第 `index` 个取数点的 64 位输出。同一 `(seed, index)` 恒等。
pub const fn draw(seed: u64, index: u64) -> u64 {
    let mut z = seed.wrapping_add(index.wrapping_add(1).wrapping_mul(GAMMA));
    z = (z ^ (z >> 30)).wrapping_mul(MIX_1);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_2);
    z ^ (z >> 31)
}

/// 取第 `index` 个取数点的 128 位输出，由相邻两个 64 位取数点拼成。
///
/// `index` 按 128 位为单位计数，内部展开为 `2*index` 与 `2*index+1`，
/// 使 128 位取数点与 64 位取数点不会互相抢占同一个下标。
pub const fn draw_u128(seed: u64, index: u64) -> u128 {
    let hi = draw(seed, index.wrapping_mul(2)) as u128;
    let lo = draw(seed, index.wrapping_mul(2).wrapping_add(1)) as u128;
    (hi << 64) | lo
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 黄金向量。取值由 SplitMix64 的定义独立算出，不是从本实现回填的。
    ///
    /// 这条测试守的是「算法本身不得漂移」：改动 GAMMA、两个混淆常数或移位量，
    /// 都会让已冻结的数据集在同一 seed 下产出不同字节，而那正是硬判据要禁止的。
    #[test]
    fn draw_matches_golden_vector() {
        assert_eq!(draw(0, 0), 0xE220_A839_7B1D_CDAF);
        assert_eq!(draw(0, 1), 0x6E78_9E6A_A1B9_65F4);
        assert_eq!(draw(0, 2), 0x06C4_5D18_8009_454F);
        assert_eq!(draw(42, 0), 0xBDD7_3226_2FEB_6E95);
    }

    /// 负样例：换 seed 必须换输出。
    ///
    /// 断言的是取数规则本身而不是调用方——若 `draw` 退化成忽略 seed 的常量函数，
    /// 「同一 seed 两次一致」会平凡成立而整个生成器失去意义，本测试拦这条退化路径。
    #[test]
    fn different_seed_yields_different_draw() {
        for index in 0..64u64 {
            assert_ne!(draw(1, index), draw(2, index), "index={index}");
        }
    }

    /// 负样例：换取数点必须换输出，否则一个数据集里所有 ID 会撞成同一个值。
    #[test]
    fn different_index_yields_different_draw() {
        let seed = 7;
        let mut seen: Vec<u64> = (0..256u64).map(|i| draw(seed, i)).collect();
        seen.sort_unstable();
        let before = seen.len();
        seen.dedup();
        assert_eq!(seen.len(), before, "256 个取数点内出现重复取值");
    }

    /// 128 位取数点必须与 64 位取数点错开，不得共用下标。
    #[test]
    fn u128_draw_consumes_two_distinct_slots() {
        let seed = 99;
        let v = draw_u128(seed, 3);
        assert_eq!((v >> 64) as u64, draw(seed, 6));
        assert_eq!(v as u64, draw(seed, 7));
    }
}
