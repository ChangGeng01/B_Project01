//! 关联编号生成。格式 `ERR-YYYYMMDD-NNNNNN`，日期取 Asia/Shanghai 自然日。
//!
//! 阶段 1 没有共享序列表，跨进程不撞号靠的是进程序号各占一个十万段：
//! `NNNNNN = 进程序号 × 100000 + 当日进程内序号 mod 100000`。
//! 格式不随实现变化，后续阶段换成数据库序列时调用点不动。

use std::sync::Mutex;

use crate::process::ProcessKind;

/// 每个进程一天的编号容量。
pub const PER_PROCESS_CAPACITY: u32 = 100_000;

pub struct IncidentNoGen {
    ordinal: u32,
    state: Mutex<DayState>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct DayState {
    day: String,
    seq: u32,
}

impl IncidentNoGen {
    pub fn new(process: ProcessKind) -> Self {
        Self {
            ordinal: process.ordinal(),
            state: Mutex::new(DayState {
                day: String::new(),
                seq: 0,
            }),
        }
    }

    /// 取一个新的关联编号。
    pub fn next(&self) -> String {
        self.next_on(&today_shanghai())
    }

    /// 日期由调用方给出，使跨日归零与回绕两条分支可被测试直接驱动。
    pub fn next_on(&self, day: &str) -> String {
        let mut guard = self.state.lock().unwrap_or_else(|p| p.into_inner());
        if guard.day != day {
            guard.day = day.to_string();
            guard.seq = 0;
        }
        let seq = guard.seq;
        guard.seq = guard.seq.wrapping_add(1);
        let tail = self.ordinal * PER_PROCESS_CAPACITY + seq % PER_PROCESS_CAPACITY;
        format!("ERR-{day}-{tail:06}")
    }
}

/// Asia/Shanghai 固定 +08:00，无夏令时。
pub fn today_shanghai() -> String {
    let now = chrono::Utc::now() + chrono::Duration::hours(8);
    now.format("%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_carries_the_process_segment() {
        let core = IncidentNoGen::new(ProcessKind::CoreServer);
        assert_eq!(core.next_on("20260811"), "ERR-20260811-100000");
        assert_eq!(core.next_on("20260811"), "ERR-20260811-100001");
    }

    // 负样例断言的是分段这条规则本身：同一天同一序号，八个进程不得撞号。
    #[test]
    fn eight_processes_do_not_collide_on_the_same_day() {
        let mut seen = std::collections::BTreeSet::new();
        for p in crate::process::ALL_PROCESSES {
            let gen = IncidentNoGen::new(p);
            for _ in 0..3 {
                assert!(seen.insert(gen.next_on("20260811")), "{} 撞号", p.name());
            }
        }
        assert_eq!(seen.len(), 24);
    }

    #[test]
    fn sequence_resets_across_natural_days() {
        let gen = IncidentNoGen::new(ProcessKind::JobWorker);
        assert_eq!(gen.next_on("20260811"), "ERR-20260811-200000");
        assert_eq!(gen.next_on("20260811"), "ERR-20260811-200001");
        assert_eq!(
            gen.next_on("20260812"),
            "ERR-20260812-200000",
            "跨自然日归零"
        );
    }

    #[test]
    fn sequence_wraps_within_its_own_segment() {
        let gen = IncidentNoGen::new(ProcessKind::CoreServer);
        {
            let mut g = gen.state.lock().unwrap();
            g.day = "20260811".into();
            g.seq = PER_PROCESS_CAPACITY - 1;
        }
        assert_eq!(gen.next_on("20260811"), "ERR-20260811-199999");
        assert_eq!(
            gen.next_on("20260811"),
            "ERR-20260811-100000",
            "回绕后仍在本进程段内"
        );
    }

    #[test]
    fn today_is_eight_digits() {
        let d = today_shanghai();
        assert_eq!(d.len(), 8);
        assert!(d.chars().all(|c| c.is_ascii_digit()));
    }
}
