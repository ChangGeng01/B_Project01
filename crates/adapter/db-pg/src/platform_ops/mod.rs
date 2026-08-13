//! `platform_ops` schema 的 SQL 实现体（db-pg-one-schema-per-file）。
//!
//! 本阶段只交付降级窗口台账的存取层 [`degradation::PgDegradationLedger`]。
//! 台账端口 `DegradationLedger` 在 ep-platform-obs；本 crate 直接实现该端口
//! （adapter 允许依赖 platform 端口，tenancy 先例），`ep_degradation_windows_open`
//! gauge 的刷新随台账读写一并落在实现体内（IT-41）。

pub mod degradation;

pub use degradation::PgDegradationLedger;
