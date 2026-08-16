//! 差异事项 `subject_ref` 的键集白名单。
//!
//! 阶段 9 计划第 9.4.7 节逐字：「唯一保留为硬约束的是输出边界：
//! `recon_discrepancies.subject_ref` 的允许键固定为**勾稽项标识、法人、会计期间、
//! 凭证号、仓库、物料、批次、科目、单据编号与账户内部标识十项**，写入时校验，
//! 出现白名单以外的键直接拒绝并按规格第 15.3 章告警。」
//!
//! # 这条白名单挡的是什么
//!
//! 同一份计划第 9.4.6 节逐字给出它的用处：「本阶段的校验语句输出列只含勾稽项标识、
//! 法人、会计期间、科目、凭证号与金额合计，**不含任何行内敏感字段**，
//! 该边界由第 9.4.7 节的 `subject_ref` 键集白名单在**写入侧**强制。」
//!
//! 也就是说它不是一条数据整洁度的规矩，是一道**数据外泄的闸**。
//! 对账跑在规格第 7.7 章的内部对账系统安全上下文里——该上下文按同节逐字
//! 「不调用字段投影器」，即它读得到的是该法人的全量合计，不经字段级裁剪。
//! 一个校验项若把它读到的东西顺手塞进 `subject_ref`，
//! 那份数据会落进 `recon_discrepancies`，而那张表是给数据责任人看的。
//!
//! **白名单必须是白名单，不能是黑名单**：黑名单要穷举所有不该出现的字段名，
//! 而那份清单随每个新模块增长；白名单只需说清允许哪十个。
//!
//! # 未覆盖：十个键的机读名是本实现取的，计划只给了中文
//!
//! 计划那句话给的是十个中文词，不是十个标识符。本模块按仓内既有列名给出机读名
//! （见 [`ALLOWED_KEYS`] 每一项的注释），但**「凭证号」与「单据编号」两项的分工
//! 判不出来**：阶段 9 计划第 3.2 节里凭证表自己的编号列就叫 `doc_no`，
//! 而白名单同时列了这两项，说明它们不是同一个东西。
//! 本实现取「凭证号 = `voucher_id`、单据编号 = `doc_no`」，登记为待裁定。

/// 允许出现在 `subject_ref` 里的十个键。**恰十个，一个不多一个不少。**
pub const ALLOWED_KEYS: [&str; 10] = [
    // 勾稽项标识。`recon_discrepancies` 自己就有这一列，同名同义。
    "check_code",
    // 法人。
    "legal_entity_id",
    // 会计期间。
    "accounting_period_id",
    // 凭证号。见模块文档：与「单据编号」的分工待裁定。
    "voucher_id",
    // 仓库。
    "warehouse_id",
    // 物料。
    "material_id",
    // 批次。
    "batch_no",
    // 科目。
    "account_id",
    // 单据编号。
    "doc_no",
    // 账户内部标识。规格第 7.8 章的行内敏感字段是**账号本身**，
    // 白名单收的是它的内部标识，不是账号——这两者差一层，差的就是这道闸的意义。
    "cash_account_id",
];

/// 校验失败的原因。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SubjectRefError {
    /// 出现了白名单以外的键。**列全而不是只报第一个**：
    /// 一个校验项一次塞进三个不该塞的字段时，只报一个会让人以为改掉那一处就能过。
    KeysNotAllowed { keys: Vec<String> },
    /// 一个键都没有。
    ///
    /// 空 `subject_ref` 判为不合法而不是合法：一条指不到具体对象的差异事项，
    /// 数据责任人拿到手里无从下手。**空集合不是「没有违规」**——
    /// 这与本卷别处「空登记表不是全部放行」是同一条纪律。
    Empty,
}

impl std::fmt::Display for SubjectRefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubjectRefError::KeysNotAllowed { keys } => write!(
                f,
                "subject_ref 出现白名单以外的键：{}；允许的十项见阶段 9 计划第 9.4.7 节",
                keys.join("、")
            ),
            SubjectRefError::Empty => {
                f.write_str("subject_ref 不得为空，一条指不到具体对象的差异事项无法处理")
            }
        }
    }
}

impl std::error::Error for SubjectRefError {}

