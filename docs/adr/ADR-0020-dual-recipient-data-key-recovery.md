# ADR-0020 双 recipient 数据密钥恢复

- 状态：已接受
- 出处：F-57 `SEC-014`、威胁模型 secret vault/broker 控制
- 取代范围：取代 ADR-0009 的单一 wrapped DEK 与“不需要另一套恢复协议”结论；数据库元数据真值、数据库打开后的 exact-ref/cache/readback 语义继续有效

## 背景

单一机器绑定或日常 KMS recipient 可以满足正常运行，却不能在 TPM、主板、OS 盘或原宿主失效后独立恢复客户数据。若唯一信封随原机丢失，数据库备份仍无法解密；若日常服务能调用恢复接口，恢复材料又会扩大在线攻击面。F-57 因此要求同一个 DEK 同时具有日常运行和离线恢复两条独立、可认证的解封路径。

## 决定

一、每个客户 DEK 只生成一次明文候选，并形成两个独立认证的信封：`operational` 信封包裹给日常 TPM/HSM/KMS recipient，`recovery` 信封包裹给离线恢复 recipient。任一正确、获授权的路径都能恢复同一个 DEK；这是二选一恢复能力，不是 2-of-2，两个 recipient 必须不同。

二、两个信封分别以认证上下文绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 和 envelope version。任何字段、recipient、代或版本不等都失败关闭；不得把一条信封改标为另一 recipient，也不得从 current key 回退替代对象保存的 exact ref。

三、日常服务只能调用 operational unwrap，不能调用 recovery。recovery 唯一使用既有 `PIV_SHAMIR_2_OF_3_V1` 离线仪式：三份 Shamir share 分别由不同 PIV token/保管人持有，任意两个不同保管人可以在洁净主机完成恢复，任意单份不能恢复。恢复后只把同一 DEK 重包裹给新宿主 operational recipient；恢复材料和明文不得进入普通服务、数据库、日志、argv、环境变量或普通文件。

四、数据库打开以后，ADR-0009 的业务元数据真值、对象 `DataKeyRefV1`/`ExactRef`、进程缓存重建、readback、状态推进和 16-key 激活矩阵继续成立。正常 readiness 使用 operational 信封逐把 readback；洁净恢复使用 recovery 信封后再建立新 operational 信封，不允许恢复接口成为在线 fallback。

五、预发布且仍为 `PLANNED` 的 `V20260901092000__platform_core_data_keys.sql` 目标必须在原路径改写为下列八个非空列，不保留单数 `wrapped_key` 或 `wrap_kek_version`：

- `operational_wrapped_key`
- `operational_wrap_key_version`
- `operational_recipient_ref`
- `recovery_wrapped_key`
- `recovery_wrap_key_version`
- `recovery_recipient_ref`
- `wrap_context_generation`
- `wrap_envelope_version`

两份 wrapped bytes 必须正长度；两个 wrap key version、context generation 和 envelope version 必须为正并处于持久类型范围；两个 recipient ref 必须非空、使用 canonical grammar 且互不相同。purpose↔algorithm、data-key version、四态时间形状和 exact 16-row 激活约束继续强制；状态为非 `DESTROYED` 的行必须保持两份有效信封，销毁只经既有状态机并保留所需审计证据。此处是尚未发布目标的原路径修订，不是改写任何已应用生产迁移历史。

### 六、operational envelope exact wire 与 provider ABI

`operational_wrapped_key` 同样不得是 provider-private blob。它必须恰为无 BOM、无尾随换行的 UTF-8 RFC 8785 JCS `OperationalDataKeyEnvelopeV1`，最大 65,536 bytes；root exact fields 为：

`schema_version=1,purpose="EP-F57-OPERATIONAL-DATA-KEY-ENVELOPE-V1",profile,deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,wrap_context_generation,wrap_envelope_version,operational_wrap_key_version,operational_recipient_ref,operational_provider_manifest_digest,kek_key_identity_digest,nonce_b64url,ciphertext_b64url,tag_b64url,created_at,predecessor_envelope_digest`。

UUID、版本、purpose、scope、时间和 predecessor 规则与下一节的 recovery envelope 相同；`operational_provider_manifest_digest` 与 `kek_key_identity_digest` 都是 `sha256:` lowerhex。nonce 恰 12 bytes、ciphertext 恰 32 bytes、tag 恰 16 bytes，均为 canonical unpadded base64url；同一 operational KEK 下 nonce 永不重复。数据库 `operational_wrap_key_version`、`operational_recipient_ref`、`wrap_context_generation`、`wrap_envelope_version` 必须与 payload 逐字相等。

