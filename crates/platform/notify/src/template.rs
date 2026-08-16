//! 模板渲染与变量白名单。
//!
//! 计划第 3.8 节的测试计划逐字点名三件事：
//! 「模板变量白名单外的变量**拒绝渲染**、无权字段的**替代文案**、`dedupe_key` 的去重」。
//!
//! 白名单这一条是安全面的：通知文案会出现在列表、推送预览与锁屏上，
//! 把一个未经审查的变量放进模板，等于开一条把任意字段渲染到屏幕上的通道。
//! **白名单外一律拒绝渲染，不是渲染成空**——渲染成空会让模板作者以为写对了，
//! 而拒绝会让他当场知道这个变量不在白名单里。

use std::collections::BTreeMap;

/// 渲染失败的原因。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderError {
    /// 模板引用了白名单之外的变量。
    VariableNotAllowed { name: String },
    /// 模板引用的变量白名单里有，但本次没给值。
    VariableMissing { name: String },
    /// 占位符没闭合。
    MalformedPlaceholder { at: usize },
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::VariableNotAllowed { name } => write!(
                f,
                "模板变量 {name} 不在白名单内，拒绝渲染；新增变量须先登记再使用"
            ),
            RenderError::VariableMissing { name } => {
                write!(f, "模板变量 {name} 在白名单内但本次未提供取值")
            }
            RenderError::MalformedPlaceholder { at } => {
                write!(f, "第 {at} 字节处的占位符没有闭合")
            }
        }
    }
}

impl std::error::Error for RenderError {}

/// 无权字段的替代文案。
///
/// 接收人对某个字段无权时**不能把值渲进去，也不能把整条通知吞掉**——
/// 吞掉等于让他不知道有这件事发生，而「有事发生」这个事实本身通常不敏感。
/// 折中是渲染这一串替代文案。
pub const REDACTED_PLACEHOLDER: &str = "（无权查看）";

/// 渲染一条模板。
///
/// 占位符形如 `{name}`。`allowed` 是该 `notice_type` 的变量白名单，
/// `values` 是本次的取值；`values` 里某个键的值为 `None` 表示接收人对该字段无权，
/// 渲染成 [`REDACTED_PLACEHOLDER`]。
pub fn render(
    template: &str,
    allowed: &[&str],
    values: &BTreeMap<String, Option<String>>,
) -> Result<String, RenderError> {
    let mut out = String::with_capacity(template.len());
    let bytes = template.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'{' {
            // 按字符推进，避免在多字节字符中间切开。
            let ch = template[i..].chars().next().expect("i 落在字符边界上");
            out.push(ch);
            i += ch.len_utf8();
            continue;
        }
        let rest = &template[i + 1..];
        let Some(end) = rest.find('}') else {
            return Err(RenderError::MalformedPlaceholder { at: i });
        };
        let name = &rest[..end];
        if !allowed.contains(&name) {
            return Err(RenderError::VariableNotAllowed {
                name: name.to_string(),
            });
        }
        match values.get(name) {
            None => {
                return Err(RenderError::VariableMissing {
                    name: name.to_string(),
                })
            }
            Some(None) => out.push_str(REDACTED_PLACEHOLDER),
            Some(Some(v)) => out.push_str(v),
        }
        i += 1 + end + 1;
    }
    Ok(out)
}

/// 去重键。同一 `dedupe_key` 的通知在同一接收人上只保留一条。
///
/// 取值由触发源给出，本函数只负责拼形状——把它固定在一处，
/// 是为了不出现两个触发源各拼各的、于是同一件事去不掉重。
pub fn dedupe_key(notice_type: &str, subject_ref: &str, recipient_user_id: &str) -> String {
    format!("{notice_type}:{subject_ref}:{recipient_user_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vals(pairs: &[(&str, Option<&str>)]) -> BTreeMap<String, Option<String>> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.map(str::to_string)))
            .collect()
    }

    #[test]
    fn renders_allowed_variables() {
        let out = render(
            "合同 {contract_no} 待你审批",
            &["contract_no"],
            &vals(&[("contract_no", Some("HT-2026-001"))]),
        )
        .expect("应渲染成功");
        assert_eq!(out, "合同 HT-2026-001 待你审批");
    }

    /// 白名单外一律拒绝渲染，**不是渲染成空**。
    /// 渲染成空会让模板作者以为写对了；拒绝会让他当场知道变量没登记。
    #[test]
    fn variable_outside_the_whitelist_is_rejected_not_blanked() {
        let err = render(
            "客户 {customer_bank_account} 已到款",
            &["contract_no"],
            &vals(&[("customer_bank_account", Some("6222..."))]),
        )
        .expect_err("必须拒绝");
        match err {
            RenderError::VariableNotAllowed { name } => {
                assert_eq!(name, "customer_bank_account");
            }
            other => panic!("应报变量不在白名单，实为 {other}"),
        }
    }

    /// 无权字段渲染成替代文案，而不是吞掉整条通知——
    /// 吞掉等于让接收人不知道有这件事，而「有事发生」本身通常不敏感。
    #[test]
    fn unauthorized_field_becomes_a_placeholder_not_a_dropped_notice() {
        let out = render(
            "订单 {order_no} 金额 {amount}",
            &["order_no", "amount"],
            &vals(&[("order_no", Some("SO-1")), ("amount", None)]),
        )
        .expect("应渲染成功");
        assert_eq!(out, format!("订单 SO-1 金额 {REDACTED_PLACEHOLDER}"));
    }

    /// 白名单里有但没给值，是模板与调用方对不上，要报错而不是当成无权。
    /// 两者合并会把一个编码错误伪装成权限结果。
    #[test]
    fn missing_value_is_not_the_same_as_no_permission() {
        let err = render("{a}", &["a"], &vals(&[])).expect_err("必须报错");
        assert!(matches!(err, RenderError::VariableMissing { .. }));
    }

    #[test]
    fn unclosed_placeholder_is_rejected() {
        let err =
            render("合同 {contract_no 待审批", &["contract_no"], &vals(&[])).expect_err("必须报错");
        assert!(matches!(err, RenderError::MalformedPlaceholder { .. }));
    }

    /// 多字节字符不得被切开——模板全是中文，按字节推进会切出乱码。
    #[test]
    fn multibyte_text_survives_rendering() {
        let out = render("你好{x}世界", &["x"], &vals(&[("x", Some("——"))])).expect("应成功");
        assert_eq!(out, "你好——世界");
    }

    #[test]
    fn empty_template_and_no_placeholder() {
        assert_eq!(render("", &[], &vals(&[])).expect("空模板"), "");
        assert_eq!(
            render("纯文本", &[], &vals(&[])).expect("无占位符"),
            "纯文本"
        );
    }

    /// 去重键三段都参与：同一件事发给两个人是两条通知，不该互相去重。
    #[test]
    fn dedupe_key_includes_the_recipient() {
        let a = dedupe_key("CONTRACT_APPROVAL", "ct-1", "u1");
        let b = dedupe_key("CONTRACT_APPROVAL", "ct-1", "u2");
        assert_ne!(a, b, "同一件事发给两个人不得互相去重");
        assert_eq!(a, dedupe_key("CONTRACT_APPROVAL", "ct-1", "u1"));
    }
}
