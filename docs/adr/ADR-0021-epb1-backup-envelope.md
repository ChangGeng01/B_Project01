# ADR-0021 EPB1 备份信封

- 状态：已接受
- 出处：F-57 勒索恢复、服务器外备份与洁净恢复契约
- 关系：ADR-0014 的 `EPC1` 继续只覆盖 `FIELD|ATTACHMENT|ARCHIVE`；备份使用本 ADR 的独立 `EPB1`

## 背景

`EPC1` 的 purpose/scope、exact data-key ref 与 AAD 构造器面向在线字段、附件和归档对象。备份集合还需要绑定发布/配置代、不可变传输对象、分块顺序和集合总长度，并且必须让日常 writer 与备份目标无法解密历史。把备份塞入 `EPC1` 会错误扩大 ADR-0014 的闭集并把在线数据密钥域带入恢复边界。

## 决定

### 一、用途与 key domain

`EPC1` 继续封闭为 `FIELD|ATTACHMENT|ARCHIVE`。数据库 base backup、WAL、附件密文副本、配置、包和证据的备份传输统一使用独立 `EPB1` AES-256-GCM 信封；二者不得互相解析、改标或回退。

每个 backup set 生成一把 32-byte CSPRNG `backup_dek`，只用于该 set，`backup_dek_version=1`。它只经 [ADR-0024](ADR-0024-f57-backup-key-envelope.md) 包裹给独立 backup recovery domain，绝不包裹给 writer、生产服务、target 或应用 operational recipient；不同 set 不复用 DEK。`epb1_envelope_version=1` 与 ADR-0024 payload 逐字相等。

### 二、`EPB1RecordV1` exact binary wire

每个 chunk 是一个独立、不可变 binary record。多字节整数一律 unsigned big-endian，UUID 一律 RFC 9562 canonical UUID 的 16 raw network-order bytes；不得使用 struct memory layout、platform endian、varint、CBOR、JSON header、padding 或尾随 bytes。固定 header 恰为 136 bytes：

| offset | length | exact value / type |
|---:|---:|---|
| 0 | 4 | ASCII `EPB1` |
| 4 | 1 | envelope version `0x01` |
| 5 | 1 | algorithm `0x01` = AES-256-GCM |
| 6 | 2 | flags/reserved `u16be=0` |
| 8 | 16 | `deployment_id` |
| 24 | 16 | `backup_set_id` |
| 40 | 16 | immutable `object_id` |
| 56 | 16 | `backup_dek_id` |
| 72 | 4 | `backup_dek_version u32be=1` |
| 76 | 8 | `release_generation u64be`，`1..=i64::MAX` |
| 84 | 8 | `config_generation u64be`，`1..=i64::MAX` |
| 92 | 8 | set-global `chunk_ordinal u64be` |
| 100 | 4 | object-local zero-based `chunk_no u32be` |
| 104 | 4 | `chunk_count u32be` |
| 108 | 8 | `total_plaintext_len u64be` |
| 116 | 4 | `chunk_plaintext_len u32be` |
| 120 | 12 | AES-GCM nonce |
| 132 | 4 | `ciphertext_len u32be`，必须等于 `chunk_plaintext_len` |
| 136 | `ciphertext_len` | ciphertext |
| `136+ciphertext_len` | 16 | GCM tag |

record total length 必须恰为 `152+ciphertext_len`；截断或尾随一字节都拒绝。version、algorithm、reserved、DEK version 未逐字命中即失败，不尝试其他 parser/algorithm。header 在任何内存分配前完成有界解析，record 最大 `8,388,760` bytes（136 + 8 MiB + 16）。

### 三、对象分块、编号与 nonce

v1 固定 chunk plaintext ceiling 为 `8,388,608` bytes。`total_plaintext_len` 允许 `0..=281,474,976,710,655`；`chunk_count=max(1,ceil(total_plaintext_len/8,388,608))` 且不超过 `33,554,432`。`chunk_no` 必须是 `0..chunk_count-1`：每个非末 chunk 长度恰为 8 MiB；末 chunk 长度恰为 `total_plaintext_len - 8,388,608*(chunk_count-1)`；空对象恰为一个 `chunk_no=0`、零长度 ciphertext 加有效 tag 的 record。对象内不允许空洞、重复、乱序、变长非末块或第二种分块尺寸。

每 set 创建一次 4-byte random `nonce_prefix` 并写入下节签名 cipher graph。record nonce 恰为 `nonce_prefix || u64be(chunk_ordinal)`。`chunk_ordinal` 在整个 set 内从 0 严格连续分配且每个 record 唯一；持久 allocation journal 在写 record 前以 CAS 预留 ordinal，重试同一 ordinal 只能提交 byte-identical record，不能把它分配给另一对象/chunk。达到 `u64::MAX`、journal 回滚/分叉、prefix 变化或任何 nonce 重用都永久关闭该 set，不生成可发布 checkpoint。不同 set 使用新 DEK，因此 prefix 碰撞不能导致同 key nonce 重用。

### 四、AAD 与加解密

