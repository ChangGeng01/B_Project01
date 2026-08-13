//! 字段级投影器（判定流水线阶段四）。
//!
//! `(object_type, row, ctx) -> Value`：不改输入，只产新值。四态
//! HIDDEN / MASKED(FULL|KEEP_LAST_4|KEEP_DOMAIN) / READ / WRITE；
//! 显式 HIDDEN 的字段不入键集合；无授权行的非敏感字段原样透传，
//! 敏感列的读入口径由查询面选列承担（04 计划 §4.1）。
//! `is_field_encrypted` 为真时：KEEP_LAST_4 取 `<col>_tail` 列、
//! FULL 与 HIDDEN 不解密、仅 READ/WRITE 且密级达标经
//! [`SensitiveFieldDecryptor`]（全库唯一解密位点）解密。
//! MASKED 与 HIDDEN 形态禁排序禁聚合（PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN）。

use std::collections::BTreeMap;
use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN;
use ep_foundation::error::AppError;
use ep_foundation::port::kms::CipherEnvelope;
use ep_foundation::port::sensitive::{FieldDecryptRequest, SensitiveFieldDecryptor};
use ep_foundation::security::{SecurityContext, SecurityLevel};
use serde_json::Value;

use crate::snapshot::FieldGrantEntry;
use crate::types::{hex_decode, FieldVisibility, MaskStyle};

/// 字段敏感元数据：是否行内加密与字段密级。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SensitiveFieldInfo {
    /// 对应列是否为信封加密密文列。
    pub is_encrypted: bool,
    /// 字段密级；解密仅当主体密级不低于该值。
    pub security_level: SecurityLevel,
}

/// 字段敏感元数据的查询面。非敏感字段返回 None；SQL 载体归
/// ep-adapter-db-pg，本 crate 只消费。
pub trait SensitiveFieldLookup: Send + Sync {
    fn lookup(&self, object_type: &str, field: &str) -> Option<SensitiveFieldInfo>;
}

/// 无敏感字段的查询面，供无加密列场景装配。
pub struct NoSensitiveFields;

impl SensitiveFieldLookup for NoSensitiveFields {
    fn lookup(&self, _object_type: &str, _field: &str) -> Option<SensitiveFieldInfo> {
        None
    }
}

/// 字段级投影器。
pub struct FieldProjector {
    sensitive: Arc<dyn SensitiveFieldLookup>,
    decryptor: Arc<dyn SensitiveFieldDecryptor>,
}

/// 单字段渲染的目标上下文：字段定位、原值、整行与行标识。
struct RenderTarget<'a> {
    object_type: &'a str,
    field: &'a str,
    value: &'a Value,
    row: &'a serde_json::Map<String, Value>,
    row_id: uuid::Uuid,
}

impl FieldProjector {
    pub fn new(
        sensitive: Arc<dyn SensitiveFieldLookup>,
        decryptor: Arc<dyn SensitiveFieldDecryptor>,
    ) -> Self {
        Self {
            sensitive,
            decryptor,
        }
    }

    /// 投影：按有效可见性逐字段产出新对象，输入 row 不被修改。
    /// 行标识用于解密请求的 AAD 定位；无 `id` 键时取 nil UUID。
    pub async fn project(
        &self,
        object_type: &str,
        row: &serde_json::Map<String, Value>,
        ctx: &SecurityContext,
        grants: &[FieldGrantEntry],
    ) -> Result<Value, AppError> {
        let row_id = row
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .unwrap_or(uuid::Uuid::nil());
        let mut out = serde_json::Map::new();
        for (field, value) in row {
            let vis = effective_visibility(ctx, grants, object_type, field);
            let Some(vis) = vis else {
                // 无授权行：非敏感字段原样透传；加密列不得透传密文。
                if self.sensitive.lookup(object_type, field).is_some() {
                    continue;
                }
                out.insert(field.clone(), value.clone());
                continue;
            };
            if vis == FieldVisibility::Hidden {
                continue;
            }
            let projected = self
                .render_field(
                    &RenderTarget {
                        object_type,
                        field,
                        value,
                        row,
                        row_id,
                    },
                    ctx,
                    vis,
                )
                .await?;
            out.insert(field.clone(), projected);
        }
        Ok(Value::Object(out))
    }

