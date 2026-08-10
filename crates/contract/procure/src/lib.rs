//! ep-contract-procure — 阶段 1 只建骨架，实质内容由后续阶段补齐。
//!
//! 职责：procure 模块对外公开的命令、查询、事件类型与 DTO，以及供其他模块调用的 trait。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节。骨架期不留 `todo!()`。
