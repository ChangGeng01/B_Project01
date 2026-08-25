# ADR-0007 `FileSecretProvider` 为阶段 1 临时实现

- 状态：已取代；只保留阶段 1 历史
- 出处：阶段 1 计划第 8 节末段
- 替换方：阶段 2 计划第 4.3a 节的 `KmsSecretProvider` 终态（本 ADR 下文同步冻结）

## 背景

阶段 1 的八个进程都要能读到数据库口令等机密才能启动，而信封加密、密钥轮换与 KMS 对接整套东西属密钥阶段。若阶段 1 不给任何取机密的手段，进程起不来；若阶段 1 直接做信封加密，则做的是密钥阶段的活，且做出来的形态大概率与该阶段的真实需求不符。

## 决定

阶段 1 交付 `FileSecretProvider`：从配置键 `secrets.dir` 指定的目录读取已关闭 ACL 继承、且 DACL 只授对应服务虚拟账户、SYSTEM 与 Administrators 的文件，文件内容即机密明文，不做信封加密、不做轮换、不做审计。当前 Windows 原生交付不使用 POSIX `0600` 权限位作为判据。

配置文件中一律只写 `secret://` 引用，不写机密本身。默认引用形态见 `docs/config-reference.md` 的 `db.password_ref` 一行。

本实现在 `docs/config-reference.md` 与本 ADR 两处显式标注为阶段 1 临时状态，由阶段 2 替换为内置 KMS 或 HSM 解封。阶段 1 文档曾声称已有不变的 `SecretProvider` 调用面，但当时实际 wiring 仍直接读取文件；阶段 2 因此必须交付真实 `SecretProvider` 端口并删去常驻进程中的直接文件读取，不能把不存在的抽象当作已交付事实。

## 理由

选「明文文件加文件权限」而不是「阶段 1 自己做一版加密」：后者要在没有密钥管理的前提下再造一个密钥，那个密钥本身还是要落在同一台机器的某个文件里，安全性没有实质提高，却多出一套将被丢弃的实现。

选「配置里只写 `secret://` 引用」：这条从阶段 1 就强制，代价是阶段 1 的配置多一层间接；收益是替换后端时配置文件一个字都不用改，且任何人从配置文件里都拿不到机密，配置文件可以进版本库、可以贴进工单。

## 后果

正面：替换点收敛到一个 provider 实现与一个配置键 `secrets.provider`。阶段 2 后生产取值闭集只有 `kms`；`file` 只存在于本 ADR 的历史切片和 `ep-secretctl migrate` 的受控旧数据读取路径，不进入常驻进程。

负面（如实记录）：阶段 1 至密钥阶段之间，机密在磁盘上是明文，其保护完全依赖上述 DACL 与运行账户隔离。这一状态不得被读成「已具备密钥管理」。同期另有一条同类状态：阶段 1 的四个请求头 `X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 只校验存在性与格式，不做任何真实校验，不得被读成「已具备鉴权」，真实校验与其端口由阶段 4 交付。两条一并写进 `docs/config-reference.md`，防止误认。

## 配套的机器判定

阶段 1 另有两条与本决定配套的 CI 断言，二者不因本实现是临时的而放宽：

- `SecretString` 未实现 `Debug` 与 `Display`，机密不可能被日志格式化出去；
- 配置结构体中任何名字含 `password`、`secret`、`key`、`token` 的字段，类型必须是 `SecretString` 或 `SecretRef`。

## 阶段 2 终态替换

### Provider、载体与调用面

生产配置固定 `secrets.provider=kms`，出现 `file` 即配置错误并以 78 拒绝启动。`secret://` 只由 `KmsSecretProvider` 解引用；`wincred://` 只用于零 KMS 进程持有的集成凭据；`bootstrap://` 只用于建立 KMS 之前必须取得的本地自举材料。云 KMS 首版不支持，`kms.backend` 只取 `builtin|hsm`。

