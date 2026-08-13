//! 密钥域六端点（A-01~A-06，02 计划 §5 逐字契约）。
//!
//! 能力常量九对登记在 `ep-platform-tenancy` 的 capability 模块，
//! 本文件按 A-20 逐对引用，不另起字面量。职责类别门禁：
//! A-01~A-06 一律要求 SECURITY（安全管理员），不命中即 403。

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::Json;
use ep_adapter_db_pg::{DataKeyInsert, KeyDomainRow};
use ep_adapter_kms::{DomainKind, KeyDomain};
use ep_foundation::capability::{ActionClass, CapabilityDomain};
use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED, PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
    PLATFORM_KEY_DOMAIN_TRANSITION_INVALID, PLATFORM_REQUEST_INVALID_PAYLOAD,
};
use ep_foundation::error::AppError;
use ep_foundation::port::kms::{KeyPurpose, KeyRef};
use ep_foundation::security::context::DutyClass;
use ep_platform_runtime::http::{ApiError, Envelope};
use ep_platform_tenancy::capability as cap;
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use super::{
    events, not_provisioned, ok_response, replay_response, to_api_error, trace_of, PlatformState,
};
use crate::wiring::context::{extract_context, require_reauth_token};

/// 内置载体的 KEK 引用：激活前置只认 `kms://builtin/` 前缀。
const KEK_REF_BUILTIN: &str = "kms://builtin/master";
/// 开通时四用途数据密钥的密级范围：SECRET=40。
const PROVISION_SCOPE: u8 = 40;

/// 引用 capability 登记（A-20）：六端点的域与动作类别在此逐对可见，
/// 保证路由与常量成对存在，缺失即编译失败。
#[allow(dead_code)]
const CAPABILITY_BINDINGS: &[(CapabilityDomain, ActionClass)] = &[
    (cap::KEY_DOMAIN_LIST_DOMAIN, cap::KEY_DOMAIN_LIST_ACTION),
    (cap::KEY_DOMAIN_GET_DOMAIN, cap::KEY_DOMAIN_GET_ACTION),
    (
        cap::KEY_DOMAIN_PROVISION_DOMAIN,
        cap::KEY_DOMAIN_PROVISION_ACTION,
    ),
    (cap::KEY_DOMAIN_ROTATE_DOMAIN, cap::KEY_DOMAIN_ROTATE_ACTION),
    (
        cap::KEY_DOMAIN_PLAN_DESTROY_DOMAIN,
        cap::KEY_DOMAIN_PLAN_DESTROY_ACTION,
    ),
    (
        cap::KEY_DOMAIN_CANCEL_DESTROY_DOMAIN,
        cap::KEY_DOMAIN_CANCEL_DESTROY_ACTION,
    ),
];

fn invalid_payload(state: &PlatformState, trace: &str, field: &str, reason: &str) -> ApiError {
    ApiError::new(
        PLATFORM_REQUEST_INVALID_PAYLOAD,
        state.system.next_incident_no(),
        trace.to_string(),
    )
    .with_details(vec![ep_platform_runtime::http::Detail {
        field: field.into(),
        reason: reason.into(),
        value: None,
    }])
}

fn purpose_of(raw: &str) -> Option<KeyPurpose> {
    match raw {
        "FIELD" => Some(KeyPurpose::Field),
        "BLIND_INDEX" => Some(KeyPurpose::BlindIndex),
        "ATTACHMENT" => Some(KeyPurpose::Attachment),
        "ARCHIVE" => Some(KeyPurpose::Archive),
        _ => None,
    }
}

