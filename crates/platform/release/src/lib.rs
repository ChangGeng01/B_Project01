//! ep-platform-release — 配置发布包、差异审查、签名、发布与回退。
//!
//! 阶段 3a 交付内容项端口四件套（`port::config_item`）：
//! trait、`ItemKind` 十五项、`ConfigPackageItem` DTO 与注册表。
//! 本 crate 除 ep-foundation 外不依赖任何 crate（退出条件 29），
//! applier 实现体随属主模块阶段在两个 apps 的 wiring 注入。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节。骨架期不留 `todo!()`。

pub mod port;

pub use port::config_item::{
    ChangeKind, ConfigItemApplier, ConfigItemApplierRegistry, ConfigPackageItem, ItemKind,
};
