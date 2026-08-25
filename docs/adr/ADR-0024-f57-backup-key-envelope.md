# ADR-0024 F-57 BackupKeyEnvelopeV1

- 状态：已接受
- 出处：F-57 `EPB1` 每备份集密钥、独立恢复域、PIV 2-of-3 与洁净主机恢复契约
- 关系：补全 ADR-0021 的 backup-specific DEK recovery envelope；与 ADR-0020 的在线数据双 recipient 信封分域，不复用其 operational recipient、token set 或 custodian set

## 背景

ADR-0021 已规定每个 backup set 使用独立 DEK，且该 DEK 只包裹给独立 backup recovery domain；`EPB1` 精确定义备份 chunk 的 AES-256-GCM 和 AAD。它没有定义 backup DEK 如何进入恢复域、三个 PIV share 如何编码、哪个算法可以启用，以及 token 丢失/轮换后如何在洁净主机继续恢复。

如果只冻结 `EPB1` chunk 而不冻结 key envelope，不同实现可以生成都能写入、却无法互相恢复的备份；recipient、share 或 generation 也可能被跨 backup set 拼接。备份恢复材料必须与生产 operational key、应用 vault、BitLocker 和 backup writer 分域，并且厂商不能持有恢复能力。

## 决定

### 一、每 backup set 独立两把临时密钥

创建一个 backup set 时，CSPRNG 分别生成：

1. 256-bit `backup_dek`：只用于该 set 的全部 `EPB1` chunk；不同 set 永不复用。
2. 256-bit `set_recovery_kek`：只用于包裹该 `backup_dek`，随后按 `PIV_SHAMIR_2_OF_3_V1` 分成三份；不同 set 永不复用。

writer 在锁页/zeroizing memory 中完成 EPB1 加密、DEK 包裹和 share 生成；写出完整、已签名且可 readback 验证的 envelope/checkpoint 后立即清零 `backup_dek`、`set_recovery_kek`、Shamir coefficient 和 plaintext shares。writer、target、生产服务和厂商均不得持有任一 PIV private key、恢复 PIN 或可枚举历史的 recovery API。writer 只读取当前签名 recovery-domain manifest 中三个 token 的 public recipient material。

这里的 recovery-domain manifest 严格复用 ADR-0020 §八点一的 shared descriptor，但签名外层唯一为 `SignedBusinessArtifactV1<BackupRecoveryDomainManifestPayloadV1>`，并固定 `domain_kind=BACKUP`、编译期/embedded `purpose="EP-F57-BACKUP-RECOVERY-DOMAIN-MANIFEST-V1"`、独立 backup-recovery-domain 签名 roster 与 `<ValidatedDataRoot>/RecoveryDomains/Backup/{current,history}` locator。其 exact 三个 recipient descriptor、algorithm、有效期、predecessor、recipient-set digest 与 revocation checkpoint 是本 ADR 所有 `recipient_set_digest`/PIV public key 的唯一权威；`RecoveryDomainSeparationEvidenceV1` 只证明域间不复用，不能替代或改写它。writer 必须锁定已验签 payload bytes/digest 后再加密，不能在过程中重新查询“当前”名册；Task 24 负责创建/轮换 BACKUP current/history，Task 2 的 APPLICATION signer、token、custodian 或 manifest 均不得授权本域。

### 二、canonical encoding 与签名

`BackupKeyEnvelopeV1` 使用严格 JSON：拒绝重复 key、未知字段、非 UTF-8、非 canonical UUID/时间/digest、无 padding base64url 之外的二进制。payload 采用 RFC 8785 JCS，digest 为 `SHA-256(JCS(payload))`，并复用 `SignedBusinessArtifactV1<BackupKeyEnvelopeV1>` 的 detached CMS、purpose separation、离线 chain/full-CRL 和闭合签名 profile。

