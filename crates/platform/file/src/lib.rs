//! ep-platform-file —— 附件对象与本地文件存储。
//!
//! 职责（阶段 3 计划第 3.1 节交付物第 5 项）：附件对象与版本模型、分片上传与
//! 断点续传、类型识别与恶意内容检查、按法人密钥域与密级子域的信封加密落盘。
//!
//! 本轮交付其中**可以脱库、脱盘判定的那一半**：
//! [`upload`] 上传会话状态与分片校验、[`path`] 落盘路径的构造与越界防护、
//! [`scan`] 恶意内容检查结论的汇总。
//! 三段式落盘的事务编排、信封加密与解密、正文读写都在适配层与 core-server，
//! 本 crate 不碰数据库、不碰磁盘、也不依赖 KMS 适配器。
//!
//! # 本轮登记的一处发现
//!
//! [`scan`] 模块里 `SKIPPED` 被当成通过——**这是全卷「未覆盖不等于通过」纪律
//! 少见的一处反例，且它是规格自己定的**（检查器不可用不该把上传打停）。
//! 本实现照办，但把跳过的检查器名字带出去，让「跳过」在库里留痕：
//! 一份从未被扫过的附件与一份扫过并通过的附件，在证据上不是同一回事。
//! **带名字这一条是本实现自加的，计划没写。** 详见该模块文档。

pub mod path;
pub mod scan;
pub mod upload;

pub use path::{storage_path, PathSegment, SegmentError, MAX_SEGMENT_LEN};
pub use scan::{summarize, CheckVerdict, ScanOutcome};
pub use upload::{validate_part, verify_assembled, PartError, UploadState};