阶段 2 在 `ep_foundation::port::secret` 新增独立的 `SecretUnsealer`，在 `ep-platform-runtime` 新增只读 `SecretProvider` 与 `KmsSecretProvider`，由 `ep-adapter-kms` 的 builtin/HSM 实现承载。业务字段加密使用的 `KmsBackend` 六方法、九项词汇与 `EPC1` 不改变；系统机密库不得给它增加第七个方法或第五种 `KeyPurpose`。`SecretProvider` 固定为同步只读接口 `resolve(&SecretRef) -> Result<SecretBytes, SecretError>`；`KmsSecretProvider` 构造时固化当前 `deployment_id` 与 `recipient`，调用方不能传入或伪造 recipient。`SecretBytes` 使用 zeroizing 内存，不实现 `Clone`、`Debug`、`Display` 或 `Serialize`，文本消费者显式转换为具有相同约束的 `SecretString`。

ABI 固定如下；`SecretUnsealRequest` 只能由 strict EPS1 parser 构造，builtin/HSM unsealer 自身还固化 expected deployment/recipient 并在触碰密钥前逐字复核。`SecretErrorKind` 是进程内非稳定错误闭集，不进入 HTTP/错误码登记；启动路径统一输出不含 ref/路径/secret 的类别与 incident correlation 后以 78 退出。

```rust
pub enum SecretRecipient { EpCore, EpWorker, EpOps, EpArchive, EpBackup, EpMigrate }
pub struct SecretUnsealRequest<'a> {
    pub deployment_id: uuid::Uuid,
    pub recipient: SecretRecipient,
    pub key_ref: &'a SecretStoreKeyRef,
    pub key_version: core::num::NonZeroU32,
    pub nonce: &'a [u8; 12],
    pub aad: &'a [u8],
    pub ciphertext_and_tag: &'a [u8],
}
pub enum SecretErrorKind {
    InvalidRef, NotFound, AccessDenied, InvalidEnvelope, IdentityMismatch,
    BootstrapUnavailable, KeyUnavailable, DecryptFailed, Io,
}
pub struct SecretError { pub kind: SecretErrorKind, pub incident_id: uuid::Uuid }
pub trait SecretUnsealer: Send + Sync + 'static {
    fn unseal(&self, request: SecretUnsealRequest<'_>) -> Result<SecretBytes, SecretError>;
}
pub trait SecretProvider: Send + Sync + 'static {
    fn resolve(&self, secret_ref: &SecretRef) -> Result<SecretBytes, SecretError>;
}
```

`SecretRef`、`BootstrapRef` 与 `SecretStoreKeyRef` 均为私有字段的 opaque strong type，只能由各自 strict parser 构造；`SecretBytes` 与 `SecretString` 的底层缓冲同样私有。上述所有 struct/enum 拒绝 serde 派生；`SecretError` 不保存 source error 正文、ref、路径、key ref 或 secret。`KmsSecretProvider::resolve` 的唯一顺序是规范化 ref→按构造时 recipient 导出路径→以拒绝 reparse/ADS/hardlink 的句柄打开并读回 owner/DACL/最终路径→有界读入并 strict parse→复核 deployment/recipient/ref/key identity/AAD→调用 bound unsealer→zeroize envelope 工作缓冲；任一步失败均不返回部分明文。

首版 `secret://` recipient 闭集为 `ep-core|ep-worker|ep-ops|ep-archive|ep-backup|ep-migrate`。`ep-integ|ep-plugin` 的持久集成凭据使用 Windows Credential Manager，`ep-portal|ep-ai` 没有 KMS 机密。未来增加 recipient 必须同时改枚举、DACL、依赖图与发布门禁。

### 无数据库循环的自举