    /// 排序准入：MASKED 与 HIDDEN 形态禁排序禁聚合。
    pub fn check_sortable(
        &self,
        ctx: &SecurityContext,
        grants: &[FieldGrantEntry],
        object_type: &str,
        field: &str,
    ) -> Result<(), AppError> {
        match effective_visibility(ctx, grants, object_type, field) {
            Some(vis) if vis.forbids_sorting() => Err(AppError::new(
                PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN,
                format!("字段 {object_type}.{field} 处于掩码或隐藏形态，禁止排序与聚合"),
            )),
            Some(_) => Ok(()),
            // 无授权行即不可见，排序同样无从谈起。
            None => Err(AppError::new(
                PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN,
                format!("字段 {object_type}.{field} 无授权行，禁止排序与聚合"),
            )),
        }
    }

    /// 单字段渲染：掩码不解密；READ/WRITE 遇加密列先过密级关再解密。
    async fn render_field(
        &self,
        target: &RenderTarget<'_>,
        ctx: &SecurityContext,
        vis: FieldVisibility,
    ) -> Result<Value, AppError> {
        let object_type = target.object_type;
        let field = target.field;
        let value = target.value;
        let row_id = target.row_id;
        let info = self.sensitive.lookup(object_type, field);
        let encrypted = info.map(|i| i.is_encrypted).unwrap_or(false);
        if let FieldVisibility::Masked(style) = vis {
            return Ok(mask_value(value, target.row, field, style, encrypted));
        }
        if !encrypted {
            return Ok(value.clone());
        }
        if let Some(info) = info {
            if ctx.clearance_level < info.security_level {
                return Ok(Value::Null);
            }
        }
        let envelope_hex = value.as_str().unwrap_or_default();
        let Some(bytes) = hex_decode(envelope_hex) else {
            return Ok(Value::Null);
        };
        let plain = self
            .decryptor
            .decrypt_field(FieldDecryptRequest {
                legal_entity_id: ctx.legal_entity_id,
                object_type: Arc::from(object_type.to_owned()),
                field_name: Arc::from(field.to_owned()),
                row_id,
                envelope: CipherEnvelope::new(bytes),
            })
            .await?;
        Ok(String::from_utf8_lossy(&plain).into_owned().into())
    }
}

/// 同一字段多角色授权取最宽松者：rank 最高者胜；无任何授权行返回 None。
fn effective_visibility(
    ctx: &SecurityContext,
    grants: &[FieldGrantEntry],
    object_type: &str,
    field: &str,
) -> Option<FieldVisibility> {
    let mut best: Option<FieldVisibility> = None;
    for g in grants {
        if g.object_type.as_ref() != object_type || g.field_name.as_ref() != field {
            continue;
        }
        if !ctx.roles.iter().any(|r| r.as_str() == g.role_code.as_ref()) {
            continue;
        }
        best = Some(match best {
            Some(b) if b.rank() >= g.visibility.rank() => b,
            _ => g.visibility,
        });
    }
    best
}

/// 掩码渲染。加密列一律不解密：KEEP_LAST_4 取 `<col>_tail` 辅助列，
/// 其余样式与 HIDDEN 语义同归全掩。非加密明文按样式裁剪。
fn mask_value(
    value: &Value,
    row: &serde_json::Map<String, Value>,
    field: &str,
    style: MaskStyle,
    encrypted: bool,
) -> Value {
    if encrypted {
        if style == MaskStyle::KeepLast4 {
            let tail_col = format!("{field}_tail");
            if let Some(tail) = row.get(&tail_col).and_then(|v| v.as_str()) {
                return Value::from(format!("************{tail}"));
            }
        }
        return Value::from(FULL_MASK);
    }
    let Some(text) = value.as_str() else {
        return Value::Null;
    };
    Value::from(match style {
        MaskStyle::Full => FULL_MASK.to_string(),
        MaskStyle::KeepLast4 => keep_last4(text),
        MaskStyle::KeepDomain => keep_domain(text),
    })
}

const FULL_MASK: &str = "************";
/// KEEP_LAST_4 明文长度下限：不足 8 位时保留部分占比过高，退化全掩。
const KEEP_LAST4_MIN_LEN: usize = 8;

fn keep_last4(text: &str) -> String {
    if text.chars().count() < KEEP_LAST4_MIN_LEN {
        return FULL_MASK.to_string();
    }
    let tail: String = text
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("************{tail}")
}

fn keep_domain(text: &str) -> String {
    match text.split_once('@') {
        Some((_, domain)) if !domain.is_empty() => format!("***@{domain}"),
        _ => FULL_MASK.to_string(),
    }
}

