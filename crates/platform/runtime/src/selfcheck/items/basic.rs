//! 非 SQL 的自检项：配置解析、时钟偏差，以及三项如实登记的 Pending。

use crate::selfcheck::registry::{SelfCheckRun, Verdict};

/// `config-parsed`。判定在进程入口就已经发生，这里上报的是那次判定的结果。
///
/// 之所以不在这里重解析一次：重解析读的是同一份文件，结论必然相同，
/// 却会掩盖「进程实际用的是哪一份配置」这个真正要报的事实。
pub struct ConfigParsed {
    /// 已生效的配置层描述，例如 `内置默认 + /etc/ep/core-server.toml + 环境变量`。
    pub layers: String,
}

#[async_trait::async_trait]
impl SelfCheckRun for ConfigParsed {
    async fn run(&self) -> Verdict {
        Verdict::Pass(format!("配置解析成功且无未知键；生效层：{}", self.layers))
    }
}

/// `config-parsed` 的失败态。配置层报错时进程无法构造其余自检项，
/// 由入口用它单独出一份只有一项的报告，再以 78 退出。
pub struct ConfigInvalid {
    pub detail: String,
}

#[async_trait::async_trait]
impl SelfCheckRun for ConfigInvalid {
    async fn run(&self) -> Verdict {
        Verdict::Fail(self.detail.clone())
    }
}

/// `clock-skew-within-limit`。
pub struct ClockSkewWithinLimit {
    pub max_ms: u32,
}

/// 时间同步状态的读取结果。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SkewReading {
    /// 已同步，最大误差（微秒）。
    Synced { max_error_us: i64 },
    /// 内核报未同步。
    Unsynchronized,
    /// 本平台读不到同步状态。这是「未覆盖」，不是「没问题」。
    Unavailable(&'static str),
}

impl ClockSkewWithinLimit {
    /// Linux 上读 `adjtimex`；其余平台读不到，如实报未覆盖。
    /// 部署目标是 Linux，开发机上的这条分支只影响本地自检，不影响交付形态。
    #[cfg(target_os = "linux")]
    pub fn read() -> SkewReading {
        // SAFETY: adjtimex 只读取内核时间状态，传入 modes = 0 不做任何修改。
        let mut buf: libc::timex = unsafe { std::mem::zeroed() };
        let rc = unsafe { libc::adjtimex(&mut buf) };
        if rc < 0 {
            return SkewReading::Unavailable("adjtimex 调用失败");
        }
        if rc == libc::TIME_ERROR {
            return SkewReading::Unsynchronized;
        }
        SkewReading::Synced {
            max_error_us: buf.maxerror,
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub fn read() -> SkewReading {
        SkewReading::Unavailable("本平台不提供 adjtimex，时钟偏差未覆盖")
    }

    /// 判定与读取分开，正负两条分支才能不依赖内核状态跑起来。
    pub fn judge(max_ms: u32, reading: SkewReading) -> Verdict {
        match reading {
            SkewReading::Synced { max_error_us } => {
                let limit_us = i64::from(max_ms) * 1_000;
                if max_error_us.abs() <= limit_us {
                    Verdict::Pass(format!(
                        "最大误差 {max_error_us} 微秒，上限 {limit_us} 微秒"
                    ))
                } else {
                    Verdict::Fail(format!(
                        "最大误差 {max_error_us} 微秒超过上限 {limit_us} 微秒"
                    ))
                }
            }
            SkewReading::Unsynchronized => {
                Verdict::Fail("内核报时钟未与授时源同步（STA_UNSYNC）".into())
            }
            SkewReading::Unavailable(why) => Verdict::Pending(why.into()),
        }
    }
}

#[async_trait::async_trait]
impl SelfCheckRun for ClockSkewWithinLimit {
    async fn run(&self) -> Verdict {
        Self::judge(self.max_ms, Self::read())
    }
}

/// 判定手段尚未交付的项。Pending 是一种如实上报的结论，不是返回成功的桩：
/// 它在报告里显形，并由 `ep_selfcheck_pending_items` 指标持续可见。
pub struct PendingItem {
    /// 必须写明承接方，否则 Pending 会变成永久停靠区。
    pub owner: &'static str,
}

#[async_trait::async_trait]
impl SelfCheckRun for PendingItem {
    async fn run(&self) -> Verdict {
        Verdict::Pending(format!("本阶段不实现，承接方：{}", self.owner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skew_within_limit_passes() {
        let v = ClockSkewWithinLimit::judge(
            1_000,
            SkewReading::Synced {
                max_error_us: 999_999,
            },
        );
        assert!(matches!(v, Verdict::Pass(_)));
    }

    // 负样例断言的是判定规则本身：超限必须失败，而不是四舍五入放过。
    #[test]
    fn skew_over_limit_fails() {
        let v = ClockSkewWithinLimit::judge(
            1_000,
            SkewReading::Synced {
                max_error_us: 1_000_001,
            },
        );
        assert!(matches!(v, Verdict::Fail(_)), "超过 1 秒必须判失败");
    }

    #[test]
    fn negative_skew_uses_absolute_value() {
        assert!(matches!(
            ClockSkewWithinLimit::judge(
                1_000,
                SkewReading::Synced {
                    max_error_us: -1_000_001
                }
            ),
            Verdict::Fail(_)
        ));
    }

    #[test]
    fn unsynchronized_kernel_fails_rather_than_passes() {
        assert!(matches!(
            ClockSkewWithinLimit::judge(1_000, SkewReading::Unsynchronized),
            Verdict::Fail(_)
        ));
    }

    #[test]
    fn unavailable_reading_is_pending_not_pass() {
        let v = ClockSkewWithinLimit::judge(1_000, SkewReading::Unavailable("读不到"));
        assert!(matches!(v, Verdict::Pending(_)), "读不到被测对象绝不判通过");
    }

    #[tokio::test]
    async fn pending_item_names_its_owner() {
        let v = PendingItem { owner: "阶段 3b" }.run().await;
        match v {
            Verdict::Pending(d) => assert!(d.contains("阶段 3b")),
            other => panic!("应为 Pending，实际 {other:?}"),
        }
    }

    #[tokio::test]
    async fn config_invalid_reports_the_key_path() {
        let v = ConfigInvalid {
            detail: "未知键 db.hostt".into(),
        }
        .run()
        .await;
        assert_eq!(v, Verdict::Fail("未知键 db.hostt".into()));
    }
}
