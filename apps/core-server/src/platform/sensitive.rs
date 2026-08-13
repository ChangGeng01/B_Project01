//! A-07 敏感字段清单只读查询（02 计划 §5）。
//!
//! 职责类别门禁：SECURITY 或 AUDIT 任一命中即可（安全管理员或审计
//! 管理员只读）。三个过滤条件经查询参数 `filter[...]` 逐位可选。
//! 响应不含任何样例值——登记表中自始不存在样例值列。

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use ep_adapter_db_pg::SensitiveFieldFilter;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::security::context::DutyClass;
use ep_platform_runtime::http::{ApiError, Envelope};
use ep_platform_tenancy::capability as cap;
use serde_json::{json, Value};

use super::{not_provisioned, ok_response, to_api_error, trace_of, PlatformState};
use crate::wiring::context::extract_context;

/// 引用 capability 登记（A-20）：路由与常量成对存在，缺失即编译失败。
#[allow(dead_code)]
const CAPABILITY_BINDING: (CapabilityDomain, ActionClass) = (
    cap::SENSITIVE_FIELD_LIST_DOMAIN,
    cap::SENSITIVE_FIELD_LIST_ACTION,
);

pub async fn list_sensitive_fields(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Query(params): Query<Vec<(String, String)>>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(
            &headers,
            &state.system,
            &[DutyClass::Security, DutyClass::Audit],
        )?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let mut filter = SensitiveFieldFilter::default();
        for (k, v) in &params {
            match k.as_str() {
                "filter[schema_name]" => filter.schema_name = Some(v.clone()),
                "filter[table_name]" => filter.table_name = Some(v.clone()),
                "filter[category]" => filter.category = Some(v.clone()),
                // 未知参数忽略：等值收窄只认这三个键。
                _ => {}
            }
        }
        let rows = db
            .sensitive_fields
            .list(&ctx, filter)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let items: Vec<Value> = rows
            .iter()
            .map(|r| {
                json!({
                    "schema_name": r.schema_name,
                    "table_name": r.table_name,
                    "column_name": r.column_name,
                    "category": r.category,
                    "is_field_encrypted": r.is_field_encrypted,
                })
            })
            .collect();
        Ok(ok_response(
            axum::http::StatusCode::OK,
            Envelope::ok(Value::Array(items), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}
