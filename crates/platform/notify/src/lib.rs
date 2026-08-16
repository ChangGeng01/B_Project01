//! ep-platform-notify —— 站内通知与移动推送出口。
//!
//! 职责（阶段 3 计划第 3.1 节交付物第 6、7 项）：通知聚合、模板渲染、
//! 接收人解析端口、推送载荷组装与脱敏、送达状态。
//!
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`fanout`] 扇出规模与未读上限、[`template`] 模板渲染与变量白名单、
//! [`push`] 推送失败的处置纪律。落库、接收人解析端口、载荷组装与令牌解封
//! 都在适配层与 job-worker，本 crate 不碰数据库、也不依赖 KMS 适配器。
//!
//! # 三条纪律，各配了守它的用例
//!
//! 一、**站内通知在业务事务内同步写入，不经 Outbox**。规格第 5.1 章把它定为
//! 首版**唯一验收不可豁免的通知渠道**，挂在至少一次投递的异步链路上会引入
//! 一个本可避免的丢失面。
//!
//! 二、**未读满了也要写**。用容量上限去丢业务提醒是不行的——见 [`fanout::judge_unread_cap`]。
//!
//! 三、**推送失败一律不冒泡**。推送是可选增强不是保证渠道，冒泡会让一个已经
//! 成功的业务动作因为推送不通而报错，而站内通知早已写好——见 [`push::judge_failure`]。

pub mod fanout;
pub mod push;
pub mod template;

pub use fanout::{
    judge_unread_cap, plan_fanout, FanoutPlan, UnreadCapOutcome, DEFAULT_SYNC_FANOUT_MAX,
    DEFAULT_UNREAD_CAP_PER_USER,
};
pub use push::{
    business_reminder_interrupted_when_push_unavailable, judge_failure, should_push,
    DeliveryStatus, PushFailureOutcome,
};
pub use template::{dedupe_key, render, RenderError, REDACTED_PLACEHOLDER};
