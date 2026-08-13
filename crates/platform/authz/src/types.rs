//! 授权域核心类型。
//!
//! 冻结出处：04 计划 §4.1 L286-L324 的类型表——六动作、判定与九变体拒绝理由、
//! 记录范围与谓词、字段可见性四态、六类高风险操作。受限声明式策略条件取
//! 六属性 × 五断言的合取形态，serde 强类型，非表达式语言（04:L307-L312）。

use ep_foundation::error::codes::{
    PLATFORM_AUTHZ_DIRECT_DB_ACCESS_FORBIDDEN, PLATFORM_AUTHZ_ISOLATION_CONTROL_FORBIDDEN,
    PLATFORM_AUTHZ_PERMISSION_ITEM_UNKNOWN,
};
use ep_foundation::error::AppError;
use ep_foundation::id::marker::{Customer, Department, Project};
use ep_foundation::id::Id;
use ep_foundation::security::context::ClientKind;
use ep_foundation::security::{SecurityContext, SecurityLevel};
use std::sync::Arc;

/// 六个动作，与 `permission_items.allowed_actions` 的六值一一对应。
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "UPPERCASE")]
pub enum Action {
    View,
    Create,
    Update,
    Submit,
    Approve,
    Export,
}

impl Action {
    pub const ALL: [Action; 6] = [
        Action::View,
        Action::Create,
        Action::Update,
        Action::Submit,
        Action::Approve,
        Action::Export,
    ];

    /// `role_permission_grants.action` 列的落库字面量。
    pub const fn as_str(self) -> &'static str {
        match self {
            Action::View => "VIEW",
            Action::Create => "CREATE",
            Action::Update => "UPDATE",
            Action::Submit => "SUBMIT",
            Action::Approve => "APPROVE",
            Action::Export => "EXPORT",
        }
    }
}

/// 六类高风险操作，与 `reauth_challenges.operation_type` 的取值一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HighRiskOperation {
    ContractEffective,
    Payment,
    InvoiceIssue,
    LedgerPosting,
    PeriodClose,
    SensitiveExport,
}

impl HighRiskOperation {
    /// 移动端发起即拒的四类受限操作（04:L512）。
    pub const MOBILE_RESTRICTED: [HighRiskOperation; 4] = [
        HighRiskOperation::Payment,
        HighRiskOperation::InvoiceIssue,
        HighRiskOperation::LedgerPosting,
        HighRiskOperation::PeriodClose,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            HighRiskOperation::ContractEffective => "CONTRACT_EFFECTIVE",
            HighRiskOperation::Payment => "PAYMENT",
            HighRiskOperation::InvoiceIssue => "INVOICE_ISSUE",
            HighRiskOperation::LedgerPosting => "LEDGER_POSTING",
            HighRiskOperation::PeriodClose => "PERIOD_CLOSE",
            HighRiskOperation::SensitiveExport => "SENSITIVE_EXPORT",
        }
    }

    /// 移动端四类受限操作在发起挑战处即拒。
    pub fn is_mobile_restricted(self, client: ClientKind) -> bool {
        matches!(client, ClientKind::Ios | ClientKind::Android)
            && Self::MOBILE_RESTRICTED.contains(&self)
    }
}

/// 判定结果。法人维度由 RLS 承担，判定流水线不做应用侧法人比较。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Decision {
    Allow,
    Deny(DenyReason),
}

/// 九个拒绝理由，与 04 计划 L300-L324 逐项一致。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DenyReason {
    LegalEntityNotGranted,
    ObjectForbidden,
    RecordNotVisible,
    FieldForbidden { field: String },
    ClassificationTooHigh { required: SecurityLevel },
    SeparationOfDutyViolation { rule_code: String },
    ReauthRequired { operation: HighRiskOperation },
    ApprovalRequired { chain_code: String },
    ScopeBindingMissing { object_type: String },
}

