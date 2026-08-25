# F-56 许可证与签名模块包终态冻结

> **F-57 现行状态（2026-08-23）：`PARTIALLY_SUPERSEDED`。** 许可四态、离线信任链、内置模块授权、停用保留数据继续有效；本文的 `MODULE_PACKAGE` 仅指十五个内置模块的许可信封，不再代表全部扩展机制。F-57 的独立 `CAPABILITY_PACKAGE` 可以声明对象、流程、UI、报表、受控迁移、WASM、Windows worker、连接器和受控容器，但仍禁止任意 DLL、脚本、直接 SQL 和越权。详见 [F-57 §5](2026-08-23-f57-governed-automation-fabric-design.md#5-配置代能力包与热插拔)。

状态：**已批准、无待选择项；在本文范围内覆盖 F-55 及更早文档的不同写法。**
日期：2026-08-22（Australia/Melbourne）
适用范围：永久授权、订阅授权、离线续期与撤销、许可计量、十五个内置业务模块的签名模块包、ServerAdmin 模块管理、F-55 AI/MCP entitlement 与 Stage 14 发布证据。

## 1. 结论与边界

本裁定关闭四个尚未形成一条可实现链的缺口：

1. `LicenseStatus` 必须真实表达生效、临期、30 天宽限和受限运行，不能再用 `Expired` 同时代表后两种相反后果；
2. 首版同时交付 `PERPETUAL` 与 `SUBSCRIPTION`，不得再用一组必填 `valid_to` 列假装两者相同；
3. F-55 的 `F55_LOCAL_AI|F55_MCP` 必须来自同一份已验签许可，不得由配置布尔值、模块码或人工证据推断；
4. “签名模块包按许可证安装、启用、停用和升级，停用保留数据”必须有确切载体、状态边、审批、回退和验收入口。

唯一低成本形态如下：

- 不增加常驻进程、监听端口、数据库、消息中间件或独立管理后端；
- 不增加数据库迁移编号：扩充尚未生成的 Stage 3 既定建表迁移，并在既定 `V20261013093300` 中补跨表外键；
- 复用既有签名配置包的导入、自动测试、审批、签名和发布端点；
- `ConfigItemApplier` 增加 `LICENSE_GRANT` 与 `MODULE_PACKAGE` 两类，`ItemKind` 终态从 F-55 的 18 项增为 20 项；
- 十五个业务模块的 Rust 代码、SQL 迁移和固定 UI 能力仍随统一、签名的产品制品交付。模块包只是经双重签名保护的声明式安装/启停/版本凭证，**不得携带或执行 DLL、EXE、脚本、SQL、WASM、容器、安装钩子、任意路径或 URL**；WASM 与受控容器继续走各自已冻结的扩展通道；
- `LICENSE_GRANT` 与 `MODULE_PACKAGE` 均是不可通用回退的单项包。发布后只能导入并发布一份新的已签名许可动作或模块动作；不得改旧行、删旧包或以数据库回滚抹除历史。

## 2. 签名文件与信任根

### 2.1 通用封套

两类内容项共用以下签名封套；封套、payload 及其全部嵌套对象都是 strict JSON，unknown/duplicate field 一律拒绝，UTF-8 无 BOM，payload 经 RFC 8785 JCS 规范化后不得超过 1,048,576 bytes。所有最终写入 PostgreSQL `text/jsonb` 的 JSON/TOML String（包括 license/package/name/reason/code/subject，以及 TOML escape 解码后的值）都必须是合法 Unicode scalar sequence 且不得含 U+0000；解析器须在 hash/签名验真之后、任何数据库写入或业务副作用之前对整棵 typed DTO 统一复查，不能依赖 PostgreSQL 报错形成另一语义。F-56 special 中任一此类字段含 U+0000 都固定返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 并保证 package/item/audit/文件投影零写入；外层 ZIP、UTF-8 或 TOML 本身尚未形成合法 typed DTO 的失败仍沿用第 2.2 节的容器/载荷错误码。`LICENSE_GRANT.after_spec` 恰为 `SignedBusinessArtifactV1<LicenseArtifactPayloadV1>`；`MODULE_PACKAGE.after_spec` 恰为第 4.1 节的 `ModulePackageItemV1`，其中 `artifact` 恰为 `SignedBusinessArtifactV1<ModulePackageManifestV1>`。模块动作与 reason 位于 `after_spec`，由 item hash 与外层配置包签名共同保护，模块 manifest 另由本封套的内层 CMS 保护；不得再套第三层封套，也不得把两种 `after_spec` 误解析成同一个泛型形状。封套恰含：

```rust
pub struct SignedBusinessArtifactV1<T> {
    pub payload: T,
    pub payload_sha256: Sha256Digest,
    pub signer_subject: String,
    pub signature_cms_b64url: String,
}
```

`Sha256Digest` 的 JSON wire 恰为 `[0-9a-f]{64}` ASCII string，解码后恰 32 bytes；拒绝大写、`0x`、base64、byte array 或其他别名，数据库 `bytea` 只存对应 32 raw bytes。`signer_subject` 虽保留既有字段名，其安全 wire 恰为 `spki-sha256:` 加 signer certificate exact DER SubjectPublicKeyInfo 的 SHA-256 lowerhex。人类可读 X.509 display name 只可从证书派生用于界面/审计显示，绝不参与身份比较、授权或 JCS。

许可/模块发行签名人的唯一授权事实是当前已独立验签、绑定本次 `deployment_id/product_build_sha256` 的 `DeploymentManifestV1.license_trusted_signer_subjects`。该字段恰含 1..64 个上述 `spki-sha256:<64-lowerhex>` token，按 UTF-8 bytes 严格递增且零重复；缺字段、空集、乱序、重复或任一非法 token 都使部署清单无效并关闭 readiness、special 变更与发布 gate。该 signed roster 是“仍可识别并分类的发行 signer identity 集”，不是绕过证书状态的活动授权表：任一当前数据库 `RELEASED` special inner 或 source outer 历史引用过的 token 都必须永久保留在后续 roster，CAB 新 roster 必须是这份历史引用 exact-set 的 superset；删除任一仍被引用 token 在替换前失败。新 artifact 仍必须同时命中 roster 且整链为 ACTIVE；保留已经撤销的 token 不会重新授权，因为完整 CRL 求值仍把它分类为 REVOKED 并只允许本节窄恢复/历史隔离。若必须移除仍被引用 token，只能可信整库回退到无该引用的完整状态或建立新 deployment，不能在热轮换中删除。`release.trusted_signer_subjects` 不再是授权源，只是可选本机 exact-set 断言：默认 `[]` 精确表示“不覆盖，使用 signed deployment roster”；非空时必须同样规范化且与 signed roster 逐项、顺序完全相等，否则 readiness 与运维 gate 失败。它绝不能增加、删除或替换签名人。首装先以离线签名部署清单取得 roster 再验证 initial special；CAB 轮换必须在同一离线发布批次原子发布新签名部署清单 roster 与对应 `license-roots.p7b`，随后按本节复验全历史/current，不能先改本地配置制造信任。

`payload_sha256=SHA-256(JCS(payload))`。`signature_cms_b64url` 只接受 RFC 4648 §5 canonical base64url-no-pad，解码后为最大 1,048,576 bytes 的单个完整 DER `ContentInfo`；`contentType` 恰为 `signedData`，`content` 恰为一个 `[0] EXPLICIT SignedData`，输入末尾零 trailing byte。`SignedData.version=3`，`encapContentInfo.eContentType=id-data` 且 `eContent` 缺省，`digestAlgorithms` 恰为只含一个 SHA-256 `AlgorithmIdentifier` 的 DER SET（OID 精确、parameters 缺省），恰有一个 `SignerInfo.version=3`；`sid` 只取并逐字匹配 leaf `subjectKeyIdentifier`，`digestAlgorithm` 同样是 parameters 缺省的 SHA-256，unsigned attributes 必须缺省。signed attributes 必须存在，wire 是 `[0] IMPLICIT`，内容恰为按 DER 排序的 `contentType=id-data,messageDigest,signingTime` 三个 Attribute，每个 attrValues SET 恰一值，零重复/未知项，`messageDigest` 等于 payload digest；实际签名 preimage 必须把该隐式字段的 content octets 重新包成 canonical DER universal `SET OF`（tag `0x31` + DER length + 原 content octets），不得直接签 wire 的 `[0]` tag，也不得重排为库私有序列。detached content 恰为 `JCS(payload)`。`signingTime` 与 payload `issued_at` 要求语义上是同一个 UTC whole-second instant，而不是比较 ASN.1 与 RFC 3339 的文本 bytes；DER 编码在 1950..2049 年只用 `UTCTime`，其余只用 `GeneralizedTime`，必须 Z-only、含秒、无小数、无 offset。CMS certificates 恰含 signer leaf 与形成唯一链所必需的零至多个非自签中间证书，不含根、CRL、重复或无关证书，并按 DER SET 规范顺序编码。leaf 必须同时具有 Digital Signature KeyUsage 与 Code Signing EKU；SignerInfo 签名算法只允许 ECDSA P-256/SHA-256（parameters 缺省）或 RSA-PSS/SHA-256（RSA modulus 至少 3072，MGF1-SHA256、saltLength=32、trailerField=1）。链中每张 certificate 的 SubjectPublicKeyInfo 也只有两种合法 profile：ECDSA 必须是 `id-ecPublicKey` 加 parameters=`prime256v1` named-curve OID，public point 恰为 65-byte uncompressed `0x04||X||Y` 且在曲线上，拒绝 explicit parameters/压缩点；RSA 必须是 `rsaEncryption` 加显式 DER NULL，modulus 至少 3072 bits、publicExponent 恰为 65537，拒绝 `id-RSASSA-PSS` SPKI 与其他参数/指数。其 SPKI token 必须与封套 `signer_subject` 逐字相等且命中已验签 `DeploymentManifestV1.license_trusted_signer_subjects`，并离线验链和撤销状态到下述固定 `license-roots.p7b`。不得读取 Windows 当前用户或本机任意根存储，不得联网补链，不接受 DEV/file key、命令行公钥或临时生成信任根。

外层配置包签名与本节内层 CMS 必须分别通过；外层签名不能替代内层签名。唯一例外是第 4.2 节“已接受 module inner signer 后来被 CRL 吊销”的 DISABLE 恢复：旧 inner 只能作为逐字自洽的已接受停用目标，当前授权来自 ACTIVE outer，绝不能作 ENABLE/安装/升级/回退的正向证明。普通配置包继续由既有部署 KMS 完成 `actions/sign`；F-56 imported special package 则在 import 时保存发行方 outer `signature/signer_subject/signed_at` exact bytes，批准后的 `actions/sign` 只复验并推进状态，绝不以部署 KMS 覆盖。内层封套始终原样保存在 `config_package_items.after_spec`，所以两层证据不会互相覆盖。

F-56 special `.epcfg` 的文件格式同时冻结，不能交给实现者选择。它是一个总长不超过 **4,193,900 bytes**、只使用 ZIP32 `STORE` 的单卷 archive；该上限与第 6 节最多 404-byte multipart framing 相加恰不超过 4,194,304-byte route body limit。local-header 顺序与 central-directory 顺序都必须逐字为 `manifest.toml`、`item.jcs`、`outer-signature.p7s`，entry 集合除此三项外为空；三个 ASCII 文件名长度依次为 13、8、19。每个 local header 固定 `version needed=20,general-purpose flags=0,compression method=0,DOS time=0x0000,DOS date=0x0021`，CRC-32、compressed size 与 uncompressed size 取该 entry 实际 bytes，filename length 取上述值，extra length=0，随后立即是文件名与 exact content。每个 central header 固定 `version made by` raw u16=20（MS-DOS host）、`version needed=20,flags=0,method=0,time=0x0000,date=0x0021`，CRC/两项 size/filename length 与 local 相等，extra/comment length=0、disk start=0、internal/external attributes=0，local-header offset 取从 archive byte 0 起的实际 u32 offset。唯一 EOCD 固定 disk/current-start-disk=0、两项 entry count=3、central size/offset 取实际 u32、comment length=0；EOCD 结束即 EOF。禁止 ZIP64、加密、data descriptor、extra field、archive/entry comment、目录项、重复名、大小写碰撞、绝对/反斜杠/`.`/`..` 路径、symlink/hardlink/reparse 属性、尾随数据或嵌套 archive。三 entry 都是 regular file；三 entry 的固定 ZIP header/directory overhead 恰为 330 bytes，任一 header bit、顺序、offset、CRC、size 或结束位置偏离都在落库前整包拒绝。`item.jcs` 最大 **2,882,850 bytes**，是唯一 item 的 `after_spec` 经过 RFC 8785 JCS 后的 exact bytes，故其 `item_hash=SHA-256(item.jcs exact bytes)` lowerhex；它与 262,144-byte manifest、1,048,576-byte outer CMS 和 330-byte ZIP overhead 的最大值总和恰为 archive 上限。payload-JCS 与 decoded inner CMS 各自的 1,048,576-byte上限保持独立，base64url 封套仍须落入本 whole-item 上限。special 固定 ADD，所以不存在空 after。全平台普通 item 算法保持兼容但补齐 REMOVE：ADD/MODIFY 的 `item_hash=SHA-256(JCS(after_spec))`，REMOVE 的 `item_hash=SHA-256(JCS(before_spec))`，不得对 null 求摘要。item kind/code/change/sort/scope/before-null 由下述已签名 manifest 逐项绑定；MODULE_PACKAGE action/reason 已在 `after_spec` 内，故内容和元数据都进入 outer 签名闭包而无需新增数据库 digest 列。

`manifest.toml` 最大 262,144 bytes，是 UTF-8 无 BOM、LF-only、结尾恰一个 LF 的 canonical TOML；禁止注释、tab、CR、重复/未知键与 Unicode normalization。root key 顺序恰为 `schema_version,purpose,package_no,name,package_version,min_platform_version,source,item_count,signer_subject,signed_at`，值分别要求 `1,"EP-F56-SPECIAL-CONFIG-OUTER-V1",1..128-byte string,1..256-byte string,canonical dotted SemVer,canonical dotted SemVer,"IMPORTED",1,1..512-byte string,UTC-second timestamp`；dotted SemVer 恰为三个 `u16` 的无前导零十进制以 `.` 连接，不含 prerelease/build。随后恰一个空行与一个 `[[items]]`，其键顺序恰为 `path,item_kind,item_code,change_kind,sort_no,applies_to_legal_entity_ids,before_spec_is_null,item_hash`，其中固定 `path="item.jcs"`、`change_kind="ADD"`、`sort_no=1`、`applies_to_legal_entity_ids=[]`、`before_spec_is_null=true`，其余值逐字等于 item 与重算 digest。每行只允许 `key = value` 的单空格格式；整数用无正号/前导零十进制，字符串使用 TOML basic string、原 Unicode scalar 与最短必要转义。解析后必须由 canonical writer 重发出并与输入 exact bytes 相等。`content_hash=SHA-256(manifest.toml exact bytes)`；import 只保存已解析 manifest 与 item 行，后续 `actions/sign` 必须从这些列与 `signer_subject/signed_at` 通过同一个 canonical writer 重建 exact manifest bytes、重算同一 hash 后再验签，禁止依赖通用 TOML reserialize 或已删除的 staging 文件。

`outer-signature.p7s` 是最大 1,048,576 bytes 的单个完整 DER `ContentInfo`，不是裸 `SignedData`；detached content 恰为 `manifest.toml` exact bytes，CMS `messageDigest` 必须等于 `content_hash`。它逐项复用上段 ContentInfo/SignedData/SignerInfo/sid/digest/certificate-set/SPKI/算法/属性闭集、signedAttrs universal-SET preimage 与 unsigned-attribute/trailing-byte 禁令，唯一时间差异是 `signingTime` 与 manifest `signed_at` 语义上为同一 UTC whole-second instant，并复用同一 DER 时间编码规则。Code Signing EKU、离线链/CRL、ACTIVE/RETIRED/REVOKED、signed deployment roster 与固定 `license-roots.p7b` 规则同样复用，但 outer 和 inner 必须各自独立验签；每一份尚未首次 RELEASE 的 special outer signer 在所有推进关口都必须为 ACTIVE。leaf 的 `spki-sha256:` token 必须逐字等于 manifest `signer_subject`；CMS DER exact bytes 写入 `signature`，这两个 manifest 值分别写入 `signer_subject/signed_at`，`signature_key_ref=null`，不得另信任上传请求自报值。这样普通包的部署 KMS outer、special publisher outer 与 inner artifact 三条 verifier/前像明确区分；只有 special outer 与 inner CMS 共享发行方离线 trust bundle，三者仍不能互相替代。

本节内层 CMS 的生产验签唯一信任包为 `C:\ProgramData\EnterprisePlatform\trust\license-roots.p7b`；它不替代普通配置包既有的外层 KMS verifier。该文件 exact bytes 最大 1,048,576，格式固定为单个完整 DER `ContentInfo`：`contentType=signedData`、`content=[0] EXPLICIT SignedData`、输入末尾零 trailing byte；`SignedData.version=1`，`digestAlgorithms` 为空 DER SET，`encapContentInfo.eContentType=id-data` 且 `eContent` 缺省，`signerInfos` 为空 DER SET，certificates 为 1..64 张 CA 证书、crls 为 1..256 份完整 base CRL并各自按 DER SET 规范顺序编码，零其他内容。这是唯一允许的 degenerate SignedData 形状；不含 leaf、私钥、URL、脚本或可执行正文。自签且自验签成功、符合下述首版 CA extension profile 的证书才是 trust anchor；其余证书必须为非自签中间 CA 且符合同一 profile。至少一张 anchor，证书 DER/SKI 与 CRL issuer+CRLNumber 均不得重复或冲突；cross-sign 或任意内容若形成零条或多条候选链仍按下一段失败。该文件必须同时命中待发布 `MANIFEST.sha256` 与安装后 readback digest。owner SYSTEM、关闭继承，显式 DACL 只有 SYSTEM/Administrators/`NT SERVICE\ep-ops` 完全控制，`NT SERVICE\ep-core` 与 `NT SERVICE\ep-worker` 只读/`READ_CONTROL`，其余无 ACE。

signer 状态不是只看 leaf，而是从唯一完整链导出。`signed_time` 对 outer 只取 manifest/CMS 一致的 `signed_at`，对 inner 只取 signed payload/manifest 的 `issued_at`，两者不能交叉替代。链中 **non-anchor** 恰为 leaf 加零至多个 intermediate；每张 non-anchor 都必须在 `signed_time` 落入自身 `[notBefore,notAfter]`，否则该 artifact 从未有效并直接为 UNTRUSTED。当前 bundle 中的自签 anchor 是显式配置的信任 datum：它必须在 `signed_time` 有效并通过自签名、CA/KeyUsage/critical-extension 检查；其 `notAfter` 后续越过 `trusted_now` 本身不把既有链变成 RETIRED，也不读取“上级 CRL”，但 anchor 从签名 bundle 被移除、替换或形成多链时立即 UNTRUSTED，只能走 CAB 信任轮换/可信恢复，不能借 RETIRED 或 REVOKED 窄路径。

整条 non-anchor 链的四态分类只在证书结构、唯一 path、signed-time 与**全部实际 issuer 的 CRL prerequisite** 成功后开始，分类内优先级固定为 `REVOKED > ACTIVE > RETIRED`；上述任一 prerequisite 失败直接是 `UNTRUSTED`，不参与该优先级。只有每个实际 issuer 都已按下一段建立唯一、覆盖 `trusted_now` 的 global-highest CRL 后，才扫描全部 non-anchor serial；任一命中即整条链 REVOKED，不论 revocationDate、该证书是 leaf 还是 intermediate、或历史签发时间。若一个 issuer 已见撤销命中但另一个 issuer 的 CRL 缺失、过期、尚未生效、同最高号冲突或非法，结果仍必须是 UNTRUSTED，禁止把局部命中提升为 REVOKED 或进入窄恢复。全链无命中且每张 non-anchor 在 `trusted_now` 也都位于有效期内才是 ACTIVE，可接受全新 artifact；无命中、全部在 `signed_time` 有效、到 `trusted_now` 至少一张 non-anchor 已过 `notAfter` 且没有任何一张尚未到 `notBefore` 时才是 RETIRED。RETIRED 只可复验“首次接受时整条链=ACTIVE、accepted-trust/source/digest/signature 仍逐字自洽”的既有 RELEASED current 或 history artifact，不接受新 import/release；其余包括当前尚未生效、约束失败、零链/多链都为 UNTRUSTED。更新只能来自同一 Authenticode 签名离线发布 CAB，在许可/模块发布 gate 关闭且 core/worker 停止的维护窗口内 staging 验证、write-through 原子替换、重启 readback；禁止在线下载、Windows 任意根、单证书临时覆盖、命令行 root 或运行中热替换。运行中 identity/digest 漂移立即关闭许可/模块变更 gate；current grant 进入 `RESTRICTED/SIGNATURE_INVALID`，current module 只关闭自身 effective runtime，但导出、安全、许可恢复与合规处置仍可用。

离线 path/CRL 选择算法同样是闭集。验签只能形成一条从 CMS leaf 经 CMS/bundle intermediate 到 bundle trust anchor 的密码学有效链；零条或多条候选链、证书签名或下述 exact extension profile 任一不成立都失败。首版明确拒绝任一证书出现 `nameConstraints`、`certificatePolicies`、`policyMappings`、`policyConstraints` 或 `inhibitAnyPolicy`，不论 critical/noncritical；需要这些能力必须升级 schema/profile，不能交给不同库默认解释。leaf extension exact-set 只允许且要求 noncritical SKI、noncritical AKI（keyIdentifier 唯一形态且匹配 issuer SKI）、critical KeyUsage（唯一 bit 为 digitalSignature）、noncritical EKU（唯一 OID 为 codeSigning），BasicConstraints 只可缺省或为 critical `CA=false,pathLen absent`；CA extension exact-set 只允许且要求 noncritical SKI、noncritical AKI（anchor 的 keyIdentifier=自身 SKI，intermediate 匹配 issuer SKI）、critical BasicConstraints（`CA=true`，pathLen 可空或非负且实际 enforce）与 critical KeyUsage（唯一 bits 为 keyCertSign+cRLSign），不得带 EKU。任一未列 extension、错 critical 位、额外 KU/EKU bit/OID 或未知 extension 都拒绝。链中每张证书以及每份 CRL 自身的签名 `AlgorithmIdentifier` 也只能是 ECDSA-with-SHA256（issuer key=P-256、parameters absent）或 RSA-PSS/SHA-256（issuer RSA modulus≥3072、hash=SHA256、MGF1-SHA256、saltLength=32、trailerField=1）；SHA-1、RSA PKCS#1 v1.5、隐式/default/NULL 参数或任何其他组合一律失败。链上每个实际签发 non-anchor 的 issuer 都必须在 bundle 内有 X.509 v2 **完整 base CRL**：CRL issuer DER Name、唯一 required noncritical AKI keyIdentifier 必须分别匹配 issuer certificate 的 subject DER Name、SKI，CRL 签名必须由该 issuer 公钥按上述闭集验证；CRL extension exact-set 除 AKI 外只允许 required noncritical CRLNumber，并必须有 nextUpdate，禁止 IssuingDistributionPoint、deltaCRLIndicator、freshestCRL、indirect/delta CRL 与任何 revoked-entry extension。entry 因而只含 serial 与 revocationDate；不接受 reasonCode/removeFromCRL 等别名。对每个 issuer 的顺序固定为 **global-highest-then-cover**：先枚举 bundle 内该 issuer 全部结构与签名合法的完整 base CRL，按 numeric CRLNumber 选择全局最高号；该最高号必须只有一份 exact DER，随后才要求它满足 `thisUpdate<=trusted_now<=nextUpdate`。全局最高号缺失必要字段、过期/尚未生效、同号出现不同 DER，或根本没有合法完整 base CRL，都按 `SIGNER_NOT_TRUSTED` 归入 UNTRUSTED；即使某个更低号仍覆盖 `trusted_now` 也绝不回退。实现必须先为整条链所有实际 issuer 完成这份 registry，任一失败立即返回 UNTRUSTED且不得扫描任何 serial；只有 registry exact-complete 后才逐张以 certificate serial exact integer 查对应 global-highest CRL，任一命中即按上段归整条链 REVOKED。不读取 CRL Distribution Point、不查 OCSP、不联网，也不接受库默认软失败。

实现依赖不在架构文档锁死为某个 Rust crate API；行为由仓库 golden conformance fixtures 锁死，至少逐字覆盖 SignedData/SignerInfo DER、signedAttrs 的实际签名 preimage、SKI sid、全部 AlgorithmIdentifier 参数、UTCTime/GeneralizedTime 边界、leaf/intermediate/anchor path、上述 leaf/CA/CRL extension exact-set、五类 policy/name-constraint 扩展拒绝、完整 base CRL 最高覆盖选择、整链 ACTIVE/RETIRED/REVOKED/UNTRUSTED 与全部反例。该 golden fixture set 是 verifier 实现前必须先提交并跑红的开发输入，不是实现完成后再选择的口径。最终选择的已审计 crate/version 只由 `Cargo.lock` 与 SBOM 固定；任何库默认 OS trust、联网、软失败或宽松 DER 行为都必须由适配层关闭并以负夹具证明。

每个 RELEASED special item 的 `config_package_items.accepted_trust_bundle_sha256` 只记录该 item **首次成功 RELEASE、进入运行投影时**使用的 bundle exact-byte 摘要，作为不可改写的审计来源；未发布 special 与全部普通 item 必须为 null，成功发布事务才允许一次 `null→32 bytes`，之后禁止改写或清空。grant 行的 `trust_bundle_sha256` 必须等于其 grant source item 的该值；revocation 与每个 MODULE_PACKAGE action 的接受摘要直接取各自不可删除 source item，不另造会在升级时丢失历史的 current-only 摘要列。合法轮换不回填这些旧值。

摘要不能替代可重放的原始信任包。安装器在首次治理前、`ep-ops` 在每次轮换替换 current 文件前，都必须把当前 `license-roots.p7b` exact bytes 以摘要命名保存到 `C:\ProgramData\EnterprisePlatform\evidence\license-trust-bundles\<64-lowerhex>.p7b`；CREATE_NEW（同名已存在则只读逐字相等）、`FlushFileBuffers`、关闭后 safe-handle readback，文件名摘要必须等于 exact bytes SHA-256。目录 owner SYSTEM、DACL PROTECTED，显式 inheritable allow ACE 只有 SYSTEM/Administrators/`NT SERVICE\ep-ops` FullControl 与 `NT SERVICE\ep-core`/`NT SERVICE\ep-worker` 的 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余无 ACE；拒绝 UNC/device/reparse/ADS/hardlink/8.3/case drift，文件不得覆盖、截短或自动清理，并进入 Stage 14 备份与恢复 exact-set。任何 special RELEASE 只有在该 digest 文件已经存在、readback exact 且与 current bundle 相等时才可进入数据库事务；外部归档失败没有“随数据库回滚”的伪原子性，只留下无害、按摘要去重的孤立 bundle 文件，后续重试复用它。

每个 special 首次 RELEASE 与投影、接受摘要同一事务追加唯一审计 `action='platform.config_special.accepted.v1'`；完整 envelope（冻结治理法人、execute SecurityContext actor/device/client、config item object/version、approval_ref、null 证据列、accepted time 与 AuditWriter 派生链列）逐字采用 Stage 3 的同名冻结，payload 是下列闭集 strict-JCS（unknown/missing key 失败）：`{schema_version:1,purpose:"EP-CONFIG-SPECIAL-ACCEPTED-V1",config_package_id,config_item_id,artifact_kind,artifact_id,artifact_action,accepted_trusted_now,accepted_trust_bundle_sha256,inner_signer_subject,inner_chain_sha256,inner_trust_state,outer_signer_subject,outer_chain_sha256,outer_trust_state,payload_sha256,item_hash,content_hash,source_projection_sha256}`。这里及下述两个 F-56 具名 typed-audit DTO 的 `schema_version` 都是 JSON number `1`，不是字符串 `"1"`；Stage 3 对无具名 typed ABI 的普通业务数值采用 canonical string 的通则不得覆盖它们。UUID 为小写 canonical，时间为 UTC whole-second，digest 为 64 lowerhex；`artifact_kind` 恰为 `LICENSE_GRANT|LICENSE_REVOCATION|MODULE_PACKAGE`，`artifact_action` 对前两类为 JSON null、模块类为五个 action wire 之一；`artifact_id` 分别取 grant/revocation/package UUID。outer state 必须为 `ACTIVE`；inner state 只允许 `ACTIVE|RETIRED_NONREVOKED|REVOKED_AS_DISABLE_TARGET`，后两值分别只用于第 2.2/4.2 节已接受 inner 复用与唯一 CRL-DISABLE 窄路径，新 GRANT/REVOCATION/INSTALL/UPGRADE 必须为 ACTIVE。`*_chain_sha256=SHA-256("EP-CMS-CHAIN-V1" ASCII || 0x00 || 对 leaf→intermediate→anchor 每张 exact DER 依次追加 u32 big-endian 长度和 DER bytes)`；signer token 与链 leaf SPKI 必须相等。

上述 `source_projection_sha256` 的唯一算法为 `SHA-256("EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1" ASCII || 0x00 || JCS(dto))`，其中 `dto` 是下列 terminal projection 闭集：`{schema_version:1,purpose:"EP-CONFIG-SPECIAL-SOURCE-PROJECTION-V1",config_package_id,package_no,source:"IMPORTED",status:"RELEASED",content_hash,outer_signature_sha256,outer_signer_subject,outer_signed_at,config_item_id,item_kind,item_code,change_kind:"ADD",sort_no:1,applies_to_legal_entity_ids:[],before_spec_sha256:null,after_spec_sha256,item_hash,accepted_trust_bundle_sha256}`；`outer_signature_sha256` 对保存的 DER outer signature bytes 求 SHA-256，`after_spec_sha256=SHA-256(item.jcs exact bytes)`。审计 hash chain、该 payload、数据库 terminal projection、不可变 accepted bundle bytes 与 inner/outer exact CMS 必须可互相重算；缺任一项时不得把该对象视为“首次 ACTIVE 接受”或打开共同发布门。

计划轮换必须由上述 CAB 同步更新签名部署清单，在 gate 重开前用新 bundle 对 **全部 RELEASED LICENSE_GRANT item（grant 与 revocation）和全部 RELEASED MODULE_PACKAGE item** 的 special outer 与 inner 分别重新验链/验 CRL，并把 current grant/current revocation/current module projection 与其 source item交叉核对，保存“旧接受摘要、新验证摘要、对象 id、outer 结论、inner 结论、总结果”的签名 exact-set 证据。唯一 current grant 或命中它的 revocation 的 inner 和/或 source special outer 复验失败才决定 deployment-level `LicenseStatus=RESTRICTED/SIGNATURE_INVALID`；当前安装模块的 inner 和/或其 current source special outer 复验失败只关闭该 module 的 effective runtime admission 并告警，绝不反向改写全局 LicenseStatus。历史 inner 或 special outer signer被新 CRL明确命中时，`HISTORICAL_SIGNER_REVOKED` 是正确的 accepted-containment 结论：只要该行仍与首次接受审计、当时 exact bundle、source/payload/digest/signature/projection 全部自洽，并且独立 current 的每一层在当前 bundle 下为 ACTIVE，或为已有 RELEASED artifact 的 RETIRED-nonrevoked 且能回指首次 ACTIVE 接受证据，该历史分类本身不阻断共同 gate；历史物保留并隔离，永不再计入 `purchased`、rollback candidate 或任何正向许可/模块证明。只有分类与实际 CRL 不等、接受链不完整，或历史对象发生断链、source/digest/signature 漂移、结构损坏等非 CRL 漂移时，运行中的独立有效 current 不被倒推改写，但许可/模块变更门与 `RG-LICENSE-MODULE-LIFECYCLE-GREEN` 保持关闭，只能用可信备份/证据恢复后重开。非计划文件或部署清单摘要漂移仍立即使 current grant 与受影响 module 各自失败关闭，不得退回旧 bundle、旧接受摘要或系统任意根。

### 2.2 包形状约束

含 `LICENSE_GRANT` 或 `MODULE_PACKAGE` 的配置包必须满足：

- `source=IMPORTED`；
- `item_count=1`；
- `change_kind=ADD`，`before_spec=null`，`after_spec` 非空；
- `applies_to_legal_entity_ids=[]`，法人范围只能来自已签名许可 payload，不能由外层数组覆盖；
- 只能创建 `action=RELEASE` 的发布单，不能创建 `ROLLBACK`；
- 仍完整经过既有九套自动测试、`CONFIG_RELEASE` 双人职责分离审批、外层签名与发布执行。

签名状态在生命周期中不作一次验过永久沿用：import、autotest、submit、approve、special `actions/sign`、create-release-order 与 execute 每次都从持久化 exact bytes 和当前 bundle 复验，reject 只允许对同一不可变 content hash 形成拒绝结论而不要求 artifact 仍可正向发布。任何推进关口和首次 RELEASE 的 special outer 都必须为 ACTIVE。首次引入的 GRANT、REVOCATION、INSTALL 与新版本 UPGRADE inner 也必须为 ACTIVE；ENABLE/DISABLE 只能原样复用当前已安装包的既有 RELEASED exact inner，ROLLBACK_VERSION 只能原样复用一个既有 RELEASED 历史版本的 exact inner，这三类复用 inner 可为 ACTIVE 或 RETIRED-nonrevoked，但必须能唯一追溯到首次以 INSTALL/UPGRADE、signer=ACTIVE 接受的 origin item 及其不可改写接受摘要。DRAFT、未 RELEASED、REVOKED、隔离历史或只有相同 identity 而非 exact bytes 的 artifact 都不算“既有”。第 4.2 节 current signer CRL 的 DISABLE 窄例外是唯一偏离；它仍要求本次新 outer 为 ACTIVE。

`LICENSE_GRANT|MODULE_PACKAGE` special package 的 `RELEASED` 是永久终态：首次 RELEASE 后不得进入普通配置包的 `SUPERSEDED` 或 `ROLLED_BACK`，后续许可、撤销或模块动作各自新增另一份仍为 RELEASED 的单项包。旧/新 current、history、superseded grant 与 current module 只由 `license_grants/module_registrations` 投影及其 source FK 表达，不能借改 config package status 表达。Stage 13 的普通 `RELEASED→SUPERSEDED` 自动边必须显式排除 special；多个 special RELEASED 同时存在是正确历史形状。这样每个 source 的 terminal projection 永远保持 `status="RELEASED"`，首次接受摘要非空且不可清，CAB/Stage 14 才能对全部 RELEASED special exact-set 重算。任何 special 为 SUPERSEDED/ROLLED_BACK、RELEASED 摘要为空、非 RELEASED 摘要非空或尝试清摘要，均由同一 deferred graph 在提交点拒绝。

错误映射是闭集：transport/multipart、ZIP32 结构与 entry/CRC/offset/size、canonical TOML/JSON/base64 的语法或硬上限在形成 typed DTO 前失败，统一使用既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`（HTTP 400、不可重试、零落库）；外层 item hash 不等使用既有 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH`；外层或内层 CMS 的 DER/摘要/签名算法/证书有效期或密码学验证不成立使用既有 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID`；链、CRL、EKU、subject 或 release root 不受信使用既有 `PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED`；成功 strict parse 后的特殊包形状、inner payload 语义、metadata/deployment/scope/日期绑定、许可直接后继或撤销目标不成立使用 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`；模块仅在签名与信任已通过后的许可、状态边、产品版本/contract/维护权/history identity/兼容性失败，才分别使用第 4.2 节三种模块码。对含这两类内容项创建通用回退单使用 `PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM`。两项 F-56 配置专码均为 409、不可重试，任一路径都不得退化为部分发布或临场新增同义码。

## 3. 许可证终态

### 3.1 已签名 payload

`LICENSE_GRANT` 的 payload 是以下 internally tagged union，wire 形状固定为 `{"artifact_kind":"GRANT",<grant 的其余字段>}` 或 `{"artifact_kind":"REVOCATION",<revocation 的其余字段>}`；不得使用 Rust 默认的外部 tag、额外 `payload` 包裹或仅凭字段猜分支。所有 enum 的 JSON wire 都取下文大写下划线值，日期固定 `YYYY-MM-DD`，时间固定 UTC 秒精度 `YYYY-MM-DDTHH:MM:SSZ`，UUID 固定小写 canonical hyphenated。`purpose` 同时承担签名域分离：

```rust
pub enum LicenseArtifactPayloadV1 {
    Grant(LicenseGrantPayloadV1),
    Revoke(LicenseRevocationPayloadV1),
}

pub enum LicenseKindV1 { Perpetual, Subscription }
pub enum LegalEntityScopeV1 { All, List }
pub enum EntitlementCodeV1 { F55LocalAi, F55Mcp }

pub struct LicenseUsageLimitsV1 {
    pub legal_entity_limit: u32,
    pub named_user_limit: u32,
    pub registered_device_limit: u32,
}

pub struct LicenseGrantPayloadV1 {
    pub schema_version: u16,                 // exact 1
    pub purpose: String,                    // exact "EP-LICENSE-GRANT-V1"
    pub grant_id: Uuid,
    pub license_no: String,                 // 1..128 UTF-8 bytes
    pub deployment_id: Uuid,
    pub governance_legal_entity_id: Uuid,
    pub issued_to: String,                  // 1..256 UTF-8 bytes
    pub issued_at: DateTime<Utc>,           // UTC seconds
    pub license_kind: LicenseKindV1,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub maintenance_valid_to: Option<NaiveDate>,
    pub legal_entity_scope: LegalEntityScopeV1,
    pub legal_entity_ids: Vec<Uuid>,
    pub limits: LicenseUsageLimitsV1,
    pub module_codes: Vec<ModuleCode>,
    pub entitlement_codes: Vec<EntitlementCodeV1>,
    pub supersedes_grant_id: Option<Uuid>,
}

pub struct LicenseRevocationPayloadV1 {
    pub schema_version: u16,                 // exact 1
    pub purpose: String,                    // exact "EP-LICENSE-REVOCATION-V1"
    pub revocation_id: Uuid,
    pub deployment_id: Uuid,
    pub grant_id: Uuid,
    pub license_no: String,
    pub issued_at: DateTime<Utc>,           // UTC seconds
    pub reason_code: String,                // CONTRACT_ENDED|REISSUED|COMPROMISED|CUSTOMER_REQUEST
}
```

`module_codes` 为 1..15 项，按 `ModuleCode` wire bytes 排序去重；`entitlement_codes` 为 0..2 项，按 wire bytes 排序去重。`ALL` 要求 `legal_entity_ids=[]`；`LIST` 要求 1..1024 个 UUID 且按 UUID bytes 排序去重，并且必须包含 `governance_legal_entity_id`。三个 limit 均为 1..1,000,000，属于商业计量上限而不是认证规模。`SUBSCRIPTION` 要求 `valid_to>=valid_from` 且 `maintenance_valid_to=valid_to`；`PERPETUAL` 要求 `valid_to=null`，`maintenance_valid_to` 可空或不早于 `valid_from`。首张 grant 的 `supersedes_grant_id=null`；续期/换证必须指向当前 grant，并在同一事务把旧行移出 current slot、新行进入 current slot。部署在首张 grant 接受前允许零个 current，接受后必须恰有一个；数据库与查询契约始终只允许至多一个，零个映射 `RESTRICTED/NO_CURRENT_GRANT`，多于一个或 current slot/投影异常映射 `RESTRICTED/SIGNATURE_INVALID`。撤销仍保留该 current 行，但立即转受限运行。

`governance_legal_entity_id` 是部署级许可/模块配置治理的唯一法人，不是模块安装范围。首张 RELEASED GRANT 冻结该部署的值；后续所有 GRANT 必须逐字相同，首版不提供原地变更，改变它必须建立新 deployment。该 UUID 在首张 submit 前必须指向已经存在且 active 的 `platform_core.legal_entities` 行；作为治理法人期间禁止停用或删除。每个 F-56 special 推进命令都必须先派生不可覆盖的 `governance_context_id`：首张 grant 从候选 signed payload 取值，其后 grant/revocation/module action 从首次 RELEASED grant history 取值；当前已鉴权 session/operator 必须在该法人具有本动作所需授权，请求头若存在只能与派生值相等，当前浏览法人、ServerAdmin 选择、配置或环境变量都不能覆盖。为兼容既有 `ck_config_packages_approval_shape`，`DRAFT|PENDING_AUTOTEST|TEST_FAILED|TEST_PASSED` 的 `approval_legal_entity_id` 必须为 NULL；submit 事务才首次把它写为派生值，`PENDING_APPROVAL` 及以后所有状态均由 deferred graph 强制它逐字等于冻结治理法人。申请人、不同审批人及执行人仍按该法人现有授权与 `CONFIG_RELEASE` 职责分离判定。若首次 grant history 的该字段、source 或签名证据不唯一或损坏，所有 special 推进失败关闭，只能从可信备份恢复；不得任选另一个法人绕过。

零 current 首装不能假设数据库里已经有人可审批，唯一自举入口因此冻结为既有五子命令工具的窄参数形态：`ep-migrate apply --initial-governance-bootstrap=<bootstrap.jcs> --initial-license-package=<license.epcfg> --receipt-out=<directory>`。它不是第六个子命令，不新增运行端点、服务、监听、数据库表或迁移；只允许在 fresh production apply 完成全部迁移后、九个常驻进程尚未开放 public readiness 时，由 Authenticode 验证通过且 PE digest 命中当前签名产品清单的 `ep-migrate` 以既有迁移账户执行。生产 fresh install 必须提供三参数；只有显式 non-production 的开发/测试空库可省略，此时必须同时满足 zero current、zero legal-entity、无 bootstrap audit/receipt，运行态固定 `RESTRICTED/NO_CURRENT_GRANT`、可信 checkpoint worker dormant，且永远不能生成 Stage 14 生产发布证据或 PASS。三个 path 不是任意路径：解析 signed deployment id 后，唯一目录为 `C:\ProgramData\EnterprisePlatform\evidence\stage14\initial-governance\<lowercase-deployment-id>\`，前两者必须分别逐字归一到其下 `bootstrap.jcs` 与 `license.epcfg`，`receipt-out` 必须逐字归一到该目录；输出文件名固定 `initial-governance.receipt.v1.jcs`，目录段 id 必须逐字等于 manifest/bootstrap/license/receipt 的 canonical deployment id。该目录 owner=`NT AUTHORITY\SYSTEM`、关闭继承/PROTECTED；显式 inheritable allow ACE 仅 `SYSTEM`、`BUILTIN\Administrators`、`NT SERVICE\ep-ops` 为 FullControl，`NT SERVICE\ep-core` 为 `FILE_GENERIC_READ|FILE_TRAVERSE|READ_CONTROL|SYNCHRONIZE`，其余账户无 ACE；ep-core 尤其没有 write/delete/WRITE_DAC/WRITE_OWNER。三条路径均用 fixed-root safe handle，拒绝 UNC/device/reparse/ADS/hardlink/8.3/case drift。输入必须已在固定名就位；receipt 只能 CREATE_NEW、`FlushFileBuffers`、关闭后 safe-handle readback，不能覆盖或写 sidecar。

`bootstrap.jcs` 最大 1,048,576 bytes、strict RFC 8785 JCS，exact root 为 `{body,authorizations}`。`body` exact 字段为 `schema_version=1,purpose="EP-INITIAL-GOVERNANCE-BOOTSTRAP-V1",bootstrap_id,deployment_id,deployment_manifest_sha256,initial_license_archive_sha256,issued_at,expires_at,legal_entity,operators`；时间 UTC 秒精度，`0<expires_at-issued_at<=24h` 且执行时在闭区间内。`legal_entity` exact 字段为 `{id,key_domain_id,code,entity_no,name,short_name}`；两个 id 都是 canonical UUID，`id` 必须等于候选 GRANT 的 `governance_legal_entity_id`，`key_domain_id` 必须全库未占用且成为下述 PROVISIONING 行的逐字主键；entity_no 为两位数字，其余按 `legal_entities` 既有列限，时区/币种固定走现有 `Asia/Shanghai/CNY` 默认。`operators` 恰两项，按 `bootstrap_role` bytes 排序且角色集合恰为 `CONFIG_OPERATOR|SECURITY_APPROVER`；每项 exact 为 `{bootstrap_role,user_id,login_name,employee_no,display_name,client,device_id,signer_subject}`，其中 user/device 都是 canonical UUID；signed `device_id` 同时成为 `user_devices.id`，其 lowercase canonical 文本成为该行外部 `device_id`，receipt 与 Stage 14 的 `device_ids` 都指行 id，不允许再生成第二个设备 UUID。两个 user/login/device/SPKI token 全部互异，client 只取 `win|mac`。`authorizations` 也恰两项、同序同角色，每项 exact 为 `{bootstrap_role,signer_subject,signature_cms_b64url}`；detached content 恰为 `JCS(body)`。两名 signer 必须分别等于对应 operator、彼此不同，并各自逐字命中 Stage 14 签名部署清单 `customer_security_admin_certificates` 的不同 ACTIVE roster entry；entry exact 为 `{certificate_sha256,signer_subject,subject_key_identifier_b64url}`，分别绑定 CMS leaf exact DER SHA-256、SPKI token 与 SKI raw bytes。每张 leaf 必须 DigitalSignature+ClientAuth、不得 CodeSigning，并同时形成唯一有效链到产品保护的 `deployment-roots.p7b` 和 `EP__AUTH__X509__TRUST_ANCHOR_REF` 为 ep-migrate recipient 解出的登录 CA/完整 base-CRL bundle；后者 exact-byte SHA-256 必须等于清单 `x509_login_trust_bundle_sha256`。两个 bundle 都只按清单 CMS/整链/CRL/算法/时间闭集验证，不能读取 Windows 任意根、联网补链或接受命令行根。body digest、两签、roster/部署清单/任一信任包摘要或顺序任一偏离均退出 78、零 bootstrap 写入。

工具在任何写入前还须 safe-handle 读取并完整验证 `license.epcfg` 的第 2 节 container、ACTIVE inner/outer、首张 GRANT（`supersedes_grant_id=null`）、deployment/governance/scope 绑定，并要求 archive SHA-256 等于 body 值；只验证，不代替后续应用内 import/审批/release。fresh 数据库前置 exact 为：`legal_entities` 零行；`user_accounts` 恰一行且 id/account_kind 分别为既有 `SYSTEM_PRINCIPAL_ID/SYSTEM` seed；`user_credentials`、`user_password_history`、`user_devices`、`sessions`、`reauth_challenges`、`login_attempts`、`account_lockouts`、`breakglass_activations` 零行；法人授权、角色、角色授权、用户角色/范围/组织绑定、审批链/节点、高风险请求与 authz config 等法人 authz 业务行零行；`key_domains/data_keys` 法人密钥域业务行零行；license grant 与 config package/item/order 零行。`permission_items`、`object_scope_bindings` 及其余 SYSTEM、deployment projection、schema/migration history seed 必须逐项等于当前签名 schema/product 清单；其中 Stage 13 固定管理 API 的 30 行权限项、12 行范围锚必须逐字等于 `V20261022090600__platform_meta_config_release.sql` 在 Stage 13 计划中冻结的 ID、code、action、object type 与真实表映射，不能把“首装只使用其中 9 个 code”误写成只需建立 9 行目录。迁移不预授任何业务角色；下面两角色的 10 条 grant 只由本 bootstrap 事务创建。任一应为空对象非空、目录缺项、额外项或字段漂移都退出 78，不提供 `--force`、更新、删除或任选已有行模式。

唯一 bootstrap PostgreSQL 事务只创建候选指定 active 法人、以 signed `key_domain_id` 为主键且 `domain_kind=LEGAL_ENTITY,state=PROVISIONING` 的密钥域、两名 ACTIVE/`is_mfa_required=true` 的 EMPLOYEE、各一条受限于治理法人的 ACTIVE Win/Mac device、各一份 console password credential 与一份来自对应 CMS leaf 的 X509 credential；随后必须先创建恰三条 `platform_authz.user_legal_entity_grants`，映射分别为 `(治理法人,SYSTEM_PRINCIPAL_ID)`、`(治理法人,CONFIG_OPERATOR user_id)`、`(治理法人,SECURITY_APPROVER user_id)`，三行 `granted_from` 都取本事务 `committed_at` 的 UTC date、`granted_to=null,granted_by=SYSTEM_PRINCIPAL_ID`，id 各为新 UUIDv7，除此之外该表零行。只有这三行可被复合 FK 看见后才创建两条互斥用户角色绑定，并调用既有 deterministic catalog 的 `ApprovalChainProvisioner::provision_defaults`；角色绑定的 user/法人和 `granted_by` 必须逐字命中上述三行，不能直接指向无法人 `user_accounts`。其余新行 id 沿用平台 UUIDv7 规则。该 PROVISIONING 行的 `legal_entity_id` 必须逐字等于治理法人，`kek_ref` 必须由已验签 deployment id 与 signed key-domain id 唯一计算为 `kms://ep/v1/deploy/<lowercase-deployment-uuid>/domain/<lowercase-key-domain-uuid>/kek/1`，`kek_version=1,provisioned_at=NULL`；logical locator 不是 KMS object 已存在的证明，所以不破坏“事务内不调用 KMS”的边界。两个密码只能用关闭 echo 的本机 `ReadConsoleW` 分别二次确认，拒绝 stdin redirection、argv/env/file，按身份模块唯一 Argon2id policy 写 PHC 串，任何输出/日志/receipt 不得含密码或 hash。角色 `F56_CONFIG_OPERATOR` 固定 duty=CONFIG，初始 `(permission_item_code,action)` exact-set 恰为八对：`(lowcode.config_package.view,VIEW)`、`(lowcode.config_package.import,CREATE)`、`(lowcode.config_package.autotest,UPDATE)`、`(lowcode.config_package.submit,SUBMIT)`、`(lowcode.config_package.sign,UPDATE)`、`(lowcode.config_release.view,VIEW)`、`(lowcode.config_release.submit,SUBMIT)`、`(lowcode.config_release.execute,UPDATE)`；角色 `SECURITY_ADMIN` 固定 duty=SECURITY，初始 exact-set 恰为两对：`(lowcode.config_package.view,VIEW)`、`(lowcode.config_package.approve,APPROVE)`，默认 `CONFIG_RELEASE` 链只指向它。bootstrap 在插入每一对前必须证明对应 `permission_items.code` 恰一行存在且 `allowed_actions` 含该 action，缺行、重复或动作不被允许都退出 78 并整事务回滚；不得只按 code 授权、任选该 permission 的其他 action 或补额外权限。后续角色变更只能在首张许可生效后走普通授权治理，不属于 bootstrap。CONFIG_OPERATOR 只能申请/签名/执行，SECURITY_APPROVER 只能形成审批结论，二者不得互授、自审或合并身份。该事务绝不调用 KMS、创建外部 key material 或把 key domain 标 ACTIVE；只有数据库写、链、角色、用户、设备、凭据、授权或审计失败才由 PostgreSQL 整笔回滚。

bootstrap 事务提交后，core-server 必须在任何 public readiness 前以 signed `bootstrap_id/key_domain_id` 调用阶段 2 唯一 `KeyDomainProvisioner` use case 的 resume 分支：锁定并核对同一治理法人下的 PROVISIONING 行及上述 exact logical locator，并把 locator 内 deployment UUID 与当前签名 deployment manifest 逐字比较，随后严格采用阶段 2 `KmsKeyMaterialProvisioner` 契约。只有 KEK 是 provider 持久 object：按 `EP-KEK-V1:<deployment>:<domain>:1` 幂等 ensure/readback，同 label identity/算法/usage/fingerprint 冲突时隔离并关闭 readiness，禁止覆盖、删除、另选 object、创建第二域或回落 provider。DEK 不形成第二持久 KMS object；DB 已有行只从 `wrapped_key` 重构并 readback，缺行才按含 `purpose+security_level_scope+version` 的 `EP-DEK-V1` operation label 生成 transient wrapped candidate，提交前崩溃可安全重生。四用途×四 scope 的 exact 16-row version=1 矩阵逐项 readback、同法人复合 FK、算法映射与摘要全部通过后，才按阶段 2 状态机在同一事务提交 `PROVISIONING→ACTIVE` 与唯一 `action='platform.key_domain.activated.v1'` 审计终结；完整 envelope（含 SYSTEM actor、key-domain object、null 证据列、system client 与 AuditWriter 派生链列）逐字采用阶段 2 的同名冻结，payload 逐字采用阶段 2 闭集，`activation_source=INITIAL_GOVERNANCE,bootstrap_id=<signed bootstrap_id>`，并绑定 KEK fingerprint、16 个 data-key id/digest。外部 KMS 失败不得伪称随前一 PostgreSQL 事务回滚，域保持 PROVISIONING 并返回 KEY_UNAVAILABLE；重启只对同 key_domain_id 继续 resume。九个常驻服务只有看到该域 ACTIVE、唯一 activation audit 与 bootstrap evidence 全部一致后，才可开放依赖该治理域的 readiness。

initial-governance 的语义摘要不再使用自然语言“排序后 hash”。统一原语为 `projection_digest(domain,dto)=SHA-256(ASCII(domain)||0x00||RFC8785_JCS(dto))`；每个 DTO root 的 `schema_version` 固定为 JSON number `1`、`purpose` 逐字等于 domain，strict parser 拒绝 unknown/duplicate/missing。UUID lowercase canonical，digest 64 lowerhex，时间 UTC whole-second，日期 `YYYY-MM-DD`，计数/版本为 JSON number、bool 为 JSON bool；所有 Option key 始终存在，无值为 JSON null。数组按下述键排序去重。raw `bootstrap.jcs/body JCS/decoded CMS DER/receipt JCS` 仍分别对 exact bytes 直接 SHA-256，不套 projection domain。

`bootstrap_authorization_registry_sha256` 唯一使用 domain `EP-INITIAL-GOVERNANCE-AUTHORIZATION-REGISTRY-V1` 和 root `{schema_version,purpose,entries}`。entry exact 为 `{bootstrap_role,user_id,device_id,signer_subject,certificate_sha256,subject_key_identifier_b64url,signature_cms_sha256}`，恰两项并按 `CONFIG_OPERATOR,SECURITY_APPROVER` enum 顺序；certificate/signature 摘要分别对 leaf exact DER 与 decoded CMS exact DER 直接 SHA-256。每项与 bootstrap body operator、authorization CMS、同一张 deployment-manifest roster leaf 的三列逐值相等。

`database_bootstrap_projection` 是审计中永久保存的初始非秘密数据库前像，domain/purpose=`EP-INITIAL-GOVERNANCE-DATABASE-PROJECTION-V1`，root exact 为 `{schema_version,purpose,legal_entity,key_domain,operators,legal_entity_grants,roles,role_permission_pairs,user_role_grants,approval_chains}`：

- `legal_entity={id,code,entity_no,name,short_name,timezone:"Asia/Shanghai",currency:"CNY",is_active:true}`；`key_domain={id,legal_entity_id,domain_kind:"LEGAL_ENTITY",state:"PROVISIONING",kek_ref,kek_version:1,provisioned_at:null}`；
- `operators` 恰两项并按 bootstrap role enum 顺序，entry exact `{bootstrap_role,user_id,login_name,employee_no,display_name,account_kind:"EMPLOYEE",home_legal_entity_id,clearance_level:20,status:"ACTIVE",is_mfa_required:true,activated_on,device_id,device_external_id,client,restricted_legal_entity_id,device_status:"ACTIVE",password_credential_id,password_credential_status:"ACTIVE",password_argon2_policy,x509_credential_id,x509_credential_status:"ACTIVE",x509_verifier,x509_credential_handle_b64url}`。`activated_on` 是 `committed_at` 的 UTC date；`device_external_id` 是 signed device UUID lowercase text；restricted/home 法人都等于治理法人。`password_argon2_policy` exact 为 `{algorithm:"ARGON2ID",version:19,memory_kib,iterations,parallelism,salt_len:16,hash_len:32}`，三项可调参数取本次已验证 effective config；DTO 只证明 credential id/status 与所用 policy，不含 password、salt、PHC/verifier 或其 digest。X509 verifier 固定 `cert-sha256:<leaf exact DER 64-lowerhex>`，handle 为 leaf SKI raw bytes 的 canonical base64url-no-pad；
- `legal_entity_grants` 恰三项，entry exact `{id,legal_entity_id,user_id,granted_from,granted_to:null,granted_by}`，按 user UUID bytes；`roles` 恰两项，按 code bytes，entry exact `{id,legal_entity_id,code,name,duty_class,is_portal_role:false,lifecycle_state:"EFFECTIVE",retired_at:null,is_active:true,deactivated_at:null}`，两项分别固定 `(F56_CONFIG_OPERATOR,"F-56 配置操作员",CONFIG)` 与 `(SECURITY_ADMIN,"安全管理员",SECURITY)`；
- `role_permission_pairs` 恰为上段八对加两对，entry exact `{role_id,role_code,permission_code,action}`，按 `(role_code,permission_code,action)` UTF-8/wire bytes；`user_role_grants` 恰两项，entry exact `{id,legal_entity_id,user_id,role_id,effective_from,effective_to:null,granted_by}`，按 user UUID bytes，effective_from 为 committed UTC date；
- `approval_chains` 不是只含 CONFIG_RELEASE，而是 `ApprovalChainProvisioner::provision_defaults` 对 Stage 4 `ApprovalScenarioCode::ALL` 的 exact **37 项**，按 Stage 4 enum 顺序。每项 exact `{chain_id,legal_entity_id,code,scenario,version_no:1,lifecycle_state:"EFFECTIVE",is_active:true,deactivated_at:null,node}`，`node={node_id,node_no:1,approver_kind:"ROLE",approver_ref:null,role_code,quorum:1,timeout_hours:24}`；chain/node id、code 与 role_code 逐字来自 Stage 4 deterministic catalog，只有该 catalog 既定 `CONFIG_RELEASE|EXTENSION_ENABLE` 使用 SECURITY_ADMIN，其他 35 项不得遗漏或被改成它。后续合法治理变更不改写此审计内嵌初始前像。

同一事务最后追加 `platform.bootstrap.initial_governance.v1` typed 审计。事务在任何 bootstrap row INSERT 前预分配审计 UUIDv7，并在取得锁后一次捕获 `committed_at` UTC whole-second，供全部 date/time 字段复用；因此 receipt 可以先引用 event id 而不形成 hash 循环。envelope 固定 `action=INITIAL_GOVERNANCE_BOOTSTRAPPED,object_type=platform.initial_governance,object_id=bootstrap_id,object_version=1,occurred_at=committed_at,client=system`，`before=null`；`after` exact 为 `{schema_version:1,purpose:"EP-INITIAL-GOVERNANCE-AUDIT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,bootstrap_authorization_registry_sha256,initial_license_archive_sha256,deployment_manifest_sha256,database_bootstrap_projection,database_bootstrap_projection_sha256,receipt_body_sha256,schema_manifest_sha256,ep_migrate_pe_sha256,committed_at,status:"COMMITTED"}`。内嵌 projection 按上段重算的 domain digest 必须等于同名字段；after/receipt/database mapping 不能只比较摘要。

该 initial-governance event 的完整 envelope 不允许由实现默认补值：`event_id=<上述预分配 UUIDv7>`、`legal_entity_id=<signed governance_legal_entity_id>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且逐字命中本事务已建立的同法人 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='INITIAL_GOVERNANCE_BOOTSTRAPPED'`、`object_type='platform.initial_governance'`、`object_id=bootstrap_id`、`object_version=1`、`before=null`、`after=<上述 exact root>`、`reason=null`、`approval_ref=null`、`reauth_ref=null`、`client='system'`、`occurred_at=committed_at`；`event_day/seq/prev_hash/hash` 只由既有 `AuditWriter` 分段链算法派生。Stage 14 必须逐列核对这份完整 envelope，而不只核 action、payload 或链 hash。

提交后工具在固定 `receipt-out` safe-handle 目录原子写唯一非秘密 `initial-governance.receipt.v1.jcs`；不得生成部署 KMS sidecar，也不得给 `ep-migrate` KMS 签名能力。receipt exact body 为 `{schema_version=1,purpose="EP-INITIAL-GOVERNANCE-RECEIPT-V1",bootstrap_id,deployment_id,bootstrap_body_sha256,initial_license_archive_sha256,governance_legal_entity_id,key_domain_id,key_domain_state="PROVISIONING",operator_user_ids,device_ids,role_codes,legal_entity_grant_ids,committed_at,audit_event_id,schema_manifest_sha256,ep_migrate_pe_sha256,status="COMMITTED"}`，四个数组各按 bytes 排序；`audit_event_id/committed_at` 逐字等于上述 envelope，`legal_entity_grant_ids` 恰为上段三行 id。`receipt_body_sha256=SHA-256(receipt exact JCS bytes)` 不得自包含，只进入审计 after。receipt 的可信性只来自双 CMS bootstrap input、命中签名产品清单的 Authenticode `ep-migrate` PE digest、数据库审计 hash chain 与 exact cross-check。若 DB 已有逐字相同审计终结但 receipt 因崩溃缺失，唯一允许的重跑是同 input digest/PE digest 只读核对完整 audit after、内嵌 projection 与数据库稳定映射后以 CREATE_NEW 补同 body receipt；已有 receipt 或任一字段不同都永久拒绝，绝不二次写业务行。Stage 14 必须用同一 DTO/domain 验证 receipt exact bytes、审计 after/hash chain、数据库 bootstrap projection、三条法人授权、37 条默认链、最初 license archive、最终首张 RELEASED grant、同 key domain 最终 ACTIVE 及 activation audit全部一致；缺一只阻止启用/发布，不阻止按本文开发。

许可并发只使用同一个 advisory key `hashtextextended('platform-license-current',0)`，但模式是闭集 `LicenseCurrentLockModeV1::{None,Shared,Exclusive}`，不能把全平台普通写都串在一把 exclusive lock 上。会产生 `BusinessWrite|BusinessApproval|IntegrationOutbound|AutomationStart` 四类受限副作用的普通 handler/job，在 `BEGIN` 与 mandatory session-context `SET LOCAL` 后以第一条业务 SQL 执行 `pg_advisory_xact_lock_shared(key)`；取得后才可调用 `IdempotencyStore::try_begin`、claim/读业务行、按 ModuleCode wire 顺序取 module shared lock并重读 `LicenseAdmissionGate`，持有至提交/回滚。四类普通事务因此可并发；许可替换/撤销的 exclusive 请求会等既有副作用排空，提交后新请求才取得 shared 并看见新状态。纯读与 `ReadReportAuditBackupExport|IdentitySecurityDisposition|ComplianceDisposition|InFlightConvergence` 的允许类固定 None；不得给已有目标法人的受限副作用谎报 None。

所有可能推进 F-56 special、替换 current 或推进可信时间的事务固定 Exclusive：第一条业务 SQL 执行 `pg_advisory_xact_lock(key)`，随后才可调用幂等存储、查询/claim package 或 worker batch。唯一总锁序为 `LICENSE_CURRENT_EXCLUSIVE →（仅 ordinary execute）platform_meta.config_release_mutex FOR UPDATE → package/order/item canonical rows → ModuleCode wire 顺序的 module locks`；ordinary execute 的连接 1 从取得 license lock 起持有两把前置锁直至 COMMIT，special execute 不取 `config_release_mutex` 且跳过 DDL 段一。随后才重读 current/history/source/dependency 并写 projection/package/order/outbox/audit。Stage 3 command middleware 的唯一 preamble 字段名为 `pre_idempotency_lock`，取值恰为上述三值；import/autotest/submit/approve/sign/create-release-order/execute 七类入口，以及 autotest accept、worker batch claim、lease/heartbeat、最终 aggregate 的每个短事务，无论包最终是否 special 都**无条件**选择 Exclusive。通用 `CompleteProcessTask` 的 typed `APPROVE` 分支因在首句前不能查 package，亦无条件 Exclusive；其他普通业务审批入口为 Shared。preamble 只能在现有 `UnitOfWork::transact` closure 内以 `&mut dyn Tx` 调用，禁止另开连接/事务，且 command handler、repository、幂等存储在 preamble 成功前调用数据库即由测试 double/架构检查失败。import 可在开事务前 safe-parse archive，但该结果不具权威性；事务内仍从持久化/候选 exact bytes 复验。九个 autotest suite 的纯只读查询事务本身为 None，不 claim、不续租、不汇总。typed `REJECT` 不推进 artifact、可信时间或 projection，固定 None，只锁自身 package/flow row 闭合同一 immutable content hash；不得在无锁事务查包后把 approve 改判为 reject或反向切换。

GRANT、REVOCATION 与 MODULE_PACKAGE applier 的 `apply` 只能在上述事务内调用；它们可以幂等地再次请求同一 transaction-level license lock，但“第一个数据库动作”指整个 special transaction 的第一条业务 SQL，不是 applier 内部较晚的第一句。取得全局锁后才重读并验证首发、直接后继、撤销目标或模块动作；不得仅靠锁一条可能不存在的 current 行。这样零 current 下两个并发首发、同一前驱的两个并发续期、续期与撤销竞态都串行重算，恰一条合法候选提交；输家按上述闭集返回 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID`，不得泄漏 SQLSTATE 或任选旧快照。

`deployment_id` 必须逐字等于签名部署清单和 Stage 14 当前部署证据的部署标识；不提供配置、环境变量、命令行或 UI 覆盖。导入事务把 `system_utc_now` 只读一次，`pre_import_trusted_now` 按第 3.3 节唯一公式计算，候选 artifact 不参与该值；候选 `issued_at>pre_import_trusted_now+5min`、链断裂、主体不受信、payload/行投影不等、续期不是当前直接后继或撤销对象不等，整项拒绝且零状态变更。GRANT 接受时新 current 的 `last_trusted_at=max(pre_import_trusted_now,candidate.issued_at)`；REVOCATION 接受时把目标 current 的 `last_trusted_at` 单调推进到 `max(existing_last_trusted_at,pre_import_trusted_now,candidate.issued_at)`，两者都在同一锁内事务完成。

信任根轮换后若唯一 current grant 或命中它的 revocation 的 inner signer 和/或其 RELEASED source special outer signer被新 CRL 明确标为 REVOKED，部署在替换提交前仍保持 `RESTRICTED/SIGNATURE_INVALID`，但不会形成永久自锁：允许一份 inner 与 outer 都由 ACTIVE signer 签发、`deployment_id/governance_legal_entity_id` 相同且 `supersedes_grant_id` 逐字指向该唯一 current id 的全新 GRANT 走第 3.3 节许可恢复链。该窄路径只在 current 的 row/source/payload/digest/signature bytes、special outer bytes 与历史接受证据仍逐字自洽、失败类别唯一为上述 inner/outer `CRL_REVOKED` 时成立；候选自身必须通过全部当前 bundle、日期、scope、用量与 direct-successor 规则，并在同一 advisory-lock 事务移槽。任意数据库/source/digest/signature 漂移、断链、多 current 或不能唯一分类的失败不得借此换证，只能先从可信备份/证据恢复。

### 3.2 数据投影

尚未生成的 `V20261013090200__platform_core_create_license_grants.sql` 一次建立以下终态列；旧十列短表被本文替代：

- `id, license_no, deployment_id, governance_legal_entity_id, issued_to, license_kind, issued_at, valid_from, valid_to, maintenance_valid_to`；
- `legal_entity_scope, legal_entity_ids, legal_entity_limit, named_user_limit, registered_device_limit`；
- `module_codes, entitlement_codes`；
- `payload_sha256, signature, signer_subject, trust_bundle_sha256`；
- `supersedes_grant_id, superseded_at, current_slot, last_trusted_at`；
- `revoked_at, revocation_id, revocation_issued_at, revocation_reason_code, revocation_payload_sha256, revocation_signature, revocation_signer_subject`；
- `grant_source_config_package_id, grant_source_config_item_id, revocation_source_config_package_id, revocation_source_config_item_id`；
- 公共 `row_version/created_*/updated_*`。

`current_slot` 只允许 `0|null` 且唯一；`supersedes_grant_id` 自外键且不得形成环。`governance_legal_entity_id` 建 `ON DELETE RESTRICT` 外键；同一 deployment 的所有 grant 必须等于最早 RELEASED grant 的该值，且法人停用命令必须先证明它不是任何部署的冻结治理法人。`signature` 与 `revocation_signature` 保存内层 CMS exact bytes。payload 由列重建 JCS 后必须与 digest/signature 同时相等；不能只信数据库布尔值。`V20261013090500__platform_meta_create_config_package_items.sql` 随建表增加 nullable `accepted_trust_bundle_sha256` 与 32-byte/普通项为空的行内 CHECK；`V20261013093300__platform_core_backfill_stage03_unpoliced_table_registry.sql` 在 config package 两表存在后才增加候选键 `UNIQUE(config_package_id,id)`、发布后不可变/跨表 DEFERRABLE commit 约束，并补六条 source FK：module 一条 package FK加一条同包复合 item FK，grant/revocation 各同样两条，全部 `ON DELETE RESTRICT`。commit 约束要求未 RELEASED special 的接受摘要为空、RELEASED special 恰为 32 bytes、普通 item 恒空，并要求 grant 行 `trust_bundle_sha256` 与 grant source item 相等；grant source 两列非空且唯一，revocation source 同空或同非空且唯一。对 special package，该图还要求 `DRAFT|PENDING_AUTOTEST|TEST_FAILED|TEST_PASSED` 的 `approval_legal_entity_id` 为空，而 `PENDING_APPROVAL` 及以后恰等于本节派生的治理法人；复合键保证 package 与 item 不能被交叉拼接。该迁移仍登记 Stage 3 六张部署级表，不新增迁移编号。

### 3.3 可信时间与四态

Rust 枚举唯一为：

```rust
pub enum LicenseStatus { Active, ExpiringSoon, GracePeriod, Restricted }
pub enum LicenseRestrictionReason { NotYetValid, ExpiredBeyondGrace, Revoked, SignatureInvalid, NoCurrentGrant }
```

wire 值固定为上述 variant 的 `SCREAMING_SNAKE_CASE`。受限原因优先级固定为：零 current=`NO_CURRENT_GRANT`；多 current、current-slot/source/payload/内外签名/信任包任一异常=`SIGNATURE_INVALID`；完整验签后撤销命中=`REVOKED`；再依次判 `NOT_YET_VALID` 与 `EXPIRED_BEYOND_GRACE`。同一次求值只能返回一个 reason；有效三态的 reason 必须为 null。

每个 core/worker 进程建立唯一 `TrustedClockV1`：在数据库连接后、public readiness 前先验证相关 audit hash chain，再读持久证据与 `system_utc_at_start`，取 `process_anchor_utc=max(initial-governance bootstrap committed_at、current/history grant.issued_at、已接受 revocation.issued_at、全部 license_grants.last_trusted_at、全部有效 trusted-time checkpoint.trusted_now、system_utc_at_start)`；尚无某类证据时只从 max 集合移除该项，不能把空集合解释为 epoch。随后捕获 OS monotonic `Instant`；进程内候选恒为 `process_anchor_utc + monotonic_elapsed`，不因 wall-clock 回拨而下降。每次 query/apply 的唯一公式是事务开始后只读一次 wall clock，并取 `trusted_now=max(上述持久证据、system_utc_now、process_anchor_utc+monotonic_elapsed)`；`trusted_date` 恰为该 UTC 时刻的 calendar date，不用服务器本地时区。

普通查询只计算而不写行，但生产启动 readiness 前、每个会推进 special package 状态的 import/autotest/submit/approve/sign/create-order/execute，以及启用后的 job-worker 目标 cadence **每 240 秒**一次 checkpoint，都必须按上段 Exclusive 锁序取同一个 `platform-license-current` key 并锁内重读 current；调度器不得把 240 秒解释为最低等待或 best-effort daily job。checkpoint 治理上下文来源优先级固定为：唯一 current grant 的冻结治理法人 → 已验证 initial-governance audit/receipt 的治理法人 → 仅首张 GRANT whole transaction 的 candidate signed `governance_legal_entity_id`；零/多/损坏或当前动作不在第三种窄口时不得任选请求法人。若 current 存在，以 compare-and-set 把其 `last_trusted_at` 推进到本次 `trusted_now`；若 current 为零，不执行不存在行的伪 CAS，改走下述 audit-only checkpoint 后继续首张 GRANT 全链。两种分支都在同一事务确保本 UTC 240-second slot 已有唯一 checkpoint，之后才完成原动作；reject 只闭合拒绝结论，不需要推进。唯一例外是上段显式 non-production、bootstrap absent、zero-current/zero-legal-entity 空库：readiness 不写无法归属的 audit、worker 保持 dormant；首张 GRANT 只有在候选治理法人及当前操作者的法人授权已由受控测试/开发 setup 建立后，才可在其 exclusive transaction 内用候选来源创建首 checkpoint。该例外不能出现在 production 或任何 Stage 14 evidence。

checkpoint 的 audit `action` 固定 `LICENSE_TRUSTED_TIME_CHECKPOINT`，`after` payload 是 unknown/missing key 均失败的 strict-JCS 闭集 `{schema_version:1,purpose:"EP-LICENSE-TRUSTED-TIME-CHECKPOINT-V1",deployment_id,slot_utc,trusted_now,current_grant_id}`；`schema_version` 是 JSON number `1`，UUID 小写 canonical，时间 UTC whole-second。`slot_utc` 唯一算法为把 `trusted_now` 转成 Unix seconds、计算 `floor(unix_seconds/240)*240` 后再发出 canonical RFC3339 UTC whole-second；它不是按本地时区、五分钟整点或“分钟可被 4 整除”近似。`current_grant_id` 为 `ensure_checkpoint` 入口在持有 exclusive lock、完成 current 重读后且任何 special 业务 mutation 前捕获的唯一 current UUID 或 JSON null；同次入口的 `trusted_now/slot_utc/current_grant_id` 组成不可变 snapshot，AuditWriter 即使在 terminal batch 稍后 INSERT 也只能复用该 snapshot，禁止在首张 grant 插入、current 换槽或撤销后重算。audit 行 `legal_entity_id` 固定为 F-56 治理法人、actor 为其 SYSTEM 授权、client=`system`。耐久语义键为 `license-trusted-time:v1:<lowercase-deployment-id>:<slot_utc>`；在 exclusive lock 内只能按 `(action,after->>'purpose',after->>'deployment_id',after->>'slot_utc')` 查询当前 deployment/slot 的 audit 行：零行才以本次 snapshot 追加，一行必须保留既有 exact payload bytes、不得 UPDATE，只核对 schema/purpose/deployment/slot、创建时 current id 形状和 hash chain，多行或任一不等立即失败关闭。同 slot 后续事务复用既有 checkpoint，不要求其 payload `trusted_now` 等于本次较新计算值；若 current 存在，`last_trusted_at` 仍可在同一事务独立 CAS 到本次 `trusted_now`。AuditWriter 是唯一 INSERT 权限方且所有合法 writer 受同一 exclusive lock，因此合法路径同一语义键至多一行，不新增表、列或普通幂等键；Stage 14 必须把多行视为篡改/实现错误而失败关闭。若 current 为零，必要的新 checkpoint 与当前 special 状态推进同事务，因此首发不自锁；首次 grant 插入仍将 `max(pre_import_trusted_now,candidate.issued_at)` 写入 `last_trusted_at`。

只有上述查询为零行时才预分配一个新的 checkpoint `event_id` UUIDv7 并追加事件；一行复用分支不得分配或写新事件。新行的完整 envelope 固定为 `legal_entity_id=<冻结治理法人>`、`actor_user_id=SYSTEM_PRINCIPAL_ID` 且命中该法人的 ACTIVE SYSTEM grant、`actor_device_id=null`、`action='LICENSE_TRUSTED_TIME_CHECKPOINT'`、`object_type='platform.license_trusted_time'`、`object_id=deployment_id`、`object_version=null`、`before=null`、`after=<入口不可变 snapshot 的上述 exact payload>`、`reason=null`、`approval_ref=null`、`reauth_ref=null`、`client='system'`、`occurred_at=<入口捕获的 trusted_now>`；`event_day/seq/prev_hash/hash` 只由既有 `AuditWriter` 分段链算法派生。`object_id` 是已验签 deployment UUID，不随 current grant 换槽；任何默认 object、数据库当前时间或业务 mutation 后时间都不等价。

这样倒拨绝不低于已验证持久证据、同一进程内绝不降低；240 秒目标 cadence 留出 60 秒调度/提交预算。连续 checkpoint 缺失、同一 slot 重复/漂移、相邻成功 checkpoint 的 `trusted_now` 差值超过 300 秒，或 wall clock 相对 monotonic trajectory 偏差超过 300 秒，都发不可抑制安全告警；Stage 14 必须重算 checkpoint audit registry、hash chain、slot 单值/映射、trusted_now 单调性及其与 bootstrap/current `last_trusted_at` 的投影关系，在有服务 uptime 证据覆盖的区间任一相邻成功值差 `>300s` 即不得发布。该 300 秒是有完整 uptime/audit 证据前提下的发布观测上限，不是 NTP/TPM、恶意宿主或被篡改证据下的硬安全保证；首版不得宣传成绝对防回拨。在没有外部可信时间源时不伪造更精确 UTC。系统时钟错误前跳一旦被上述受控写持久化不得自动回拨或 direct SQL 重置，唯一恢复是按 Stage 14 可信备份完整恢复数据库与审计链后重算。判定边界唯一为：

- `SUBSCRIPTION`：`trusted_date<valid_from` 为 `RESTRICTED/NOT_YET_VALID`；`trusted_date<valid_to-60 days` 为 `ACTIVE`；从 `valid_to-60 days`（含）至 `valid_to`（含）为 `EXPIRING_SOON`；`valid_to` 后第 1 至第 30 个自然日为 `GRACE_PERIOD`；其后为 `RESTRICTED/EXPIRED_BEYOND_GRACE`；
- `PERPETUAL`：从 `valid_from` 起始终为 `ACTIVE`，维护到期不改变运行态；签名失效或明确撤销时为 `RESTRICTED`；
- 任一 kind 被已验签撤销命中时立即 `RESTRICTED/REVOKED`，不经过临期或宽限；无 current grant 或 current grant 不能完整复验时按受限运行失败关闭，但系统进程仍可启动。

`ACTIVE|EXPIRING_SOON|GRACE_PERIOD` 都是当前有效许可。宽限期全部功能可用，只扩大告警；`RESTRICTED` 才执行总体规格第 3.4 章后果：阻止常规业务写入、普通业务审批、集成出站和新自动化任务，统一返回 `PLATFORM.LICENSE.RESTRICTED`（`BUSINESS_CONFLICT/409/retryable=false`）；允许查询、普通报表、审计、备份、数据导出、身份安全处置、合规更正/删除/销毁以及在途 Outbox/Saga 收敛。全局 `LicenseStatus` 只描述 current grant 本身；当它有效但 `legal_entity_scope=LIST` 未包含本次已鉴权的目标法人时，该法人请求按同一受限后果处理，禁止上述四类副作用并返回同一码，但不把全局状态改成 `RESTRICTED`，且该法人的历史查询/报表/备份/导出与安全合规处置仍可用。为避免首次安装或到期后自锁，另有一个不可扩展的许可恢复例外：`LICENSE_GRANT` 的 import→autotest→submit→Win/Mac `CONFIG_RELEASE` 审批结论→sign→release-order/execute 全链，以及 `MODULE_PACKAGE` 中 `DISABLE` 动作的同一全链可继续；该例外仍执行原权限、双人职责分离、内外签名、部署绑定和审计，不允许普通配置项、模块 INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 或其他业务审批借用。许可状态不得成为启动 Blocking 自检。

### 3.4 永久许可的更新权

永久许可的 `maintenance_valid_to` 只控制新产品版本、模块 package version、安全补丁和连接器更新的接收资格，不停止已交付能力。模块 manifest 的 `released_on` 晚于该日期时拒绝安装/升级；日期为空表示本 grant 没有更新权。已安装版本、历史数据、查询、报表、备份和导出继续可用。紧急安全撤下指令与签名撤销文件不受维护到期限制；它们可停用不安全能力但不得删除客户数据。

### 3.5 用量

ServerAdmin 每次读取和每日监测任务都在同一 repeatable-read snapshot 按唯一 SQL 谓词实时统计：已启用法人=`platform_core.legal_entities.is_active=true`；命名用户=`platform_core.user_accounts.account_kind<>'SYSTEM' AND status IN ('ACTIVE','LOCKED','SUSPENDED')`，因此未激活/已停用不计而 PORTAL/BREAKGLASS 不能被用来绕过计量；已注册设备=`platform_core.user_devices.status IN ('PENDING','ACTIVE')`，REVOKED 不计。每日任务只刷新三项 metrics、对越限/恢复边缘发既有告警并让 ServerAdmin 显著提示；不新增 usage 表、日终持久化快照、月度商业申报、联网遥测或向发行方发送数据。首版若需留档，只由管理员按既有受权数据导出生成当时点报表并纳入普通导出审计，不能冒充连续日终或月度签名申报。超过任一上限不阻断建法人、建用户、登记设备或业务操作。模块和 F-55 entitlement 则是功能授权硬门：没有当前有效授权时不得安装/启用相应业务入口。未来若要自动月报或厂商计量上报，必须另立隐私、签名、留存、出站与同意裁定。

## 4. 签名模块包终态

### 4.1 已签名 manifest 与操作项

内层 manifest 只描述统一产品内已经编译存在的模块：

```rust
pub struct ModulePackageManifestV1 {
    pub schema_version: u16,                 // exact 1
    pub purpose: String,                    // exact "EP-MODULE-PACKAGE-V1"
    pub package_id: Uuid,
    pub package_code: String,               // [a-z][a-z0-9._-]{0,63}
    pub package_version: SemVerV1,          // MAJOR.MINOR.PATCH，各 0..65535
    pub module_code: ModuleCode,
    pub issued_at: DateTime<Utc>,
    pub released_on: NaiveDate,
    pub min_platform_version: SemVerV1,
    pub max_platform_version_exclusive: Option<SemVerV1>,
    pub module_contract_version: u32,        // wire/persistence effective domain 1..=2147483647
    pub module_contract_sha256: Sha256Digest,
    pub data_on_disable: String,             // exact "RETAIN"
    pub package_kind: String,               // exact "DECLARATIVE_BUILTIN_MODULE"
}

pub struct SemVerV1 {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

pub enum ModulePackageActionV1 { Install, Enable, Disable, Upgrade, RollbackVersion }

pub struct ModulePackageItemV1 {
    pub schema_version: u16,                 // exact 1
    pub action: ModulePackageActionV1,
    pub reason: String,                     // 1..1000 UTF-8 bytes
    pub artifact: SignedBusinessArtifactV1<ModulePackageManifestV1>,
}

pub struct ProductModulesManifestV1 {
    pub schema_version: u16,                 // exact 1
    pub purpose: String,                    // exact "EP-PRODUCT-MODULES-V1"
    pub product_version: SemVerV1,
    pub modules: Vec<ProductModuleContractV1>,
}

pub struct ProductModuleContractV1 {
    pub module_code: ModuleCode,
    pub module_contract_version: u32,        // 1..=2147483647
    pub module_contract_sha256: Sha256Digest,
    pub module_dependencies: Vec<ModuleCode>,
}

pub struct ModuleContractDescriptorV1 {
    pub schema_version: u16,                 // exact 1
    pub purpose: String,                    // exact "EP-MODULE-CONTRACT-V1"
    pub module_code: ModuleCode,
    pub module_contract_version: u32,        // 1..=2147483647
    pub module_dependencies: Vec<ModuleCode>,
    pub abi_entries: Vec<ModuleAbiEntryV1>,
}

pub enum ModuleAbiKindV1 { Command, Query, Event, Job, Permission }

pub struct ModuleAbiEntryV1 {
    pub kind: ModuleAbiKindV1,
    pub code: String,
    pub schema_sha256: Sha256Digest,
}
```

`SemVerV1` 的 JSON wire 恰为 strict object `{"major":u16,"minor":u16,"patch":u16}`，不接受字符串、预发布段、build metadata、负数或额外字段；大小按 `(major,minor,patch)` 无符号字典序比较。`min_platform_version` 必填；`max_platform_version_exclusive` 的 JSON key 必须存在但可为 null，null 唯一表示无上界。非空时它必须严格大于 `min_platform_version`；当前签名产品 manifest 的同形状版本必须满足 `min_platform_version <= product_version`，并且仅在 max 非空时再满足 `product_version < max_platform_version_exclusive`。

所谓“当前签名产品 manifest”只有一份可执行定义：待签发布目录的 `target/release-package/product-modules.v1.jcs`，安装后路径固定 `C:\EP\product-modules.v1.jcs`。它是最大 262,144 bytes 的 strict RFC 8785 JCS、UTF-8 无 BOM；`modules` 必须恰含 15 个 `ModuleCode`，按 wire bytes 排序且各一行，每行 dependencies 同样排序去重、只指闭集、不得自依赖，整图必须 DAG。

每个 contract digest 的来源和前像固定，不再允许手填“不透明常量”。仓库必须恰有以下 15 个 strict descriptor：`contracts/modules/{mdm,crm,cpq,clm,sales,procure,inventory,costing,project,service,finance,ledger,invoice,portal,reporting}.contract.v1.jcs`；每个最大 262,144 bytes、UTF-8 无 BOM、RFC 8785 JCS exact bytes，DTO 恰为上面的 `ModuleContractDescriptorV1`。文件名、`module_code` 一一对应；dependencies 按 wire bytes 排序去重、只指该 15 值闭集且不含自身；`abi_entries` 为 1..4096 项，按 `(kind wire bytes,code UTF-8 bytes)` 排序且组合唯一，kind wire 恰为 `COMMAND|QUERY|EVENT|JOB|PERMISSION`，code 匹配 `[a-z][a-z0-9_.-]{0,127}`。每项 `schema_sha256` 的唯一前像是同模块目录 `contracts/modules/<module-wire>/schemas/<schema_sha256-lowerhex>.schema.v1.jcs` 的 exact bytes；schema file 最大 65,536 bytes、strict RFC 8785 JCS，root `$schema` 必须逐字为 `https://json-schema.org/draft/2020-12/schema`，禁止外部/网络/文件 `$ref`，只允许同文档 `#` fragment。摘要文件名、entry 值与重算 SHA-256 必须三者相等。

`module_contract_sha256=SHA-256(descriptor exact bytes)`，`module_contract_version` 逐字取 descriptor；Rust 类型保留 `u32`，但 descriptor、产品清单、模块包、解析器与 PostgreSQL `int` 投影的共同有效域固定为 `1..=2147483647`，解析后在任何比较或入库前做 checked conversion，`0` 与 `2147483648..=u32::MAX` 均以 invalid payload 拒绝，不能截断、回绕或依赖数据库 cast 失败。descriptor 任何 byte/依赖/ABI/schema digest 改变都必须严格增加 version，同 version 不得映射另一 digest；达到上界后不能再发布下一 contract version，必须先经新 schema/version 设计裁定。每个 `ep-contract-<module>` 编译期导出由 descriptor 生成的 `MODULE_CONTRACT_VERSION`、`MODULE_CONTRACT_SHA256` 与 `MODULE_ABI_REGISTRY`；`cargo xtask module-contracts verify` 对 compiled public command/query/event/job/permission registry 与 descriptor entries 做双向 exact-set 比较，并重算全部 schema、descriptor、依赖 DAG，缺/多/重复/漂移一律构建失败。`product-modules.v1.jcs` 只从这 15 个已验证 descriptor 的 version/digest/dependencies 与工作区 canonical product version 生成，再 strict 回读；不得从环境变量、数据库、模块包、另一个 dependency registry 或手写 Rust 常量取值。实际 digest/version 是签名 build 实例值，但算法、路径、前像和生成者已在此冻结。

`product-modules.v1.jcs` 必须作为 `MANIFEST.sha256` closed roster 的必有 regular file，其 digest 被签名 manifest 与产品 Authenticode CAB 共同覆盖。安装器用 safe handle 复制、flush、原子发布并 readback；core/worker 每次模块动作及启动后的模块运行门都通过 `C:\EP` 固定根 safe-handle resolver 打开，拒绝 reparse/ADS/hardlink/路径漂移，核对 exact file digest 命中已验签 `MANIFEST.sha256` 后才 strict parse。Stage 14 的 product manifest projection 必须包含该 exact digest、product_version、15 行 contract/dependency digest与 DAG 结论；数据库、环境变量、ServerAdmin 或 MODULE_PACKAGE 都不能提供第二份产品目录。

模块包的 `module_contract_version/module_contract_sha256` 必须逐字等于该文件中目标 `ModuleCode` 的编译期值，且 `product_version` 落在包声明的半开兼容区间。模块包没有附件、解包目录或可执行正文；出现 code/sql/script/file/url/hook/capability grant 字段即 unknown-field 拒绝。

模块依赖闭包的唯一来源也是签名产品 manifest 内的编译期 `module_dependencies` 有向无环图；模块包不得自报或改写依赖。构建门禁先证明 15 个 module code 恰各一节点、边两端都在闭集且无环。INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 都从该图计算“目标 module + 传递依赖闭包”，要求同一份当前有效签名许可的 `module_codes` 完整覆盖该集合；INSTALL 只落到 disabled，因而不要求依赖已经启用，ENABLE 才额外要求每个依赖都为 `INSTALLED_ENABLED`。不存在数据库手填依赖、环境变量依赖或按缺失依赖静默安装/启用。

历史 package identity 也是一一映射。全部 RELEASED MODULE_PACKAGE items 上，`package_id` 只能对应一个 exact inner artifact；`(module_code,package_code,package_version)` 也只能对应同一个 `package_id` 与 exact inner artifact。重复带回同一 exact inner 用于 ENABLE/DISABLE/ROLLBACK_VERSION 合法，不同 payload/digest/signature/signer bytes 冒用任一 identity 则在 release 锁内按 `PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID` 整项拒绝。`V20261013093300` 的同一 deferred F-56 graph trigger 在 COMMIT 扫描 RELEASED history 强制这两个映射，无新历史表或运行时任选分支。

安装态和五个模块动作都是**部署全局**事实，special package 的 `applies_to_legal_entity_ids` 必须为空，动作 API 不接受或推断某个法人作为安装范围；`legal_entity_scope` 只在每次业务请求上把当前有效许可的 module/entitlement 能力裁到目标法人。因此全局 `INSTALLED_ENABLED` 不等于任一法人可用：运行时必须同时满足全局安装态、当前有效签名许可含目标 module/entitlement、请求法人命中签名 scope，以及对应权限/能力门禁。

### 4.2 安装态与合法边

`ModuleState` 保持三态，但合法动作改为以下闭集：

| 当前态 | 动作 | 结果 | 守卫 |
|---|---|---|---|
| NOT_INSTALLED | INSTALL | INSTALLED_DISABLED | 当前许可含目标 module 与依赖闭包；manifest/平台/契约/维护期通过；从未安装或历史只作追溯 |
| INSTALLED_DISABLED | ENABLE | INSTALLED_ENABLED | artifact 与已安装 identity 完全相等；当前许可有效且含 module code；模块自检与依赖闭包通过 |
| INSTALLED_ENABLED | DISABLE | INSTALLED_DISABLED | 总是允许；通常 artifact identity 必须与当前已安装包逐字段相等；当前 signer 被 CRL 吊销时只准第 4.2 节窄重签恢复；先撤业务路由/写入，再排空在途，停定时器和新 Outbox 派发 |
| INSTALLED_DISABLED | UPGRADE | INSTALLED_DISABLED | 当前许可有效且含 module code/依赖闭包；目标 semver 严格更高、manifest 兼容、维护权有效 |
| INSTALLED_DISABLED | ROLLBACK_VERSION | INSTALLED_DISABLED | 当前许可有效且含 module code/依赖闭包；目标是历史已验签版本、仍与当前产品/契约兼容；显式新审批，不是通用回退 |

不存在 `NOT_INSTALLED→INSTALLED_ENABLED`、启用态升级、卸载、DELETE、降版本伪装升级或 direct SQL 边。非法边统一 `PLATFORM.MODULE.TRANSITION_INVALID`；授权不足用 `PLATFORM.MODULE.LICENSE_REQUIRED`；只有 inner/outer 密码学与 signer trust 已按第 2.2 节通过后，产品版本、模块契约摘要、维护权、历史 package identity 或其他兼容条件不成立才用 `PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE`。密码学或信任失败始终分别用 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID|SIGNER_NOT_TRUSTED`，不得被模块码吞并；第 4.2 节 CRL-DISABLE 窄例外只改变是否允许处置，不改变失败分类。

五条动作的 inner/outer 状态不留隐式分支：INSTALL 与 UPGRADE 首次引入的 inner 必须 ACTIVE；ENABLE/DISABLE 的 inner 必须与 current package exact bytes 相等且有唯一 RELEASED INSTALL/UPGRADE origin，ROLLBACK_VERSION 的 inner 必须与唯一 RELEASED historical INSTALL/UPGRADE origin exact bytes 相等，这三类复用 inner 可为 ACTIVE 或 RETIRED-nonrevoked；每个新动作 package 的 special outer 一律 ACTIVE。ROLLBACK_VERSION 只能选 semver 严格低于 current 的历史 origin，UPGRADE 只能选严格高于 current 的全新 identity；同版本或只改签名的替换都拒绝。所有判断都在动作独占锁内按第 2.2 节再次复验。

若当前安装 package 的 inner signer 和/或其 current RELEASED source special outer signer 被当前 bundle CRL 明确标为 REVOKED，该模块的业务写、审批、自动化与外发运行门立即关闭，但 deployment-level LicenseStatus 仍只由 current grant/revocation 决定；该 package 及所有 inner/outer signer 被标记 REVOKED 的历史 package 永不再是 ENABLE/ROLLBACK_VERSION 的正向证据。为确保仍能安全停用，唯一例外是新的 special package 把当前已安装 `SignedBusinessArtifactV1<ModulePackageManifestV1>` **整份 exact bytes 原样带回**、固定 `action=DISABLE`，并由 ACTIVE signer 对包含 action/reason/旧 artifact 的新 outer CMS 签名；旧 inner、旧 source outer、不可变 source item/接受摘要及其 payload/digest/signature/projection 必须逐字自洽，失败类别只能是旧 inner 和/或旧 source outer `CRL_REVOKED`，未撤销的另一层必须为 ACTIVE 或 RETIRED-nonrevoked。此时只把旧 inner 视为“曾合法接受的停用目标身份”，不宣称已撤销层在当前 bundle 下通过正向验签；本次 ACTIVE outer 才是停用授权。该窄例外仍需完整审批、outer 验签、独占排空和审计，提交只改变停用状态/时间/reason；不得据此 ENABLE、INSTALL、UPGRADE、ROLLBACK_VERSION 或恢复 revoked 历史包。

这条窄停用不能靠“新旧 inner bytes 相同”推断来源关系，因为同一 inner 可被多个 ENABLE/DISABLE outer 合法带回。除本次 recovery item 自身的 `platform.config_special.accepted.v1` 外，状态更新同一 audit terminal batch 还必须恰写一条 `action='MODULE_SIGNER_REVOKED_DISABLED'` 的仅追加事件，same-byte 幂等回放只返回既有结果，不追加第二条。该 event 的 `before` 不是摘要占位，而是锁内更新前完整的 typed `EP-F56-CURRENT-MODULE-PROJECTION-V1` DTO；其 exact keys 为 `{schema_version:1,purpose:"EP-F56-CURRENT-MODULE-PROJECTION-V1",id,module_code,display_name,row_version,install_state,package_id,package_code,package_version,package_payload_sha256,package_signature_cms_sha256,package_signer_subject,package_signed_at,module_contract_version,module_contract_sha256,min_platform_version,max_platform_version_exclusive,released_on,source_config_package_id,source_config_item_id,installed_at,state_changed_at,enabled_at,disabled_at,last_transition_reason}`。`schema_version`、`row_version`、SemVer 三分量与 `module_contract_version` 都按此具名 ABI 使用 JSON number；row version 为 `1..=9223372036854775807`，contract version 为 `1..=2147483647`，三个 SemVer 值为 strict object/null，摘要为 lowerhex，时间为 UTC whole-second。该 typed DTO 不受普通无具名审计“业务数值写字符串”的默认规则覆盖。

该 recovery terminal batch 必须在写入前预分配两个互异的新 UUIDv7 event id，并以唯一链顺序先写 `MODULE_SIGNER_REVOKED_DISABLED`、再写 `platform.config_special.accepted.v1`；accepted event 必须是该 batch 最后一条。两事件的 `legal_entity_id` 都是冻结治理法人，`actor_user_id/actor_device_id/client` 都逐字取同一次 execute 的受信 `SecurityContext`，`approval_ref` 都等于 `config_packages.approval_ref`，`reason/reauth_ref` 都为 null。recovery event 其余 envelope 固定 `object_type='platform.module_registrations'`、`object_id=before.id`、`object_version=after.row_version`、`before=<上述完整 current projection DTO>`、`after=<下述 recovery metadata DTO>`、`occurred_at=after.disabled_at`；accepted event 的其余 envelope 逐字采用第 2.1 节同名冻结。两事件的 `event_day/seq/prev_hash/hash` 只由既有 `AuditWriter` 按上述顺序链式派生；same-byte 回放不得预分配、插入或重排任一事件。

event 的 `after` 是 unknown/missing key 均失败的 strict-JCS recovery 闭集 `{schema_version:1,purpose:"EP-MODULE-SIGNER-REVOKED-DISABLED-V1",module_code,previous_source_config_package_id,previous_source_config_item_id,recovery_config_package_id,recovery_config_item_id,before_projection_sha256,after_projection_sha256,disabled_at,reason_sha256}`，其中 `schema_version` 为 JSON number `1`。四个 source/recovery id 分别逐字取 `before` DTO 与本次 RELEASED DISABLE item；`reason_sha256=SHA-256(ASCII("EP-MODULE-DISABLE-REASON-V1")||0x00||UTF-8(recovery item reason))`。两个 projection digest 的算法均为 `SHA-256(ASCII("EP-F56-CURRENT-MODULE-PROJECTION-V1")||0x00||JCS(dto))`：before digest 必须从 audit `before` exact bytes 重算；after DTO 只能由 before 作下列确定变换重建——`row_version` checked `+1`、`install_state="INSTALLED_DISABLED"`、`state_changed_at=disabled_at=event.after.disabled_at`、`last_transition_reason=recovery item reason`，其余每个 key 逐字保留。event envelope 固定 `object_type='platform.module_registrations'`、`object_id=before.id`、`object_version=after.row_version`、`occurred_at=after.disabled_at`；after digest必须由这份派生 DTO重算，若该停用仍是 current containment state，数据库 current row 还必须逐键等于派生 after DTO，后来已有合法动作时则沿后续审计/投影链验证而不得要求现态倒退。after 保留 previous source 两列与旧 inner/package 投影，只有转换字段来自 recovery item。

Stage 14 的 recovery peer 只能从这条 action 的 typed before/after、审计 hash chain、派生 projection、recovery item accepted event 与当前/后续投影链得到；不允许信任未给前像的摘要，或从相同 package id、相同 inner、相近时间猜选。事件缺失、重复、任一 preimage/digest/time/id/reason/object version 不等时，该收容状态不能被认证为 PASS。

停用完成后，若 current grant 有效，允许一份 inner/outer 均由 ACTIVE signer 签发、semver 严格更高且通过全部依赖/契约/维护权守卫的新 UPGRADE 替换 revoked current projection；旧 package 只提供版本比较与审计来源，不能充当新包的正向信任。任一 bad digest/bad signature/bad source/断链、终结审计不闭合或不能唯一分类为 CRL 的失败仍拒绝上述两条收容路径并要求可信恢复。

跨进程停用/启用/升级的模块并发原语是 `ModuleOperationGate`，每个 module key 为 `hashtextextended('platform-module:' || ModuleCode wire,0)`，且它永远排在上段同一许可 key 的 Shared/Exclusive lock 之后。core-server 的每个模块业务写事务必须先以第一条业务 SQL 取得 license Shared，再在读取业务行前用同一事务取得 owner module 的 `pg_advisory_xact_lock_shared(key)`，随后重读许可与第 5 节 effective query 并把两锁持有到 commit。job-worker 的 claim/lease 短事务先取 license Shared、再取需要的 module transaction-level shared 并重验；事务提交后若要产生真实外部副作用，不得跨网络调用持有数据库事务，而须在派发前以专用 worker 连接依次取得 license **session-level shared**、owner module session-level shared，重读 admission/effective query，并把两把 session lock 持有到该次派发或取消终结，在 finally 按 module→license 逆序显式 unlock，进程崩溃由 PostgreSQL 断连释放。若许可或模块的 exclusive 操作在 claim 后先提交，worker 取得 session shared 后的重读必须取消而非派发；这关闭 claim→dispatch 间隙而不把外部调用塞进业务事务。

配置发布取得 module locks 的局部顺序唯一为 ModuleCode wire bytes 升序，但它永远位于全局 license advisory lock 与 package/order/item row locks 之后；整次取得 module locks 的总 deadline 固定 30 秒而不是每把 30 秒。INSTALL/UPGRADE/ROLLBACK_VERSION 取目标 exclusive；ENABLE 取目标 exclusive 加其传递依赖 shared，锁内再次证明每个依赖 raw enabled 且 effective trust/许可成立；DISABLE 为了在产品 DAG 损坏时仍能安全失败关闭并排空所有潜在反向依赖，固定取得全部 15 个 module exclusive，提交却只把目标置 disabled，任何 dependent 的 raw state 均不联动改写。独占请求进入等待后，新共享请求排队，既有共享持有者完成后动作才重读全部状态并写变化；30 秒内未全部取得则整个发布事务回滚并返回 `INFRASTRUCTURE/503/retryable=true` 的 `PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT`，状态、路由、幂等、Outbox 与审计零部分变化。目标停用后，所有依赖它的模块因第 5 节递归 effective 判定自动变为不可运行；目标重新启用且全闭包复验通过后，这些 dependent 无需改 raw state 即自动恢复。

DISABLE 在独占锁内写 `INSTALLED_DISABLED`、幂等结果和审计终结批后提交；其后排队的 core/worker 共享请求重读到 disabled，在读业务 payload 或派发外部消息前失败关闭。UI/业务 HTTP 写入口由同一个运行时守卫表现为不可用或只读，不能靠进程内缓存抢先放行。历史表、附件、审计、配置包、包身份和许可行一律保留；授权查询、报表、审计检索、备份、导出与合规处置仍可用。重新启用时同样取得独占锁并逐次重验内外签名、许可、契约、依赖闭包和数据 schema，任一失败保持 disabled。

### 4.3 数据投影

尚未生成的 `V20261013090100__platform_core_create_module_registrations.sql` 在既有列上增加：

- `package_id, package_code, package_version_major/minor/patch`；
- `package_payload_sha256, package_signature, package_signer_subject, package_signed_at`；
- `module_contract_version, module_contract_sha256, min_platform_version, max_platform_version_exclusive, released_on`；
- `source_config_package_id, source_config_item_id, enabled_at, disabled_at, last_transition_reason`。

fresh migration 必须原子 seed 恰 15 行，不采用稀疏表或“missing=NOT_INSTALLED”解释。按 ModuleCode wire 顺序的 `(id,module_code,display_name)` exact catalog 为：`00000000-0000-7000-8000-000000000601/mdm/主数据管理`、`00000000-0000-7000-8000-000000000602/crm/客户关系管理`、`00000000-0000-7000-8000-000000000603/cpq/配置、定价与报价`、`00000000-0000-7000-8000-000000000604/clm/合同生命周期管理`、`00000000-0000-7000-8000-000000000605/sales/销售与订单`、`00000000-0000-7000-8000-000000000606/procure/采购管理`、`00000000-0000-7000-8000-000000000607/inventory/库存管理`、`00000000-0000-7000-8000-000000000608/costing/成本管理`、`00000000-0000-7000-8000-000000000609/project/项目管理`、`00000000-0000-7000-8000-000000000610/service/售后服务`、`00000000-0000-7000-8000-000000000611/finance/收付款与往来`、`00000000-0000-7000-8000-000000000612/ledger/总账与结账`、`00000000-0000-7000-8000-000000000613/invoice/发票管理`、`00000000-0000-7000-8000-000000000614/portal/供应商门户`、`00000000-0000-7000-8000-000000000615/reporting/报表与分析`。此 catalog 不允许使用会改变 UUID 位数的省略前缀表示。15 行均由 `SYSTEM_PRINCIPAL_ID` 创建/更新，`install_state=NOT_INSTALLED`，全部 package/source/install/action time 列为空。行不得 DELETE，ModuleCode/display catalog 不可运行时编辑；缺、多、重复或错 seed 使 schema 检查失败。**F-57 当前产品显示名窄覆盖：**UUID、wire code、15 行基数和所有许可/依赖语义保持不变，仅 `ledger` 显示为“经营分录与期间”、`portal` 显示为“客户与供应商门户”；本段两个旧中文名只作 F-56 历史输入，实施不得恢复。

NOT_INSTALLED 时 package/source/安装与动作时间投影全空。两个 INSTALLED 态的 package identity、签名、契约、`min_platform_version`、released_on、source、`installed_at/state_changed_at/last_transition_reason` 全非空；`max_platform_version_exclusive` 是该兼容区间中唯一允许为 NULL 的列，NULL 表示无上界，非空时才强制严格大于 min 并把当前产品版本限制在 max 之前。首次 INSTALL 固定 `installed_at=state_changed_at=disabled_at` 且 `enabled_at=null`，后续 ENABLE 固定 `enabled_at=state_changed_at` 并保留既有 `disabled_at`，DISABLE 固定 `disabled_at=state_changed_at` 并保留既有 `enabled_at`，UPGRADE/ROLLBACK_VERSION 只更新 package/source/reason 与 `state_changed_at`，不抹除安装/启停时间。`source_config_item_id` 唯一且与 package FK 同一配置包；`V20261013093300` 补 `ON DELETE RESTRICT` 外键。每次动作的接受 bundle 摘要留在该不可删除 source item 的 `accepted_trust_bundle_sha256`，不向 current module 行复制易漂移的第二份值；current 与全部历史都能通过 source FK 精确取回。版本历史不另建表，完整事实保存在不可删除的签名 config package item 与审计链中。

## 5. 唯一运行时契约与 F-55 接入

`ep-platform-license` 是唯一实现方，对外契约替换为：

```rust
pub struct LicenseEvaluationV1 {
    pub status: LicenseStatus,
    pub restriction_reason: Option<LicenseRestrictionReason>,
    pub trusted_now: DateTime<Utc>,
}

pub trait ModuleLicenseQuery: Send + Sync {
    fn module_state(&self, module: ModuleCode) -> Result<ModuleState, AppError>;
    fn license_evaluation(&self) -> Result<LicenseEvaluationV1, AppError>;
    fn module_is_currently_licensed(&self, module: ModuleCode, legal_entity_id: Uuid) -> Result<bool, AppError>;
    fn entitlement_is_currently_licensed(&self, entitlement: EntitlementCodeV1, legal_entity_id: Uuid) -> Result<bool, AppError>;
    fn feature_is_enabled(&self, feature_code: &str, legal_entity_id: Uuid) -> Result<bool, AppError>;
}
```

查询逐次锁定或读取同一 current grant 快照，重建 payload、验 CMS/信任根/部署绑定、计算可信时间后再判范围；普通 query 不因“推进”二字写数据库。任何解析、签名或 current-slot 异常均失败关闭，不得任选一行或沿用缓存许可。`license_evaluation` 必须一次性返回同一快照导出的 status/reason/trusted time；有效三态的 reason 为空，Restricted 恰有一个第 3.3 节原因，调用方不得分三次查询拼出撕裂快照。

`module_state` 只返回 15-row registry 的 raw 管理投影，不能用于业务放行。`module_is_currently_licensed` 是唯一 effective runtime admission：在同一 repeatable-read snapshot 中，从签名 product DAG 计算“目标+传递依赖闭包”，并要求闭包每一行 raw=`INSTALLED_ENABLED`、current projection 与唯一 RELEASED source item/接受摘要/inner exact artifact/special outer exact bytes 全部自洽、inner 与 source outer 在当前 bundle 下均为 ACTIVE 或 RETIRED-nonrevoked、contract/version/dependencies 逐字匹配当前签名 product manifest，且同一 current grant 处于有效三态、`module_codes` 覆盖整个闭包、目标法人命中 signed scope。已知且结构完整的负态——未安装/disabled、依赖 disabled、未授权、scope 不命中、许可到期/撤销/无 current、已明确 signer CRL revoked——返回 `Ok(false)`；IO、strict parse、零/多 source、digest/signature/source/catalog/DAG/投影歧义返回 `Err(AppError)`，所有调用方都按不可用失败关闭。任何调用方不得以 `module_state==INSTALLED_ENABLED`、缓存布尔值或 bootstrap 显示值替代。

`feature_is_enabled` 先读取唯一 feature row，再无条件对该行 `module_code` 调用上述 effective gate；仅当 row `is_enabled=true` 且 owner module effective=true 才可返回 true。`requires_license` 只保留既有 feature 元数据含义，不能跳过 owner module/依赖/签名 product/current grant/scope 任一检查。core/worker 在取得第 4.2 节 shared lock之后、读取业务 payload/claim/dispatch 前调用 effective gate；所有 route/job/event/approval-owner/outbound/IPC registry 必须有唯一 owner ModuleCode，平台 AI/MCP 基础设施本身不虚构第十六个模块，AI/MCP 对具体业务对象执行 Write/Approve/Submit 时仍由该对象 owner module 的同一 registry/gate 保护。

`F55_LOCAL_AI` 同时控制 AI；`F55_MCP` 同时控制入站与出站 MCP，不存在方向许可。F-55 `currently_licensed` 在 `ACTIVE|EXPIRING_SOON|GRACE_PERIOD` 为 true，在 `RESTRICTED` 为 false；`purchased` 只表示 current/history 中仍能按当前 bundle 验签且未被标记 `HISTORICAL_SIGNER_REVOKED` 的 grant 曾含相应 entitlement，不放行业务。AI/MCP 平台入口先判 entitlement；若随后操作具体业务对象，再独立经过该对象 owner module effective gate，两者不能互相替代。

受限运行的唯一 admission 契约同样由 `ep-platform-license` 的 `crates/platform/license/src/admission.rs` 提供，internal enum wire 一律取 `SCREAMING_SNAKE_CASE`：

```rust
pub enum LicenseAdmissionEffectV1 {
    BusinessWrite,
    BusinessApproval,
    IntegrationOutbound,
    AutomationStart,
    ReadReportAuditBackupExport,
    IdentitySecurityDisposition,
    ComplianceDisposition,
    InFlightConvergence,
    LicenseGrantRecovery,
    ModuleDisableRecovery,
}

pub struct LicenseAdmissionRequestV1 {
    pub effect: LicenseAdmissionEffectV1,
    pub legal_entity_id: Option<Uuid>,
}

pub trait LicenseAdmissionGate: Send + Sync {
    fn admit(&self, request: &LicenseAdmissionRequestV1) -> Result<LicenseEvaluationV1, AppError>;
}

pub enum LicenseAdmissionBindingV1 {
    Fixed(LicenseAdmissionEffectV1),
    ConfigRelease { fallback_effect: LicenseAdmissionEffectV1 },
    McpInbound,
}
```

`admit` 在允许时返回本次同快照的 `LicenseEvaluationV1` 供审计，在第 3.3 节四类禁止副作用命中全局 Restricted，或命中有效 LIST 的法人 scope 缺口时，只返回 `PLATFORM.LICENSE.RESTRICTED`。后六个允许类不因这两种许可状态被阻断，但仍完整执行原认证、授权、RLS、签名、职责分离和审计。`legal_entity_id=Some` 时才判 LIST scope；部署级动作传 None，不能用 None 绕过本来已有目标法人的业务请求。

入口分类不能由 handler 临时传一个 enum 冒充。core-server 的每个外部 `/api/v1/`、`/portal` 与 `/mcp` method+规范 route template 都必须在既有路由注册元组中带一个 `LicenseAdmissionBindingV1`；认证前 sign-in/MFA 路由固定 `IdentitySecurityDisposition`，在凭证验证成功后、签发会话前执行。core-server/job-worker 的每个 scheduler、Outbox dispatcher、审批 owner callback 与对外 IPC operation 也必须在非 HTTP registry 登记。`Fixed` 用于静态效果；`ConfigRelease` 只允许用于 import、run-autotest、submit、approve、reject、sign、create-release-order、execute 八类配置发布入口，在 strict parse 与锁内重读目标 package/items 后把 `LICENSE_GRANT` 映射 LicenseGrantRecovery、把 `MODULE_PACKAGE:DISABLE` 映射 ModuleDisableRecovery，其余取 fallback（approve/reject 只能 BusinessApproval，其余只能 BusinessWrite）；`McpInbound` 只允许 `/mcp`，在已验签 manifest binding 解析后把 `Read|Export` 映射 ReadReportAuditBackupExport、`Approve` 映射 BusinessApproval、`Write|Submit` 映射 BusinessWrite。

两个 wiring 的 admission key exact-set 必须分别等于实际 route/job/event/approval-owner/outbound-operation registry，缺失、额外或重复都由 `xtask` 拒绝，并由只读静态自检 `license-admission-registry-consistent` 以 Blocking/退出 78 处理。该自检只比较编译期注册表，不读取许可、模块或客户业务状态，绝不恢复已删除的 `license-and-modules-consistent` 启动门。`InFlightConvergence` 只含外部副作用已经发生后的回执落库、终态记录、取消和不产生新受限副作用的补偿；首次或重试外发、领取新任务、产生新业务效果都不属于收敛，PENDING/DISPATCHING 的待外发项目在 Restricted 时原样保留并仍按 IntegrationOutbound 拒绝。

Stage 14 的 `F55EntitlementEvidenceQuery` 继续存在，但其唯一来源改为本节 current/history signed grant 投影；旧 `F55LicenseGrantPayloadV1`、把 entitlement 塞入旧 `signature` CMS 且不扩列的临时方案、`Valid|ExpiringSoon|Expired|Revoked` 与人工布尔值全部作废。证据 summary 状态使用本节四态，`currently_licensed` 包含 `GracePeriod`。

## 6. ServerAdmin 与既有 API

ServerAdmin 不新增后端路由。既有 `POST /api/v1/platform/config-packages/actions/import` 保留 Win/Mac 使用的 `application/json {attachment_object_id}`，并在同一路径、同权限增加可由**已认证 Win、Mac 或 ServerAdmin**使用的 `multipart/form-data` 形态；这样零 current 的 bootstrap Win/Mac operator 可以直接提交首张 LICENSE_GRANT，而无需先走会被 Restricted 阻断的通用附件上传，也无需伪造 ServerAdmin device/session。其 body grammar 唯一为：请求 `Content-Type` 恰为未加引号的 `multipart/form-data; boundary=<token>`，所以 token 必须同时是 HTTP token；产品再收窄为 1..70 ASCII bytes 且只含 `[A-Za-z0-9'._+-]`，拒绝空格、引号、括号、逗号、斜杠、冒号、等号、问号及其他字符。无 preamble/epilogue，CRLF-only；恰一个名为 `package` 的 file part，headers 顺序与 bytes 恰为 `Content-Disposition: form-data; name="package"; filename="<filename>"\r\n`、`Content-Type: application/vnd.enterprise-platform.epcfg+zip\r\n`，其后一个空行；filename 匹配 `[A-Za-z0-9][A-Za-z0-9._-]{0,121}\.epcfg`（7..128 ASCII bytes），零其他 header、part 或 form field；最后恰为 CRLF、closing boundary、CRLF。由此 framing size 恰为 `136 + 2*boundary_len + filename_len`、最大 404 bytes，file/archive hard cap 固定 4,193,900 bytes，合法最大 archive 可经该唯一路径到达。Restricted 下只有该 route 在 safe parse 后、并于 exclusive transaction 内从持久候选 exact bytes 再确认唯一 item=`LICENSE_GRANT` 时映射 `LicenseGrantRecovery`；普通包与非 DISABLE 模块包仍拒绝，通用 attachment create/upload 永不因此取得恢复权限。

该路径获得唯一的编译期 route-local body-limit 窄例外：`Content-Length` 必须存在且规范十进制值在 `1..=4,194,304`，并逐字等于 `framing_size+archive_size`；缺失、非法、为零、超限或任何 `Transfer-Encoding` 都在读取 body/创建 staging 前拒绝。流式读取同时以 4,194,304-byte body 与 4,193,900-byte file 两个硬截止拒绝短读/长读，其他路由与全局 1 MiB 上限完全不变。长度、boundary、framing、header、part、MIME、filename、扩展名、短读/长读或 archive 上限任一不符都稳定映射既有 `PLATFORM.REQUEST.INVALID_PAYLOAD`（HTTP 400、不可重试、零落库），不新增 413 或同义错误码。handler 只把 file bytes 写入 `C:\ProgramData\EnterprisePlatform\staging\config-import\<request-id>.epcfg` 的 CREATE_NEW 文件；目录 owner SYSTEM、断继承，SYSTEM/Administrators/ep-core 可管理，其余无 ACE；拒绝 UNC/device/reparse/ADS/hardlink，逐流算 digest，验完后无论成败关闭句柄并删除。该 staging 文件不是 attachment、不是通用文件能力，不能由任何下载、列表或 API 读取。为关闭进程崩溃遗留，core-server 在 public readiness 前必须以 fixed-root safe handle 枚举该专用目录；只认可名称为 canonical lowercase request UUID 加 `.epcfg`、regular file、single-link、无 reparse/ADS 且 owner/DACL 与目录契约相符的遗留并删除，任何其他对象先安全隔离、发不可抑制告警并阻断 readiness。每次 import 在取得本 request-id 的 CREATE_NEW target handle 前也执行同一受控 stale recovery；目录内容永不作为权威输入，恢复审计只记 file digest/size/result，不记绝对路径或内容。若数据库已提交，幂等键与 content hash 返回既有 package 后仍删除 staging；若未提交则允许同 request-id 从头重试。ServerAdmin 的 `platform.document_attachment=N/A` 保持不变。

向导只组合如下既有能力：

1. 选择本机签名 `.epcfg` 并以同一路径 multipart import；同一 strict transport 也供已认证 Win/Mac 的首张许可恢复使用，不赋予通用文件上传能力；
2. 显示内外签名、许可/模块动作、影响、当前/目标状态与数据保留声明；
3. 调用既有 autotest 与 submit-for-approval；审批结论只由 Win/Mac 的既有审批待办完成，ServerAdmin 只能只读显示待办与结论，绝不调用 approve/reject；批准后 ServerAdmin 可按既有权限调用 sign、release-order/execute；
4. 只读展示 current grant、四态、可信时间、用量、模块当前包与历史发布记录；不返回 signature 正文、绝对路径、私钥引用或秘密。

权限沿用现有 `lowcode.config_package.*` 与职责分离审批，不创设超级管理员。许可证和模块包只能由具备 import/submit 权限的人发起、由不同 `SECURITY_ADMIN` 在 Win/Mac 按 `CONFIG_RELEASE` 链批准，再由具备 release execute 权限的人执行。ServerAdmin 不能 direct DB/KMS/file/service 操作，也不能绕过签名或审批。许可证续期、撤销、模块启停和升级都不提供手工布尔开关。

导入成功固定写 `source=IMPORTED,status=DRAFT,approval_legal_entity_id=null`，并把发行方外层 signature/signer/signed_at 原样保存在 config package 既有三列；special package 从落库起内容不可修改。autotest 前后同样保持 approval 法人为空；submit 锁内重验派生治理上下文与当前会话授权后，才在进入 `PENDING_APPROVAL` 的同一事务首次写入该法人。批准后的 `actions/sign` 对 special package 只重新验证并保留这些 exact bytes，再迁移到 `SIGNED_PENDING_RELEASE` 和写审计，不调用部署 KMS 覆盖发行方签名；普通配置包仍按既有逻辑由部署密钥签名。special item `item_code` 唯一格式为 grant `license-grant.<lowercase-grant-uuid>`、revoke `license-revocation.<lowercase-revocation-uuid>`、module `<lowercase-module-wire>.<install|enable|disable|upgrade|rollback-version>.<lowercase-package-uuid>`，并逐项与 inner payload identity/action 相等。

当前许可/模块只读 DTO 不新增路由：`GET /api/v1/platform/client-bootstrap?client=server_admin` 增加可空字段 `license_module_admin`，仅当已认证 ServerAdmin 会话同时具有 `lowcode.config_package.view` 时填充，否则为 null；其他 client 逐字为 null。非空对象所有键始终存在，exact 字段为 `license_no_masked`、`license_kind`、`license_status`、`restriction_reason`、`valid_from`、`valid_to`、`maintenance_valid_to`、`last_trusted_at`、`usage`、`module_codes`、`entitlement_codes`、`modules`；前八项中的不可用 Option 逐字序列化 JSON null，绝不省略。`usage` exact object 恰有 `legal_entities,named_users,registered_devices` 三键，每个值的 `limit,current,over_limit` 三键也始终存在，形状恰为 `{limit:u32|null,current:u64,over_limit:bool|null}`；`limit=null` 时 `over_limit=null`，否则 `over_limit=(current>limit)`。即使没有可信 current grant 也返回三个实际 `current`，此时三个 `limit/over_limit` 均为 null。`license_no_masked` 对至少四个 Unicode scalar 的可信号码固定为 `"****" + 最后四个 scalar`，不足四个时固定为 `"****"`，不得暴露长度或其他字符；`package_version` 复用第 4.1 节 strict `SemVerV1` object。`modules` 恰有 15 行，每行 `module_code,display_name,install_state,package_trust_status,package_code,package_version,state_changed_at` 七键全部存在，后三个不可用值写 JSON null、不得省略；`package_trust_status` 闭集为 `NOT_INSTALLED|TRUSTED|SIGNER_REVOKED|INVALID`，从 current bundle 对 source item/投影重算，明确 `INSTALLED_ENABLED` 不等于 effective runtime 可用。零 current 时 status/reason 固定 `RESTRICTED/NO_CURRENT_GRANT`，其余许可身份/日期/可信时间均 null、两个 code 集为空；`SIGNATURE_INVALID` 时也不得显示来自未受信 current 行的身份、日期、code 或 limit。其余受限原因只显示已经完整验签的 current grant 脱敏字段。两个 code 集与 15 行均按 wire bytes 排序；unknown key、任一 missing key、把 null 改成省略或反之均为 OpenAPI/序列化契约负例；对象不含 signature、payload、source ref、path、key ref、secret 或原始 `license_no`。

## 7. 实施归属、迁移与目录

- Stage 3b：扩充 090100/090200/090500/093300；090500 建 item 表、18 值 CHECK 与 `accepted_trust_bundle_sha256`，093300 才补父候选键、六条 source FK 和跨表/不可变约束；同时更新 `ep-platform-license`、两个 applier、四态和运行时守卫；
- Stage 13b：既定 `V20261022090500` 一次把 item kind CHECK 与 Rust `ItemKind::ALL` 从 18 扩为终态 20 项，新增项只为 `MCP_CONNECTOR|MCP_MANIFEST_VERSION`；既定 `V20261022090600` 同时建立 config release 终态并按 Stage 13 §3.2.10 精确 seed 固定管理 API 的 30 个 permission item、12 个 object-scope binding、逐字段冲突断言且零自动 role grant，供本节 bootstrap 的 10 个 permission-action pair 从完整目录中引用；`epcfg` 支持生成/只读校验两种单项包；ServerAdmin 只复用同一 API；
- Stage 13c：AI/MCP 许可判定改读 `EntitlementCodeV1`；不再定义自己的许可 payload；
- Stage 14a/14b：证据 ABI 与 collector 改读 F-56；发布门禁新增 `RG-LICENSE-MODULE-LIFECYCLE-GREEN`，并把它列为 AI/MCP applicability 的共同前置；
- `docs/migration-catalog.md` 的总数保持不变；没有任何本裁定新增 SQL 文件。

实现顺序固定为 Stage 3b 许可/模块与 item kind 端口 → Stage 13b config package 终态/ServerAdmin 组合 → Stage 13c entitlement 消费 → Stage 14b 真实证据。任何阶段不得用临时 DTO、人工种子、环境变量或假签名跨过前置。

## 8. 必测矩阵与发布门禁

至少逐项覆盖：

1. permanent/subscription 正常、未来生效、60 天临期边界、到期后第 1/30/31 天、撤销立即受限、系统时钟倒拨；
2. 续期只能直接替换 current，两个并发续期恰一成功，旧行和签名证据保留；current signer 新近 CRL 吊销时只有 ACTIVE signer 的逐字 direct-successor 重签恢复可通过，任意漂移不能借路；
3. `.epcfg` 三 entry/ZIP32-STORE/canonical TOML/after-spec JCS+item hash/manifest 元数据绑定/outer CMS exact 前像逐项正反覆盖；内外签名篡改、wrong deployment、错误 scope、数组乱序/重复、unknown field、DEV root、链/撤销失败均零状态变更；
4. 三项用量超限只告警不阻断；法人、模块、AI/MCP scope 不足硬拒绝；
5. 五条模块合法动作与全部非法边；INSTALL/ENABLE/UPGRADE/ROLLBACK_VERSION 的目标+依赖授权闭包、ENABLE 的依赖启用态、启用态不能升级、降版本只能显式 `ROLLBACK_VERSION`；current inner signer CRL 吊销时业务运行关闭，只有 ACTIVE outer 对“DISABLE + 原样旧 inner”重新授权可停用，且 `MODULE_SIGNER_REVOKED_DISABLED` action 与 accepted event、previous/recovery ids、before/after projection、reason、时间及 hash chain 恰一闭合；缺失、重复、伪 peer、同 inner 多 recovery 猜选与 digest 漂移全拒，revoked 历史不能 rollback；跨 core/worker 共享锁、独占排空、30 秒超时、崩溃释锁和排队请求提交后重检全部通过；
6. 模块停用后界面/写入/定时任务/新 Outbox 停止，授权查询/报表/审计/备份/导出和数据 checksum 保持，再启用恢复；
7. 含两种特殊 item 的包非 IMPORTED、不是单项、带 before、外层法人覆盖或创建通用 ROLLBACK 时均稳定拒绝；ServerAdmin multipart 的 part/4 MiB/staging ACL/删除、只读审批与 Win/Mac 结论路径均通过；
8. restricted run 的允许/禁止集合逐项正反验证，尤其身份安全处置与合规处置不可误拦；
9. F-55 的 AI/MCP purchased/currently licensed 只从同一 signed grant 重算，GracePeriod 可用，Restricted 关闭业务路由；
10. fresh PostgreSQL 验证 090100/090200/090500/093300 的列、CHECK、`accepted_trust_bundle_sha256` 一次写入/普通项恒空/发布态形状、093300 父候选键与六条 source FK、grant/source 摘要相等和 catalog 描述逐项相等；Stage 3 `ItemKind` 恰 18 项，Stage 13 的 090500 ALTER 后恰 20 项。
11. admission exact-set 覆盖全部 HTTP/MCP route、scheduler、Outbox、审批 callback 与外发 IPC operation；制造缺项、额外项、重复项、ConfigRelease/MCP 错 resolver、法人 scope 绕过和把首次/重试外发伪装 InFlightConvergence，均须在 xtask、启动自检或运行前门禁失败；共同 Stage 14 gate 必须绑定该 registry digest 与正反测试证据；trust rotation evidence exact-set 必须覆盖全部 RELEASED special items，允许的历史 CRL 隔离与必须失败的历史漂移不能混淆。

`RG-LICENSE-MODULE-LIFECYCLE-GREEN` 只有在以上自动测试、真实 PostgreSQL、配置包全链和 Stage 14 签名证据均通过时为绿。证据缺失只阻止相关能力启用/发布，不改变本文设计，也不阻止按本文开始开发。

## 9. 无未决声明

本文范围内没有产品、商业模型、状态、数据列、签名、API、审批、回退、迁移编号、AI/MCP entitlement 或发布判据留给开发者选择。客户名称、许可金额、实际模块清单、许可日期、用量上限、签名主体与部署标识是每份已签名商业/部署输入的实例值，不是设计未决。未来若要支持可执行原生模块包、模块自带 SQL、在线许可证服务器、远程强停机、应用商店式模块市场或第三种许可类型，必须另立高于 F-56 的裁定。