/// 字段可见性矩阵（只读投影）：字段 → 有效可见性，便于端点侧说明。
pub fn visibility_matrix(
    ctx: &SecurityContext,
    grants: &[FieldGrantEntry],
    object_type: &str,
    fields: &[&str],
) -> BTreeMap<String, Option<FieldVisibility>> {
    fields
        .iter()
        .map(|f| {
            (
                (*f).to_owned(),
                effective_visibility(ctx, grants, object_type, f),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::tests::ctx_with;
    use ep_foundation::security::context::ClientKind;
    use std::sync::Mutex;

    /// 回声解密载体：把信封字节按 UTF-8 输出，便于断言解密路径被走到。
    struct EchoDecryptor {
        calls: Mutex<Vec<String>>,
    }

    #[async_trait::async_trait]
    impl SensitiveFieldDecryptor for EchoDecryptor {
        async fn decrypt_field(&self, request: FieldDecryptRequest) -> Result<Vec<u8>, AppError> {
            self.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(request.field_name.to_string());
            Ok(request.envelope.as_bytes().to_vec())
        }
    }

    /// 固定元数据查询面：仅 `finance.cash_accounts.bank_no` 加密且机密级。
    struct FixtureLookup;

    impl SensitiveFieldLookup for FixtureLookup {
        fn lookup(&self, object_type: &str, field: &str) -> Option<SensitiveFieldInfo> {
            if object_type == "finance.cash_accounts" && field == "bank_no" {
                Some(SensitiveFieldInfo {
                    is_encrypted: true,
                    security_level: SecurityLevel::Confidential,
                })
            } else {
                None
            }
        }
    }

    fn projector(decryptor: Arc<EchoDecryptor>) -> FieldProjector {
        FieldProjector::new(Arc::new(FixtureLookup), decryptor)
    }

    fn grant(role: &str, field: &str, visibility: FieldVisibility) -> FieldGrantEntry {
        FieldGrantEntry {
            role_code: Arc::from(role),
            object_type: Arc::from("finance.cash_accounts"),
            field_name: Arc::from(field),
            visibility,
        }
    }

    fn row_with(bank_no: Value, tail: Option<&str>) -> serde_json::Map<String, Value> {
        let mut row = serde_json::Map::new();
        row.insert(
            "id".into(),
            Value::from("00000000-0000-7000-8000-00000000000a"),
        );
        row.insert("name".into(), Value::from("基本户"));
        row.insert("bank_no".into(), bank_no);
        if let Some(t) = tail {
            row.insert("bank_no_tail".into(), Value::from(t));
        }
        row
    }

    #[tokio::test]
    async fn read_write_encrypted_decrypts_via_the_single_point() {
        let dec = Arc::new(EchoDecryptor {
            calls: Mutex::new(Vec::new()),
        });
        let p = projector(dec.clone());
        // 明文值以十六进制信封表示：ascii "6225" 的 hex。
        let envelope = crate::types::hex_encode(b"6225");
        let row = row_with(Value::from(envelope), None);
        let grants = vec![grant("FINANCE", "bank_no", FieldVisibility::Read)];
        let mut ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        // Internal(20) < Confidential(30)：不解密，输出 null。
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["bank_no"], Value::Null, "密级不足不解密");
        assert!(dec
            .calls
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .is_empty());
        // 提密级至 Confidential 后经解密位点输出明文。
        ctx = with_clearance(ctx, SecurityLevel::Confidential);
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["bank_no"], Value::from("6225"));
        assert_eq!(out["name"], Value::from("基本户"), "非授权字段不受影响");
        let calls = dec.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(calls.len(), 1, "全库唯一解密位点被调用一次");
    }

    #[tokio::test]
    async fn masked_styles_and_encrypted_tail_column() {
        let dec = Arc::new(EchoDecryptor {
            calls: Mutex::new(Vec::new()),
        });
        let p = projector(dec.clone());
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        // 加密列 KEEP_LAST_4：取 bank_no_tail，不解密。
        let row = row_with(Value::from(crate::types::hex_encode(b"x")), Some("8899"));
        let grants = vec![grant(
            "FINANCE",
            "bank_no",
            FieldVisibility::Masked(MaskStyle::KeepLast4),
        )];
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["bank_no"], Value::from("************8899"));
        // 加密列 FULL：恒全掩。
        let grants = vec![grant(
            "FINANCE",
            "bank_no",
            FieldVisibility::Masked(MaskStyle::Full),
        )];
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["bank_no"], Value::from(FULL_MASK));
        // 明文 KEEP_DOMAIN 与短串 KEEP_LAST_4 退化。
        let mut plain = serde_json::Map::new();
        plain.insert("mail".into(), Value::from("alice@example.com"));
        plain.insert("phone".into(), Value::from("138"));
        let grants = vec![
            grant(
                "FINANCE",
                "mail",
                FieldVisibility::Masked(MaskStyle::KeepDomain),
            ),
            grant(
                "FINANCE",
                "phone",
                FieldVisibility::Masked(MaskStyle::KeepLast4),
            ),
        ];
        let out = p
            .project("finance.cash_accounts", &plain, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["mail"], Value::from("***@example.com"));
        assert_eq!(out["phone"], Value::from(FULL_MASK), "长度不足 8 退化全掩");
        assert!(
            dec.calls
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .is_empty(),
            "掩码不解密"
        );
    }

    #[tokio::test]
    async fn hidden_and_ungranted_fields_leave_no_key() {
        let dec = Arc::new(EchoDecryptor {
            calls: Mutex::new(Vec::new()),
        });
        let p = projector(dec);
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        let row = row_with(Value::from("secret"), None);
        let grants = vec![grant("FINANCE", "bank_no", FieldVisibility::Hidden)];
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert!(out.get("bank_no").is_none(), "HIDDEN 不入键");
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &[])
            .await
            .expect("可投影");
        assert!(out.get("bank_no").is_none(), "无授权行不入键");
        assert_eq!(out["name"], Value::from("基本户"));
    }

    #[tokio::test]
    async fn multiple_roles_take_the_most_permissive() {
        let dec = Arc::new(EchoDecryptor {
            calls: Mutex::new(Vec::new()),
        });
        let p = projector(dec.clone());
        let ctx = ctx_with(vec!["FINANCE", "AUDIT"], ClientKind::Win);
        let row = row_with(Value::from(crate::types::hex_encode(b"6225")), None);
        let grants = vec![
            grant("AUDIT", "bank_no", FieldVisibility::Masked(MaskStyle::Full)),
            grant(
                "FINANCE",
                "bank_no",
                FieldVisibility::Masked(MaskStyle::KeepLast4),
            ),
        ];
        let out = p
            .project("finance.cash_accounts", &row, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(
            out["bank_no"],
            Value::from(FULL_MASK),
            "同为掩码取 rank 相同者不越权"
        );
        // 再加 READ 角色即取最宽松。
        let ctx = ctx_with(vec!["FINANCE", "AUDIT", "TELLER"], ClientKind::Win);
        let mut grants = grants;
        grants.push(grant("TELLER", "bank_no", FieldVisibility::Read));
        let mut row2 = row.clone();
        row2.insert(
            "bank_no".into(),
            Value::from(crate::types::hex_encode(b"9527")),
        );
        let ctx = with_clearance(ctx, SecurityLevel::Confidential);
        let out = p
            .project("finance.cash_accounts", &row2, &ctx, &grants)
            .await
            .expect("可投影");
        assert_eq!(out["bank_no"], Value::from("9527"));
    }

    #[test]
    fn sortable_is_forbidden_for_masked_and_hidden() {
        let dec = Arc::new(EchoDecryptor {
            calls: Mutex::new(Vec::new()),
        });
        let p = projector(dec);
        let ctx = ctx_with(vec!["FINANCE"], ClientKind::Win);
        let grants = vec![grant(
            "FINANCE",
            "bank_no",
            FieldVisibility::Masked(MaskStyle::Full),
        )];
        let err = p
            .check_sortable(&ctx, &grants, "finance.cash_accounts", "bank_no")
            .expect_err("掩码禁排序");
        assert_eq!(err.code, PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN);
        let grants = vec![grant("FINANCE", "bank_no", FieldVisibility::Read)];
        assert!(p
            .check_sortable(&ctx, &grants, "finance.cash_accounts", "bank_no")
            .is_ok());
        let err = p
            .check_sortable(&ctx, &[], "finance.cash_accounts", "bank_no")
            .expect_err("无授权禁排序");
        assert_eq!(err.code, PLATFORM_AUTHZ_SORT_FIELD_FORBIDDEN);
    }

    fn with_clearance(ctx: SecurityContext, level: SecurityLevel) -> SecurityContext {
        let input = ep_foundation::security::context::HumanContextInput {
            user_id: ctx.user_id,
            session_id: ctx.session_id,
            legal_entity_id: ctx.legal_entity_id,
            device_id: ctx.device_id,
            client: ctx.client,
            clearance_level: level,
            roles: ctx.roles,
            duty_classes: ctx.duty_classes,
            department_scope: ctx.department_scope,
            position_ids: ctx.position_ids,
            project_scope: ctx.project_scope,
            customer_scope: ctx.customer_scope,
            record_shares: ctx.record_shares,
            data_scope_tags: ctx.data_scope_tags,
            snapshot_version: ctx.snapshot_version,
            is_breakglass: ctx.is_breakglass,
            request_id: ctx.request_id,
            trace_id: ctx.trace_id,
        };
        SecurityContext::human(input)
    }
}
