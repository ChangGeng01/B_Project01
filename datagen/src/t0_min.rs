//! `t0-min` 最小样本档：一个法人、一个客户、一个产品。
//!
//! 列的取法只有一个来源：`docs/data-dictionary.md` 第 2 节的九个公共列，
//! 加档案类表的三个附加列 `code`、`is_active`、`deactivated_at`。
//! 为什么不写业务自有列：阶段 1 明写不建任何业务表，物理表与其自有列由后续阶段的
//! 迁移定义，此处若先发明一批列名，后续阶段建表时必然对不上而要么改样本档要么迁就它。
//!
//! `kind` 因此是逻辑实体名而非物理表名，落库映射由使用样本档的阶段自行给出。

use crate::record::{Dataset, Record, Value};
use crate::scale::Scale;
use crate::uuid7::{uuid_v7, BASE_INSTANT_RFC3339};

/// 三个逻辑实体名。条数判据按这三个名字数。
pub const KIND_LEGAL_ENTITY: &str = "legal_entity";
pub const KIND_CUSTOMER: &str = "customer";
pub const KIND_PRODUCT: &str = "product";

/// 取数点下标。写死而不按顺序自增：新增实体不得改动既有实体的 ID 取值。
const SLOT_LEGAL_ENTITY: u64 = 0;
const SLOT_CUSTOMER: u64 = 1;
const SLOT_PRODUCT: u64 = 2;

/// 内部密级，取数据字典第 3 节的 20。样本数据不涉密，不取更高级别。
const SECURITY_LEVEL_INTERNAL: i64 = 20;

/// 生成 `t0-min` 样本档。除 `seed` 外不读任何外部输入。
pub fn build(seed: u64) -> Dataset {
    let legal_entity_id = uuid_v7(seed, SLOT_LEGAL_ENTITY);
    let customer_id = uuid_v7(seed, SLOT_CUSTOMER);
    let product_id = uuid_v7(seed, SLOT_PRODUCT);

    let records = vec![
        archive_record(KIND_LEGAL_ENTITY, &legal_entity_id, &legal_entity_id, "LE-0001", "Legal Entity 0001"),
        archive_record(KIND_CUSTOMER, &customer_id, &legal_entity_id, "CUST-0001", "Customer 0001"),
        archive_record(KIND_PRODUCT, &product_id, &legal_entity_id, "PROD-0001", "Product 0001"),
    ];

    Dataset::new(Scale::T0Min.as_str(), seed, records)
}

