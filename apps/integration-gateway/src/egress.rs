//! 出网客户端骨架：白名单判定、超时、退避与熔断。
//!
//! 本阶段不发起任何真实请求，只做一次对配置白名单的自检式解析，
//! 以及把熔断的状态迁移写成可判定的纯函数——没有被测对象的重试与熔断，
//! 上线那天才第一次运行，是最贵的一种"没测过"。

use std::time::{Duration, Instant};

use ep_platform_runtime::config::{BreakerCfg, EgressTarget};

/// 白名单判定。取值必须整段相等，不做后缀匹配：
/// 后缀匹配会让 `evil-example.com` 命中 `example.com`。
/// 运行期的出网目标匹配：**精确字符串相等，没有任何规范化**。
///
/// F-82：本函数今天只被测试调用——出网路径本体尚未实现，`rehearse` 在 F-81 之前
/// 曾是它唯一的「生产」调用点，而那处是拿白名单问它自己的恒真判据，已被换掉。
/// 用 `expect` 而不是 `allow`：真出现调用点时该属性会自己报错要求移除，
/// 不会像 `allow` 那样一直挂着。
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "出网路径本体未实现；本函数是其匹配语义的唯一定义处，由测试钉住"
    )
)]
pub fn is_allowed(allowlist: &[EgressTarget], target: &str) -> bool {
    allowlist.iter().any(|t| t.as_str() == target)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// 熔断器。计数与时间分开传入，判定因此可以脱离真实时钟被测试。
pub struct Breaker {
    cfg: BreakerCfg,
    state: BreakerState,
    consecutive_failures: u16,
    opened_at: Option<Instant>,
    half_open_inflight: u8,
}

impl Breaker {
    pub fn new(cfg: BreakerCfg) -> Self {
        Self {
            cfg,
            state: BreakerState::Closed,
            consecutive_failures: 0,
            opened_at: None,
            half_open_inflight: 0,
        }
    }

    pub fn state(&self) -> BreakerState {
        self.state
    }

    /// 是否放行一次请求。`now` 由调用方给，便于验证半开窗口。
    pub fn allow(&mut self, now: Instant) -> bool {
        match self.state {
            BreakerState::Closed => true,
            BreakerState::Open => {
                let elapsed = self
                    .opened_at
                    .map(|t| now.duration_since(t))
                    .unwrap_or_default();
                if elapsed >= Duration::from_millis(u64::from(self.cfg.open_ms)) {
                    self.state = BreakerState::HalfOpen;
                    self.half_open_inflight = 0;
                    self.allow(now)
                } else {
                    false
                }
            }
            BreakerState::HalfOpen => {
                if self.half_open_inflight < self.cfg.half_open_probes {
                    self.half_open_inflight += 1;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn on_success(&mut self) {
        self.state = BreakerState::Closed;
        self.consecutive_failures = 0;
        self.opened_at = None;
        self.half_open_inflight = 0;
    }

    pub fn on_failure(&mut self, now: Instant) {
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        if self.state == BreakerState::HalfOpen
            || self.consecutive_failures >= self.cfg.failure_threshold
        {
            self.state = BreakerState::Open;
            self.opened_at = Some(now);
            self.half_open_inflight = 0;
        }
    }
}

/// 启动时对出网骨架做一次演练，不发起任何真实请求。
///
/// 白名单的唯一规范形校验已经由 `EgressTarget::parse` 在反序列化时完成；这里演练
/// 当前熔断参数能走完 Closed → Open → HalfOpen → Closed。
/// 后者能在启动时就抓出 `half_open_probes = 0` 这类「熔断后永不恢复」的取值——
/// 那种取值在真实故障发生前完全看不出来。
pub fn rehearse(allowlist: &[EgressTarget], cfg: BreakerCfg) -> Result<String, String> {
    if cfg.failure_threshold == 0 {
        return Err("egress.breaker.failure_threshold 为 0，熔断器会立刻断开".into());
    }
    if cfg.half_open_probes == 0 {
        return Err("egress.breaker.half_open_probes 为 0，熔断后永不恢复".into());
    }

    let mut b = Breaker::new(cfg);
    let t0 = Instant::now();
    if !b.allow(t0) {
        return Err("熔断器初始状态不放行".into());
    }
    for _ in 0..cfg.failure_threshold {
        b.on_failure(t0);
    }
    if b.state() != BreakerState::Open {
        return Err(format!("连续 {} 次失败后未熔断", cfg.failure_threshold));
    }
    let after = t0 + Duration::from_millis(u64::from(cfg.open_ms));
    if !b.allow(after) {
        return Err("熔断窗口结束后未进入半开".into());
    }
    b.on_success();
    if b.state() != BreakerState::Closed {
        return Err("半开期成功后未闭合".into());
    }
    Ok(format!(
        "白名单 {} 项，熔断参数演练通过（阈值 {}，窗口 {} 毫秒，半开探测 {} 个）",
        allowlist.len(),
        cfg.failure_threshold,
        cfg.open_ms,
        cfg.half_open_probes
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(s: &str) -> EgressTarget {
        EgressTarget::parse(s).expect("测试取值必须合法")
    }

    #[test]
    fn allowlist_matches_whole_entries_only() {
        let list = [target("https://esign.example.com")];
        assert!(is_allowed(&list, "https://esign.example.com"));
    }

    // 负样例断言的是白名单判定这条规则本身：相似域名不得命中。
    #[test]
    fn a_lookalike_host_is_not_allowed() {
        let list = [target("https://esign.example.com")];
        assert!(!is_allowed(&list, "https://esign.example.com.evil.test"));
        assert!(!is_allowed(&list, "https://evil-esign.example.com"));
        assert!(
            !is_allowed(&[], "https://esign.example.com"),
            "空白名单一律拒绝"
        );
    }

    #[test]
    fn breaker_opens_after_threshold_consecutive_failures() {
        let cfg = BreakerCfg {
            failure_threshold: 3,
            open_ms: 1_000,
            half_open_probes: 1,
        };
        let mut b = Breaker::new(cfg);
        let now = Instant::now();
        for _ in 0..2 {
            b.on_failure(now);
        }
        assert_eq!(b.state(), BreakerState::Closed, "未达阈值不得熔断");
        b.on_failure(now);
        assert_eq!(b.state(), BreakerState::Open);
        assert!(!b.allow(now), "熔断窗口内一律拒绝");
    }

    #[test]
    fn breaker_half_opens_after_the_window_and_limits_probes() {
        let cfg = BreakerCfg {
            failure_threshold: 1,
            open_ms: 1_000,
            half_open_probes: 1,
        };
        let mut b = Breaker::new(cfg);
        let t0 = Instant::now();
        b.on_failure(t0);
        let later = t0 + Duration::from_millis(1_000);
        assert!(b.allow(later), "窗口到点后放一个探测");
        assert_eq!(b.state(), BreakerState::HalfOpen);
        assert!(!b.allow(later), "半开只放 half_open_probes 个");
    }

    #[test]
    fn a_failed_probe_reopens_the_breaker() {
        let cfg = BreakerCfg {
            failure_threshold: 1,
            open_ms: 10,
            half_open_probes: 1,
        };
        let mut b = Breaker::new(cfg);
        let t0 = Instant::now();
        b.on_failure(t0);
        let later = t0 + Duration::from_millis(10);
        assert!(b.allow(later));
        b.on_failure(later);
        assert_eq!(b.state(), BreakerState::Open, "半开期失败必须立刻重新熔断");
    }

    #[test]
    fn rehearsal_passes_with_the_documented_defaults() {
        let msg = rehearse(
            &[target("https://esign.example.com")],
            BreakerCfg::default(),
        )
        .unwrap();
        assert!(msg.contains("演练通过"), "{msg}");
    }

    // 负样例断言的是演练这条规则本身：熔断后永不恢复的取值必须被抓出来。
    #[test]
    fn rehearsal_rejects_parameters_that_never_recover() {
        let cfg = BreakerCfg {
            failure_threshold: 5,
            open_ms: 100,
            half_open_probes: 0,
        };
        assert!(rehearse(&[], cfg).is_err());
        let cfg = BreakerCfg {
            failure_threshold: 0,
            open_ms: 100,
            half_open_probes: 1,
        };
        assert!(rehearse(&[], cfg).is_err());
    }

    #[test]
    fn success_closes_the_breaker_and_clears_the_counter() {
        let cfg = BreakerCfg {
            failure_threshold: 2,
            open_ms: 10,
            half_open_probes: 1,
        };
        let mut b = Breaker::new(cfg);
        let now = Instant::now();
        b.on_failure(now);
        b.on_success();
        b.on_failure(now);
        assert_eq!(b.state(), BreakerState::Closed, "成功一次后计数必须归零");
    }

    /// `is_allowed` 是**精确相等**匹配；newtype 保证配置只可能保存唯一规范形。
    #[test]
    fn is_allowed_is_exact_equality_with_no_normalisation() {
        let list = vec![EgressTarget::parse("https://a.example.com").expect("合法白名单项")];
        assert!(is_allowed(&list, "https://a.example.com"));
        // 以下每一条在语义上都指同一个目标，但精确相等一条都不认。
        for near in [
            "https://A.example.com",
            "https://a.example.com:443",
            "https://a.example.com/",
            "HTTPS://a.example.com",
        ] {
            assert!(!is_allowed(&list, near), "{near} 不应命中：匹配无规范化");
        }
    }

    /// 非规范形在配置边界被拒，rehearse 只负责熔断状态机。
    #[test]
    fn parser_rejects_noncanonical_targets_before_rehearsal() {
        let cfg = BreakerCfg {
            failure_threshold: 2,
            half_open_probes: 1,
            ..Default::default()
        };
        for bad in ["https://A.Example.com", "https://a.example.com:443"] {
            assert!(
                EgressTarget::parse(bad).is_err(),
                "{bad} 应在配置边界被拒绝"
            );
        }
        let good = vec![EgressTarget::parse("https://a.example.com:8443").expect("合法")];
        assert!(rehearse(&good, cfg).is_ok(), "规范形应通过");
    }
}
