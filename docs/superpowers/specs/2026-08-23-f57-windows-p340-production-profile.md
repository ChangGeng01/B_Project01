# F-57 Windows Server 2022 / P340 生产档案

> 日期：2026-08-23（Australia/Melbourne）
> 状态：已批准硬件和生产设计输入；尚未完成 Windows 实机、容量、恢复或发布认证
> 开发门：`DESIGN_READY` / `DEVELOPMENT_AUTHORIZATION_REQUIRED`；首次获开发授权后只可从收敛计划 G0 开始，本文描述的 F-57 安装、运行时、探针和证据链全部仍为 `NOT_IMPLEMENTED`
> 适用硬件：ThinkStation P340 Tower、i5-10500、32GB RAM、256GB SSD、单 1TB HDD
> 适用系统：Windows Server 2022 原生服务
> 权威关系：[F-57 总体设计](2026-08-23-f57-governed-automation-fabric-design.md) 的生产实施附录；生产运营 exact 契约见[客户端、生命周期与安全运营执行契约](2026-08-23-f57-client-lifecycle-security-contract.md)

## 1. 结论

当前机器只有在本档案全部上线门通过后，才可以作为约 20 名活跃用户的低资源 `SINGLE_DISK_DEGRADED_PRODUCTION`（单磁盘降级生产）节点；通过前只是候选硬件，不能承载真实客户数据：

- 本地 AI 模型关闭；
- 所有持久客户数据和衍生数据位于加密 HDD；
- SSD 运行 Windows、程序和可重建静态资源，并只允许 master 冻结的 exact 四类无客户数据运行控制/代码状态；
- 大型报表并发为 1，其他重任务排队；
- UPS、服务器外追加式备份、离线轮换备份和完整恢复演练在录入真实数据前完成；
- 权威状态值和界面必须同时显示 `SINGLE_DISK_DEGRADED_PRODUCTION` / “单磁盘降级生产”，不承诺磁盘故障连续运行、硬件热插拔或统一四小时恢复；
- Windows Server 驱动、TPM、BitLocker、存储、网卡、散热和断电恢复实机验证通过。

现有 P340、256GB SSD 和单块 1TB HDD 单独存在时永远不能达到本项目最高安全档生产。上线最少还要实际取得并验证：兼容 UPS；由独立身份管理、位于服务器外且只追加的自动备份落点；至少两块加密、交替使用且非窗口期物理断开的离线轮换 HDD；彼此分域并离线双人保管的 BitLocker、应用 vault 与备份恢复材料；以及一台洁净恢复主机或可验证、能在演练窗口独占使用的临时恢复硬件。缺少任一项只能开发/测试，不得录入真实客户数据。

P340 是工作站，不是具备冗余电源、热插拔背板、ECC 认证证据和服务器级 BMC 的服务器。部分配置可能选配 Intel AMT，但 AMT 不等同于具备独立电源、独立管理网络和完整硬件遥测的服务器 BMC；在本机实证前不得把它计作带外恢复能力。本档案只定义受控使用方法，不把工作站改写成服务器级高可用硬件。

## 2. 冻结硬件档

| 字段 | 值 | 影响 |
|---|---|---|
| `hardware_profile_id` | `THINKSTATION_P340_I5_10500_32GB_256GB_SSD_1TB_HDD` | 与 runtime topology/P340 wire 相同的容量、节流和诚实状态唯一标识；硬件变化必须生成新 ID |
| CPU | i5-10500，6C/12T | 设计目标为 20 人普通业务、不承载本地模型；是否足够只由实机容量证书确认 |
| RAM | 32GB | 需要有界连接、worker 和缓存；不得按旧 64GB 预算 |
| OS SSD | 256GB | 承载 OS、程序、依赖、可重建静态资源及 exact 四类无客户数据运行控制/代码状态；闭集外持久字节失败 |
| Data HDD | 单 1TB | 所有权威/衍生数据；单点故障、低随机 IOPS |
| OS | Windows Server 2022 | 原生 SCM/ACL/防火墙/BitLocker；无 WSL/Linux 依赖；不是永久平台，须在支持生命周期内完成后继 LTSC 迁移认证 |
| Active users | 20 | 容量认证基线，不是许可或硬登录上限 |
| Heavy report concurrency | 1 | 报表请求异步接受，后台单实例执行 |
| Local model | Disabled | provider 契约存在但无模型进程和资源预留 |

### 2.1 最高安全档的低成本最低物料类别

下表是“复用现有主机后，进入最高安全档生产认证流程至少不能缺什么”的类别级 BOM，不指定品牌、型号或采购价格，也不表示买到物料即可上线。站点可选择更便宜的合规产品，但每一项都要通过本档案的兼容、容量、身份分离、断电和恢复证据；用同一台主机、同一身份或同一块盘兼任多项，不算补齐。

| 类别 | 最低数量/形态 | 最低用途与限制 |
|---|---|---|
| 权威主机 | 复用现有 P340 一台 | 运行 Windows Server 2022、Rust authority 与 PostgreSQL 16；仍是单机、单电源、无热插拔的工作站风险档 |
| 权威数据盘 | 现有 1TB HDD 一块，须为健康、可认证的 CMR；不满足则更换 | 承载全部权威与衍生持久数据；单盘只能显示降级，未来两块匹配企业 CMR HDD 镜像是优先可用性升级 |
| UPS | 一台可通信、可自检且实测运行时间足够完成安全关机的设备 | 只能经第 11 节批准 carrier 接入；普通插排、无通信 UPS 或只写标称 VA/W 不合格 |
| 服务器外连续目标 | 一个独立身份、独立故障域、HDD-backed、只追加且容量达标的目标 | writer 不能枚举历史、删除、覆盖、改 ACL 或缩短保留；本机目录、同盘分区、VSS 或普通可写共享不合格 |
| 离线轮换介质 | 至少两块不同 `media_id` 的加密 HDD | 交替形成已验证 generation，非受控窗口物理断开并异地/分区保管；不能同时常连 |
| 备份恢复凭证 | 三枚相互独立的 PIV token/recipient，由三名不同保管人持有 | 采用 ADR-0024 的 2-of-3；厂商、writer、目标和任何单人均不能恢复 |
| 应用数据与 secret-vault 恢复凭证 | **另三枚**相互独立的 PIV token/recipient，由另外三名保管人持有 | 采用 ADR-0020/`PIV_SHAMIR_2_OF_3_V1` 恢复应用 vault、字段/附件/归档 DEK 和 pre-DB secret；与备份三枚 token、recipient set、信封、PIN、保管人和轮换仪式完全分域，禁止复用 |
| 洁净恢复能力 | 一台专用，或在演练窗口可独占且容量达标的兼容主机与 HDD 工作空间 | 必须能从洁净 Windows Server、离线安装包和客户持有材料完成恢复；原 P340、原 SSD 和厂商在线服务都不是唯一前提 |