fn hex_of(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn domain_json(row: &KeyDomainRow) -> Value {
    json!({
        "id": row.id.to_string(),
        "domain_kind": row.domain_kind,
        "state": row.state,
        "kek_version": row.kek_version,
        "provisioned_at": row.provisioned_at.map(|t| t.to_rfc3339()),
        "active_key_count": row.active_key_count,
    })
}

// —— A-01：密钥域列表（分页与排序白名单在装配层实现，偏离登记）。——

/// A-01 查询参数：`page`、`page_size`、`sort`（created_at|domain_kind）。
#[derive(Deserialize, Default)]
pub struct ListParams {
    page: Option<u64>,
    page_size: Option<u64>,
    sort: Option<String>,
}

pub async fn list_key_domains(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Query(params): Query<ListParams>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(20);
        if page < 1 || !(1..=200).contains(&page_size) {
            return Err(invalid_payload(
                &state,
                &trace,
                "page/page_size",
                "分页参数越界",
            ));
        }
        let sort = params.sort.as_deref().unwrap_or("created_at");
        if !matches!(sort, "created_at" | "domain_kind") {
            return Err(invalid_payload(
                &state,
                &trace,
                "sort",
                "排序列不在白名单内",
            ));
        }
        let mut rows = db
            .key_domains
            .list_for_entity(&ctx)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        if sort == "domain_kind" {
            rows.sort_by(|a, b| a.domain_kind.cmp(&b.domain_kind));
        }
        let total = rows.len() as u64;
        let start = ((page - 1) * page_size) as usize;
        let items: Vec<Value> = rows
            .iter()
            .skip(start)
            .take(page_size as usize)
            .map(domain_json)
            .collect();
        let mut env = Envelope::ok(Value::Array(items), trace.clone());
        env.meta = Some(json!({"page": page, "page_size": page_size, "total": total}));
        Ok(ok_response(axum::http::StatusCode::OK, env))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// —— A-02：单个密钥域与其 DEK 版本摘要（任何响应不含 wrapped_key）。——

pub async fn get_key_domain(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let got = db
            .key_domains
            .get(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let (domain, keys) = got.ok_or_else(|| {
            // RLS 不可见与不存在同形：一律 404 NOT_FOUND_OR_DENIED。
            ApiError::new(
                PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                state.system.next_incident_no(),
                trace.clone(),
            )
        })?;
        let mut data = domain_json(&domain);
        data["keys"] = Value::Array(
            keys.iter()
                .map(|k| {
                    json!({
                        "purpose": k.purpose,
                        "security_level_scope": k.security_level_scope,
                        "version": k.version,
                        "state": k.state,
                        "activated_at": k.activated_at.to_rfc3339(),
                    })
                })
                .collect(),
        );
        Ok(ok_response(
            axum::http::StatusCode::OK,
            Envelope::ok(data, trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// —— A-03：开通密钥域（业务幂等：ACTIVE 重放；KMS 不可用即 503）。——

#[derive(Deserialize)]
pub struct ProvisionBody {
    domain_kind: String,
}

pub async fn provision_key_domain(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Json(body): Json<ProvisionBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let kms = state
            .kms
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        if body.domain_kind != "LEGAL_ENTITY" {
            return Err(invalid_payload(
                &state,
                &trace,
                "domain_kind",
                "首版只支持 LEGAL_ENTITY",
            ));
        }
        // 幂等重放：同法人同 kind 已有 ACTIVE 域即原样返回并置重放头。
        if let Some(existing) = db
            .key_domains
            .domain_of_kind(&ctx, "LEGAL_ENTITY")
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?
        {
            if existing.state == "ACTIVE" {
                return Ok(replay_response(Envelope::ok(
                    domain_json(&existing),
                    trace.clone(),
                )));
            }
        }
        let le_id = ctx.legal_entity_id;
        let domain_id = Uuid::now_v7();
        // KMS 侧六态迁移的前两段：登记 PROVISIONING 并生成四用途 DEK。
        // 任一步失败即装配不可用语义，按 503 NOT_PROVISIONED 上抛。
        let kms_fail = |state: &PlatformState, trace: &str| not_provisioned(state, trace);
        kms.register_domain(KeyDomain::new_provisioning(
            domain_id,
            le_id,
            DomainKind::LegalEntity,
            KeyRef::new(KEK_REF_BUILTIN),
        ))
        .map_err(|_| kms_fail(&state, &trace))?;
        let mut inserts = Vec::new();
        for purpose in [
            KeyPurpose::Field,
            KeyPurpose::BlindIndex,
            KeyPurpose::Attachment,
            KeyPurpose::Archive,
        ] {
            let key_id = kms
                .generate_data_key(domain_id, purpose, PROVISION_SCOPE)
                .map_err(|_| kms_fail(&state, &trace))?;
            let wrapped = kms
                .wrapped_key_of(key_id)
                .ok_or_else(|| kms_fail(&state, &trace))?;
            inserts.push(DataKeyInsert {
                id: key_id,
                key_domain_id: domain_id,
                purpose: purpose.as_str(),
                security_level_scope: PROVISION_SCOPE,
                version: 1,
                algorithm: ep_adapter_kms::DekAlgorithm::for_purpose(purpose).as_str(),
                wrapped_key_hex: hex_of(&wrapped),
                wrap_kek_version: 1,
            });
        }
        db.key_domains
            .insert_provisioning(
                &ctx,
                domain_id,
                le_id.as_uuid(),
                "LEGAL_ENTITY",
                KEK_REF_BUILTIN,
                inserts,
            )
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        // 两侧激活：KMS 先核验四用途 version=1，再落库激活。
        kms.activate_domain(domain_id)
            .map_err(|_| kms_fail(&state, &trace))?;
        let inserted = db
            .key_domains
            .get(&ctx, domain_id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?
            .ok_or_else(|| kms_fail(&state, &trace))?;
        db.key_domains
            .activate_domain(&ctx, domain_id, inserted.0.row_version)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let final_row = db
            .key_domains
            .get(&ctx, domain_id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?
            .ok_or_else(|| kms_fail(&state, &trace))?;
        events::record_pending_emit(
            &state,
            events::KEY_DOMAIN_PROVISIONED,
            &domain_id.to_string(),
        );
        Ok(ok_response(
            axum::http::StatusCode::CREATED,
            Envelope::ok(domain_json(&final_row.0), trace.clone()),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// —— A-04：轮换指定 purpose 的 DEK（需重新认证；并发 409）。——

#[derive(Deserialize)]
pub struct RotateBody {
    purpose: String,
}

pub async fn rotate_key_domain(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(body): Json<RotateBody>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
        require_reauth_token(&headers, &state.system)?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let kms = state
            .kms
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let purpose = purpose_of(&body.purpose)
            .ok_or_else(|| invalid_payload(&state, &trace, "purpose", "用途取值不在四枚举内"))?;
        let got = db
            .key_domains
            .get(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let (domain, keys) = got.ok_or_else(|| {
            ApiError::new(
                PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                state.system.next_incident_no(),
                trace.clone(),
            )
        })?;
        if domain.state != "ACTIVE" {
            return Err(to_api_error(
                AppError::new(
                    PLATFORM_KEY_DOMAIN_TRANSITION_INVALID,
                    "只有 ACTIVE 域可轮换",
                ),
                &state,
                &trace,
            ));
        }
        // 库内该 purpose 的在役行：提供乐观锁谓词的版本基准。
        let active = keys
            .iter()
            .filter(|k| k.purpose == body.purpose && k.state == "ACTIVE")
            .max_by_key(|k| k.version)
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let purpose_str = purpose.as_str();
        let (new_id, new_version, wrapped) = if kms.domain_snapshot(id).is_some() {
            // 同进程内注册表与库连续：走 KMS 轮换，进程内互斥在途即 409。
            let report = kms
                .rotate(id, purpose)
                .map_err(|e| to_api_error(e, &state, &trace))?;
            let wrapped = kms
                .wrapped_key_of(report.new_data_key_id)
                .ok_or_else(|| not_provisioned(&state, &trace))?;
            (report.new_data_key_id, report.new_version, wrapped)
        } else {
            // 重启后注册表断档：以库为基准推导新版本号（偏离登记）。
            let next = keys
                .iter()
                .filter(|k| k.purpose == body.purpose)
                .map(|k| k.version)
                .max()
                .unwrap_or(0)
                + 1;
            let (key_id, wrapped) = kms.generate_detached_data_key();
            (key_id, next as u16, wrapped)
        };
        let insert = DataKeyInsert {
            id: new_id,
            key_domain_id: id,
            purpose: purpose_str,
            security_level_scope: active.security_level_scope as u8,
            version: new_version,
            algorithm: ep_adapter_kms::DekAlgorithm::for_purpose(purpose).as_str(),
            wrapped_key_hex: hex_of(&wrapped),
            wrap_kek_version: domain.kek_version as u32,
        };
        let rows = db
            .key_domains
            .rotate(
                &ctx,
                id,
                ctx.legal_entity_id.as_uuid(),
                purpose_str,
                insert,
                active.row_version,
            )
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        events::record_pending_emit(&state, events::KEY_DOMAIN_ROTATED, &id.to_string());
        Ok(ok_response(
            axum::http::StatusCode::OK,
            Envelope::ok(
                json!({
                    "id": id.to_string(),
                    "purpose": purpose_str,
                    "new_version": rows.new_version,
                    "new_data_key_id": rows.new_data_key_id.to_string(),
                    "retiring_data_key_id": rows.retiring_data_key_id.to_string(),
                    "retiring_state": "RETIRING",
                }),
                trace.clone(),
            ),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}

// —— A-05：销毁前排程。审批端口属阶段 4，未装配一律 403（规格明写）。——

pub async fn plan_destroy_key_domain(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(_id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
    require_reauth_token(&headers, &state.system)?;
    let _ = ctx;
    // 双人审批判定经端口调用阶段 4；端口未装配时集成层不得出具凭据，
    // 按 02 计划 A-05 一律 403 OBJECT_FORBIDDEN。
    Err(ApiError::new(
        PLATFORM_AUTHZ_OBJECT_FORBIDDEN,
        state.system.next_incident_no(),
        trace_of(&headers),
    ))
}

// —— A-06：撤销销毁计划（仅 DESTROY_PLANNED 可调，否则 409）。——

pub async fn cancel_destroy_key_domain(
    State(state): State<Arc<PlatformState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Response {
    let trace = trace_of(&headers);
    let out: Result<Response, ApiError> = async {
        let ctx = extract_context(&headers, &state.system, &[DutyClass::Security])?;
        require_reauth_token(&headers, &state.system)?;
        let db = state
            .db
            .clone()
            .ok_or_else(|| not_provisioned(&state, &trace))?;
        let got = db
            .key_domains
            .get(&ctx, id)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        let (domain, _) = got.ok_or_else(|| {
            ApiError::new(
                PLATFORM_AUTHZ_NOT_FOUND_OR_DENIED,
                state.system.next_incident_no(),
                trace.clone(),
            )
        })?;
        if domain.state != "DESTROY_PLANNED" {
            return Err(to_api_error(
                AppError::new(
                    PLATFORM_KEY_DOMAIN_TRANSITION_INVALID,
                    "只有 DESTROY_PLANNED 域可撤销销毁计划",
                ),
                &state,
                &trace,
            ));
        }
        db.key_domains
            .set_domain_state(&ctx, id, "DESTROY_PLANNED", "ACTIVE", domain.row_version)
            .await
            .map_err(|e| to_api_error(e, &state, &trace))?;
        // KMS 内存注册表若仍持有该域，同步回退；重启断档时以库为准。
        if let Some(kms) = state.kms.clone() {
            if kms.domain_snapshot(id).is_some() {
                kms.restore_from_destroy_plan(
                    id,
                    &format!("audit://core-server/cancel-destroy/{id}"),
                )
                .map_err(|e| to_api_error(e, &state, &trace))?;
            }
        }
        Ok(ok_response(
            axum::http::StatusCode::OK,
            Envelope::ok(
                json!({"id": id.to_string(), "state": "ACTIVE"}),
                trace.clone(),
            ),
        ))
    }
    .await;
    match out {
        Ok(r) => r,
        Err(e) => e.into_response(),
    }
}