AES-256-GCM AAD bytes 恰为 `ASCII("EP-F57-ADR0021-EPB1-AAD-V1\0") || header[0..136]`。header 包含 nonce 和全部长度/身份/代际字段；调用者不能供应任意 AAD。ciphertext 是同一 chunk plaintext 的等长输出，tag 固定 16 bytes 并与 record 分栏保存。任何 header 位、deployment/set/object/DEK/generation/ordinal/chunk/length/nonce、ciphertext 或 tag 改动都返回统一认证失败，不返回部分 plaintext，也不回退其他 key/version。

恢复时先验证签名 graph/checkpoint、ADR-0024 key envelope 与 target receipt，再按 manifest exact record digest 打开 record；完成结构/bounds/AAD/tag 验证后才释放该 chunk 到隔离恢复 workspace。全部 object chunks 成功且累计长度 exact-match 后才能原子发布对象；一个失败不得留下“部分已恢复”业务对象。

### 五、签名 `Epb1CipherGraphV1`

backup set 的签名 manifest/checkpoint 必须包含 strict RFC 8785 JCS `Epb1CipherGraphV1`，root exact fields 为：

`schema_version=1,purpose="EP-F57-EPB1-CIPHER-GRAPH-V1",deployment_id,backup_set_id,backup_dek_id,backup_dek_version=1,backup_key_envelope_digest,epb1_envelope_version=1,release_generation,config_generation,chunk_size_bytes=8388608,nonce_prefix_b64url,objects,total_record_count,total_ciphertext_bytes,created_at`。

`nonce_prefix_b64url` 是 4 raw bytes 的 canonical unpadded base64url。`objects` 非空，按 `(object_kind wire bytes,logical_sequence,object_id bytes)` 排序唯一；item exact fields 是 `{object_id,object_kind,logical_sequence,total_plaintext_len,chunk_count,first_chunk_ordinal,last_chunk_ordinal,chunks}`。`object_kind` 闭集为 `POSTGRES_BASE|POSTGRES_WAL|ATTACHMENT_CIPHERTEXT|CONFIG_BUNDLE|EVIDENCE_BUNDLE|PACKAGE_BUNDLE|VAULT_BUNDLE`。`chunks` 按 chunk_no 排序，每项 exact fields 是 `{chunk_no,chunk_ordinal,record_object_key,record_len,record_sha256,target_receipt_digest}`；`record_object_key` grammar 为 `epb1/v1/<64-lowerhex>` 且末段逐字等于去掉 `sha256:` 前缀的 `record_sha256`。first/last ordinal、对象/全局计数、record/ciphertext bytes 总和必须重算相等，所有 ordinal 全局 exact `0..total_record_count-1`。

graph 不保存业务名称、源路径、客户正文或明文摘要。公开 receipt 只含 opaque object key、record length/digest、target identity 和 conditional-create result。graph、ADR-0024 envelope digest、PostgreSQL backup/WAL span、附件 recovery cut、target receipts、release/config generation 和服务器外 checkpoint 在同一外层签名 evidence 中交叉绑定；任何一侧缺失、重复或不等均不可恢复/认证。

### 六、权限与 KAT

只有独立 recovery 身份在受控恢复流程中可以取得 backup DEK 并解密。writer 只能创建当前 set record、上传并用一次性 exact-object handle readback 刚写 ciphertext；target 只能 conditional-create/retention/readback，不得解密；两者不能枚举历史或取得 recovery API/key。业务服务不能解析 EPB1。

Task 24 必须提交 `crates/platform/backup/tests/fixtures/adr0021-epb1-v1.json`：固定 DEK、IDs、generations、nonce prefix 和 ordinal，覆盖空对象、1 byte、恰 8 MiB、8 MiB+1 的 deterministic plaintext recipe，记录每个 exact header/AAD/ciphertext/tag/record SHA-256/cipher graph JCS digest；另含 every-header-field mutation、little-endian、reserved bit、version/algorithm、record truncation/trailing byte、0/overflow generation、length allocation bomb、chunk gap/duplicate/reorder、ordinal/prefix/nonce reuse、wrong graph/receipt/key-envelope digest 和 tag mutation negatives。writer 与 recovery-tool、旧/新 release 读取同一 v1 wire、三种 ADR-0024 two-token 组合和真实服务器外 target capture 必须对同一 vector 逐字通过；生产永不使用 fixture key/RNG。

## 理由

独立 `EPB1` 使备份认证语义与在线对象信封分离，也使 backup-specific DEK 只存在于恢复域。若复用 `EPC1` 或日常 writer 的密钥，攻破生产写入身份可能同时获得历史解密能力，且在线 AAD 无法完整证明备份集合、代和分块边界。

## 后果

正面：备份目标只接触不可识别的密文对象；跨集合、跨对象、跨代、乱序、重复和长度漂移都在认证/清单核对处失败；恢复密钥与生产身份分域。

代价：备份实现必须维护独立 wire parser、nonce 计数、每集合密钥生命周期、签名 ciphertext manifest/checkpoint 和恢复 conformance fixtures。

## 影响范围

- backup platform/adapter 的 envelope、crypto、manifest、checkpoint 与 restore 模块；
- backup writer、服务器外 target、离线介质与恢复身份；
- base backup、WAL、附件密文、配置/证据的分块传输与签名收据；
- 威胁模型、Windows/P340 恢复门与备份 conformance 测试。