签名由 daily writer 无法调用的 `BackupCheckpointSignerPort` 产生。签名检查点同时绑定 backup manifest、EPB1 ciphertext graph、target receipts、`BackupKeyEnvelopeV1` digest、release/config generation 和 recovery-domain manifest digest。签名/digest 位于外层，不能进入自身 payload。

UUID 使用小写连字符格式，digest 使用小写 `sha256:`，时间使用 UTC RFC 3339；`encrypted_shares` 固定按 `share_index` 1、2、3 排序。任何 unknown algorithm/version/field 均失败关闭，不存在兼容猜测或 provider-private locator。

### 三、`BackupKeyEnvelopeV1` exact schema

payload 只能包含：

| 字段 | 类型与约束 |
|---|---|
| `schema_version` | 常量 `1` |
| `purpose` | 常量 `EP-F57-BACKUP-KEY-ENVELOPE-V1`；CMS `signingTime` 必须逐字等于 `created_at` 的同一 UTC whole-second |
| `envelope_id` | UUID |
| `deployment_id` | UUID |
| `backup_set_id` | UUID；与 EPB1 set/manifest 精确相同 |
| `backup_dek_id` | UUID；只属于该 set |
| `backup_dek_version` | 常量 `1`；未来轮换只增不降 |
| `epb1_envelope_version` | 当前 ADR-0021 认证版本 |
| `release_generation` | 正整数 |
| `config_generation` | 正整数 |
| `wrap_generation` | 正整数；同一 set 的 rewrap/recipient rotation 只增不降 |
| `recovery_domain_id` | UUID；独立于应用 vault/BitLocker/operational domain |
| `recovery_domain_manifest_digest` | 三 token/custodian active manifest 的 `sha256:` digest |
| `recovery_policy` | 常量 `PIV_SHAMIR_2_OF_3_V1` |
| `backup_dek_wrap_algorithm` | 常量 `AES_256_GCM_V1` |
| `shamir_algorithm` | 常量 `SHAMIR_GF256_VSSS_RS_V1` |
| `piv_share_wrap_algorithm` | 本 ADR 第五节闭集之一；同一 envelope 三 share 必须相同 |
| `recipient_set_digest` | 三个 recipient descriptor canonical list 的 `sha256:` digest |
| `backup_dek_wrap_nonce` | 12 bytes base64url；该 KEK 下唯一 |
| `backup_dek_wrapped_key` | AES-GCM ciphertext+16-byte tag，base64url，解密后恰为 32 bytes |
| `encrypted_shares` | 恰好三个 `EncryptedRecoveryShareV1`，index exact-set `{1,2,3}` |
| `created_at` | `TrustedUtc` |
| `previous_envelope_digest` | 首代为 JSON `null`；轮换时为前一 signed envelope digest |

数据库/文件持久化不得拆成可被部分更新的独立真相。envelope 是不可变对象；同一 `(deployment_id, backup_set_id, wrap_generation)` 唯一。新 generation 只能追加，不能覆盖旧 envelope。

### 四、AAD 与 share plaintext 的唯一构造

`BackupDekWrapAadV1` 是下列字段的严格 JCS object：

```text
schema_version = 1
purpose = "F57_BACKUP_DEK_WRAP"
deployment_id
backup_set_id
backup_dek_id
backup_dek_version
epb1_envelope_version
release_generation
config_generation
wrap_generation
recovery_domain_id
recovery_domain_manifest_digest
recovery_policy
backup_dek_wrap_algorithm
shamir_algorithm
piv_share_wrap_algorithm
recipient_set_digest
```

`backup_dek` 使用 `set_recovery_kek`、`backup_dek_wrap_nonce` 和该 AAD 执行 AES-256-GCM。任一字段变化、跨 set/deployment/generation/recipient-set 替换或 tag 失败均不得返回 plaintext。

每个 Shamir plaintext share 使用固定 69-byte `RecoverySharePlaintextV1` binary：

