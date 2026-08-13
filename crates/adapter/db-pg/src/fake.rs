//! 假连接。纯逻辑单测的驱动替身：记录全部操作序列，可预置错误与查询结果。
//!
//! 本模块公开导出，供本 crate 单测与后续装配侧测试共用；
//! 它不触碰任何真实连接，也不引入数据库专有行为，
//! `begin`/`commit`/`rollback` 只改自身的 `in_tx` 标志。

use std::collections::VecDeque;

use ep_foundation::port::tx::IsolationKind;

use crate::conn::{DbConn, DbValue, PgError};
use crate::session::SET_SESSION_VAR_STMT;

/// 记录下来的一次连接操作。断言写入/清除序列、事务边界的依据。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FakeOp {
    Execute(String, Vec<DbValue>),
    Query(String, Vec<DbValue>),
    Begin {
        isolation: IsolationKind,
        read_only: bool,
    },
    Commit,
    Rollback,
}

#[derive(Default)]
pub struct FakeConn {
    /// 按发生顺序记录的全部操作。
    pub ops: Vec<FakeOp>,
    /// 预设的错误队列，execute/query 按序消费，每条只失败一次。
    pub errors: VecDeque<PgError>,
    /// 预设的查询结果队列，query 按序消费，空则返回零行。
    pub query_rows: VecDeque<Vec<Vec<DbValue>>>,
    /// execute 的固定影响行数。
    pub execute_affected: u64,
    in_tx: bool,
}

impl FakeConn {
    pub fn new() -> Self {
        Self::default()
    }

    /// 让后续某一次语句执行失败一次，可连续预置多条。
    /// 预置的错误只由业务语句消费：会话变量语句与事务边界语句
    /// （begin/commit/rollback）不吞错，保证 transact 四步的
    /// 前置动作在单测里总是成功。
    pub fn fail_next(&mut self, err: PgError) {
        self.errors.push_back(err);
    }

    pub fn push_rows(&mut self, rows: Vec<Vec<DbValue>>) {
        self.query_rows.push_back(rows);
    }

    fn take_error(&mut self) -> Result<(), PgError> {
        match self.errors.pop_front() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

#[async_trait::async_trait]
impl DbConn for FakeConn {
    async fn execute(&mut self, sql: &str, params: &[DbValue]) -> Result<u64, PgError> {
        if sql != SET_SESSION_VAR_STMT {
            self.take_error()?;
        }
        self.ops
            .push(FakeOp::Execute(sql.to_string(), params.to_vec()));
        Ok(self.execute_affected)
    }

    async fn query(&mut self, sql: &str, params: &[DbValue]) -> Result<Vec<Vec<DbValue>>, PgError> {
        self.take_error()?;
        self.ops
            .push(FakeOp::Query(sql.to_string(), params.to_vec()));
        Ok(self.query_rows.pop_front().unwrap_or_default())
    }

    async fn begin(&mut self, isolation: IsolationKind, read_only: bool) -> Result<(), PgError> {
        self.ops.push(FakeOp::Begin {
            isolation,
            read_only,
        });
        self.in_tx = true;
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), PgError> {
        self.ops.push(FakeOp::Commit);
        self.in_tx = false;
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), PgError> {
        self.ops.push(FakeOp::Rollback);
        self.in_tx = false;
        Ok(())
    }

    async fn in_transaction(&mut self) -> Result<bool, PgError> {
        Ok(self.in_tx)
    }
}