impl DenyReason {
    /// `ep_authz_denied_total` 的 reason 标签取值。
    pub fn as_metric_reason(&self) -> &'static str {
        match self {
            DenyReason::LegalEntityNotGranted => "legal_entity_not_granted",
            DenyReason::ObjectForbidden => "object_forbidden",
            DenyReason::RecordNotVisible => "record_not_visible",
            DenyReason::FieldForbidden { .. } => "field_forbidden",
            DenyReason::ClassificationTooHigh { .. } => "classification_too_high",
            DenyReason::SeparationOfDutyViolation { .. } => "separation_of_duty",
            DenyReason::ReauthRequired { .. } => "reauth_required",
            DenyReason::ApprovalRequired { .. } => "approval_required",
            DenyReason::ScopeBindingMissing { .. } => "scope_binding_missing",
        }
    }
}

/// 记录级编译结果：全部可见、按谓词过滤、不可见。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RecordScope {
    All,
    Predicate(RecordPredicate),
    None,
}

/// IN 列表退化 EXISTS 的阈值：部门集合超过该值时谓词标记改走 EXISTS 子查询，
/// 渲染由 ep-adapter-db-pg 承接。
pub const IN_LIST_THRESHOLD: usize = 200;

/// 日期比较基准：一切「今天」一律取上海时区的日期，不用数据库会话时区。
pub const SHANGHAI_TODAY_SQL: &str = "(now() AT TIME ZONE 'Asia/Shanghai')::date";

/// 有效期窗口比较：`from_col <= 今天` 与 `to_col >= 今天`，列名来自对象范围绑定。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValidityWindow {
    pub from_col: Option<Arc<str>>,
    pub to_col: Option<Arc<str>>,
}

/// 记录级谓词。纯构造产物，不触库；SQL 渲染由 ep-adapter-db-pg 承接
/// （与 tenancy/idempotency 同构：谓词构造在 platform，渲染实现体在 adapter）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecordPredicate {
    /// 本人经 owner 列可见。
    pub owner_self: bool,
    /// 部门闭包展开后的部门集合（含自身）。
    pub departments: Arc<[Id<Department>]>,
    /// 部门集合超过 [`IN_LIST_THRESHOLD`] 时置真，渲染侧改走 EXISTS。
    pub prefer_exists_for_departments: bool,
    pub projects: Arc<[Id<Project>]>,
    pub customers: Arc<[Id<Customer>]>,
    /// 显式共享的记录标识，取自安全上下文的 `record_shares`。
    pub shared_record_ids: Arc<[uuid::Uuid]>,
    /// 行密级上限：当前主体的密级。
    pub max_security_level: SecurityLevel,
    /// 日期口径比较窗口，见 [`SHANGHAI_TODAY_SQL`]。
    pub validity_windows: Arc<[ValidityWindow]>,
}

/// 对象范围绑定，`object_scope_bindings` 的内存形态（登记制，无列法人）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectScopeBinding {
    pub object_type: Arc<str>,
    pub schema_name: Arc<str>,
    pub table_name: Arc<str>,
    pub owner_user_col: Option<Arc<str>>,
    pub owning_dept_col: Option<Arc<str>>,
    pub project_col: Option<Arc<str>>,
    pub customer_col: Option<Arc<str>>,
    pub security_level_col: Arc<str>,
    pub valid_from_col: Option<Arc<str>>,
    pub valid_to_col: Option<Arc<str>>,
}

/// 字段可见性四态。授权行缺省即拒绝，不入键集合。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE", tag = "kind")]
pub enum FieldVisibility {
    Hidden,
    Masked(MaskStyle),
    Read,
    Write,
}

/// 掩码样式三值，与 `field_permissions.mask_style` 的取值一致。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaskStyle {
    Full,
    KeepLast4,
    KeepDomain,
}

