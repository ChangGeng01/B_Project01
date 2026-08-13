//! RLS 会话变量。四条变量的名字、顺序与写入语句在本模块统一定义，
//! 池钩子与事务流程一律引用这里的常量，不得各自拼语句。
//!
//! 会话变量固定顺序四条 `app.legal_entity_id`、`app.user_id`、
//! `app.request_id`、`app.trace_id`，出处是技术基线第 1.4 节与
//! db/migrations 的 RLS 模板（全库唯一策略函数 `platform_core.apply_le_rls`
//! 引用 `current_setting('app.legal_entity_id', true)`）。
//!
//! 写入一律用 `select set_config($1, $2, false)` 的参数化形态，第三参
//! `false` 表示会话级生效而不是仅当前事务。归还连接前逐项设回空串，
//! 不使用 DISCARD ALL。变量缺失即默认拒绝，因此清除必须写空串而不是
//! 删变量。

use ep_foundation::error::AppError;
use ep_foundation::security::SecurityContext;

use crate::conn::{DbConn, DbValue};

/// 四条会话变量名，顺序即写入与清除的固定顺序。
pub const SESSION_VARS: [&str; 4] = [
    "app.legal_entity_id",
    "app.user_id",
    "app.request_id",
    "app.trace_id",
];

/// 会话变量的写入语句，参数化形态。连接钩子（sqlx 路径）同样引用本常量，
/// 全工作区只此一处拼写。
pub const SET_SESSION_VAR_STMT: &str = "select set_config($1, $2, false)";

/// 一次事务要写入连接的四条会话变量取值，字段顺序与 [`SESSION_VARS`] 一一对应。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SessionContext {
    pub legal_entity_id: String,
    pub user_id: String,
    pub request_id: String,
    pub trace_id: String,
}

impl SessionContext {
    /// 从安全上下文取四条取值。标识一律取其 UUID 文本形态。
    pub fn from_security(ctx: &SecurityContext) -> Self {
        Self {
            legal_entity_id: ctx.legal_entity_id.as_uuid().to_string(),
            user_id: ctx.user_id.as_uuid().to_string(),
            request_id: ctx.request_id.as_str().to_string(),
            trace_id: ctx.trace_id.as_str().to_string(),
        }
    }

    /// 按 [`SESSION_VARS`] 的固定顺序给出四个取值。
    pub fn values(&self) -> [String; 4] {
        [
            self.legal_entity_id.clone(),
            self.user_id.clone(),
            self.request_id.clone(),
            self.trace_id.clone(),
        ]
    }

    /// 按固定顺序把四条变量写入连接。任何一条失败即整体失败。
    pub async fn apply(&self, conn: &mut dyn DbConn) -> Result<(), AppError> {
        for (name, value) in SESSION_VARS.iter().zip(self.values()) {
            conn.execute(
                SET_SESSION_VAR_STMT,
                &[DbValue::Text((*name).to_string()), DbValue::Text(value)],
            )
            .await
            .map_err(|e| e.into_app_error())?;
        }
        Ok(())
    }

    /// 归还连接前逐项设回空串，顺序与写入一致。
    pub async fn clear(conn: &mut dyn DbConn) -> Result<(), AppError> {
        for name in SESSION_VARS {
            conn.execute(
                SET_SESSION_VAR_STMT,
                &[
                    DbValue::Text(name.to_string()),
                    DbValue::Text(String::new()),
                ],
            )
            .await
            .map_err(|e| e.into_app_error())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ep_foundation::id::Id;
    use ep_foundation::principal::SYSTEM_PRINCIPAL_ID;
    use ep_foundation::security::context::{RequestId, TraceId};

    use super::*;
    use crate::fake::{FakeConn, FakeOp};

    fn ctx() -> SecurityContext {
        SecurityContext::system(
            Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            RequestId::new("0199aa11bb22cc33").expect("固定取值合法"),
            TraceId::new("0199aa11bb22cc330199aa11bb22cc33").expect("固定取值合法"),
        )
    }

    #[test]
    fn session_vars_order_is_frozen() {
        assert_eq!(
            SESSION_VARS,
            [
                "app.legal_entity_id",
                "app.user_id",
                "app.request_id",
                "app.trace_id",
            ],
            "四条变量的名字与顺序是 RLS 模板的引用对象，不得改动"
        );
    }

    #[tokio::test]
    async fn apply_writes_four_vars_in_fixed_order() {
        let mut conn = FakeConn::new();
        let sc = SessionContext::from_security(&ctx());
        sc.apply(&mut conn).await.expect("写入应成功");
        let writes: Vec<_> = conn
            .ops
            .iter()
            .filter_map(|op| match op {
                FakeOp::Execute(sql, params) if sql == SET_SESSION_VAR_STMT => {
                    Some((params[0].clone(), params[1].clone()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(writes.len(), 4, "必须恰好写四条");
        let names: Vec<_> = writes.iter().map(|(n, _)| n.clone()).collect();
        assert_eq!(
            names,
            SESSION_VARS.map(|s| DbValue::Text(s.to_string())),
            "写入顺序必须与登记顺序一致"
        );
        assert_eq!(writes[0].1, DbValue::Text(SYSTEM_PRINCIPAL_ID.to_string()));
    }

    #[tokio::test]
    async fn clear_sets_every_var_back_to_empty() {
        let mut conn = FakeConn::new();
        SessionContext::clear(&mut conn).await.expect("清除应成功");
        let clears: Vec<_> = conn
            .ops
            .iter()
            .filter_map(|op| match op {
                FakeOp::Execute(sql, params) if sql == SET_SESSION_VAR_STMT => {
                    Some((params[0].clone(), params[1].clone()))
                }
                _ => None,
            })
            .collect();
        assert_eq!(clears.len(), 4);
        for (_, value) in &clears {
            assert_eq!(*value, DbValue::Text(String::new()), "清除即设回空串");
        }
    }
}