/// 校验一组 `subject_ref` 的键。
///
/// 入参是**键的清单**而不是一个 JSON 值：本 crate 不接触 JSON
/// （`ep-platform-recon` 的依赖里没有 `serde_json`，也不该有），
/// 取键是调用方的事，判键是这里的事。
pub fn validate_keys<'a>(keys: impl IntoIterator<Item = &'a str>) -> Result<(), SubjectRefError> {
    let mut offending: Vec<String> = Vec::new();
    let mut seen_any = false;
    for k in keys {
        seen_any = true;
        if !ALLOWED_KEYS.contains(&k) && !offending.iter().any(|o| o == k) {
            offending.push(k.to_string());
        }
    }
    if !seen_any {
        return Err(SubjectRefError::Empty);
    }
    if offending.is_empty() {
        Ok(())
    } else {
        Err(SubjectRefError::KeysNotAllowed { keys: offending })
    }
}

/// 某个键在不在白名单里。供调用方在别处复用同一判据，不必各自再拼一遍。
pub fn is_allowed(key: &str) -> bool {
    ALLOWED_KEYS.contains(&key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_whitelist_has_exactly_the_ten_keys_the_plan_names() {
        assert_eq!(
            ALLOWED_KEYS.len(),
            10,
            "改这张表必须先改阶段 9 计划第 9.4.7 节"
        );
        let mut sorted = ALLOWED_KEYS;
        sorted.sort_unstable();
        let mut deduped = sorted.to_vec();
        deduped.dedup();
        assert_eq!(deduped.len(), 10, "十个键必须互异——写重一个就等于少一个");
    }

    #[test]
    fn every_allowed_key_passes() {
        assert_eq!(validate_keys(ALLOWED_KEYS), Ok(()));
        for k in ALLOWED_KEYS {
            assert!(is_allowed(k), "{k} 在白名单里却被判否");
            assert_eq!(validate_keys([k]), Ok(()));
        }
    }

    /// 本模块的要害：白名单以外的键一律拒。
    /// 这不是数据整洁度的规矩，是一道数据外泄的闸——
    /// 对账跑在不调用字段投影器的安全上下文里，它读得到该法人的全量合计。
    #[test]
    fn keys_outside_the_whitelist_are_refused() {
        for bad in [
            "bank_account_no", // 账号本身，正是规格第 7.8 章的行内敏感字段
            "id_card_no",
            "salary",
            "customer_name",
            "remark",
            "amount", // 金额不在十项内——差异表自己有三个金额列，不该再塞进来
        ] {
            assert!(!is_allowed(bad), "{bad} 不该在白名单里");
            assert!(
                matches!(
                    validate_keys([bad]),
                    Err(SubjectRefError::KeysNotAllowed { .. })
                ),
                "{bad} 应被拒绝"
            );
        }
    }

    /// 一次塞进多个不该塞的键时要**列全**，不能只报第一个——
    /// 只报一个会让人以为改掉那一处就能过。
    #[test]
    fn every_offending_key_is_listed_not_just_the_first() {
        let err = validate_keys(["legal_entity_id", "salary", "doc_no", "id_card_no"])
            .expect_err("应拒绝");
        match err {
            SubjectRefError::KeysNotAllowed { keys } => {
                assert_eq!(keys, vec!["salary".to_string(), "id_card_no".to_string()]);
            }
            other => panic!("应报白名单外的键，实为 {other}"),
        }
    }

    /// 同一个违规键出现两次只报一次——报重了会让人以为有两处要改。
    #[test]
    fn a_repeated_offender_is_reported_once() {
        let err = validate_keys(["salary", "salary"]).expect_err("应拒绝");
        match err {
            SubjectRefError::KeysNotAllowed { keys } => assert_eq!(keys.len(), 1),
            other => panic!("实为 {other}"),
        }
    }

    /// 空 `subject_ref` 判为不合法。
    /// **空集合不是「没有违规」**——一条指不到具体对象的差异事项无法处理，
    /// 而一个只做「逐键查白名单」的实现会让空集合静默通过。
    #[test]
    fn an_empty_subject_ref_is_not_an_implicit_pass() {
        assert_eq!(validate_keys([]), Err(SubjectRefError::Empty));
    }

    /// 大小写敏感：`Legal_Entity_Id` 不是 `legal_entity_id`。
    /// 放松大小写等于让白名单多出一批没被审过的写法。
    #[test]
    fn the_whitelist_is_case_sensitive() {
        assert!(!is_allowed("Legal_Entity_Id"));
        assert!(!is_allowed("LEGAL_ENTITY_ID"));
    }

    #[test]
    fn error_messages_name_the_offenders_and_cite_the_rule() {
        let msg = validate_keys(["salary"]).expect_err("应拒绝").to_string();
        assert!(msg.contains("salary"), "要点名违规键，实为 {msg}");
        assert!(msg.contains("9.4.7"), "要点出规则出处，实为 {msg}");
    }
}
