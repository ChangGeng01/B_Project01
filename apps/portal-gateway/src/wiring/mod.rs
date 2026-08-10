//! portal-gateway 的装配。
//!
//! 本目录**不在** `xtask archcheck` 的 unwired-absent 断言面内：
//! 该规则按阶段 1 退出条件 26 只断言 `apps/core-server/src/wiring/` 与
//! `apps/job-worker/src/wiring/` 两个目录，见 xtask/src/archcheck/source.rs 的 WIRING_DIRS。
//! 本目录仍应遵守同一纪律，不出现 Noop、Stub、Fake、Dummy 四类前缀的实现类型或注入行。
