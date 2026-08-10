//! 每请求一条访问日志。
//!
//! 访问日志与业务日志共用同一固定字段集合，差别只在 `target` 与 `operation`
//! 的取值上；不另立一套字段，否则两套字段会各自漂移。

use super::{LogFields, Level};

/// 一次 HTTP 请求的访问日志素材。
#[derive(Clone, Debug)]
pub struct AccessLog {
    /// 模板路径，不是实例路径。与指标 `route` 标签同源，避免两处各自取值。
    pub route: String,
    pub method: String,
    pub status: u16,
    pub duration_ms: u64,
    pub trace_id: String,
    pub request_id: Option<String>,
    pub error_code: Option<String>,
    pub error_category: Option<String>,
}

impl AccessLog {
    pub fn level(&self) -> Level {
        if self.status >= 500 {
            Level::Error
        } else {
            Level::Info
        }
    }

    pub fn into_fields(self) -> LogFields {
        let outcome = if self.status < 400 { "ok" } else { "error" };
        LogFields {
            target: "http.access",
            msg: format!("{} {} {}", self.method, self.route, self.status),
            trace_id: Some(self.trace_id),
            span_id: None,
            request_id: self.request_id,
            legal_entity_id: None,
            user_id: None,
            device_id: None,
            module: Some("platform".to_string()),
            operation: Some(format!("{} {}", self.method, self.route)),
            duration_ms: Some(self.duration_ms),
            outcome: Some(outcome),
            error_code: self.error_code,
            error_category: self.error_category,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(status: u16) -> AccessLog {
        AccessLog {
            route: "/api/v1/system/health".into(),
            method: "GET".into(),
            status,
            duration_ms: 3,
            trace_id: "4bf92f3577b34da6a3ce929d0e0e4736".into(),
            request_id: None,
            error_code: None,
            error_category: None,
        }
    }

    #[test]
    fn success_is_info_and_ok() {
        let f = sample(200).into_fields();
        assert_eq!(f.outcome, Some("ok"));
    }

    #[test]
    fn server_error_is_error_level_and_error_outcome() {
        assert_eq!(sample(503).level(), Level::Error);
        assert_eq!(sample(503).into_fields().outcome, Some("error"));
    }

    #[test]
    fn client_error_is_error_outcome_but_not_error_level() {
        assert_eq!(sample(400).level(), Level::Info, "4xx 不需要人工介入，不占 ERROR");
        assert_eq!(sample(400).into_fields().outcome, Some("error"));
    }
}
