//! `t0` 与 `small` 样本档：T0 最小样本的平台部分（D-09）。
//!
//! 口径来自阶段 2 计划 D-09：`--scale t0` 产出 1 个法人及其组织架构最小行，
//! `--scale small` 产出 2 个法人。两个档位共用同一套生成与校验，只差法人个数。
//!
//! 组织架构最小行按 `platform_core` 第 3.5 节的表形态取：每个法人配一个集团、
//! 一个组织（CORPORATION、无上级）、一个部门（挂该组织、无上级、level 1）、
//! 一个岗位（挂该部门、rank 1）。自有列的取值与迁移里的 CHECK 约束逐条对齐：
//! `entity_no` 是两位数字、`org_kind` 在三个允许取值内、`level_no` 与 `rank_no` 大于 0。
//!
//! 与 `t0_min` 同样的纪律：`kind` 是逻辑实体名而非物理表名，落库映射由
//! 使用样本档的阶段自行给出；除 `seed` 外不读任何外部输入。

use crate::record::{Dataset, Record, Value};
use crate::scale::Scale;
use crate::uuid7::{uuid_v7, BASE_INSTANT_RFC3339};

/// 五个逻辑实体名。条数判据按这五个名字数。
pub const KIND_ENTERPRISE_GROUP: &str = "enterprise_group";
pub const KIND_LEGAL_ENTITY: &str = "legal_entity";
pub const KIND_ORGANIZATION: &str = "organization";
pub const KIND_DEPARTMENT: &str = "department";
pub const KIND_POSITION: &str = "position";

/// 每个法人占十个取数点下标，五类记录在段内偏移固定。
/// 写死而不用计数器：新增实体不得改动既有实体的 ID 取值。
const STRIDE: u64 = 10;
const OFFSET_GROUP: u64 = 0;
const OFFSET_LEGAL_ENTITY: u64 = 1;
const OFFSET_ORGANIZATION: u64 = 2;
const OFFSET_DEPARTMENT: u64 = 3;
const OFFSET_POSITION: u64 = 4;

/// 内部密级，取数据字典第 3 节的 20。样本数据不涉密，不取更高级别。
const SECURITY_LEVEL_INTERNAL: i64 = 20;

/// 生成平台样本档。`legal_entities` 取 1 即 `t0`，取 2 即 `small`。
pub fn build(scale: Scale, seed: u64, legal_entities: usize) -> Dataset {
    let mut records = Vec::with_capacity(legal_entities * 5);
    for i in 0..legal_entities {
        let base = (i as u64) * STRIDE;
        let group_id = uuid_v7(seed, base + OFFSET_GROUP);
        let le_id = uuid_v7(seed, base + OFFSET_LEGAL_ENTITY);
        let org_id = uuid_v7(seed, base + OFFSET_ORGANIZATION);
        let dept_id = uuid_v7(seed, base + OFFSET_DEPARTMENT);
        let pos_id = uuid_v7(seed, base + OFFSET_POSITION);
        let no = format!("{:02}", i + 1);

        records.push(group_record(
            &group_id,
            &format!("GRP-{no}"),
            &format!("Enterprise Group {no}"),
        ));
        records.push(legal_entity_record(
            &le_id,
            &format!("LE-{no}"),
            &no,
            &format!("Legal Entity {no}"),
            &group_id,
        ));
        records.push(organization_record(
            &org_id,
            &le_id,
            &format!("ORG-{no}"),
            &format!("Organization {no}"),
        ));
        records.push(department_record(
            &dept_id,
            &le_id,
            &format!("DEPT-{no}"),
            &format!("Department {no}"),
            &org_id,
        ));
        records.push(position_record(
            &pos_id,
            &le_id,
            &format!("POS-{no}"),
            &format!("Position {no}"),
            &dept_id,
        ));
    }
    Dataset::new(scale.as_str(), seed, records)
}