首版 `profile` 只有 `WINDOWS_TPM2_BROKERED_AES256_GCM_V1|PKCS11_HSM_BROKERED_AES256_GCM_V1`。前者 recipient grammar 为 `tpm2://<deployment-uuid>/hosts/<host-uuid>/keys/<64-lowerhex>`，末段是签名 TPM recipient descriptor（EK digest、PCR policy digest、service SID、WDAC policy digest、NV generation 和 key identity）的 JCS SHA-256；后者为 `pkcs11-object://<deployment-uuid>/<provider-uuid>/keys/<64-lowerhex>`，末段是签名 token serial/slot/CKA_ID/mechanism/policy descriptor 的 JCS SHA-256。profile 与 URI scheme 必须匹配。TPM sealed key、HSM object 和 provider manifest 必须当前、未撤销并 exact-match digest；软件导出 KEK、DPAPI/WinCred 客户 KEK、任意 locator、第三种 profile 或自动 fallback 均拒绝。未来云 KMS 需新版本化 ADR/profile/KAT，不能借 `CUSTOM` 字符串进入 v1。

operational DEK-wrap AAD bytes 恰为 `UTF8("EP-F57-ADR0020-OPERATIONAL-DEK-WRAP-AAD-V1") || 0x00 || JCS({schema_version:1,profile,deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,wrap_context_generation,wrap_envelope_version,operational_wrap_key_version,operational_recipient_ref,operational_provider_manifest_digest,kek_key_identity_digest,created_at,predecessor_envelope_digest})`。共享 crypto 层独占构造 AAD；provider ABI 只接受已验证的 `OperationalWrapRequestV1`，其字段恰为上述绑定值、12-byte nonce 和不可导出的 32-byte DEK handle，并只返回 `{ciphertext[32],tag[16],provider_readback_digest[32]}`。unwrap 只接受同一 verified request/envelope，返回锁页/zeroizing DEK handle；调用者不能传 raw AAD、切换 recipient/profile 或取得 KEK。builtin TPM 与 PKCS#11 HSM adapter 必须读回自身当前 identity/policy 后再做 AEAD，不能仅凭数据库 locator。

Task 2 必须提交 `crates/adapter/kms/tests/fixtures/adr0020-operational-envelope-v1.json`，包含两个 profile 的固定 JCS/AAD/AES-GCM vector、wrong-profile/scheme、provider-manifest swap、PCR/policy/key-identity drift、nonce reuse、predecessor/version rollback 和 cross-field mutation negatives；Windows 实 TPM、实际批准 HSM（若部署启用）与 pure-crypto vector 必须逐字互操作。未部署 HSM 时其 profile 保持未激活而不是用 mock 认证。

### 七、application recovery envelope exact wire

`recovery_wrapped_key` 不是 provider-private blob。它必须恰为无 BOM、无尾随换行的 UTF-8 RFC 8785 JCS `DataKeyRecoveryEnvelopeV1`，最大 1,048,576 bytes；重复/未知/缺失字段、非 canonical JSON/base64url/UUID/time/digest 均拒绝。root exact fields 按下列逻辑集合定义（JCS 决定实际 key 顺序）：

`schema_version=1,purpose="EP-F57-DATA-KEY-RECOVERY-ENVELOPE-V1",profile="PIV_SHAMIR_2_OF_3_V1",deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,wrap_context_generation,wrap_envelope_version,recovery_wrap_key_version,recovery_recipient_ref,recipient_set_digest,threshold=2,share_count=3,dek_wrap,shares,created_at,predecessor_envelope_digest`。

UUID 均为 lowercase hyphenated；`data_key_version` 为 1..65535；三个 generation/version 为 1..2,147,483,647；`key_purpose` 只取 `FIELD|BLIND_INDEX|ATTACHMENT|ARCHIVE`，`security_level_scope` 只取整数 `10|20|30|40`。`predecessor_envelope_digest` 首版必须为 JSON null，轮换后必须为前一 envelope exact JCS 的 `sha256:` lowerhex；不能缺失或空字符串。`recovery_recipient_ref` exact grammar 为 `piv-shamir://<deployment-uuid>/<legal-entity-uuid>/<key-domain-uuid>/sets/<64-lowerhex>`，末段逐字等于去掉 `sha256:` 前缀的 `recipient_set_digest`；数据库列必须与 payload exact-equal。

`dek_wrap` exact fields 是 `{algorithm:"AES_256_GCM",nonce_b64url,ciphertext_b64url,tag_b64url}`。nonce 恰 12 random bytes、ciphertext 恰 32 bytes、tag 恰 16 bytes，均 canonical unpadded base64url；每次 envelope/rotation 使用新的 CSPRNG nonce，跨任意 data key/recovery KEK 重复立即拒绝。恢复 KEK 恰 32 random bytes，只存在于 locked/zeroizing ceremony memory，用 AES-256-GCM 加密同一 32-byte DEK。