此 BOM 只把支出压到满足已批准安全边界的最低类别，不以低成本为由删除 UPS、服务器外只追加层、两块离线介质、**两套共六枚且保管人不重合的应用/备份 PIV 恢复凭证**或洁净恢复能力，也不把 RAID1、暖备和备份互相替代。

## 3. 已接受但不能消除的风险

| 风险 | 现行处置 |
|---|---|
| 单 HDD | 显示降级状态；服务器外连续备份和离线副本；未来 RAID1 |
| 单电源 | 当前 UPS 为硬门；断电演练；未来更换服务器级硬件可降低 |
| 单网卡 | 网卡、端口或交换链路故障即服务中断；不承诺网络冗余，必须保留现场恢复路径 |
| 无热插拔背板 | 更换/增加内部 HDD 必须关机维护 |
| 未证明具备服务器级 BMC | 管理失联时需要现场处置；即使发现选配 AMT，也不把它等同 BMC 或据此承诺远程硬件恢复 |
| 当前配置无 ECC 认证证据 | 内存测试、校验和、备份与恢复；残余位翻转风险必须披露 |
| Windows Server 非 P340 官方预装目标 | 对实际 BIOS/驱动/控制器逐项认证；不能靠同型号猜测 |
| 单机权威 | 主机故障即停机；未来暖备才提供快速提升 |