/// 形状违反项。产出物不满足档位声明时逐条列出，不合并成一句话。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ShapeViolation {
    /// 某个逻辑实体的条数与档位声明不符。
    WrongCount {
        kind: &'static str,
        expected: usize,
        actual: usize,
    },
    /// 出现了五个逻辑实体之外的记录。
    UnexpectedKind(String),
    /// 记录的 legal_entity_id 指向了档位里不存在的法人。
    DanglingLegalEntity { kind: &'static str },
    /// 子记录的法人归属与其父链不一致（跨法人引用）。
    CrossLegalEntity { kind: &'static str },
    /// 记录缺必填列。
    MissingField {
        kind: &'static str,
        field: &'static str,
    },
    /// 两条记录的 id 相同。
    DuplicateId(String),
}

impl std::fmt::Display for ShapeViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeViolation::WrongCount {
                kind,
                expected,
                actual,
            } => write!(f, "{kind} 应有 {expected} 条，实际 {actual} 条"),
            ShapeViolation::UnexpectedKind(kind) => write!(f, "出现档位之外的记录 {kind}"),
            ShapeViolation::DanglingLegalEntity { kind } => {
                write!(f, "{kind} 的 legal_entity_id 指向了档位里不存在的法人")
            }
            ShapeViolation::CrossLegalEntity { kind } => {
                write!(f, "{kind} 的父链归属了另一个法人")
            }
            ShapeViolation::MissingField { kind, field } => {
                write!(f, "{kind} 缺必填列 {field}")
            }
            ShapeViolation::DuplicateId(id) => write!(f, "id {id} 出现了两次"),
        }
    }
}

const ALL_KINDS: [&str; 5] = [
    KIND_ENTERPRISE_GROUP,
    KIND_LEGAL_ENTITY,
    KIND_ORGANIZATION,
    KIND_DEPARTMENT,
    KIND_POSITION,
];

fn text_of(record: &Record, field: &'static str) -> Option<String> {
    match record.field(field) {
        Some(Value::Text(s)) => Some(s.clone()),
        _ => None,
    }
}

