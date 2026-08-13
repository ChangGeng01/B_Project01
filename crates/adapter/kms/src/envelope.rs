//! EPC1 信封：单列 bytea 自描述密文格式（02 计划第 4.3 节逐字布局）。
//!
//! 字节布局：偏移 0 魔数 `EPC1`（4B）；偏移 4 算法标识（1B，0x01=AES-256-GCM，
//! 预留 0x02 起给商用密码档位）；偏移 5 `data_keys.id`（16B）；偏移 21 DEK 版本
//! （2B 大端 u16）；偏移 23 随机 nonce（12B）；偏移 35 密文加 16B 认证标签。
//!
//! AAD 三段拼接：16 字节法人标识大端 + UTF-8 `schema.table.column` + 16 字节行标识。
//! 行标识入 AAD 使密文不可跨行搬运，更正须重加密。

use ep_foundation::error::codes::{
    PLATFORM_CRYPTO_AAD_MISMATCH, PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID,
};
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::kms::Aad;
use ep_foundation::AppError;

/// 魔数四字节。
pub const MAGIC: [u8; 4] = *b"EPC1";
/// 算法标识：AES-256-GCM。0x02 起预留给商用密码档位，首版只认 0x01。
pub const ALGO_AES_256_GCM: u8 = 0x01;
/// 头部定长：魔数 4 + 算法 1 + DEK 标识 16 + 版本 2 + nonce 12。
pub const HEADER_LEN: usize = 35;
/// 最短信封：头部 35 + 认证标签 16（空明文仍加密，密文段只含标签）。
pub const MIN_ENVELOPE_LEN: usize = HEADER_LEN + 16;
/// 明文上限：超过 1MB 拒绝，走附件通道。
pub const PLAINTEXT_MAX: usize = 1024 * 1024;

/// 解出的信封头与密文段引用。
#[derive(Debug)]
pub struct ParsedEnvelope<'a> {
    pub dek_id: uuid::Uuid,
    pub dek_version: u16,
    pub nonce: &'a [u8],
    /// 密文加 16B 标签，交由 AEAD 一并校验。
    pub ciphertext: &'a [u8],
}

/// 拼装信封。调用方保证 `nonce` 恰为 12 字节、`ciphertext` 含标签。
pub fn assemble(dek_id: uuid::Uuid, dek_version: u16, nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    out.extend_from_slice(&MAGIC);
    out.push(ALGO_AES_256_GCM);
    out.extend_from_slice(dek_id.as_bytes());
    out.extend_from_slice(&dek_version.to_be_bytes());
    out.extend_from_slice(nonce);
    out.extend_from_slice(ciphertext);
    out
}

/// 校验魔数与长度并解出头部。五类非法（魔数错、长度不足、未知算法标识、
/// nonce 截断、标签截断）统一返 `PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID`。
pub fn parse(bytes: &[u8]) -> Result<ParsedEnvelope<'_>, AppError> {
    let invalid = |why: &str| {
        AppError::new(
            PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID,
            format!("EPC1 信封格式非法：{why}"),
        )
    };
    if bytes.len() < MIN_ENVELOPE_LEN {
        return Err(invalid(&format!(
            "长度 {} 不足最短信封 {MIN_ENVELOPE_LEN}",
            bytes.len()
        )));
    }
    if bytes[0..4] != MAGIC {
        return Err(invalid("魔数不是 EPC1"));
    }
    if bytes[4] != ALGO_AES_256_GCM {
        return Err(invalid(&format!("未知算法标识 0x{:02X}", bytes[4])));
    }
    let dek_id = uuid::Uuid::from_bytes(bytes[5..21].try_into().expect("区间定长 16"));
    let dek_version = u16::from_be_bytes(bytes[21..23].try_into().expect("区间定长 2"));
    Ok(ParsedEnvelope {
        dek_id,
        dek_version,
        nonce: &bytes[23..35],
        ciphertext: &bytes[35..],
    })
}

