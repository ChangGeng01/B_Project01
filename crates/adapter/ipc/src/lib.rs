//! ep-adapter-ipc — 阶段 1 只建骨架，实质内容由后续阶段补齐。
//!
//! 职责：进程间接口的客户端与服务端，Unix domain socket 承载。
//!
//! 编译期断言：本 crate 的依赖方向由 `xtask archcheck` 逐条断言，
//! 允许的上游见技术基线第 1.3 节。骨架期不留 `todo!()`。