`deployment_id` 取既有签名部署配置中的非零规范 UUID，在数据库连接前可用。builtin 系统机密库不让上述六个 recipient 共同读取数据 KMS 的 `C:\EP\kms\master.key`；该 common master 继续只承载原数据 KMS 且其既有 ACL 不扩张。`ep-secretctl bootstrap` 为每个 recipient 独立生成 32 个 CSPRNG 字节，并用 Windows DPAPI machine scope 封装为 `C:\EP\kms\bootstrap\<recipient>\secret-store-kek-<key-version>.dpapi`。文件关闭继承，允许 ACE 只含该 recipient、SYSTEM、Administrators；DPAPI additional entropy 固定为 `SHA-256("EP-BUILTIN-SECRET-STORE-KEK-V1\0" || deployment_id[16] || u16be(recipient_len) || recipient || u32be(key_version))`。取得另一个 recipient 的 blob 或当前 recipient 的 entropy 都不能成为读取其他目录或选择其他 recipient 的产品路径。

HSM 为每个 recipient 建独立的 nonextractable AES-256 secret-store object，`CKA_SENSITIVE=true`、`CKA_EXTRACTABLE=false`，只允许 AES-GCM encrypt/decrypt。`kms.hsm.pin_ref` 固定为 `bootstrap://windows-dpapi/hsm-pin#1`，按引用版本落到 `C:\EP\kms\bootstrap\<recipient>\<sha256(canonical-bootstrap-ref)>.dpapi`；additional entropy 固定为 `SHA-256("EP-HSM-PIN-BOOTSTRAP-V1\0" || deployment_id[16] || u16be(recipient_len) || recipient || u16be(ref_len) || canonical_bootstrap_ref)`。解出的 PIN 必须是 1..255 UTF-8 bytes，禁止 NUL、CR、LF。HSM module、slot、PIN 或 object 任一不可用即退出 78，不得回落 builtin。

启动顺序固定为“签名配置与 deployment_id → 当前 recipient 的 DPAPI KEK，或 DPAPI PIN 加 HSM object → KmsSecretProvider → 解出数据库口令 → 第一次数据库连接”。必须以夹具证明数据库为空或不可达不影响机密解密，并证明机密缺失时数据库 connect 调用次数为零。

### 引用与 `EPS1` 信封

`SecretRef` 规范形态为 `secret://<segment>/<segment>[/...]#<version>`：2..8 段，每段匹配 `[a-z0-9][a-z0-9._-]{0,63}`；总长不超过 512 UTF-8 bytes；version 匹配 `[1-9][0-9]{0,9}`、无前导零且不大于 2147483647。空段、`.`、`..`、反斜线、冒号、百分号编码、query、额外 `#`、UNC、绝对路径和 ADS 一律拒绝。`BootstrapRef` 只接受 `bootstrap://windows-dpapi/hsm-pin#<version>`，version 使用同一规则。`SecretStoreKeyRef` 只接受下表后的 builtin/HSM 两种形态：deployment UUID 必须为小写连字符 canonical text，recipient 必须命中枚举，HSM slot 为无前导零的 u32 十进制（0 本身合法），key version 与信封逐字相等；三种 parser 都不得先接受非规范输入再重写。文件名不使用引用原文，固定为 `C:\EP\secrets\<recipient>\<sha256(canonical-secret-ref) lowerhex>.eps1`。

系统机密信封 magic 为 `EPS1`，与数据库字段的 `EPC1` 是两个协议：

| offset | size | 内容 |
|---:|---:|---|
| 0 | 4 | ASCII `EPS1` |
| 4 | 1 | schema version `1` |
| 5 | 1 | algorithm `1`，即 AES-256-GCM |
| 6 | 2 | big-endian flags，必须为 0 |
| 8 | 16 | deployment UUID bytes |
| 24 | 4 | big-endian key version，1..2147483647 |
| 28 | 2 | recipient length，1..32 |
| 30 | 2 | canonical secret ref length，1..512 |
| 32 | 2 | canonical key ref length，1..512 |
| 34 | 4 | plaintext length，1..65536 |
| 38 | 12 | CSPRNG nonce |
| 50 | 可变 | recipient、secret ref、key ref、等长 ciphertext、16-byte GCM tag |