/// 形状违反项。产出物不满足档位声明时逐条列出，不合并成一句话。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ShapeViolation {
    /// 某个逻辑实体的条数不是 1。
    WrongCount { kind: &'static str, expected: usize, actual: usize },
    /// 出现了三个逻辑实体之外的记录。
    UnexpectedKind(String),
    /// 跨法人引用。数据字典第 2 节明写禁止。
    CrossLegalEntity { kind: &'static str },
    /// 记录缺必填列。
    MissingField { kind: &'static str, field: &'static str },
}

/// 校验产出物确实是 `t0-min` 声称的形状。
///
/// 为什么在运行期再校一遍而不只靠单元测试：判据是「`t0-min` 生成一个法人一个客户一个产品」，
/// 而使用方拿到的是进程产出的文件，不是测试进程里的对象。这道校验让形状不符的样本档
/// 以非零退出码收尾，而不是被写出去等下游发现。
pub fn verify(dataset: &Dataset) -> Result<(), Vec<ShapeViolation>> {
    let mut violations = Vec::new();

    for kind in [KIND_LEGAL_ENTITY, KIND_CUSTOMER, KIND_PRODUCT] {
        let actual = dataset.count_of(kind);
        if actual != 1 {
            violations.push(ShapeViolation::WrongCount { kind, expected: 1, actual });
        }
    }
    for record in dataset.records() {
        if ![KIND_LEGAL_ENTITY, KIND_CUSTOMER, KIND_PRODUCT].contains(&record.kind()) {
            violations.push(ShapeViolation::UnexpectedKind(record.kind().to_string()));
        }
    }

    // 法人自身的 id 是唯一的合法 legal_entity_id 取值。
    let anchor = dataset
        .records()
        .iter()
        .find(|r| r.kind() == KIND_LEGAL_ENTITY)
        .and_then(|r| r.field("id"))
        .cloned();

    for record in dataset.records() {
        let kind = record.kind();
        let Some(owner) = record.field("legal_entity_id") else {
            violations.push(ShapeViolation::MissingField { kind, field: "legal_entity_id" });
            continue;
        };
        match &anchor {
            Some(a) if a == owner => {}
            Some(_) => violations.push(ShapeViolation::CrossLegalEntity { kind }),
            None => violations.push(ShapeViolation::MissingField {
                kind: KIND_LEGAL_ENTITY,
                field: "id",
            }),
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

impl std::fmt::Display for ShapeViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeViolation::WrongCount { kind, expected, actual } => {
                write!(f, "{kind} 应有 {expected} 条，实际 {actual} 条")
            }
            ShapeViolation::UnexpectedKind(kind) => write!(f, "出现档位之外的记录 {kind}"),
            ShapeViolation::CrossLegalEntity { kind } => {
                write!(f, "{kind} 的 legal_entity_id 指向了另一个法人")
            }
            ShapeViolation::MissingField { kind, field } => {
                write!(f, "{kind} 缺必填列 {field}")
            }
        }
    }
}

/// 一条档案类记录：九个公共列按数据字典的顺序，再接档案类的三个附加列。
fn archive_record(
    kind: &'static str,
    id: &str,
    legal_entity_id: &str,
    code: &str,
    name: &str,
) -> Record {
    // created_by 与 updated_by 一律引用系统主体常量，不另取字面量（数据字典第 2 节末段）。
    let system_principal = ep_foundation::principal::SYSTEM_PRINCIPAL_ID.to_string();
    Record::new(
        kind,
        vec![
            ("id", Value::Text(id.to_string())),
            ("legal_entity_id", Value::Text(legal_entity_id.to_string())),
            ("security_level", Value::Int(SECURITY_LEVEL_INTERNAL)),
            // 空数组，形态取公共列的默认值 '{}'。
            ("data_scope_tags", Value::Text("{}".to_string())),
            ("row_version", Value::Int(1)),
            ("created_at", Value::Text(BASE_INSTANT_RFC3339.to_string())),
            ("created_by", Value::Text(system_principal.clone())),
            ("updated_at", Value::Text(BASE_INSTANT_RFC3339.to_string())),
            ("updated_by", Value::Text(system_principal)),
            ("code", Value::Text(code.to_string())),
            ("name", Value::Text(name.to_string())),
            ("is_active", Value::Bool(true)),
            ("deactivated_at", Value::Null),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// D-09 硬判据之一：一个法人、一个客户、一个产品，一条不多一条不少。
    #[test]
    fn t0_min_has_exactly_one_of_each() {
        let ds = build(1);
        assert_eq!(ds.count_of(KIND_LEGAL_ENTITY), 1);
        assert_eq!(ds.count_of(KIND_CUSTOMER), 1);
        assert_eq!(ds.count_of(KIND_PRODUCT), 1);
        assert_eq!(ds.records().len(), 3, "t0-min 不得含第四条记录");
    }

    /// D-09 硬判据之一：同一 seed 两次生成字节一致。
    #[test]
    fn same_seed_yields_identical_bytes() {
        for seed in [0u64, 1, 42, u64::MAX] {
            let a = build(seed).encode().expect("t0-min 必须可编码");
            let b = build(seed).encode().expect("t0-min 必须可编码");
            assert_eq!(a, b, "seed={seed} 两次生成字节不一致");
        }
    }

    /// 负样例：换 seed 必须换字节。
    ///
    /// 断言的是生成规则本身：若 `build` 把 seed 丢掉而产出常量数据集，
    /// 上一条「同一 seed 字节一致」会平凡为真而判据形同虚设，本条拦这条退化路径。
    #[test]
    fn different_seed_yields_different_bytes() {
        let a = build(1).encode().unwrap();
        let b = build(2).encode().unwrap();
        assert_ne!(a, b, "两个不同 seed 产出了同一批字节");
    }

    /// 跨法人引用是数据字典明写禁止的，客户与产品必须挂在同一个法人下。
    #[test]
    fn customer_and_product_belong_to_the_only_legal_entity() {
        let ds = build(7);
        let le = ds.records().iter().find(|r| r.kind() == KIND_LEGAL_ENTITY).unwrap();
        let le_id = le.field("id").unwrap().clone();
        assert_eq!(le.field("legal_entity_id"), Some(&le_id), "法人自身的 legal_entity_id 应指向自己");
        for kind in [KIND_CUSTOMER, KIND_PRODUCT] {
            let r = ds.records().iter().find(|r| r.kind() == kind).unwrap();
            assert_eq!(r.field("legal_entity_id"), Some(&le_id), "{kind} 越法人引用");
        }
    }

    /// 三条记录的 ID 必须互不相同。
    #[test]
    fn ids_are_distinct() {
        let ds = build(3);
        let mut ids: Vec<_> = ds.records().iter().map(|r| r.field("id").unwrap().clone()).collect();
        ids.sort_by_key(|v| format!("{v:?}"));
        ids.dedup();
        assert_eq!(ids.len(), 3);
    }

    /// created_by 与 updated_by 必须是系统主体常量，不得另取字面量。
    #[test]
    fn audit_columns_use_system_principal() {
        let expected = Value::Text(ep_foundation::principal::SYSTEM_PRINCIPAL_ID.to_string());
        for r in build(5).records() {
            assert_eq!(r.field("created_by"), Some(&expected));
            assert_eq!(r.field("updated_by"), Some(&expected));
        }
    }

    /// 真产出物必须通过形状校验。
    #[test]
    fn built_dataset_passes_verify() {
        for seed in [0u64, 1, 42] {
            assert_eq!(verify(&build(seed)), Ok(()), "seed={seed}");
        }
    }

    fn probe(kind: &'static str, id: &str, owner: &str) -> Record {
        archive_record(kind, id, owner, "X-0001", "X")
    }

    /// 负样例：多出一个客户必须被校验规则本身拦下。
    #[test]
    fn extra_customer_is_rejected() {
        let ds = Dataset::new(
            "t0-min",
            1,
            vec![
                probe(KIND_LEGAL_ENTITY, "le", "le"),
                probe(KIND_CUSTOMER, "c1", "le"),
                probe(KIND_CUSTOMER, "c2", "le"),
                probe(KIND_PRODUCT, "p1", "le"),
            ],
        );
        let violations = verify(&ds).expect_err("两个客户必须判不符");
        assert!(violations.contains(&ShapeViolation::WrongCount {
            kind: KIND_CUSTOMER,
            expected: 1,
            actual: 2,
        }));
    }

    /// 负样例：缺产品必须被拦下，不得因为「法人与客户都在」就放行。
    #[test]
    fn missing_product_is_rejected() {
        let ds = Dataset::new(
            "t0-min",
            1,
            vec![probe(KIND_LEGAL_ENTITY, "le", "le"), probe(KIND_CUSTOMER, "c1", "le")],
        );
        let violations = verify(&ds).expect_err("缺产品必须判不符");
        assert!(violations.contains(&ShapeViolation::WrongCount {
            kind: KIND_PRODUCT,
            expected: 1,
            actual: 0,
        }));
    }

    /// 负样例：跨法人引用必须被拦下。
    #[test]
    fn cross_legal_entity_reference_is_rejected() {
        let ds = Dataset::new(
            "t0-min",
            1,
            vec![
                probe(KIND_LEGAL_ENTITY, "le", "le"),
                probe(KIND_CUSTOMER, "c1", "another-le"),
                probe(KIND_PRODUCT, "p1", "le"),
            ],
        );
        let violations = verify(&ds).expect_err("跨法人引用必须判不符");
        assert!(violations.contains(&ShapeViolation::CrossLegalEntity { kind: KIND_CUSTOMER }));
    }

    /// 负样例：档位之外的记录必须被拦下。
    #[test]
    fn unexpected_kind_is_rejected() {
        let ds = Dataset::new(
            "t0-min",
            1,
            vec![
                probe(KIND_LEGAL_ENTITY, "le", "le"),
                probe(KIND_CUSTOMER, "c1", "le"),
                probe(KIND_PRODUCT, "p1", "le"),
                Record::new("supplier", vec![("legal_entity_id", Value::Text("le".into()))]),
            ],
        );
        let violations = verify(&ds).expect_err("多出的实体必须判不符");
        assert!(violations.contains(&ShapeViolation::UnexpectedKind("supplier".to_string())));
    }

    /// 负样例：缺 legal_entity_id 必须报缺列，而不是被当成「没有跨法人问题」放行。
    #[test]
    fn missing_legal_entity_id_is_rejected() {
        let ds = Dataset::new(
            "t0-min",
            1,
            vec![
                probe(KIND_LEGAL_ENTITY, "le", "le"),
                probe(KIND_CUSTOMER, "c1", "le"),
                Record::new(KIND_PRODUCT, vec![("id", Value::Text("p1".into()))]),
            ],
        );
        let violations = verify(&ds).expect_err("缺 legal_entity_id 必须判不符");
        assert!(violations.contains(&ShapeViolation::MissingField {
            kind: KIND_PRODUCT,
            field: "legal_entity_id",
        }));
    }

    /// 九个公共列按数据字典第 2 节的顺序在前，档案类三个附加列在后。
    #[test]
    fn column_order_follows_the_data_dictionary() {
        let expected = [
            "id", "legal_entity_id", "security_level", "data_scope_tags", "row_version",
            "created_at", "created_by", "updated_at", "updated_by",
            "code", "name", "is_active", "deactivated_at",
        ];
        let bytes = build(1).encode().unwrap();
        let text = String::from_utf8(bytes).unwrap();
        let line = text.lines().find(|l| l.starts_with(KIND_LEGAL_ENTITY)).unwrap();
        let names: Vec<&str> = line
            .split('\t')
            .skip(1)
            .map(|kv| kv.split('=').next().unwrap())
            .collect();
        assert_eq!(names, expected);
    }
}
