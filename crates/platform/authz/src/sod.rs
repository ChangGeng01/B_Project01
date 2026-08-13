//! SoD（职责分离）四类静态校验纯函数。
//!
//! 配置保存期与运行期共用同一份纯函数（04 计划 §4.5）：
//! 1. DUTY_EXCLUSION——五类管理员（SYSTEM/DATA/SECURITY/AUDIT/KEY）两两互斥；
//!    CONFIG 属临时取用职责，仅与 SECURITY 互斥；
//! 2. ROLE_EXCLUSION——sod_rules 登记的角色对不得同授一人；
//! 3. SELF_APPROVAL——审批节点展开用户集与发起人集交集须为空，冲突指出节点号；
//! 4. CHAIN_SKIP——审批链不存在「跳过」：节点号自 1 连续、quorum 介于 1 与
//!    节点展开人数之间，不存在跳过字段。
//!
//! 节点展开为空：保存期拒绝保存，运行期返 PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER。

use ep_foundation::error::codes::{
    PLATFORM_APPROVAL_NODE_HAS_NO_APPROVER, PLATFORM_REQUEST_INVALID_PAYLOAD,
    PLATFORM_SOD_DUTY_CONFLICT, PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN,
};
use ep_foundation::error::AppError;
use ep_foundation::security::context::DutyClass;

/// 五类常任管理员职责，两两互斥。
pub const EXCLUSIVE_DUTIES: [DutyClass; 5] = [
    DutyClass::System,
    DutyClass::Data,
    DutyClass::Security,
    DutyClass::Audit,
    DutyClass::Key,
];

/// 一条角色互斥规则（`sod_rules` 的运行形态）。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SodRoleRule {
    pub rule_code: String,
    pub role_a: String,
    pub role_b: String,
}

/// 审批节点规格：序号、法定数与展开后的审批人用户集。
/// 用户标识以 UUID 承载，便于规格 JSON 序列化。
#[derive(Clone, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalNodeSpec {
    pub node_seq: i32,
    pub quorum: u32,
    pub approver_user_ids: Vec<uuid::Uuid>,
}

/// SoD 违例的分类结果。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SodViolation {
    DutyConflict { pair: (DutyClass, DutyClass) },
    RoleExclusion { rule_code: String },
    SelfApproval { node_seq: i32 },
    ChainSkip { detail: String },
    NodeHasNoApprover { node_seq: i32 },
}

/// 一：职责互斥。五类管理员两两不得兼任；CONFIG 仅与 SECURITY 互斥。
pub fn check_duty_exclusion(duties: &[DutyClass]) -> Option<SodViolation> {
    for (i, a) in duties.iter().enumerate() {
        for b in duties.iter().skip(i + 1) {
            if a == b {
                continue;
            }
            let both_exclusive = EXCLUSIVE_DUTIES.contains(a) && EXCLUSIVE_DUTIES.contains(b);
            let config_vs_security = matches!(
                (a, b),
                (DutyClass::Config, DutyClass::Security) | (DutyClass::Security, DutyClass::Config)
            );
            if both_exclusive || config_vs_security {
                return Some(SodViolation::DutyConflict { pair: (*a, *b) });
            }
        }
    }
    None
}

/// 二：角色互斥。任一规则的两角色同时出现即违例。
pub fn check_role_exclusion(roles: &[&str], rules: &[SodRoleRule]) -> Option<SodViolation> {
    for rule in rules {
        let has_a = roles.contains(&rule.role_a.as_str());
        let has_b = roles.contains(&rule.role_b.as_str());
        if has_a && has_b {
            return Some(SodViolation::RoleExclusion {
                rule_code: rule.rule_code.clone(),
            });
        }
    }
    None
}

/// 三：自审自批。任一节点展开用户集命中任一发起人即违例，指出节点号。
pub fn check_self_approval(
    initiators: &[uuid::Uuid],
    nodes: &[ApprovalNodeSpec],
) -> Option<SodViolation> {
    for node in nodes {
        let hit = node
            .approver_user_ids
            .iter()
            .any(|u| initiators.contains(u));
        if hit {
            return Some(SodViolation::SelfApproval {
                node_seq: node.node_seq,
            });
        }
    }
    None
}

