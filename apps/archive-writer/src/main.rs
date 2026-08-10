//! archive-writer — 阶段 1 空壳进程。
//!
//! crate 名与进程名、systemd 单元名、cgroup slice 名一一对应，
//! 由 `xtask codecheck` 断言。

mod wiring;

fn main() {
    println!("archive-writer skeleton");
}