```text
offset 0..4   ASCII "BKS1"
offset 4..36  SHA-256(JCS(BackupDekWrapAadV1))
offset 36     share_index，exact 1|2|3
offset 37..69 32-byte Shamir share value
```

recipient descriptor 精确为 `{share_index, custodian_id, token_id, piv_slot, recipient_spki_sha256, recipient_key_version, piv_share_wrap_algorithm}`。三个 descriptor 必须有不同 custodian、token、SPKI 和 index，按 index 排序后 JCS/hash 得到 `recipient_set_digest`。share ciphertext 必须绑定自身 descriptor 和整个 set digest，不能把三个合法 envelope 的 share 混拼。

### 五、闭合算法枚举

当前只接受以下算法标识；不得使用自由字符串、PKCS#1 v1.5、AES-CBC、静态 IV、软件 token private key 或自动降级：

- digest：`SHA_256_V1`；
- backup DEK wrap：`AES_256_GCM_V1`，96-bit nonce、128-bit tag；
- recovery policy：`PIV_SHAMIR_2_OF_3_V1`；
- Shamir：`SHAMIR_GF256_VSSS_RS_V1` 唯一实现固定为 `vsss-rs = 5.4.0`，`default-features=false, features=["std","primitive","zeroize"]`，Cargo.lock checksum 与 SBOM 必须命中；使用该版本 constant-time `Gf256` byte-sequence Shamir、`SequentialParticipantNumberGenerator` 从 1 步进 1，share index exact `{1,2,3}`、threshold exact `2`、secret exact 32 bytes。禁止 6.x/pre-release、floating range、git source 或第二套 Shamir 实现；
- PIV share wrap 二选一：
  - `PIV_RSA2048_OAEP_SHA256_MGF1_SHA256_SLOT_9D_V1`；OAEP label 为 `SHA-256(JCS(ShareWrapBindingV1))`；
  - `PIV_ECDH_P256_HKDF_SHA256_AES256_GCM_SLOT_9D_V1`；ephemeral key 为 canonical uncompressed SEC1 P-256，HKDF salt 32 random bytes，info 为 `SHA-256(JCS(ShareWrapBindingV1))`，AES-GCM nonce 12 random bytes、tag 16 bytes。

`ShareWrapBindingV1` 精确包含 `deployment_id`、`backup_set_id`、`backup_dek_id/version`、`wrap_generation`、`recovery_domain_id`、`recovery_domain_manifest_digest`、`recipient_set_digest` 和完整 recipient descriptor。RSA 变体的 `EncryptedRecoveryShareV1` 只能含 descriptor、`ciphertext`；ECDH 变体只能含 descriptor、`ephemeral_public_key`、`hkdf_salt`、`nonce`、`ciphertext_and_tag`。variant 外字段和三 share 混用算法均拒绝。

激活某一算法前，三枚实际 token、Windows PC/SC middleware、签名 recovery-tool 和洁净主机必须通过该算法 conformance；失败不能自动尝试另一算法。增加算法必须新增版本化 ADR、KAT 和 downgrade negatives。

### 六、Shamir、PIV 与恢复仪式

三个 encrypted share 分别交给三个不同 custodian/PIV token；任何时刻单一 custodian、单一 token、daily writer、target 或生产管理员都不能恢复。`recovery-tool` 是唯一允许的恢复 executable，要求：

1. 在洁净 Windows Server 上离线验证 recovery-domain manifest、BackupKeyEnvelope 签名/链/CRL/时间戳、checkpoint、EPB1 manifest 和所有 digest/generation；
2. 读取两个不同 custodian、不同 token、不同 share index 的 PIV 9D slot，要求 user presence/PIN 和双人批准；
3. 在 `VirtualLock`/zeroizing memory 解密并验证两个 69-byte share 的 magic、AAD digest、index 和 descriptor binding；
4. 仅用这两个 share 重构 `set_recovery_kek`，解开 backup DEK 并验证 AES-GCM tag/AAD；
5. 使用该 backup DEK 恢复本 set 的 EPB1 chunks，核对 chunk 顺序、总长度、ciphertext manifest/checkpoint 和业务恢复 cut；
6. 完成或失败后清零 PIN buffer、share、coefficient/reconstructed KEK、backup DEK 和中间明文；禁止进入 argv、环境变量、普通文件、pagefile/dump、日志或 Event Log。

