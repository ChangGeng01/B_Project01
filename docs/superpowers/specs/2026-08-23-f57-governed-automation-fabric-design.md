# F-57 可治理自动化底座（Governed Automation Fabric）总体设计

> 批准日期：2026-08-23（Australia/Melbourne）
> 收敛修订：2026-08-24（Australia/Melbourne）
> 状态：**CURRENT / APPROVED；2026-08-24 架构收敛修订已由用户批准。本文可以作为实施计划与开发的规范输入，但本次批准只授权完成文档再基线，不授权开始 F-57 产品实现、迁移、实机认证或发布；开发授权已另于 2026-08-27 由使用方授予（F-65），见 :767 与 README:8**
> 产品定位：面向合同驱动型企业的、本地优先、可治理、可组合的业务自动化操作系统
> 生产基线：单台 ThinkStation P340 Tower、Windows Server 2022、i5-10500、32GB RAM、256GB SSD、单 1TB HDD、约 20 名活跃用户
> 功能基线：《管理软件基本需求》及现行 PRD 中与本文不冲突的业务细节

## 0. 本文用途与规范语言

本文把 2026-08-23 逐节批准的下一代设计冻结为开发输入。它解决的不是“再增加几个模块”，而是把现有平台提升为具有安全热插拔、耐久闭环、动态权限、客户自控部署和长期可演进能力的产品底座。

本文中的“必须”“禁止”“只能”“不得”是发布门禁；“可以”“允许”只表示在前置门禁满足后可启用，不表示默认启用。设计控制只有在代码、测试、Windows 实机、备份恢复和安全验证均取得证据后，才能宣称已经实现。

本文不授权开始开发。实施只能依据后续 F-57 实施计划，由用户另行批准后执行。

## 1. 权威关系与旧文档裁决

### 1.1 现行权威顺序

发生文字差异时，只有下面恰 25 份具名文件参加全局 precedence。README、评审和取代登记只能链接本表，不得复制、缩写或另排一套顺序。每一行只包含一份可定位文件；数字越小优先级越高：

1. 本文 F-57 总体设计；
2. [F-57 业务执行契约](2026-08-23-f57-business-execution-contract.md)；
3. [客户端/生命周期/安全运营执行契约](2026-08-23-f57-client-lifecycle-security-contract.md)；
4. [F-57 需求追踪矩阵](../reviews/2026-08-23-f57-requirements-traceability.md)；
5. [权威和取代登记](../reviews/2026-08-23-f57-authority-supersession-register.md)；
6. [ADR-0025](../../adr/ADR-0025-f57-capability-graph-and-feature-first-boundaries.md)；
7. [ADR-0024](../../adr/ADR-0024-f57-backup-key-envelope.md)；
8. [ADR-0023](../../adr/ADR-0023-f57-provider-manifest-resource-grant.md)；
9. [ADR-0022](../../adr/ADR-0022-f57-multi-lane-ci-and-windows-2022-authority.md)；
10. [ADR-0021](../../adr/ADR-0021-epb1-backup-envelope.md)；
11. [ADR-0020](../../adr/ADR-0020-dual-recipient-data-key-recovery.md)；
12. [ADR-0019](../../adr/ADR-0019-f57-runtime-topology-and-measured-connection-budget.md)；
13. [Windows/P340 生产档案](2026-08-23-f57-windows-p340-production-profile.md)；
14. [仓库级威胁模型](../../threat-model.md)（仅该文件首页声明的 F-57 现行部分）；
15. [`docs/f57-task-ownership.seed.tsv`](../../f57-task-ownership.seed.tsv)；
16. [`docs/f57-migration-baseline.v1.tsv`](../../f57-migration-baseline.v1.tsv)；
17. [`docs/f57-legacy-migration-disposition.seed.tsv`](../../f57-legacy-migration-disposition.seed.tsv)；
18. [F-57 收敛实施主计划](../plans/2026-08-24-f57-converged-program.md)；
19. [G0 启动计划](../plans/2026-08-24-f57-g0-bootstrap-implementation.md)；
20. [G1/G2 权威主干计划](../plans/2026-08-24-f57-authority-spine-implementation.md)；
21. [G3/G4 CTC-01 计划](../plans/2026-08-24-f57-ctc01-implementation.md)；
22. [G5/G6 扩展与发布计划](../plans/2026-08-24-f57-expansion-release-implementation.md)；
23. [F-50 财务一致性裁定](2026-08-21-f50-financial-consistency-design.md)；
24. [《管理软件基本需求》原始需求](../../介绍/管理软件基本需求.docx)；
25. [首版 PRD](2026-08-09-first-release-prd.md) 中未被以上文件改变的领域细节。

F-51、F-55、F-56、2026-07-19 设计、旧十四阶段计划、旧评审、配置/数据字典等未列文件不组成隐含的第 26 级，也不能彼此按日期裁决。它们只可作为历史/detail input：其中窄规则只有在上面 1–25 的某份文件明确重述，或以具名文件与具名章节/登记项精确绑定时，才通过该上位文件获得现行效力。若实施需要一个未被 1–25 收编的旧细节，或两个未列输入互相不等，文档门必须失败并先修订上位文件；不得由开发者猜测、按日期选新、或把“明确保留”解释成整份旧文件复活。

其中三份冻结登记仍保持原有内容与摘要：baseline 以 SHA-256=`52930d7ae32ee02ddda38199bcc144f5f6747fcfbe33e740741a0f21604ca8fd` 冻结 78 行（66 个不可变已存在、3 个待受控修订后应用、7 个由 F-57 取代且必须缺席、2 个延期接口且必须缺席），并与 310 行 legacy disposition exact-join 为 388 行 pre-F57 catalog；legacy 完整文件 SHA-256 固定为 `06566ca354b6279391e5ec3a0152316a8eb38d1f10cb09dc23953370883c3196`。实现计划只能落实上位契约，不能弱化 exact-set、状态、守恒、安全边界或阶段状态。G0 生成的数据字典、接口、迁移、错误码、事件、指标和影响面投影在运行时是图绑定的机器读物，但永远不能反向覆盖 CapabilityGraph；生成物与本表上位输入不等时必须失败关闭，而不是再用“后生成者优先”裁决。

低位文档不得反向恢复被本文取代的固定角色、固定九进程、首版本地模型、声明式内置模块包限制、SSD 持久化缓存或旧管理端形态。

### 1.2 明确保留与明确取代

| 旧输入 | 保留内容 | 被 F-57 取代的内容 |
|---|---|---|
| 2026-07-19 总体设计 | Rust、PostgreSQL、私有部署、领域能力边界 | 上一代总体架构、客户端/管理端形态、可插拔和自动化模型 |
| 首版 PRD | 客户、合同、订单、采购、库存、财务、售后、项目、报表等业务细节 | 固定岗位作为授权边界、旧模块组合方式、旧非功能容量和旧延期判断 |
| F-50 | 资金、发票、核销、冲销、台账、期间与一致性不变量 | 无；仅由本文重新限定“法定会计/税务系统可外接”的产品范围 |
| F-51 | 已批准的具体业务取值，且仅限与本文不冲突者 | 固定 RoleCode 是主要权限模型、旧配置发布、旧容量/流程默认值 |
| F-55 | 签名、隔离、双向 MCP、客户自控 carrier 的安全意图 | 首版本地模型交付、固定九进程、独立 ServerAdmin 客户端、旧 AI/MCP 能力闭集 |
| F-56 | 许可四态、客户数据保留、离线签名与信任链意图 | 模块只能声明已编译内置模块、禁止 WASM/容器/客户能力包的旧限制 |
| 旧十四阶段计划 | 可复用的领域测试、迁移和财务实现细节 | 作为整体的现行执行入口；不得在完成 F-57 再基线前直接执行 |
| 旧开发就绪验收 | F-49 九项与 F-10 冲突的历史关闭证据 | “旧体系可直接开发”的当前结论 |

### 1.3 仍然成立的历史结论

- F-49 的九项财务问题及旧 F-10 内部矛盾继续视为已由 F-50 关闭，不因 F-57 重新打开。
- 客户数据保留、离线签名、客户持钥、失败关闭、服务器外备份和完整导出继续成立。
- 历史文件允许保留原文以便追溯，但必须通过文档首页、索引或本裁决明确标记其现行范围。

## 2. 产品定义与系统宪法

### 2.1 产品定义

系统是“合同履约与回款闭环自动化平台”，主链为：

```text
客户 → 报价 → 合同 → 销售订单 → 采购需求 → 采购订单
     → 到货/交付 → 开票 → 收款 → 售后 → 续约
```

它不是以模块菜单为中心的传统 ERP，也不是仅靠聊天驱动的 AI 产品。它以业务目标、责任、异常、证据和闭环为中心，同时保留传统档案、列表、搜索、表单和报表作为辅助入口。

### 2.2 不可配置关闭的宪法

| ID | 规则 |
|---|---|
| F57-A01 | Windows Server 权威节点是数据、权限、流程、配置、审计和业务结果的唯一权威。 |
| F57-A02 | 客户端、门户、Excel、AI、MCP、插件和外部系统均不得直接连接或修改权威数据库。 |
| F57-A03 | 身份验证、授权、职责分离、事务、签名、审计、备份保护、财务不变量和数据生命周期属于不可关闭的安全底线。 |
| F57-A04 | 所有权威写入必须通过强类型业务能力命令，由服务器在执行时重新验证。 |
| F57-A05 | 每项业务能力必须有唯一所有者；其他能力只能调用公开命令、读取授权视图或订阅已提交事实。 |
| F57-A06 | 客户定制必须以版本化、可签名、可测试、可回滚的能力包表达；不得形成客户专属内核分支。 |
| F57-A07 | AI 只能在明确授权包络内调用能力；AI 不得成为数据或业务规则权威。 |
| F57-A08 | 高风险动作必须执行重新认证、职责分离和显式审批；低风险动作可在授权包络内自动执行。 |
| F57-A09 | 系统必须能在断电、重启、网络中断、插件崩溃和长期等待后，从持久检查点继续。 |
| F57-A10 | 所有持久化客户数据及衍生数据必须进入 HDD；SSD 仅承载系统、程序、可重建静态文件，以及不含客户/业务 authority 字节、受签名清单和独立大小/保留/镜像/重建策略约束的 exact 四类运行控制/代码状态。 |
| F57-A11 | 安全性和恢复能力必须按部署证据与演练结果诚实展示，不得因存在设计文档而宣称已实现。 |
| F57-A12 | 单一客户数据库内的多个法人必须以法人标识、行级安全和独立密钥域隔离。 |
| F57-A13 | 系统必须提供开放、版本化、可验证的完整导出，不得以厂商密钥、格式或许可证锁住客户数据。 |
| F57-A14 | 首版生产数据、备份、日志、索引、诊断和可关联客户的衍生数据必须在中国大陆境内处理和持久化；地点或证据未知即失败关闭。 |
| F57-A15 | 客户端分发、终端数据保护和浏览器能力差异必须按真实载体取证并诚实披露；无法强制的控制不得宣称为强制。 |