/// 按当前行重构 AAD：16 字节法人标识（UUID 字节即大端序）+
/// UTF-8 `schema.table.column` + 16 字节行标识。
pub fn aad_for_row(legal_entity_id: Id<LegalEntity>, column_fqn: &str, row_id: uuid::Uuid) -> Aad {
    let mut bytes = Vec::with_capacity(16 + column_fqn.len() + 16);
    bytes.extend_from_slice(legal_entity_id.as_uuid().as_bytes());
    bytes.extend_from_slice(column_fqn.as_bytes());
    bytes.extend_from_slice(row_id.as_bytes());
    Aad::new(bytes)
}

/// 标签校验失败的统一出口：AAD 由调用方按当前行重构，不一致即返
/// `PLATFORM.CRYPTO.AAD_MISMATCH`（BUSINESS_CONFLICT）。
pub fn aad_mismatch() -> AppError {
    AppError::new(
        PLATFORM_CRYPTO_AAD_MISMATCH,
        "认证标签校验失败：AAD 与当前行不符，密文不可跨行搬运",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_offsets_are_exact() {
        let dek_id = uuid::Uuid::from_u128(0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF);
        let nonce = [7u8; 12];
        let ct = vec![9u8; 20];
        let env = assemble(dek_id, 0x0102, &nonce, &ct);
        assert_eq!(&env[0..4], b"EPC1");
        assert_eq!(env[4], 0x01);
        assert_eq!(&env[5..21], dek_id.as_bytes());
        assert_eq!(&env[21..23], &[0x01, 0x02]);
        assert_eq!(&env[23..35], &nonce);
        assert_eq!(&env[35..], &ct[..]);
        assert_eq!(env.len(), HEADER_LEN + 20);
    }

    #[test]
    fn parse_roundtrip() {
        let dek_id = uuid::Uuid::from_u128(42);
        let env = assemble(dek_id, 3, &[1; 12], &[2; 32]);
        let parsed = parse(&env).expect("可解析");
        assert_eq!(parsed.dek_id, dek_id);
        assert_eq!(parsed.dek_version, 3);
        assert_eq!(parsed.nonce, &[1; 12]);
        assert_eq!(parsed.ciphertext.len(), 32);
    }

    #[test]
    fn parse_rejects_five_kinds_of_corruption() {
        let env = assemble(uuid::Uuid::nil(), 1, &[0; 12], &[5; 16]);
        // 一、长度不足。
        assert!(parse(&env[..MIN_ENVELOPE_LEN - 1]).is_err());
        // 二、魔数错。
        let mut bad = env.clone();
        bad[0] = b'X';
        assert!(parse(&bad).is_err());
        // 三、未知算法标识。
        let mut bad = env.clone();
        bad[4] = 0x02;
        assert!(parse(&bad).is_err());
        // 四、nonce 截断即整体长度不足。
        assert!(parse(&env[..34 + 16]).is_err());
        // 五、标签截断即整体长度不足。
        assert!(parse(&env[..HEADER_LEN + 15]).is_err());
        for e in [parse(&env[..10]), parse(&bad)] {
            let err = e.unwrap_err();
            assert_eq!(err.code, PLATFORM_CRYPTO_CIPHERTEXT_FORMAT_INVALID);
        }
    }

    #[test]
    fn aad_is_three_segments() {
        let le = Id::<LegalEntity>::from_uuid(uuid::Uuid::from_u128(0x0102));
        let row = uuid::Uuid::from_u128(0x0304);
        let aad = aad_for_row(le, "mdm.customers.phone_no", row);
        let b = aad.as_bytes();
        assert_eq!(&b[0..16], le.as_uuid().as_bytes());
        assert_eq!(&b[16..b.len() - 16], "mdm.customers.phone_no".as_bytes());
        assert_eq!(&b[b.len() - 16..], row.as_bytes());
    }
}