/// 四：链形完整性。节点号自 1 起连续、quorum 合法、无跳过可言。
pub fn check_chain_shape(nodes: &[ApprovalNodeSpec]) -> Option<SodViolation> {
    if nodes.is_empty() {
        return Some(SodViolation::ChainSkip {
            detail: "审批链没有任何节点".into(),
        });
    }
    for (i, node) in nodes.iter().enumerate() {
        let expected = (i + 1) as i32;
        if node.node_seq != expected {
            return Some(SodViolation::ChainSkip {
                detail: format!(
                    "节点号不连续：第 {i} 位应为 {expected}，实为 {}",
                    node.node_seq
                ),
            });
        }
        let count = node.approver_user_ids.len() as u32;
        if node.quorum < 1 || node.quorum > count.max(1) {
            return Some(SodViolation::ChainSkip {
                detail: format!("节点 {} 的 quorum {} 非法", node.node_seq, node.quorum),
            });
        }
    }
    None
}

/// 节点展开为空校验：保存期与运行期同一结论，仅出处不同。
pub fn check_nodes_non_empty(nodes: &[ApprovalNodeSpec]) -> Option<SodViolation> {
    nodes
        .iter()
        .find(|n| n.approver_user_ids.is_empty())
        .map(|n| SodViolation::NodeHasNoApprover {
            node_seq: n.node_seq,
        })
}

/// 违例 → 错误映射。
pub fn violation_error(v: &SodViolation) -> AppError {
    match v {
        SodViolation::DutyConflict { pair } => AppError::new(
            PLATFORM_SOD_DUTY_CONFLICT,
            format!("职责互斥冲突：{:?} 与 {:?} 不得兼任", pair.0, pair.1),
        ),
        SodViolation::RoleExclusion { rule_code } => AppError::new(
            PLATFORM_SOD_DUTY_CONFLICT,
            format!("角色互斥规则 {rule_code} 冲突"),
        ),
        SodViolation::SelfApproval { node_seq } => AppError::new(
            PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN,
            format!("节点 {node_seq} 的审批人包含发起人，禁止自审自批"),
        ),
        SodViolation::ChainSkip { detail } => AppError::new(
            PLATFORM_REQUEST_INVALID_PAYLOAD,
            format!("审批链形态非法：{detail}"),
        ),
        SodViolation::NodeHasNoApprover { node_seq } => AppError::new(
            PLATFORM_APPROVAL_NODE_HAS_NO_APPROVER,
            format!("节点 {node_seq} 展开后无任何审批人"),
        ),
    }
}

/// 保存期全量校验：四类顺序执行，任一违例即拒绝保存。
pub fn validate_for_save(
    duties: &[DutyClass],
    roles: &[&str],
    rules: &[SodRoleRule],
    initiators: &[uuid::Uuid],
    nodes: &[ApprovalNodeSpec],
) -> Result<(), AppError> {
    first_violation(duties, roles, rules, initiators, nodes)
        .map(|v| Err(violation_error(&v)))
        .unwrap_or(Ok(()))
}

/// 运行期校验：与保存期同一份纯函数；空节点同样返 NODE_HAS_NO_APPROVER。
pub fn validate_for_run(
    duties: &[DutyClass],
    roles: &[&str],
    rules: &[SodRoleRule],
    initiators: &[uuid::Uuid],
    nodes: &[ApprovalNodeSpec],
) -> Result<(), AppError> {
    validate_for_save(duties, roles, rules, initiators, nodes)
}