## 3. 双端 C/S 架构与权威边界

### 3.1 两种 UI/UX

系统提供两种逻辑界面，但只有一个权威节点：

```text
Windows Server 2022 权威节点
├─ 服务器控制中心（设计、审批、签名、发布、恢复）
├─ Rust 权威内核与模块化业务主体
├─ PostgreSQL 16 权威数据库
├─ 自动化、插件、MCP、连接器和后台任务
└─ HDD 权威数据
          │
          │ 签名配置代、任务、裁剪数据、强类型命令
          ▼
Windows / macOS / iOS / Android Workbench
```

服务器控制中心属于权威节点的一部分。客户配置/模型的受限编译、影响预览、审批、验签、generation 激活和回滚只由客户 Windows Server 权威端控制；需要签发时调用客户批准的独立 PIV/HSM/TPM signer role，不把可导出私钥交给应用进程。厂商二进制构建与发行签名属于 §3.1.1 的独立故障域，不在生产 P340 上执行。管理员可以在服务器本机使用，也可以从受信任管理网络打开远程界面；远程界面没有独立权威。

Workbench 是办公端，只负责显示、输入、设备能力、最小离线缓存和业务意图提交。

### 3.1.1 生产权威、构建与签名故障域

“控制中心、配置编译、审批和激活位于客户 Windows Server 权威端”不表示生产服务器兼任通用源码构建、依赖下载、CI 或厂商发行签名机。故障域固定如下：

- 生产 Authority 只运行权威事务、策略裁决、受控客户配置编译、签名制品验证与激活；不得作为通用 CI/build runner；
- 厂商二进制构建、依赖获取、可复现构建和发行签名在独立、受控的 Windows 构建/签名环境完成，生产端只接收 digest 已固定的制品、SBOM、签名链和验签材料；
- 客户模型、流程和 UI schema 可以在 Authority 上的受限 compiler worker 编译，但该 worker 无生产表直写、无任意脚本、无任意网络，只能输出待审批 generation；
- publisher/production signer 私钥必须位于独立 PIV/HSM/TPM role，core、plugin、compiler、CI 和远程支持均不得取得可导出私钥；
- 生产 P340 只参与本机安装、容量、UPS、存储和恢复等实机证据，不承担全局 CI 聚合。

### 3.2 Windows-only 服务器基线

权威节点必须以原生 Windows 服务运行，核心不得依赖 Linux、WSL、Kubernetes、厂商 SaaS 控制平面或第二台应用服务器。Linux 专用 AI/OCR/工业组件只能作为客户可选的外部提供者连接，不能成为核心运行条件。

产品架构和安装包格式必须保留未来接入客户控制 IaaS Windows Server 的独立扩展缝，但当前首版生产 profile 只接受物理 P340 的 `SINGLE_DISK_DEGRADED_PRODUCTION`；当前选择 `IAAS_WINDOWS_SERVER_HDD_STRICT` 必须以 `PROFILE_NOT_IMPLEMENTED` / `STORAGE_MEDIA_UNVERIFIED` 失败关闭。未来只有新 graph/profile version 能启用云 carrier，且客户仍须控制操作系统、数据库、数据卷、密钥和更新；云提供者还必须可审计地证明权威卷底层 HDD 介质、缓存和快照边界符合策略。普通 IaaS 云盘不得存放本项目要求物理 HDD 的正式生产数据，P340 证据也不得复用。

Windows Server 2022 是当前首发认证基线，不是永久平台。Microsoft 生命周期页列出的 PT 日期为主流支持截至 2026-10-13、扩展支持截至 2031-10-14。第一阶段必须交付 OS adapter seam、2022 安装/恢复证据、后继 LTSC 探针和签名迁移 playbook；Windows Server 2025 或当时后继 LTSC 的真实安装、原地/并行迁移、驱动、BitLocker/TPM、数据库恢复、服务账号、回滚和业务一致性认证作为明确阶段边界 `DEF-010` 激活。主流支持结束后签发新生产证书必须附补丁来源、支持策略、客户风险接受和已排期迁移认证；扩展支持结束前必须完成迁移。迁移证据未通过不得擅自变更生产基线（稳定需求 `NFR-016`）。

### 3.3 客户端权威规则

| ID | 规则 |
|---|---|
| F57-C01 | 客户端不得持有数据库凭据，也不得把本地缓存解释为权威状态。 |
| F57-C02 | 客户端必须报告 desired generation 与 observed generation；高风险能力在版本不兼容时失败关闭。 |
| F57-C03 | 页面、字段、动作和显示条件来自签名 UI schema；安全规则仍由服务器执行，不能只靠隐藏控件。 |
| F57-C04 | 四平台保证业务结果、权限和审计一致；交互布局按桌面和移动设备自适应，不要求像素一致。 |
| F57-C05 | Control Studio 的旧名称只作为历史术语；现行产品名称为“服务器控制中心”。 |
| F57-C06 | 四端包必须绑定客户控制的签名/分发资料、包 digest、最低安全版本和撤销状态；未签名、被撤销或低于最低版本的客户端不得提交业务命令。 |
| F57-C07 | Workbench 在线只使用版本化员工 HTTPS API；macOS/iOS/Android 不得使用 Windows 内部 IPC，所有端不得各自发明业务协议。 |

## 4. 五层架构与进程边界

### 4.1 五层结构

```text
L4 交互层        Workbench / 服务器控制中心 / 门户
L3 业务能力包    CRM / 合同 / 销售 / 采购 / 财务 / 售后 / 自定义
L2 隔离扩展层    WASM / Windows worker / MCP / AI / OCR / 受控容器
L1 基础适配层    PostgreSQL / 文件 / 身份 / Excel / 签章 / 备份 / 密钥
L0 可信内核      事务 / 授权 / 审计 / 配置代 / 自动化 / 签名 / 迁移
```

### 4.2 L0 可信内核

可信内核只包含必须统一保证的机制：事务、幂等、权限裁决、审计、配置代、能力注册、耐久任务、签名验证、迁移门禁、恢复和资源降级。内核不得包含客户行业流程或厂商特定连接器。

内核不接受运行时原生 DLL 注入。内核升级只能通过签名安装包、维护窗口、备份检查、原子替换和失败回滚完成。

### 4.3 L1 基础适配

数据库、文件、搜索、身份、邮件、通知、电子签章、Excel、OCR、备份、密钥和监控以稳定能力契约连接。第一阶段唯一必须认证的权威数据库提供者是 PostgreSQL 16；SQL Server、Oracle、MySQL、ERP 或生产数据库只能作为外部数据源或命令提供者。稳定的 `AuthoritativeDatabaseProvider` seam 必须保留，但在逐 provider conformance 完成前不得把其他数据库配置为权威库（稳定需求 `DBP-001`）。

### 4.4 L2 隔离扩展

优先运行形式为 WASM 和受 Windows Job Object 约束的签名独立 worker。受控容器仍是稳定产品能力，但首条纵向闭环只冻结 Container Carrier ABI、签名 manifest、权限声明、能力探测、审计事件和确定性错误 `HOST_CAPABILITY_UNAVAILABLE`；不得以在当前 P340 上实现 HCS/Hyper-V 执行器作为纵向闭环前置条件。具体 HCS/Hyper-V carrier 只能在目标主机的 Windows feature、虚拟化或 nesting、内存、磁盘、补丁、容量和隔离证据全部通过后，作为独立增量实现并认证。请求不可用 carrier 时必须显式拒绝，禁止静默降级到 WASM、进程内或权限更宽的 worker。P340 32GB 档默认禁用容器，未认证 IaaS carrier 也保持不可激活；WASM 与受控 worker 仍可使用。扩展必须声明数据、字段、能力、文件、网络、密钥、CPU、内存、时限、依赖、测试和回滚。默认全部拒绝，批准后按最小权限授予（稳定需求 `PKG-003`）。

### 4.5 L3 业务能力包

能力包可以包含对象、字段、关系、命令、事件、状态机、自动化、权限、页面、报表、模板、连接器声明、迁移和测试。每个对象只有一个 owning package；跨包写入只能通过公开能力。

### 4.6 L4 交互层

交互层只解释签名 UI schema 并调用能力。门户必须通过独立网关暴露裁剪投影，不能公开核心服务或数据库。

### 4.7 进程拓扑不是产品契约

产品冻结的是信任边界和能力契约，不冻结“恰好九个进程”。当前硬件的最小部署可以把可信业务模块装配为模块化主体，把插件、门户、集成、备份等信任或资源边界拆成独立 Windows 服务。未来调整进程数量不得改变业务契约、数据权威或审计语义。

首版逻辑运行角色固定如下；实现可以在不降低隔离的前提下合并同信任等级角色，但不得把不同信任等级角色共宿：

| 逻辑角色 | 唯一责任 | 禁止事项 |
|---|---|---|
| Authority Host | 唯一业务命令入口、模块化业务主体、动态授权、耐久自动化和当前状态写入 | 不承载不受信插件或公网入口 |
| PostgreSQL 16 | 唯一权威数据存储与事务提交 | 不接受客户端、门户、插件、MCP、Excel 或外部 provider 直连 |
| Portal Gateway | 门户认证、裁剪查询和白名单命令转交 | 零业务 SQL、零 KMS、零权威文件目录访问 |
| Integration Gateway | 已批准网络出站、provider/MCP 协议转换和回执转交 | 零业务 SQL、零 KMS、零任意文件、零直接状态变更 |
| Extension Host | WASM、签名 Windows worker 与已认证 container carrier 的隔离执行 | 不拥有业务事实，不持有通用数据库凭据 |
| Backup Writer | 接收受控备份流并写入服务器外追加目标 | 仅可使用专用 backup/replication 权限；不得读取或修改普通业务表，不得删除历史备份 |
| Ops Agent | UPS、卷、服务、时间和硬件健康控制 | 不持有业务数据库写权限，不执行领域命令 |

Integration Gateway 的写入输入必须转换为版本化 typed command 或 proposal，再由 Authority 重新验证身份、法人、行/字段权限、策略、配置代和幂等键。其配置、依赖图或运行拓扑若声明数据库连接串、数据库角色、连接池、迁移器、KMS 解封或权威目录访问，安装、启动和 generation 激活必须失败关闭。

每个可激活 generation 在其签名 generation envelope 已经生成后，必须确定性构造并按内容摘要存储一份不可变的 strict plain JCS `RuntimeTopologyDeclarationV1`；它不是 signed envelope，也不增加 signer-registry row。declaration 绑定 deployment、authority epoch、完整已签名 generation envelope digest、capability-graph digest、hardware/storage/workload profile、storage manifest、capacity policy definition，以及每个参与者的服务身份、二进制摘要、carrier、Windows Service SID、IPC/ACL、依赖、readiness、资源等级、有界队列和允许的持久化类别；同时冻结数据库连接消费者 exact-set、角色、用途、权限类别、常驻/峰值上限、超时和预算权重。它不得提前携带 candidate、P340 soak 或 capacity-certificate ref。实际二进制、服务身份、端点、ACL、连接消费者、池上限、参与者集合或硬件能力与 declaration 不一致时不得 ready。只有同一候选的 P340 终态 PASS 后，G6 才可构造 strict plain JCS `RuntimeTopologyCertificationV1`，把该 declaration 与 candidate、P340 soak、capacity certificate、宿主指纹及三个 profile exact-bind；发布证书再绑定 certification，生产启用还必须重验现场读回。拓扑、硬件、连接预算或 carrier 能力改变必须产生新 generation/declaration 并重做适用的 P340/certification，禁止用环境变量或命令行静默扩权。

