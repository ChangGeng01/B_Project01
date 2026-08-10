//! 追踪上下文。
//!
//! 只做两件事：在进程内新建一条 trace，以及为门户请求生成 `X-Correlation-Id`。
//! 不接受外部传入的 traceparent 这条纪律落在 portal-gateway 的装配处，不在这里，
//! 因为这里只是取值类型，读不到请求头。

use std::sync::atomic::{AtomicU64, Ordering};

/// W3C trace-context 的 trace-id，32 位小写十六进制。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TraceContext {
    trace_id: String,
    span_id: String,
}

/// 门户侧公网关联标识。与内部 trace_id 分离，公网侧看不到内部链路标识。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CorrelationId(String);

static SPAN_SEQ: AtomicU64 = AtomicU64::new(1);

impl TraceContext {
    /// 新建一条 trace。熵取自 UUIDv4，与 `Id<T>` 的 UUIDv7 分开：
    /// trace-id 不需要时间有序，混用会把生成时刻泄漏到公网侧的关联标识里。
    pub fn new() -> Self {
        let trace = uuid::Uuid::new_v4().as_u128();
        Self { trace_id: format!("{trace:032x}"), span_id: next_span_id() }
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn span_id(&self) -> &str {
        &self.span_id
    }

    /// 同一 trace 下的下一个 span。
    pub fn child(&self) -> Self {
        Self { trace_id: self.trace_id.clone(), span_id: next_span_id() }
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

fn next_span_id() -> String {
    let n = SPAN_SEQ.fetch_add(1, Ordering::Relaxed);
    let salt = uuid::Uuid::new_v4().as_u128() as u64;
    format!("{:016x}", n ^ salt)
}

impl CorrelationId {
    pub fn new() -> Self {
        Self(format!("{:032x}", uuid::Uuid::new_v4().as_u128()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_id_is_32_lowercase_hex() {
        let t = TraceContext::new();
        assert_eq!(t.trace_id().len(), 32);
        assert!(t.trace_id().chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
        assert_eq!(t.span_id().len(), 16);
    }

    #[test]
    fn child_keeps_trace_and_changes_span() {
        let parent = TraceContext::new();
        let child = parent.child();
        assert_eq!(parent.trace_id(), child.trace_id());
        assert_ne!(parent.span_id(), child.span_id());
    }

    #[test]
    fn two_traces_do_not_collide() {
        assert_ne!(TraceContext::new().trace_id(), TraceContext::new().trace_id());
    }

    #[test]
    fn correlation_id_is_not_the_trace_id() {
        // 负样例断言的是「公网关联标识不得复用内部 trace_id」这条规则本身。
        let t = TraceContext::new();
        let c = CorrelationId::new();
        assert_ne!(c.as_str(), t.trace_id());
    }
}