impl FieldVisibility {
    /// 同一字段多角色授权时取最宽松者：WRITE > READ > MASKED > HIDDEN。
    pub const fn rank(self) -> u8 {
        match self {
            FieldVisibility::Hidden => 0,
            FieldVisibility::Masked(_) => 1,
            FieldVisibility::Read => 2,
            FieldVisibility::Write => 3,
        }
    }

    /// 掩码与隐藏形态禁排序禁聚合。
    pub const fn forbids_sorting(self) -> bool {
        matches!(self, FieldVisibility::Hidden | FieldVisibility::Masked(_))
    }
}

/// 策略效果。显式拒绝优先：任一 DENY 命中即拒。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// 受限声明式条件：合取范式，子句全部成立才算命中。
#[derive(Clone, PartialEq, Eq, Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct PolicyCondition {
    pub clauses: Vec<PolicyClause>,
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyClause {
    pub attribute: PolicyAttribute,
    pub assertion: PolicyAssertion,
}

/// 六属性，与 04 计划 L307 一致。
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAttribute {
    Department,
    Position,
    Project,
    Customer,
    SecurityLevel,
    DataScopeTag,
}

/// 五断言：in / not_in / lte / gte / has_tag。
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyAssertion {
    In(Vec<String>),
    NotIn(Vec<String>),
    Gte(i64),
    Lte(i64),
    HasTag(String),
}

impl PolicyCondition {
    /// 对安全上下文求值。上下文在会话建立时已冻结授权集合，求值纯内存。
    pub fn evaluate(&self, ctx: &SecurityContext) -> bool {
        self.clauses.iter().all(|c| evaluate_clause(c, ctx))
    }
}

fn evaluate_clause(clause: &PolicyClause, ctx: &SecurityContext) -> bool {
    match (&clause.attribute, &clause.assertion) {
        (PolicyAttribute::SecurityLevel, PolicyAssertion::Gte(v)) => {
            ctx.clearance_level.code() as i64 >= *v
        }
        (PolicyAttribute::SecurityLevel, PolicyAssertion::Lte(v)) => {
            ctx.clearance_level.code() as i64 <= *v
        }
        (PolicyAttribute::DataScopeTag, PolicyAssertion::HasTag(tag)) => ctx
            .data_scope_tags
            .iter()
            .any(|t| t.as_str() == tag.as_str()),
        (PolicyAttribute::Department, PolicyAssertion::In(ids)) => department_hits(ctx, ids),
        (PolicyAttribute::Department, PolicyAssertion::NotIn(ids)) => !department_hits(ctx, ids),
        (PolicyAttribute::Position, PolicyAssertion::In(ids)) => ctx
            .position_ids
            .iter()
            .any(|p| ids.contains(&p.to_string())),
        (PolicyAttribute::Position, PolicyAssertion::NotIn(ids)) => !ctx
            .position_ids
            .iter()
            .any(|p| ids.contains(&p.to_string())),
        (PolicyAttribute::Project, PolicyAssertion::In(ids)) => ctx
            .project_scope
            .iter()
            .any(|p| ids.contains(&p.to_string())),
        (PolicyAttribute::Project, PolicyAssertion::NotIn(ids)) => !ctx
            .project_scope
            .iter()
            .any(|p| ids.contains(&p.to_string())),
        (PolicyAttribute::Customer, PolicyAssertion::In(ids)) => ctx
            .customer_scope
            .iter()
            .any(|c| ids.contains(&c.to_string())),
        (PolicyAttribute::Customer, PolicyAssertion::NotIn(ids)) => !ctx
            .customer_scope
            .iter()
            .any(|c| ids.contains(&c.to_string())),
        // 属性与断言的搭配仅限上表；其余组合按不命中处理。
        _ => false,
    }
}

fn department_hits(ctx: &SecurityContext, ids: &[String]) -> bool {
    use ep_foundation::security::context::DepartmentScope;
    match &ctx.department_scope {
        DepartmentScope::All => true,
        DepartmentScope::Subtree(root) => ids.contains(&root.to_string()),
        DepartmentScope::Explicit(list) => list.iter().any(|d| ids.contains(&d.to_string())),
    }
}

