# 企业私有化运营平台仓库级威胁模型

> **F-57 现行适用性（2026-08-23）。** 本文件只有 `Overview` 的 F-57 产品边界和 `F-57 增补威胁与强制控制` 是现行规范；两个明确标注 `HISTORICAL_NON_NORMATIVE_APPENDIX` 的旧威胁库存只作追溯。本威胁模型以 [F-57 可治理自动化底座](superpowers/specs/2026-08-23-f57-governed-automation-fabric-design.md) 为最高产品与架构边界。固定九进程、首版本地模型、第五客户端、旧声明式包限制、旧 SSD/C 盘路径、旧 RTO 和旧能力闭集不得作为实现依据。现行文档范围见 [F-57 权威登记](superpowers/reviews/2026-08-23-f57-authority-supersession-register.md)。

## Overview

### 目的与状态

本文件是本仓库产品的仓库级安全威胁模型，覆盖 F-57、需求追踪、未冲突 PRD/F-50 细节、接口契约、数据字典、配置、事件、指标、迁移、客户端、部署、备份和发布链。它为后续实现、安全审查、测试和发布签字提供统一边界，不针对某一提交或某一个模块。

当前仓库已经存在基础设施、平台骨架与早期迁移，但本轮只冻结完整产品的设计与开发输入，没有启动新的业务功能实现。因此，本文提到的控制均是后续实现必须满足的安全不变量；既有骨架也不构成控制已经完整实现、测试已经执行或认证已经通过的证据。实现后的证据以测试、渗透、恢复演练和 F-57 实施计划最终发布门为准。

### 产品与运行形态

产品是一套面向企业私有部署的可治理自动化底座，覆盖客户、合同、销售订单、采购、库存、项目、经营财务、投诉与售后工单，并提供可定制数据库、动态权限、耐久闭环、签名能力包、Excel/文档处理、客户/供应商门户和受控 AI/MCP。核心后端采用 Rust 可信内核加模块化业务主体，事务数据库第一版只认证 PostgreSQL 16；Windows、macOS、iOS 和 Android Workbench 消费同一组强类型业务能力。

当前首版正式生产基线只是在客户自控 ThinkStation P340 物理机上的一台 Windows Server 2022。服务器控制中心、权威服务、PostgreSQL、文件存储、自动化和启用的隔离宿主均在该 Windows Server 上运行；进程数量不是产品契约。客户之间使用独立部署；同一客户的多个法人共享一个逻辑数据库，并以 RLS、动态属性权限和独立密钥域隔离。服务器控制中心属于权威节点，远程浏览器只是显示入口，不是第五个业务客户端。客户自控 IaaS 仅是未来独立 profile 扩展缝，当前必须失败关闭。

F-56 的内置 `MODULE_PACKAGE` 继续承担许可和内置模块生命周期；F-57 的 `CAPABILITY_PACKAGE` 是独立扩展载体，可以声明对象、流程、UI、报表、受控迁移、WASM、签名 Windows worker、连接器和在证据通过后的受控 Windows 容器。任何包仍不得任意注入 DLL、脚本或直接 SQL，也不得直接连接权威数据库。第一阶段冻结 AI provider、工具、权限和审计接口，但不交付本地模型；外部 AI 默认关闭。

