# ADR-0014 EPC1 自描述信封与 purpose/scope 约束的 AAD 构造器

- 状态：已接受并冻结为首版终态
- 出处：阶段 2 §4.1、§4.3；阶段 3 附件分块与 Stage 14 归档补强
- 范围说明：`EPC1` 只覆盖 `FIELD|ATTACHMENT|ARCHIVE`；备份加密不属于 EPC1，现行备份信封使用 [ADR-0021](ADR-0021-epb1-backup-envelope.md) 的独立 `EPB1`

## 背景

字段、附件和归档密文必须能定位其 exact DEK，同时抵抗跨法人、跨对象、跨版本、跨分块搬运。只有“法人+字段+行”一种 AAD 会迫使附件/归档自行拼字节或复用错误 purpose；若调用方可以传任意 AAD bytes 或任意 purpose/scope，adapter 无法证明所选密钥与认证上下文属于同一安全子域。

## 决定

一、每块 AES-256-GCM 密文使用 `EPC1`：偏移 0 为魔数 `EPC1`（4B）；偏移 4 为算法 `0x01`（1B）；偏移 5 为 `data_keys.id`（16B）；偏移 21 为 DEK version（2B，大端非零 u16）；偏移 23 为随机 nonce（12B）；偏移 35 为 ciphertext 加 16B tag。`data_keys.version`、供给/readback DTO、`DataKeyRefV1` 与本 header 共用 `1..65535` exact 域；任何大值截断均拒绝。其他算法标识未裁定，不得因“预留”自行启用。

二、`Aad` 是字段私有、不可从任意 bytes/purpose/scope 构造的 strong value，内部固定 `{purpose,security_level_scope,authenticated_bytes}`；scope 只接受 `10|20|30|40`。adapter 必须同时核对 `KeyPurpose`、handle/envelope 所指 data key 与 AAD 私有 purpose/scope，不能只验 tag。

三、构造器闭集只有三类：

- `Aad::for_field(legal_entity_id,column_fqn,row_id,effective_level)`：authenticated bytes 为 16-byte 法人 UUID、UTF-8 `schema.table.column`、16-byte row UUID 三段拼接；metadata 固定 purpose=`FIELD`。
- `Aad::for_attachment_chunk(legal_entity_id,level,attachment_object_id,attachment_version_id,total_plaintext_len,chunk_no)`：`ASCII("EP-ATTACHMENT-CHUNK-AAD-V1\0") || legal[16] || u16be(level) || object[16] || version[16] || u64be(total_plaintext_len) || u32be(chunk_no)`；metadata 固定 purpose=`ATTACHMENT`。
- `Aad::for_archive_chunk(legal_entity_id,level,archive_object_id,total_plaintext_len,chunk_no)`：`ASCII("EP-ARCHIVE-CHUNK-AAD-V1\0") || legal[16] || u16be(level) || archive[16] || u64be(total_plaintext_len) || u32be(chunk_no)`；metadata 固定 purpose=`ARCHIVE`。

盲索引不使用 `Aad`，只经 scoped selector 的 `derive_blind_key`。三类构造器不得互换，也不得新增 caller-chosen purpose。

四、字段写入经六方法 `KmsBackend::wrap` 选择 current `(FIELD,scope)`；字段读取经 `unwrap` 按 EPC1 id/version 定位 exact key。附件和归档每个对象开始时只经 `KmsPinnedDataKeyBackend::CurrentForWrite` 取得一次不可伪造 handle，持久化其 `DataKeyRefV1`，全部 chunks 用同一 handle；历史读/Range/续传只以 `ExactRef` 重开，绝不漂移到 current。id/version、purpose/scope、AAD 与对象 projection 任一不等都失败关闭。

五、标签失败且法人/scope/AAD 不等统一按已冻结的 `PLATFORM.CRYPTO.AAD_MISMATCH`；exact key 已销毁、缺失或不可 readback 按相应 `DECRYPT_FAILED`/`PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE` 契约处理，不向调用方泄漏密钥材料或 provider 细节。

六、96-bit 随机 nonce 的单 key 上限沿用 NIST SP 800-38D。首版容量估算为每日 10 万次、单版本五年约 `1.8×10^7≈2^24` 次，低于 `2^32` 调用上限；轮换启用新 DEK/version 后重新计数。真实量增长两个数量级前必须收紧轮换周期并重做风险评估。

## 理由

EPC1 内嵌 id/version 避免第二份密文映射台账；强类型 AAD 把密文与法人、用途、密级及具体字段/对象/版本/chunk 在密码学上绑定。pinned handle 进一步保证长文件在并发轮换期间不会混用版本。

## 后果

正面：跨法人、跨字段、跨对象、跨版本与跨 chunk 搬运均在认证或元数据核对处失败；轮换只影响新对象，旧对象仍可由 exact ref 解密。

代价：密文不能直接跨行/对象复制，更正、合并、迁移与重分块必须解密后在新 AAD 下重新加密；每块固定 35-byte header 加 16-byte tag；版本达到 65535 前必须另立带新 wire/迁移的 ADR，不得溢出或回绕。

## 影响范围

- `ep_foundation::port::kms::{Aad,CipherEnvelope,DataKeyRefV1,DataKeySelectorV1}`；
- `crates/adapter/kms` 的 EPC1 编解码与 builtin/HSM adapter；
- 字段加密、Stage 3 `EPA1` 附件、Stage 14 archive chunk；
- `docs/data-dictionary.md`、迁移目录与跨卷 conformance fixtures。
