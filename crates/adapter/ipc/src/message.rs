//! IPC 报文。形态取阶段 1 计划第 6.3 节的三个样例，字段一字不改。

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 协议版本。首版恒为 1。
pub const PROTOCOL_VERSION: u8 = 1;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IpcRequest {
    pub v: u8,
    pub kind: String,
    pub id: String,
    pub method: String,
    pub payload: Value,
}

impl IpcRequest {
    pub fn new(method: &str, payload: Value) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            kind: "request".into(),
            id: uuid::Uuid::now_v7().to_string(),
            method: method.to_string(),
            payload,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IpcErrorBody {
    pub code: String,
    pub category: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IpcResponse {
    pub v: u8,
    pub kind: String,
    pub id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcErrorBody>,
}

impl IpcResponse {
    pub fn ok(id: &str, payload: Value) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            kind: "response".into(),
            id: id.to_string(),
            ok: true,
            payload: Some(payload),
            error: None,
        }
    }

    pub fn failed(id: &str, error: IpcErrorBody) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            kind: "response".into(),
            id: id.to_string(),
            ok: false,
            payload: None,
            error: Some(error),
        }
    }
}

/// 由错误码常量表构造错误体，取值不在调用点内联。
pub fn error_body(code: ep_foundation::ErrorCode, message: impl Into<String>) -> IpcErrorBody {
    let reg = ep_foundation::error::codes::REGISTERED
        .iter()
        .find(|r| r.code == code)
        .expect("IPC 返回的错误码必须先登记在 ep-foundation 的常量表中");
    IpcErrorBody {
        code: code.0.to_string(),
        category: reg.category.as_str().to_string(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_round_trips_through_json() {
        let req = IpcRequest::new("system.ping", serde_json::json!({}));
        let text = serde_json::to_string(&req).unwrap();
        assert_eq!(serde_json::from_str::<IpcRequest>(&text).unwrap(), req);
        assert!(text.contains("\"kind\":\"request\""));
        assert!(text.contains("\"v\":1"));
    }

    #[test]
    fn success_response_has_no_error_field() {
        let text = serde_json::to_string(&IpcResponse::ok("x", serde_json::json!({"a":1}))).unwrap();
        assert!(!text.contains("error"), "成功响应不得带空的 error 字段：{text}");
    }

    #[test]
    fn failure_response_has_no_payload_field() {
        let body = error_body(ep_foundation::error::codes::PLATFORM_ROUTE_NOT_FOUND, "未知方法");
        let text = serde_json::to_string(&IpcResponse::failed("x", body)).unwrap();
        assert!(!text.contains("payload"), "失败响应不得带空的 payload 字段：{text}");
        assert!(text.contains("PERMISSION_DENIED"));
    }

    #[test]
    fn ids_are_uuid_v7_so_frames_carry_their_own_time_order() {
        let a = IpcRequest::new("system.ping", Value::Null);
        let b = IpcRequest::new("system.ping", Value::Null);
        assert_ne!(a.id, b.id);
        assert_eq!(a.id.as_bytes()[14], b'7');
    }
}