F-57 生产运行时只能采用原生 Windows Service、Service SID、命名管道、Job Object、DACL 和 Windows 证书存储。活动生产路径不得依赖 Linux、WSL、systemd、cgroup、Podman、Compose、Kubernetes 或 `/run`、`/etc`、`/var/lib` 等 Linux 路径；历史 Linux 资产只能作为不可发布研究夹具。Windows 构建、安装、升级、服务/IPC ACL、启动、停止和恢复证据是发布门，Linux 构建成功不能替代。

### 4.8 逻辑分层不等于物理 crate 分层

业务代码采用 feature-first：每个稳定 bounded context 优先使用一个业务 crate，并在内部组织 `contract`、`domain`、`application` 和测试模块。只有满足下列至少一项时，才允许抽成独立 crate：

1. 被两个以上 bounded context 作为稳定公共 IDL 复用；
2. 需要独立安全审计、编译目标或载体；
3. 需要独立发布、兼容承诺或第三方 SDK；
4. 已有非骨架实现且独立边界能显著缩短构建或验证范围。

不得为尚未实现的业务域预创建成组三层空 crate，也不得把逻辑层数映射成进程数。现有成熟的平台 crate 可以保留；现有空业务骨架在写入第一条业务实现前必须按 bounded context 合并或给出保留证据。跨域依赖仍只能指向公开 contract，不能因为物理合并而共享表直写。

本节的物理边界、渐进迁移与影响范围由已接受的 [ADR-0025](../../adr/ADR-0025-f57-capability-graph-and-feature-first-boundaries.md) 登记；任何 crate 重排都必须由收敛实施计划中的测试先行任务授权并通过架构门。

### 4.9 单一能力图与生成投影

系统必须只有一个可签名、可版本化的业务能力语义源 `CapabilityGraphV1`。每个能力节点由唯一 `CapabilityId + version + owning_feature` 标识，并至少绑定 command/query/fact、输入/结果/错误 schema、状态与不变量、授权 scope/conditions/risk/SoD、数据 owner、Objective/closure、允许 carrier、生命周期和证据要求。

复杂语义不得因为不适合塞进一行 graph struct 就退化成第二真值。G0 先建立通用语义合约 wire/compiler/projection；以后每个 owning task 只可读取现行契约中以唯一 BEGIN/END marker 和 digest 明确登记的机器表锚点，并通过 graph-owned 的 exact source-header/index/codec、contract-kind validator 与独立 normalization golden，把状态守卫/不变量、授权 scope/condition/CandidateQuery、Objective trigger/reopen/closure、责任能力、effect intent、timeout、termination和 compensation 转换成排序、强类型的 `SemanticContractRowV1`/`SemanticContractFieldV1`。策略参数、状态定义和 workflow graph 等嵌套结构只能使用 schema-bound `CANONICAL_JCS_OBJECT`，不得降级成普通 UTF8。唯一例外是业务合同 §8.4 的 workflow：它以固定 authoring rule 直接作为 `GRAPH_NATIVE + WORKFLOW_DEFINITION_REGISTRY` 行进入图，禁止另建 Markdown workflow 表。每份绑定都携带 typed provenance、投影路径/摘要、可在 graph root exact-resolve 的 row schema、exact row count、owner 和 projection targets。Graph digest 因而直接覆盖语义行；strict `SemanticContractProjectionV1` JSON 和 `SemanticContractsManifestV1` primary 都从已编译绑定生成，不是编译所需的先决输入。Graph 内的 lifecycle、authorization 和 objective 字段只是对这些绑定行的强类型索引，必须逐字段 exact-join，不能作为另一份可独立修改的摘要；Objective definition、trigger/execution/compensation 用冻结的 15-kind exact row-key set，timeout/termination 用 trigger 所引用的 dedicated policy-ID exact set，workflow 用全局 workflow-ID row key并按 definition.objective_kind 覆盖同一 15-kind set，三者不得混为“都是 15 行”。运行时 registry、OpenAPI、客户端、UI、MCP、Excel 和测试只能从同一已编译绑定生成；契约表、绑定、投影任一不等即整代拒绝，历史 seed 永久不回写。

以下内容只能是同一能力图的确定性投影，不得继续作为可独立手改的第二真值：

- OpenAPI、Rust/TypeScript DTO 和客户端协议；
- UI schema、页面 action、字段与查询用途；
- 权限目录、风险、职责分离和责任候选；
- MCP tool、Excel action、provider capability 与资源申请；
- package manifest、状态域、事件、错误和测试 manifest；
- 供人阅读的登记表、追踪表和文档视图。

`MODULE_PACKAGE` 只表达许可投影，`CAPABILITY_PACKAGE` 是能力图子图的版本化载体，`ProviderManifestV1` 只声明某能力的 carrier、外部端点和资源上限；三者不得重复定义业务语义。一个能力只能有一个事实写 owner；跨能力只能调用登记 command、读取授权 projection 或订阅 committed fact。

能力图存在未知引用、重复 owner、依赖环、缺失 schema、投影不一致或 digest 漂移时，编译和 generation 激活必须失败。所有生成物必须携带相同 graph digest 和 generator version。当前 API discriminator、component shape、component/state binding、state domain 和 direct-route 五类种子在新编译器完成首次确定性导入前仍是受控输入；导入验收后必须降为不可变 `HISTORICAL_IMPORT_SNAPSHOT`，仅供导入审计。现行 API 只由 CapabilityGraph 在 `docs/generated/f57/` 生成，禁止人工双写或回写 seed。Requirement ownership、旧迁移处置、FreshPG 和 CI profile 仍属于交付/证据登记，不塞入产品能力图。

## 5. 配置代、能力包与热插拔

### 5.1 签名配置代

一个发布代必须冻结以下内容的相容组合：

- 数据对象和数据库迁移；
- 权限策略与职责分离；
- 工作流、自动化和闭环条件；
- UI schema、报表和模板；
- AI、MCP、Excel、连接器和插件版本；
- 能力契约、依赖和回滚策略。

唯一发布过程为：

```text
草稿 → 编译检查 → 模拟 → 审批 → 签名 → 预下载
     → 原子启用 → 观察 → 保留或回滚
```

`GenerationState` 的 wire/SQL 闭集精确为 `DRAFT|COMPILED|SIMULATED|APPROVED|SIGNED|PREDOWNLOADED|ACTIVATING|OBSERVED|ROLLING_BACK|ROLLED_BACK|REJECTED`，允许边只有：`DRAFT→COMPILED|REJECTED`、`COMPILED→SIMULATED|REJECTED`、`SIMULATED→APPROVED|REJECTED`、`APPROVED→SIGNED|REJECTED`、`SIGNED→PREDOWNLOADED|REJECTED`、`PREDOWNLOADED→ACTIVATING|REJECTED`、`ACTIVATING→OBSERVED|ROLLING_BACK`、`OBSERVED→ROLLING_BACK`、`ROLLING_BACK→ROLLED_BACK`。`REJECTED` 与 `ROLLED_BACK` 是不可恢复终态；OBSERVED 表示该代已完整成功观察，是否为当前权威另由 observed pointer 决定，保留不产生新状态。未列边一律拒绝，修订必须用新 generation，不能把终态改回草稿。

激活期间 desired pointer 可指向目标，但 observed pointer 在全部 item、schema、package、workflow、UI/client compatibility 和 post-activation probe 成功前始终指向上一 OBSERVED 代。重启时若 journal、实际读取和 digest 证明目标全部精确应用且 probe 通过，则唯一收敛为 OBSERVED；否则唯一收敛为 ROLLING_BACK，并持续按已签 reverse plan 恢复上一 observed pointer，成功后目标代进入 ROLLED_BACK。ROLLING_BACK 中再次崩溃只可续跑同一 reverse plan；失败/Unknown 时保持 ROLLING_BACK、禁止新权威写并升级事故，不能把混合状态标成 OBSERVED/ROLLED_BACK。一次已 OBSERVED 的代后来因 canary/事故回滚时也走 `OBSERVED→ROLLING_BACK→ROLLED_BACK`；上一代记录保持 OBSERVED 并重新成为 observed pointer。

每次权威命令必须记录配置代、权限策略版本、工作流版本、能力包版本、客户端版本和幂等标识（稳定需求 `GOV-008`）。缺失任一适用版本的命令不得进入权威提交。

每个已签名 generation 必须绑定 `CapabilityGraphV1`、projection manifest、storage manifest、capacity policy definition、所有 item/reverse-plan digest 与 required-participant exact-set。随后构造的 `RuntimeTopologyDeclarationV1` 反向绑定该完整已签名 generation envelope digest，不得让 generation 反向引用 declaration 形成摘要环，也不得在此阶段绑定尚未产生的 capacity certificate；该证书只能在 P340 终态后由 `RuntimeTopologyCertificationV1` 绑定。所有需要独立签名的业务制品统一使用 `SignedBusinessArtifactV1<T>` 外层协议；其 wire exact 复用 F-56 §2.1 已冻结的四字段 `payload|payload_sha256|signer_subject|signature_cms_b64url` detached-CMS 封套，purpose 必须是 payload `T` 内的强类型字段并由 typed expectation 验证，不得新增第二种外层封套。payload 不得再次包含自由格式 `signature`、`signing_key_id`、算法或证书链字段。迁移计划的唯一类型是 `SignedBusinessArtifactV1<MigrationPlanPayloadV1>`，并且必须作为同一 generation 的具名 item 被 digest 引用；单独持有迁移计划签名不能激活 DDL。

### 5.2 三种可插拔等级

| 等级 | 适用对象 | 切换语义 |
|---|---|---|
| 原子代切换 | 页面、报表、权限、规则、流程新版本、MCP 配置 | 新请求使用新代；事务内不得跨代 |
| 排空后替换 | WASM、Windows worker、满足 `HOST_CAPABILITY_CONDITIONAL` 证据门的 Hyper-V-isolated Windows container、连接器、AI/OCR 提供者 | 停止新任务、完成或安全转移旧任务、健康验证后切换；P340 32GB 与未认证 carrier 的 container activation 默认拒绝 |
| 维护升级 | Rust 内核、数据库引擎、加密和存储底座 | 签名升级、维护窗口、备份验证、失败回滚 |

“可插拔”表示具有受治理的替换路径，不表示所有底层组件都允许运行中强制拔除。

### 5.3 包信任与许可