DEK-wrap AAD bytes 恰为 `UTF8("EP-F57-ADR0020-DEK-WRAP-AAD-V1") || 0x00 || JCS({schema_version:1,deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,wrap_context_generation,wrap_envelope_version,recovery_wrap_key_version,recovery_recipient_ref,recipient_set_digest,profile:"PIV_SHAMIR_2_OF_3_V1",created_at,predecessor_envelope_digest})`。字段、类型和 null 形状逐字取 envelope；调用者不得供应任意 AAD bytes。任何 mismatch 统一失败关闭且不能尝试 operational fallback。

### 八、Shamir 与 PIV share exact wire

实现固定依赖 `vsss-rs = 5.4.0`，`default-features=false, features=["std","primitive","zeroize"]`，Cargo.lock checksum/SBOM 必须命中；禁止 6.x pre-release、floating range、git branch 或第二套 Shamir 实现。使用该版本 constant-time `Gf256` byte-sequence Shamir、threshold=2、share_count=3、`SequentialParticipantNumberGenerator` 从 1 步进 1；输入恰为 32-byte recovery KEK。plaintext share 恰 33 bytes：首字节 identifier `0x01|0x02|0x03`，其后 32 bytes 是对应 GF(256) share value；数组按 identifier 升序且任何 zero/duplicate/out-of-range identifier 拒绝。

`shares` 恰三项，每项 strict fields 为 `{share_index,piv_token_id,custodian_id,piv_slot:"9D",recipient_spki_sha256,algorithm:"PIV_ECDH_P256_HKDF_SHA256_AES256_GCM_V1",ephemeral_public_key_sec1_b64url,nonce_b64url,ciphertext_b64url,tag_b64url}`。`share_index` 恰 1、2、3；token/custodian UUID 各自三项全异；recipient SPKI 是 `sha256:` lowerhex 并命中签名 PIV enrollment registry。ephemeral public key 是恰 65-byte uncompressed SEC1 P-256 point，必须 on-curve、非 identity、每 share 新生成；nonce 12 bytes、ciphertext 33 bytes、tag 16 bytes。PIV slot 9D 私钥不可导出；token 必须用 P-256 ECDH、用户 presence/PIN 和当前未撤销证书。

每 share 的 shared secret 经 HKDF-SHA-256 派生 32-byte AES key：salt 恰为 raw 32-byte `recipient_set_digest`，info 恰为 `UTF8("EP-F57-ADR0020-PIV-SHARE-KEY-V1") || 0x00 || JCS({deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,recovery_wrap_key_version,share_index,piv_token_id,custodian_id,piv_slot:"9D",recipient_spki_sha256})`。share AES-256-GCM AAD 是 `UTF8("EP-F57-ADR0020-PIV-SHARE-AAD-V1") || 0x00 || JCS({purpose:"EP-F57-DATA-KEY-RECOVERY-ENVELOPE-V1",profile:"PIV_SHAMIR_2_OF_3_V1",deployment_id,legal_entity_id,key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,wrap_context_generation,wrap_envelope_version,recovery_wrap_key_version,recovery_recipient_ref,recipient_set_digest,share_index,piv_token_id,custodian_id,piv_slot:"9D",recipient_spki_sha256,ephemeral_public_key_sec1_b64url})`。解密后 identifier 必须与 share_index 一致；只组合两个不同 identifier/custodian/token 的 share。

`recipient_set_digest = SHA-256(JCS([{share_index,piv_token_id,custodian_id,piv_slot,recipient_spki_sha256},...]))`，数组按 share_index；digest 不含随机 ephemeral/ciphertext，以稳定标识 custody set。Task 2 必须提交 `crates/adapter/kms/tests/fixtures/adr0020-piv-shamir-v1.json`：固定 recovery KEK/DEK、固定 vsss RNG stream、三份 plaintext share、三套 P-256 recipient/ephemeral keys/nonces、HKDF/AAD、ciphertext/tag、三个两-share恢复结果与所有单-share失败；Linux/macOS pure-crypto KAT 和 Windows real-PIV interoperability 对同一 wire 逐字通过。生产永不使用 fixture RNG/key。

### 八点一、共同 recovery-domain manifest 权威

