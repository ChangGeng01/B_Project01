//! 假连接。纯逻辑单测的驱动替身：记录全部操作序列，可预置错误与查询结果。
//!
//! 本模块公开导出，供本 crate 单测与后续装配侧测试共用；
//! 它不触碰任何真实连接，也不引入数据库专有行为，
//! `begin`/`commit`/`rollback` 只改自身的 `in_tx` 标志。

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

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
    /// 下一次 commit/rollback 的独立故障；事务边界不与业务语句共用
    /// `errors` 队列，避免会话变量语句或快照查询提前吞掉故障。
    commit_error: Option<PgError>,
    rollback_error: Option<PgError>,
    /// 第几次（1 起）session-variable 写入失败一次，覆盖 apply 与 clear。
    session_error: Option<(usize, PgError)>,
    session_execute_count: usize,
    /// 被丢弃的连接也可由测试观察其完整边界操作顺序。
    op_observer: Option<Arc<Mutex<Vec<FakeOp>>>>,
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

    pub fn fail_commit(&mut self, err: PgError) {
        self.commit_error = Some(err);
    }

    pub fn fail_rollback(&mut self, err: PgError) {
        self.rollback_error = Some(err);
    }

    pub fn fail_session_execute(&mut self, call_number: usize, err: PgError) {
        assert!(call_number > 0, "session 写入序号从 1 开始");
        self.session_error = Some((call_number, err));
    }

    pub fn observe_ops(&mut self, observer: Arc<Mutex<Vec<FakeOp>>>) {
        self.op_observer = Some(observer);
    }

    fn record(&mut self, op: FakeOp) {
        if let Some(observer) = &self.op_observer {
            observer
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push(op.clone());
        }
        self.ops.push(op);
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
        if sql == SET_SESSION_VAR_STMT {
            self.session_execute_count += 1;
            self.record(FakeOp::Execute(sql.to_string(), params.to_vec()));
            if self
                .session_error
                .as_ref()
                .is_some_and(|(call_number, _)| *call_number == self.session_execute_count)
            {
                let (_, error) = self.session_error.take().expect("刚刚确认存在故障");
                return Err(error);
            }
        } else {
            self.take_error()?;
            self.record(FakeOp::Execute(sql.to_string(), params.to_vec()));
        }
        Ok(self.execute_affected)
    }

    async fn query(&mut self, sql: &str, params: &[DbValue]) -> Result<Vec<Vec<DbValue>>, PgError> {
        self.take_error()?;
        self.record(FakeOp::Query(sql.to_string(), params.to_vec()));
        Ok(self.query_rows.pop_front().unwrap_or_default())
    }

    async fn begin(&mut self, isolation: IsolationKind, read_only: bool) -> Result<(), PgError> {
        self.record(FakeOp::Begin {
            isolation,
            read_only,
        });
        self.in_tx = true;
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), PgError> {
        self.record(FakeOp::Commit);
        if let Some(error) = self.commit_error.take() {
            return Err(error);
        }
        self.in_tx = false;
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), PgError> {
        self.record(FakeOp::Rollback);
        if let Some(error) = self.rollback_error.take() {
            return Err(error);
        }
        self.in_tx = false;
        Ok(())
    }

    async fn in_transaction(&mut self) -> Result<bool, PgError> {
        Ok(self.in_tx)
    }
}
