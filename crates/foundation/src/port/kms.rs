//! 密钥管理端口。
//!
//! 阶段 1 只建空文件。按裁定 F-04，`KmsBackend` 与其调用词汇由阶段 2 补齐，
//! 六个方法为 `wrap`、`unwrap`、`derive_blind_key`、`sign`、`verify`、`health`；
//! 内置 KMS 与客户 HSM 两种载体的实现落在 ep-adapter-kms，本模块不声明任何载体类型。
//!
//! 端口下沉本模块的理由：ep-platform-release 与 ep-platform-audit 需要命名该 trait，
//! 而基线第 1.3 节允许项只准 platform 依赖 foundation 与其他 platform；
//! 端口停在 ep-adapter-kms 会构成 platform 反向依赖 adapter。
//!
//! `derive_blind_key` 的返回宽度不随本批冻结，只冻结三参数形态：
//! 阶段 2 计划第 4.4 节定 `BlindIndex([u8; 16])`，而同文件第 11 节假设三要求
//! `finance.cash_accounts` 取完整 32 字节，两处未合一之前不得写死。