PIV enrollment registry 不是未定义的外部名册。Application 与 backup 两域共享 strict inner `RecoveryDomainDescriptorV1` 与 recipient 数据形状，但公开签名 payload 必须是两个不同 Rust newtype：`ApplicationRecoveryDomainManifestPayloadV1` 和 `BackupRecoveryDomainManifestPayloadV1`。两者 wire root exact fields 都是 `schema_version=1,purpose,domain_kind,deployment_id,domain_id,generation,recipient_set_version,algorithm,threshold=2,share_count=3,recipients,recipient_set_digest,issued_at,not_before,expires_at,predecessor_manifest_sha256,revocation_checkpoint_sha256`，却分别拥有唯一编译期 `BusinessArtifactPayloadV1::PURPOSE`。Application wrapper 固定 `domain_kind=APPLICATION,purpose="EP-F57-APPLICATION-RECOVERY-DOMAIN-MANIFEST-V1"`；Backup wrapper 固定 `domain_kind=BACKUP,purpose="EP-F57-BACKUP-RECOVERY-DOMAIN-MANIFEST-V1"`。禁止一个 tagged payload 运行期切 purpose，也禁止对同一 Rust type 实现两次 signer trait。算法 v1 仅为 `PIV_ECDH_P256_HKDF_SHA256_AES256_GCM_V1`，threshold/share_count 固定 2/3；`generation`、`recipient_set_version` 为正数且严格递增。

`recipients` 恰三项，按 `share_index=1,2,3` 排序；每项 exact fields 为 `share_index,piv_token_id,custodian_id,piv_slot="9D",recipient_key_version,recipient_spki_sha256,recipient_certificate_sha256,piv_attestation_sha256,certificate_serial_hex,certificate_not_before,certificate_not_after`。三个 token、custodian、SPKI 全异；key version 为正；证书在整个 manifest 有效窗口内有效并命中该域完整离线 CRL/attestation policy。`recipient_set_digest` 恰为 `SHA-256(JCS([{share_index,piv_token_id,custodian_id,piv_slot,recipient_key_version,recipient_spki_sha256,recipient_certificate_sha256,piv_attestation_sha256},...]))`，lowercase `sha256:`；它同时决定 `recovery_recipient_ref` 末段。`revocation_checkpoint_sha256` 绑定该域最新、单调且服务器外保存的 full-CRL/token-loss checkpoint；未知、过期、撤销或 checkpoint 回退的 recipient 不能用于新 envelope/ceremony。

存储载体分别严格为 `SignedBusinessArtifactV1<ApplicationRecoveryDomainManifestPayloadV1>` 与 `SignedBusinessArtifactV1<BackupRecoveryDomainManifestPayloadV1>`。APPLICATION current locator 是 `<ValidatedDataRoot>/RecoveryDomains/Application/current.v1.json`，history locator 是 `<ValidatedDataRoot>/RecoveryDomains/Application/history/<generation>-<payload_sha256>.v1.json`；BACKUP 使用同构的 `.../Backup/...` 路径。固定路径都按 final handle 验证在 authority HDD，拒绝 reparse/hardlink/alternate stream；写 history 后 file+directory flush/readback，再以 compare-and-swap 原子替换 current。新 current 必须 `generation=old+1`、`recipient_set_version=old+1`、`predecessor_manifest_sha256=old payload digest`，且域、deployment 不变；首版 predecessor 为 JSON null。current 与历史只增不改，凡仍被 data-key/backup envelope 引用的 generation 永不删除。签名后 registry swap、两个 current、跳代、错 predecessor、跨域 signer/purpose/recipient、application/backup token 或 custodian 交集均失败关闭。

APPLICATION 由 Task 2 的 application-recovery-domain roster 签发并实现；BACKUP 由 Task 24 的独立 backup-recovery-domain roster 签发并实现。两者 schema 都是 `docs/evidence/f57-recovery-domain-manifest.schema.json`，该 schema 对 root/nested objects 均 `additionalProperties:false` 并编码上述 conditional purpose/domain rules。Task 2/24 必须分别验证 current/history/CAS/rotation/revocation，Task 24 还必须把两域 manifest exact set 与 `RecoveryDomainSeparationEvidenceV1` 交叉核验；该 evidence 不能替代 manifest 本身。

### 九、签名 recovery manifest、轮换与撤销