Lenovo P340 Tower 的机位、控制器和官方操作系统范围以 [官方 PSREF](https://psref.lenovo.com/syspool/Sys/PDF/ThinkStation/ThinkStation_P340_Tower/ThinkStation_P340_Tower_Spec.PDF) 为硬件核验来源；实际机器的托架、线缆、SATA 端口和控制器仍需现场检查。Windows Server 2022 的支持边界以 [Microsoft 生命周期页面](https://learn.microsoft.com/en-us/lifecycle/products/windows-server-2022) 为准；该页面列出的 PT 日期为 Mainstream End **2026-10-13**、Extended End **2031-10-14**。这些是本档案记录的官方日期，不是产品内硬编码常量；每次安装、生产认证和后继 LTSC 裁决都必须重新查询官方生命周期并保存查询时间与证据。主流支持结束不是自动停产条件，但其后签发新生产证书必须同时具备补丁来源、支持策略、客户风险接受和已排期的 Windows Server 2025/后继 LTSC 迁移认证；扩展支持结束前必须完成迁移，不得把 2022 当作永久平台。

## 4. 双卷和路径政策

本节的 `HDD_STRICT` 约束只作用于 Windows Server 权威节点上的内容承载或可关联客户的持久数据；Workbench 终端可按离线协议保存最小、加密、可撤销、非权威缓存。实现不得只检查 `C:`/`D:` 字符。第一阶段所有业务相关 Windows 卷只认证 GPT + NTFS，ReFS、FAT 和 exFAT 均拒绝。BitLocker 固定使用 Windows software encryption 与 XTS-AES-256，禁止以不透明硬件自加密代替；OS、数据和离线介质必须在载入真实数据前达到 100% 加密。签名 deployment manifest 与容量证书必须绑定 filesystem、GPT、volume GUID 与 serial、物理磁盘和控制器身份、logical/physical sector、cluster size、write-cache/flush 行为、encryption method/status 和 protector exact-set；任一变化均触发重新验证。

### 4.1 SSD 软件卷

允许：

- Windows Server 2022；
- 签名产品程序和 DLL；
- 无客户数据的静态 Web/UI 资源；
- 可重新安装依赖；
- 可重新下载且不包含客户训练数据的模型文件；
- 非秘密的验证信任锚和公开证书，包括由安装器、WDAC 与客户受控登记保护的客户公钥；私钥、恢复材料和应用主密钥仍禁止。
- 可重新登记、受 TPM 约束的 machine-key/certificate-store binding，以及已签名 Set A 清单中逐项有界的 Windows/product/cache metadata；
- 唯一 mutable Set B：有界 POWER capsule、有界 package-recovery continuation capsule、recovery-domain-signed kernel pointer/journal head、可重建的 content-addressed signed native-code slot/cache。四类各自必须满足固定 media/path、大小、保留、off-host mirror、终态删除与 SSD-loss 重建契约；任何第五类或客户/业务 authority 字节均禁止。

禁止：

- PostgreSQL data/WAL/temp；
- 客户配置代或客户能力包；
- 业务日志、附件、索引、报表和导出；
- 客户凭据正文；
- 含业务载荷的 pagefile、dump、TEMP、WebView/browser cache、Defender quarantine 或打印 spool。

### 4.2 HDD 权威数据卷

规范根目录由签名 deployment manifest 的 `data_root` 指定，子目录固定归类：

| 子目录 | 内容 | 备份 |
|---|---|---|
| `postgres/data` | PGDATA，包含 live `pg_wal` 与数据库临时关系 | 连续 + 全量 |
| `postgres/wal` | 仅 WAL archive staging；不是 live `pg_wal` | 连续 |
| `postgres/temp/process` | PostgreSQL 进程 TEMP/TMP；不是用户 tablespace | 不单独恢复，但必须在 HDD |
| `postgres/temp/restore` | PostgreSQL 恢复 scratch；不与进程临时目录混用 | 不单独恢复，但必须在 HDD |
| `files` | 附件和版本 | 连续 + 全量 |
| `audit` | 审计链与本地 checkpoint | 连续 + 全量 |
| `indexes` | 搜索、报表和派生索引 | 可重建但仍在 HDD |
| `packages` | 客户配置代、客户能力包和 exact manifest | 连续 + 全量 |
| `logs` | 可能含业务关联的应用日志 | 按保留策略 |
| `temp` | 导入、文档、OCR、AI/MCP 临时业务文件 | 加密、限时清理 |
| `spool` | Outbox、集成、备份和归档暂存 | 连续状态 |
| `exports` | 待领取导出和报表文件 | 加密、到期清理 |
| `plugin-work` | WASM/worker/container 受控工作区 | 按 manifest |
| `quarantine` | 未通过附件扫描的对象 | 加密、禁止执行 |
| `secrets` | 客户凭据密文和秘密库元数据 | 独立加密与恢复 |
| `dumps` | 获批诊断 dump | 默认禁用；启用时限期处置 |
| `backup-staging` | 本地加密备份暂存 | 不计作独立副本 |

### 4.3 OS 泄漏通道

生产安装必须明确处理：

- pagefile 移至 HDD 或禁用，并验证 32GB 内存档下不会因禁用导致不稳定；
- 禁用 hibernation 与 Fast Startup；禁止 `hiberfil.sys` 保存业务进程内存；
- WER、服务 crash dump 默认关闭，获批时写 HDD；
- VSS/shadow copies 不得在 SSD 保存客户内容，也不得计作备份层；Windows Search 禁止索引业务根；
- ETW、HTTP.sys/IIS（若启用）、Defender/EDR operational log 只允许固定事件码和随机 incident ID，不得记录客户值、请求/响应正文、对象 ID 或可反查 digest；
- `%TEMP%`、服务 profile temp、WebView cache 和本地控制中心浏览器 profile 写 HDD；
- Windows Event Log 只写固定事件码和与客户/对象不可关联的随机 incident ID；禁止客户值、对象 ID、客户正文、客户正文哈希或可反查 digest；
- Defender/扫描器 quarantine 写 HDD；
- RDP clipboard/drive/printer redirection 默认关闭；RDP bitmap/cache 和获批管理 profile 不得承载客户正文；
- Print Spooler 默认禁用；服务器禁止作为普通打印、邮件、浏览和办公机器，避免系统 spool/cache 引入业务内容。

F-57 最终发布门必须在完整混合负载下以实际打开后的 final handle、volume GUID 和设备证据跟踪全部写入；junction、reparse point、mount point、符号链接、路径替换与检查后改向必须进入 TOCTOU 负例。证明 SSD 没有内容承载或可关联客户的持久字节，未知路径即失败，不能以字符串路径检查或“没有观察到问题”代替登记。

## 5. 秘密和密钥

客户数据库口令、外部连接器凭据、API token 和其他客户秘密不得存入 SSD 上的 WinCred、服务 profile 或明文文件。

现行低成本方案：

1. 凭据正文以 envelope encryption 保存在 HDD `secrets` vault；
2. 每用途/法人使用独立 data key；
3. data key 由非导出 wrapping key 包装；
4. wrapping key handle 位于 TPM、客户 HSM 或 KMS，不把主密钥正文写入磁盘；
5. 服务通过 secret broker 取得短期、作用域化的内存凭据；
6. 日志、dump、错误、环境变量和命令行不得出现 secret；
7. 轮换、恢复和销毁要求双人批准与审计。

如果 P340 TPM、驱动或恢复证据不通过，则生产必须连接客户 HSM/KMS；不能降级为 SSD 凭据文件。

OS SSD 的 BitLocker 自举只允许同一最高安全档下的两个互斥模式。当前 P340 基线固定 `TPM_ONLY_UNATTENDED`：OS SSD 使用 TPM/PCR protector，并以 Secure Boot、锁闭机房、UEFI/外部启动门禁和实测 UPS 后无人值守重启补足物理边界。若部署签名切换到 `TPM_PIN_ATTENDED`，则增加启动 PIN，但必须撤销无人值守恢复声明，配置启动失败告警，并单独实测有人值守 RTO；不得同时宣称两种模式的优点。DATA_HDD 不使用 Windows fixed-data auto-unlock；其 protector exact-set 固定为 `{PUBLIC_KEY,RECOVERY_PASSWORD}`。只有 Secure Boot、PCR 与 OS trusted boot 校验通过后，独立、最小权限、无出站网络的 `EPF57DataVolumeUnlockBroker` 才可从九个签名约束的 pre-HDD Set-A locator 验证 policy、certificate chain、bootstrap authority、目标卷身份和 TPM NV anti-rollback，重开现有 Microsoft Platform Crypto Provider 中 TPM-backed/nonexportable 的证书私钥，并以固定非零 thumbprint、空 PIN、本机 packet-private WMI 调用 `UnlockWithCertificateThumbprint`。Authority、恢复任务和普通管理员均不得取得该私钥或 48 位 recovery password。SSD 的密钥相关窄例外仅包括 Windows/BitLocker 自身管理数据、受界限约束且可重新登记的 TPM-bound machine-key/certificate-store binding 与非秘密 locator/trust metadata；应用主密钥、客户秘密正文和可导出 wrapping key 仍禁止。OS SSD 与 DATA_HDD 的 recovery password 分别生成、服务器外保存、双人保管，并与应用 secret vault、备份和客户数据密钥恢复材料分域。普通重启必须证明 broker unlock 且 `fixed_data_auto_unlock=false`；TPM/主板或 OS SSD 损坏时不得从 DER/SPKI/TPM handle 重建旧私钥，只能在 admission closed 下由两名授权人使用服务器外 recovery password 解锁，创建新 TPM key/certificate/PUBLIC_KEY protector、提升 authority epoch 与 TPM NV、完成一次普通重启验收后移除旧 protector。上线与每次相关变更都必须演练对应路径及 recovery-password 被盗后的轮换和影响边界。

应用凭据不能以 PostgreSQL 表作为唯一自举真相，客户 deployment manifest 也不得放入 SSD software root。启动顺序固定为：验证签名二进制/WDAC，以及 SSD Set A 中 exact 九个非秘密 pre-HDD locator、客户公钥信任锚、证书策略/链、bootstrap authority 和 TPM NV head；按已签名 OS BitLocker 模式完成 Secure Boot/PCR/TPM（PIN 模式还需有人输入 PIN）验证；由 `EPF57DataVolumeUnlockBroker` 对编译绑定的唯一 DATA_HDD 执行 explicit-thumbprint PUBLIC_KEY unlock，并读回正确卷、protector、证书、mounted/dirty-clear 与 `fixed_data_auto_unlock=false`；从已验证 HDD `packages` 读取 detached-signed pre-DB deployment manifest；验证其签名、撤销状态、final volume identity，并与 TPM NV/sealed 的单调 revision 和 digest 对照防回滚；正常启动用日常 TPM/HSM operational recipient 解开 HDD 上的 pre-DB secret-vault envelope 和数据库凭据，洁净恢复则用独立离线的 `PIV_SHAMIR_2_OF_3_V1` recovery recipient（固定 3 份 share、任意 2 份重构并由不同保管人实行双人控制），两个用途不能互相调用；首次连接 PostgreSQL；再把数据库内 key/vault metadata 与 manifest digest 对照。locator/policy/certificate/broker/volume readback、secret、manifest 或 root 任一缺失，root 与 manifest 同时替换、revision 降级、manifest 位于 SSD、DATA_HDD 仍锁定或 data root 指向 SSD 时，数据库连接次数必须为零。

信任根变更与 deployment manifest 变更不得在同一个批准中完成；恢复材料必须保存 exact manifest、信任根、撤销信息、最后已接受 revision 和服务器外签名 checkpoint，防止本地管理员同时替换清单与根后把 SSD 伪装成合规数据卷。

## 6. Windows 服务与信任边界

产品契约不冻结进程数量，但当前安装至少按以下信任角色分离 Windows 服务身份：

| 角色 | 责任 | 禁止能力 |
|---|---|---|
| Authority | 强类型 API、事务、授权、审计、控制中心后端 | 不直接出任意外网 |
| Automation worker | 耐久任务、计时器、Outbox 和对账 | 不签配置代、不授予权限 |
| Plugin host | WASM、签名 worker、可选 Windows 容器 | 无 DB/KMS 主凭据 |
| Integration gateway | 经批准的外部网络连接 | 无 DB、文件库或全局网络 |
| Portal gateway | 客户/供应商投影和命令入口 | 无 DB、管理 API 和内部文件 |
| Ops agent | 健康、容量和证据采集 | 无业务写和客户正文读取 |
| Backup/Archive | 备份写出和校验 | 无业务命令；不能删除历史目标 |
| PostgreSQL | 权威存储 | 不出网、不运行交互登录 |

PostgreSQL 16 必须按唯一的 `Postgres16WindowsPackageLockV1 -> Postgres16WindowsInstallContractV1 -> Postgres16WindowsInstallReadbackV1` 链运行：V1 永不允许降级，package `.control`/SBOM/扩展集合 exact-match；服务 `ep-postgres16`，显示名 `Enterprise Platform PostgreSQL 16`，账户 `NT SERVICE\ep-postgres16`，`UNRESTRICTED`、按需启动、无自动重启恢复动作。九个路径角色按固定顺序唯一出现并与 live final-handle readback 双射；SSD Set A 只放 package-lock 固定的 engine，`pg_ctl runservice -N ep-postgres16 -D <signed-PGDATA> -w` 的 PGDATA/config、live WAL、TLS、日志、archive staging、`temp/process` 和 `temp/restore` 均落在 DATA_HDD。依赖 unlock broker 只保证顺序，不代表 ready；Authority 必须在同一 boot 的卷解锁、storage manifest、vault、配置和 TLS 读回全部通过后显式启动。关键 GUC、HBA、ident 采用冻结 canonical vector，effective vector 必须逐字节相等；监听集合严格为排序去重的 `127.0.0.1|::1`。服务使用 TLS/SCRAM/channel binding、无 `trust`/外网 CIDR/ambient include/用户 tablespace，`fsync`、`full_page_writes`、`synchronous_commit`、checksums、`wal_level=replica` 与 `archive_mode` 均按签名投影开启。禁止 `initdb --waldir`，避免用 reparse 把 live WAL 从 PGDATA 分裂。

模块化业务能力可以装配在 Authority 主体内；只有信任、资源或公开入口边界才拆进程。服务使用 Virtual Service Account 或独立 gMSA/客户账号，NTFS ACL、Windows Firewall、Job Object 和 named pipe ACL 按角色最小化。

## 7. 网络、TLS 与服务器使用规则

- Windows Firewall 默认拒绝入站和出站；逐端口、目的地和服务身份批准。
- PostgreSQL 仅本机受控访问，不对办公网或公网开放。
- Authority 办公入口、管理入口和门户入口使用不同路由/站点和证书策略。
- Rust TLS adapter 是产品默认终止点，私钥由 secret broker 提供；客户已有 WAF/反向代理可作为外部 provider，但不能绕过服务端认证。
- 管理入口只允许管理网络/VPN/受控跳板，要求 MFA、重新认证、CSRF/CSP 和会话到期。
- RDP 默认关闭；必须使用时限定来源、MFA、时段和人员，并审计。
- 服务器不收发普通邮件、不浏览互联网、不运行 Office、不作为文件共享或普通用户桌面。
- 外部插件/连接器仅按 manifest 允许域名、IP、端口、代理和重定向；普通出站恒拒绝。

生产附件必须通过经批准的 Defender/AMSI/ICAP 或等价扫描 provider。扫描 unavailable/timeout/unknown 时对象保持 quarantine，不能退回 `NONE`。

## 8. 资源优先级和 admission control

资源保障顺序：

1. PostgreSQL commit、WAL 和恢复状态；
2. 身份、授权和审计；
3. 增量备份；
4. 普通查询和保存；
5. 到期自动化；
6. MCP、OCR 和连接器；
7. 批量导入导出、报表、AI 和维护。

实现要求：

- 数据库、HTTP、worker、plugin 和文件通道均为有界队列；
- 不使用无界 Tokio task、无界 channel 或无界客户端轮询；
- 大型报表请求在 2 秒内返回已受理或明确拒绝，后台并发固定 1；
- 本地 AI 不启动，也不预留旧 F-55 的模型内存/pipe 容量；
- HDD latency 或 queue 升高时先暂停低优先级任务；
- 备份不可无限抢占交易，但备份保护超窗必须升级为不可抑制风险；
- 所有暂停任务保留 durable checkpoint，资源恢复后继续。

## 9. 容量和磁盘安全线

### 9.1 空间公式

```text
emergency_reserve = max(20 GiB, data_volume_capacity × 5%)
yellow_free       = max(emergency_reserve × 2, p95_daily_growth × 30)
red_free          = emergency_reserve
```

- 低于 `yellow_free`：停止新大型导入、导出、报表和非必要索引重建；
- 低于 `red_free`：释放预留后只允许完成在途事务、审计、安全处置和受控停机；
- 无法写审计/WAL：权威写失败关闭；
- 预留空间不得被普通文件或插件占用。

旧 800GB/2TB 认证数据集和统一 4 小时恢复不适用于当前 1TB HDD。

### 9.2 20 人混合负载

容量证书必须通过 barrier 证明 20 个不同且已认证的业务 principal 在同一重叠窗口真实处于活跃交互，不得用同一账号并发、60 秒内顺序打点或后台任务凑成 20 人。这个 20 人是全部使用端的聚合 envelope，身份/入口 exact mix 为 **15 个 Workbench + 3 个客户门户 + 2 个供应商门户**；此外必须同时存在 **1 个 Control Center 管理会话**，它不计入 20 人，但使用独立保留资源，不能因业务负载被饿死。

同一认证窗口还必须让上述 20 个 principal 覆盖第二组业务动作 mix；入口 mix 和动作 mix 是两个同时成立的维度，不要求一一按固定岗位映射：

- 11 名用户列表、搜索、详情和统一时间线；
- 5 名用户创建、编辑和保存客户/合同/订单/工单；
- 2 名用户执行审批或高风险重新认证；
- 2 名用户上传/下载附件；
- Workbench 登录/重连 burst 与门户附件上传 burst；
- 1 条采购/财务/售后闭环自动化；
- 1 个大型报表请求在后台运行；
- 同时运行增量备份、审计 checkpoint 和健康采集。

容量证书必须绑定 software/config generation、PostgreSQL 版本、各核心表行数、附件总量和大小分布、索引与 WAL 规模、HDD 已用比例、最近 30/90 日增长分布以及后台队列初始值。空库、小样本、低填充盘或缺少增长压力的数据不得签发“20 人”结论；任一 generation、磁盘型号/固件、内存、扫描器或关键索引变化均按影响重新认证。

局域网且排除文件传输/外部服务等待时，普通交互读 P95 目标不超过 2 秒，普通权威写 P95 目标不超过 3 秒；不达标时容量状态为黄/红，不允许放宽安全规则来达标。

## 10. 监测、降级和健康状态

必须采集并保留在 HDD：

- CPU、working set、commit、page fault；
- OS SSD 的 SMART、剩余寿命、温度、掉盘/重试、free space 与 Windows 更新/回滚预算；
- HDD latency、queue、throughput、SMART、温度、坏扇区和 free space；
- PostgreSQL commit/WAL、锁、长事务、连接和 checkpoint；
- HTTP/worker/plugin queue；
- 自动化 run、effect unknown、retry、incident；
- 备份年龄、离线副本年龄和最近恢复演练；
- UPS 电源、电池和安全关机结果；
- Windows 服务/驱动/BitLocker/Defender/防火墙状态。

健康输出必须区分：性能降级、保护缺失、单盘风险、备份超窗和发布阻断，不能只给一个绿色圆点。

## 11. UPS 与断电

UPS 是当前上线硬门，不是未来升级项。`crates/platform/ups-contract` 与 `docs/evidence/f57-ups-contract.v1.schema.json` 唯一拥有 `UpsAdapterManifestV1`、状态、命令、ACK 和分离的 `UpsStatusPortV1|UpsOutletControlPortV1`；P340/发布只导入。Authority 只通过现有 `EPAuthorityControl` 内静态链接、候选绑定的第一方 Rust adapter 调用这些端口，不新增服务，不加载厂商 DLL，不启动厂商子进程，不让 UPS 软件直连 PostgreSQL。首发 carrier exact-set 为：

- `WINDOWS_STANDARD_POWER_STATUS`：仅把 Windows 聚合的 AC、电池供电、电量和剩余估算映射成 typed status；`255|0xFFFFFFFF` 和不可取得的 communication/self-test/output 均保持 `UNKNOWN`，控制端口固定返回 `CAPABILITY_INSUFFICIENT`。它只能作工程监视/告警兜底，不能满足最高生产档、受控 outlet 或 POWER 证据；
- `SIGNED_VENDOR_ADAPTER`：候选 artifact set 锁定 manifest、实现字节、配置 schema、设备 profile、能力、transport 和 credential policy，并提供型号、序列号、固件、self-test、output、告警、幂等 outlet schedule 与同 command-ID 查询。最高档固定要求此 carrier。

首版 manifest 的 adapter 版本只经自定义 parser 接受无 build metadata 的规范 Semantic Versioning 2.0.0 字符串。轮询/状态最大年龄/provider 自检最大年龄/命令 ACK 最大等待固定为 `5/15/86400/30` 秒。标准 carrier 的 `supported_device_profiles` 必须为空，只能声明 AC、电池供电、电量和剩余时间四项能力，并固定为 Windows system status + 无凭据；vendor carrier 的设备 profile 必须非空、按 `device_profile_id` 严格排序且不得重复，每个 profile 的固件 revision 与受控 outlet group 都必须非空、严格排序且不得重复，并声明完整十项能力、精确选择一个已列 profile。USB/local-device 只能配 service-SID device ACL，并使用 canonical 小写 GUID、uppercase Configuration-Manager device instance 与 exact digest；HTTPS mutual TLS 只能配不可导出的 CNG 客户端证书；SNMPv3 authPriv 只能配 DPAPI-NG service-SID sealed secret。网络 endpoint 使用 numeric-IP octets + nonzero port 的 structured row，运行时 exact 一行，不接受 DNS、文本/IP 别名、重复或额外目标；任何交叉组合均拒绝。

标准状态的 `device_profile_id=null`，其 `ups_adapter_identity=sha256(JCS({carrier_kind,adapter_manifest_ref,adapter_configuration_sha256,configuration_generation}))` 只表示 Windows carrier/配置身份，不冒充 UPS 硬件身份。vendor 状态的 profile ID 非空且精确命中 manifest，硬件身份固定为 `sha256(JCS({carrier_kind,adapter_manifest_ref,device_profile_id,manufacturer,model,serial_number,firmware_revision}))`；vendor 身份读回、状态、主机指纹、命令和 ACK 都必须重复该 nominal digest。每个状态的 runtime binding digest 必须等于 signed identity 内 runtime-security readback；initial/previous/trigger status 逐一 join，POWER 前后两个状态还必须同 boot/PID/start-key。sequence 只在该 binding 从 1 递增，进程重启须先签新 identity 才可重置。标准 self-test 恒为 UNKNOWN；vendor PASS/FAIL 必须带 UPS-owner 解析的原始 provider attestation，P340/POWER 只接受非未来且 24 小时内的 PASS，操作 05 仍只读，过期时阻止并要求实际运行自检。`valid_until=observed_at+15000ms`，已知电量必须在 `0..=100`，告警枚举严格排序且不得重复，只接受该绑定下最新且未过期状态。adapter 必须在任何 provider 调用前耐久化同一 boot/source 的私有 monotonic start marker；ACK 只能在 `start <= observed <= min(start+30000,command deadline)` 时成立。响应丢失只可在该内层期限内 query/adopt 原字节 ACK；30 秒时未知即 `COMMAND_STATE_UNKNOWN` 且禁止重发。两项 UTC 时间仅供报告，POWER 外层 600 秒只作 User32/composite/preshutdown 对账，不能放宽、重置或复活 30 秒内层期限。

运行安全读回也采用闭合矩阵：标准模式不带 device ACL/firewall ref 且零目标；USB/local-device 必须带 device ACL 与 deny-all firewall 两个读回且零目标；网络模式不带 device ACL，必须带 exact-endpoint firewall 读回并只观察到 manifest 中那一个目标。两类读回字段均为 `application/octet-stream`，只由 UPS owner 的 `EP_F57_UPS_DEVICE_ACL_READBACK_V1|EP_F57_UPS_FIREWALL_READBACK_V1` 解析，不增加 release POWER opaque parser、schema 或 signer；所有模式的意外模块、子进程和凭据导出/泄漏计数必须为零。

生产必须取得新鲜 `UpsStatusReadbackV1`，已知且通过在线/剩余运行时间/通信/自检/output 状态，并逐字绑定 manifest、nominal adapter identity、配置 digest/代、实际服务 SID/PID/start-key/held binary、UPS 硬件、受控 outlet group、P340 电源输入和证据时间。USB 模式网络为零且设备 ACL 只给 SYSTEM 与控制服务 SID；网络模式只能访问 manifest 的 literal IP/port/protocol/pinned peer，DNS、proxy、redirect 和其他 socket 均为零。凭据只允许 nonexportable CNG key 或 DATA_HDD 上 DPAPI-NG service-SID sealed secret，argv、环境变量、日志、证据和普通配置中的 secret 计数必须为零。carrier 失联、字段未知、签名/版本/配置/服务身份/固件不合格或无法驱动下列固定顺序时为 `CAPABILITY_INSUFFICIENT`，阻止上线。普通 IaaS 不得借宿主“有冗余电源”的描述继承本档案的 UPS 证书；没有站点可见且已批准的电源/安全关机证据时，IaaS 仍不满足此门。

必须验证：

1. 市电断开检测；
2. 达到电池阈值后禁止新长任务；
3. PostgreSQL、worker 和文件写入安全收敛；
4. Windows 自动关机；
5. 电力恢复后的受控启动和自检；
6. 多次断电不会绕过 BitLocker 或留下半个配置代。

outlet 命令是严格 `UpsOutletCycleCommandV1`，不携带 endpoint、credential、path、argv 或厂商 raw payload。同一 `(ups_adapter_identity,command_id)` 且同一 command digest 只能 query/adopt 并返回字节完全相同的 `UpsOutletCycleCommandAckV1`；同 ID 不同 digest 冲突，设备状态无法查询即 `COMMAND_STATE_UNKNOWN` 且禁止重发。boot 已变化但 preboot composite ACK 未落盘时仍失败关闭，不能用事后可见事件重建 PASS。

UPS 失联、失效、电池过期或 carrier 证据过期必须形成不可抑制告警；不能因仍有市电而标绿。更换 UPS、通信方式、adapter、固件、电池或阈值都会使原断电证据失效，并触发重新演练。

## 12. 备份和勒索恢复

最高安全档同时需要三层：

1. **连续层**：服务器外 HDD 目标，自动增量，writer 只创建/校验，不能覆盖、删除、改 ACL 或改保留；
2. **离线层**：定期完全离线、加密、轮换并存放在不同物理地点的 HDD；
3. **恢复材料层**：独立身份保管的密钥、部署清单、安装包、签名 checkpoint 和操作手册。

备份 manifest 必须由 backup writer 无法取得的签名能力确认，避免被攻陷 writer 伪造“最新健康点”。新备份即使签名通过也要做恶意/逻辑污染检测，不能总是恢复最新时间点。每个 backup set 的独立 backup DEK、recovery-only 信封、三份加密 share、2-of-3 轮换与跨洁净主机互操作以 [ADR-0024](../../adr/ADR-0024-f57-backup-key-envelope.md) 为准；不得复用日常 operational recipient、writer 身份或应用 vault 保管人。

active-config 必须 exact 指向当前签名 `BackupTopologyV1` head；revision 从 1/null predecessor 开始，后续只允许 prior+1/exact predecessor，旧签名、fork 或目录“最新”不能回滚当前拓扑。拓扑按 enum 固定六角色、服务器外连续目标和按序 A/B 两块离线介质；writer/target-agent credential 分别 exact-join mTLS client/server SPKI，live evidence 必须逐字读回各自 host/media、failure、administration、credential、custody/location domain。每块介质恰有两个按 principal 排序、互异且与六角色分域的人类 custody binding。三块盘均为 HDD/GPT/NTFS/BitLocker software XTS-AES-256/100%；连续目标 protector exact-set 为 `{PUBLIC_KEY,RECOVERY_PASSWORD}`，离线盘为 `{RECOVERY_PASSWORD}`。有效最短保留期取站点法律期限、90 天、`2×检测延迟P99+洁净恢复验证窗口`、`2×轮换间隔` 四者最大值；离线代年龄不得超过 7 天，至少保留两代。

每次安装、PITR、生产启用及 retry 都要用新 challenge/session/object 采集 `StorageSafeguardReadbackV1`；其 expiry 精确为 `observed + topology.max_age` 且 max age 不超过 300 秒，消费时 current head 和 trusted now 均有效。所有 subordinate refs typed-load同一 support-evidence root：target probe/receipt 由 topology-pinned target-agent 单签，A/B transition/safe-eject/disconnection/custody/health 由两个当前 human custodians 双签，并 exact-hash包含字段。目标必须同时满足容量公式和真实 `total/free/quota/reserve` 不等式，partial count/bytes/oldest optionality一致、过期为零、耗尽暂停批量任务而不改历史；writer/target-agent/maintenance/retention/signer/recovery 六类完整最小权限负探针逐项拒绝。writer 只有一次性“刚写对象”精确读回 capability；target-agent 的直接存储操作全部拒绝。上线时 A/B 所有 domain/location 必须逐字匹配 topology，只能为 `VERIFIED_DISCONNECTED|SEALED_VERIFIED`，零 attachment、授权撤销、安全弹出、物理断开、健康、双人 custody，且 `bundle_contains_recovery_material=false`；任一 UNKNOWN/过期/共享域/链/签名/容量/探针失败都是不可抑制风险。

服务器外连续目标必须按 deployment/writer 设置对象数、字节数、写入速率和并发 quota，并保留普通 writer 永远不可占用的 emergency reserve。quota 的数值由目标容量、实际恢复集、保留代际和增长证据签发，不存在适用于所有客户的固定默认值。target 满、quota 异常突增、partial upload 超时或 generation 爆发时必须立即：

1. 把备份保护状态置为不可抑制风险，不得继续显示绿色或把本机 staging 当成功副本；
2. 保留已完成、已签名和已 pin 的对象，普通 writer 不得删除历史、覆盖 generation、改 ACL、缩短 retention 或自行扩大 quota；
3. 先暂停批量导入、导出、重报表和非必要 provider，再按保护窗决定受控停机；不得让备份流量无限抢占 PostgreSQL commit、WAL 和审计；
4. 只允许独立低权 maintenance identity 按签名清单回收已超时且从未完成的 partial object；不得触碰完成对象或已 pin generation；
5. 扩容/换目标后必须 readback quota、reserve、身份和历史代际，再重新形成签名 checkpoint。

离线 HDD 的唯一精确状态图以 [客户端、生命周期与安全运营执行契约 §10.1](2026-08-23-f57-client-lifecycle-security-contract.md#101-备份抗耗尽) 为准：首次轮换为 `BLANK → ENROLLED → ACTIVE_APPEND → VERIFIED_DISCONNECTED`；下一轮仅在容量、健康和保留策略仍通过时允许 `VERIFIED_DISCONNECTED → ROTATION_DUE → ACTIVE_APPEND`；停止追加时为 `ACTIVE_APPEND → SEALED_VERIFIED → RETIRED_PENDING_DISPOSAL → DESTROYED`。`ENROLLED` 要求介质身份、BitLocker/文件系统、保管人和容量证据登记完成；只有 `ACTIVE_APPEND` 可在获批窗口追加；`VERIFIED_DISCONNECTED` 要求 ciphertext/readback、manifest、ADR-0024 envelope、签名 checkpoint 和最小恢复抽检均通过后物理断开。`SEALED_VERIFIED` 之后不得回到可写状态。旧 `media_id` 在 `DESTROYED` 后终态不复用；物理介质经双人批准、可验证 crypto-erase/销毁与重新验收后如需再用，必须以新 `media_id` 从 `BLANK` 开始。任何时候都必须保留跨越检测窗口的多个已验证 generation，不得用最新一次成功覆盖全部 known-clean 历史。

有效勒索保留期不得短于 `max(site_legal_retention, 90 days, 2 × measured_detection_lag_p99 + clean_restore_validation_window, 2 × offline_rotation_interval)`；已验证离线集的年龄不得超过 7 天。站点策略只能缩短离线集年龄或延长保留期，不能反向放宽。检测滞后无实测、保留身份可单方缩短、连续目标装不下签名策略要求的全部代际，或任一离线介质/洁净恢复主机装不下“实际可恢复集 + 加密/校验/恢复工作空间 + 30 日 P95 增长余量”时，生产认证失败。

恢复演练从洁净 Windows Server 开始，恢复：

- PostgreSQL 和 WAL；
- 附件与版本；
- 配置代和客户能力包；
- 审计链和服务器外 checkpoint；
- secret vault 与可用 wrapping key；
- 许可、身份和设备状态；
- 业务事实、当前投影和勾稽结果。

系统不存在跨客户、跨数据量或跨硬件通用的 RPO/RTO。候选 RPO/RTO 只能由每次演练的实际数据量、介质、软件/配置 generation、增长压力和洁净恢复拓扑签发；未演练、证书过期或任一绑定条件变化后的数字不得在 UI、导出、合同或销售材料中展示为承诺。

## 13. RAID1 升级

P340 需要先验证最终安装两块匹配企业 HDD 所需的托架、可选 flex-bay/升级件、供电、SATA、控制器和 Windows Server 驱动。现有 1TB HDD 只有在型号、CMR、固件、健康、工作负载等级和配对兼容性全部认证后才可复用，此时只增加一块匹配盘；否则先移除现盘，再安装两块匹配盘。升级过程：

1. 完整服务器外备份并验证；
2. 关机、断电、安装磁盘；
3. 建立并验证 RAID1/镜像；
4. 恢复或迁移全部权威数据；
5. 校验数据库、附件、审计和密钥；
6. 运行 20 人容量与断电测试；
7. 未被认证复用的原 1TB HDD 清除权威角色，只作非权威暂存；已被认证复用的盘继续作为镜像成员，不执行本步。

重建期间性能、断电和磁盘替换必须演练。RAID1 不增加备份层，也不是硬件热插拔承诺。

## 14. 暖备预留

第一阶段不启用自动故障转移，但必须冻结以下接口：

- `deployment_id` 与单调 `authority_epoch`；
- 唯一写权威 lease/证据；
- 客户端 authority discovery 和 epoch 校验；
- PostgreSQL、附件、配置代、审计 checkpoint 和 secret vault 的一致恢复点；
- replication lag、密钥可用性、备份健康和 promotion readiness；
- 双人 promotion、旧主 fencing 证据和审计；
- failover、failback、rejoin、损坏复制和脑裂测试。

无法证明旧主已被隔离时不得提升。暖备会复制逻辑损坏或勒索效果，因此永远不计作不可变/离线备份。

## 15. IaaS carrier

相同 Windows Server 安装包可以运行在客户 IaaS，但 `HDD_STRICT` 合规不能从“云盘”名称推断。只有取得以下证据才可认证：

- 权威数据卷的底层介质为 HDD；
- 缓存、快照、临时盘和 provider 运维副本的介质与权限符合策略；
- 客户控制 OS、密钥、备份、网络和管理员；
- 生产、备份和恢复材料处于不同凭据/故障域。

普通 IaaS 无法证明时，状态固定 `STORAGE_MEDIA_UNVERIFIED`，可以做开发/测试或采用另行批准的存储政策，但不得承载本项目字面“全部数据物理 HDD”的正式生产。

### 15.1 中国大陆驻留

首版真实客户数据及一切可关联客户的衍生数据只允许在中国大陆境内处理和持久化。该边界同时覆盖物理 P340/IaaS 权威节点、数据库/WAL、附件、索引、日志、审计、导出、临时文件、连续目标、离线轮换、恢复材料元数据、监控明细、支持诊断包、provider 输入/输出和客户可关联遥测；不能只证明主数据库在境内。

deployment、provider、backup、support 和 diagnostics manifest 必须记录 jurisdiction、region/物理保管地、处理方、endpoint/redirect、数据类别、证据 digest、验证时间和有效期。物理 P340、服务器外连续目标、两块离线介质和洁净恢复主机都必须位于中国大陆境内；IaaS 必须使用客户控制的中国大陆区域，并证明快照、缓存、日志、运维副本和灾备不会跨境。地点为 `UNKNOWN`、证据过期、DNS/重定向/支持通道越境或 provider 无法承诺境内处理时，对应 capability 失败关闭，禁止录入或发送真实客户数据（`NFR-017`）。

## 16. 正式上线硬门

### 硬件与系统

- [ ] BIOS、TPM、Secure Boot、BitLocker、网卡和存储驱动通过；
- [ ] OS、数据与离线介质均为 GPT + NTFS、BitLocker software XTS-AES-256 且 100% 加密；ReFS/FAT/exFAT、硬件自加密替代、算法/状态不明或载入数据后再加密均失败；
- [ ] UEFI 管理密码由双人分域保管；禁用未批准的 USB/外部介质启动和 PXE；每次认证读回 Secure Boot、TPM clear 状态、boot order 与启动项 exact-set；
- [ ] Intel AMT 未使用时保持未配置/禁用；若现场启用，必须另有管理网、独立凭据、TLS 信任、固件证据和单独认证，且仍不得计作服务器级 BMC；
- [ ] 机器位于受控机房，启用可用的机箱锁/防拆开关或封签并记录检查；未经批准的拆机使生产状态失效；
- [ ] USB removable device 默认拒绝，AutoRun 禁用，复合 HID/NIC/boot-class 与 BadUSB 负例通过；只在离线轮换窗口按已登记 `media_id`、硬件序列号、volume GUID 与 BitLocker protector 临时放行对应备份盘，完成写入/验证后安全弹出、撤销设备授权并证明介质物理断开；
- [ ] 记录 OS SSD 的型号、序列号、固件、SMART、剩余寿命、温度、掉盘行为和空闲空间；冻结 Windows 更新/回滚空间预算，并把 SSD 故障后的洁净重装与权威数据恢复耗时纳入证书；
- [ ] 记录每块生产 HDD 的实际型号、序列号、固件、容量、CMR/SMR 记录方式、厂商工作负载等级和保修状态；任何一项未知，或盘为 SMR，均不得仅凭空载/短时稳定测试放行；
- [ ] HDD SMART、温度、flush、坏扇区和断电行为通过；
- [ ] 内存测试和至少 72 小时混合稳定负载通过；
- [ ] `SIGNED_VENDOR_ADAPTER` manifest/config/profile/service/transport/credential 实机读回通过；canonical device/network endpoint、status→signed runtime binding、24 小时内 provider-authenticated self-test PASS、5 秒轮询/15 秒状态有效期、30 秒 monotonic provider ACK 与独立 600 秒 POWER 对账窗通过；same-ID/same-digest response-loss query/adopt、changed-digest conflict、UNKNOWN 不重发，以及 UPS 自动安全关机和恢复启动通过。

### 数据路由

- [ ] deployment manifest 精确绑定 software/data volume；
- [ ] 完整混合负载写跟踪证明 SSD 无客户或衍生字节；
- [ ] pagefile、dump、TEMP、quarantine、cache 和 secret vault 符合本档案；
- [ ] 权威、provider、备份、离线介质、洁净恢复、支持和诊断的中国大陆驻留 manifest/readback 全部有效，未知或跨境路径为 0；
- [ ] 任一未知写路径为 0。

### 安全和网络

- [ ] 防火墙默认拒绝、数据库不暴露；
- [ ] TLS、证书轮换、管理入口、门户和 RDP 边界通过；
- [ ] Defender/WDAC 或等价允许列表启用；
- [ ] 附件扫描 provider 必须可用，`NONE` 为 0；verdict 绑定实际字节 digest、engine 与签名 definition，按批准 W32Time 计算的 definition age 必须 `<=72h`，签名离线定义更新及 stale/revoked/wrong-engine/伪造时间负例全部通过；
- [ ] W32Time source、offset、last-successful-sync、回拨/快进证据和 monotonic duration 通过持续取证；P340 默认最大 offset 为 1,000ms、成功样本最大年龄为 900s，现场只能收紧；时间不可信时高风险写入、附件发布、签名、授权和配置代发布失败关闭；
- [ ] 插件、MCP 和连接器默认零权限，逐项批准。

### 持久性与恢复

- [ ] PostgreSQL checksums、fsync、强杀和断电测试通过；
- [ ] 固定 PostgreSQL 16 安装包、签名、SBOM、服务身份、PGDATA/WAL/temp/log/archive 路径和 `postgresql.conf`/`pg_hba.conf` 已取证；base backup、连续 WAL、`pg_verifybackup`、PITR 与附件一致 cut 在洁净 Windows 主机通过；
- [ ] 当前状态、事实、审计和 Outbox 原子性通过；
- [ ] 服务器外 writer 删除/覆盖/改 ACL/保留的负向探针全部拒绝；
- [ ] 新鲜 `StorageSafeguardReadbackV1` exact-match active-config current-head `BackupTopologyV1`；revision/predecessor、typed support evidence 单签/双签、六角色负探针、physical total/free/quota/reserve、partial optionality/保留公式、A/B 状态链/无恢复材料/物理断开/健康/exact 两人 custody 全部通过；
- [ ] 连续目标使用独立 mTLS writer 身份和应用层加密；独立 checkpoint signer 在验证目标回执、对象 digest、PG backup manifest/WAL span 与附件集后才签名，writer 不可取得 signer/recovery key；
- [ ] retention 保管身份只能按策略延长，不能单方缩短；最新备份被投毒时可从多个更早、已签名且仍在保留期的 generation 完整恢复；
- [ ] 恢复拓扑 exact-set 同时含 `CONTINUOUS_APPEND_ONLY`、`OFFLINE_ROTATION`、`RECOVERY_MATERIAL`，三者 stable target/media ID、凭据、故障域与保管身份均已取证；本机 `LOCAL_DIR` 只可作 staging；
- [ ] 至少两块不同 `media_id` 的完全离线加密 HDD 已交替形成，除受控轮换/演练窗口外全部物理断开；
- [ ] 连续目标 quota/reserve、耗尽/partial-object 处置和离线介质 exact 图均通过正反向测试：`BLANK → ENROLLED → ACTIVE_APPEND → VERIFIED_DISCONNECTED`，`VERIFIED_DISCONNECTED → ROTATION_DUE → ACTIVE_APPEND`（容量/健康/保留仍通过），以及 `ACTIVE_APPEND → SEALED_VERIFIED → RETIRED_PENDING_DISPOSAL → DESTROYED`（`SEALED_VERIFIED` 后不可回写，销毁后复用必须新建 `media_id`）；
- [ ] 每个连续目标、每块离线介质和洁净恢复主机的可用容量都不小于实际可恢复集、加密/校验/恢复工作空间与 30 日 P95 增长余量之和；连续目标还须满足签名保留策略所需全部代际，容量不足即失败；
- [ ] 从洁净 Windows Server 完整恢复通过。

### 容量和诚实状态

- [ ] 15 Workbench + 3 客户门户 + 2 供应商门户的 20 人聚合负载，与 1 个独立保留资源的 Control Center 会话、增量备份、自动化、附件和单报表同时通过；
- [ ] 低优先级任务可节流、持久等待并恢复；
- [ ] 绿色/黄色/红色阈值与用户提示通过；
- [ ] 永久展示“单磁盘降级生产”；
- [ ] 不宣称 HA、热插拔、统一四小时恢复或未经验证的云 HDD。

第一轮完整恢复演练未通过前，禁止录入真实生产数据。