/// 权限项编码的两类禁用前缀（`ck_permission_items_forbidden_codes` 的应用侧守门）。
pub fn guard_permission_item_code(code: &str) -> Result<(), AppError> {
    if let Some(rest) = code.strip_prefix("platform.legal_entity_isolation") {
        if rest.is_empty() || rest.starts_with('.') {
            return Err(AppError::new(
                PLATFORM_AUTHZ_ISOLATION_CONTROL_FORBIDDEN,
                format!("权限项 {code} 触碰法人隔离禁入前缀"),
            ));
        }
    }
    if let Some(rest) = code.strip_prefix("platform.direct_db_access") {
        if rest.is_empty() || rest.starts_with('.') {
            return Err(AppError::new(
                PLATFORM_AUTHZ_DIRECT_DB_ACCESS_FORBIDDEN,
                format!("权限项 {code} 触碰直连数据库禁入前缀"),
            ));
        }
    }
    Ok(())
}

/// 权限项编码形态：`<module>.<table>` 小写下划线两段。
pub fn check_permission_item_shape(code: &str) -> Result<(), AppError> {
    let err = || {
        AppError::new(
            PLATFORM_AUTHZ_PERMISSION_ITEM_UNKNOWN,
            format!("权限项编码 {code} 形态非法，需为 <module>.<table> 小写下划线"),
        )
    };
    let (module, table) = code.split_once('.').ok_or_else(err)?;
    if module.is_empty() || table.is_empty() {
        return Err(err());
    }
    let ok = |s: &str| {
        s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    };
    if !ok(module) || !ok(table) {
        return Err(err());
    }
    Ok(())
}

