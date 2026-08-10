//! 密级。
//!
//! 待裁定：技术基线第 1.4 节把 `clearance_level: SecurityLevel` 列为
//! 安全上下文第 7 字段，但没有冻结 `SecurityLevel` 自身的取值集合。
//! 本文件按规格的四级密级取值实现，若后续裁定给出第二套取值，改这里一处。

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
         serde::Serialize, serde::Deserialize)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Secret,
}