当前物理生产硬件基线是 ThinkStation P340 Tower、i5-10500、32GB RAM、256GB SSD 和单块 1TB HDD。未来客户自控 IaaS 只能通过新的 graph/profile version 启用互斥的独立 `IAAS_WINDOWS_SERVER_HDD_STRICT` 认证载体，并且不继承 P340 CPU/主板/物理 UPS 证据；当前 selector 对它固定返回 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED`。所有持久客户数据和衍生数据必须写入加密 HDD；SSD 只允许 signed Set A 的系统、程序、静态依赖、可重新下载模型与可重新登记的非秘密 metadata，以及 exact 四类 mutable Set B：有界 POWER capsule、有界 package-recovery continuation capsule、recovery-domain-signed kernel pointer/journal head、可重建的 content-addressed signed native-code slot/cache。每类都有独立大小/保留/off-host mirror/终态删除/SSD-loss 重建契约，闭集外持久字节或客户/业务 authority 字节失败关闭。当前单 HDD 必须显示为“单磁盘降级生产”，服务器外备份、UPS 和完整恢复演练是上线前置条件。

### 关键资产

| 资产 | 影响 |
|---|---|
| 客户、联系人、合同、订单、采购、库存、项目、工单与附件 | 商业秘密、个人信息与业务连续性 |
| 应收、应付、预收、预付、发票、资金、凭证、期间与成本台账 | 资金损失、财务错报、税务与审计风险 |
| 法人隔离策略、权限快照、审批与重新认证证据 | 越权与职责分离失效 |
| 审计链、Outbox、幂等记录、对账结果与销毁证明 | 事实不可追溯、抵赖与隐蔽篡改 |
| 数据密钥、备份密钥、签名密钥、服务凭据与恢复材料 | 跨法人或整实例解密、制品伪造、恢复失败 |
| 数据库、附件、事务日志、服务器外只追加连续副本、至少两介质离线轮换层、恢复材料与签名检查点 | 勒索、误删、硬件损坏后的恢复能力 |
| 模块包、插件包、配置包、迁移、离线补丁与构建制品 | 供应链执行权限与全实例完整性 |
| 客户端本地缓存、草稿、令牌与设备插件 IPC | 终端丢失、恶意本机进程与会话劫持 |
| AI 模型包、提示模板、字段目录、签名 QueryPlan 与资源认证报告 | 供应链替换、越权计划、结果泄漏与资源耗尽 |
| MCP manifest、短期 grant、binding、secret-broker/vault credential reference 与外部响应 | confused deputy、SSRF、凭据泄漏、工具注入与命令重放 |
| 服务器控制中心制品、签名配置代、desired/observed 回执与部署证据 | 管理端被误当超级管理员、降版/分裂代、客户端判定绕过、云控制面残余风险被隐瞒 |

### 主要安全目标

1. 任何请求都不能绕过客户部署边界、法人边界、记录级权限、字段级权限或职责分离。
2. 财务与库存效果必须只由冻结的业务事件和计量规则产生，借贷、数量、金额、核销与历史切片始终可重放并对平。
3. 业务写入、审计、Outbox、幂等和必要台账在同一事务内原子提交；失败不得形成“业务成功但审计或事件缺失”。
4. 附件、导入文件、插件、配置、迁移和外部响应在进入可信域前均按不可信输入处理。
5. 日常运行身份被攻破时，攻击者不能同时取得生产数据、删除审计证据并删除或覆盖服务器外全部可恢复副本。
6. 签名、密钥、凭据和敏感明文不进入日志、错误响应、遥测、客户端持久化或非必要进程。
7. 所有失败路径均显式拒绝、可审计且不产生部分业务效果；不存在静默放行、空实现或默认成功。

## HISTORICAL_NON_NORMATIVE_APPENDIX A：F-55/F-56 信任边界库存

> 本附录保留旧架构的资产、信任边界和安全假设，便于确认威胁没有因架构升级而被遗忘。涉及固定角色、固定进程、模块包限制、本地 AI、MCP 闭集、ServerAdmin、WinCred、Linux 路径、备份或容量的旧实现语句全部非规范；F-57 当前实现只能引用后面的现行增补节。

### 参与者与控制权

| 参与者 | 可控制的输入或权限 | 不应天然拥有的权限 |
|---|---|---|
| 企业内部普通用户 | 四端请求、搜索条件、表单、Excel、附件 | 跨法人、越字段、审批、自提权、直接数据库访问 |
| 经理、财务、管理员、审计员 | 其角色允许的审批、重新认证、配置或查询 | 单人绕过职责分离、读取密钥、改写审计与历史凭证 |
| 供应商门户用户 | 公网登录、采购确认、送货与发票上传 | 内部 API、其他供应商、其他法人或原始内部字段 |
| 客户运维与数据库/存储管理员 | Windows、网络、数据库、备份落点和密钥设施 | 应用无法从技术上完全约束此类特权主体；其操作须由客户治理与独立审计约束 |
| 厂商开发与构建人员 | 源码、依赖、迁移、构建和签名流水线 | 直接连接客户生产、隐藏遥测、在线回传或未经批准的发布权限 |
| 模块/插件发布者 | 签名包、能力清单、版本与资源请求 | 默认网络、文件、密钥、数据库或任意业务对象权限 |
| 电子签章服务、移动推送服务、客户 ICAP | 协议响应、回调和服务可用性 | 核心数据库、未脱敏业务数据或任意出网能力 |
| MCP 客户端与获批远端/本地 MCP server | 协议帧、工具/资源名、schema 内值与服务可用性 | 自报身份/法人/权限、通用网络/文件/SQL/shell、高风险或审批动作 |
| 未来 IaaS provider 与客户 tenant root | 宿主、磁盘快照、虚拟网络、VM 电源与控制面 | 当前 profile selector 必须拒绝；未来新 graph/profile version 启用后，应用仍无法技术阻止其复制/回滚/停机，风险须由客户控制、vTPM 证据、职责分离与合同披露约束 |
| 外部攻击者与恶意本机进程 | 公网入口、凭据攻击、恶意文件、网络响应、本机 IPC | 任何受信身份、持久化执行、密钥与服务器外备份处置权 |

### 信任边界

| 边界 | 进入可信侧的数据 | 必须执行的控制 |
|---|---|---|
| 四端客户端 → 反向代理/core-server | 令牌、命令、查询、文件、幂等键 | TLS、认证、授权、对象归属重检、限流、输入约束、幂等、审计；客户端判定不作为授权依据 |
| 公网供应商门户 → portal-gateway → core-server | 外部账号、采购/发票/附件操作 | 独立站点、证书、进程、系统账号与能力 API；供应商、法人、号码和对象绑定；只返回脱敏投影 |
| core-server/job-worker → PostgreSQL | 安全上下文、SQL 与事务 | 参数化访问、四具名连接池、RLS ENABLE+FORCE、复合外键、同事务审计/Outbox；运行账号无 DDL；`integration-gateway` 不取得数据库、KMS、平台文件或 Outbox 能力 |
| ops-agent → PostgreSQL 运维视图 | 健康、积压与复制状态只读查询 | 独立 `Ops` 只读池、固定视图与语句超时；不得读取业务表、写入或执行 DDL |
| 法人 A → 共享数据库 → 法人 B | 行、字段、搜索、报表、缓存和导出 | `legal_entity_id` 全链路传播；RLS、ABAC、独立密钥域与负向测试同时成立 |
| 文件/Excel/历史数据源 → 导入与文件存储 | 任意名称、格式、公式、压缩内容、元数据与正文 | 格式/大小/展开量限制、公式注入防护、路径规范化、病毒扫描策略、内容寻址、批准模板、逐行错误与回滚 |
| 许可证/模块包/配置包 → 平台运行时 | 许可范围、模块动作、声明、配置和表达式 | 外层配置包与内层 CMS 分别验签、部署/版本/范围绑定、九套自动测试、双人审批；模块包只声明内置模块状态，禁止动态 code/SQL/script/file/url/hook；停用保留数据 |
| WASM/受控容器插件包 → plugin-host | 可执行扩展、能力与资源声明 | 签名与哈希、许可证、版本锁、最小能力、无默认权限、资源限额、禁止直连数据库与明文机密 |
| integration-gateway → 外部服务 | 签章、推送与 ICAP 请求/响应 | 封闭目的地、域名/IP/端口批准、超时、大小限制、重放防护、响应验证、最小脱敏载荷；ICAP 仅回环窄例外 |
| 生产服务器 → 服务器外备份落点 | WAL、全量备份、附件、审计证据、配置和密钥恢复材料 | 写出前加密、完整性校验、内容唯一键、只创建不覆盖、写入身份无删除权、恢复身份分离、失败门禁 |
| 开发/CI → 离线补丁与生产安装 | 源码、依赖、构建制品、迁移、签名 | 固定 CI 入口、依赖锁、SBOM、可复现构建、生产 Authenticode、离线验签、双人发布批准 |
| 客户端设备 → 本地缓存/原生设备插件 | 草稿、会话、脱敏数据、打印/USB Key 请求 | 设备安全存储、最少离线数据、会话失效、独立插件子进程、受限 IPC、能力清单与崩溃隔离 |
| core-server → `\\.\pipe\ep-ai` → ai-inferer | 裁剪字段目录、问题原文、固定模板/模型版本 | 对端 SID/DACL、五 operation 闭集、单数据集计划校验、结果字节永不进入 AI、零 DB/网络/文件写、Job Object 与 15+30 上限 |
| MCP 客户端 → `POST /mcp` → core-server | JSON-RPC 方法、短期 grant、工具/资源调用 | 2026-07-28 六方法闭集、grant 绑定 session/device/le/manifest、逐次权限重检、列表裁剪、大小/超时、审计与高风险绝对禁区 |
| core-server → integration-gateway/plugin-host → MCP server | 已批准 manifest、最小字段 payload、有界响应 | 远端固定 HTTPS origin+SPKI、SSRF/DNS rebinding/代理/重定向拒绝；本地签名包、禁 child/network/任意文件；响应 schema/字段/字节闭包 |
| ServerAdmin → core-server | 管理查询、签名包/manifest/配置操作 | 独立静态 SPA、现有会话/MFA/CSRF/CSP、18×5 矩阵、RLS/SoD/审批不变；无新角色、进程、端口或直接 DB/KMS/file/shell |
| 未来 IaaS 控制面 → 单机 VM | 宿主、虚拟盘、快照、网络、vTPM | 当前不存在可信入口且必须拒绝；未来新版本才可要求客户自控境内 tenant/region、vTPM attestation、无托管组件、故障域与凭据域分离、同机快照不得冒充离站备份、完整恢复演练 |

### 攻击者可控、运营者可控与开发者可控输入

- 攻击者可控：公网请求、口令/MFA 尝试、对象 ID、查询与排序参数、自然语言问题、MCP JSON-RPC/payload/名称、幂等键、文件名与正文、XLSX/CSV 单元格、压缩包、供应商发票号码、签章回调、外部 HTTP/ICAP/MCP 响应、插件输入、可被窃取的普通用户会话或短期 grant。
- 运营者可控：用户与角色、法人映射、网络入口、WAF、证书、KMS/HSM、客户 ICAP、备份落点、保留策略、恢复材料、配置包批准、插件/MCP manifest 能力批准、模型包安装、carrier/provider/region 证据和离线补丁安装。
- 开发者或发布者可控：Rust/客户端源码、SQL 迁移、事件与错误码、配置定义、依赖锁文件、构建脚本、模块/插件/MCP SDK、AI/MCP schema、签名制品和数据迁移模板。
- 任何来自以上三类来源的数据都不能仅因“来自内网”“由管理员提交”“带有效签名”而跳过业务不变量；签名证明来源和完整性，不证明内容安全或业务正确。

### 必须保持的安全不变量

1. `SecurityContext` 的法人、主体、角色、系统用途与重新认证事实由受控入口构造；业务请求不能自行提交或改写。
2. 所有法人业务表、外键、唯一键、查询、事件、缓存、文件元数据、报表和导出都保留法人维度；跨法人不以全局管理员或系统账号隐式放行。
3. 每个命令先做能力授权，再做对象/字段/状态检查；高风险动作还必须重新认证和满足审批、职责分离要求。
4. 运行期账号没有 DDL，插件和门户不直连事务库；归档与备份复制身份不具备业务表查询权，但其物理副本覆盖整簇的残余风险必须如实披露。
5. 金额、数量、发票、核销、退款、红冲、采购暂估与总账规则均以冻结的服务端规则和数据库约束为准；客户端或 Excel 的合计不是权威值。
6. 会计凭证、审计效果、冲销、核销释放与库存成本效果只追加，不原地改写历史事实；更正通过显式反向链表达。
7. 同一命令的业务状态、审计、Outbox、幂等结果和必要凭证/台账在一个事务中成功或全部失败。
8. 文件路径、对象键、归档名和备份名由服务端生成并规范化；用户输入不能形成路径穿越、覆盖现有对象或选择任意本地路径。
9. 所有密钥按用途和部署/法人域分离；明文密钥与秘密不出受控进程，不进入日志、错误、报表、导出或备份清单。
10. 生产补丁、模块、插件和配置只有在签名、版本、许可证、兼容性、自动测试与批准均成立时启用；停用保留历史数据与审计。
11. 服务器外备份写入身份只能条件创建新的唯一对象，并通过一次性 exact-object handle 读取刚写 ciphertext 所需的有限校验数据；它不能列举历史，也不具备删除、覆盖、改名、改保留策略、改 ACL 或清理版本的能力；恢复与处置使用分离身份。
12. 任何运行期或发布门所依赖的负向检查缺失、失败或无法判定时均失败关闭，不得把“不知道”折算为通过。
13. AI 模型只产生闭集 QueryPlan，结果、结果派生文本和业务值永不进入 ai-inferer、提示、模型缓存、日志、审计正文、dump 或幂等响应体；执行前重新授权。
14. MCP 的主体、法人、设备、权限、manifest 与次数由服务端 grant 事实构造并逐次重检；共享 `HighRiskOperation` 七值（六类业务高风险加 `DATA_MIGRATION`）、合同终止与审批动作在发布和运行两层均不可达。
15. ServerAdmin、Mcp 与其他 ClientKind 的来源不可由请求任意伪造；ServerAdmin 仍受 90 格、RLS、SoD 与审批，Mcp 不进入能力矩阵。
16. 当前 `IAAS_WINDOWS_SERVER_HDD_STRICT` 必须固定失败关闭，不能进入候选、发布或生产 terminal；未来新 graph/profile version 启用后，IaaS 才可只替换硬件承载体，且无 provider/region/驻留/vTPM/控制权/备份故障域证据或出现托管组件时仍须失败关闭。

### 关键假设与残余边界

- 客户负责服务器物理安全、Windows 基线、网络分区、WAF/防火墙、时间同步、证书与客户管理员治理；产品必须提供可验证的最小配置和失败门禁，但不能阻止拥有客户操作系统、数据库、KMS 与备份存储最高权限的主体联合滥用。
- 首版 P340 单机部署没有高可用；物理服务器失效会停机，控制目标是依靠服务器外且故障/凭据域分离的副本恢复，不是持续可用。未来 IaaS VM/宿主也具有同类且额外的 provider 控制面风险，但不属于当前正路径。
- 客户提供的服务器外存储必须支持冻结的不可覆盖写入和权限负向检查。不能证明写入身份无删除/覆盖权时，系统必须打开不可抑制的保护缺失窗口并阻止发布；这仍不等同于经认证 WORM。
- 客户存储管理员或同时控制生产服务器、备份落点、全部离线介质和恢复密钥的恶意主体可破坏所有副本，属于应用无法消除的残余风险。首版必须已有至少两块加密离线轮换介质，除受控轮换/演练窗口外物理断开，并采用不同管理域、独立凭据和分域恢复材料；首版未提供的是经认证 WORM，而不是离线轮换层。
- 电子签章、移动推送、客户 ICAP 与获批 MCP server 可能不可用或返回恶意数据；核心交易必须有明确失败/降级路径，不能因此扩大出网或降低 schema/领域验证。
- 客户端设备可能丢失、越狱/Root 或被本机恶意软件控制；服务端继续是授权和不变量权威，本地安全存储只能降低而不能消除终端风险。
- 未来 IaaS provider/tenant root 处于平台控制之外，可能复制磁盘/内存、回滚快照、改变网络或关机；vTPM 只增强证据，不能消除该残余风险。因此当前路径固定关闭，未来启用也必须继续披露这一残余风险。签名发行者或获批 MCP server 自身被攻破同样仍是供应链/外部信任风险。

## F-57 增补威胁与强制控制

本节是 F-57 新增攻击面与失败门的现行权威；后文旧实现细节只有在不冲突时才补充本节。

### 1. 权威节点、控制中心与网络入口

威胁：攻击者把远程管理页面当成独立权威、绕过服务器编译/签名；办公客户端尝试直接访问管理 API；数据库、控制面或内部插件端口被暴露到办公网/公网；明文或弱 TLS 被降级。

强制控制：

- 服务器控制中心的全部决定、编译、审批、签名和发布在 Windows Server 权威节点完成；浏览器只携带用户输入和裁剪展示。
- 管理面使用独立路由/站点、客户证书、MFA/重新认证、管理网络 allowlist 和 CSRF/CSP 等浏览器控制。
- 办公 API、门户 API、管理 API 和插件 IPC 分离；PostgreSQL 只接受权威服务身份，不对客户端网络监听。
- 客户端至权威节点、门户至 gateway 和允许的服务间远程通道都必须使用客户批准的 TLS 策略；生产不得回退到明文。
- 管理页面失效不停止已经持久化的业务自动化；权威 API 失效时客户端只能保留草稿。
- 本地命名管道同样是不可信边界：必须使用 first-instance 与 `PIPE_REJECT_REMOTE_CLIENTS`；server-instance 权只授予 `SYSTEM` 和 exact owning Windows service SID，客户端仅可获得 concrete data-right mask（当前为 `0x00120183`），授予对象只能是逐项 exact client service SID，或由签名配置冻结、安装与运行时读回成员 exact-set 的专用本地客户端组。客户端、该组、Authority facade 和普通管理员都不能创建 first/second/replacement instance；不得使用 `GENERIC_WRITE`、宽泛 Users/Administrators 组或环境继承 ACL（独立维护管道另走双人授权），双方还须校验进程身份、SID、签名和期望二进制 digest。
- IPC 帧必须有界并绑定 nonce、authority epoch、configuration generation、过期时间和重放记录；低权用户抢占管道名、客户端/服务端冒充、远程管道、超大帧、旧 generation handle 或 ACL/对端身份读回失败时，相关服务不得进入 ready。受控 worker 子进程还须受 Job Object、handle 继承和生命周期限制。
- 恶意或错误 NTP、W32Time 停止、时钟回拨/快进和 DST 不得延长授权、复活租约、绕过证书/CRL 或破坏审计顺序。权威节点持续记录批准时间源、offset、last-sync 与跳时证据；duration/lease 使用 monotonic clock，安全墙钟不可用、超限或回拨时，高风险命令、配置发布、签名验证和时间型授权失败关闭。

### 2. 配置代分裂、降级与回滚攻击

威胁：攻击者向部分客户端或 worker 下发旧权限/旧流程；同一事务跨两个配置代；回滚只回 UI 而不回权限或迁移；伪造 observed state 隐藏漂移。

强制控制：

- 一个签名配置代必须绑定 schema、权限、流程、UI、报表、包、连接器和契约 digest。
- 每个命令在事务开始时固定一代并记录到业务事实和审计；事务中不得重新解析到另一代。
- desired/observed 由服务器签名状态和客户端/worker 回执组成，客户端自报不能单独证明一致。
- 回滚必须有兼容计划；已经产生的业务事实不删除，必要时追加补偿。
- 旧客户端无法解释安全关键规则时只读或阻止相应能力，不允许“尽力执行”。

### 3. 动态授权、委托与任务解析

威胁：权限图复杂导致意外放大；管理员把自己没有的能力转授；临时授权到期后缓存仍可用；任务改派绕过法人/字段/职责分离；拒绝规则被宽泛允许覆盖；权限解释泄漏无权对象存在性。

强制控制：

- 裁决顺序固定为内核拒绝 → 法人/分类/SoD 拒绝 → scoped deny → conditional allow → default deny。
- grant 必须保存来源、理由、批准人、作用域、条件、有效期、delegation ceiling 和撤销事实。
- 每次命令和查询重新裁决；缓存必须绑定 policy generation、主体、设备、法人和到期时间。
- 改派只选择当前满足能力和作用域的主体，不自动增加 grant。
- 发布前模拟新增/失去访问、受影响记录样本和 SoD 冲突；高风险授权变更 maker 与 approver 分离。
- 权限解释只返回调用者有权知道的规则和泛化原因，不确认无权对象是否存在。

### 4. 数据模型编译与客户自定义

威胁：恶意或错误模型制造 SQL 注入、超长锁表迁移、全表扫描、索引爆炸、跨包改表、删除历史、绕过 RLS/加密/审计，或以 JSON/EAV 逃避约束。

强制控制：

- 客户提交声明式模型，不提交 SQL；标识符、类型、关系和索引由可信编译器生成。
- 每个对象有唯一 owning package；跨包只能使用契约，不允许直接写表。
- 核心表、审计、财务事实、身份和许可不开放任意扩展 DDL。
- 迁移先静态检查、影子演算、锁时间/空间预算、备份 checkpoint 和双人审批，再由专用迁移身份执行。
- 破坏性变更采用新增、回填、核对、切换和延迟处置；普通管理员不能直接 DROP。
- 所有新业务表必须自动获得法人维度、RLS、审计、版本、生命周期和备份登记，否则发布失败。

### 5. 能力包和扩展供应链

威胁：厂商或客户签名密钥被盗；包声明少量权限但运行时访问更多；依赖替换、版本降级、影子加载与正式加载不一致；包停用时任务丢失；容器/worker 逃逸；许可证包与能力包混淆。

强制控制：

- F-56 `MODULE_PACKAGE` 与 F-57 `CAPABILITY_PACKAGE` 使用不同 purpose、schema 和 applier，不得类型混淆。
- 内外签名、信任根、CRL、SBOM、依赖 digest、权限、网络、文件、密钥、资源、迁移、测试和 rollback 全部绑定 exact manifest。
- 厂商根、客户根和第三方客户批准根分离；撤销一个发布者不能隐式授权另一个。
- WASM、Windows worker 和容器都没有数据库凭据；只能持短期 scoped capability token。
- 安装先验证/自测/影子加载；替换先停收新任务、排空或持久转移，再原子切换配置代。
- 停用保留数据、事实、审计和导出；在途任务必须进入完成、补偿或显式等待，不能丢弃。
- 任意 DLL 注入、任意脚本、任意 SQL、安装 hook、宿主路径和未声明网络均永久拒绝。

### 6. 耐久自动化、重复效果和未知结果

威胁：断电后重复付款/采购；租约过期后两个 worker 同时执行；外部系统已成功但响应丢失；补偿与原效果交错；流程升级改变运行中语义；无限循环耗尽 HDD；AI 结果成为确定性主链前置。

强制控制：

- 每项 effect 有稳定幂等键、attempt、lease、状态、请求/响应 digest、外部关联号和 reconciliation policy。
- 内部状态、事实、审计和 Outbox 同事务；外部效果在提交后执行。
- `UNKNOWN` 结果先查询/对账，不能直接重试；付款、退款、下单等必须有业务级去重或人工处置。
- 运行实例固定 automation version；新版本不静默接管旧 run。
- 补偿是新事实并引用原效果，不删除或改写原记录。
- 循环必须有检查周期、退出条件、重试上限、资源预算和人工升级；异常增长触发 admission control。
- AI、MCP 或插件不可用时，确定性业务主链继续、等待或进入人工任务。

### 7. 离线客户端和动态 UI schema

威胁：旧客户端缓存旧权限；本地草稿包含被撤销字段；两个设备离线修改同一金额；恶意 UI schema 诱导用户批准不同内容；设备撤销后继续提交；Root/Jailbreak 读取缓存。

强制控制：

- 缓存使用设备密钥并绑定主体、法人、policy/config generation、字段投影和到期时间。
- 离线只产生草稿和 intent；服务器重新验证权限、记录版本、规则和幂等键。
- 非敏感且能证明不冲突的字段才可自动合并；金额、状态、权限、合同、库存和审批冲突必须人工决定。
- 审批显示内容和待签摘要由服务器产生并绑定命令，客户端文案不能改变签名语义。
- 撤销、越狱/Root、超期或客户端能力不足时拒绝高风险命令并清除/过期缓存。

### 8. SSD/HDD 路径逃逸和数据残留

威胁：组件使用 `%TEMP%`、`C:\ProgramData`、系统 pagefile、WER dump、日志库默认目录或容器层，把客户数据写入 256GB SSD；卸载后残留；IaaS 云盘名称被错误当作 HDD 证据。

强制控制：

- `HDD_STRICT` 只约束 Windows Server 权威节点上内容承载或可关联客户的持久数据；Workbench 终端仅可保存离线协议明确允许的最小、加密、可撤销、非权威缓存，不能据此放宽服务器边界。
- 部署 manifest 绑定 OS/software volume 与 data volume 的稳定设备标识，不能只信盘符或路径字符串。
- PostgreSQL data/WAL、索引、附件、审计、应用日志、导出、temp、spool、插件/容器 workdir、含数据的 pagefile/dump 全部路由到加密 HDD。
- PostgreSQL 16 的 Windows 安装不能依赖安装员临场选择：唯一 schema/owner 持有 19-field `Postgres16WindowsPackageLockV1`、13-field `Postgres16WindowsInstallContractV1`、4-field `Postgres16WindowsEventLogFixtureSetV1`、19-field `Postgres16WindowsEventLogScanCoverageV1` 与 17-field `Postgres16WindowsInstallReadbackV1` 五个 strict root；它们必须分别经已签 `WindowsAuthorityArtifactSetV1`、contract 内 scan contract 和 `ReleaseWindowsServiceInstallEvidenceV1` 传递认证。package lock 的 `installed_files` 是 engine root 下全部普通文件与 SBOM 的双射闭集，逐文件固定规范相对路径、重开后的长度与 SHA-256，并以完整向量摘要封口；`.control` 文件、bundled/available/installed/enabled extension 集合也须 exact-join。V1 仅允许 clean install 或完全相同 lock 的幂等接管；发现任何不同的已有版本——无论更旧还是更新——都在修改服务或数据前返回 `MAINTENANCE_UPGRADE_REQUIRED`，不得靠版本比较自造升级或降级路径。
- `ep-postgres16` 固定为按需启动的虚拟服务账户、无自动恢复重启；DATA_HDD 解锁、存储清单、vault、配置与 TLS 全部验证前，PostgreSQL 进程数必须为零。九个路径角色的 unresolved `canonical_sddl_template` 只含指定 service/account 占位符；服务与账户创建后才从同一安装证据解析 numeric SID、应用 ACL，并把 live `canonical_dacl_sddl`/摘要逐项读回。投影与读回按 enum 顺序双射，禁止继承 ACE、额外管理员 ACE、掩码放宽、owner/group 漂移或路径别名：Engine 只在 SSD Set A，PGDATA（含 live `pg_wal` 与数据库临时关系）、归档 staging、独立 process/restore scratch、日志、TLS 与有效配置全部在 DATA_HDD。
- PostgreSQL 的关键 GUC/HBA/ident effective canonical vectors 必须逐字节等于投影，监听 exact-set 只有 loopback，GUC 精确为 `max_connections=64|reserved_connections=4|superuser_reserved_connections=3` 且保留两槽不可分配安全余量。每个消费者带 `NORMAL|RESERVED|SUPERUSER` 权限类并分别校验 `N+2<=57`、`R<=4`、`N+R+2<=61`、`S<=3`、`N+R+S+2<=64`；应用不能拥有 reserved/superuser 权限，migration 只能使用 `pg_use_reserved_connections`，recovery 才能使用 superuser，防止正常连接耗尽保留位。HBA 只证明 `hostssl`+`scram-sha-256`；libpq `channel_binding=require` 与实际协商必须由逐 consumer authenticated session probe 另行证明。禁止 `initdb --waldir`、用户 tablespace、reparse 后代、外部 CIDR、`trust`、ambient include 或 `postgresql.auto.conf` 偷换有效值。`cluster_system_identifier`、从重开 control bytes 得到的 `pg_control_system_identifier`、认证 SQL probe 得到的 `sql_system_identifier` 与外层 `postgres_system_identifier` 必须是同一非空十进制值；候选的 `DATABASE_CUT`、`WAL_CUT` 和三次 `RESTORED_DATABASE` 还必须经 typed install-evidence 路径逐项等于该外层值，不接受“同版本”“最新安装”或调用者提供的 cluster。运行证据只接受 typed `service_state=RUNNING`，同时 exact-join 已安装 image、service SID、loopback listener 数且两个非 loopback 计数为零，不能用任意 SCM 字符串或 pending 状态冒充 ready。
- Windows 持久性和日志路径同样闭合：固定 `fsync/full_page_writes/synchronous_commit=on`；`wal_sync_method=fsync_writethrough` 只是 PostgreSQL 16/Windows 兼容性 pin，不单独构成耐久性证明。生产启用前用 package/SBOM 已验证的 `pg_test_fsync.exe` 对同一 DATA_HDD 测试文件分别采集 `fsync` 与 `fsync_writethrough` 的支持、正吞吐和零 I/O 错误，并把卷、driver stack、write-cache policy 与工具字节绑定；Task 15 必须再与同一候选的真实 P340 UPS/write-cache 及受控 HDD flush/power-cut 证据 exact-join，任一绑定变化即失效。固定 collector→stderr→DATA_HDD、24 小时/100MB 轮换且不 truncate，server eventlog destination 关闭。Windows Event Log 零客户数据不是一个计数器断言：严格 coverage root 必须覆盖 `Application` channel 两个固定 provider 的注册读回、同 boot 起止 bookmark/record ID/time、完整记录区间、零 clear/drop/unexplained gap、fixture set ref/digest 与全部 fixture 执行，并要求 `coverage_complete=true` 和客户 token 命中数零；缺对象、截断或错配一律阻止安装就绪。
- PostgreSQL 历史日志删除是独立高风险边界：已签 exact policy 固定普通 `max_age=30 days`、`max_total=20 GiB`、当前打开日志不删、最近至少 7 日必须保留，站点法定/合同保留和对象 legal hold 只能延长。只有现有 `EPAuthorityControl` 内签名、双人批准、带 exact 预览清单/前后 digest/审计回执的 typed `POSTGRES_LOG_RETENTION_CLEANUP` 可删除既定 closed log；PostgreSQL service identity、backup writer 与普通 Authority 的删除/改 ACL/解除 hold/改政策负探针必须全部被拒绝。legal hold 导致 30 日/20 GiB 无法同时满足时保留 protected bytes 并 fail-close，不得删 current/7-day/held 集。DATA_HDD free 低于 `max(existing yellow_free,50 GiB)` 暂停批量，低于 `max(existing red_free,40 GiB)` 立即 deployment-wide hold；现有 P340 公式和 100 GiB file floor 产生更高值时必须取更高值。
- 安装、启动、升级和 F-57 最终发布门都运行写入探针与路径枚举；发现客户字节落 SSD 即阻止生产就绪。
- Windows Error Reporting、服务 dump、pagefile、Defender quarantine 和临时目录必须明确配置或证明不含客户载荷。
- SSD 上的 Windows Event Log 只能记录固定事件码与不可关联客户/对象的随机 incident ID；客户值、对象 ID、客户正文、客户正文哈希和可反查 digest 一律禁止。
- 当前 profile selector 不评估 IaaS 存储合规并固定拒绝：返回 `PROFILE_NOT_IMPLEMENTED`，同时投影 `STORAGE_MEDIA_UNVERIFIED`。只有未来新 graph/profile version 明确落地后，IaaS 存储才可能凭独立介质/服务合同和部署证据声明满足 HDD；普通“云盘”标签永不构成证据。
- OS SSD 的 BitLocker 在同一最高安全档内只允许互斥的 `TPM_ONLY_UNATTENDED` 与 `TPM_PIN_ATTENDED`：当前 P340 基线固定前者并实测 UPS 后无人值守重启；启用 PIN 时必须取消该声明，改为有人值守启动、告警和单独 RTO。两者都要求 Secure Boot/PCR/OS trusted boot。DATA_HDD protector exact-set 固定为 `{PUBLIC_KEY,RECOVERY_PASSWORD}`，Windows fixed-data auto-unlock 必须为 false；trusted boot 后只能由无出站网络的独立 restricted-LocalSystem `EPF57DataVolumeUnlockBroker` 验证九个 pre-HDD locator、证书策略/链、bootstrap authority、TPM NV 与目标卷，再用现有 TPM-backed/nonexportable 证书私钥、固定 thumbprint 和本机 WMI `UnlockWithCertificateThumbprint` 解锁。SSD 上仅允许受界限约束、可重新登记的 TPM-bound machine-key/certificate-store binding 与非秘密 locator/trust metadata；应用主密钥、客户秘密和可导出 wrapping key 永久禁止。clean-SSD/TPM-loss 只能在 admission closed 下经服务器外 48 位 recovery password 双人仪式重建新 key/certificate/PUBLIC_KEY protector、提升 epoch/NV 并普通重启验收，不能从公开材料重建旧私钥。
- 第一阶段业务相关卷只认证 GPT + NTFS 与 Windows software BitLocker XTS-AES-256，禁止以不透明硬件自加密替代；OS、数据和离线介质在接收真实数据前必须 100% 加密。ReFS/FAT/exFAT、算法/状态不明、错误 protector exact-set 或未证明 durable flush/power-loss 语义均不 ready。
- OS 卷与数据卷 recovery password 独立、服务器外、双人保管，并与应用 vault、备份和客户数据密钥恢复材料分域；TPM/主板或 OS 盘损坏、recovery-password theft 及 DATA_HDD clean-SSD 重新登记必须演练，无人值守 UPS 重启只在 `TPM_ONLY_UNATTENDED` 模式验收，PIN 模式验收有人值守恢复。
- pre-DB deployment manifest 本体只能位于 HDD `packages`，不得放入 SSD software root。信任锚来自签名二进制/WDAC 与 SSD 上允许的非秘密客户公钥；trusted boot/BitLocker 解锁后，从非 OS 卷固定 locator 读取 detached-signed manifest，再用 final volume identity 与 TPM NV/sealed 单调 revision+digest 验证。信任根和 manifest 禁止同批替换，恢复材料保存 exact manifest/root/revocation/checkpoint。任何 SSD manifest、root+manifest 同换、降版或 data-root 指向 SSD 的尝试在首次数据库连接前失败。

#### 8.1 Secret vault 与 broker

威胁：TPM 机器绑定使主板或 OS 盘损坏后无法恢复；唯一 recovery recipient 被盗或丢失；跨法人/用途解封；调用者诱导 broker 代解密其他 secret；过期/撤销 handle 重放；vault metadata 回滚/替换；明文残留在内存、dump、argv、环境变量或日志。

强制控制：依 [ADR-0020](adr/ADR-0020-dual-recipient-data-key-recovery.md)，每个 DEK 分别 wrap 给日常 TPM/HSM operational recipient 与独立、离线的 recovery recipient；任一正确用途路径可解开同一 DEK，不要求两域同时在线。日常域仅能在受信启动、正确 service SID、用途和配置代下调用 operational unwrap，不能调用 recovery 接口；recovery 材料固定使用现有 `PIV_SHAMIR_2_OF_3_V1`/双人保管并能在洁净主机脱离原机恢复，三种不同双 share 组合均可恢复，任何单一保管人不能恢复。两种 recipient 分别以认证上下文绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 和 envelope version，且 recipient 必须不同。broker 只签发与调用者 service SID、recipient、调用、generation、到期和次数绑定的短柄，不返回跨用途选择能力；撤销或过期柄不可重放。vault metadata、单调 revision 和 digest 与 pre-DB manifest/服务器外 checkpoint 核对；默认 `master.key`、WinCred 正文和环境变量/命令行 secret 永久禁止。敏感缓冲区尽量锁页、用后零化，生产 dump 排除 secret；洁净主机必须能用独立 recovery recipient 完成跨机恢复，并证明任一日常 recipient 泄漏不能解封其他法人/用途。

### 9. 单 HDD、勒索和备份破坏

威胁：单盘机械故障导致全停；生产管理员或勒索软件删除同机 WAL/备份；服务器把普通 SMB/NAS 写权限当不可变；备份加密密钥与生产密钥同时失窃；备份一直成功但从未恢复过。

强制控制：

- 当前 P340 永久显示“单磁盘降级生产”，不承诺磁盘故障连续运行。
- 生产数据上线前必须同时已有服务器外追加式自动增量层、完全离线加密轮换层、独立恢复身份/材料和完整恢复演练；异地位置不能替代离线层，离线层也不能替代自动增量层。
- 备份加密只使用 [ADR-0021](adr/ADR-0021-epb1-backup-envelope.md) 的独立 `EPB1` AES-256-GCM；`EPC1` 仍只覆盖 FIELD/ATTACHMENT/ARCHIVE。每个 backup set 使用只包裹给独立 backup recovery domain 的专用 DEK，AAD 绑定 deployment、backup set、immutable object、chunk、总明文长度、release/config generation 和 envelope version，并强制 nonce 唯一、chunk 顺序与长度。
- 备份 writer 只能创建和必要校验读取；不能覆盖、删除、改 ACL、改保留或清理版本。处置使用独立身份和双人审批。
- “必要校验读取”只允许 `AppendOnlySinkV1` 在成功追加后签发一次私有 affine `VerifiedJustAppendedObjectV1`，由第一次 exact-object readback 按值消费；响应丢失只能采用已持久化的 readback receipt，不能重新读取内容。接口不得提供历史列举、任意读取、覆盖、删除、改名、改 ACL、接管所有权或缩短保留的方法。
- `BackupTopologyV1` 由 deployment/epoch/generation/storage-manifest 和 active-config current-head 绑定；install/checkpoint/PITR/activation 的 enclosing binding 再把它与候选 exact-join。active config 必须逐字选择 supplied `BackupTopologySigningTrustCurrentPointerV1` 与 topology；pointer typed-load 唯一签名 `BackupTopologySigningTrustManifestV1`，两者按 generation/predecessor 单调推进。部署 bootstrap 固定独立 trust-manifest authority，manifest 再固定 topology signer `CN=EP F57 Backup Topology Authority,O=Enterprise Platform` 的 leaf SPKI、离线 chain、revocation snapshot 与 transparency checkpoint；只有由该 verified-current trust 值构造的私有 `BackupTopologyAuthorityV1` 可验证 topology。topology/storage/support evidence、candidate signer、ambient Windows root、应用/备份恢复域和 ADR-0020 2-of-3 recipient/share roster 均不得认证 topology signer。topology revision 1/null predecessor 后只能 prior+1 且 exact 引用前一完整 envelope；旧签名、fork、回滚或目录“最新”均拒绝。
- topology 的 `authority_storage_manifest_ref` 必须是 active configuration 选择的当前 `F57AuthorityStorageManifestV1`，其 deployment/epoch/generation 与 topology、recovery cut、checkpoint draft/payload 和每份 safeguard readback exact-join；最高档的 `backup_target_ids` 必须是严格 singleton `[continuous_target.target_id]`。角色按 enum exact 六行且 principal/credential 唯一；writer credential exact-join mTLS client SPKI，target-agent credential exact-join server SPKI，target receipt 的 signer principal 必须是该 topology role binding 中的实际 principal 而非角色名文本。它同时固定生产主机、一台服务器外连续目标和按序 `ROTATION_A|ROTATION_B` 两块 distinct HDD；A/B 的 media ID、hardware serial、volume identity、volume GUID 与 live physical-disk identity 各自非空且两两不同。令 `E={PRODUCTION,CONTINUOUS,ROTATION_A,ROTATION_B}` 且 `D={failure,administration,credential,custody,location}`；对任意 `d∈D` 与 `x≠y∈E`，必须从 topology-pinned support evidence 直接证明 `domain[d,x]≠domain[d,y]`。五组域各 6 个 pair 全部必须成立，Boolean 不作原始证据。同 tenant/root/管理组、复用 SPKI/secret/recovery credential、同宿主/机房/电源故障边界、同 custody roster 或同位置任一负例都必须失败。
- clean install 只能以 `expected_latest_backup_checkpoint_ref=None`、连续/离线 retained refs 与 current head 全空、`INITIALIZING + INITIAL_POPULATION` 开始；它可以完成基础设施安装，却不能授权 PITR、发布、恢复认证或生产。每个检查点必须在 immutable recovery cut 已存在后、draft 构造前新采 `BackupCheckpointPreparation` 与 strict `StorageSafeguardReadbackV1`，并 exact-repeat backup set、正序号、context、barrier、cut、expected prior head。第一份检查点仅允许 sequence 1/previous None；签名后进入 `BOOTSTRAPPING`，先把 current head 复制并验证到 A/B，再在仍低于 minimum 且其余健康条件成立时由已验证 head checked+1 产生 sequence 2/后续代。达到最小连续与 A/B-union 代数且最新 head 的 A/B 验证闭合后，下一次 fresh readback 才能成为 `HEALTHY`。retained refs 必须全部 typed-load、按 sequence 排序且逐项 previous-link；backup set ID 不可跨链复用，current head 必须匹配当前 trust/topology/storage tuple。陈旧 head、分叉、断链、循环、错误 media/tag、跳号或用目录“最新”代替显式 ref 均失败。
- 正常 trust/topology/storage roots 只可从 fresh `HEALTHY` 轮换；在 CAS 前必须先建立 deployment-wide `ProductionAdmissionHoldV1`，拒绝新请求/长任务、排空全部 accepted lease 并落盘 `write_barrier_id`。一次 CAS 再固定旧/新 tuple 并进入 `TRANSITIONING`，仅允许一份以旧 head checked+1 续接、绑定新 roots 和同一 hold/barrier 的 bridge checkpoint。之后必须进入 `BOOTSTRAPPING + CURRENT_ROOTS_ROTATION`，不得再建 checkpoint，只能完成 bridge 的 A/B 离线复制/验证后回到 `HEALTHY`；hold 在 `TRANSITIONING|BOOTSTRAPPING` 全程不可撤销。只有 fresh `HEALTHY`、transition 为空、head exact-bind 新 tuple 且 admission CAS 重新验证 epoch/OBSERVED generation/零旧 lease 才可重开；未恢复健康前禁止第二次轮换。
- DATA_HDD 灾难死亡是单独 `DATA_HDD_DISASTER_REPLACEMENT`，不走 normal root rotation，也不要求死盘 fresh `HEALTHY`。唯一可接受链从服务器外 current configuration/trust 与最后已认证 checkpoint/cut 出发，经双人恢复授权、deployment-wide hold、旧 authority/死盘 fencing、checked 提升 authority epoch/storage generation、新 HDD physical+volume identity/GPT/NTFS/BitLocker/新 storage manifest、洁净恢复和 PostgreSQL PITR/全数据核对、新 tuple 上连续备份和 A/B 空链 bootstrap 到 fresh `HEALTHY`，再完成当前 P340 容量/载体重认证，最后才允许 admission CAS 接管。未来新的 IaaS graph/profile version 必须另行冻结其等价重认证链。该 PITR 是 hold 下的恢复专用权限，不给普通 non-healthy 运行态放行；任一缺口保持生产关闭。
- 每次服务安装、checkpoint preparation、PITR、生产启用及 retry 都必须新采 strict plain `StorageSafeguardReadbackV1`，使用新 32-byte challenge、session、对象和同 boot/attempt binding；expiry 必须 checked-equal `observed + topology.max_age`，max age 在 1..=300 秒，消费时 trusted now 与 current topology head 均仍有效。唯一 support-evidence root 对 target receipt 使用 target-agent 单签、对介质转换/安全弹出/物理断开/保管/健康使用 topology 中两个互异人类保管者双签；所有 refs 必须 typed-load 正确 tag 并 exact-hash containing-field projection。writer、target-agent、partial-maintenance、retention、signer、recovery 六类的完整 canonical 权限负探针逐项被存储侧拒绝且前后摘要不变；target-agent 的任意直接操作全部拒绝，正常 append 只走一次性 capability。只有连续 retained refs 与 A/B 两个 verified-ref 向量都非空、每个离线向量都是连续链子集、A/B 两个离线向量的并集包含 latest head，且连续集合与 A/B 并集分别满足最小保留代数并通过 fresh readback，状态才是 `HEALTHY`；任何缺证、漂移或未知转为 `NON_SUPPRESSIBLE_RISK`，不能以降级标签压掉。`INITIALIZING|BOOTSTRAPPING|TRANSITIONING|NON_SUPPRESSIBLE_RISK` 均不能授权 PITR、发布、恢复认证或生产启用。
- 容量不只计算公式：连续盘 `total>=retained+validation+growth+reserve`、`free>=validation+growth+reserve`、quota+reserve 不越 total、used 不越 quota、reserve 实际可用；离线盘 `total>=recoverable+validation+growth` 且 free 足够 validation+growth。所有加法 checked，partial count/bytes/oldest-age optionality一致且 expired 为零。最小有效保留期 checked-equal `max(法定/合同保留,90 天,2×检测延迟 P99+洁净恢复验证窗口,2×离线轮换周期)`，至少两个已验证代际、离线年龄不超过 604800 秒；A/B 永远 `bundle_contains_recovery_material=false`。每块介质的 sequence 1 必须是 null predecessor/null previous state 的 `BLANK`，其后只允许八条边：`BLANK→ENROLLED→ACTIVE_APPEND→VERIFIED_DISCONNECTED→ROTATION_DUE→ACTIVE_APPEND`，或从 `ACTIVE_APPEND→SEALED_VERIFIED→RETIRED_PENDING_DISPOSAL→DESTROYED`；`SEALED_VERIFIED` 不得回到可写态，`DESTROYED` 为终态，重用物理盘必须换新 media ID 并重启 sequence 1。生产启用时 A/B 只能处于 `VERIFIED_DISCONNECTED|SEALED_VERIFIED`，零挂载、授权撤销、安全弹出、物理断开、健康且恰好两个互异人类 custody binding/signature；transition sequence/predecessor/ref/hash 和 live head 必须逐项吻合。
- 备份密钥、恢复凭据和生产管理员分离；厂商默认不能解密。
- 同机副本、RAID、VM snapshot 和普通可写共享均不得单独计入勒索恢复层。
- 恢复在洁净 Windows Server 上验证数据库、附件、审计链、配置代、包、密钥和业务勾稽，不以“文件可打开”代替。
- 离线轮换介质除挂载窗口外必须物理断开；同一时刻接入全部轮换盘、备份代际被恶意快速耗尽、只选择攻击者推荐的“最新恢复点”均视为恢复攻击，必须以独立签名检查点和多代抽检发现。
- Windows Local Administrators、Backup Operators、VSS/存储管理员、EDR/Defender 管理员和备份存储管理员的联合滥用属于高影响攻击故事；产品必须分离身份、记录独立检查点并明确剩余治理风险，不能声称软件能约束所有客户 root 合谋。

### 10. 暖备脑裂和恢复回放

威胁：网络分区后主备同时写；旧主恢复联网后接受旧 generation 命令；未经验证的 standby 被提升；复制把勒索密文或逻辑损坏同步过去。

强制控制：

- 任意时刻一个 deployment 只有一个带有效 write authority epoch 的节点。
- 提升前必须通过可验证 fencing 隔离旧主机；无法证明隔离则不得提升。
- 客户端 discovery 和命令绑定 authority epoch，旧 epoch 请求拒绝。
- 暖备健康只代表快速故障恢复，不替代不可变/离线备份；勒索时可选择从更早洁净点恢复。
- RPO/RTO 只按实测演练认证，不因存在复制配置自动宣称达标。

### 11. P340 工作站与 Windows Server 残余风险

威胁：消费/工作站级单电源与单网卡、非热插拔盘、未证明具备服务器级 BMC；选配 AMT 被误当成服务器 BMC；Windows Server 驱动或固件未受官方支持；断电、过热、SSD/HDD 健康不可见；系统生命周期缩短可维护窗口。

强制控制：

- 上线前验证 BIOS、TPM、BitLocker、网卡、存储控制器、磁盘健康、散热、断电恢复、UPS 自动关机和 Windows 服务重启。
- UPS 的攻击面包括把 Windows 聚合电池状态冒充设备身份/自检/输出状态、恶意或被替换的适配器取得任意设备/网络权限、配置投影偷换受控 outlet，以及响应丢失后重复安排断电或在重启后事后拼接 ACK。`UpsAdapterManifestV1`、`UpsStatusReadbackV1`、`UpsOutletCycleCommandV1` 与 `UpsOutletCycleCommandAckV1` 必须使用唯一 UPS schema；manifest 的 `implementation_binary_ref` 必须逐字节等于 `WindowsAuthorityArtifactSetV1.authority_kernel_binary_ref`，其重开摘要再等于运行时 `held_implementation_binary_sha256`。`configuration_projection` 固定 configuration generation、设备 profile、outlet group 与受保护主机供电路径，status/command/ACK 都逐字重复 projection 摘要和 generation。状态与控制端口分权，命令本身不得携带 endpoint、credential、路径、argv 或厂商任意 payload。
- Windows 标准电源状态 carrier 只可用于 AC/电池/电量/runtime 监测，manifest 的设备 profiles 必须为空、status 的 profile ID 必须为 null/self-test 为 UNKNOWN，且逻辑 adapter identity 只绑定 carrier/manifest/configuration，不能冒充 UPS 硬件身份；未知值必须保持 `UNKNOWN`，不得补默认。它没有受控 outlet、设备身份、自检和按 command ID 查询能力，所以最高安全档与 `POWER_SHUTDOWN` 只能使用候选绑定的 `SIGNED_VENDOR_ADAPTER`，能力不足必须返回 `CAPABILITY_INSUFFICIENT`。
- 签名厂商适配器只在既有 `EPAuthorityControl` 内以第一方 Rust 代码运行，不加载厂商 DLL、不启动子进程。每个 status exact-hash signed identity 的 boot/PID/start-key runtime binding，跨进程 status/sequence 不可混用；P340/POWER 只接受由 closed provider raw attestation 证明的 24 小时内 self-test PASS。USB 设备使用 canonical GUID/instance 且 ACL 只授 SYSTEM 与该 service SID、网络全拒；网络型使用 numeric-IP octets/nonzero port structured endpoint，runtime exact 一行，禁 DNS、文本别名、proxy、redirect和额外目标。凭据仅可为 service-SID 限定的不可导出 CNG key 或 DPAPI-NG sealed secret，不得进入 argv、环境、日志或证据。
- status 每 5 秒采集、有效期严格为 `observed+15s`，provider-attested self-test 最长 86400 秒，command ACK 最长 30 秒；sequence 只在同一已验证 process-start binding 内递增，进程重启必须重建安全绑定。同一 `(ups_adapter_identity,command_id)` 和相同 command digest 必须 query/adopt 并返回逐字节相同 ACK。adapter 在任何 provider 调用前耐久化不可重采的 private monotonic start marker；厂商成功调度还必须返回 1..128 字节且匹配 `[A-Za-z0-9][A-Za-z0-9._:-]{0,127}` 的非空 `provider_operation_id`，并在 ACK 前把它与 adapter identity、command ID、command digest 耐久绑定；schedule、exact-ID query 与 operation-log readback 必须逐字重复。同一 ID 不同 digest 为 `COMMAND_ID_CONFLICT`；operation ID 缺失、漂移或跨 command，或 30 秒时无法证明是否已发送/无法查询，均为 `COMMAND_STATE_UNKNOWN` 且禁止重发。ACK 必须同 boot/source 且在 checked `start <= observed <= min(start+30s,command deadline)` 内被观察，UTC 仅供报告；POWER 的 600 秒只作 User32/composite/preshutdown 崩溃对账窗，不能放宽、重置或复活内层限制。boot 已改变而 composite ACK 未耐久化时必须保持失败，不能用事后观察到的断电/重启事实重建 PASS。
- 最新 status 到 `observed+15s` 即视为 runtime link loss，无论 AC 显示什么都立即创建 deployment-wide `ProductionAdmissionHoldV1`、拒绝新请求和新长任务。从首次失鲜 monotonic tick 起只有一个累计 60 秒、不因闪断重置的恢复窗；只有原 identity/configuration/runtime binding 不变且连续两个递增 sequence 的 fresh status 在 communication/self-test/output/runtime 全 PASS，才可用 admission CAS 撤销 hold。60 秒未闭合时，即使 AC 仍在也启动本地安全链：排空、耐久审计/Outbox/附件、fresh PostgreSQL checkpoint、停库、Windows shutdown。无 outlet 控制或无 same-command typed ACK 不得宣称外部操作成功，但仍须完成 guest/local shutdown；再启动需人工处置和新鲜 UPS/电源/DATA_HDD/PostgreSQL 证据后的新 admission CAS。
- 依赖图本身是防复制/防旁路控制：`ep-platform-release` 必须直接依赖唯一 `ep-platform-ups-contract`，`ep-authority-kernel` 必须直接组合该 contract 与 `ep-adapter-ups-windows`；release/kernel 不得复制 UPS wire、从 testkit 反向取生产类型或绕过 typed ports。Task 13 先以 `f57_ups_adapter_contract` 和 `f57_ups_command_reconciliation` 冻结 common/P340 行为，Task 14 只把同一合同接入 POWER 长链，Task 15 才能在同一 clean frozen candidate 与真实 UPS 上生成发布证据；编译、schema DAG、byte golden、崩溃切片和实机 gate 缺一不可放行。
- 缺少冗余电源、磁盘热插拔和经证明的服务器级 BMC 作为已接受残余风险展示；即使存在 AMT 也不得据此宣称具备 BMC 等价能力。
- 每块生产 HDD 必须记录型号、序列号、固件、CMR/SMR、厂商工作负载等级和保修状态；未知项或 SMR 盘不能仅凭空载、短时或 72 小时稳定测试取得生产放行。
- OS SSD 也必须记录型号、序列号、固件、SMART、剩余寿命、温度、掉盘行为和更新空间预算；SSD 故障后的洁净重装与权威数据恢复必须计时演练。单网卡/单电源中断作为可见残余风险，不得被备份或 UPS 伪装成高可用。
- Windows Server 补丁进入签名、备份、维护和回滚门；同时保持 OS adapter seam 和迁移计划，不把 2022 视为永久平台。
- 升级 RAID1、64GB、UPS、独立备份设备或暖备后重新认证容量和恢复，不继承旧证书。
- 非 ECC 位翻转、控制器/固件缺陷、写缓存谎报 flush、恶意 USB/启动介质和物理拆机均进入上线/周期性演练；无法通过软件消除的风险永久显示并由客户签收。

#### 11.1 未来客户自控 IaaS Windows Server `HDD_STRICT` 扩展缝（当前关闭）

威胁：provider 把 SSD-backed/cache-backed 虚拟盘标为 HDD，快照/缓存/临时盘/运维副本越境或被 provider 人员复制，tenant root 与备份恢复凭据同域，vTPM/VM 回滚后重放旧 epoch，或将 provider 电源描述当作已验证物理 UPS。

强制控制：当前 selector 必须在读取任何 provider 证据前以 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED` 拒绝，IaaS 不得进入候选、发布、激活或生产 terminal。未来只有新 graph/profile version 可增加独立 `IAAS_WINDOWS_SERVER_HDD_STRICT` profile/recipe；它必须与 P340 互斥且不硬编码或复用 P340 CPU/主板/物理 UPS/outlet。届时必须证明客户控制 tenant/subscription、OS/网络/密钥/备份管理，客户控制的中国大陆 region 及 cache/snapshot/temp/host-migration/support 驻留，fresh Secure Boot/trusted boot/vTPM attestation，BitLocker/anti-rollback/clean-vTPM 恢复，虚拟数据卷到底层 HDD 介质的可验证映射，cache/snapshot/replica/temp/maintenance-copy 的介质、加密、保留和 provider 运维边界，以及 `PRODUCTION|CONTINUOUS|A|B` 四个实体在 failure/administration/credential/custody/location 五域的全部 30 条逐对不等。provider power/shutdown 等价证据必须提供 15 秒 fresh status、60 秒 hold/本地 checkpoint-停库-guest shutdown、幂等 control-plane operation ID 与 query/adopt/无 ACK 不宣称成功，不得伪造 outlet ACK。同 tenant snapshot 不是备份；快照回滚必须被 authority epoch/storage generation/checkpoint head 负例拒绝。

