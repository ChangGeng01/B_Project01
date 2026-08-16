//! ep-platform-recon —— 内部对账与强制不变量校验。
//!
//! 归属阶段 9a（裁定 A-06），注册方固定为阶段 7、8、9b、11 四个。
//! 本轮交付其中**可以脱库判定的那一半**：
//! [`model`] 的值类型与豁免纪律、[`gate`] 的关账受理前提判定。
//!
//! 执行器（逐法人遍历、`snapshot_transact` 导出快照、逐批传递）、
//! 三张表的迁移、签名语句集校验，按 A-06 归阶段 9a 的落库交付，本 crate 不碰数据库。
//!
//! # 为什么先做关账前提这一块
//!
//! 规格第 10.2 章逐字「差异清零前不得关账」，而**关账是不可逆的**：
//! 已关闭期间不再接受任何凭证写入，首版又不做反结账。
//! 判松了会让一个账实不符的期间被永久关掉，判紧了只是关不成、运维来问一句。
//! **两种错的代价不对称**，所以这一块每一条判定都取保守那一侧，
//! 并在各自的文档里写明取的是哪一侧、为什么。
//!
//! 本轮**未交付** `ReconCheck`、`ReconRegistry`、`ReconExecutor` 三个契约 trait：
//! 它们的签名里有 `SnapshotCtx` 与 `AppError` 两个尚不存在的类型，
//! 现在写只能先造两个占位再回头改。留到那两个类型落地后一次做对——
//! 这里不留 `todo!()`，也不留占位类型。

pub mod gate;
pub mod model;

pub use gate::{check_close_admission, CloseBlocker, CloseFacts};
pub use model::{
    validate_waiver, BatchWindow, DiscrepancyState, ReconCategory, ReconDiscrepancy, ReconRunKind,
    ReconRunOutcome, ReconRunStatus,
};