/// 校验产出物确实是档位声称的形状：五类记录各 `expected_les` 条，
/// 引用链集团→法人→组织→部门→岗位逐级闭合，且不跨法人。
pub fn verify(dataset: &Dataset, expected_les: usize) -> Result<(), Vec<ShapeViolation>> {
    let mut violations = Vec::new();

    for kind in ALL_KINDS {
        let actual = dataset.count_of(kind);
        if actual != expected_les {
            violations.push(ShapeViolation::WrongCount {
                kind,
                expected: expected_les,
                actual,
            });
        }
    }
    for record in dataset.records() {
        if !ALL_KINDS.contains(&record.kind()) {
            violations.push(ShapeViolation::UnexpectedKind(record.kind().to_string()));
        }
    }

    // id 全档位唯一：同一 id 落两行在库里就是主键冲突，必须在这里先拦。
    let mut seen = std::collections::BTreeSet::new();
    for record in dataset.records() {
        if let Some(Value::Text(id)) = record.field("id") {
            if !seen.insert(id.clone()) {
                violations.push(ShapeViolation::DuplicateId(id.clone()));
            }
        } else {
            violations.push(ShapeViolation::MissingField {
                kind: record.kind(),
                field: "id",
            });
        }
    }

    let le_ids: std::collections::BTreeSet<String> = dataset
        .records()
        .iter()
        .filter(|r| r.kind() == KIND_LEGAL_ENTITY)
        .filter_map(|r| text_of(r, "id"))
        .collect();
    let group_ids: std::collections::BTreeSet<String> = dataset
        .records()
        .iter()
        .filter(|r| r.kind() == KIND_ENTERPRISE_GROUP)
        .filter_map(|r| text_of(r, "id"))
        .collect();
    // 组织、部门各自按 id 记录其法人归属，供子链核对。
    let org_owner: std::collections::BTreeMap<String, String> = dataset
        .records()
        .iter()
        .filter(|r| r.kind() == KIND_ORGANIZATION)
        .filter_map(|r| {
            Some((
                text_of(r, "id")?,
                text_of(r, "legal_entity_id").unwrap_or_default(),
            ))
        })
        .collect();
    let dept_owner: std::collections::BTreeMap<String, String> = dataset
        .records()
        .iter()
        .filter(|r| r.kind() == KIND_DEPARTMENT)
        .filter_map(|r| {
            Some((
                text_of(r, "id")?,
                text_of(r, "legal_entity_id").unwrap_or_default(),
            ))
        })
        .collect();

    for record in dataset.records() {
        let kind = record.kind();
        match kind {
            KIND_ENTERPRISE_GROUP => {}
            KIND_LEGAL_ENTITY => {
                let Some(group) = text_of(record, "group_id") else {
                    violations.push(ShapeViolation::MissingField {
                        kind,
                        field: "group_id",
                    });
                    continue;
                };
                if !group_ids.contains(&group) {
                    violations.push(ShapeViolation::DanglingLegalEntity { kind });
                }
            }
            KIND_ORGANIZATION => {
                check_owner(record, &le_ids, &mut violations);
            }
            KIND_DEPARTMENT => {
                let owner = check_owner(record, &le_ids, &mut violations);
                if let Some(org) = text_of(record, "organization_id") {
                    if org_owner.get(&org) != owner.as_ref() {
                        violations.push(ShapeViolation::CrossLegalEntity { kind });
                    }
                } else {
                    violations.push(ShapeViolation::MissingField {
                        kind,
                        field: "organization_id",
                    });
                }
            }
            KIND_POSITION => {
                let owner = check_owner(record, &le_ids, &mut violations);
                if let Some(dept) = text_of(record, "department_id") {
                    if dept_owner.get(&dept) != owner.as_ref() {
                        violations.push(ShapeViolation::CrossLegalEntity { kind });
                    }
                } else {
                    violations.push(ShapeViolation::MissingField {
                        kind,
                        field: "department_id",
                    });
                }
            }
            _ => {}
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

/// 核对一条带 `legal_entity_id` 的记录：列必须在、取值必须在档位法人集合内。
/// 返回其法人归属供父链核对；列缺失或悬空时返回 None。
fn check_owner(
    record: &Record,
    le_ids: &std::collections::BTreeSet<String>,
    violations: &mut Vec<ShapeViolation>,
) -> Option<String> {
    let Some(owner) = text_of(record, "legal_entity_id") else {
        violations.push(ShapeViolation::MissingField {
            kind: record.kind(),
            field: "legal_entity_id",
        });
        return None;
    };
    if !le_ids.contains(&owner) {
        violations.push(ShapeViolation::DanglingLegalEntity {
            kind: record.kind(),
        });
        return None;
    }
    Some(owner)
}

/// 九个公共列中带 `legal_entity_id` 的八件套，再接实体自有列。
fn common_fields(id: &str, legal_entity_id: Option<&str>) -> Vec<(&'static str, Value)> {
    // created_by 与 updated_by 一律引用系统主体常量，不另取字面量（数据字典第 2 节末段）。
    let system_principal = ep_foundation::principal::SYSTEM_PRINCIPAL_ID.to_string();
    let mut fields = vec![("id", Value::Text(id.to_string()))];
    if let Some(le) = legal_entity_id {
        fields.push(("legal_entity_id", Value::Text(le.to_string())));
    }
    fields.extend([
        ("security_level", Value::Int(SECURITY_LEVEL_INTERNAL)),
        // 空数组，形态取公共列的默认值 '{}'。
        ("data_scope_tags", Value::Text("{}".to_string())),
        ("row_version", Value::Int(1)),
        ("created_at", Value::Text(BASE_INSTANT_RFC3339.to_string())),
        ("created_by", Value::Text(system_principal.clone())),
        ("updated_at", Value::Text(BASE_INSTANT_RFC3339.to_string())),
        ("updated_by", Value::Text(system_principal)),
    ]);
    fields
}

/// 集团行：不带 `legal_entity_id`（表十三登记豁免表）。
fn group_record(id: &str, code: &str, name: &str) -> Record {
    let mut fields = common_fields(id, None);
    fields.extend([
        ("code", Value::Text(code.to_string())),
        ("name", Value::Text(name.to_string())),
        ("is_active", Value::Bool(true)),
        ("deactivated_at", Value::Null),
    ]);
    Record::new(KIND_ENTERPRISE_GROUP, fields)
}

/// 法人行：不带 `legal_entity_id`（隔离机制自身的元数据），`entity_no` 取两位数字。
fn legal_entity_record(
    id: &str,
    code: &str,
    entity_no: &str,
    name: &str,
    group_id: &str,
) -> Record {
    let mut fields = common_fields(id, None);
    fields.extend([
        ("code", Value::Text(code.to_string())),
        ("entity_no", Value::Text(entity_no.to_string())),
        ("name", Value::Text(name.to_string())),
        ("is_active", Value::Bool(true)),
        ("deactivated_at", Value::Null),
        ("group_id", Value::Text(group_id.to_string())),
    ]);
    Record::new(KIND_LEGAL_ENTITY, fields)
}

/// 组织行：`org_kind` 取 CORPORATION，无上级组织。
fn organization_record(id: &str, le_id: &str, code: &str, name: &str) -> Record {
    let mut fields = common_fields(id, Some(le_id));
    fields.extend([
        ("code", Value::Text(code.to_string())),
        ("name", Value::Text(name.to_string())),
        ("org_kind", Value::Text("CORPORATION".to_string())),
        ("parent_organization_id", Value::Null),
        ("is_active", Value::Bool(true)),
    ]);
    Record::new(KIND_ORGANIZATION, fields)
}

/// 部门行：挂本法人组织、无上级部门、`level_no` 取 1。
fn department_record(id: &str, le_id: &str, code: &str, name: &str, org_id: &str) -> Record {
    let mut fields = common_fields(id, Some(le_id));
    fields.extend([
        ("code", Value::Text(code.to_string())),
        ("name", Value::Text(name.to_string())),
        ("organization_id", Value::Text(org_id.to_string())),
        ("parent_department_id", Value::Null),
        ("level_no", Value::Int(1)),
        ("is_active", Value::Bool(true)),
        ("deactivated_at", Value::Null),
    ]);
    Record::new(KIND_DEPARTMENT, fields)
}

/// 岗位行：挂本法人部门、`rank_no` 取 1。
fn position_record(id: &str, le_id: &str, code: &str, name: &str, dept_id: &str) -> Record {
    let mut fields = common_fields(id, Some(le_id));
    fields.extend([
        ("code", Value::Text(code.to_string())),
        ("name", Value::Text(name.to_string())),
        ("department_id", Value::Text(dept_id.to_string())),
        ("rank_no", Value::Int(1)),
        ("is_active", Value::Bool(true)),
        ("deactivated_at", Value::Null),
    ]);
    Record::new(KIND_POSITION, fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// D-09 硬判据：`t0` 是一个法人及其组织架构最小行，五类各一条。
    #[test]
    fn t0_has_one_legal_entity_with_full_org_skeleton() {
        let ds = build(Scale::T0, 1, 1);
        for kind in ALL_KINDS {
            assert_eq!(ds.count_of(kind), 1, "{kind} 应恰好一条");
        }
        assert_eq!(ds.records().len(), 5, "t0 不得含第六条记录");
    }

    /// D-09 硬判据：`small` 是两个法人，每个法人各带一套组织架构最小行。
    #[test]
    fn small_has_two_legal_entities_each_with_full_org_skeleton() {
        let ds = build(Scale::Small, 1, 2);
        for kind in ALL_KINDS {
            assert_eq!(ds.count_of(kind), 2, "{kind} 应恰好两条");
        }
        assert_eq!(ds.records().len(), 10, "small 不得含第十一条记录");
    }

    /// 同一 seed 两次生成字节一致。
    #[test]
    fn same_seed_yields_identical_bytes() {
        for (scale, n) in [(Scale::T0, 1), (Scale::Small, 2)] {
            for seed in [0u64, 1, 42, u64::MAX] {
                let a = build(scale, seed, n)
                    .encode()
                    .expect("平台样本档必须可编码");
                let b = build(scale, seed, n)
                    .encode()
                    .expect("平台样本档必须可编码");
                assert_eq!(a, b, "scale={scale:?} seed={seed} 两次生成字节不一致");
            }
        }
    }

    /// 负样例：换 seed 必须换字节，拦住「丢掉 seed 产出常量数据集」的退化。
    #[test]
    fn different_seed_yields_different_bytes() {
        let a = build(Scale::T0, 1, 1).encode().unwrap();
        let b = build(Scale::T0, 2, 1).encode().unwrap();
        assert_ne!(a, b, "两个不同 seed 产出了同一批字节");
    }

    /// 真产出物必须通过形状校验。
    #[test]
    fn built_datasets_pass_verify() {
        for seed in [0u64, 1, 42] {
            assert_eq!(verify(&build(Scale::T0, seed, 1), 1), Ok(()), "seed={seed}");
            assert_eq!(
                verify(&build(Scale::Small, seed, 2), 2),
                Ok(()),
                "seed={seed}"
            );
        }
    }

    /// 引用链上的 id 必须互不相同。
    #[test]
    fn ids_are_distinct() {
        let ds = build(Scale::Small, 3, 2);
        let mut ids: Vec<_> = ds
            .records()
            .iter()
            .map(|r| r.field("id").unwrap().clone())
            .collect();
        ids.sort_by_key(|v| format!("{v:?}"));
        ids.dedup();
        assert_eq!(ids.len(), 10);
    }

    /// 法人行的 entity_no 必须是两位数字，与迁移 CHECK 约束对齐。
    #[test]
    fn entity_no_is_two_digits() {
        for r in build(Scale::Small, 1, 2).records() {
            if r.kind() == KIND_LEGAL_ENTITY {
                let Some(Value::Text(no)) = r.field("entity_no") else {
                    panic!("法人行缺 entity_no")
                };
                assert_eq!(no.len(), 2);
                assert!(no.chars().all(|c| c.is_ascii_digit()), "{no}");
            }
        }
    }

    /// 负样例：部门挂到另一个法人的组织必须被拦下。
    #[test]
    fn cross_legal_entity_department_is_rejected() {
        // 两个法人的完整骨架，唯独 dept1 挂到 le2 的组织 org2 上。
        let ds = Dataset::new(
            "small",
            1,
            vec![
                group_record("g1", "GRP-01", "Group 01"),
                group_record("g2", "GRP-02", "Group 02"),
                legal_entity_record("le1", "LE-01", "01", "Legal Entity 01", "g1"),
                legal_entity_record("le2", "LE-02", "02", "Legal Entity 02", "g2"),
                organization_record("org1", "le1", "ORG-01", "Organization 01"),
                organization_record("org2", "le2", "ORG-02", "Organization 02"),
                department_record("dept1", "le1", "DEPT-01", "Department 01", "org2"),
                department_record("dept2", "le2", "DEPT-02", "Department 02", "org2"),
                position_record("pos1", "le1", "POS-01", "Position 01", "dept1"),
                position_record("pos2", "le2", "POS-02", "Position 02", "dept2"),
            ],
        );
        let violations = verify(&ds, 2).expect_err("跨法人部门必须判不符");
        assert!(
            violations.contains(&ShapeViolation::CrossLegalEntity {
                kind: KIND_DEPARTMENT
            }),
            "{violations:?}"
        );
    }

    /// 负样例：少一个法人必须被判不符，不得因为组织架构行都在就放行。
    #[test]
    fn missing_legal_entity_is_rejected() {
        let ds = build(Scale::T0, 1, 1);
        let violations = verify(&ds, 2).expect_err("一个法人对不上两法人的声明");
        assert!(violations.contains(&ShapeViolation::WrongCount {
            kind: KIND_LEGAL_ENTITY,
            expected: 2,
            actual: 1,
        }));
    }

    /// 负样例：档位之外的记录必须被拦下。
    #[test]
    fn unexpected_kind_is_rejected() {
        let mut ds = build(Scale::T0, 1, 1);
        let mut records = ds.records().to_vec();
        records.push(Record::new(
            "supplier",
            vec![("id", Value::Text("x".into()))],
        ));
        ds = Dataset::new("t0", 1, records);
        let violations = verify(&ds, 1).expect_err("多出的实体必须判不符");
        assert!(violations.contains(&ShapeViolation::UnexpectedKind("supplier".to_string())));
    }
}
