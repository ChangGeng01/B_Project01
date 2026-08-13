//! A-08 迁移历史视图（02 计划 §5）。
//!
//! 职责类别门禁：SYSTEM（系统管理员只读）。响应元素六字段，
//! `meta` 增 `expected_version_by_binary` 与 `is_consistent`：
//! 库内每行都在二进制内嵌清单内，且库内最大版本与二进制期望版本
//! 一致，两者同时成立才判一致；任一行缺失于清单即如实判不一致。

use std::sync::Arc;

use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use ep_adapter_db_pg::DataFoundationCheck;
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::security::context::DutyClass;
use ep_platform_runtime::http::{ApiError, Envelope};
use ep_platform_runtime::migrations::{embedded_migrations, expected_version_by_binary};
use ep_platform_tenancy::capability as cap;
use serde_json::{json, Value};

use super::{not_provisioned, ok_response, to_api_error, trace_of, PlatformState};
use crate::wiring::context::extract_context;

/// 引用 capability 登记（A-20）：路由与常量成对存在，缺失即编译失败。
#[allow(dead_code)]
const CAPABILITY_BINDING: (CapabilityDomain, ActionClass) = (
    cap::MIGRATION_HISTORY_LIST_DOMAIN,
    cap::MIGRATION_HISTORY_LIST_ACTION,
);

pub async fn list_migrations(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::System])?;
        let _ = ctx; // 历史表不带法人列，上下文仅用于门禁；查询走系统通道。
        let db = state.db.clone().ok_or_else(|| not_provisioned(&state, &trace))?;
        let rows = db
            .foundation_check
            .migration_rows()
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let mut consistent = true;
        let mut max_version: Option<u64> = None;
        let items: Vec<Value> = rows
            .iter()
            .map(|r| {
                // schema 归属与执行路径都只能来自内嵌清单：历史表无 schema 列，
                // 清单外的行如实置空并判不一致，绝不臆造归属。
                let embedded = embedded_migrations()
                    .iter()
                    .find(|e| e.version == r.version && e.name == r.name);
                match embedded {
                    Some(e) => {
                        max_version = Some(max_version.map_or(e.version, |m| m.max(e.version)));
                        json!({
                            "schema_name": e.schema,
                            "version": r.version,
                            "name": r.name,
                            "applied_on": r.applied_on,
                            "checksum": r.checksum,
                            "applied_via": if e.concurrent { "CONCURRENT" } else { "TRANSACTIONAL" },
                        })
                    }
                    None => {
                        consistent = false;
                        json!({
                            "schema_name": null,
                            "version": r.version,
                            "name": r.name,
                            "applied_on": r.applied_on,
                            "checksum": r.checksum,
                            "applied_via": null,
                        })
                    }
                }
            })
            .collect();
        let expected = expected_version_by_binary();
        if max_version != expected {
            consistent = false;
        }
        let mut env = Envelope::ok(Value::Array(items), trace.clone());
        env.meta = Some(json!({
            "expected_version_by_binary": expected,
            "is_consistent": consistent,
        }));
        Ok(ok_response(axum::http::StatusCode::OK, env))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}