/// 小写十六进制编码。
pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// 小写十六进制解码，非法字符返回 None。
pub fn hex_decode(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(text.len() / 2);
    for i in (0..text.len()).step_by(2) {
        out.push(u8::from_str_radix(&text[i..i + 2], 16).ok()?);
    }
    Some(out)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ep_foundation::id::marker::LegalEntity;
    use ep_foundation::security::context::{
        ClientKind, DataScopeTag, DepartmentScope, DeviceId, HumanContextInput, RequestId,
        RoleCode, TraceId,
    };

    pub(crate) fn ctx_with(roles: Vec<&str>, client: ClientKind) -> SecurityContext {
        SecurityContext::human(HumanContextInput {
            user_id: Id::from_uuid(uuid::Uuid::from_u128(1)),
            session_id: Id::from_uuid(uuid::Uuid::from_u128(2)),
            legal_entity_id: Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(3)),
            device_id: DeviceId::new("DEV01").expect("合法"),
            client,
            clearance_level: SecurityLevel::Internal,
            roles: Arc::from(
                roles
                    .into_iter()
                    .map(|r| RoleCode::new(r).expect("大写"))
                    .collect::<Vec<_>>(),
            ),
            duty_classes: Arc::from([] as [ep_foundation::security::context::DutyClass; 0]),
            department_scope: DepartmentScope::All,
            position_ids: Arc::from([] as [Id<ep_foundation::id::marker::Position>; 0]),
            project_scope: Arc::from([] as [Id<Project>; 0]),
            customer_scope: Arc::from([] as [Id<Customer>; 0]),
            record_shares: Arc::from([] as [ep_foundation::security::context::RecordShare; 0]),
            data_scope_tags: Arc::from([DataScopeTag::new("region:APAC").expect("合法")]),
            snapshot_version: 1,
            is_breakglass: false,
            request_id: RequestId::new("0199aa11").expect("合法"),
            trace_id: TraceId::new(&"0".repeat(32)).expect("合法"),
        })
    }

    #[test]
    fn action_literals_match_the_six_values() {
        let want = ["VIEW", "CREATE", "UPDATE", "SUBMIT", "APPROVE", "EXPORT"];
        for (i, a) in Action::ALL.iter().enumerate() {
            assert_eq!(a.as_str(), want[i]);
        }
    }

    #[test]
    fn mobile_restricted_covers_exactly_four_operations() {
        for op in HighRiskOperation::MOBILE_RESTRICTED {
            assert!(op.is_mobile_restricted(ClientKind::Ios));
            assert!(op.is_mobile_restricted(ClientKind::Android));
            assert!(!op.is_mobile_restricted(ClientKind::Win));
        }
        assert!(!HighRiskOperation::ContractEffective.is_mobile_restricted(ClientKind::Ios));
        assert!(!HighRiskOperation::SensitiveExport.is_mobile_restricted(ClientKind::Android));
    }

    #[test]
    fn deny_reason_metric_labels_are_stable() {
        assert_eq!(
            DenyReason::ScopeBindingMissing {
                object_type: "x".into()
            }
            .as_metric_reason(),
            "scope_binding_missing"
        );
        assert_eq!(
            DenyReason::ObjectForbidden.as_metric_reason(),
            "object_forbidden"
        );
    }

    #[test]
    fn condition_evaluation_is_conjunctive() {
        let ctx = ctx_with(vec!["SALES"], ClientKind::Win);
        let cond = PolicyCondition {
            clauses: vec![
                PolicyClause {
                    attribute: PolicyAttribute::SecurityLevel,
                    assertion: PolicyAssertion::Gte(20),
                },
                PolicyClause {
                    attribute: PolicyAttribute::DataScopeTag,
                    assertion: PolicyAssertion::HasTag("region:APAC".into()),
                },
            ],
        };
        assert!(cond.evaluate(&ctx));
        let deny = PolicyCondition {
            clauses: vec![PolicyClause {
                attribute: PolicyAttribute::SecurityLevel,
                assertion: PolicyAssertion::Gte(30),
            }],
        };
        assert!(!deny.evaluate(&ctx), "clearance 20 不满足 gte 30");
        assert!(
            PolicyCondition::default().evaluate(&ctx),
            "空条件无条件命中"
        );
    }

    #[test]
    fn forbidden_permission_prefixes_are_guarded() {
        assert!(guard_permission_item_code("platform.legal_entity_isolation").is_err());
        assert!(guard_permission_item_code("platform.legal_entity_isolation.x").is_err());
        assert!(guard_permission_item_code("platform.direct_db_access").is_err());
        assert!(guard_permission_item_code("sales.sales_order").is_ok());
    }

    #[test]
    fn permission_item_shape_is_two_lowercase_segments() {
        assert!(check_permission_item_shape("sales.sales_order").is_ok());
        assert!(check_permission_item_shape("Sales.x").is_err());
        assert!(check_permission_item_shape("noseparator").is_err());
        assert!(check_permission_item_shape(".x").is_err());
    }

    #[test]
    fn hex_round_trips() {
        let bytes = [0x00u8, 0x7f, 0xff, 0x10];
        assert_eq!(hex_decode(&hex_encode(&bytes)).expect("可解"), bytes);
        assert!(hex_decode("abc").is_none(), "奇数长度拒");
        assert!(hex_decode("zz").is_none(), "非法字符拒");
    }

    #[test]
    fn visibility_rank_prefers_the_most_permissive() {
        assert!(FieldVisibility::Write.rank() > FieldVisibility::Read.rank());
        assert!(FieldVisibility::Read.rank() > FieldVisibility::Masked(MaskStyle::Full).rank());
        assert!(FieldVisibility::Masked(MaskStyle::Full).rank() > FieldVisibility::Hidden.rank());
        assert!(FieldVisibility::Masked(MaskStyle::KeepLast4).forbids_sorting());
        assert!(FieldVisibility::Hidden.forbids_sorting());
        assert!(!FieldVisibility::Read.forbids_sorting());
    }
}
