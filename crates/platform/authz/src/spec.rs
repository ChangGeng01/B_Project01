//! 三个 AUTHZ 类 applier 的配置条目规格类型。
//!
//! after_spec/before_spec 的 JSON 形态即本模块各结构体的
//! `deny_unknown_fields` 序列化形态（04 计划 §4.7）。

use serde::{Deserialize, Serialize};

use crate::sod::ApprovalNodeSpec;
use crate::types::{Action, FieldVisibility, PolicyCondition, PolicyEffect};

/// 一条角色授予规格：权限项编码 + 动作集合。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleGrantSpec {
    pub permission_item_code: String,
    pub actions: Vec<Action>,
}

/// 角色规格，AUTHZ_ROLE 的 after_spec/before_spec 形态。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoleSpec {
    pub role_code: String,
    pub is_portal_role: bool,
    pub grants: Vec<RoleGrantSpec>,
}

/// 一条访问策略规格。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySpec {
    pub policy_code: String,
    /// 空表示约束全部角色。
    pub role_code: Option<String>,
    pub object_type: String,
    pub effect: PolicyEffect,
    pub priority: i32,
    #[serde(default)]
    pub condition: PolicyCondition,
}

/// 一条职责互斥规则规格。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SodRuleSpec {
    pub rule_code: String,
    pub role_a: String,
    pub role_b: String,
}

/// 审批链规格：链码 + 节点序列。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalChainSpec {
    pub chain_code: String,
    pub nodes: Vec<ApprovalNodeSpec>,
}

/// 策略域规格，AUTHZ_POLICY 的 after_spec/before_spec 形态，
/// 覆盖 access_policies / sod_rules / approval_chains / approval_chain_nodes 四表。
#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct AuthzPolicySpec {
    pub policies: Vec<PolicySpec>,
    pub sod_rules: Vec<SodRuleSpec>,
    pub approval_chains: Vec<ApprovalChainSpec>,
}

/// 字段授权规格，AUTHZ_FIELD_GRANT 的 after_spec/before_spec 形态。
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldGrantSpec {
    pub role_code: String,
    pub object_type: String,
    pub field_name: String,
    pub visibility: FieldVisibility,
}