- 厂商标准包由厂商信任根签名；
- 客户自建包由客户信任根签名；
- 第三方包须由客户明确批准其证书和权限；
- 包必须携带 SBOM、权限清单、依赖、资源限制、受控迁移、测试和回滚信息；迁移只能由可信迁移器执行，不得把任意 SQL 或脚本交给扩展运行（稳定需求 `PKG-002`）；
- 模块可以按许可证安装、启用、停用和升级；停用始终保留历史数据和证据；
- 厂商升级不得覆盖客户扩展包；客户包不得修改可信内核。

### 5.4 运行中版本

新运行实例使用新版本；已运行实例固定原版本。紧急变更只能明确选择继续、补偿后停止或重新启动，禁止静默迁移。

Authority 是唯一 generation 协调者，并持有持久激活 journal 与全局 activation mutex。已签名 `GenerationManifestV1` 中的 required participants 必须从同一 compiled graph 确定性派生，并与 `RuntimeTopologyDeclarationV1` 的 ACTIVE participant exact-set 一一对应；每个参与者必须分别验证签名、依赖和本地实际状态，完成适用的原子切换、排空替换或维护升级，并对完全相同的 generation envelope digest 写入持久 `GenerationParticipantV1` ACK。只有 required participant exact-set 全部 ACK 后才能推进 observed pointer；失败、超时、摘要不一致或重启丢失状态时只能保持上一 observed generation 或执行完整回滚，禁止部分激活。

每个在途命令、工作流、外部 effect 和可恢复任务必须持有 `ArtifactPinLeaseV1`，记录 execution、participant、generation、package/version digest、取得/续租/释放状态。短调用可使用有界租约；持久流程与 `Unknown` 外部效果还必须保存不可因租约超时而消失的持久引用。只有同时满足以下条件才允许归档或回收旧包、规则、schema 和 provider：

1. 不存在活动租约或持久引用；
2. 不存在 `Unknown` effect 或待对账任务；
3. 回滚窗口已经关闭；
4. 不受备份、审计、诉讼保全或法务保留约束。

停用先禁止新调用，再排空既有调用；历史数据、附件、审计、恢复材料和读取证据必须继续安全保留。进程崩溃、参与者重启、ACK 丢失或租约超时均不得单独成为删除旧制品的依据。

## 6. 数据架构与可定制数据库

### 6.1 四个数据区域

| 区域 | 内容 | 修改方式 |
|---|---|---|
| 可信核心区 | 身份、授权、审计、配置代、自动化、密钥引用、许可 | 仅可信内核和受控迁移 |
| 标准业务区 | 客户、合同、订单、采购、财务、服务等 | 标准能力包；允许安全扩展 |
| 客户扩展区 | 自定义对象、字段、关系、规则、页面和报表 | 客户签名能力包 |
| 外部连接区 | 外部数据库、ERP、生产、银行、税务等 | 只读、同步、事件或受控命令 |

### 6.2 关系结构编译器

客户定义的高价值对象必须编译为真实 PostgreSQL 表、列、外键、约束和索引。通用 EAV 或单一 JSON 大表不得作为业务主存；JSON 只用于低结构、稀疏、插件私有且不参与关键一致性的载荷。

结构发布必须经过命名、类型、基数、索引、权限、保留期、数据量、迁移和回滚检查。破坏性变更采用“新增结构 → 回填 → 核对 → 切换 → 延迟处置”，禁止直接 DROP 生产数据。

自定义物理模型是 deployment-scoped，不是 legal-entity-scoped。`CustomerModelSpecV1` 必须包含 deployment、model、version、owning package、objects 和 relations；`ObjectCode` 在一个 deployment 的当前和全部历史 generation 中全局唯一，并确定性映射到 `ext.<object_code>`。code、object id 和物理表名退役后永久保留，禁止重用。两个法人对同一 code 提交不同物理形状必须返回确定性 namespace conflict，不能合并、覆盖或最后写胜出。

法人差异使用独立的 `CustomerModelBindingV1` 表达，只能缩小 enabled objects/fields、validation profile、UI schema 和 permission profile，不能改变列类型、关系、索引、约束或物理表名，也不能直接产生 DDL。每条 `ext` 记录必须携带 `legal_entity_id`、`security_level`、scope tags、row version 和审计列，并具有 `UNIQUE(legal_entity_id,id)`、FORCE RLS 与同法人复合 FK。自定义对象引用标准核心对象时只能使用稳定、登记的 public object reference；禁止对内核私有表建立临时 FK 或共享表写入。

每个自定义对象必须编译成不可分割的 `CompiledObjectSecurityBundleV1`，同时包含：物理身份和 owner；公共列、类型、关系、基数、删除规则和约束；RLS 与跨法人负例；capability、scope、字段权限和查询用途；敏感字段、加密、blind index、密钥 selector 和数据分类；审计、事实/Outbox、生命周期、保留、legal hold、处置 tombstone、备份、恢复和完整导入导出；允许的 API/UI/报表/Excel/MCP/plugin 投影；索引、HDD、锁、回填、回滚和恢复 checkpoint 影响；生成物 digest 与兼容代。任一组成缺失、重复、越 owner、跨法人、资源超限或 digest 不一致时，整个对象不得进入 observed generation。

编译器只能接受受约束模型，不接受客户 SQL、触发器、函数、任意表达式或自选物理名称。DDL 可以由迁移 journal 分阶段执行，但 bundle 完整验证并原子切换 observed generation 前，不得暴露路由、grant、查询、自动化或 UI。第一条纵向闭环只允许新增对象/字段/索引、扩大兼容类型和声明废弃；DROP、物理重命名、类型收窄及长锁迁移必须后移到具备双版本读取、回填核对、恢复 checkpoint 和维护证据的档位。每个 deployment 必须对自定义表、列、索引、行宽、预计行数、迁移锁时和 HDD 增长设置签名配额。

### 6.3 当前状态与不可变事实

系统采用混合模型：关系表保存当前状态，不可变事实保存发生过的业务变化。它不是全量事件溯源。

同一 PostgreSQL 事务必须原子写入（稳定需求 `GOV-009`）：

1. 当前业务状态；
2. 不可变业务事实；
3. 审计记录；
4. 待发送 Outbox 事件。

财务、审批和关键履约纠错必须追加冲销、补偿或更正事实，不得覆盖历史。附件必须版本化并保存校验值、来源、密级和关联证据。

### 6.4 多法人

每条业务记录必须具备法人、数据分类、权限作用域和密钥域信息。PostgreSQL RLS 负责法人和硬边界，业务授权引擎负责项目、客户、记录、字段、金额、状态和条件。跨法人访问必须显式授权。

每次业务查询和写入都必须执行 `BEGIN → 建立事务级 SecurityContext → 数据库侧验证并 readback → 授权与业务 SQL → COMMIT/ROLLBACK`。repository 只能接收已经完成上述证明的 `AuthorizedPgTx`，不得接收裸 pool connection；禁止业务路径使用会话级 `set_config(..., false)`。事务上下文必须由应用数据库角色不可伪造的 `SecurityContextEnvelopeV1` 或等价数据库侧验证机制建立，再投影为 transaction-local context；仅调用 `SET LOCAL` 并自行 readback 不能单独满足最高安全档。设置、验证、readback、rollback 或归还清理任一步失败，连接必须销毁而不是回池；迁移、恢复和运维身份不得复用业务连接池。

FORCE RLS 是纵深防御：它必须阻断漏写法人谓词、普通 repository 缺陷和连接池污染，但不得被描述为已经抵御“应用数据库角色获得任意 SQL 执行能力”。后者仍可能读取当前法人中超出业务授权的数据，因此还必须通过参数化/静态 SQL、禁止运行时动态 SQL、最小表/函数权限、独立迁移身份、服务进程隔离、字段查询授权器、SQL 注入负例和审计降低风险。最高安全档的认证材料必须明确这一残余信任边界，不能用 RLS 名称代替证明。

### 6.5 外部系统一致性

禁止跨两个数据库伪装成单一 ACID 事务。外部调用必须保存 effect 记录、幂等键、请求、响应、业务关联号和对账状态；结果未知时先查询确认，再决定重试或补偿。

## 7. 动态能力权限

### 7.1 授权结构

一项授权由以下维度组成：

```text
principal × capability × resource scope × action
          × conditions × validity × delegation
```

principal 可以是用户、组、团队、项目、部门、服务、AI、插件、客户或供应商。scope 可以精确到法人、客户、项目、对象、记录、字段和附件。conditions 可以包含金额、状态、数据分类、设备、网络、时间、本人关系、职责分离和审批结果。

所有通用主体引用必须使用 `PrincipalRefV1 { kind: PrincipalKindV1, id: UUID }`；`GrantEnvelope`、delegation、`CandidateQuery`、assignment、审批、会话投影、AI/plugin/provider 调用和审计均不得使用裸 `principal_id`。只有 `actor_user_id` 等字段在 schema 中被明确限定为单一实体类型时才可使用专用 UUID。`PrincipalKindV1` 只能取签名 exact registry 中的闭集；同一 UUID 出现在不同 kind 中不得相等、合并或继承权限。授权快照、幂等键、唯一键和 decision explanation 必须包含 kind。

### 7.2 角色与岗位

旧 RoleCode、岗位和角色包只作为授权模板及业务标签，不再是唯一授权边界。任务声明所需能力，不声明固定岗位；分配器根据能力、作用域、负载、在线状态、SLA、职责分离、回避和委托选择执行者。

### 7.3 裁决顺序

1. 内核安全底线；
2. 法人隔离、数据分类和职责分离强制拒绝；
3. 明确作用域拒绝；
4. 满足条件的作用域允许；
5. 默认拒绝。

数据必须在服务器查询阶段裁剪、过滤或脱敏，不得先传给客户端再隐藏。

字段返回可见性 `HIDDEN|MASKED|READ|WRITE` 与查询用途必须正交授权。`allowed_query_uses` 的闭集为 `FILTER|SORT|GROUP|AGGREGATE|SEARCH|EXPORT`，默认空集；`HIDDEN` 强制空集，`MASKED` 只允许返回批准的 mask，`WRITE` 不自动获得查询或导出用途。查询编译器在生成 SQL 前必须同时裁决行范围、返回可见性和每个字段的用途；未授权字段不得进入 predicate、join、sort、group、aggregate、全文/blind-index search、facet、报表参数、Excel 筛选、MCP 参数或插件查询。

`COUNT_ROWS` 与 `EXISTS` 使用独立对象级 capability，并且只能作用于已经完成行级授权的集合。字段聚合同时要求对象聚合能力和字段 `AGGREGATE`。游标必须绑定 principal、device、legal entity、generation、query type 和 authorization digest。无权对象、无权字段、字段不存在、唯一冲突和被裁剪结果必须使用稳定、不可枚举的错误，不得返回隐藏字段名、冲突值、隐藏总数、facet、最小/最大值或可推断权限差异的分页水位。Control Center、Employee API、portal、reporting、Excel、MCP、AI、插件、打印和导出必须复用同一服务端查询授权器。

### 7.4 管理、委托与模拟

获得授权的客户管理员可以建立模板、临时授权、委托和自动失效规则，但不得授予超过自身 delegation ceiling 的能力。高风险授权变更要求提交人与审批人分离。发布前必须展示新增/失去访问的人、受影响数据样本和职责分离冲突。