`apps/recovery-tool` 只接受共享 verifier 验证后的 `SignedBusinessArtifactV1<DataKeyRecoveryManifestPayloadV1>`。payload exact fields 为 `schema_version=1,purpose="EP-F57-DATA-KEY-RECOVERY-MANIFEST-V1",ceremony_id,deployment_id,legal_entity_id,operation,items,approved_by,requested_at,not_before,expires_at,generation,audit_ref`。`operation` 闭集为 `VERIFY_ONLY|CLEAN_HOST_REWRAP|RECIPIENT_SET_ROTATION`；`items` 非空，按 `(key_domain_id,data_key_id,data_key_version)` 排序唯一，每项 exact `{key_domain_id,data_key_id,data_key_version,key_purpose,security_level_scope,recovery_wrap_key_version,recovery_envelope_digest,target_operational_recipient_ref,target_recovery_domain_manifest_digest,target_recipient_set_digest,target_recovery_wrap_key_version}`。

该 item 是 strict conditional union：

- `VERIFY_ONLY`：四个 `target_*` 全部必须为 JSON null；
- `CLEAN_HOST_REWRAP`：只允许 `target_operational_recipient_ref` 为与旧 operational recipient 不同的 canonical non-null ref，其余三个 target 为 null；
- `RECIPIENT_SET_ROTATION`：`target_operational_recipient_ref=null`，其余三项必须非空；两个 digest 为 `sha256:` lowerhex，`target_recovery_wrap_key_version=recovery_wrap_key_version+1` 且不溢出。

rotation 执行时必须锁定并验证 manifest 中指定的 target recovery-domain manifest exact bytes/digest，重算其三个排序 recipient descriptor 得到 `target_recipient_set_digest`，并证明新 token/custodian/SPKI exact-set 与旧集合按轮换政策有效且互异；不得在验签后改为查询“registry 当前值”。签名 manifest digest、目标 domain manifest digest、target set digest、生成的新 envelope predecessor 和数据库 CAS 在同一执行证据中交叉绑定。wrong-shape、target 字段混用、version 跳跃、签名后 registry swap、digest/descriptor drift 和用 operational target 代替 recovery set 全部在任何 unwrap 前失败。

`approved_by` 恰两名不同、当前有 application-recovery 能力且不在目标三名 custodian 内的 principal UUID，排序唯一。窗口 `0 < expires_at-not_before <= 4h`，`requested_at<=not_before<=expires_at`，CMS signingTime 恰等于 requested_at；manifest、ceremony_id 和每 item 以 durable CAS single-use，部分执行只能 FAILED_CONTAINED 后按同一 manifest 幂等对账，不能跳项。

application recovery 使用独立 trust domain/roster、recipient set、tokens、custodians and purpose；不得复用 backup、BitLocker、release-signing 或 operational identities。正常服务 binary/identity 无 recovery-tool invoke、PIV、manifest 或 recovery unwrap capability。VERIFY_ONLY 不返回 DEK，只产 digest-bound verification evidence；另两种操作只把 DEK 重包给批准的新 operational recipient或新 recovery set，随后立即零化。

recipient-set rotation 必须生成全新 256-bit recovery KEK、全新 Shamir polynomial、三名有效且互异的新 token/custodian、nonce/ephemeral keys和 `recovery_wrap_key_version+1` envelope，并以 predecessor digest 连接；在一个 CAS 事务验证新 envelope/KAT/readback 后切 current，保留旧 ciphertext/envelope/audit 但禁止普通 current 解析。token lost/stolen/revoked 立即禁止包含它的新 ceremony并进入 ROTATION_REQUIRED；只有签名紧急 rotation manifest 和剩余两个未撤销 share 可一次性恢复后换整套，不能复制旧 share或只替一份。recipient/custodian/certificate、wrap-context、algorithm/profile 或 data-key binding 任一改变都需新 envelope；rollback、跳版本、predecessor mismatch、把 REVOKED/DESTROYED envelope 重新 current 均拒绝。DEK `DESTROYED` 后 recovery ceremony 永久拒绝，但 envelope、revocation和销毁证据按策略保留。

## 理由

双 recipient 信封把日常可用性与灾难恢复分开，同时避免要求两个域同时在线。若只保留 operational 信封，原机损坏会让备份不可恢复；若把恢复接口交给日常服务，攻破运行身份就可能越过离线双人控制。

## 后果

正面：同一 DEK 可由日常或离线路径独立恢复，且 recipient、用途和代际均受认证绑定；数据库打开后的 exact-ref 行为不漂移。

代价：供给、轮换、readback、迁移约束、审计 payload、恢复演练和销毁证据都必须同时处理两份信封；任何一份缺失、重复 recipient 或绑定不一致都阻止激活与发布。

## 影响范围

- `data_keys` 数据字典与预发布迁移 `V20260901092000`；
- KMS/HSM provision/readback DTO、缓存装载与 exact-ref 解封；
- `recovery-tool`、PIV/Shamir 仪式、洁净主机恢复与重包裹；
- 激活审计、轮换、销毁、威胁模型和恢复发布门。