任一 provider/region/vTPM/HDD/cache/snapshot/operations-boundary/backup-domain/power-shutdown 事实不可证明、不可独立复验、过期或变更，都使状态固定 `STORAGE_MEDIA_UNVERIFIED` 并关闭生产。当前 IaaS recipe 仍是 `NOT_IMPLEMENTED`，P340 仍是当前物理基线；任何文档声明都不代表已取得生产证书。

### 12. AI、MCP 与工具型提示注入

威胁：业务文档或工具响应包含提示注入，诱导 AI 调用越权能力；MCP server 伪造 schema、返回超大/恶意内容、SSRF、重放或窃取凭据；模型建议被误当批准；外部 AI 泄漏高密数据。

强制控制：

- 模型输出只是 proposal；工具解析、授权、字段裁剪和命令验证由确定性代码完成。
- AI/MCP 使用与人相同的 principal/capability/scope 模型，但不得委托或自提权。
- 高风险命令在工具注册、plan compile、运行授权和业务 handler 四层均要求人工批准或绝对不可达。
- 外部 AI 默认关闭；数据 allowlist、密级、最小化和脱敏在出站前确定性执行，最高密级恒拒绝。
- MCP 网络目的地、DNS/IP、代理、重定向、端口、文件和响应大小均按 manifest 限制；凭据用短期引用，不进入提示或响应。
- AI/MCP 失败、超时或停用不降低确定性流程的安全门。