高风险目录采用 exact-set，不允许模块自行降级：主数据合并；合同签署、签章、生效、重大变更和终止；价格/信用越权；订单取消；供应商准入、采购单发出和收货例外；库存调整；开票/红冲；付款、退款和银行账户；经营分录更正、期间锁定和迟到事实顺延例外；敏感导出；授权、委托和破窗；配置代、能力包、schema 和迁移；法律保留释放和数据处置；信任根、许可、provider 和批准时间源变更；密钥、备份、恢复、远程支持授权和 authority 提升。新增类别必须先更新 `AUTH-007`、风险级别、重新认证、职责分离和测试登记。

## 8. 耐久自动化与闭环引擎

### 8.1 一等对象

自动化内核至少包含以下稳定概念：

- `automation_definition` 与不可变 `automation_version`；
- `automation_run` 与 `cycle`；
- `objective`、`obligation`、`closure_predicate`；
- `work_item`、`step_attempt`、`effect`、`evidence`；
- `timer`、`lease`、`incident`、`compensation`；
- 版本、配置代、权限快照引用和关联业务事实。

### 8.2 运行循环

```text
观察事实 → 计算未完成责任 → 分配 → 执行
        → 验证效果 → 收集证据 → 判断闭环
        → 完成 / 等待 / 重试 / 补偿 / 改派 / 升级
```

闭环条件必须是可计算且有证据的条件，不得只依赖人工把状态改为“已完成”。退货、退款、付款撤回、投诉或责任变化可以重新打开相关闭环。

### 8.3 耐久与失败语义

- 每一步前后写持久检查点；
- 内部命令使用数据库事务；
- 外部投递采用至少一次语义加幂等效果，不宣传无法证明的“外部绝对一次”；
- 网络超时但结果未知时进入 `RECONCILING`，禁止盲目重试；
- 临时故障退避重试，业务拒绝进入异常任务，数据矛盾进入隔离事故；
- 人员离职或权限失效时重新解析执行者，不扩大新执行者权限；
- 循环必须声明退出条件、检查周期、重试上限和升级路径；
- AI 或插件不可用不得阻塞确定性业务主链。

### 8.4 聚焦型实现

第一版使用 Rust 与 PostgreSQL 实现聚焦型耐久内核，不引入需要独立集群的完整外部工作流平台。实现必须保留稳定契约，使未来替换调度器或增加暖备时不改变业务包。

## 9. AI、MCP、Excel 与外部能力

### 9.1 AI

第一版保留 AI provider、模型、提示、工具和授权接口，但不把本地模型交付作为开发前置条件。外部 AI 默认关闭；高保密数据禁止外发。AI 只能调用强类型能力，所有模型、提示、工具、输入范围、输出、审批和结果必须审计。

合同生效、付款、退款、最终审批、权限、配置、密钥、备份和破坏性动作必须人工批准。AI 失败时系统仍能执行确定性业务流程。

### 9.2 MCP

MCP 是受控工具连接层，不是核心事务总线。MCP 服务必须登记工具、对象、字段、网络、文件、密钥、资源、数据等级、超时和停用规则。每次调用使用最小权限短期凭据并写入审计。

### 9.3 Excel

Excel 是受控办公入口，不是数据库客户端。导入先形成变更提案，执行模板版本、逐行校验、权限、记录版本、业务规则和影响预览，再经正式能力提交。导出执行字段裁剪、脱敏、水印、审批和审计。禁止 VBA 或加载项直接访问数据库；Office 加载项只能作为可选签名连接器。

### 9.4 能力提供者

同一能力可以连接本地、自建、私有云或经批准的外部提供者。业务流程依赖能力契约，不写死厂商。连接器可被热停用，核心闭环必须能进入等待、替代提供者、人工处理或补偿，而不是丢失任务。`ProviderManifestV1`、`ResourceGrantV1`、canonical encoding 和四层权限交集以 [ADR-0023](../../adr/ADR-0023-f57-provider-manifest-resource-grant.md) 为准；任一权限层未知、过期或撤销即拒绝。

第一阶段必须完成认证的核心 provider 目标固定为本地文件、Excel/CSV/Word/PDF、REST/Webhook/MCP、SMTP 和 AD/LDAP；在对应证据通过前一律显示为未认证并保持关闭。OIDC/SAML 以及企业微信、钉钉、飞书、Microsoft 365、WPS、银行、税务和签章厂商均使用同一签名 provider/capability-package 契约，但必须逐 provider 取得 conformance 证据后才能启用；它们不是全部捆绑在核心安装包中的现成实现。

上述集合是 F-57 完整发布目标，不是首条业务闭环的串行前置。`CTC-01` 只需要本地文件/附件、Excel/CSV proposal、SMTP 或一个确定性模拟 effect provider，以及一个只读 REST/MCP conformance probe；MCP 写能力、Word/PDF 高级生成、AD/LDAP 生产认证和其他 provider 可沿依赖 DAG 在其首次使用档位补齐。未认证 provider 必须显示 `DISABLED_NOT_CERTIFIED`，禁止静默替代、扩大权限或伪装可用。

通用 XML/SOAP/XSD 不在第一阶段核心 provider exact-set（`DEF-011`）。行业或厂商确需 XML 时，只能作为签名 provider 的显式 codec 逐项认证，禁用 DTD、外部实体、XInclude 和网络取资源，并继续通过 ImportProposal 或类型化命令，不能形成通用写旁路。

## 10. Workbench、离线与门户

### 10.1 任务与异常中心

Workbench 首页以今日任务、未完成责任、等待、异常、决策和闭环为中心。客户、合同、订单和项目提供统一时间线，聚合相关责任、财务、采购、附件、售后和自动化证据。传统模块导航继续作为辅助入口。

### 10.2 跨平台技术门

