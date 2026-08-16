//! ep-platform-flow —— 持久化工作流引擎。
//!
//! 职责（阶段 3 计划第 3.4.8 节）：流程定义版本、实例、步骤、人工任务、
//! 定时器、SLA、补偿、运行约束、版本迁移与模拟。
//!
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`state`] 实例状态机与其守卫、[`compensation`] 补偿的选取与次序、
//! [`step`] 步骤幂等键与运行约束。
//! 调度取件、行锁、一步一事务的落库、定时器扫描都在适配层与 job-worker，
//! 本 crate 不碰数据库。
//!
//! 为什么这三块值得先做：它们是**错了不会当场报错**的地方。
//! 状态机少一条守卫，非法迁移会照常提交；补偿次序反了，撤销会先拆依赖再拆前提；
//! 幂等键形状变一点，重放就会把副作用做第二遍。三者都要到出事那天才显形，
//! 而那时现场早已不在。脱库可测，就能在写完当天把它们测穿。
//!
//! 守卫条件表达式的最小求值器由 [`expr`] 交付（上一轮留的那一件）：
//! 字段引用、六种比较、与或非、集合成员、空判定，加一个不超过 12 个函数的白名单。
//! 它的三处要害都在值语义而不在解析——数字不用 `f64`、空一律报错、
//! 深度闸门在解析期而不靠求值步数——逐条理由见该模块的文档。

pub mod compensation;
pub mod expr;
pub mod state;
pub mod step;

pub use compensation::{
    judge as judge_compensation, plan as plan_compensation, CompensationStep, StepOutcome,
    StepRecord,
};
pub use expr::{evaluate, EvalCtx, Guard, GuardError, GuardValue, VarLookup, DEFAULT_MAX_STEPS};
pub use state::{allowed_transitions, transition, Guards, InstanceState, TransitionError, Trigger};
pub use step::{check_limits, execution_no, idempotency_key, LimitBreach, RunFacts, RunLimits};