### 13. 生产附件恶意内容策略

威胁不仅包括普通恶意文件，还包括被攻陷 provider 伪造 `PASS`、病毒库过期但服务仍健康、压缩炸弹/递归档、多格式 polyglot、扫描后替换字节或路径，以及把旧 digest 的 verdict 复用于新对象。

最高安全档的生产部署不得以“未扫描但提示降级”发布附件。接收链固定为：写入 HDD 加密 quarantine；冻结字节并计算 digest、长度、类型、结构和展开预算；调用经批准 provider；把 verdict 与该 digest、engine identity、definition version、时间和策略签名绑定；复核签名与字节未变后原子发布。签名 provider policy 必须定义 `maximum_definition_age` 且不得超过 72 小时；断网部署只通过签名离线定义包更新，定义年龄超限即视为 scanner unavailable。任一字节、路径或元数据变化必须重新扫描；定义过期、`UNKNOWN`、`SKIPPED`、timeout 或 provider 不可用全部保持隔离，附件不可下载或内联展示。`NONE` 只允许开发、测试或无真实客户数据环境，不构成生产认证。发布门必须包含 EICAR、stale definitions、zip-bomb、递归档、polyglot、伪造 verdict 和 TOCTOU 替换负例。

### 14. 现行安全不变量补充

