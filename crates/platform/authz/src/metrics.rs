//! 授权域指标汇端口。
//!
//! 六个指标名在 `crates/platform/obs/src/metrics/registry.rs` 登记，
//! 本 crate 不依赖 ep-platform-obs：填充桥接由两个 apps 的 wiring 目录
//! 实现本端口注入，未装配时以 [`SilentMetricsSink`] 静默运行。

/// 授权域指标的填充面。标签取值与登记表逐项一致：
/// legal_entity_id、operation_type、outcome、reason。
pub trait AuthzMetricsSink: Send + Sync {
    /// `ep_authz_decision_duration_seconds`：一次判定的时长。
    fn observe_decision(&self, legal_entity_id: &str, allowed: bool, seconds: f64);
    /// `ep_authz_denied_total`：一次拒绝计数，reason 取 [`crate::types::DenyReason::as_metric_reason`]。
    fn count_denied(&self, legal_entity_id: &str, reason: &str);
    /// `ep_authz_scope_truncated_total`：部门闭包按深度截断一次。
    fn count_scope_truncated(&self, legal_entity_id: &str);
    /// `ep_reauth_challenges_total`：签发一次重新认证挑战。
    fn count_reauth_challenge(&self, legal_entity_id: &str, operation: &str);
    /// `ep_session_admission_queue_wait_seconds` 与 `ep_session_admission_rejected_total`：
    /// 准入等待时长与拒绝计数。
    fn observe_admission(&self, seconds: f64, admitted: bool, reason: &str);
}

/// 未装配真实桥接时的静默实现。不是占位替身：判定与准入的正确性
/// 不依赖指标填充，指标缺失只影响观测面。
#[derive(Default, Clone, Copy, Debug)]
pub struct SilentMetricsSink;

impl AuthzMetricsSink for SilentMetricsSink {
    fn observe_decision(&self, _legal_entity_id: &str, _allowed: bool, _seconds: f64) {}
    fn count_denied(&self, _legal_entity_id: &str, _reason: &str) {}
    fn count_scope_truncated(&self, _legal_entity_id: &str) {}
    fn count_reauth_challenge(&self, _legal_entity_id: &str, _operation: &str) {}
    fn observe_admission(&self, _seconds: f64, _admitted: bool, _reason: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silent_sink_accepts_every_call() {
        let sink = SilentMetricsSink;
        sink.observe_decision("le", true, 0.001);
        sink.count_denied("le", "object_forbidden");
        sink.count_scope_truncated("le");
        sink.count_reauth_challenge("le", "PAYMENT");
        sink.observe_admission(0.2, false, "queue_full");
    }

    #[test]
    fn sink_is_object_safe() {
        fn _assert(_x: std::sync::Arc<dyn AuthzMetricsSink>) {}
    }
}