fn first_violation(
    duties: &[DutyClass],
    roles: &[&str],
    rules: &[SodRoleRule],
    initiators: &[uuid::Uuid],
    nodes: &[ApprovalNodeSpec],
) -> Option<SodViolation> {
    check_duty_exclusion(duties)
        .or_else(|| check_role_exclusion(roles, rules))
        .or_else(|| check_chain_shape(nodes))
        .or_else(|| check_nodes_non_empty(nodes))
        .or_else(|| check_self_approval(initiators, nodes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(n: u128) -> uuid::Uuid {
        uuid::Uuid::from_u128(n)
    }

    fn node(seq: i32, quorum: u32, users: &[uuid::Uuid]) -> ApprovalNodeSpec {
        ApprovalNodeSpec {
            node_seq: seq,
            quorum,
            approver_user_ids: users.to_vec(),
        }
    }

    #[test]
    fn five_admin_duties_are_pairwise_exclusive() {
        // 两两组合一律冲突。
        for (i, a) in EXCLUSIVE_DUTIES.iter().enumerate() {
            for b in EXCLUSIVE_DUTIES.iter().skip(i + 1) {
                let v = check_duty_exclusion(&[*a, *b]);
                assert!(
                    matches!(v, Some(SodViolation::DutyConflict { .. })),
                    "{a:?} 与 {b:?} 应互斥"
                );
            }
        }
    }

    #[test]
    fn config_conflicts_only_with_security() {
        let v = check_duty_exclusion(&[DutyClass::Config, DutyClass::Security]);
        assert!(matches!(v, Some(SodViolation::DutyConflict { .. })));
        for other in [DutyClass::Audit, DutyClass::Data] {
            // CONFIG 与其余两类的临时组合不在本校验职责内（另有审批链约束）。
            let v = check_duty_exclusion(&[DutyClass::Config, other]);
            assert!(v.is_none(), "CONFIG 与 {other:?} 不在互斥对");
        }
        assert!(check_duty_exclusion(&[DutyClass::Config]).is_none());
    }

    #[test]
    fn role_exclusion_hits_registered_pairs() {
        let rules = vec![SodRoleRule {
            rule_code: "SOD-001".into(),
            role_a: "PAY_MAKER".into(),
            role_b: "PAY_CHECKER".into(),
        }];
        let v = check_role_exclusion(&["PAY_MAKER", "PAY_CHECKER"], &rules);
        assert_eq!(
            v,
            Some(SodViolation::RoleExclusion {
                rule_code: "SOD-001".into()
            })
        );
        assert!(check_role_exclusion(&["PAY_MAKER"], &rules).is_none());
    }

    #[test]
    fn self_approval_points_at_the_node_seq() {
        let initiator = user(1);
        let nodes = vec![node(1, 1, &[user(2), user(3)]), node(2, 1, &[user(1)])];
        let v = check_self_approval(&[initiator], &nodes);
        assert_eq!(v, Some(SodViolation::SelfApproval { node_seq: 2 }));
        let clean = vec![node(1, 1, &[user(2)])];
        assert!(check_self_approval(&[initiator], &clean).is_none());
    }

    #[test]
    fn chain_shape_requires_contiguous_seq_and_legal_quorum() {
        let gap = vec![node(1, 1, &[user(2)]), node(3, 1, &[user(4)])];
        assert!(matches!(
            check_chain_shape(&gap),
            Some(SodViolation::ChainSkip { .. })
        ));
        let bad_quorum = vec![node(1, 3, &[user(2)])];
        assert!(matches!(
            check_chain_shape(&bad_quorum),
            Some(SodViolation::ChainSkip { .. })
        ));
        let zero_quorum = vec![node(1, 0, &[user(2)])];
        assert!(matches!(
            check_chain_shape(&zero_quorum),
            Some(SodViolation::ChainSkip { .. })
        ));
        let ok = vec![node(1, 1, &[user(2)]), node(2, 2, &[user(3), user(4)])];
        assert!(check_chain_shape(&ok).is_none());
        assert!(matches!(
            check_chain_shape(&[]),
            Some(SodViolation::ChainSkip { .. })
        ));
    }

    #[test]
    fn empty_expansion_rejects_save_and_runtime_alike() {
        let nodes = vec![node(1, 1, &[user(2)]), node(2, 1, &[])];
        for validate in [validate_for_save, validate_for_run] {
            let err = validate(&[], &[], &[], &[user(1)], &nodes).expect_err("空展开拒");
            assert_eq!(err.code, PLATFORM_APPROVAL_NODE_HAS_NO_APPROVER);
        }
    }

    #[test]
    fn clean_configuration_passes_and_conflicts_map_to_codes() {
        let nodes = vec![node(1, 1, &[user(2)])];
        assert!(validate_for_save(&[], &[], &[], &[user(1)], &nodes).is_ok());
        let err = validate_for_save(
            &[DutyClass::System, DutyClass::Audit],
            &[],
            &[],
            &[],
            &nodes,
        )
        .expect_err("职责冲突");
        assert_eq!(err.code, PLATFORM_SOD_DUTY_CONFLICT);
        let err = validate_for_save(&[], &[], &[], &[user(2)], &nodes).expect_err("自批");
        assert_eq!(err.code, PLATFORM_SOD_SELF_APPROVAL_FORBIDDEN);
    }
}