三种组合 `(1,2)`、`(1,3)`、`(2,3)` 必须得到同一 KEK/DEK 和相同恢复结果。任一单 share、重复 custodian、重复 token/index、错误 PIN、错误 SPKI/key version、错 deployment/set/generation/AAD、撤销 recipient、篡改 ciphertext/tag、非洁净/未批准工具均失败。

原 authority TPM、OS SSD、数据库和 production network 不是恢复前提。洁净主机只需签名 recovery-tool、批准 PC/SC/PIV middleware、两枚有效 token、签名 recovery/checkpoint bundle、EPB1 ciphertext 和恢复工作空间。

### 七、KAT 与互操作门

Task 24 必须提交唯一 normative fixture `crates/platform/backup/tests/fixtures/adr0024-backup-key-envelope-v1.json`，最大 1 MiB、strict RFC 8785 JCS。它以 lowerhex 固定输入 `set_recovery_kek=000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f`、threshold-2 唯一非零 polynomial coefficient bytes `a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf` 和 participant numbers `01|02|03`，并保存 `vsss-rs 5.4.0` 产生的三份 exact 33-byte `identifier||share_value`、三种两份重构结果以及每一单份不能满足门限的结果。fixture 自身同时固定下列完整 vectors，raw bytes/digest 进入 release/SBOM evidence；任何实现重算结果不等即不互操作，不得重生成“新期望值”：

- 固定 32-byte DEK、KEK、nonce 和 BackupDekWrapAadV1 的 JCS bytes/digest、AES-GCM ciphertext/tag；
- 上述固定 secret/coefficient 下三个 `SHAMIR_GF256_VSSS_RS_V1` share bytes，三种 2-of-3 重构均匹配，单 share 被门限 API 拒绝；
- 三个 69-byte RecoverySharePlaintextV1 exact bytes；
- 每个启用 PIV share-wrap algorithm 的 software vector、三枚实际 token vector 和 wrong-label/binding negatives；
- writer 实现与 recovery-tool 互操作、旧/新 release 读取同一 version、跨洁净主机恢复；
- nonce reuse、share reorder/duplicate、recipient mix-and-match、unknown field/algorithm/version、JCS 非 canonical、tag/ciphertext/digest mutation 全部失败。

fixture 的 root exact fields 为 `schema_version=1,purpose="EP-F57-ADR0024-BACKUP-KEY-ENVELOPE-KAT-V1",dependency,shamir_vector,dek_wrap_vector,recovery_share_plaintexts,piv_rsa_vectors,piv_ecdh_vectors,envelope_jcs,envelope_digest,negative_mutations`；`dependency` exact 为 `{crate:"vsss-rs",version:"5.4.0",default_features:false,features:["primitive","std","zeroize"],cargo_lock_checksum}`，feature 数组按 bytes 排序。每项 binary 同时保存 canonical unpadded base64url 和 SHA-256，parser 要求两者重算一致；`negative_mutations` 是具名、排序唯一的 input/expected-stable-error 集，不能只写自然语言。只通过 mock、只证明“文件能打开”、缺该 committed fixture 或只用原主机 TPM 恢复均不构成 conformance。生产永不使用 fixture secret、coefficient、nonce、ephemeral/private key 或 RNG stream。

### 八、轮换、丢失、被盗与撤销

