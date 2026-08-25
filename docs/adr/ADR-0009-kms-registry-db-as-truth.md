# ADR-0009 数据库 wrapped DEK 为唯一持久真相，KMS 注册表只作可重建缓存

- 状态：部分被 ADR-0020 取代；数据库元数据、exact-ref、cache 与数据库打开后的 readback 语义继续接受
- 取代说明：[ADR-0020](ADR-0020-dual-recipient-data-key-recovery.md) 取代本文单一 wrapped DEK 和“不需要另一套恢复协议”的结论，现行契约为 operational/recovery 双 recipient 信封
- 出处：阶段 2 §4.1、§4.2；F-56 首装与 pinned data-key 补强
- 替换关系：本文现行内容替代早期“A-06 缺注册表可跳过”和“从库内 KEK 版本直接生成 DEK”的写法

## 背景

内置 KMS 或客户 HSM adapter 在进程内需要缓存已 readback 的数据密钥身份，字段、附件和归档才能高效选钥；进程崩溃或重启后缓存必然为空。若缓存成为第二份持久真相，就必须解决数据库、文件/HSM 与缓存之间的脑裂；若空缓存又被当成“可以跳过”，服务会在没有证明 KEK/DEK 可解封时开放 readiness，并可能令旧附件误取当前新密钥。

## 决定

一、持久真相只有两部分：PostgreSQL `key_domains`/`data_keys` 保存业务状态、logical KEK locator 与 wrapped DEK；所选 KMS/HSM provider 只保存该 locator 对应的版本化 KEK object。明文 DEK、可回读的第二个 provider DEK object和进程注册表都不得持久化。

二、所有开通、轮换与 F-56 首装 resume 都只经 application 层唯一 `KeyDomainProvisioner`，并调用独立 `KmsKeyMaterialProvisioner::{ensure_kek,generate_detached_data_key,readback_wrapped_data_key}`。缺数据库行时才生成 transient DEK candidate、立即由 exact KEK 包裹并销毁明文；已有行绝不重生，必须从该行重构 wrapped DTO并实际 readback。provider label、deployment/domain/legal-entity、purpose/scope、data-key id、非零 u16 版本、算法、KEK 版本、32-byte 长度与摘要任一不等即失败关闭。

三、core-server 与 job-worker 在依赖密钥的 public readiness 前，必须从数据库读取全部非 `DESTROYED` data key 并逐把调用相同 readback，成功后才重建进程注册表。不得因注册表为空、某行缺失、provider 不可用或 A-06 只是快照而跳过；任一失败统一返回 `PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE` 并关闭相关 readiness。运行期 exact-ref cache miss 只允许按同法人/id/version只读加载一行、readback 后恰重试一次；不存在、多行、归属不符或再次失败即停止，不回退 current。

四、新密文写入只使用 `DataKeySelectorV1::CurrentForWrite` 并要求目标 tuple 恰一 `ACTIVE`；历史字段/附件/归档读取和中断续传只使用对象已保存的 `DataKeyRefV1`/`ExactRef`。`ACTIVE|RETIRING|RETIRED` 可解密既有对象，`DESTROYED` 不可；历史读绝不重新解析为 current。持久 wire、数据库 version 与 EPC1 两字节版本共用 `1..65535` 域，不得截断。

五、首次 `PROVISIONING→ACTIVE` 只有在四用途 `FIELD|BLIND_INDEX|ATTACHMENT|ARCHIVE` × 四 scope `10|20|30|40` 的 version=1 exact 16-row 矩阵全部 readback 后才可发生；域、16 行与唯一 `platform.key_domain.activated.v1` exact audit payload必须同一 PostgreSQL 事务提交。审计或矩阵缺、多、重复、错序、错算法、错摘要时不得形成 ACTIVE/readiness/发布正向证据。

六、外部 KMS 操作与 PostgreSQL 不伪装成分布式原子事务。数据库提交前崩溃只会遗留可安全重生的 transient candidate或按固定 label识别的 provider material；恢复必须锁定同一 `key_domain_id` 的 `PROVISIONING` 行继续，绝不删除未知对象、创建第二域、任选 orphan或切换 provider。只有目标法人完全没有 `key_domains` 行才返回 `PLATFORM.KEY_DOMAIN.NOT_PROVISIONED`；已有 `PROVISIONING|ACTIVE` 行后的所有供给/readback/矩阵/审计问题都返回 `KEY_UNAVAILABLE`。

## 理由

此形态让数据库 wrapped DEK 成为单一可备份、可审计、可恢复的业务事实，同时让 provider 只承担不可导出的 KEK。进程缓存可丢且能从受控事实重建，不需要另一套恢复协议；pinned exact ref 又保证轮换只影响新写，不会让已存对象漂移选钥。

## 后果

正面：重启、轮换和中断续传都有唯一行为；缓存丢失不丢密钥身份；HSM 与 builtin 共享同一 application 契约；Stage 14 可以从数据库、provider readback 与 activation audit 三方重算证据。

代价：KMS/HSM 不可用或任一 wrapped row 损坏时，相关 readiness/读取会失败关闭；本 ADR 不构成 KMS 高可用承诺。未来改变持久真相、版本宽度或恢复策略必须另立 ADR、迁移与威胁模型。

## 影响范围

- `ep_foundation::port::kms` 的 `KmsKeyMaterialProvisioner`、`KmsPinnedDataKeyBackend` 与 strong values；
- `crates/adapter/kms` 的 builtin/HSM 实现与进程缓存；
- `KeyDomainProvisioner`、`DataKeyRegistryLoader`、字段/附件/归档调用方；
- Stage 2 数据字典、Stage 3 附件/TOTP、F-56 首装与 Stage 14 证据门。
