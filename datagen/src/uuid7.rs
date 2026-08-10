//! 由 seed 决定的 UUIDv7 字面量。
//!
//! 为什么不用 `uuid` crate：本 crate 只依赖 `ep-foundation`，引入第二个直接依赖会改动
//! 仓库根 `Cargo.toml` 与 `Cargo.lock`，而生成器只需要「按 8-4-4-4-12 输出十六进制」
//! 这一件事，`std` 足够。生成的值最终要落进数据字典第 2 节的 `id uuid` 列，
//! 因此必须满足 UUIDv7 的版本位与变体位校验，不能是任意十六进制串。
//!
//! 为什么时间戳段取固定基准时刻而不取当前时钟：UUIDv7 的高 48 位按规范是毫秒时间戳，
//! 若取当前时钟，同一 seed 两次生成必然不同字节，硬判据直接失效。
//! 数据集因此整体锚在一个写死的基准时刻上，见 `BASE_INSTANT_MS`。

/// 数据集基准时刻，2026-01-01T00:00:00Z 的 Unix 毫秒。
///
/// 该常量同时是全部记录 `created_at` 与 `updated_at` 的取值，
/// 使数据集在时间维度上也只由 seed 决定。
pub const BASE_INSTANT_MS: u64 = 1_767_225_600_000;

/// 基准时刻的 RFC3339 字面量，与 `BASE_INSTANT_MS` 同源，两处不得各自取值。
pub const BASE_INSTANT_RFC3339: &str = "2026-01-01T00:00:00Z";

/// 按 `(seed, index)` 产出一个 UUIDv7 形态的字面量。
///
/// 高 48 位取 `BASE_INSTANT_MS`，其余位取自确定性取数点，
/// 版本位置 7、变体位置 0b10，与 `ep_foundation::principal::SYSTEM_PRINCIPAL_ID` 同一形态约定。
pub fn uuid_v7(seed: u64, index: u64) -> String {
    let entropy = crate::rng::draw_u128(seed, index);
    let mut bytes = [0u8; 16];

    // 高 48 位：毫秒时间戳，大端。
    let ts = BASE_INSTANT_MS & 0x0000_FFFF_FFFF_FFFF;
    for (i, slot) in bytes[..6].iter_mut().enumerate() {
        *slot = ((ts >> (40 - 8 * i)) & 0xFF) as u8;
    }
    // 其余 80 位取熵。
    for (i, slot) in bytes[6..].iter_mut().enumerate() {
        *slot = ((entropy >> (72 - 8 * i)) & 0xFF) as u8;
    }

    bytes[6] = (bytes[6] & 0x0F) | 0x70; // 版本 7
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // 变体 0b10

    format_hyphenated(&bytes)
}

fn format_hyphenated(bytes: &[u8; 16]) -> String {
    const GROUPS: [usize; 5] = [4, 2, 2, 2, 6];
    let mut out = String::with_capacity(36);
    let mut at = 0usize;
    for (g, len) in GROUPS.iter().enumerate() {
        if g > 0 {
            out.push('-');
        }
        for b in &bytes[at..at + len] {
            out.push(nibble(b >> 4));
            out.push(nibble(b & 0x0F));
        }
        at += len;
    }
    out
}

const fn nibble(v: u8) -> char {
    // 小写十六进制，与 uuid crate 的 Display 及数据字典中的字面量写法一致。
    match v {
        0..=9 => (b'0' + v) as char,
        _ => (b'a' + (v - 10)) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_hyphenated_uuid(s: &str) -> bool {
        let groups: Vec<&str> = s.split('-').collect();
        groups.len() == 5
            && [8, 4, 4, 4, 12] == groups.iter().map(|g| g.len()).collect::<Vec<_>>()[..]
            && groups
                .iter()
                .all(|g| g.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
    }

    #[test]
    fn shape_is_hyphenated_lowercase() {
        assert!(is_hyphenated_uuid(&uuid_v7(1, 0)));
    }

    /// 版本位必须是 7、变体位必须是 0b10，否则入不了 uuid 列的校验。
    #[test]
    fn version_and_variant_bits_are_set() {
        for index in 0..64u64 {
            let s = uuid_v7(index, index);
            let ver = s.as_bytes()[14] as char;
            let var = s.as_bytes()[19] as char;
            assert_eq!(ver, '7', "{s}");
            assert!(matches!(var, '8' | '9' | 'a' | 'b'), "{s}");
        }
    }

    /// 负样例：换 seed 必须换 UUID。
    ///
    /// 若时间戳段之外的位没有真正取自 seed，本测试失败——它拦的是
    /// 「整个数据集所有 seed 产出同一批 ID」这条退化路径。
    #[test]
    fn different_seed_yields_different_uuid() {
        assert_ne!(uuid_v7(1, 0), uuid_v7(2, 0));
        assert_ne!(uuid_v7(1, 0), uuid_v7(1, 1));
    }

    /// 时间戳段不随 seed 变：它锚在基准时刻上，这是字节一致的前提。
    #[test]
    fn timestamp_prefix_is_pinned_to_base_instant() {
        let a = uuid_v7(1, 0);
        let b = uuid_v7(2, 5);
        assert_eq!(&a[..13], &b[..13]);
    }

    /// 同一入参必须恒等。
    #[test]
    fn same_input_is_stable() {
        assert_eq!(uuid_v7(42, 3), uuid_v7(42, 3));
    }
}