recipient/token/custodian 轮换不重加密 EPB1 数据，但必须对每个仍在 retention 内的 backup set 新建更高 `wrap_generation` 的不可变 `BackupKeyEnvelopeV1`：在双人仪式中用任意有效两 share 重构旧 set KEK、解开 DEK，生成新 set KEK/三 share，重包裹同一 DEK到新 recipient set，签名并写服务器外 checkpoint。旧、新 envelope digest 通过 `previous_envelope_digest` 链接。

新 envelope 对所有 retained sets 完成三种 2-of-3 抽检及至少一次洁净主机恢复前，不得撤销或销毁旧材料。通过后签名 revocation record 绑定旧 `recovery_domain_manifest_digest`、旧 envelope digest、原因、时间、新 digest 和双人处置证据；不得原位修改或静默删除。

- 丢失/被盗一枚 token：立即阻止使用旧 recipient set 创建新 backup，打开不可抑制恢复材料告警；用另两枚完成全 retained-set rewrap/恢复验证，再处置旧 token。单枚泄漏不等于可恢复，但仍按安全事件轮换。
- 同时丢失两枚 token：受影响且没有另一独立、已验证 envelope 的 backup set 视为不可恢复；生产认证立即失败，不能用厂商后门、生产 TPM 或 writer key 补救。
- custodian 变更、PIV key renewal、算法变更和 recovery-domain manifest generation 变化均执行相同 append-only rewrap；算法变化不能在同一 schema/version 内静默发生。
- envelope 或 token 被撤销后，正常恢复拒绝；仅客户双人批准的隔离取证流程可读取保留证据，不能把撤销材料重新标为 active。

### 九、分域和禁止项

backup recovery domain 的三 token/envelope/custodian set 必须与应用 vault、在线 operational recipient、BitLocker OS/data recovery、release signing、checkpoint signing 和 backup target administrator 分离。一个人可以承担多个普通业务职责，但不得同时满足任意恢复 threshold，也不得同时拥有 writer、retention disposal 和 recovery 能力。

禁止：把 backup DEK 包裹给 writer/target/生产 TPM operational recipient；复用厂商全局 master key；把 KEK/share/PIN/private key 写入数据库、WinCred、普通文件、日志、dump 或备份 manifest；用同一 DEK/KEK/nonce 跨 set；降低 threshold；以两域同时在线的 2-of-2 代替 2-of-3；以单个导出软件 key 代替三个 PIV token。

## 理由

每 set DEK 与每 set recovery KEK 把损害限制在单一备份集；writer 只需 recovery public material即可创建三份加密 share，不需要恢复 private key。完整 AAD、recipient-set digest 和固定 share plaintext 防止跨部署、跨 set、跨 generation 和跨 token 拼接。2-of-3 同时允许丢失一枚 token 和保持双人控制，append-only rewrap 让轮换不必重加密全部 EPB1 数据。

## 后果

正面：ADR-0021 的每 set DEK 现在有唯一、可互操作、客户独立控制的恢复路径；任一单人/单 token/writer/target 不能解密历史；原机损坏后仍能在洁净主机恢复；算法、token 丢失和轮换均有可验证状态机。

代价：每个 backup set 产生三个 PIV share ciphertext 和独立 key envelope；三 token/custodian、PC/SC middleware、KAT、跨机演练和 retained-set rewrap 增加运营工作。丢失两枚 token 可能真实造成不可恢复，系统必须如实显示而不能提供厂商后门。

## 影响范围

- backup platform/adapter 的 key envelope、EPB1 crypto、manifest、rotation、checkpoint 与 restore；
- `backup-writer`、`BackupCheckpointSignerPort`、recovery-tool、PIV/PCSC adapter 和洁净恢复主机；
- recovery-domain manifest、recipient/token/custodian registry、revocation/rotation evidence；
- Task 24 KAT、三种 2-of-3、丢失/被盗、跨洁净主机和 retained-set rewrap tests；
- Server Control Center 的恢复材料健康、token 轮换、envelope generation、恢复认证和不可恢复告警。