第一候选为 Tauri 2、React 与 Rust 客户端能力层。正式建设业务 UI 前，必须在 Windows、macOS、iOS、Android 验证安全存储、离线数据库、文件、相机、扫码、通知、企业签名、安装、更新、撤销和大表单性能。分发必须使用客户控制的开发者主体、证书、MDM、离线仓库或托管商店；具体 carrier、签名、最小版本、撤销和 Web/PWA 合同回退以[客户端/生命周期/安全运营执行契约 §3](2026-08-23-f57-client-lifecycle-security-contract.md#3-四端签名分发更新和撤销)为准（稳定需求 `CLI-008`）。

若任一关键移动能力未通过，则整套客户端 UI 改用 Flutter 与 Rust 共享能力层。第一版不得同时维护两套分叉客户端技术体系。稳定产品契约是 UI schema 与 capability API，不是某个前端框架。

### 10.3 离线

离线只允许读取最小授权缓存、草拟表单和附件、记录现场工作和创建待提交业务意图。付款、退款、最终审批、合同生效、库存权威修改、权限和配置不得离线生效。

设备缓存必须使用每设备密钥、操作系统安全存储、加密、自动过期和远程撤销。默认不保存通用业务投影；只有签名策略逐对象/逐字段允许的 `MinimalOfflineProjectionV1` 可在有界、最迟 24 小时、可撤销的非权威缓存中用于离线读取。重新联网后服务器重新验证当前权限、配置代、记录版本和业务规则。冲突不得静默最后写胜出；高风险冲突必须人工处理。

### 10.4 门户

客户和供应商门户经独立 gateway 暴露字段投影和受控命令。门户不得访问数据库、密钥、文件目录或服务器管理面。门户可以内网、DMZ 或客户云部署，也可以整体热停用。

客户门户当前白名单为：查看报价、合同、订单、交付、发票、收款、设备、工单和批准文档；确认交付/验收；提交投诉、服务请求和服务证据。禁止付款、修改金额、合同生效、权限和配置动作。供应商门户当前白名单仍是采购单/交期确认、ASN、发票上传、对账查看和自有资料更新五项。未列入白名单的对象与命令一律拒绝。

### 10.5 员工协议、终端 DLP 与门户身份

Workbench 的会话、命令、查询、任务流、文件分片和 schema 路径统一使用 `CLI-009` 的员工 HTTPS API；客户门户账号的邀请、绑定、MFA、恢复、撤销、客户合并和关系终止按 `POR-003` 执行。受管原生端强制脱敏、水印、导出审批、打印/剪贴板/share 控制和受管文件失效；不合规设备降级为只读或拒绝高密级。浏览器门户强制服务端裁剪、水印、审批和审计，但打印、剪贴板、截图和下载后失效只能尽力控制，必须明示边界。完整矩阵以[客户端/生命周期/安全运营执行契约 §§1–3](2026-08-23-f57-client-lifecycle-security-contract.md)为准（稳定需求 `CLI-004`、`CLI-009`、`POR-003`、`SEC-015`）。

## 11. 业务功能基线

### 11.1 必须覆盖的能力域

| 域 | 核心范围 |
|---|---|
| 公共基础 | 多法人、组织、动态权限、任务、审批、通知、附件、搜索、审计、模板、导入导出 |
| CRM | 客户、联系人、商机、跟进、沟通、信用、风险、投诉与客户 360；只通过类型化命令消费报价结果 |
| CPQ | 报价、报价版本、价格审批，以及向合同或订单转换；独立拥有报价事实 |
| 合同 | 关键字段、条款、义务、收付款计划、附件版本、审批、签章、变更、续签、终止 |
| 销售订单 | 版本、拆分、合并、退换货、分批交付、签收验收；`STANDARD` 与 `DROP_SHIP` 是第一阶段必须完成认证的目标，寄售/订阅/租赁只保留 provider seam |
| 采购 | 需求、供应商、询比价、采购订单、交期、收货、验收、退货、应付和评价 |
| 项目/交付 | 项目、里程碑、任务、物料、交付、安装、验收、成本、风险 |
| 基础库存 | 仓库、占用、收货、发货、退货、批次/序列号与履约所需数量金额台账 |
| 经营财务 | 应收应付、发票、收付款、退款、核销、对账、账龄、现金流、经营总账与毛利 |
| 售后 | 投诉、安装、维修、巡检、维保、SLA、派单、配件工时、签字、根因与回访 |
| 管理 | 履约、采购、交付、现金、风险、异常、工作量、安全、备份和硬件驾驶舱 |
| 平台定制 | 对象、字段、关系、页面、流程、权限、报表、品牌、插件、连接器和门户 |

### 11.2 生产触发采购

采购需求来源是可插拔能力，可来自销售订单、合同、项目、库存规则、人工申请或外部生产系统。第一版不自研完整 MRP/MES；生产系统通过受控连接器创建标准采购需求。

### 11.3 专业系统边界

第一阶段原生提供履约闭环需要的基础库存和经营财务能力。法定总账、税务申报、工资、完整 MRP/MES、高级排产、大型 WMS 和自动化立库不作为第一阶段自研重点，可连接专业系统。F-50 的内部资金、发票、台账和一致性不变量继续适用于平台原生记录。

“经营财务”明确包含：不可变且平衡的内部经营分录、受控经营科目映射、试算、业务子账对账及经营期间永久锁定。经营期间一旦锁定就永不重开；迟到事实必须顺延记入下一开放经营期间，并保留原业务日期、顺延依据、关联原事实和更正链。它不宣称满足任一司法辖区的法定科目、法定凭证账簿、税务申报、工资或法定年结，也不冒充专业财税系统的法定会计期间（`RULING-FIN-PERIOD-01`，稳定需求 `FIN-011`、`FIN-013`）。

### 11.4 CTC-01 首条合同履约与回款纵向闭环

第一条可运行切片 exact 主链为：

```text
Customer
→ ContractVersion（关键字段、付款计划、已扫描附件）
→ SalesOrder（STANDARD，source=CONTRACT_VERSION）
→ ProcurementDemand（source=SALES_ORDER）
→ PurchaseOrder → GoodsReceipt → DeliveryEvidence
→ Invoice → CashReceiptAllocation
→ ObjectiveClosed（证据齐全）
```

`CTC-01` 只认证 `STANDARD`、合同来源订单和销售订单来源采购；`DROP_SHIP`、采购六来源完整矩阵、服务/项目和高级报表仍是 F-57 完整发布要求，但不得成为首条切片的前置。切片必须同时证明 typed command、CAS/幂等、两法人隔离负例、动态 grant 与执行中撤权、一个职责分离审批、当前状态+fact+audit+Outbox 同事务、一个签名 generation，以及 Objective/Obligation/Effect/Evidence/Unknown/reconcile/closure。合同附件仍必须进入 HDD quarantine，绑定字节 digest，并取得 malware-clean evidence 后才能发布，不能因文件平台后移而建立旁路。

CTC-01 的 `ObjectiveClosed` 只指客户侧主闭环：`CONTRACT_FULFILMENT`、`SALES_ORDER_FULFILMENT` 和 `RECEIVABLE_COLLECTION` 必须各自满足关闭谓词并进入 `CLOSED`。由于本切片 exact 主链没有采购发票、应付和供应商付款，`PROCUREMENT_FULFILMENT` 必须保持 `WAITING`，唯一阻断 obligation 为 `PURCHASE_AP_CLOSED`；类型化明细固定为 `ProcurementSettlementGapV1 { purchase_invoice_recorded: false, payable_recognized: false, supplier_payment_settled: false }`。不得把收货解释为采购结算闭合，也不得用自由文本、人工勾选或空证据令采购目标假关闭。G5 补齐采购发票→应付→供应商付款后，三个字段全部为 true，采购目标才可进入 closure review 并关闭。

销售订单在首次 RELEASE 前冻结 `sales_type`，RELEASE 后不得在 `STANDARD` 与 `DROP_SHIP` 间原位转换。一次 canonical release 必须且只能产生一个主履约族：`STANDARD` 只写 `SALES_ORDER_RELEASED` 并创建 `SALES_ORDER_FULFILMENT`；`DROP_SHIP` 只写 `DROP_SHIP_ORDER_RELEASED` 并创建 `DROP_SHIP_FULFILMENT`。两类 release fact、Objective、obligation、effect、evidence 和 reopen fact 禁止交叉消费；`(legal_entity_id, sales_order_id, objective_family='PRIMARY_ORDER_FULFILMENT')` 必须在订单生命周期内唯一。重放返回同一 Objective，重开只追加原 kind 的新 cycle。

最小 UI 只要求受控服务器管理面和 Windows Workbench 跑通该闭环；macOS/iOS/Android 同期可以进行技术 probe，但未认证平台不得发布生产包。切片至少演练重复点击、并发更新、执行中撤权、进程重启、外部成功但响应超时进入 Unknown 后对账、HDD 黄线、备份与业务负载重叠，以及数据库与附件一致恢复。通过只产生 `DEV_SLICE_GREEN`，不代表四端、门户、离线、P340 或最高安全生产已经认证。

## 12. 安全、审计与勒索恢复

### 12.1 安全域与身份分离

Windows 管理员、系统安全管理员、数据库服务、业务配置、插件、门户、备份写入、备份销毁和密钥恢复必须使用分离身份。单一账号不得同时修改生产数据、删除历史备份和恢复主密钥。

外部身份提供者只负责认证；平台动态权限引擎负责授权。本地账号、双人破窗应急账号和 AD/LDAP 是第一阶段必须完成认证的路径；在对应证据通过前不得宣称可用。OIDC/SAML 通过签名 provider contract 接入，只有具体 provider 的 issuer、audience、签名、元数据、重放和注销 conformance 证据通过后才启用。高风险操作必须 MFA 或重新认证；外部身份故障不得使客户失去本地控制。

Windows 安全时间必须由批准的 W32Time source 提供，并持久记录 source、offset、last-successful-sync 和健康状态。业务持续时间、租约、重试退避和超时一律使用 monotonic clock；墙钟只用于可审计时间戳和业务日期。检测到墙钟回退、超出批准 offset 或时间源不可用时，合同生效、付款/退款、最终审批、授权/破窗、许可、签名、配置/包/流程发布、密钥和备份处置等高风险动作必须 fail-closed，普通低风险读取可显著降级并告警（稳定需求 `SEC-013`）。

### 12.2 加密与客户持钥

- SSD 系统卷、HDD 数据卷和备份介质均加密；
- 高保密字段、附件和备份使用应用层加密；
- 不同法人、用途和数据等级使用独立密钥域；
- 客户可以使用客户部署内置、由客户控制的密钥服务与 TPM wrapping，也可以连接客户控制的 HSM/KMS；每个部署必须有客户独立持有的恢复材料，厂商不得持有恢复材料，也不得预置通用应用主密钥文件；
- 厂商默认不能解密生产数据；
- 轮换、恢复和销毁要求双人控制与不可删除审计。

磁盘加密只防介质离线读取，不能替代不可变备份和权限分离。

OS SSD 的 BitLocker protector 语义固定，并在同一最高安全档内只允许两个互斥运行模式：`TPM_ONLY_UNATTENDED` 使用 TPM/PCR、Secure Boot、锁闭机房与物理启动门禁换取 UPS 后无人值守重启；`TPM_PIN_ATTENDED` 在同一控制之上增加启动 PIN，但必须取消无人值守恢复声明，改用有人值守启动、告警与单独实测的 RTO。当前 P340 生产基线固定为 `TPM_ONLY_UNATTENDED`；部署若切换模式必须重新签署配置代并重跑启动、恢复和容量证据，不能同时宣称两种模式的优点。DATA_HDD 独立固定 protector exact-set `{PUBLIC_KEY,RECOVERY_PASSWORD}`，禁止 Windows fixed-data auto-unlock；OS trusted boot 成功后，只允许独立、无出站网络的 restricted-LocalSystem `EPF57DataVolumeUnlockBroker` 验证九个 pre-HDD locator、证书策略/链、bootstrap authority、TPM NV 和目标卷，并以现有 TPM-backed/nonexportable 证书私钥、固定 thumbprint、本机 WMI `UnlockWithCertificateThumbprint` 解锁。SSD 上唯一允许的密钥相关窄例外是受界限约束且可重新登记的 TPM-bound machine-key/certificate-store binding，以及非秘密 locator/trust metadata；应用主密钥、客户秘密或可导出 recovery material 绝不允许。OS 卷与 DATA_HDD recovery password 必须彼此独立、服务器外、双人保管，并与应用 vault 恢复材料、备份恢复身份和材料分域；clean-SSD/TPM-loss 必须在 admission closed 下通过双人 48 位 recovery-password 仪式创建新 key/certificate/PUBLIC_KEY protector、提升 epoch/NV、普通重启验收后移除旧 protector，不能从公开 metadata 重建旧私钥（稳定需求 `SEC-012`）。

数据库启动前需要秘密时，只能通过最小 pre-DB secret broker 取得。每个 DEK 必须分别 wrap 给日常 TPM/HSM operational recipient 与独立、离线的 recovery recipient；这是两个隔离用途的可恢复路径，不是要求两域同时在线的 2-of-2。日常域只能在受信启动、正确 service SID、用途和配置代下调用 operational unwrap，不能调用 recovery 接口；recovery recipient 固定采用 `PIV_SHAMIR_2_OF_3_V1`，生成 3 份由不同保管人持有的 share，任意 2 份才能在洁净主机脱离原机完成恢复，任何单一保管人均不能恢复。两份信封的认证上下文均精确绑定 deployment、legal entity、purpose、data-key id/version、wrap-context generation、recipient 和 envelope version，精确编码与失败语义以 [ADR-0020](../../adr/ADR-0020-dual-recipient-data-key-recovery.md) 为准。broker 只发短期、不可导出的 handle，并同时绑定具体 call、recipient service 与 config generation，错接收者、旧代、重放或过期一律拒绝。禁止通用应用主密钥文件，也禁止把客户秘密放入 WinCred、服务 profile 或普通环境变量。洁净跨机恢复必须只依赖客户独立恢复材料、签名部署身份和双人批准，并证明原机 TPM、OS 盘或数据库均不是唯一恢复前提（稳定需求 `SEC-014`）。

### 12.3 勒索恢复防线

1. 专用服务器、最小网络暴露、MFA、应用允许列表、补丁和插件隔离；
2. PostgreSQL 时间点恢复，用于逻辑错误，但同盘材料不计入灾难副本；
3. 服务器外、加密、追加式 HDD 增量备份，写入身份不能覆盖或删除历史；
4. 至少两块不同 `media_id` 的加密离线轮换 HDD，除受控轮换/恢复窗口外全部物理断开，轮换身份与连续备份身份分离；
5. 未来可增加凭据分离的 Windows Server 暖备，提升前必须 fencing 旧主机；暖备永远不计作备份层。

当前上线 exact-set 必须同时含 `CONTINUOUS_APPEND_ONLY`、`OFFLINE_ROTATION` 和独立 `RECOVERY_MATERIAL` 三种恢复角色；连续目标、每块离线介质和洁净恢复主机都必须容纳实际恢复集、加密/校验工作空间与增长余量。每个 backup set 使用独立 backup DEK，其 recovery-only 信封、三份加密 share、2-of-3 互操作与跨洁净主机恢复以 [ADR-0024](../../adr/ADR-0024-f57-backup-key-envelope.md) 为准（稳定需求 `SEC-016`）。当前单 HDD 上线前还必须具备兼容 UPS 和完整洁净恢复演练。连续服务器外副本、离线轮换、暖备和 RAID 彼此都不能互相替代。

P340 单 HDD 的基础备份默认采用 `PostgreSQL 输出 → 固定大小分块 → 当场认证加密 → 服务器外追加目标` 的流式管道。本机 `backup-staging` 只能保存容量证书限定的有界分块和断点状态，禁止在同一 HDD 落完整基础备份副本或未加密完整备份。外部目标确认后及时释放分块；重启后根据分块摘要和目标回执继续或安全重做。某工具只能先落完整本地副本时，P340 单盘生产路径必须拒绝，除非进入签名维护状态并重新证明空间、延迟和恢复边界。“备份成功”只在外部目标回执、清单完整、digest 验证和恢复性检查全部完成后成立，本地暂存完成不算成功。

### 12.4 审计

审计必须追加写入并形成哈希链，定期向服务器外写签名检查点。审计至少记录 actor、时间、设备、认证、授权依据、配置代/包/流程版本、before/after、审批、AI/MCP/插件、导入导出、密钥、备份、恢复和结果证据；字段完整性由稳定需求 `SEC-010` 验收。无需区块链。

权威节点 SSD 上的 Windows Event Log 只允许固定稳定事件码和随机生成、不可推导业务对象的 incident ID；不得写客户值、业务对象 ID、客户正文哈希、文件名、查询文本或其他可关联客户的载荷。详细诊断和审计必须进入 HDD 受控存储（稳定需求 `SEC-011`）。

### 12.5 更新与支持

厂商不得强制推送生产更新。客户通过离线介质或客户仓库接收签名包，完成签名、SBOM、兼容、模拟、审批、预部署、原子切换、观察和回滚。

系统默认零出站、零强制遥测、零永久远程通道。支持时由客户检查、脱敏并导出诊断包；远程协助必须限定人员、时间、范围和权限，到期撤销并完整审计。

数据保留、法律保留、隐私处置和销毁必须使用版本化 policy、双人释放和可验证 disposition；法律保留优先于普通清理，恢复后必须重放处置 tombstone，不能让数据因恢复复活（稳定需求 `SEC-007`）。远程支持使用无永久通道的限时状态机（稳定需求 `SEC-006`）。安全事件与漏洞采用隔离、服务器外证据、凭据/证书/密钥轮换、known-clean 恢复、业务对账和再放行闭环（稳定需求 `SEC-017`）。三项精确契约见[客户端/生命周期/安全运营执行契约 §§6–8](2026-08-23-f57-client-lifecycle-security-contract.md)。

## 13. Windows Server、HDD 与 P340 生产基线

### 13.1 当前硬件档案

| 项目 | 冻结值 |
|---|---|
| 主机 | Lenovo ThinkStation P340 Tower |
| CPU | Intel Core i5-10500，6 核 12 线程 |
| 内存 | 32GB |
| 系统盘 | 256GB SSD |
| 数据盘 | 单块 1TB HDD，可后续增加 |
| 服务器系统 | Windows Server 2022，原生服务，不使用 WSL |
| 用户基线 | 约 20 名活跃用户；不是硬拒绝上限，也不是许可口径 |
| 本地模型 | 延后开发，当前容量档关闭 |

### 13.2 SSD/HDD 路由

`HDD_STRICT` 只约束 authority node 上承载内容或可关联客户的持久数据；终端仍可按 §10.3 保存最小、加密、可撤销、非权威缓存。权威 SSD 的 Set A 只允许 Windows、程序、静态资源、可重装依赖、可重新下载模型、由签名安装器/WDAC 保护的非秘密验证信任锚（含客户登记公钥）、§12.2 明确允许且可重新登记的 OS-managed sealed BitLocker/key-store metadata，以及 §12.4 限定的固定事件码和随机 incident ID。Set B 是唯一 mutable 例外，exact 四类为：有界 POWER capsule、有界 package-recovery continuation capsule、recovery-domain-signed kernel pointer/journal head、可重建的 content-addressed signed native-code slot/cache；每类必须使用独立签名 media/path、大小、保留、off-host mirror、终态删除与 SSD-loss 重建契约，任何第五类、未登记路径或客户/业务 authority 字节都失败关闭。客户 deployment manifest 本体必须位于 HDD `packages`，不得放入 SSD software root。PostgreSQL data/WAL、附件、索引、审计、应用日志、报表、导入导出、插件工作区、临时业务文件、含业务数据的 pagefile/dump 和所有衍生数据必须位于加密 HDD 数据卷。密钥材料不与业务数据同盘：它不得成为 SSD/HDD 明文文件。生产凭据密文保存在 HDD 加密秘密库，只有非导出的 wrapping handle 可以位于客户批准的 TPM、HSM 或 KMS；不得把客户凭据正文持久化到 SSD 上的 WinCred、服务 profile 或普通文件。密钥元数据和业务密文仍位于 HDD（稳定需求 `NFR-002`、`SEC-011`、`SEC-012`）。

安装器和启动自检必须验证每个路径，并同时验证 OS/DATA_HDD protector exact-set、Secure Boot、PCR、九个 pre-HDD locator、证书策略/链、broker restricted token/WMI 权限、explicit-thumbprint unlock readback 与 `fixed_data_auto_unlock=false`；无法证明不写 SSD、不能阻止第二 pipe instance，或不满足该解锁门禁的组件不能取得生产认证。必须分别演练普通 broker reboot unlock、TPM/OS SSD 损坏后的双人八步 recovery-password 重新登记和任一 recovery password 被盗，确认新 key/certificate/protector 与 epoch/NV 原子推进、旧 protector 仅在普通重启验收后移除、数据卷不误解锁且应用/备份恢复域不被连带攻破（稳定需求 `NFR-015`）。RAM 可以缓存和计算，但不得成为唯一持久副本。

### 13.3 低资源生产档

当前档默认关闭本地模型和 Windows container activation；WASM 与受 Job Object 约束的 worker 可用。大型报表并发为 1，OCR、批量导入导出和低优先级自动化排队。资源优先级固定为：

1. PostgreSQL 事务和 WAL；
2. 身份、授权和审计；
3. 增量备份；
4. 交互查询与保存；
5. 到期自动化；
6. MCP、OCR 和连接器；
7. 批量作业、报表、AI 和维护。

系统必须按磁盘延迟、队列、剩余空间、锁、内存、CPU、备份年龄和 UPS 状态动态降级。数据卷必须保留普通业务不可占用的紧急空间。

Authority 必须持有唯一的全机 `CapacityGovernor`。其他服务开始 OCR、报表、插件、MCP、连接器、归档、批处理或大流量备份前，必须经认证 IPC 取得带期限、按字节/并发计量的资源许可；每个服务仍须有本地硬上限和有界队列。Governor 不可用时不得接纳新的非关键重任务，禁止无界 task、channel、重试和临时文件。保护 PostgreSQL、WAL、身份、安全审计和交互保存的方式只能是限制其他 I/O，不能暂停或节流 WAL。

容量证书必须冻结运行 participant set、连接预算、队列/worker 上限、临时与 backup-staging 字节上限、磁盘延迟阈值和降级动作；拓扑、硬件、池预算或 generation 改变后旧证书失效。20 人认证必须把基础备份、WAL 传输、附件操作和重报表与混合业务负载重叠运行，并证明 72 小时内无队列失控、磁盘耗尽或审计丢失。

### 13.4 低成本技术约束

第一阶段不引入 Kafka、Redis、Elasticsearch、Kubernetes、Temporal 集群或独立分析仓库作为运行必需。优先使用 PostgreSQL、Rust 模块化主体、内存缓存和有界后台队列。未来增加的 provider 不能改变当前单节点可运行性；该禁止运行时依赖约束由稳定需求 `NFR-013` 验收。

### 13.5 容量认证

安装后必须测量 CPU、内存、HDD 顺序/随机/同步写、PostgreSQL 提交、附件、加密、备份、恢复和 20 人混合业务负载，生成含硬件、数据规模、配置代和软件版本的容量证书。20 人是 Workbench、客户门户和供应商门户合计；认证同时运行独立保留资源的 Control Center 管理会话，并包含登录/重连与门户附件 burst。容量状态为绿色、黄色或红色；第 21 名用户不被强制拒绝，但低优先级工作可被节流。

P340 是工作站，必须额外验证 Windows Server 驱动、BIOS、TPM、BitLocker、存储控制器、网卡、散热、断电恢复、UPS 和磁盘健康。未通过不得宣传为服务器级高可用。

### 13.6 升级路线

1. 当前 32GB、单 HDD：单磁盘降级生产，UPS、服务器外追加式增量备份、离线轮换副本和恢复演练为上线硬门；
2. 最终安装两块匹配企业 HDD 组成经验证的 RAID1：现有 1TB HDD 通过型号、健康、性能和兼容认证时只加一块匹配盘；未通过则移除现盘后安装两块新盘，权威数据迁入镜像；
3. 增加到 64GB，并升级独立备份设备和电源保障；
4. 增加第二台 Windows Server 暖备。

未通过复用认证而被移除的现有 1TB HDD，在安全擦除和处置批准后只能作为非权威暂存，不能作为独立备份；若通过认证并成为 RAID1 成员，则不适用该句。P340 内部磁盘升级必须关机并进入维护窗口，不得宣传为硬件热插拔。

### 13.7 可用性与恢复目标

第一版只有一个写权威，不采用主主或双活。暖备提升前必须 fencing 旧主机。

- 健康暖备候选 SLO：RPO 不超过 5 分钟，批准提升后的 RTO 不超过 30 分钟；
- 从服务器外备份完整恢复候选 SLO：RPO 不超过 15 分钟，RTO 由数据量、HDD 和实测恢复证书决定，不承诺统一四小时。

这些值始终绑定具体数据量、配置代、软件版本和硬件/介质档案，只是候选目标 SLO。首次上线、滚动 90 天三次连续成功、证书 90 天有效期、失败/变更失效与 UI 状态闭集以[客户端/生命周期/安全运营执行契约 §10.5](2026-08-23-f57-client-lifecycle-security-contract.md#105-恢复认证策略)为准。未满足时不得标记为“已认证”，也不得在 UI、导出、合同或销售材料中展示为承诺（稳定需求 `NFR-014`）。

### 13.8 数据驻留、语言、币种与业务时间

首版生产客户数据、备份、日志、索引、诊断、支持导出、provider 输入输出及一切可关联客户的衍生数据只允许在中国大陆境内处理和持久化。deployment、provider 和 backup manifest 必须携带 jurisdiction/region/处理位置证据；未知、过期或跨境端点失败关闭（稳定需求 `NFR-017`）。

首版产品语言固定 `zh-CN`、经营币种固定 `CNY`、业务自然日和默认显示时区固定 `Asia/Shanghai`；权威时间戳仍以 UTC 保存，持续时间使用 monotonic clock。多语言、多币种、外汇、进出口、报关和信用证不能通过低代码或插件旁路启用（稳定需求 `NFR-018`）。精确边界见[客户端/生命周期/安全运营执行契约 §5](2026-08-23-f57-client-lifecycle-security-contract.md#5-中国大陆驻留与首版本地化)。

## 14. 商业支点与产品化约束

### 14.1 目标市场

第一市场为合同驱动的 B2B 设备分销、系统集成、工程交付和技术服务企业。销售切口是“合同履约与回款闭环”，不是替换所有 ERP。

### 14.2 行业包飞轮

每次客户定制必须尽可能沉淀为经过脱敏、通用化、签名和自动测试的行业能力包。共享的是流程结构、闭环条件、补偿路径、报表和连接器，不是客户原始数据。结构建议只有经客户本地检查、脱敏和批准后才能导出。

### 14.3 单一安全档

产品只提供完整高安全基线，不出售关闭加密、审计、备份、数据导出或签名验证的低档版本。收入来自平台许可、容量、行业包、连接器、维护、恢复验证和伙伴认证。永久许可可以提供，但更新权和维护另行计费，客户历史数据不因许可变化被删除。

### 14.4 一亿元机械目标

以下只是反推产品化效率的商业目标，不是收入保证：

```text
300 家企业客户 × 年均 32 万元 = 9,600 万元
50 次新客户激活或重大扩展 × 8 万元 = 400 万元
合计 = 1 亿元
```

实现该目标的核心约束是：一个客户不得对应一个内核分支；交付必须主要由标准平台、行业包、客户配置包和少量连接器完成。

## 15. 错误、验证与发布门禁

### 15.1 统一错误分类

错误必须包含稳定错误码、安全用户说明、retryable、人工动作、业务/运行关联号、已完成与未完成效果及建议下一步。分类至少包括输入、业务冲突、权限、并发、临时依赖、外部结果未知、数据一致性、资源和安全事件。

权限错误不得泄露无权对象；并发不得静默覆盖；未知外部结果必须对账；一致性异常必须隔离并停止危险写入。

### 15.2 强制故障演算

测试必须覆盖重复点击/消息、并发编辑、双离线设备、执行中撤权、旧客户端、新配置、插件中途停用、外部成功但响应超时、付款后断网、Windows 断电/更新、W32Time 回退/漂移/不可用与 monotonic duration、PostgreSQL 中断、HDD 慢/满/损坏、TPM/OS 盘损坏、BitLocker PCR 漂移和 recovery key 被盗、pre-DB broker AAD/handle 重放与洁净跨机恢复、附件损坏、备份失败与恶意容量耗尽、主备分区、AI 越权、MCP 恶意载荷、Provider 权限交集错配、Root/Jailbreak/缓存复制/端点撤销、签名证书失窃、跨境 provider/备份、时区/夏令时、金额舍入、部分收付/退货、员工离职、远程支持超时、安全事件恢复、勒索和洁净恢复。

### 15.3 发布前门禁

- 需求追踪到能力、数据、权限、测试和证据；
- 配置代通过 schema、权限、流程、并发、故障、财务、库存、审计和回滚模拟；
- 四平台通过能力硬门、离线冲突和设备撤销测试；
- 四平台通过客户签名、安装、更新、最低版本、撤销和终端 DLP 载体差异测试；
- 威胁模型、越权、跨法人、插件/MCP、供应链、密钥和审计验证完成；
- 数据生命周期、法律保留、完整导出、远程支持和安全事件闭环取得证据；
- 数据驻留、`zh-CN`、`CNY`、`Asia/Shanghai`/UTC 存储边界和跨境负例通过；
- P340/Windows Server 容量认证完成；
- 所有数据路径位于 HDD；
- 服务器外备份与完整恢复演练完成；
- 没有未处置的严重数据一致性或安全问题；
- 单 HDD 状态必须显著显示为“单磁盘降级生产”。

### 15.4 完成档位与诚实状态

交付状态闭集为：

| 状态 | 唯一含义 | 明确不代表 |
|---|---|---|
| `DESIGN_READY` | 权威设计、契约与实施计划无未裁决矛盾 | 任何代码或运行能力已实现 |
| `DEV_SLICE_GREEN` | CTC-01 在合成/脱敏数据上通过最小 Windows 纵向闭环与规定故障测试 | 四端、门户、离线、硬件或生产安全已认证 |
| `INTEGRATION_GREEN` | 当前 due capability、provider、客户端和跨域集成在候选树上通过 | 真实生产、RPO/RTO 或最高安全可对外承诺 |
| `RELEASE_CERTIFIED` | 同一 `CandidateRunIdentityV1` 与签名最终候选完成全部 release-due 测试、签名、四端、P340、UPS、备份和洁净恢复证据，且离线证据链可从显式 bundle root 完整重验 | 超出证书硬件、数据量、拓扑和有效期的能力 |

低档通过永远不能推出高档通过。`DEV_SLICE_GREEN` 只能使用合成或脱敏数据。安全宪法 F57-A01…A15、typed write、runtime authorization、CAS/幂等、单一 owner、fact+audit+Outbox 原子提交、generation pinning、Unknown 对账、HDD 路由、附件隔离、可恢复格式和诚实状态显示在任何可运行切片中都不得延期。

现有 RequirementID 保持稳定，但每项必须增加 `first_due_profile` 和 `release_due_profile`。后移只改变首次到期档位，不改变最终 Requirement、owner、测试和证据；未到期或未认证能力统一显示 `DISABLED_NOT_CERTIFIED`。

### 15.5 依赖 DAG、首条闭环与四层 CI

现行 25 个 F57 task ID 继续作为 Requirement ownership/evidence bucket，编号大小不再表示唯一全局开发顺序。重写后的每个工作节点必须声明 `requires`、`produces`、migration reservation、affected capabilities、`first_due_profile` 和 `release_due_profile`；只有显式 DAG edge 可以阻塞并行工作。迁移保留号与最终 apply 顺序继续单调串行，但业务代码、客户端 probe、硬件 probe 和独立测试可以并行，并在合并点执行同一 fresh PostgreSQL 验证。

实施计划必须把原 Task 1 拆成可独立评审的四个 gate：权威/Windows 运行时再基线；CapabilityGraph schema/compiler 与生成投影；签名/generation/证据骨架；旧迁移与分层 CI 清理。Task 1 不再要求在第一项工作中生成完整生产 CNG/TSA/CRL、25-task aggregate、四端生产签名或全部长时证据。第一条业务价值必须按 `CTC-01` 提前出现，不能等待完整 provider、完整四端、完整模型设计器和所有平台能力铺满后才验证。

CI 分为四层：

1. `L0 Developer`：format/lint/archcheck、能力图生成无 diff、受影响 feature 单元/属性测试；
2. `L1 Pull Request`：受影响 feature 与依赖者、到期 fresh PG migration、Rust/TS/OpenAPI 投影和关键安全静态负例；
3. `L2 Integration Candidate`：G4 证明完整 Windows authority、CTC-01 E2E/故障注入和最小 Workbench；G5 还可证明选定栈四平台同协议集成；最终 G6 在生产签名制品上执行自己与 L3 不相交的 final-L2 集合；
4. `L3 Release Certification`：同一签名最终候选精确合并 final-L2、先前同运行物理 carrier 与只运行一次的剩余 release/provider TestID，覆盖全部 release-due Requirement、四端生产签名、安全/provider conformance、P340 72 小时、UPS/满盘/断电、服务器外追加备份、离线介质、勒索与洁净恢复。

长时硬件和恢复证据只能由 Rust 封闭 dispatcher 先在显式签名 run journal 中持久化 `TEST_STARTED`，再调用固定 AllSigned carrier recipe，不能直接执行脚本后补证据，也不能伪装成普通 PR job。journal 使用 OS 单写者 lease、签名哈希链、崩溃对账和严格延伸 checkpoint；同一 `(candidate_identity_sha256,gate_run_id,TestID)` 不得第二次开始。候选、L2、L3、G0…G5 receipt 与 release certificate 采用 create-new/atomic-write + typed envelope-bound 恢复规则，任何重启只采用原字节，不得重签择优。185 行统一使用 signed `RequirementEvidenceBindingV1`，每个 due TestID 的 TestID→result 映射唯一，再按 RequirementID 聚合。最终 certificate 通过 signed relative refs 从显式绝对 bundle root 到达候选、L2/L3、六级 receipt、journal checkpoint 与全部结果；不得扫描或猜文件。生产 Authority 不承担 CI 聚合，只参与其自身实机证据。

## 16. 第一阶段明确不做

- 不交付本地大模型本身，但交付 AI provider 和授权契约；
- 不自研完整 MRP、MES、高级排产、大型 WMS 或自动化立库；
- 不自研完整法定会计、税务申报和工资系统；
- 不把旧 PRD 的其余延期项因 F-57 未重复列出而自动恢复；HR、GRC、法务、商旅、ECM/CMS/GIS、PLM/PIM/QMS 等继续延期；
- 不认证寄售、订阅和租赁销售闭环；第一阶段只交付其 provider seam，完整认证目标限于 `STANDARD` 和 `DROP_SHIP`；
- 不交付主主、双活或多写数据库；
- 不认证 PostgreSQL 以外的权威数据库；
- 不认证通用 XML/SOAP/XSD 交换面；特定行业/厂商 XML 只能作为逐项认证的 provider codec；
- 不允许任意原生 DLL 热注入；
- 不允许离线产生付款、最终审批、合同生效、权限或配置权威结果；
- 不依赖 Linux、WSL、Kubernetes、厂商云控制面或强制遥测；
- 不保证未经演练的 RPO/RTO，也不把 RAID 描述成备份。

## 17. 已关闭与现场选择

本设计不存在仍需产品负责人重新选择的产品方向；2026-08-24 收敛修订、ADR-0025 与五文件实施计划集已获用户批准并达到 `DESIGN_READY`。项目状态机节点为 `READY_NOT_AUTHORIZED`（保持至 `G0_BOOTSTRAP_GREEN`）；**开发授权已于 2026-08-27 由使用方明确授予**（`DEVELOPMENT_AUTHORIZED=true`，留证 00c F-65），G0 可开工。以下属于实施或部署现场选择，不改变本文的目标架构：

- Tauri 2 是否通过四平台硬门；失败后的唯一回退已固定为 Flutter + Rust；
- 新增 HDD 的具体型号、控制器或 Windows 镜像实现，以实机证据选择；
- 客户在已认证 core/provider 目录中启用哪些连接器、身份提供者、密钥提供者和外部 AI；未取证 provider 保持关闭；
- 当前客户 P340 物理机的具体网络、备份介质和责任人；未来 IaaS 只有在另行授权的新 graph/profile version 落地后才成为独立现场选择；
- 法规、财税、密码应用和行业认证所需的专业签字。

这些选择具有固定验证门和失败处置，开发人员不得因此改变本文的产品边界。

## 18. 参考架构来源

- DeepSeek Harness 对 durable events、live hooks、capability seam 和 plugin ownership 的分离：[Architecture](https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/architecture.md)
- PostgreSQL 16 WAL 与恢复配置：[PostgreSQL 16 Documentation](https://www.postgresql.org/docs/16/wal-configuration.html)
- Tauri 2 跨平台能力与移动端边界：[Tauri Documentation](https://v2.tauri.app/)
- Flutter 支持平台与桌面/移动能力：[Flutter Documentation](https://docs.flutter.dev/)
- Windows Server 生命周期：[Microsoft Lifecycle](https://learn.microsoft.com/en-us/lifecycle/products/windows-server-2022)
- ThinkStation P340 Tower 的磁盘位、RAID、内存与支持操作系统边界：[Lenovo PSREF](https://psref.lenovo.com/syspool/Sys/PDF/ThinkStation/ThinkStation_P340_Tower/ThinkStation_P340_Tower_Spec.PDF)

外部资料用于帮助确定能力边界；本文批准的约束才是本项目的规范输入。