1. Windows Server 权威节点上任一内容承载或可关联客户的持久字节落入 SSD 都是生产阻断，不是普通告警；终端最小加密非权威缓存适用 §7 的独立规则。
2. 任一能力包、配置代、权限或迁移无法完成 exact 签名/依赖/回滚验证时失败关闭。
3. 任一高风险效果的状态为未知时必须对账，不能重复执行或标成成功。
4. 任一动态授权不能证明来源、范围、期限和委托上限时拒绝。
5. 任一上线部署没有服务器外追加式自动增量层、完全离线加密轮换层、独立恢复身份/材料和洁净恢复演练时不得存入真实生产数据。
6. 任一暖备提升不能证明旧主 fencing 时不得取得写权威。
7. 任一客户端、插件、MCP、AI、门户或 Excel 路径都不得绕过同一业务能力、动态授权和审计。
8. 安全控制、备份、审计、加密和完整导出不因许可证或模块停用而消失。
9. 热替换后仍运行的 zombie worker、旧 token、旧 generation 和排空超时一律不得接受新效果；不可逆迁移只能走维护档并保留前向修复证据。
10. 长链重试风暴、循环重开、补偿级联或外部 provider 抖动不得耗尽单 HDD/WAL/审计空间；资源预算、队列上限、熔断和人工事故箱必须共同生效，已接受耐久任务不得静默丢弃。
11. normal current-roots 轮换在 CAS 前必须全局 hold/drain/barrier，过渡全程保持 hold，仅 fresh `HEALTHY` + exact 新 roots + admission CAS 可重开。
12. DATA_HDD 死盘不要求不可读旧盘 fresh `HEALTHY`，但只有 off-host current trust/config/checkpoint/cut 出发的双人、fenced、高 epoch/generation、新卷、洁净恢复/核对、fresh backup/A-B 与容量重认证链能接管。
13. UPS status 失鲜 15 秒即全局 hold；只有同 identity 在累计 60 秒内两次 fresh PASS 可撤销，否则 AC 仍在也必须本地 checkpoint/停库/关机，无 outlet ACK 不得假定成功。
14. PostgreSQL 历史日志只能由 `EPAuthorityControl` typed 操作按签名清单清理，current/7-day/legal-hold 集不可删；空间门取现有与 50/40 GiB 底线中更严格值。
15. 当前 IaaS selector 必须稳定返回 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED` 且不得进入任何生产 terminal；未来新 graph/profile version 即使启用独立 recipe，也不得复用 P340 证据，并须完整证明客户控制/大陆驻留/vTPM/HDD-cache-snapshot-运维边界/备份域/power-shutdown 等价性。

## HISTORICAL_NON_NORMATIVE_APPENDIX B：旧攻击面详本

> 下文是 F-55/F-56 时期的攻击故事详本，只用于复用仍然适用的测试想法。出现扫描降级、仅 WASM、固定出网进程、固定服务数、WinCred、本地模型、六方法 MCP、第五客户端、90 格或旧延期范围时，以 F-57 增补节和需求追踪为准，禁止照抄实现。

### 1. 身份、会话、授权与职责分离

相关风险包括凭据填充、弱 MFA 恢复、会话固定/重放、越权对象 ID、批量接口遗漏字段权限、管理员自批和重新认证证据复用。服务端必须对每个命令和查询重新计算能力、法人、记录、字段、状态与职责分离，不信任客户端隐藏按钮或门户投影。

攻击者故事：窃取普通销售账号后，攻击者尝试把请求中的 `legal_entity_id` 或合同 ID 换成另一法人、直接调用财务端点，或复用一次重新认证令牌提交多次付款。若任一路径能返回数据或产生效果，均是隔离/高风险授权失效；正确行为是统一拒绝、无部分写入、记录可归因审计并触发必要限流与告警。

### 2. 公网门户、API 与浏览器攻击面

供应商门户与核心同机，因此 SQL/命令注入、XSS、CSRF、SSRF、请求走私、路径混淆、反序列化、过量请求或 portal-gateway 本地提权的影响半径高。缓解包括独立站点/证书/系统账号、WAF、严格路由、同源与 CSRF 策略、输出编码、参数化查询、大小与速率限制、只调用受控能力 API、只读取脱敏投影，以及 portal 进程无数据库凭据。

攻击者故事：恶意供应商在发票备注或附件名植入脚本，试图在采购人员浏览时窃取会话；或构造其他供应商的采购单 ID。存储、列表、导出和通知渲染必须统一编码，服务端必须把外部账号同时绑定法人、供应商和目标单据，不能只校验“已登录”。

### 3. 文件、Excel、附件与历史迁移

不可信文件可能包含宏、公式注入、路径穿越、压缩炸弹、解析器漏洞、恶意 PDF/图片、同形文件名、超大行数或伪造 MIME。XLSX/CSV 导出还可能把以 `=`, `+`, `-`, `@` 开头的数据解释为公式。历史迁移源可能包含重复键、恶意编码、缺失安全属性或能造成整批错账的数据。

缓解要求：白名单格式、魔数与 MIME 交叉检查、大小/页数/行数/压缩展开限制、安全解析器、公式转义、服务端对象键、不可执行存储、扫描与隔离状态、批准的版本化迁移模板、试运行、逐项对账、幂等批次、错误队列和整批可冲销计划。扫描器不可用时按冻结模式拒绝或显式降级，不把“未扫描”标成“安全”。

攻击者故事：供应商上传外观为 PDF 的可执行或解析器攻击文件，并诱导内部用户下载。系统必须在发布给业务用户前完成类型、限额和扫描策略，原始对象不可被浏览器以内联可执行方式打开；扫描结论、哈希和版本进入审计。

### 4. 数据库、RLS 与查询/报表

最重要的失败类是漏法人列、RLS 未 FORCE、复合外键只按裸 UUID、分析查询绕过字段权限、会话变量池化污染、动态 SQL 注入和迁移账号长期启用。缓解包括法人复合键、RLS 矩阵、会话取用/归还清理、参数化 SQL、只读角色、查询解析和资源上限、临时 DDL 身份、迁移窗口审计，以及跨法人负向测试。

攻击者故事：法人 A 的报表请求先占用连接，连接归还时未清安全上下文，法人 B 随后复用同一连接并读到 A 的数据。连接池必须在取用前设置并复核上下文、归还前清除；任何清理失败都销毁连接，不再放回池。

### 5. 财务、库存与并发状态机

金额符号错误、重复过账、核销/退款/红冲次序不一致、采购暂估重复入账、关账后回写、竞态超额和错误锁序可导致可利用的财务篡改或长期不平。缓解来自冻结的 `VoucherSourceKind`/`MeasureKey` 映射、服务器端金额重算、唯一键、效果链、全局锁序、同事务凭证与台账、期间历史切片、逐日和关账前勾稽、不可静默豁免的差异门禁。

攻击者故事：拥有合法付款权限的内部人员并发提交相同业务键，试图形成两张付款或多释放预付；或者利用作废、退款和红冲的不同顺序制造负未核销余额。幂等键、锁后重算、唯一约束和追加效果链必须使并发结果等价于某一合法串行顺序，任何不变量失败整事务回滚。

### 6. 插件、低代码与配置发布

签名插件仍是潜在恶意代码；签名只证明发布者，不证明行为安全。低代码表达式、只读 SQL、流程与补偿策略可能造成资源耗尽、越权查询或绕过业务状态机。首版服务端只允许签名 WASM Component，默认零能力；能力按对象、字段、域名、端口、文件、密钥和资源逐项审批。插件不直连数据库、不读取明文机密，宿主强制超时、内存/CPU/调用限额并审计。

攻击者故事：经批准的报表插件在更新版本中尝试读取未声明字段并向新域名外传。每个版本必须重新验签并匹配能力清单；宿主在调用时而非只在安装时检查权限，目的地不在批准集合即拒绝并停用/告警。

### 7. 外部集成与 SSRF/回调

集成网关是首版唯一业务出网进程。签章回调、推送服务和 ICAP 可返回伪造状态、重放结果、超长响应、重定向或指向本机管理服务。缓解包括固定连接器、目的地清单、DNS/IP 重绑定防护、禁止未批准重定向、mTLS/签名验证、请求关联和重放窗口、超时/大小限制、最小载荷与错误隔离。ICAP 是唯一回环 TCP 窄例外，只能连接客户批准的 IP 字面量回环端口，不接受主机名、重定向或非回环目标。

攻击者故事：恶意管理员把签章 URL 改成云元数据或本机数据库管理端口。配置发布必须拒绝不在连接器类型和批准目的地集合中的地址；运行时解析后的每个 IP 仍须复核，不能只在保存配置时检查字符串。

### 8. 客户端、本地缓存与设备插件

风险包括令牌落盘、离线草稿泄露、调试日志含 PII、深链/自定义协议注入、WebView XSS 到原生桥、恶意本机进程劫持 IPC，以及签名原生插件越权读取设备或文件。缓解包括系统安全存储、最小缓存、字段级加密/脱敏、会话撤销、严格 IPC 消息类型与对端身份、原生桥能力封闭、插件独立子进程、签名与本地能力清单。

攻击者故事：桌面插件请求“打印”能力后尝试读取客户端缓存数据库或会话令牌。插件子进程不得继承这些句柄或密钥，只接收核心传入的最小打印数据；越权 IPC 调用拒绝并记录设备审计。

### 9. 密钥、秘密与密码边界

风险包括同一密钥跨法人/用途/进程复用、六个服务共享一个系统机密 master 导致横向解密、KMS bootstrap 与数据库口令形成循环、明文或旧迁移树残留、密钥出现在日志或备份、恢复材料与数据副本同放、备份身份可解封全部密钥、删除密钥绕过保留义务，以及盲索引宽度/派生方式漂移。缓解包括用途与法人域分离、数据 KMS common master 与系统机密库分离、每 recipient 独立 DPAPI machine-scope 32-byte KEK 或 HSM nonextractable object、绑定 deployment/recipient/purpose 的 entropy/AAD、严格 `EPS1` 信封与版本 ref、固定 32 字节 HMAC-SHA256 盲索引、双人控制、恢复材料分离、定期核验、密钥销毁证明和日志脱敏。`ep-secretctl` 是唯一 writer/legacy reader；生产发布必须证明 legacy、quarantine、staging 均无残留且常驻制品无 legacy reader。

攻击者故事：备份落点账号被盗后，攻击者同时寻找同目录中的密钥恢复材料。实现必须把恢复材料交给不同身份和落点保管；单独取得加密副本不能解密，单独取得恢复材料也不能读取副本。

攻击者故事：攻击者攻破 `ep-worker` 后复制另一个 recipient 的 DPAPI blob 或 EPS1 文件，或诱导进程把 recipient 改为 `ep-backup`。Provider 构造时必须固化 deployment 与 recipient，DACL、DPAPI entropy、HSM object、信封 AAD 和 key ref 五层逐字绑定；取得本服务的 KEK 不能形成选择或解密其他 recipient 的产品路径。

### 10. 审计、事件、幂等与重放

攻击者可能尝试让业务成功而审计失败、删除 Outbox 后重放请求、伪造系统用途、乱序消费或借死信重放重复产生效果。缓解包括同事务写入、哈希链与段根签名、稳定事件/错误/类型目录、消费者幂等键、效果唯一约束、租约队列、DEAD 状态与受控 replay、系统上下文构造封闭及关账前对账。

攻击者故事：内部人员中断审计写入后再次提交付款，希望付款落库但证据缺失。审计、业务效果和 Outbox 任一写入失败时整笔付款失败；恢复后相同幂等键只能返回原结果或保持失败，不能再生成第二笔。

### 11. 构建、依赖、签名与离线更新

供应链风险包括依赖替换、构建机泄密、迁移脚本植入、签名密钥滥用、白标制品混淆、旧版本降级和伪造离线补丁。缓解包括本地 Git、锁定依赖、离线依赖仓库、由 Rust-owned F-57 command family 唯一判定且 CI 仅作薄适配、SBOM、可复现构建、制品摘要、生产 Authenticode、版本单调性、客户侧离线验签、双人批准和可审计安装/回退。

攻击者故事：攻击者把合法旧补丁重新签名或复制到另一白标客户。安装器必须校验客户/产品标识、版本、兼容范围、包摘要与签名链；不允许仅凭“签名有效”跨客户或降级安装。

### 12. 勒索软件、备份破坏与恢复欺骗

单机形态使本机文件、数据库和在线凭据可能同时被勒索软件控制。攻击者目标通常不是只加密生产盘，而是使用备份写入身份删除、覆盖或污染服务器外历史副本，再伪造校验成功。控制重点是权限断链：归档/备份写入身份只能创建不可覆盖对象和读取校验所需内容，不能删除、覆盖、改名、清理版本、修改 ACL/保留策略或取得恢复/处置身份；对象键按内容或备份集唯一，重复写不得覆盖；恢复身份密封且不用于日常服务；到期处置使用第三身份并双人批准。

对象存储必须使用条件创建并以 IAM 负向测试证明日常身份无法删除/覆盖/改保留；SMB/Windows 目录必须以 ACL 明确拒绝删除、子项删除、改 ACL 和取得所有权，并使用创建新文件而不是覆盖打开。平台在部署与周期自检中验证这些负向条件；缺失、返回不确定或测试失败时打开不可抑制的 `OFFSITE_COPY_PROTECTION_MISSING` 窗口并阻止发布。恢复演练必须从服务器外副本重建并验证业务不变量，不能把“文件可列出”当作可恢复。

攻击者故事：勒索软件取得 `backup-writer` 服务身份并尝试枚举后删除所有备份。正确配置下，该身份最多能创建新的加密对象并读取有限校验数据，删除、覆盖、重命名和权限变更均被存储侧拒绝；拒绝结果进入独立审计。若客户存储管理员凭据同时被攻破，首版不能声称阻止删除，这一残余风险必须通过客户身份分域、额外离线/不可变层和合同披露处理。

### 13. 运维、迁移与恢复特权

临时 DDL、数据迁移、恢复、销毁和紧急运维具有高权限，最容易绕过正常 API。缓解包括独立短期身份、无常驻数据库凭据的 `ep-data-migrate`、只读来源、签名模板、受控迁移 API、试运行与对账、操作窗口、双人批准、逐步审计、恢复隔离环境和结束后回收权限。任何工具不得直写业务表或把原始数据/凭据长期存入平台数据库。

攻击者故事：实施人员在迁移 CSV 中把记录法人改为自己可访问的法人，以掩盖跨法人数据。迁移模板必须显式赋值并校验安全属性，源键、规范化哈希、目标键和批次可追溯；对账按原法人和目标法人分别验证，差异不可静默批准。

### 14. 本地分析 AI

风险包括提示注入诱导越权字段或任意 SQL、模型包替换、compose 后撤权仍执行、查询结果进入模型/日志/dump、缓存跨法人或跨用户命中，以及推理内存/并发拖垮交易。控制按 F-55：模型输入恰好是裁剪目录、问题原文和固定模板；输出只能是单数据集计划；记录谓词由确定性代码最外层 AND 注入；人工确认后执行前重新检查全部安全事实；结果永不回模型；签名数据型模型包、零 DB/网络/文件写进程、独立 Job Object、15 运行 + 30 排队与九条收容断言失败关闭。

攻击者故事：有权查看 A 法人报表的用户在 compose 后切换法人或被撤销字段权限，再重放旧 token。execute 必须因安全摘要、目录摘要、模型/提示版本或法人变化返回稳定冲突且零查询；不能把旧结果或旧缓存返回。

### 15. 双向 MCP 与受控子进程

风险包括窃取短期 grant、设备证明重放、confused deputy、隐藏工具/资源枚举、普通命令重放、高风险动作绕审批、SSRF/DNS rebinding、redirect/proxy、远端响应注入、本地包路径穿越、子进程再生、网络/host mount/secret 环境变量泄漏，以及管理员把 credential 写进自己的 vault 后误以为服务可读。控制是 2026-07-28 六方法闭集、十分钟/百次并绑定 session/device/le/manifest 的 grant、逐调用 ECDSA 设备证明与单调 counter、逐次授权与列表裁剪、ExistingCommand 幂等、七项绝对禁区、固定 HTTPS origin+SPKI、gateway 零 DB/KMS/file/outbox、本地签名包、AppContainer+WFP+Job Object、禁 child/network/任意文件、响应 schema/字段/字节闭包及 Credential Manager 引用。WinCred 只由 SCM 加载服务账户 profile 后的目标服务 current token 经 60 秒、严格 DACL 的本地维护管道执行 CredWrite/CredDelete；签名 `ep-secretctl`、双人 CMS grant、purpose probe/rollback、2560-byte 硬界、服务重启持久读取和全路径 zeroize 均失败关闭。首次 Win32 mutation 前的 write-through intent 只含授权元数据；崩溃残留只能重建 CLOSED_FAILED 并要求同 target/purpose 新双人 grant 纠正，不能自动使用残留 credential。HTTP/ServerAdmin/argv/env/secret file 无入口。

攻击者故事：获批的 MCP connector 把目标域名解析到回环或云元数据地址，再用 302 跳到内部管理面。integration-gateway 必须在每次解析和连接时拒绝私有/保留地址、DNS rebinding、代理与重定向，并只连接 manifest 固定 origin 和 SPKI；失败不回退到通用 HTTP。

### 16. ServerAdmin 与客户端来源

风险包括把管理 SPA 当成超级管理员、伪造 `X-Client: server_admin|mcp`、以“仅查看”路径执行写入、把 90 格降成 UI 隐藏规则，以及通过静态制品获得独立端口或可写目录。控制是独立嵌入式静态 SPA、现有会话/MFA/device/CSRF/CSP、ServerAdmin 无新角色、18×5 数据库与编译期 hash 逐格一致、能力闸先于授权但不代替授权、Mcp 只能由 grant middleware 构造且不进矩阵。

攻击者故事：普通桌面会话自行提交 `X-Client: server_admin` 试图访问配置发布。服务端必须验证会话建立时登记的客户端/设备事实而非相信头值；即使来源合法，仍必须满足对象权限、字段权限、职责分离、重新认证和标准审批。

### 17. 未来 IaaS carrier 与云控制面（当前仅负向边界）

风险包括误把未来接口当作当前能力、把云盘快照当离站备份、生产与备份共享 tenant root/凭据域、provider/region 不满足数据驻留、vTPM 缺失却继续启用、宿主快照复制内存/密钥，以及用云托管数据库/KMS 偷换拓扑。当前控制是只允许 P340 carrier，任何 IaaS 选择固定失败关闭。未来新版本才可增加部署表 CHECK、provider/region/驻留/客户控制/vTPM attestation 证据、禁止托管组件、生产与备份在 site/zone 或 region、账户/凭据域和介质三维批准隔离，以及独立恢复和发布门禁。

攻击者故事：IaaS 客户把同一 VM 的第二块虚拟盘和同一 tenant 管理员可删除的快照登记为离站副本。部署门与恢复演练必须判定故障域不分离并拒绝发布；“provider 有 SLA”或“快照成功”不能替代不可覆盖负向探针与真实恢复。

### 不在首版内的攻击故事

- 通过 OCR 模型投毒、RAG 数据污染、外部 AI 回传或工业协议攻击运行时：这些能力未交付，出现监听、工具注册或隐藏配置即属范围违规。
- 攻击 Kubernetes、云托管数据库/KMS、SaaS、HA 或多区域复制：首版不支持这些形态；未来认证不得沿用本模型中单机 Windows/IaaS VM 的边界。
- 攻击整个平台容器化或工业协议适配器：首版只允许 plugin-host 为获批 MCP server 使用 `LOCAL_SIGNED_STDIO` 单子进程沙箱，或使用满足全部宿主证据的可选 `LOCAL_WINDOWS_HYPERV_CONTAINER`；通用容器插件、平台容器化与工业接入仍延期。
- 要求应用阻止同时拥有 Windows、数据库、KMS 和备份存储最高权限的恶意客户管理员：应用不能建立高于客户基础设施所有者的信任根，只能分权、检测、留证与如实披露。

## Severity Calibration

### Critical

满足下列任一条件通常为严重级：无需合法高权限即可跨客户或跨法人任意读取/修改数据；取得根密钥、签名密钥或可批量解密全部法人数据；从公网、文件或插件实现产品进程远程代码执行并触及核心/数据库；改写财务凭证或审计链且无法检测；日常产品身份能够删除或覆盖全部服务器外可恢复副本；绕过高风险重新认证和审批后执行付款、开票、结账或敏感导出。

例子：供应商门户的对象引用漏洞可下载任意客户合同附件；WASM 宿主逃逸后读取数据库凭据；`backup-writer` 凭据可删除所有历史备份且平台仍报告通过。

### High

通常为高危：读取或篡改单一法人中的大量 PII/商业数据；以普通内部账号执行未授权财务或库存效果；SSRF 访问本机管理服务或密钥接口；持久 XSS 可夺取财务/管理员会话；制品或迁移签名/版本校验可绕过；备份或审计链被污染导致恢复或追溯不可信；并发缺陷可稳定制造重复付款、重复核销或账务不平。

例子：报表只读池遗漏字段裁剪而导出整法人联系人；签章回调重放使合同重复生效；恶意 XLSX 导致受控迁移 API 以错误法人批量落库。

### Medium

通常为中危：在单一已授权对象范围内造成有限完整性损害；需特定内部权限且影响可由审计和回滚完整恢复；可造成有界后台任务或报表拒绝服务但不阻塞核心交易；泄露非敏感内部标识、版本或有限元数据；安全降级未清晰呈现但不会扩大权限。

例子：低代码表达式可耗尽其单次资源额度并延迟一个后台窗口；错误响应暴露存在性但不返回字段；单设备日志包含非敏感单据号且保留期受限。

### Low

通常为低危：只存在于测试、示例、历史研究稿或未交付入口，且无生产路由；不含秘密的轻微信息泄露；需要已经拥有等同或更高能力的客户基础设施最高权限，且不增加攻击者能力；只影响延期可选能力的可用性。

例子：历史研究稿中的示例端点缺少限流但不会被构建；已能完全控制客户 Windows、数据库和 KMS 的主体还可读取本机普通配置文件。若同一问题为该主体提供隐蔽持久化、越过独立备份管理域或破坏审计证据，严重度应按新增能力上调。

### 校准原则

- 以实际可达路径、所需前置权限、跨法人/跨客户范围、数据敏感度、资金影响、可恢复性和可检测性共同判定，不只按漏洞类别命名。
- 单机部署会放大门户进程本地提权、服务身份横向移动和勒索软件的影响，但不会把所有本机信息泄露自动升为严重级。
- “客户管理员可做”不是自动排除项：若产品本可分离身份却把删除备份、解密数据和修改审计集中给一个日常服务身份，仍按实际新增攻击能力评级。
- 延期能力只有在生产构建、路由、配置或监听中确实不可达时才降低严重度；出现隐藏入口、默认开启或可通过普通配置激活时，应按真实能力重新评级。
- 设计文档中的控制只有在实现、负向测试与发布证据同时存在时才计入缓解；当前文档冻结状态不降低未来实现缺陷的严重度。

## 规范依据

- 总体范围、架构、安全、部署、风险与发布门：`docs/superpowers/specs/2026-07-19-enterprise-private-operations-platform-design.md`
- 首版业务与角色/端点需求：`docs/superpowers/specs/2026-08-09-first-release-prd.md`
- 共享技术基线：`docs/superpowers/plans/2026-08-10-first-release-dev-plan/00b-technical-baseline.md`
- 身份与授权：`docs/superpowers/plans/2026-08-10-first-release-dev-plan/04-identity-authz.md`
- 财务一致性：`docs/superpowers/specs/2026-08-21-f50-financial-consistency-design.md`
- 本地 AI、双向 MCP、ServerAdmin 与云承载：`docs/superpowers/specs/2026-08-22-f55-approved-ai-mcp-server-admin-cloud-freeze.md`
- 许可证、离线可信时间与声明式签名模块包：`docs/superpowers/specs/2026-08-22-f56-license-signed-module-package-freeze.md`
- 客户端、低代码与插件：`docs/superpowers/plans/2026-08-10-first-release-dev-plan/13-clients-lowcode.md`
- 运维、备份、恢复与发布：`docs/superpowers/plans/2026-08-10-first-release-dev-plan/14-ops-backup-release.md`
- 唯一登记表：`docs/error-codes.md`、`docs/event-catalog.md`、`docs/config-reference.md`、`docs/metrics-catalog.md`、`docs/data-dictionary.md`、`docs/impact-catalog.md` 与 `docs/migration-catalog.md`