最大合法文件为 66658 bytes；不允许尾随字节、未知 flag/算法或非规范字段。AAD 逐字为 `"EP-SECRET-ENVELOPE-V1\0" || envelope[0..50] || recipient || canonical_secret_ref || canonical_key_ref`。builtin key ref 固定 `kms://builtin/secret-store/<deployment-uuid>/<recipient>#<key-version>`，HSM 固定 `kms://hsm/slot/<slot>/secret-store/<deployment-uuid>/<recipient>#<key-version>`。文件内 deployment、recipient、secret ref、key ref/version 必须与请求和当前部署逐字一致。

### 轮换、迁移与生产关闭条件

机密版本不可变，永不覆盖 `#N`。先为所有声明 recipient 在同目录以 CREATE_NEW staging 写 `#N+1`，FlushFileBuffers，关闭重开并严格解析，解密后在 zeroizing 内存逐字节比对，再同卷原子发布；全部 recipient 成功后才通过签名配置把引用改为 `#N+1`。不设 `latest`、监听器或隐式 fallback。旧版本至少保留 24 小时且所有声明消费者都已由 `secrets-resolvable` 确认新版本，两个条件同时成立后才可退役。HSM PIN 的 `bootstrap://...#N` 同样 prepare、probe、显式切换、退役。

随产品交付的一次性签名工具 `ep-secretctl` 的顶层子命令闭集固定为八个：`bootstrap|put|verify|migrate|finalize-migration|retire|inventory|wincred`。前七个属于 secret-store 子集；`wincred` 只实现 `docs/config-reference.md` 第 5 节的服务 current-token 维护协议，绝不直接写管理员自己的 vault。该工具进入 SBOM 与签名发布清单，不注册服务、不监听端口、不连数据库，不把 secret 写入 stdout、stderr、Event Log 或 receipt。`put` 以及 HSM 模式 `bootstrap` 的 PIN 明文输入方式只有本机交互式 console：用关闭 echo 的 `ReadConsoleW` 读取并要求二次一致确认，拒绝重定向 stdin、argv、env 与文件输入；builtin 模式 `bootstrap` 只由 CSPRNG 生成 KEK，不读取明文。`wincred` 的输入遵守配置参考第 5 节的同一 console 限制；`migrate` 是唯一允许从下述受控 legacy 明文文件读入的例外。`verify|inventory|retire|finalize-migration` 均不得请求或读取明文。Stage 1 升级只由 `migrate` 读取旧 `<legacy-root>/<domain>/<name>#<version>`；它验证 DACL、regular file、无 reparse/ADS/hardlink、原文件 1..65536 bytes，并为兼容当时实际 resolver 精确执行 UTF-8 `str::trim()`，trim 后结果仍须为 1..65536 bytes，否则整次迁移失败且不生成信封。每个目的 EPS1 reopen/decrypt 且逐字节一致后才把旧树移入 `C:\EP\secrets-legacy-quarantine\<run-id>`。`finalize-migration` 按客户介质消毒政策移除隔离树，但不得宣称 SSD 安全擦除；legacy、quarantine、staging 任一残留都使生产发布门禁失败。receipt 只含 deployment/run/tool/binary digest、ref、recipient、envelope digest 和状态，不含明文或明文摘要。发布测试必须枚举并逐字比对上述八个顶层子命令，拒绝别名、隐藏命令与第九个命令，并为所有禁用输入面提供负例。

## 追加：同期四头临时披露已由阶段 4 关闭（任务 #23）

本 ADR「后果」负面段披露的同期同类状态——四个请求头 `X-Legal-Entity-Id`、`X-Device-Id`、`X-Client`、`Authorization` 只校存在性与格式——已由阶段 4 关闭：真实校验经端口在 core-server 装配注入（认证层令牌摘要查 sessions，法人层对照授权集合），关闭说明见 ADR-0011 追加段与 `docs/config-reference.md` 第 5 节。`FileSecretProvider` 自身也已由本 ADR 的阶段 2 终态替换；生产配置、常驻二进制与发布证据均不得再选择或携带它。
