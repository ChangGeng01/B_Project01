# F-57 业务执行补充契约

> 日期：2026-08-23（Australia/Melbourne）
> 收敛修订：2026-08-24（Australia/Melbourne）
> 文档状态：CURRENT_SUBJECT / APPROVED
> 实现状态：NOT_IMPLEMENTED
> 产品选择状态：CLOSED；CTC-01 与 STANDARD/DROP_SHIP XOR 修订已批准
> 适用范围：F-57 第一阶段 CRM、CPQ、销售、采购、服务、项目、门户、自动化、报表和四端业务执行
> 权威关系：本文是 F-57 总体设计在业务执行主题下的现行补充规格；不得覆盖 F-57 系统宪法、F-50 财务不变量、最高安全档和明确延期边界。本文与旧 PRD 冲突时，仅在本文列明的业务主题内由本文取代旧排除或旧固定岗位口径

## 0. 用途、状态与规范语言

本文把 F-57 已经提升为当前范围、但旧 PRD 曾明确排除或未完整展开的业务能力冻结为可执行契约。它不增加新的产品域，不恢复未提升的旧延期项，也不代表任何代码、迁移、四端、Windows 实机或生产认证已经完成。

本文中的“必须”“只能”“禁止”“不得”是实现和发布门禁。“默认”是首发配置代的冻结值，可配置项只有在本文同时给出合法范围、验证方式和失败语义时才可修改。配置输入不是开发人员可以重新选择的产品语义。

本文全部能力当前统一为 NOT_IMPLEMENTED。只有对应 owner task、数据库契约、命令/事实、状态机、自动测试和证据全部通过后，单项才能晋级；不得以本文存在为由宣称功能可用。

本文为保留需求追踪而出现的 `Task 1…25` 是历史 `F57-01…F57-25` **所有权桶标签**，不是执行顺序、交付时间或旧计划授权。实际到期唯一由 2026-08-24 收敛主计划 §4 的基础映射与逐 Requirement 覆盖表 `docs/f57-requirement-delivery-profile-overrides.v1.tsv` 共同解析；`slice_probe_profiles_json` 只允许提前验证窄切片，不能提前满足整项需求。若本文 task 标签与新计划的文件、迁移、依赖、到期档位或门禁冲突，以新计划为准，业务 exact-set 和不变量仍以本文为准。

### 0.1 绑定的现行需求

本文细化但不另造范围，绑定以下稳定需求：

| 主题 | 现行 RequirementID |
|---|---|
| CRM 与 CPQ | CRM-003、CRM-004、CPQ-001 |
| 销售订单来源与闭环 | SAL-001、SAL-003、SAL-004、SAL-005、SAL-006、SAL-007 |
| 采购与询比价 | PROC-001、PROC-002、PROC-003、PROC-004、PROC-005、PROC-006、PROC-007、PROC-008、PROC-009 |
| 服务、投诉、设备与项目 | SRV-001 至 SRV-010、PRJ-001 至 PRJ-004 |
| 自动化与动态责任 | AUT-001 至 AUT-007、AUTH-001 至 AUTH-006、PLT-003 |
| 报表 | REP-001 至 REP-004 |
| 客户/供应商外部门户 | POR-001、POR-002、POR-003、IDP-001、AUTH-005 |
| 四端 | CLI-001 至 CLI-009、GOV-001、GOV-008 |
| 集成未知结果 | FIN-009、MCP-003、AUT-002 |
| 当前地域与产品说明 | NFR-009、NFR-014、NFR-017、NFR-018；本文第 15 节给出业务部署约束 |

### 0.2 唯一所有者

下表的 owner 值必须逐字使用主计划 §2.1 的 `FeatureOwnerIdV1`，或明确标注为平台机制 `PlatformMechanismIdV1`。`mdm|clm|sales|procure|inventory|finance|invoice|ledger|service|project|portal|identity|automation` 只是历史模块/数据库别名，禁止作为 CapabilityGraph owner、事实 owner、生成路径或权限判断输入。

| 事实 | 唯一 owner |
|---|---|
| 客户/供应商/产品/单位/仓库/价目主档，联系人、地址、重复识别、合并和历史身份 | `customer-master`；合并是高风险追加事实，被合并客户历史不得删除 |
| 商机与跟进 | `crm` |
| 客户 360 | `crm` 的授权投影；只聚合 owner 事实，不复制第二份权威事实 |
| 报价、报价版本与客户接受证据 | `cpq` |
| 合同版本、义务、收付款计划和合同附件链接 | `contracting` |
| 客户信用额度、占用、释放和敞口 | `receivable-cash`；CRM 只显示授权投影 |
| 客户投诉、服务请求、工单、设备、服务权益和服务关闭 | `service-cycle`；CRM/门户只是登记渠道 |
| 销售订单、商业快照、交付确认和退换货 | `sales-order` |
| 采购需求、RFQ、供应商报价版本、评估和授标 | `procurement` |
| 库存数量、金额、批次和序列事实 | `inventory-fulfilment` |
| 安装执行和现场技术证据 | `service-cycle`；由 INSTALLATION 工单承载 |
| 项目、任务、里程碑、项目风险和项目关闭 | `project-cycle`；项目只引用 `sales-order` 交付与 `service-cycle` 安装证据 |
| 企业现金/银行结算账户主档、应收、客户收款、核销、客户侧资金冲正 | `receivable-cash`；账户标识只保存加密 handle/blind reference 与授权展示用末四位，`payable-cash` 只能引用 |
| 应付、供应商付款、核销、供应商侧资金冲正 | `payable-cash`；不得复制或更新结算账户主档 |
| 内部经营分录、经营科目映射、试算和经营期间 | `operating-ledger` |
| 销售发票事实与红冲 | `sales-invoicing` |
| 采购发票事实与红冲 | `purchase-invoicing` |
| Objective、Obligation、Effect、Evidence、Incident 和 Cycle | 平台机制 `platform.flow`；自动化不得取得业务事实所有权 |
| 客户门户认证器、设备和会话 | 平台机制 `platform.identity` |
| 客户门户邀请、客户/联系人/法人 binding 生命周期和跨 identity/portal 的即时撤销编排 | `portal-identity`；只引用 identity 会话和 portal 投影，不复制认证器或客户主档 |
| 客户门户 allowlist 和裁剪投影定义 | `portal-experience`；客户主档只由 `customer-master` 拥有 |
| 指标、报表、看板和打印定义及血缘 | `reporting`；来源事实仍由各业务 owner 拥有 |

跨域能力只能调用公开命令、读取授权投影或消费已提交事实，不得直接更新其他 owner 的表。共享 `invoice`、`finance` 或 `portal` 物理 schema 不产生共享 writer；每张受保护表必须 exact-map 到上表一个 feature owner 和一个 repository module。

## 1. 全域执行不变量

### 1.1 命令、版本和事实

1. 每个 Workbench/employee API 在线业务命令及重放 `ClientIntent` 必须使用员工 API 唯一信封中的 `request_id`、`command_type`、`idempotency_key`、`expected_generation`、`expected_subject_version`、`generation_report`、`client_version`、`device_key_id`、`device_signature` 和类型化 `payload`；actor、当前法人、设备权威、权限策略和 authority epoch 只能由服务器认证上下文重建。Control Center 和 portal 不伪装成员工设备：它们分别只使用 Task 16 `ControlCommandEnvelope` 与 §10/Task 22 `PortalCommandEnvelope` 的唯一 IDL；三种入口都必须映射为 Task 6 同一 `CommandPipeline` 的受信 `SecurityContext + CapabilityCommand`，不得产生第二条业务写通道。
2. 同一法人、command_type、idempotency_key 只能产生一个权威结果。相同键不同载荷必须拒绝。
3. 当前状态、不可变业务事实、审计、Outbox 和命令回执必须在同一 PostgreSQL 事务提交。
4. 已批准、已发出、已接受或已经产生下游事实的商业内容不得原位修改；变化创建新版本或追加更正/冲销事实。
5. 所有跨对象引用必须带 legal_entity_id，并以同法人复合外键或等价数据库约束防止跨法人拼接。
6. 所有金额首发币种固定为 CNY；金额、数量、税率和舍入继续服从 F-50 与旧 PRD 未冲突细节。客户端计算只供预览，服务器必须重算。
7. 业务日期使用对象所属法人的 Asia/Shanghai 业务时区；授权、租约、到期、重试和证据时间使用可信服务器时间。客户端时间只作未信任显示元数据。
8. 历史事实禁止物理删除。业务撤销、作废、合并、纠错和重开必须追加原因、操作者、证据和前后版本引用。

### 1.2 通用状态机纪律

1. 状态只能由列明的类型化命令或确定性事实推动；禁止通用 UpdateStatus、管理员直接置终态或数据库脚本改业务状态。
2. 进入终态前必须计算 closure predicate；人工勾选“完成”不能代替证据。
3. 终态被新事实推翻时，按本文登记的重开规则追加新 cycle；不得覆盖旧 cycle 的关闭结论和证据。
4. 取消只适用于尚无不可逆业务效果的对象。已有外部、库存、资金、发票、签章或客户接受效果时必须走补偿、更正、退货、红冲、退款或影响处置。
5. 外部效果响应丢失时 effect 进入 UNKNOWN、对应 Objective 进入 RECONCILING；在证明结果前不得盲重试，不得创建语义相同的新对象绕过未知状态。
6. 每个状态转换都必须记录 from_state、to_state、wire `request_id`、actor、reason_code、generation、policy_version、occurred_at、business_fact_refs 和 evidence_refs；持久层若沿用内部列名 `command_id`，其值必须逐字等于 wire `request_id`，且该字段不得出现在客户端信封中。

### 1.3 动态权限

1. 销售、采购、财务、技术、管理者、项目经理等名称只可作为授权模板或工作台视图，不得成为运行时状态机条件。
2. 每次执行必须按主体、能力、法人、记录/字段范围、条件、期限、设备、金额、风险、委托上限和职责分离重新鉴权。
3. 分配给某人不等于授予权限；权限存在也不等于已分配责任。
4. 状态机、自动化和报表不得出现固定 RoleCode 兜底。无人可处理时进入显式无候选事故，不得自动扩大权限。

## 2. CRM 商机与跟进

### 2.1 当前对象

首发不实现独立线索或潜客对象。商机必须关联同法人已生效客户；未成为客户的对象先按 MDM/CRM 客户准入流程建档。

Opportunity 当前字段至少包括：

| 字段 | 规则 |
|---|---|
| opportunity_id、legal_entity_id、opportunity_no | 服务器生成；法人内编号唯一 |
| customer_id、customer_version | 必填；保存历史快照引用 |
| title、summary | 标题必填；摘要可空 |
| stage | 只取本文闭集 |
| amount_minor、currency | 预计金额不得为负；currency 固定 CNY |
| probability_bps | 0 至 10000；除终态约束外不按阶段暗改 |
| expected_close_on | 非终态必填，可版本化变更 |
| responsibility_query_id | 指向能力型候选解析，不保存固定岗位 |
| current_assignee | 可空；必须是 foundation 唯一 `PrincipalRefV1 {kind,id}`；只是当前责任，不授予权限，主体 kind 参与相等、幂等与审计 |
| source_kind、source_ref | MANUAL、CUSTOMER_INTERACTION、SERVICE_SIGNAL、RENEWAL_SIGNAL 四类 |
| next_action_at | 进入 QUALIFYING、SOLUTION、COMMERCIAL 后必填 |
| loss_reason_code、cancel_reason_code | 仅对应状态可有值 |
| canonical_successor_kind/id | WON 时必填，只取 QUOTE、CONTRACT、SALES_ORDER |
| row_version、generation | 每次权威变化递增并留历史 |

FollowUp 是不可变完成事实，至少保存 opportunity_id、occurred_at、channel、actor_id、summary、outcome_code、next_action_at、attachment_refs 和 evidence_refs。错误跟进不能覆盖或删除，只能追加 CorrectFollowUp，引用原记录并说明原因。

PlannedFollowUp 是待办而非完成事实，状态闭集为 `PLANNED|COMPLETED|CANCELLED|OVERDUE`。允许边只有 `PLANNED→COMPLETED|CANCELLED|OVERDUE` 和 `OVERDUE→COMPLETED|CANCELLED|PLANNED`；`COMPLETED`、`CANCELLED` 是不可恢复终态，未列边全部拒绝：

- PLANNED 必须有 due_at、purpose 和 responsibility_query_id；
- 完成命令在同一事务把任务置 COMPLETED 并追加 exact-one FollowUp；
- 可信时间越过 due_at 且未完成/取消时置 OVERDUE 并升级，但不自动伪造 FollowUp；
- PLANNED 或 OVERDUE 取消都必须有 closed reason_code；已经 COMPLETED 的任务不得取消；
- OVERDUE 完成后进入 COMPLETED，保留 overdue_at 和逾期时长；OVERDUE 改期只有在新 due_at 晚于可信当前时间时通过 `RescheduleFollowUp` 回到 PLANNED，追加旧/新 due_at、原因和审批，不覆盖逾期历史；PLANNED 改期保持 PLANNED 但同样追加版本事实。

### 2.2 商机状态机

闭集状态：

| 状态 | 含义 |
|---|---|
| DRAFT | 信息未完成，不进入漏斗和自动提醒 |
| QUALIFYING | 已确认客户和基本需求，正在资格判断 |
| SOLUTION | 正在确认范围、产品、交付和解决方案 |
| COMMERCIAL | 已进入价格、报价、合同或订单商业阶段 |
| WON | 已由服务器确认存在唯一 canonical successor |
| LOST | 客户明确不成交或商业机会失败 |
| CANCELLED | 重复、录错、客户无效或经批准停止跟进 |

允许转换：

| From | To | 必须条件 |
|---|---|---|
| DRAFT | QUALIFYING | 客户有效、title、预计金额、expected_close_on、next_action_at 完整 |
| DRAFT | CANCELLED | 原因必填；无任何已发报价或下游对象 |
| QUALIFYING | SOLUTION | 至少一条有效跟进和需求摘要 |
| QUALIFYING | LOST/CANCELLED | reason_code 和说明必填 |
| SOLUTION | COMMERCIAL | 交付范围和商业接收路径已确定 |
| SOLUTION | QUALIFYING | 退回原因和下一步必填 |
| SOLUTION | LOST/CANCELLED | reason_code 和说明必填 |
| COMMERCIAL | SOLUTION | 商业条件变化，保留已有报价事实 |
| COMMERCIAL | WON | 只能由 Quote/Contract/SalesOrderAccepted 事实触发；canonical successor exact-one |
| COMMERCIAL | LOST/CANCELLED | 没有仍有效的已接受报价或有效下游对象；原因必填 |
| LOST/CANCELLED | QUALIFYING | ReopenOpportunity；新 cycle、原因、next_action_at 和当前鉴权 |
| WON | COMMERCIAL | 仅当全部 canonical successor 在产生不可逆履约前失效，且自动化收到 SuccessorInvalidated；必须创建新 cycle |

WON 时 probability_bps 必须为 10000；LOST/CANCELLED 时必须为 0。其他阶段由有权人员明确输入，不得用未经批准的模型自动改写。

### 2.3 跟进与超时

1. QUALIFYING、SOLUTION、COMMERCIAL 的 next_action_at 到期仍无新跟进时创建逾期 work item；不得自动改变商机阶段。
2. 跟进可由人工、客户门户已允许命令、邮件/消息 provider 证据或受控导入产生；来源必须可追溯。
3. AI 只能提出跟进草稿或分类建议，不能自行标记 WON/LOST、不能创建价格承诺。
4. 客户合并时，商机通过批准的客户合并计划迁移引用；历史快照不改写，重复商机由独立合并命令处置。

### 2.4 商机验收

T-F57-CRM-003 必须至少包含：

- opportunity_lifecycle_exact_transitions
- won_requires_exact_one_canonical_successor
- lost_or_cancelled_reopen_creates_new_cycle
- invalidated_successor_reopens_without_deleting_history
- follow_up_is_append_only_and_correction_links_original
- planned_follow_up_exact_state_and_completion_fact
- overdue_follow_up_escalates_without_fake_completion
- opportunity_current_assignee_uses_full_principal_kind_and_id
- expired_or_revoked_assignee_does_not_expand_authority
- concurrent_stage_change_uses_optimistic_version

## 3. CPQ 报价与版本

### 3.1 报价容器和不可变版本

Quote 是稳定容器；QuoteVersion 是商业内容权威。每个版本至少冻结：

- 同法人 customer_id/customer_version 和可选 opportunity_id；
- CNY 币种；
- 全部行的产品/物料快照、说明、数量、单位、单价、折扣、税率、净额、税额、价税合计；
- 交付地址、交付计划、付款计划、报价有效截止时刻；
- 退换、取消、保修和服务权益摘要；
- 价格来源、越权原因、审批策略版本；
- 模板版本、附件、客户可见文档 digest；
- predecessor_version、version_no、content_digest、generation。

报价版本一经离开 DRAFT，商业字段不可原位修改。更改任一商业字段必须创建下一个 version_no。

### 3.2 报价版本状态机

闭集状态：

| 状态 | 含义 |
|---|---|
| DRAFT | 可编辑，客户不可见 |
| PENDING_APPROVAL | 等待价格/风险审批，不可编辑 |
| APPROVED | 内容已批准但尚未对客户生效 |
| ISSUE_PENDING | 正在向客户发布；外部效果尚未确认 |
| ISSUED | 客户已取得该 exact version |
| ACCEPTED | 客户完整接受全部版本内容 |
| REJECTED | 客户拒绝该版本 |
| EXPIRED | 有效期已过且未接受 |
| WITHDRAWAL_PENDING | 撤回通知外部结果待确认 |
| WITHDRAWN | 报价已成功撤回 |
| SUPERSEDED | 新版本已经 ISSUED，旧版本不再可接受 |

允许转换：

| From | To | 必须条件 |
|---|---|---|
| DRAFT | PENDING_APPROVAL | 金额重算一致、付款/交付/有效期完整、客户和价目有效 |
| PENDING_APPROVAL | APPROVED | 全部审批节点通过；提交人与审批人分离 |
| PENDING_APPROVAL | DRAFT | 驳回或申请人在首个决定前撤回；审批历史保留 |
| APPROVED | ISSUE_PENDING | IssueQuote；客户文档 digest 和外部幂等键冻结 |
| ISSUE_PENDING | ISSUED | provider/门户取得可验证送达或发布证据 |
| ISSUE_PENDING | APPROVED | 只在证明未执行后允许；Unknown 时保持 ISSUE_PENDING 并进入 RECONCILING |
| ISSUED | ACCEPTED | 客户对 exact version 完整接受并提供身份、时间、文档 digest 证据 |
| ISSUED | REJECTED | 客户拒绝证据或授权内部登记的客户拒绝证明 |
| ISSUED | EXPIRED | 可信时间超过 valid_until；与接受命令串行化，先提交者生效 |
| ISSUED | WITHDRAWAL_PENDING | 有权人员发起撤回；已有 ACCEPTED 不允许撤回 |
| WITHDRAWAL_PENDING | WITHDRAWN | 撤回效果确认 |
| WITHDRAWAL_PENDING | ISSUED | 证明撤回未执行；Unknown 时不得回退 |
| APPROVED/ISSUED | SUPERSEDED | 同一 Quote 的新版本成功进入 ISSUED；旧版本自动追加 superseded_by |

### 3.3 首发禁止部分接受

1. 第一阶段不支持客户部分接受报价，不支持只接受部分行、部分数量、部分交付计划或部分条款。
2. AcceptQuoteVersion 的接受载荷只能包含 quote_version_id、content_digest、客户身份和接受证据，不能携带修改后的行、金额、数量或条款。
3. 客户要求任何变化时，必须创建新报价版本并重新走价格校验、审批、发布和完整接受。
4. 部分接受请求必须返回稳定业务拒绝，指出需要新版本；不得把剩余行静默取消，不得把接受内容改写成订单。

### 3.4 转合同或订单

1. ACCEPTED 版本只能建立一个 canonical conversion root，target_kind exact-one 为 CONTRACT 或 SALES_ORDER。
2. 相同幂等键重复转换返回原目标；试图把同一接受版本同时直接转换为合同和订单必须拒绝。
3. 选择 CONTRACT 时，订单若产生只能以后由该合同版本派生；选择 SALES_ORDER 时按第 4 节 QUOTE_VERSION 来源建立无合同订单。
4. 已成功接受和转换的报价版本永不覆盖。若 canonical target 在不可逆履约前取消，商机重新进入 COMMERCIAL；新的商业承诺必须创建新的 Quote 容器，不能在已 ACCEPTED 版本下追加可接受版本，也不能再次消费旧接受证据。REJECTED、EXPIRED、WITHDRAWN 或尚未接受就被 SUPERSEDED 的版本可以在原 Quote 容器下产生新版本。
5. 报价转换保存 source quote/version、content_digest 和逐字段商业快照，不依赖报价当前投影。

### 3.5 CPQ 验收

T-F57-CPQ-001 必须至少包含：

- quote_version_exact_state_machine
- commercial_content_is_immutable_after_submission
- issue_timeout_stays_reconciling_without_duplicate_send
- acceptance_requires_exact_version_digest
- partial_acceptance_is_rejected_and_new_version_required
- new_issued_version_supersedes_old_issued_version
- accepted_version_has_exact_one_conversion_root
- quote_to_contract_and_quote_to_order_are_separate_certified_paths
- expiry_and_acceptance_are_serialized_at_exact_boundary

## 4. 三种销售订单来源与无合同商业快照

当前销售类型只取 STANDARD 和 DROP_SHIP。历史 NORMAL 只允许在一次性再基线迁移中映射为 STANDARD，不得作为现行 API、数据库枚举、事件、菜单、报表维度或新数据取值。

### 4.1 来源闭合联合类型

SalesOrderSourceKind 只能取：

- CONTRACT_VERSION
- QUOTE_VERSION
- MANUAL_AUTHORITY

销售订单头必须有三个 nullable 引用列：

- source_contract_version_id
- source_quote_version_id
- source_manual_authority_id

数据库 exact-one 约束：

| source_kind | contract | quote | manual |
|---|---:|---:|---:|
| CONTRACT_VERSION | 非空 | 空 | 空 |
| QUOTE_VERSION | 空 | 非空 | 空 |
| MANUAL_AUTHORITY | 空 | 空 | 非空 |

三个来源引用必须同法人、已通过来源状态门禁，并在订单创建后不可改变。订单行不得再强制要求 source_contract_id；每行引用订单头的 source_snapshot_line_id，并可额外保存来源行引用。

### 4.2 CommercialSnapshotV1

每张订单无论来源都必须拥有完整、不可变 CommercialSnapshotV1，至少包括：

- customer_id/customer_version、bill_to、ship_to 和联系人快照；
- 行号、销售项目类型和 ID、产品/物料版本、说明、数量、单位和换算；
- 单价、折扣、税率、净额、税额、价税合计和 CNY；
- 交付计划、交付地点、签收/验收要求；
- 收款/付款计划、开票依据和信用规则快照；
- 退货、换货、取消和变更条款；
- 保修期限、服务权益摘要、设备建档要求；
- source_kind、source_ref、source_content_digest；
- pricing_policy_version、credit_policy_version、generation 和 snapshot_digest。

下游交付、发票、AR、收款、退换、设备和售后读取该快照或其已提交事实，不读取来源对象的当前可变字段。

### 4.3 各来源准入

CONTRACT_VERSION：

- 来源必须已生效；
- 订单数量、价格、税、交付、付款和权益只能来自该合同版本的未履行义务；
- 合同变更只影响未履行部分，并通过正式影响计划生成订单新版本。

QUOTE_VERSION：

- 来源必须为 ACCEPTED 且 conversion root 尚未被消费；
- 报价必须完整提供生成 CommercialSnapshotV1 所需的付款、交付、退换和权益条款；
- 缺任一必需商业条款时拒绝转换，不使用系统隐式默认填空；
- 转换后该报价版本的 canonical target 固定为此订单。
- 同一报价版本一旦直接转订单，永久禁止再用该版本转合同。

MANUAL_AUTHORITY：

- ManualOrderAuthority 必须保存客户、逐行商业内容、付款/交付/退换/保修条款、人工建单 reason_code、业务证据和发起人；
- 必须经过独立审批，审批人不得是创建人；价格或信用越权继续触发对应更高风险审批；
- 不允许借人工来源绕过报价接受、合同生效、客户/产品准入、信用、税额、库存、交期或动态权限；
- 可以建立一次性保修摘要，但不能凭人工订单创建周期服务合同或订阅/租赁/寄售闭环。

### 4.4 无合同订单的下游规则

1. QUOTE_VERSION 和 MANUAL_AUTHORITY 订单的交付、发票、AR、收款、退换货和设备/保修以 CommercialSnapshotV1 为权威商业依据。
2. 无合同订单不得伪造 contract_id，也不得建立空壳合同。
3. 客户门户显示“来源：已接受报价”或“来源：经批准人工订单”，不得显示虚构合同。
4. 取消、变更和终止只走订单影响流程；不存在合同终止事件。
5. 需要新增周期服务、续约或长期收付款义务时必须另建并生效合同，不能修改原无合同订单补入。
6. 信用占用、开票、收款、退款和经营分录与合同来源订单使用同一 F-50 守恒规则。
7. 后续若为已存在的 QUOTE_VERSION 或 MANUAL_AUTHORITY 订单补签合同，合同必须显式采用 RECORD_EXISTING_ORDER adoption，并逐义务引用现有 order/order line；CLM-004 把这些数量视为已派生，不得再次自动建单。合同新增且未被 adoption 覆盖的义务才可生成新订单。
8. 合同自动派单的幂等键必须包含 contract_version_id、obligation_id 和 derivation_kind；同一义务无论重复激活、重放 Outbox 或先 adoption 后激活都最多产生一个 canonical order allocation。

### 4.5 来源和闭环验收

T-F57-SAL-001 必须至少包含：

- order_source_exact_one_database_constraint
- contract_quote_manual_sources_each_create_complete_snapshot
- source_reference_is_immutable_after_order_creation
- direct_quote_order_does_not_require_fake_contract
- manual_order_requires_independent_approval
- missing_contractless_commercial_term_rejects_conversion
- downstream_invoice_cash_return_service_use_snapshot_not_live_source
- direct_quote_order_cannot_convert_same_quote_to_contract
- later_contract_adopts_existing_order_without_duplicate_derivation
- historical_normal_sales_type_is_rejected_except_controlled_migration

Tasks 19 和 20 必须分别跑通三来源建单，以及 STANDARD/DROP_SHIP 的交付、退换、发票和资金闭环；只证明 quote→contract→order 不足以验收 SAL-001。

## 5. 六来源采购、RFQ、评估与授标

### 5.1 ProcurementDemand 来源 exact-one

DemandSourceKind 只能取：

- CONTRACT
- SALES_ORDER
- PROJECT
- INVENTORY_RULE
- MANUAL_REQUEST
- EXTERNAL_PRODUCTION

每条 ProcurementDemandLine 必须引用一个且仅一个 source ref，并保存 source_version、source_line、requested_item_snapshot、accepted_requested_quantity、uom、required_on、delivery_site、cancelled_quantity、currently_valid_awarded_quantity、unawarded_open_quantity 和 source_digest；后三个数量只能由 §5.2 的 owner facts 派生，不能作为三个可独立编辑的真值。

EXTERNAL_PRODUCTION 的幂等键固定为 legal_entity_id、provider_id、external_system_id、external_demand_id、external_version。相同版本重放返回原 demand；低版本拒绝；高版本只通过现有 `procurement_demand.change` 创建差异事实。

### 5.2 需求数量守恒

1. 合并、拆分、询价和授标通过 DemandAllocation 关联，不改写原来源。
2. 对每条 demand line，唯一需求层公式固定为 `accepted_requested_quantity = cancelled_quantity + currently_valid_awarded_quantity + unawarded_open_quantity`。`currently_valid_awarded_quantity` 只统计已批准且未被撤销/减量的 AwardAllocation；`unawarded_open_quantity` 是尚未被有效 Award 占用且未取消的数量。旧字段名 `awarded_quantity/open_quantity` 若仍存在，只能是这两个量的投影别名，不得形成另一套计算口径。
3. 三层额度分别校验，禁止用命令后的当前 `unawarded_open_quantity` 同时约束全部阶段：
   - `award.decide` 批准的本次正数量只能占用该命令加锁前的 `unawarded_open_quantity`，成功后同量从 unawarded 移入 currently-valid-awarded；
   - 未撤销 PO allocation 的累计数量不得超过其引用的有效已批准 AwardAllocation 数量；`purchase_order.create`、`purchase_order.submit`、`purchase_order.decide` 与 `purchase_order.issue` 均不能隐式扩大或制造 Award；
   - accepted goods receipt 的累计净数量不得超过已发出 PO allocation 的累计净数量；合法超收只能走现行收货例外审批，并保存超收批准证据，不能回写扩大 Award 或 PO。
4. 不同法人不得合并。不同物料/版本、单位不可证明换算、交付地点不兼容或 required_on 超出同一采购窗口时不得自动合并。
5. 来源事实减少或撤销时，只能减少尚未授标部分；已授标/已发 PO 必须进入影响处置或采购变更。
6. 释放必须停留在事实对应层级：`award.revoke` 只把确证未被下游不可撤销效果占用的 Award 数量释放回 `unawarded_open_quantity`；PO 作废、供应商拒单或确证短缺只释放 PO 层未执行数量，仍有效 Award 下可重新建 PO，不自动撤销 Award；收货拒收、收货冲销或退货只释放收货层对应净数量。只有 typed impact/shortfall/replacement fact 明确要求重采，并依法撤销/减量上层占用时，才重算需求层；Unknown 不释放任何层级。

### 5.3 Demand 状态机

| 状态 | 含义 |
|---|---|
| DRAFT | 尚未完成准入 |
| READY | 已校验，可不经 RFQ 进入受控直采授标，或进入询价；两条路径都必须先形成 approved Award 才能建 PO |
| SOURCING | 至少一个有效 RFQ round 正在执行 |
| PARTIALLY_AWARDED | 部分数量已有有效授标 |
| AWARDED | 至少存在一笔当前有效授标，且全部未取消数量均已授标 |
| CLOSED | 采购、收货/退货和相关经济义务满足 |
| CANCELLED | 全量在任何授标前经批准取消 |

状态不得由通用更新接口直接写入。允许边、唯一命令和门禁固定为：

| 当前状态 | 允许后继 | 唯一命令/事实与门禁 |
|---|---|---|
| DRAFT | READY、CANCELLED | `procurement_demand.admit` 要求来源 exact-one、快照、数量、单位、地点和日期校验通过；`procurement_demand.cancel` 要求历史上从未存在任何 Award/PO 事实且 maker-checker 批准 |
| READY | SOURCING、PARTIALLY_AWARDED、AWARDED、CANCELLED | `procurement_demand.start_sourcing` 要求至少一个 RFQRound 进入 OPEN；RFQ 或 DIRECT_PURCHASE 都只能由 `award.propose` 后不同主体执行 `award.decide` 形成 approved Award，再按需求层公式进入部分或全量授标；取消仍要求历史上从未存在任何 Award/PO 事实 |
| SOURCING | READY、PARTIALLY_AWARDED、AWARDED | 最后一个有效 round 以 NO_AWARD/CANCELLED/CANCELLED_BY_REVISION 终结且替代 round 尚未 OPEN 时回 READY；批准 Award 后按有效授标数量进入部分或全量授标 |
| PARTIALLY_AWARDED | READY、AWARDED | `procurement_demand.start_sourcing` 可为剩余 `unawarded_open_quantity` 新建并 OPEN round，但 Demand 仍保持 PARTIALLY_AWARDED；也可继续走 DIRECT_PURCHASE 的 `award.propose`/`award.decide`；全部有效 Award 被 `award.revoke` 合法撤销且无已发 PO 占用时回 READY；`unawarded_open_quantity` 归零时进入 AWARDED |
| AWARDED | READY、PARTIALLY_AWARDED、CLOSED | typed supplier-rejection/shortfall/return facts 与 `award.revoke` 只能按 §5.2 释放对应层级，并按剩余有效 Award 和 `unawarded_open_quantity` 回到 READY 或 PARTIALLY_AWARDED；`procurement_demand.close` 必须满足采购、收货/退货和经济义务关闭谓词 |
| CLOSED | READY、PARTIALLY_AWARDED | 仅登记表中的重开事实可执行 `procurement_demand.reopen`；先追加 `DemandReopened` 和新 cycle，再按仍有效 Award 数量确定目标 |
| CANCELLED | 无 | 终态；来源恢复时创建新 Demand 并引用旧事实，不得复活原记录 |

状态派生优先级固定为：已满足 closure predicate 才是 CLOSED；否则 `cancelled_quantity = accepted_requested_quantity`、`currently_valid_awarded_quantity = 0` 且历史上从未存在任何 Award/PO 事实时为 CANCELLED；否则 `currently_valid_awarded_quantity > 0` 且 `unawarded_open_quantity > 0` 为 PARTIALLY_AWARDED；否则 `currently_valid_awarded_quantity > 0` 且 `currently_valid_awarded_quantity + cancelled_quantity = accepted_requested_quantity` 为 AWARDED；有效授标为 0 且有 OPEN round 为 SOURCING；其余已准入记录为 READY。全量取消但历史上存在 Award/PO、取消量没有 maker-checker 事实、或数量公式不成立时不是某个可派生状态，而是非法事实组合并失败关闭。不得因同时存在 OPEN round 把已有部分授标隐藏成 SOURCING，也不得在 `Effect=Unknown` 时释放数量。重开不设置可手工置位的 REOPENED 状态；原 cycle、Award、PO 和数量事实全部保留。

### 5.4 RFQ round 状态机

RFQ 是稳定容器；RFQRound 是不可变询价版本。

| 状态 | 允许后继 | 门禁 |
|---|---|---|
| DRAFT | PENDING_APPROVAL、CANCELLED | 询价行、候选供应商、截止时间、交付和比较口径完整 |
| PENDING_APPROVAL | OPEN、DRAFT、CANCELLED | maker-checker；驳回回 DRAFT 并留历史 |
| OPEN | EVALUATING、CANCELLED、CANCELLED_BY_REVISION | 截止前收报价；发布后的内容不得原位改；只有替代 round 已成功 OPEN 才可进入 CANCELLED_BY_REVISION |
| EVALUATING | AWARDED、NO_AWARD、OPEN、CANCELLED_BY_REVISION | 截止已到；重新 OPEN 只能通过新截止变更事实并通知全部候选；只有替代 round 已成功 OPEN 才可进入 CANCELLED_BY_REVISION |
| AWARDED | 无 | 至少一个批准 Award，分配守恒 |
| NO_AWARD | 无 | 无合格报价或决定不授标；原因和证据必填 |
| CANCELLED | 无 | 尚无有效 Award；原因必填 |
| CANCELLED_BY_REVISION | 无 | 替代 `RFQRound` 已处于 OPEN；`replacement_round_id`、变更原因和全部候选通知证据必填；旧报价只读 |

OPEN 后变更物料、数量、交付、条款、候选范围或比较口径必须创建新 RFQRound。新 round 成功进入 OPEN 后，旧 round 只能终结为 CANCELLED_BY_REVISION，不得提前终结，也不得覆盖旧报价；NO_AWARD 只表示本 round 在原比较口径下决定不授标，不能代替版本换轮。

### 5.5 供应商报价版本

SupplierQuoteVersion 至少保存：

- supplier_id/supplier_version、rfq_round_id；
- 行、可供数量、单位、单价、税、净额/税额/总额、交期、有效期；
- 付款、运输、质量/资质和偏差说明；
- 收件渠道、原始文档 digest、录入人/connector、received_at；
- predecessor_version、version_no、content_digest。

状态闭集：

- RECEIVED
- VALID
- LATE_REJECTED
- DISQUALIFIED
- WITHDRAWN
- SUPERSEDED
- SELECTED
- NOT_SELECTED

状态不得手工置位；允许边和结果固定为：

| 当前状态 | 允许后继 | 门禁 |
|---|---|---|
| RECEIVED | VALID、LATE_REJECTED、DISQUALIFIED、WITHDRAWN、SUPERSEDED | `ValidateSupplierQuote` 重算截止时间、供应商资格、单位、税、金额、交付和原始证据；截止后只能 LATE_REJECTED；资格/内容失败只能 DISQUALIFIED；供应商可在任何选择前撤回；同一 supplier+round 的后继版本只有成功进入 VALID 后才可把旧 RECEIVED 版本标为 SUPERSEDED |
| VALID | DISQUALIFIED、WITHDRAWN、SUPERSEDED、SELECTED、NOT_SELECTED | 新证据使资格失败可 DISQUALIFIED；选择前可 WITHDRAWN；已验证的同 supplier+round 后继版本可 SUPERSEDED；批准 AwardAllocation 选中任意正数量即 SELECTED；round 终结且没有任何选中数量才是 NOT_SELECTED |
| LATE_REJECTED | 无 | 终态；如需考虑，只能用新 round/新版本 |
| DISQUALIFIED | 无 | 终态；修正内容必须创建新版本 |
| WITHDRAWN | 无 | 终态；恢复报价必须创建新版本 |
| SUPERSEDED | 无 | 终态；保留原始证据和 predecessor/successor 链 |
| SELECTED | 无 | 终态；Award 撤销不改写报价历史，后续处置写 Award/Demand 事实 |
| NOT_SELECTED | 无 | 终态；后续新一轮必须创建新 QuoteVersion |

同一 QuoteVersion 的部分行/部分数量被选中，版本状态也固定为 SELECTED，未选数量由 AwardAllocation 明细解释；不得把同一版本拆成互相矛盾的 SELECTED/NOT_SELECTED 状态。任何终态均不得原位恢复。

供应商报价由有权内部人员根据原始证据登记，或由通过认证的 connector 导入。POR-002 的供应商门户白名单不含在线报价，本文不扩大该白名单。

截止后收到的报价一律 LATE_REJECTED；需要考虑时必须建立新 RFQRound 或经批准延长截止且对全部候选供应商等同通知，不得只为某一供应商回拨时间。

### 5.6 评估和授标

1. 系统先执行资格门：供应商启用、资质有效、同法人、报价有效、单位可换算、税额可重算、交付可满足或已声明偏差。失败项不得进入可选集合。
2. 系统对合格报价统一展示净额、税额、总额、可供数量、承诺交期、质量/风险证据和偏差，不用未冻结的黑盒评分自动授标。
3. Award sourcing kind 只取 `RFQ|DIRECT_PURCHASE`。READY 可选 DIRECT_PURCHASE 而不创建 RFQ，但仍必须由有 procurement.award.create 能力的主体调用现有 `award.propose`，再由不同主体以现有 `award.decide` 审批；只有 APPROVED 才形成有效 Award。DIRECT_PURCHASE proposal 必须保存供应商资格、价格/条款证据、直采 reason、政策例外/风险决定和 comparison_snapshot_digest，不得把“免 RFQ”解释为“免授标”或免 maker-checker。
4. 同一 demand line 可分给多个供应商；每个 `award.decide` 只能占用加锁前 `unawarded_open_quantity`，各有效 AwardAllocation 按 §5.2 守恒。
5. 选择同等交付条件下非最低总额报价时，必须填写 exception reason 并走风险审批。并列最低价也必须明确选择并记录理由。
6. 供应商报价的部分数量可以被 AwardAllocation 选中，但不得改写报价。如果部分数量会改变阶梯价格、最低采购量、运输、税或交付条款，必须取得新的 SupplierQuoteVersion 后才能授标。
7. RFQ 和 DIRECT_PURCHASE 都只有在 Award APPROVED 后才可调用 `purchase_order.create`；随后仍须依次通过现有 `purchase_order.submit`、`purchase_order.decide` 和 `purchase_order.issue`。`purchase_order.issue` 只消费已批准 PO 和有效 Award 的剩余额度，绝不隐式授标；PO 发出仍属于 AUTH-007 高风险效果。

### 5.7 授标撤销和重开

| 事实 | 处理 |
|---|---|
| Award 批准但 PO 未发出 | 通过 `award.revoke` 追加撤销；确证无下游占用的分配回到 `unawarded_open_quantity`，RFQ 可新建 round，或重新发起 DIRECT_PURCHASE proposal |
| PO 发出被证明未执行 | 走 PO 取消/变更后只释放 PO 层未执行额度；Award 仍有效时可在其剩余额度内重建 PO，不自动回到 `unawarded_open_quantity` |
| PO 发出结果 Unknown | Demand 与 Award 保持各自原状态且数量继续占用；只有该次 PO issue `Effect` 及其 `PROCUREMENT_FULFILMENT` objective 进入 RECONCILING，禁止创建替代 PO |
| 供应商拒单或取消未履行数量 | typed fact 先释放 PO 层确证未履行数量；只有同时撤销/减量 Award 或 replacement decision 明确需要重新寻源时才重算 demand |
| 短收、拒收 | 只释放收货层最终确认缺口；需要替换时按 typed replacement/shortfall fact 逐层处置，不按整单重开 |
| 采购退货 | 只冲减对应 accepted receipt 净量；只有仍需替换的退回数量按 typed replacement fact 逐层重开，纯退款不自动重采 |
| 来源订单/项目取消 | 尚未授标部分取消；已授标部分进入影响处置 |
| 无报价或全部不合格 | RFQ 为 NO_AWARD，Demand 返回 READY 并创建异常 work item |

### 5.8 采购验收

T-F57-PROC-001 和 T-F57-PROC-003 必须至少包含：

- six_demand_sources_are_exact_one_and_idempotent
- merge_split_preserve_source_and_quantity_conservation
- demand_accepted_equals_cancelled_valid_awarded_and_unawarded_open
- fully_cancelled_never_awarded_demand_derives_cancelled_not_awarded
- fully_cancelled_after_any_award_or_po_fact_fails_closed
- rfq_round_content_is_immutable_after_open
- late_quote_requires_equal_treatment_or_new_round
- supplier_quote_versions_preserve_original_evidence
- procurement_demand_edges_commands_and_derived_precedence_are_exact
- supplier_quote_version_edges_are_closed_and_terminal_history_is_immutable
- award_requires_comparison_snapshot_and_maker_checker
- ready_direct_purchase_uses_award_propose_and_decide_without_rfq
- purchase_order_issue_cannot_implicitly_create_or_expand_award
- partial_multi_supplier_award_preserves_unawarded_open_quantity
- award_po_and_receipt_each_enforce_its_own_quantity_ceiling
- revocation_shortfall_and_return_release_only_the_corresponding_layer
- changed_partial_quantity_terms_require_new_supplier_quote_version
- po_unknown_blocks_duplicate_award_and_send
- supplier_rejection_short_receipt_and_return_reopen_exact_shortage
- supplier_portal_does_not_gain_unapproved_quote_command

## 6. 五类服务工单、权益、配件、工时和周期维保

### 6.1 投诉状态机与服务边界

投诉事实只由 service 拥有。CRM、客户 360、Workbench 和客户门户调用 RegisterComplaint 类型化命令并展示授权投影，不保存第二份投诉。

ComplaintState 只取：

- REGISTERED
- ACKNOWLEDGED
- INVESTIGATING
- ACTION_PLANNED
- WAITING_CUSTOMER
- RESOLVED
- CLOSURE_REVIEW
- CLOSED
- CANCELLED

允许转换：

| From | To | 门禁 |
|---|---|---|
| REGISTERED | ACKNOWLEDGED/CANCELLED | 受理响应证据；只有重复/误录且无处理效果时可取消，重复项必须引用 canonical complaint |
| ACKNOWLEDGED | INVESTIGATING | 分类、严重度、责任解析和响应目标完整 |
| INVESTIGATING | ACTION_PLANNED/WAITING_CUSTOMER | 调查时间线和所需客户信息明确 |
| WAITING_CUSTOMER | INVESTIGATING/ACTION_PLANNED | 收到信息或按受控无响应策略形成证据 |
| ACTION_PLANNED | RESOLVED/INVESTIGATING | 纠正动作、关联工单/CAPA 和结果证据完整；失败回调查 |
| RESOLVED | CLOSURE_REVIEW/INVESTIGATING | closure predicate 满足或证据不足退回 |
| CLOSURE_REVIEW | CLOSED/INVESTIGATING | 独立复核；原处理人不得单人关闭 MAJOR/CRITICAL |
| CLOSED | INVESTIGATING | 客户有证据争议、同因复发、CAPA 失败或关联工单重开；新 cycle |

投诉关闭要求已完成首次响应、调查、责任/原因、客户可见结论，并且所有阻断工单/CAPA 已关闭；受控无响应必须满足第 6.5 节的联系证据规则。投诉不因创建工单自动关闭，也不因 CRM 将其标记“已跟进”而关闭。

Sales 拥有交付确认，service 拥有 INSTALLATION 工单和现场执行，project 拥有项目里程碑/验收聚合。项目安装里程碑只能通过类型化命令创建/引用 INSTALLATION 工单；里程碑关闭引用 service 的已关闭安装证据和适用的 sales/customer 验收，不复制安装或交付事实。

### 6.2 工单类型闭集

WorkOrderKind 只取：

- INSTALLATION
- REPAIR
- INSPECTION
- MAINTENANCE
- TECHNICAL_SUPPORT

不能以 CUSTOM 绕过类型状态、证据或成本规则。行业扩展可在签名能力包中增加子分类，但必须映射到上述一种且不能减弱关闭谓词。

### 6.3 通用工单状态机

| 状态 | 含义 |
|---|---|
| DRAFT | 信息未完整，不进入 SLA |
| TRIAGED | 类型、优先级、客户、设备/产品和权益已初判 |
| UNASSIGNED | 等待能力型责任解析 |
| ASSIGNED | 已分配但未接受 |
| ACCEPTED | 执行者已接受责任 |
| IN_PROGRESS | 正在处理 |
| WAITING_CUSTOMER | 等客户输入/许可/确认 |
| WAITING_PART | 等配件或采购 |
| WAITING_EXTERNAL | 等外部 provider 或第三方 |
| RESOLVED | 处理动作已完成，等待证据和下游勾稽 |
| CLOSURE_REVIEW | closure predicate 已满足，等待必要的独立复核 |
| CLOSED | 本 cycle 有证据关闭 |
| CANCELLED | 尚未产生业务效果且经批准取消 |

允许转换：

- DRAFT → TRIAGED/CANCELLED
- TRIAGED → UNASSIGNED/ASSIGNED/CANCELLED
- UNASSIGNED → ASSIGNED
- ASSIGNED → ACCEPTED/UNASSIGNED
- ACCEPTED → IN_PROGRESS/UNASSIGNED
- IN_PROGRESS ↔ 三种 WAITING
- IN_PROGRESS/WAITING → RESOLVED
- RESOLVED → IN_PROGRESS/CLOSURE_REVIEW
- CLOSURE_REVIEW → CLOSED/IN_PROGRESS
- CLOSED → IN_PROGRESS，仅由本文重开事实触发并创建新 cycle

开始配件领用、工时、费用、客户签字或外部效果后不得 CANCELLED；必须关闭、补偿或形成明确未完成事故。

### 6.4 权益判定

ServiceEntitlementSource exact-one：

- SERVICE_CONTRACT_VERSION
- WARRANTY
- CHARGEABLE_APPROVAL

优先级固定为：匹配的有效服务合同 → 匹配的在保权益 → 经客户/内部批准的收费服务。前一项存在但明确不覆盖当前 kind/设备/日期时继续判断下一项。

EntitlementSnapshot 至少保存：

- customer、equipment/product、work_order_kind、service_occurred_on；
- source_kind/source_ref/source_version；
- 覆盖范围、免收费项、客户承担项、响应/解决 SLA；
- 配件、工时、差旅和第三方费用规则；
- 判定规则版本、证据和 digest。

权益按服务发生时事实冻结。后续合同或保修更正不覆盖已发生快照；如证明原判定错误，追加 ReassessEntitlement，产生收费/退款/成本调整义务并保留前后链。

### 6.5 各类型必需证据和关闭谓词

| Kind | 必需业务证据 | 类型关闭条件 |
|---|---|---|
| INSTALLATION | 来源订单/项目、设备序列或产品、安装清单、现场照片/附件、测试结果、客户签收/验收 | 全部强制清单通过；设备归属生效；客户验收确认；配件/工时/费用勾稽完成 |
| REPAIR | 故障描述、诊断、根因、维修动作、前后测试、使用配件、工时、客户确认 | 测试通过；所有配件和成本事实确认；MAJOR/CRITICAL 必须有 CAPA；客户确认或受控超时证据 |
| INSPECTION | 版本化检查表、逐项结果、异常、照片/测量证据 | 所有必填项完成；异常已解决或已转为有明确 owner 的关联 obligation；不得静默忽略异常 |
| MAINTENANCE | 周期计划/服务合同、计划时点、保养清单、使用配件/耗材、结果、下次到期 | 当前 occurrence 证据完整；异常已处置；下一 occurrence 已唯一生成，或计划已合法终止 |
| TECHNICAL_SUPPORT | 问题、沟通时间线、诊断、解决方案、验证结果 | 客户确认解决，或在通知后达到受控无响应期限并经独立复核；高严重度必须记录根因/CAPA |

客户无响应不能由执行者单人直接关闭。默认等待 7 个自然日；配置代可在 1 至 30 日内调整。到期后必须保留至少两次不同日期的联系证据，并由非原执行者复核。

### 6.6 配件

1. 服务只拥有 PartUsage 请求和工单关联；库存 owner 执行预留、领用、退回、报损和估价。
2. PartUsageState 闭集为 `REQUESTED|RESERVED|ISSUED|RETURNED|CONSUMED|SCRAPPED|CANCELLED`。允许边只为 `REQUESTED→RESERVED|CANCELLED`、`RESERVED→ISSUED|CANCELLED`、`ISSUED→RETURNED|CONSUMED|SCRAPPED`；四个终态均不可恢复。`RESERVED→CANCELLED` 必须先由库存 owner 确认释放预留；`ISSUED→RETURNED` 绑定退回库存 movement，`ISSUED→CONSUMED` 绑定安装/使用证据，`ISSUED→SCRAPPED` 绑定批准报损、库存损耗和估价事实。ISSUED 后不能删除、取消或把报损伪装为已使用。
3. 工单不得直接改库存；负库存、跨法人、停用物料、序列/批次不匹配一律拒绝。
4. 工单关闭时每个 ISSUED 配件必须已进入 CONSUMED、RETURNED、SCRAPPED，或关联阻断关闭的有证据事故，不得悬空。一个 PartUsage line 只能有一种终局；部分使用、部分退回或部分报损必须在最终转换前拆成来源守恒的多条 line，禁止用一个状态掩盖混合结果。
5. 保内不收费不等于无成本；库存估价仍进入经营服务成本。

### 6.7 工时和费用

1. TimeEntry 保存执行者、开始/结束、可信持续时间、工作类型、计费属性和证据；同一人员不得有重叠已批准工时。
2. 自动计时每人同一时刻只能有一个活动 timer。断电后按持久 checkpoint 恢复，不按客户端墙钟猜测。
3. 手工补录和修改已提交工时必须说明原因并独立审批；已批准记录只追加 correction。
4. 劳务成本使用服务发生时已批准费率快照；费率变化不重算历史。
5. Expense 保存类型、金额、税、附件和批准状态；拒绝或冲销不计入净成本。
6. service 聚合成本但不写经营分录。parts 成本来自 inventory，labor/expense 确认来自 service，经营分录由 ledger/finance 通过公开事实生成。
7. 收费服务只生成 ServiceChargeProposal；发票申请、AR、收款和退款仍由 invoice/finance 拥有。服务合同的自动周期计费引擎首发延期，合同付款计划可按现行 CLM/FIN 路径开票。

### 6.8 根因、CAPA、满意度与复发

1. 严重度只取 LOW、NORMAL、MAJOR、CRITICAL。
2. MAJOR/CRITICAL 工单关闭前必须有 RootCause 和至少一个 CorrectiveAction；PreventiveAction 无适用时必须有不适用原因。
3. 关联同客户、设备/产品、故障分类在关闭后 30 日内再次发生时，系统标记 RECURRENCE 并重开原服务 objective 或建立关联新工单；不得仅作为普通新单隐藏复发。
4. CLOSED 后创建 `CUSTOMER_SATISFACTION_FOLLOW_UP` objective。评分 1 至 5；不响应不记为 0 分。
5. 满意度请求默认开放 7 日；收到回复或到期证据后结束，不阻塞工单操作关闭，但阻塞完整服务 cycle 的最终质量统计完成。

### 6.9 周期维保

1. 首发周期类型只取 ONE_TIME 和 CALENDAR_INTERVAL；预测维护、传感器触发和复杂 EAM 继续延期。
2. recurrence_anchor 固定为服务合同/维护计划的 approved_start_at，不以实际完成日漂移。
3. occurrence_key 固定由 legal_entity、plan_version、schedule_rule_id、planned_at 组成；全局唯一，重启、重复 timer 和多 worker 不得重复生成。
4. 服务器停机跨过一个或多个到期时点后，恢复时为每个仍有效且未生成的到期时点分别补建初态即为 OVERDUE 的 occurrence，不合并、不改成 PLANNED/DUE、不静默跳过。
5. 合同或计划在 planned_at 前已暂停时不生成；终止生效后不得生成尚未创建的未来 occurrence。合同/计划在 occurrence 已生成后提前终止时，每个尚未完成 occurrence 只能进入 CANCELLED，并绑定 typed `PlanTerminationEvidence`（合同/计划版本、终止 reason、effective_at、影响处置和审批）；已开始业务效果的 occurrence 必须先完成补偿/影响处置，不能删除或伪装完成。
6. 设备停用、转移、找不到或状态与计划冲突时必须打开 typed incident，occurrence 保持其事实应有的 DUE/OVERDUE/IN_PROGRESS，不进入 COMPLETED 或另造跳过状态。
7. 当前 occurrence 进入 COMPLETED 时必须证明下一 occurrence 已存在，或计划按期结束/合法终止且有 typed `PlanTerminationEvidence`；CANCELLED 不制造维护成功事实。

### 6.10 服务重开

CLOSED 工单在以下事实发生时创建新 cycle 并回 IN_PROGRESS：

- 客户在 30 日内对同一结果提出有证据争议；
- 测试/验收证据被撤销、损坏或证明不真实；
- 配件退回、成本冲销或收费失败使原 closure predicate 不再成立；
- 同一故障分类发生 RECURRENCE；
- CAPA 失败或逾期且与本工单相关；
- 下游库存、发票、退款或签字效果从 Confirmed 变为冲突/撤销。

仅收到低评分不自动重开；低于 3 分必须创建回访异常，由有权人员判断是否构成上述重开事实。

### 6.11 服务验收

T-F57-SRV-003、T-F57-SRV-006、T-F57-SRV-008、T-F57-SRV-009、T-F57-SRV-010 必须至少包含：

- five_work_order_kinds_use_exact_common_and_type_predicates
- entitlement_exact_one_and_priority_are_deterministic
- parts_move_only_through_inventory_and_never_disappear
- labor_overlap_manual_correction_and_rate_snapshot
- major_critical_require_root_cause_and_capa
- maintenance_occurrence_is_unique_across_restart_and_downtime
- downtime_backfills_each_missed_occurrence_as_overdue
- maintenance_occurrence_has_no_skipped_state_or_edge
- early_contract_or_plan_termination_cancels_with_typed_evidence
- disabled_equipment_creates_incident_not_false_completion
- close_requires_evidence_and_reopen_preserves_prior_cycle
- charge_proposal_does_not_bypass_invoice_or_finance_owner
- customer_nonresponse_requires_timeout_evidence_and_independent_review
- complaint_channel_does_not_create_second_crm_fact
- complaint_close_requires_response_investigation_and_linked_obligations
- project_installation_milestone_references_service_and_sales_facts

## 7. 项目风险、成本和收款节点

### 7.1 项目风险对象

ProjectRisk 至少包含：

- project_id、risk_no、category、description；
- likelihood 1 至 5、impact 1 至 5、score 为两者乘积；
- impact_summary、可选 exposure_amount_minor；
- responsibility_query_id、可空 `current_assignee: PrincipalRefV1 {kind,id}`；该引用只是当前责任，不授予权限，kind 参与相等、幂等和审计；
- response_strategy：AVOID、MITIGATE、TRANSFER、ACCEPT；
- mitigation_actions、due_at、review_at；
- trigger_fact_refs、evidence_refs、row_version、generation。

项目风险状态：

| 状态 | 允许后继 | 唯一命令/门禁 |
|---|---|---|
| OPEN | MITIGATING、ACCEPTED、CLOSED | `StartRiskMitigation` 要求至少一个有责任人和期限的未完成 action；`AcceptResidualRisk` 要求不同主体审批、残余暴露和 review_at；`CloseRisk` 只允许风险已消失、已避免或已转移且证据完整 |
| MITIGATING | MONITORING、ACCEPTED、CLOSED | `EnterRiskMonitoring` 要求当前 mitigation actions 已完成且观察指标/期限完整；残余风险可独立审批为 ACCEPTED；风险消除可 CLOSED |
| MONITORING | MITIGATING、ACCEPTED、CLOSED | 触发恶化或新 action 时进入 MITIGATING；观察完成后的残余风险可独立审批为 ACCEPTED；关闭谓词满足才可 CLOSED |
| ACCEPTED | OPEN、MITIGATING、CLOSED | review 维持接受只追加 `RiskAcceptanceReviewed`、不改状态；重开触发出现时，有已批准且未完成 mitigation action 则 MITIGATING，否则 OPEN；风险确证消失/转移可 CLOSED |
| CLOSED | OPEN、MITIGATING | 只有登记的重开事实可 `ReopenRisk` 并创建新 cycle；有已批准且未完成 mitigation action 时 MITIGATING，否则 OPEN |

score 大于等于 15、CRITICAL category、已逾期或可能影响合同交付/客户验收/收款的风险必须升级；不得由创建人自己 ACCEPT。

CLOSED 或 ACCEPTED 风险在触发条件再次出现、里程碑延期、合同/项目变更、成本或收款偏差超过当前签名策略阈值时，必须按上表确定唯一目标并创建新 cycle。`response_strategy=TRANSFER` 只是处置策略，不是状态；不得用“仍在接受”绕过到期 review。SLA 和历史不重置，所有状态写入均需 row_version CAS、原因、触发事实和 evidence refs。

### 7.2 项目成本和收款节点

1. project 不写库存、服务、采购、发票或资金事实，只聚合其公开事实。
2. 项目成本等于已确认且未冲销的 inventory issue/cost、direct procurement cost、service labor/expense 和批准的其他经营成本；每项必须可下钻来源。
3. ProjectReceiptMilestone 保存合同版本/付款计划引用、触发条件、应收金额、due_on 和 acceptance evidence requirement。
4. 里程碑完成只能创建 invoice/receivable obligation，不得伪造已开票或已收款。
5. §14.6 的 `PROJECT_RECEIPT_MILESTONE_V1` 行是唯一状态 registry，本节只解释其 closure coverage，不形成第二真值。每个收款节点由 finance owner facts 唯一派生 `CANCELLED|PAID|WAIVED|INVOICED|BLOCKED|DUE|READY`，禁止状态写命令。`cash_coverage` 只累计有效到款核销；`waiver_coverage` 只累计有效 maker-checker 减免、核销或法律消灭事实；二者不得重复覆盖同一金额且累计不超过里程碑金额。closure coverage 只有三种：`cash_coverage = milestone_amount` 为 PAID；`cash_coverage + waiver_coverage = milestone_amount` 且 waiver 为正为 WAIVED；正式合同变更取消该节点，且已有发票/应收/到款效果已撤销、退款或合法处置完毕，为 CANCELLED。
6. 派生优先级固定为：合法取消 coverage 完整则 CANCELLED；否则全额 cash coverage 为 PAID；否则 cash+waiver 全覆盖为 WAIVED；否则有效已开票覆盖里程碑金额为 INVOICED；否则前置义务未满足为 BLOCKED；否则服务器业务日达到 due_on 为 DUE；否则 READY。任何时刻按这一顺序只能得到一个状态。
7. 合同变更撤销/取代取消、waiver 撤销/更正、到款核销释放/收款冲销、发票红冲或前置义务反向事实都必须按同一优先级重新派生；历史状态和原 owner facts 不改写。PAID、WAIVED、CANCELLED 因合法反向事实失去 coverage 时均可回到 INVOICED/BLOCKED/DUE/READY 等唯一结果，不是持久终态。
8. 项目关闭要求全部强制里程碑和任务结束、交付/验收证据有效、开放风险为 0、采购和服务关联义务关闭、项目成本与收款节点完成对账；每个强制 ProjectReceiptMilestone 必须为 PAID、WAIVED 或 CANCELLED，并带完整 coverage refs。
9. 项目不得自行生成 sales delivery、service installation、invoice 或 receipt 事实；它只能发出公开命令并引用这些 owner 返回的 evidence。相同外部验收文档只能有一个 canonical digest，项目、销售和服务保存引用而非各自上传三份权威副本。

### 7.3 项目验收

T-F57-PRJ-003 必须至少包含：

- risk_score_state_and_escalation_are_exact
- risk_acceptance_requires_independent_approval_and_review_date
- risk_transition_edges_and_reopen_target_are_deterministic
- project_risk_current_assignee_uses_full_principal_kind_and_id
- milestone_creates_receivable_obligation_not_fake_cash
- receipt_milestone_exact_states_priority_and_single_derivation
- receipt_milestone_paid_waived_cancelled_require_exact_closure_coverage
- receipt_milestone_reverse_facts_rederive_without_history_rewrite
- project_cost_is_source_backed_and_reversal_aware
- closed_project_reopens_on_acceptance_withdrawal_or_material_variance

## 8. 业务 ObjectiveKind 关闭与重开登记

### 8.1 通用 Objective 状态

ObjectiveState 只取：

- OPEN
- WAITING
- RECONCILING
- INCIDENT
- CLOSURE_REVIEW
- CLOSED
- ABANDONED

ABANDONED 不是任意“结束”。只有业务合同/客户决定、法律禁止、主体消失或经授权影响处置证明目标不再应完成时才可使用；必须列出未完成 obligation、影响、补偿和批准证据。财务、库存、签章或已发生外部效果不能通过 ABANDONED 消失。

状态只由 objective owner 的强类型命令和谓词重算写入，允许边固定为：

| 当前状态 | 允许后继 |
|---|---|
| OPEN | WAITING、RECONCILING、INCIDENT、CLOSURE_REVIEW、ABANDONED |
| WAITING | OPEN、RECONCILING、INCIDENT、CLOSURE_REVIEW、ABANDONED |
| RECONCILING | OPEN、WAITING、INCIDENT、CLOSURE_REVIEW、ABANDONED |
| INCIDENT | OPEN、WAITING、RECONCILING、CLOSURE_REVIEW、ABANDONED |
| CLOSURE_REVIEW | CLOSED、OPEN、WAITING、RECONCILING、INCIDENT、ABANDONED |
| CLOSED | OPEN、WAITING、RECONCILING、INCIDENT |
| ABANDONED | 无 |

每次重算按以下优先级得到唯一非终态：存在尚未裁定的外部效果 Unknown 为 RECONCILING；存在未受控的安全、数据完整性、重复效果或矛盾证据为 INCIDENT；关闭谓词全部为 true 且无前两类阻断为 CLOSURE_REVIEW；唯一剩余阻断是登记的外部等待原因为 WAITING；其他情况为 OPEN。`ApproveObjectiveClosure` 只把当前 CLOSURE_REVIEW 变为 CLOSED；复核拒绝后按同一优先级确定目标。`AbandonObjective` 只在上述合法事由、全部已发生效果已确证/补偿且批准证据完整时可从非终态进入 ABANDONED。

关闭复核通过 Task 12 的普通 `WorkItemAssignment` 分配，不引入固定岗位或 Task 23 的通用审批依赖。系统仅为 `assignment_kind=OBJECTIVE_CLOSURE_REVIEW` 的工单接受内部强类型 `DecideObjectiveClosureV1={objective_id,objective_cycle,expected_objective_version,closure_digest,decision,evidence_refs,source_work_item_id}`，其中 `decision=APPROVE|REJECT`；actor、能力、对象/法人范围、当前受理人、SoD、重新认证和配置代全部由服务端会话与分配快照重建，不能出现在该载荷中。Employee 的 `work_item.complete` 只有在工单 kind、objective/cycle/version/closure digest 与当前 CLOSURE_REVIEW 快照逐项相等时，才在同一事务 exact-once 调用该命令：APPROVE 写 CLOSED，REJECT 按上述优先级重算。结果固定为不可变 `ObjectiveClosureDecisionRecordedV1={objective_id,objective_cycle,decision,resulting_state,row_version,closure_digest,source_work_item_id,audit_entry_id}`；幂等重放返回同一结果，旧 cycle、旧 digest、非当前受理人、同人违反 SoD、跳过工单或直接写库全部拒绝。

CLOSED 只在 §8.2 登记的重开事实到达后执行 `ReopenObjective`：追加新 cycle 和 `ObjectiveReopened`，再按上述优先级进入 OPEN、WAITING、RECONCILING 或 INCIDENT；不得直接回 CLOSURE_REVIEW，也不得覆盖旧 closure digest。ABANDONED 为终态；决定改变时创建引用旧目标的新 Objective。任意未列边、直接数据库更新或把 Unknown 当 CLOSED 均拒绝。

### 8.2 当前 ObjectiveKind 登记表

| ObjectiveKind | definition_owner | 触发 | 关闭谓词 | 重开事实 |
|---|---|---|---|---|
| OPPORTUNITY_CONVERSION | crm | 商机进入 COMMERCIAL | 存在 WON + canonical successor，或 LOST/CANCELLED 有完整证据 | successor 在不可逆履约前失效；LOST/CANCELLED 经正式 Reopen |
| QUOTE_RESOLUTION | cpq | QuoteVersion 进入 ISSUE_PENDING | ISSUED 后 ACCEPTED/REJECTED/EXPIRED/WITHDRAWN/SUPERSEDED 之一有证据；Unknown 不关闭 | 不重开已终结版本；需要新版本或新 Quote。target 失效重开商机而非篡改报价 |
| CONTRACT_FULFILMENT | clm | 合同生效 | 当前合同版本全部交付、收付款、服务和其他义务满足/合法补偿；终止影响清单为零 | 合同变更新增义务、退货/退款、验收撤销、付款冲销、服务义务失败 |
| SALES_ORDER_FULFILMENT | sales | STANDARD 订单释放 | 全部行交付/取消数量守恒；验收、退换、开票/AR 和订单义务满足；无未知效果 | 拒收、退货/换货、交付撤销、发票/收款冲销或库存证明失效 |
| DROP_SHIP_FULFILMENT | sales | DROP_SHIP 订单释放 | 供应商发运、客户签收/验收、销售发票/应收和采购应付链均勾稽 | 客户拒收/退货、供应商发运撤销、任一发票/资金效果冲销 |
| PROCUREMENT_FULFILMENT | procure | Demand READY | 需求数量全部关闭；PO、收货/退货、采购发票/AP/付款义务勾稽；无 Unknown | Award/PO 撤销、供应商拒单、短拒收、需替换的采购退货、付款/发票冲销 |
| RECEIVABLE_COLLECTION | finance | 应收或收款计划到期/生效 | 有效应收由已确认核销、有效贷项/冲销或批准减免完全覆盖 | 收款冲正、退款、发票重开、核销释放、金额更正 |

CTC-01 不改变上述采购关闭谓词。其 exact 主链只关闭 `CONTRACT_FULFILMENT`、`SALES_ORDER_FULFILMENT`、`RECEIVABLE_COLLECTION`；`PROCUREMENT_FULFILMENT` 必须保持 `WAITING`，blocking obligation 固定为 `PURCHASE_AP_CLOSED`，并保存类型化 `ProcurementSettlementGapV1={purchase_invoice_recorded:false,payable_recognized:false,supplier_payment_settled:false}`。这三个字段来自相应 owner 的已提交事实，不接受客户端或自动化自报。G5 形成采购发票、应付和供应商付款事实后，三个字段全为 true，才允许按同一 closure rule 推进；任一冲销会重新变 false 并按登记规则重开。
| RETURN_REFUND_CLOSURE | sales | 退货/退款获批 | 实物去向、库存/成本、发票红冲、应收释放和退款全部按适用分支完成 | 数量差异、退货拒收、红冲/退款失败或冲销、客户争议 |
| CUSTOMER_COMPLAINT_RESOLUTION | service | 投诉受理 | 响应、责任、关联工单/CAPA、客户结论或受控无响应证据完成 | 客户有证据争议、同因复发、CAPA 失败、关联工单重开 |
| SERVICE_WORK_ORDER_CLOSURE | service | 工单 TRIAGED | 通用谓词 + 对应 kind 谓词 + 成本/配件/证据勾稽 | 第 6.10 节任一事实 |
| CUSTOMER_SATISFACTION_FOLLOW_UP | service | 服务工单 CLOSED | 收到 1–5 分评分并保存可信提交证据，或 7 日窗口到期并保存受控无响应证据；不阻塞工单操作关闭 | 原评分/无响应证据失效、客户证明提交归属错误，或关联工单重开；创建新 cycle，不改写旧评分 |
| PERIODIC_MAINTENANCE_CYCLE | service | 维护计划生效或到期 | 当前 occurrence 有证据关闭且下一 occurrence 唯一生成，或计划合法结束 | occurrence 证据失效、设备/合同影响导致当前期未完成；未来期用新 occurrence |
| PROJECT_DELIVERY_ACCEPTANCE | project | 项目 ACTIVE | 里程碑/任务/验收/采购/服务/风险/成本/收款节点全部满足 | 验收撤销、项目/合同变更、新风险、成本/收款实质差异 |
| CONTRACT_RENEWAL | clm | 进入续签窗口 | 新合同/版本生效，或经批准不续签且存量义务处置完成 | 窗口内客户决定改变、续签版本失效；原合同履约另行保持 |
| SUPPLIER_RETURN_RECOVERY | procure | 采购退货确认 | 退货发运/接收、库存/成本、进项发票/AP 和返款/替换需求全部闭合 | 供应商拒收、返款冲销、替换采购失败、数量差异 |

表中的 `definition_owner` 只表示 Objective 定义由哪个业务 feature 维护；Objective 实例、cycle、obligation、effect、evidence 和 closure 的权威写 owner 始终是平台机制 `platform.flow`，`automation` 只是禁止用于所有权判断的历史模块/数据库别名。业务 feature 只能通过公开 fact/command 触发或影响 Objective，禁止各域自行建立第二套 Objective 表。

局部业务不变量 `SALES_PRIMARY_FULFILMENT_XOR`：销售订单在首次 RELEASE 前冻结 `sales_type`，RELEASE 后不得在 `STANDARD` 与 `DROP_SHIP` 间原位转换。一次 canonical release 必须且只能产生一个主履约族：

- `sales_type=STANDARD`：恰写一条 payload 中 `sales_type` 为 `STANDARD` 的 `SALES_ORDER_RELEASED`，且只创建 `SALES_ORDER_FULFILMENT`；
- `sales_type=DROP_SHIP`：恰写一条 payload 中 `sales_type` 为 `DROP_SHIP` 的 `DROP_SHIP_ORDER_RELEASED`，且只创建 `DROP_SHIP_FULFILMENT`。

两类 release fact、ObjectiveKind、obligation、effect、evidence 和 reopen fact 不得交叉消费。`platform.flow` 的权威 store（物理数据库别名可为 `automation`）必须以 `(legal_entity_id, sales_order_id, objective_family='PRIMARY_ORDER_FULFILMENT')` 保证订单生命周期内恰有一个 canonical 主履约 Objective；重放返回同一 Objective，重开只追加同 kind 的新 cycle。物理别名不得进入 CapabilityGraph owner、权限判断或事实所有权。并发释放、响应丢失和重复 Outbox delivery 必须证明最终仍恰有一个 release fact 和一个主履约 Objective。

#### 8.2.0 机器可编译的触发、关闭与重开登记

上表只供人阅读；实现、`f57check` 和 `ClosureRegistry` 的唯一机器输入是下表，不得从中文句子分词或临场命名。除 `[]` 表示空数组外，所有 `|` 分隔项都是 ASCII upper-snake token，按字节序排序去重后进入签名 generation；runtime export 必须 exact-match，未知 token、prose-only cell、空白别名或大小写变体均拒绝。

<!-- F57-SEMANTIC-TABLE:objective_trigger_closure_registry_v1:BEGIN -->
| ObjectiveKind | trigger_kinds exact-set | closure_rule_id | reopen_trigger_kinds exact-set | timeout_policy_id | termination_policy_id |
|---|---|---|---|---|---|
| OPPORTUNITY_CONVERSION | OPPORTUNITY_ENTERED_COMMERCIAL | OPPORTUNITY_DECISION_AND_SUCCESSOR_V1 | OPPORTUNITY_FORMALLY_REOPENED\|OPPORTUNITY_SUCCESSOR_INVALIDATED_BEFORE_IRREVERSIBLE_FULFILMENT | OPPORTUNITY_DECISION_TIMEOUT_V1 | OPPORTUNITY_TERMINATION_V1 |
| QUOTE_RESOLUTION | QUOTE_VERSION_ENTERED_ISSUE_PENDING | QUOTE_VERSION_TERMINAL_DISPOSITION_V1 | [] | QUOTE_VALID_UNTIL_V1 | QUOTE_TERMINATION_V1 |
| CONTRACT_FULFILMENT | CONTRACT_BECAME_EFFECTIVE | CONTRACT_OBLIGATION_COVERAGE_V1 | CONTRACT_ACCEPTANCE_WITHDRAWN\|CONTRACT_CHANGE_ADDED_OBLIGATION\|CONTRACT_PAYMENT_REVERSED\|CONTRACT_RETURN_OR_REFUND_OPENED\|CONTRACT_SERVICE_OBLIGATION_FAILED | CONTRACT_OBLIGATION_DUE_V1 | CONTRACT_TERMINATION_V1 |
| SALES_ORDER_FULFILMENT | SALES_ORDER_RELEASED | SALES_ORDER_QUANTITY_AND_OBLIGATION_COVERAGE_V1 | SALES_CUSTOMER_REJECTED_DELIVERY\|SALES_DELIVERY_REVOKED\|SALES_INVENTORY_EVIDENCE_INVALIDATED\|SALES_INVOICE_OR_RECEIPT_REVERSED\|SALES_RETURN_OR_EXCHANGE_OPENED | SALES_DELIVERY_TIMEOUT_V1 | SALES_ORDER_TERMINATION_V1 |
| DROP_SHIP_FULFILMENT | DROP_SHIP_ORDER_RELEASED | DROP_SHIP_END_TO_END_COVERAGE_V1 | DROP_SHIP_AP_OR_AR_REVERSED\|DROP_SHIP_CUSTOMER_REJECTED_OR_RETURNED\|DROP_SHIP_SUPPLIER_SHIPMENT_REVOKED | DROP_SHIP_DELIVERY_TIMEOUT_V1 | DROP_SHIP_TERMINATION_V1 |
| PROCUREMENT_FULFILMENT | PROCUREMENT_DEMAND_READY | PROCUREMENT_QUANTITY_AND_OBLIGATION_COVERAGE_V1 | PROCUREMENT_AWARD_OR_PO_REVOKED\|PROCUREMENT_INVOICE_OR_PAYMENT_REVERSED\|PROCUREMENT_REPLACEMENT_REQUIRED\|PROCUREMENT_SUPPLIER_REJECTED_ORDER | PROCUREMENT_LEAD_TIME_V1 | PROCUREMENT_TERMINATION_V1 |
| RECEIVABLE_COLLECTION | RECEIVABLE_BECAME_DUE_OR_EFFECTIVE | RECEIVABLE_FULL_COVERAGE_V1 | RECEIVABLE_ALLOCATION_RELEASED\|RECEIVABLE_AMOUNT_CORRECTED\|RECEIVABLE_INVOICE_REOPENED\|RECEIVABLE_PAYMENT_REVERSED\|RECEIVABLE_REFUND_OPENED | RECEIVABLE_DUNNING_SCHEDULE_V1 | RECEIVABLE_NO_ABANDONMENT_V1 |
| RETURN_REFUND_CLOSURE | RETURN_REFUND_APPROVED | RETURN_REFUND_BRANCH_COVERAGE_V1 | RETURN_CREDIT_OR_REFUND_FAILED_OR_REVERSED\|RETURN_CUSTOMER_DISPUTED\|RETURN_QUANTITY_VARIANCE_FOUND\|RETURN_RECEIPT_REJECTED | RETURN_COMPLETION_TIMEOUT_V1 | RETURN_REFUND_TERMINATION_V1 |
| CUSTOMER_COMPLAINT_RESOLUTION | CUSTOMER_COMPLAINT_ACCEPTED | COMPLAINT_RESPONSE_INVESTIGATION_AND_REMEDIATION_V1 | COMPLAINT_CAPA_FAILED\|COMPLAINT_CUSTOMER_EVIDENCE_DISPUTE\|COMPLAINT_RELATED_WORK_ORDER_REOPENED\|COMPLAINT_SAME_CAUSE_RECURRED | COMPLAINT_SEVERITY_SLA_V1 | COMPLAINT_TERMINATION_V1 |
| SERVICE_WORK_ORDER_CLOSURE | SERVICE_WORK_ORDER_TRIAGED | SERVICE_COMMON_AND_KIND_PREDICATES_V1 | SERVICE_CAPA_FAILED_OR_OVERDUE\|SERVICE_COST_OR_CHARGE_EFFECT_INVALIDATED\|SERVICE_CUSTOMER_RESULT_DISPUTED\|SERVICE_DOWNSTREAM_EFFECT_CONFLICTED_OR_REVOKED\|SERVICE_EVIDENCE_INVALIDATED\|SERVICE_FAULT_RECURRED | SERVICE_ENTITLEMENT_SLA_V1 | SERVICE_WORK_ORDER_TERMINATION_V1 |
| CUSTOMER_SATISFACTION_FOLLOW_UP | SERVICE_WORK_ORDER_CLOSED | SATISFACTION_RATING_OR_CONTROLLED_NO_RESPONSE_V1 | SATISFACTION_EVIDENCE_INVALIDATED\|SATISFACTION_RELATED_WORK_ORDER_REOPENED\|SATISFACTION_SUBMISSION_ATTRIBUTION_CORRECTED | SATISFACTION_WINDOW_V1 | SATISFACTION_TERMINATION_V1 |
| PERIODIC_MAINTENANCE_CYCLE | MAINTENANCE_OCCURRENCE_DUE\|MAINTENANCE_PLAN_BECAME_EFFECTIVE | MAINTENANCE_OCCURRENCE_AND_SUCCESSOR_V1 | MAINTENANCE_CURRENT_OCCURRENCE_IMPACT_UNRESOLVED\|MAINTENANCE_OCCURRENCE_EVIDENCE_INVALIDATED | MAINTENANCE_GRACE_V1 | MAINTENANCE_TERMINATION_V1 |
| PROJECT_DELIVERY_ACCEPTANCE | PROJECT_BECAME_ACTIVE | PROJECT_DELIVERY_FULL_COVERAGE_V1 | PROJECT_ACCEPTANCE_WITHDRAWN\|PROJECT_CONTRACT_OR_PROJECT_CHANGED\|PROJECT_COST_OR_RECEIPT_MATERIAL_VARIANCE\|PROJECT_NEW_RISK_OPENED | PROJECT_EARLIEST_DUE_V1 | PROJECT_TERMINATION_V1 |
| CONTRACT_RENEWAL | CONTRACT_RENEWAL_WINDOW_OPENED | CONTRACT_RENEWAL_DECISION_AND_SUCCESSOR_V1 | CONTRACT_RENEWAL_CUSTOMER_DECISION_CHANGED\|CONTRACT_RENEWAL_SUCCESSOR_INVALIDATED | CONTRACT_RENEWAL_WINDOW_V1 | CONTRACT_RENEWAL_TERMINATION_V1 |
| SUPPLIER_RETURN_RECOVERY | SUPPLIER_RETURN_CONFIRMED | SUPPLIER_RETURN_FULL_RECOVERY_V1 | SUPPLIER_RETURN_QUANTITY_VARIANCE_FOUND\|SUPPLIER_RETURN_REFUND_REVERSED\|SUPPLIER_RETURN_REPLACEMENT_FAILED\|SUPPLIER_RETURN_SUPPLIER_REJECTED_RECEIPT | SUPPLIER_RETURN_TIMEOUT_V1 | SUPPLIER_RETURN_TERMINATION_V1 |
<!-- F57-SEMANTIC-TABLE:objective_trigger_closure_registry_v1:END -->

`ClosureRule`、`TriggerKind` 与 reopen `TriggerKind` 只能由这张表生成；`QUOTE_RESOLUTION` 的 reopen 数组恰为空，终结版本只能新建版本/Quote，不得造 `NO_REOPEN` 假 token。每个 trigger/reopen fact 的 payload schema 由对应 owner 的 typed fact catalog 提供并绑定 evidence digest；缺 schema 或不能 exact-join owner fact 时该 generation 不得激活。

### 8.2.1 每种 Objective 的执行合同 exact registry

下表是 `ClosureRegistryV1` 的唯一输入，不允许实现方增加 `CUSTOM` obligation/effect/evidence。每个实例的 `responsibility_query` 都必须是 §11 的 `CandidateQuery`，固定绑定表中 capability、subject 的 legal_entity/object scope、当前 grant/device、SoD exclusions 和 due_at；查不到人进入 `ESCALATED_NO_CANDIDATE`，不得回退到固定岗位。表内用 `|` 分隔的 token 是闭集；只有标明“按适用分支”的 obligation 可由签名分支谓词记录 `NOT_APPLICABLE`，其余必须生成并闭合。

<!-- F57-SEMANTIC-TABLE:objective_execution_registry_v1:BEGIN -->
| ObjectiveKind | obligations exact-set | responsibility capability | permitted typed effect intents | evidence exact-set |
|---|---|---|---|---|
| OPPORTUNITY_CONVERSION | COMMERCIAL_INPUT_COMPLETE\|DECISION_RECORDED\|CANONICAL_SUCCESSOR_LINKED | crm.opportunity.convert | ISSUE_QUOTE_REQUEST\|CREATE_CONTRACT_DRAFT_REQUEST\|CREATE_SALES_ORDER_DRAFT_REQUEST\|RECORD_LOSS_OR_CANCEL | OpportunitySnapshot\|CommercialValidation\|OpportunityDecision\|CanonicalSuccessorRef |
| QUOTE_RESOLUTION | ISSUE_OUTCOME\|CUSTOMER_DISPOSITION\|VERSION_TERMINATION | cpq.quote.resolve | ISSUE_QUOTE_VERSION\|RECORD_QUOTE_ACCEPTANCE\|RECORD_QUOTE_REJECTION\|WITHDRAW_QUOTE_VERSION\|SUPERSEDE_QUOTE_VERSION | QuoteVersionSnapshot\|QuoteIssueReceipt\|CustomerDispositionEvidence\|TrustedTimeEvidence\|SupersessionRef |
| CONTRACT_FULFILMENT | VERSION_OBLIGATIONS_REGISTERED\|DELIVERY_COVERAGE\|BILLING_COVERAGE\|COLLECTION_COVERAGE\|SERVICE_COVERAGE\|TERMINATION_IMPACT_ZERO | clm.contract.fulfil | RELEASE_SALES_ORDER\|CREATE_PROCUREMENT_DEMAND\|CREATE_SERVICE_WORK\|REQUEST_INVOICE\|REQUEST_COLLECTION\|APPLY_CONTRACT_CHANGE\|TERMINATE_CONTRACT | ContractVersion\|ObligationLedger\|DeliveryFact\|InvoiceFact\|CashAllocationFact\|ServiceClosureFact\|TerminationImpactDecision |
| SALES_ORDER_FULFILMENT | LINE_QUANTITY_CONSERVED\|DELIVERY_ACCEPTED\|RETURN_EXCHANGE_CLOSED\|INVOICE_AR_CLOSED\|SERVICE_OBLIGATION_CLOSED | sales.order.fulfil | ALLOCATE_STOCK\|CONFIRM_DELIVERY\|AUTHORIZE_RETURN\|REQUEST_SALES_INVOICE\|REQUEST_COLLECTION\|CREATE_SERVICE_WORK | SalesOrderSnapshot\|InventoryMovement\|DeliveryAcceptance\|ReturnClosure\|SalesInvoiceFact\|ReceivableCoverage\|ServiceClosureFact |
| DROP_SHIP_FULFILMENT | SUPPLIER_ORDERED\|SUPPLIER_SHIPMENT_CONFIRMED\|CUSTOMER_DELIVERY_ACCEPTED\|SALES_AR_CLOSED\|PURCHASE_AP_CLOSED\|RETURN_CHAIN_CLOSED | sales.drop_ship.fulfil | ISSUE_PURCHASE_ORDER\|RECORD_SUPPLIER_SHIPMENT\|CONFIRM_CUSTOMER_DELIVERY\|REQUEST_SALES_INVOICE\|REQUEST_PURCHASE_INVOICE\|REQUEST_RECEIVABLE_COLLECTION\|REQUEST_PAYABLE_SETTLEMENT\|AUTHORIZE_DROP_SHIP_RETURN | DropShipOrderSnapshot\|PurchaseOrderFact\|SupplierShipmentEvidence\|CustomerDeliveryEvidence\|SalesInvoiceFact\|PurchaseInvoiceFact\|CashCoverage\|ReturnClosure |
| PROCUREMENT_FULFILMENT | DEMAND_QUANTITY_CONSERVED\|SOURCING_RESOLVED\|PO_RESOLVED\|RECEIPT_RETURN_CLOSED\|PURCHASE_AP_CLOSED | procure.demand.fulfil | OPEN_RFQ_ROUND\|APPROVE_AWARD\|ISSUE_PURCHASE_ORDER\|RECORD_RECEIPT_OR_RETURN\|REQUEST_PURCHASE_INVOICE\|REQUEST_PAYABLE_SETTLEMENT | ProcurementDemandSnapshot\|RFQDecision\|AwardSnapshot\|PurchaseOrderFact\|InventoryReceiptReturnFact\|PurchaseInvoiceFact\|PayableCoverage |
| RECEIVABLE_COLLECTION | RECEIVABLE_IDENTIFIED\|DUE_TRACKED\|COVERAGE_COMPLETE\|REVERSAL_RESOLVED | finance.receivable.collect | ISSUE_DUNNING_NOTICE\|APPLY_CASH\|REQUEST_CREDIT_MEMO\|APPROVE_WAIVER_OR_WRITEOFF\|INITIATE_REFUND | ReceivableFact\|DueSchedule\|DunningReceipt\|CashAllocationFact\|CreditOrWriteoffFact\|ReversalFact |
| RETURN_REFUND_CLOSURE | RETURN_AUTHORIZED\|PHYSICAL_DISPOSITION\|INVENTORY_COST_REVERSED\|INVOICE_AR_RELEASED\|REFUND_RESOLVED | sales.return.fulfil | AUTHORIZE_RETURN\|RECEIVE_RETURN\|RESTOCK_OR_DISPOSE\|REQUEST_CREDIT_NOTE\|RELEASE_RECEIVABLE\|EXECUTE_REFUND | ReturnAuthorization\|ReturnMovement\|CostReversalFact\|CreditNoteFact\|ReceivableRelease\|RefundFact |
| CUSTOMER_COMPLAINT_RESOLUTION | RESPONSE_SENT\|INVESTIGATION_COMPLETE\|RESPONSIBILITY_DECIDED\|WORK_ORDER_CAPA_CLOSED\|CUSTOMER_CONCLUSION_RECORDED | service.complaint.resolve | CREATE_SERVICE_WORK_ORDER\|CREATE_CAPA\|SEND_CUSTOMER_RESPONSE\|ESCALATE_INCIDENT | ComplaintSnapshot\|ResponseReceipt\|InvestigationEvidence\|ResponsibilityDecision\|WorkOrderClosure\|CapaClosure\|CustomerConclusionOrNoResponse |
| SERVICE_WORK_ORDER_CLOSURE | ENTITLEMENT_FROZEN\|KIND_PREDICATE_MET\|PARTS_LABOR_EXPENSE_CLOSED\|CUSTOMER_EVIDENCE_RESOLVED\|CAPA_CLOSED_IF_REQUIRED | service.work_order.fulfil | RESERVE_OR_ISSUE_PART\|RECORD_LABOR_EXPENSE\|REQUEST_CUSTOMER_ACCEPTANCE\|CREATE_CHARGE_PROPOSAL\|CREATE_CAPA | WorkOrderSnapshot\|EntitlementSnapshot\|KindEvidenceBundle\|InventoryMovement\|LaborExpenseFact\|CustomerAcceptanceOrNoResponse\|CapaClosure |
| CUSTOMER_SATISFACTION_FOLLOW_UP | SURVEY_DELIVERY_ATTEMPTED\|RATING_OR_CONTROLLED_NO_RESPONSE\|LOW_SCORE_EXCEPTION_RESOLVED | service.satisfaction.follow_up | SEND_SURVEY\|RECORD_RATING\|RECORD_CONTROLLED_NO_RESPONSE\|CREATE_FOLLOW_UP_EXCEPTION | SurveyDeliveryReceipt\|RatingEvidence\|TrustedTimeEvidence\|ContactAttemptEvidence\|FollowUpExceptionClosure |
| PERIODIC_MAINTENANCE_CYCLE | OCCURRENCE_UNIQUE\|WORK_ORDER_RESOLVED\|NEXT_OCCURRENCE_OR_PLAN_END | service.maintenance.fulfil | CREATE_MAINTENANCE_OCCURRENCE\|CREATE_MAINTENANCE_WORK_ORDER\|SCHEDULE_NEXT_OCCURRENCE\|TERMINATE_MAINTENANCE_PLAN | MaintenancePlanVersion\|OccurrenceUniquenessEvidence\|WorkOrderClosure\|NextOccurrenceRef\|PlanTerminationEvidence |
| PROJECT_DELIVERY_ACCEPTANCE | MILESTONES_CLOSED\|TASKS_CLOSED\|ACCEPTANCE_VALID\|PROCUREMENT_SERVICE_CLOSED\|RISKS_ZERO\|COST_RECONCILED\|RECEIPT_MILESTONES_CLOSED | project.delivery.fulfil | CREATE_PROJECT_TASK\|REQUEST_PROCUREMENT\|REQUEST_SERVICE_INSTALLATION\|REQUEST_INVOICE\|REQUEST_RISK_ACTION | ProjectVersion\|TaskClosureSet\|MilestoneClosureSet\|AcceptanceEvidence\|ProcurementServiceCoverage\|RiskSnapshot\|CostReconciliation\|ReceiptMilestoneCoverage |
| CONTRACT_RENEWAL | RENEWAL_DECISION_RECORDED\|SUCCESSOR_EFFECTIVE_OR_NO_RENEWAL_APPROVED\|EXISTING_OBLIGATIONS_DISPOSED | clm.contract.renew | CREATE_RENEWAL_QUOTE\|CREATE_CONTRACT_VERSION\|RECORD_NO_RENEWAL_DECISION | RenewalWindowEvidence\|CustomerDecision\|SuccessorContractVersion\|NoRenewalApproval\|ExistingObligationDisposition |
| SUPPLIER_RETURN_RECOVERY | RETURN_AUTHORIZED\|RETURN_SHIPPED_RECEIVED\|INVENTORY_COST_CLOSED\|PURCHASE_AP_CLOSED\|REFUND_OR_REPLACEMENT_CLOSED | procure.supplier_return.fulfil | AUTHORIZE_SUPPLIER_RETURN\|SHIP_SUPPLIER_RETURN\|REQUEST_PURCHASE_CREDIT\|REQUEST_SUPPLIER_REFUND\|CREATE_REPLACEMENT_DEMAND | SupplierReturnAuthorization\|ReturnShipmentReceipt\|SupplierReceiptOrDispute\|InventoryCostFact\|PurchaseCreditFact\|RefundFact\|ReplacementDemandClosure |
<!-- F57-SEMANTIC-TABLE:objective_execution_registry_v1:END -->

本表 `evidence exact-set` 的 CamelCase 项是 strict `SchemaRefV1`，表示关闭计算必须加载的 evidence payload schema；它们不是 `E-F57-*` 交付证据编号，也不得塞入 `EvidenceIdV1`。CapabilityGraph Objective summary 对应字段固定为 `closure_evidence_schemas`。G0 的 `OBJECTIVE_EXECUTION_REGISTRY_V1` adapter 必须用 `SCHEMA_REF_SET` 解码、排序并逐项 exact-resolve；把这些值当普通字符串、UpperToken 或交付 EvidenceID 均失败关闭。

Registry 分支解释不得制造新命令或新 evidence owner：`PROCUREMENT_FULFILMENT` 的 `AwardSnapshot` 对 RFQ 与 DIRECT_PURCHASE 都必需；既有 `RFQDecision` 类型必须以其签名 sourcing-path 判别记录 RFQ round 结果，或记录 READY 上 DIRECT_PURCHASE 的“RFQ 不适用”决定、理由和 `award.propose`/`award.decide` refs，不能用缺失 evidence 表示直采。`PERIODIC_MAINTENANCE_CYCLE` 的计划/合同提前终止分支必须以既有 `PlanTerminationEvidence` 覆盖每个 CANCELLED occurrence，不能追加不存在的跳过 token。`PROJECT_DELIVERY_ACCEPTANCE` 的既有 `ReceiptMilestoneCoverage` 必须携带 §14.6 唯一派生状态、coverage amount/refs、优先级命中项及全部有效反向事实 refs；项目不得复制 finance facts。表中 effect intent 仍只是 owner-command 意图登记，不是另一组公开 API discriminator。

所有 effect intent 只调用相应 owner 的公开命令；objective engine 不得直接写销售、库存、发票、资金、服务或项目事实。

`EffectState` 的 wire/SQL 闭集精确为 `PREPARED|DISPATCHED|UNKNOWN|CONFIRMED|FAILED_NOT_EXECUTED|COMPENSATED|CONFLICTED`，Rust variants 精确为 `Prepared|Dispatched|Unknown|Confirmed|FailedNotExecuted|Compensated|Conflicted`。允许边和唯一语义如下；未列边全部拒绝：

| 当前状态 | 允许后继 | 唯一门禁 |
|---|---|---|
| PREPARED | DISPATCHED、FAILED_NOT_EXECUTED | dispatch lease/intent 已持久化后才可 DISPATCHED；只有可验证“从未开始 dispatch”的取消证据才可 FAILED_NOT_EXECUTED |
| DISPATCHED | CONFIRMED、FAILED_NOT_EXECUTED、UNKNOWN | provider 的确定成功回执→CONFIRMED；确定拒绝且证明零效果→FAILED_NOT_EXECUTED；超时、连接中断、响应丢失或歧义→UNKNOWN |
| UNKNOWN | CONFIRMED、FAILED_NOT_EXECUTED、COMPENSATED、CONFLICTED | 只能由 §9 的双人决定和独立证据驱动前三个结果；同一判定窗口已有相反独立证据时→CONFLICTED |
| CONFIRMED | COMPENSATED、CONFLICTED | owner 的已确认补偿事实→COMPENSATED；任何相反/重复/错对象证据→CONFLICTED |
| FAILED_NOT_EXECUTED | CONFLICTED | 迟到成功、外部副作用或重复执行证据→CONFLICTED |
| COMPENSATED | CONFLICTED | 迟到原效果、补偿撤销/失效或重复效果证据→CONFLICTED |
| CONFLICTED | 无 | 终态；只能新建引用原 effect 的恢复/补偿 effect，不得改写冲突证据 |

`CONFLICTED` 是唯一不可恢复终态；`FAILED_NOT_EXECUTED` 与 `COMPENSATED` 是正常业务终结结果，但仍保留且只保留一条由迟到相反证据触发的 `→CONFLICTED` 安全边；`CONFIRMED` 只能被 owner 已确认补偿或冲突证据推进。reconciliation 是 Objective 的 `RECONCILING` 状态和独立的 reconciliation-attempt 事实，不是 EffectState；查询期间 effect 保持 UNKNOWN。补偿调用本身必须使用新的 effect_id 并走同一状态图；其结果 Unknown 时，原 effect 保持 CONFIRMED/UNKNOWN，不得提前标成 COMPENSATED。旧 `RECONCILING` effect token、`FAILED_NO_EFFECT`/`FailedNoEffect` 及任何别名必须由 parser 和数据库约束拒绝。任何 UNKNOWN 或 CONFLICTED 都不满足 obligation。

### 8.2.2 Timeout、补偿和授权终止 exact registry

`ObjectivePolicyV1` 为签名 generation 数据，必须逐 kind 保存下表 policy ID、默认值、允许范围、时区/日历和 escalation query；缺失或越界时该 kind 不得激活。合同/订单/权益自身给出更早的 due_at 时取更早者。超时只能执行表中动作，绝不自动制造业务成功、自动付款、自动签收或自动核销。

| ObjectiveKind | timeout contract | compensation command exact-set | authorized termination |
|---|---|---|---|
| OPPORTUNITY_CONVERSION | `OPPORTUNITY_DECISION` 默认 30 日、范围 1–180 日；超时升级，不自动 LOST | WITHDRAW_ISSUED_QUOTE\|CANCEL_UNEFFECTED_DRAFT\|MARK_DUPLICATE_SUCCESSOR | CUSTOMER_DECLINED\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION；`crm.opportunity.terminate` + maker-checker |
| QUOTE_RESOLUTION | `valid_until` 是唯一到期点；可信时间到达后 EXPIRED，新条件建新版本 | WITHDRAW_QUOTE_VERSION\|SUPERSEDE_QUOTE_VERSION\|CORRECT_SUCCESSOR_LINK | CUSTOMER_DECLINED\|SUPERSEDED\|LEGAL_PROHIBITION；`cpq.quote.terminate`；已签发版本不可删除 |
| CONTRACT_FULFILMENT | 每个 ContractObligation.due_at；超时创建违约/风险升级，不自动终止 | APPLY_CONTRACT_CHANGE\|CANCEL_UNEXECUTED_DOWNSTREAM\|REQUEST_RETURN_CREDIT_REFUND\|CREATE_TERMINATION_IMPACT | CONTRACT_TERMINATED\|LEGAL_PROHIBITION\|PARTY_LEGAL_TERMINATION；`clm.contract.terminate` + 独立审批，影响清单必须为零 |
| SALES_ORDER_FULFILMENT | `promised_at + SALES_DELIVERY_GRACE`，默认 1 日、范围 0–30 日；超时升级 | CANCEL_UNEXECUTED_DELIVERY\|AUTHORIZE_RETURN_EXCHANGE\|REQUEST_CREDIT_REFUND\|REVERSE_INVENTORY_BY_OWNER | CUSTOMER_CANCELLED\|CONTRACT_TERMINATED\|LEGAL_PROHIBITION；`sales.order.cancel`；有外部效果时只能补偿后结束 |
| DROP_SHIP_FULFILMENT | 客户 promised_at 与供应商 committed_at 取更早者，grace 同上 | CANCEL_UNEXECUTED_PO\|AUTHORIZE_CUSTOMER_AND_SUPPLIER_RETURN\|REQUEST_CREDIT_REFUND\|RECONCILE_AP_AR | CUSTOMER_CANCELLED\|CONTRACT_TERMINATED\|LEGAL_PROHIBITION；`sales.drop_ship.cancel` + procurement/finance impact approval |
| PROCUREMENT_FULFILMENT | `required_on - signed_lead_time`；lead time 0–365 日；错过时升级，不自动授标 | REVOKE_UNISSUED_AWARD\|CANCEL_CONFIRMED_UNEXECUTED_PO\|RETURN_TO_SUPPLIER\|REQUEST_PURCHASE_CREDIT_REFUND | SOURCE_CANCELLED\|DEMAND_NO_LONGER_REQUIRED\|LEGAL_PROHIBITION；`procure.demand.cancel`，已发 PO 必须先处置 |
| RECEIVABLE_COLLECTION | receivable.due_on；dunning 默认 0/7/30 日、每点范围 0–180 日 | REVERSE_BAD_ALLOCATION\|ISSUE_CREDIT_OR_WRITEOFF\|EXECUTE_REFUND\|REOPEN_RECEIVABLE | ABANDONED 禁止；仅 SOURCE_REVERSED\|APPROVED_WAIVER\|LEGAL_EXTINGUISHMENT 作为 closure coverage，由 `finance.receivable.adjust` maker-checker 执行 |
| RETURN_REFUND_CLOSURE | `RETURN_COMPLETION` 默认 14 日、范围 1–90 日；超时进入异常，不假定已退 | REVERSE_REFUND\|REVERSE_CREDIT_NOTE\|CORRECT_INVENTORY_DISPOSITION\|REOPEN_RECEIVABLE | CUSTOMER_WITHDREW_BEFORE_EFFECT\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION；`sales.return.terminate`；任何实物/资金效果先补偿 |
| CUSTOMER_COMPLAINT_RESOLUTION | severity policy：CRITICAL 1h、MAJOR 4h、其他 1 business day 首响；解决默认 3/7/15 日、范围 1h–30 日 | RETRACT_AND_REISSUE_RESPONSE\|REOPEN_WORK_ORDER_OR_CAPA\|CORRECT_RESPONSIBILITY_DECISION | DUPLICATE_CONFIRMED\|CUSTOMER_FORMALLY_WITHDREW；`service.complaint.terminate` + 独立复核，法规/安全义务仍保留 |
| SERVICE_WORK_ORDER_CLOSURE | EntitlementSnapshot 的 response/resolve SLA；缺失时默认 4h/3 日、范围 1h–90 日 | RETURN_OR_SCRAP_PART\|CORRECT_LABOR_EXPENSE\|REVERSE_CHARGE_REFUND\|REOPEN_WORK_ORDER | CUSTOMER_CANCELLED_BEFORE_EFFECT\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION；`service.work_order.cancel`；有业务效果后不得 ABANDONED |
| CUSTOMER_SATISFACTION_FOLLOW_UP | 固定 7 个自然日；到期需受控无响应证据 | INVALIDATE_RATING_EVIDENCE\|OPEN_NEW_FOLLOW_UP_CYCLE | DUPLICATE_CONFIRMED\|CONTACT_LEGALLY_PROHIBITED；`service.satisfaction.terminate`；普通无响应走 closure 而非终止 |
| PERIODIC_MAINTENANCE_CYCLE | occurrence.due_at + `MAINTENANCE_GRACE` 默认 1 日、范围 0–30 日 | CANCEL_FUTURE_OCCURRENCE\|COMPENSATE_WORK_ORDER_EFFECTS\|RESCHEDULE_FROM_CANONICAL_PLAN | PLAN_TERMINATED\|EQUIPMENT_DECOMMISSIONED\|CONTRACT_TERMINATED；`service.maintenance.terminate`；已完成 occurrence 不删除 |
| PROJECT_DELIVERY_ACCEPTANCE | 各 task/milestone.due_at；项目级 timeout 取最早未完成项；超时创建风险并升级 | CANCEL_OPEN_PROJECT_TASK\|REQUEST_DOMAIN_OWNER_REVERSAL\|REOPEN_MILESTONE_OR_RISK\|RECONCILE_COST_RECEIPT | PROJECT_TERMINATED\|LEGAL_PROHIBITION\|DUPLICATE_CONFIRMED；`project.terminate` + termination impact maker-checker |
| CONTRACT_RENEWAL | signed renewal window；默认到期前 90 日至到期日、范围 1–365 日 | WITHDRAW_RENEWAL_QUOTE\|SUPERSEDE_SUCCESSOR_DRAFT\|APPLY_CONTRACT_IMPACT | NO_RENEWAL_APPROVED\|CUSTOMER_DECLINED\|SUPERSEDED；`clm.contract.renew.terminate`，原合同履约不得被终止 |
| SUPPLIER_RETURN_RECOVERY | `SUPPLIER_RETURN_COMPLETION` 默认 14 日、范围 1–90 日 | RECALL_UNSHIPPED_RETURN\|RECEIVE_BACK_OR_RECONCILE\|REVERSE_PURCHASE_CREDIT_REFUND\|REOPEN_REPLACEMENT_DEMAND | RETURN_WITHDRAWN_BEFORE_SHIPMENT\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION；`procure.supplier_return.terminate`；已发运必须完成处置 |

#### 8.2.2.1 机器可编译的 timeout 与 termination policy

上一张表供业务阅读；机器不得解析其中中文、日期短语或分号。`TimeoutPolicyDefinitionV1` strict fields 恰为 `policy_id,model,anchor_tokens,selection,calendar,parameters,on_timeout_actions,manufactures_success`；数组按 token 排序去重，`manufactures_success` 恒为 false。`parameters` 是由 `model` 判别的 strict object：`FIXED_GRACE` 只含 `default_seconds,min_seconds,max_seconds`；`EXACT_FACT_TIME|EACH_FACT_DUE|EARLIEST_OPEN_FACT_DUE` 必须为空对象；`SIGNED_LEAD_TIME_BEFORE` 只含 `min_days,max_days`；`OFFSET_SCHEDULE` 只含 `default_offsets_days,min_offset_days,max_offset_days`；`SEVERITY_MATRIX` 只含 `response_seconds_by_severity,resolution_seconds_by_severity,min_seconds,max_seconds`；`ENTITLEMENT_OR_FALLBACK` 只含 `fallback_response_seconds,fallback_resolution_seconds,min_seconds,max_seconds`；`FIXED_WINDOW` 只含 `seconds`；`RENEWAL_WINDOW` 只含 `default_days_before_expiry,min_days_before_expiry,max_days_before_expiry`。下表 code span 内每个 object 都是 strict JSON/JCS 唯一输入；未知/缺字段、不同单位、浮点数、负数、数组乱序/重复或额外 key 均拒绝。

<!-- F57-SEMANTIC-TABLE:timeout_policy_registry_v1:BEGIN -->
| timeout_policy_id | canonical `TimeoutPolicyDefinitionV1` JSON |
|---|---|
| OPPORTUNITY_DECISION_TIMEOUT_V1 | `{"anchor_tokens":["OPPORTUNITY_ENTERED_COMMERCIAL_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["ESCALATE_DECISION_OVERDUE"],"parameters":{"default_seconds":2592000,"max_seconds":15552000,"min_seconds":86400},"policy_id":"OPPORTUNITY_DECISION_TIMEOUT_V1","selection":"SINGLE"}` |
| QUOTE_VALID_UNTIL_V1 | `{"anchor_tokens":["QUOTE_VERSION_VALID_UNTIL"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"EXACT_FACT_TIME","on_timeout_actions":["MARK_QUOTE_VERSION_EXPIRED"],"parameters":{},"policy_id":"QUOTE_VALID_UNTIL_V1","selection":"SINGLE"}` |
| CONTRACT_OBLIGATION_DUE_V1 | `{"anchor_tokens":["CONTRACT_OBLIGATION_DUE_AT"],"calendar":"CONTRACT_CALENDAR","manufactures_success":false,"model":"EACH_FACT_DUE","on_timeout_actions":["CREATE_CONTRACT_BREACH","ESCALATE_CONTRACT_RISK"],"parameters":{},"policy_id":"CONTRACT_OBLIGATION_DUE_V1","selection":"EACH"}` |
| SALES_DELIVERY_TIMEOUT_V1 | `{"anchor_tokens":["SALES_ORDER_PROMISED_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["ESCALATE_SALES_DELIVERY_OVERDUE"],"parameters":{"default_seconds":86400,"max_seconds":2592000,"min_seconds":0},"policy_id":"SALES_DELIVERY_TIMEOUT_V1","selection":"SINGLE"}` |
| DROP_SHIP_DELIVERY_TIMEOUT_V1 | `{"anchor_tokens":["CUSTOMER_PROMISED_AT","SUPPLIER_COMMITTED_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["ESCALATE_DROP_SHIP_OVERDUE"],"parameters":{"default_seconds":86400,"max_seconds":2592000,"min_seconds":0},"policy_id":"DROP_SHIP_DELIVERY_TIMEOUT_V1","selection":"EARLIEST"}` |
| PROCUREMENT_LEAD_TIME_V1 | `{"anchor_tokens":["PROCUREMENT_REQUIRED_ON"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"SIGNED_LEAD_TIME_BEFORE","on_timeout_actions":["ESCALATE_PROCUREMENT_LATE"],"parameters":{"max_days":365,"min_days":0},"policy_id":"PROCUREMENT_LEAD_TIME_V1","selection":"SINGLE"}` |
| RECEIVABLE_DUNNING_SCHEDULE_V1 | `{"anchor_tokens":["RECEIVABLE_DUE_ON"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"OFFSET_SCHEDULE","on_timeout_actions":["ISSUE_DUNNING_DAY_0","ISSUE_DUNNING_DAY_30","ISSUE_DUNNING_DAY_7"],"parameters":{"default_offsets_days":[0,7,30],"max_offset_days":180,"min_offset_days":0},"policy_id":"RECEIVABLE_DUNNING_SCHEDULE_V1","selection":"SINGLE"}` |
| RETURN_COMPLETION_TIMEOUT_V1 | `{"anchor_tokens":["RETURN_REFUND_APPROVED_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["OPEN_RETURN_COMPLETION_INCIDENT"],"parameters":{"default_seconds":1209600,"max_seconds":7776000,"min_seconds":86400},"policy_id":"RETURN_COMPLETION_TIMEOUT_V1","selection":"SINGLE"}` |
| COMPLAINT_SEVERITY_SLA_V1 | `{"anchor_tokens":["CUSTOMER_COMPLAINT_ACCEPTED_AT"],"calendar":"SIGNED_BUSINESS_CALENDAR","manufactures_success":false,"model":"SEVERITY_MATRIX","on_timeout_actions":["ESCALATE_COMPLAINT_FIRST_RESPONSE","ESCALATE_COMPLAINT_RESOLUTION"],"parameters":{"max_seconds":2592000,"min_seconds":3600,"resolution_seconds_by_severity":{"CRITICAL":259200,"MAJOR":604800,"OTHER":1296000},"response_seconds_by_severity":{"CRITICAL":3600,"MAJOR":14400,"OTHER":86400}},"policy_id":"COMPLAINT_SEVERITY_SLA_V1","selection":"SINGLE"}` |
| SERVICE_ENTITLEMENT_SLA_V1 | `{"anchor_tokens":["SERVICE_WORK_ORDER_TRIAGED_AT"],"calendar":"ENTITLEMENT_CALENDAR","manufactures_success":false,"model":"ENTITLEMENT_OR_FALLBACK","on_timeout_actions":["ESCALATE_SERVICE_RESOLUTION","ESCALATE_SERVICE_RESPONSE"],"parameters":{"fallback_resolution_seconds":259200,"fallback_response_seconds":14400,"max_seconds":7776000,"min_seconds":3600},"policy_id":"SERVICE_ENTITLEMENT_SLA_V1","selection":"SINGLE"}` |
| SATISFACTION_WINDOW_V1 | `{"anchor_tokens":["SATISFACTION_SURVEY_OPENED_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_WINDOW","on_timeout_actions":["RECORD_CONTROLLED_NO_RESPONSE_CANDIDATE"],"parameters":{"seconds":604800},"policy_id":"SATISFACTION_WINDOW_V1","selection":"SINGLE"}` |
| MAINTENANCE_GRACE_V1 | `{"anchor_tokens":["MAINTENANCE_OCCURRENCE_DUE_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["MARK_MAINTENANCE_OCCURRENCE_OVERDUE"],"parameters":{"default_seconds":86400,"max_seconds":2592000,"min_seconds":0},"policy_id":"MAINTENANCE_GRACE_V1","selection":"SINGLE"}` |
| PROJECT_EARLIEST_DUE_V1 | `{"anchor_tokens":["OPEN_PROJECT_MILESTONE_DUE_AT","OPEN_PROJECT_TASK_DUE_AT"],"calendar":"PROJECT_CALENDAR","manufactures_success":false,"model":"EARLIEST_OPEN_FACT_DUE","on_timeout_actions":["CREATE_PROJECT_RISK","ESCALATE_PROJECT_OVERDUE"],"parameters":{},"policy_id":"PROJECT_EARLIEST_DUE_V1","selection":"EARLIEST"}` |
| CONTRACT_RENEWAL_WINDOW_V1 | `{"anchor_tokens":["CONTRACT_EXPIRY_AT"],"calendar":"CONTRACT_CALENDAR","manufactures_success":false,"model":"RENEWAL_WINDOW","on_timeout_actions":["ESCALATE_RENEWAL_WINDOW"],"parameters":{"default_days_before_expiry":90,"max_days_before_expiry":365,"min_days_before_expiry":1},"policy_id":"CONTRACT_RENEWAL_WINDOW_V1","selection":"SINGLE"}` |
| SUPPLIER_RETURN_TIMEOUT_V1 | `{"anchor_tokens":["SUPPLIER_RETURN_CONFIRMED_AT"],"calendar":"UTC_CALENDAR","manufactures_success":false,"model":"FIXED_GRACE","on_timeout_actions":["OPEN_SUPPLIER_RETURN_INCIDENT"],"parameters":{"default_seconds":1209600,"max_seconds":7776000,"min_seconds":86400},"policy_id":"SUPPLIER_RETURN_TIMEOUT_V1","selection":"SINGLE"}` |
<!-- F57-SEMANTIC-TABLE:timeout_policy_registry_v1:END -->

`TerminationPolicyDefinitionV1` strict fields 恰为 `policy_id,reason_codes,capability,decision_mode,guard_id`；reason_codes 排序去重，`decision_mode` 只取 `CAPABILITY_AND_SOD|MAKER_CHECKER|INDEPENDENT_REVIEW|CROSS_DOMAIN_IMPACT_APPROVAL`。每个非空 reason 展开一个 `TerminationRule={policy_id,reason_code,capability,decision_mode,guard_id}`；空数组展开零 termination rules。guard 是已注册的 typed predicate，不是脚本。

<!-- F57-SEMANTIC-TABLE:termination_policy_registry_v1:BEGIN -->
| termination_policy_id | reason_codes exact-set | capability | decision_mode | guard_id |
|---|---|---|---|---|
| OPPORTUNITY_TERMINATION_V1 | CUSTOMER_DECLINED\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION | crm.opportunity.terminate | MAKER_CHECKER | OPPORTUNITY_NO_IRREVERSIBLE_EFFECT_OR_COMPENSATED_V1 |
| QUOTE_TERMINATION_V1 | CUSTOMER_DECLINED\|LEGAL_PROHIBITION\|SUPERSEDED | cpq.quote.terminate | CAPABILITY_AND_SOD | QUOTE_VERSION_RETAINED_AND_TERMINAL_V1 |
| CONTRACT_TERMINATION_V1 | CONTRACT_TERMINATED\|LEGAL_PROHIBITION\|PARTY_LEGAL_TERMINATION | clm.contract.terminate | MAKER_CHECKER | CONTRACT_TERMINATION_IMPACT_ZERO_V1 |
| SALES_ORDER_TERMINATION_V1 | CONTRACT_TERMINATED\|CUSTOMER_CANCELLED\|LEGAL_PROHIBITION | sales.order.cancel | CAPABILITY_AND_SOD | SALES_EXTERNAL_EFFECTS_COMPENSATED_V1 |
| DROP_SHIP_TERMINATION_V1 | CONTRACT_TERMINATED\|CUSTOMER_CANCELLED\|LEGAL_PROHIBITION | sales.drop_ship.cancel | CROSS_DOMAIN_IMPACT_APPROVAL | DROP_SHIP_PROCUREMENT_FINANCE_IMPACT_CLOSED_V1 |
| PROCUREMENT_TERMINATION_V1 | DEMAND_NO_LONGER_REQUIRED\|LEGAL_PROHIBITION\|SOURCE_CANCELLED | procure.demand.cancel | CAPABILITY_AND_SOD | PROCUREMENT_ISSUED_PO_DISPOSED_V1 |
| RECEIVABLE_NO_ABANDONMENT_V1 | [] | finance.receivable.adjust | MAKER_CHECKER | RECEIVABLE_CLOSURE_COVERAGE_ONLY_V1 |
| RETURN_REFUND_TERMINATION_V1 | CUSTOMER_WITHDREW_BEFORE_EFFECT\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION | sales.return.terminate | CAPABILITY_AND_SOD | RETURN_PHYSICAL_AND_FINANCIAL_EFFECTS_COMPENSATED_V1 |
| COMPLAINT_TERMINATION_V1 | CUSTOMER_FORMALLY_WITHDREW\|DUPLICATE_CONFIRMED | service.complaint.terminate | INDEPENDENT_REVIEW | COMPLAINT_REGULATORY_AND_SAFETY_OBLIGATIONS_RETAINED_V1 |
| SERVICE_WORK_ORDER_TERMINATION_V1 | CUSTOMER_CANCELLED_BEFORE_EFFECT\|DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION | service.work_order.cancel | CAPABILITY_AND_SOD | SERVICE_NO_EFFECT_OR_ALL_EFFECTS_COMPENSATED_V1 |
| SATISFACTION_TERMINATION_V1 | CONTACT_LEGALLY_PROHIBITED\|DUPLICATE_CONFIRMED | service.satisfaction.terminate | CAPABILITY_AND_SOD | SATISFACTION_NO_RESPONSE_USES_CLOSURE_NOT_TERMINATION_V1 |
| MAINTENANCE_TERMINATION_V1 | CONTRACT_TERMINATED\|EQUIPMENT_DECOMMISSIONED\|PLAN_TERMINATED | service.maintenance.terminate | CAPABILITY_AND_SOD | MAINTENANCE_COMPLETED_OCCURRENCES_RETAINED_V1 |
| PROJECT_TERMINATION_V1 | DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION\|PROJECT_TERMINATED | project.terminate | MAKER_CHECKER | PROJECT_TERMINATION_IMPACT_CLOSED_V1 |
| CONTRACT_RENEWAL_TERMINATION_V1 | CUSTOMER_DECLINED\|NO_RENEWAL_APPROVED\|SUPERSEDED | clm.contract.renew.terminate | CAPABILITY_AND_SOD | ORIGINAL_CONTRACT_FULFILMENT_UNCHANGED_V1 |
| SUPPLIER_RETURN_TERMINATION_V1 | DUPLICATE_CONFIRMED\|LEGAL_PROHIBITION\|RETURN_WITHDRAWN_BEFORE_SHIPMENT | procure.supplier_return.terminate | CAPABILITY_AND_SOD | SUPPLIER_RETURN_SHIPMENT_EFFECT_DISPOSED_V1 |
<!-- F57-SEMANTIC-TABLE:termination_policy_registry_v1:END -->

`CompensationCommandRegistryV1` 是第三张机器表；每行 exact fields 为 `objective_kind,compensation_commands[]`，数组按 token byte order 排序唯一，不允许从人读表抽取：

<!-- F57-SEMANTIC-TABLE:compensation_command_registry_v1:BEGIN -->
| ObjectiveKind | compensation_commands exact-set |
|---|---|
| OPPORTUNITY_CONVERSION | CANCEL_UNEFFECTED_DRAFT\|MARK_DUPLICATE_SUCCESSOR\|WITHDRAW_ISSUED_QUOTE |
| QUOTE_RESOLUTION | CORRECT_SUCCESSOR_LINK\|SUPERSEDE_QUOTE_VERSION\|WITHDRAW_QUOTE_VERSION |
| CONTRACT_FULFILMENT | APPLY_CONTRACT_CHANGE\|CANCEL_UNEXECUTED_DOWNSTREAM\|CREATE_TERMINATION_IMPACT\|REQUEST_RETURN_CREDIT_REFUND |
| SALES_ORDER_FULFILMENT | AUTHORIZE_RETURN_EXCHANGE\|CANCEL_UNEXECUTED_DELIVERY\|REQUEST_CREDIT_REFUND\|REVERSE_INVENTORY_BY_OWNER |
| DROP_SHIP_FULFILMENT | AUTHORIZE_CUSTOMER_AND_SUPPLIER_RETURN\|CANCEL_UNEXECUTED_PO\|RECONCILE_AP_AR\|REQUEST_CREDIT_REFUND |
| PROCUREMENT_FULFILMENT | CANCEL_CONFIRMED_UNEXECUTED_PO\|REQUEST_PURCHASE_CREDIT_REFUND\|RETURN_TO_SUPPLIER\|REVOKE_UNISSUED_AWARD |
| RECEIVABLE_COLLECTION | EXECUTE_REFUND\|ISSUE_CREDIT_OR_WRITEOFF\|REOPEN_RECEIVABLE\|REVERSE_BAD_ALLOCATION |
| RETURN_REFUND_CLOSURE | CORRECT_INVENTORY_DISPOSITION\|REOPEN_RECEIVABLE\|REVERSE_CREDIT_NOTE\|REVERSE_REFUND |
| CUSTOMER_COMPLAINT_RESOLUTION | CORRECT_RESPONSIBILITY_DECISION\|REOPEN_WORK_ORDER_OR_CAPA\|RETRACT_AND_REISSUE_RESPONSE |
| SERVICE_WORK_ORDER_CLOSURE | CORRECT_LABOR_EXPENSE\|REOPEN_WORK_ORDER\|RETURN_OR_SCRAP_PART\|REVERSE_CHARGE_REFUND |
| CUSTOMER_SATISFACTION_FOLLOW_UP | INVALIDATE_RATING_EVIDENCE\|OPEN_NEW_FOLLOW_UP_CYCLE |
| PERIODIC_MAINTENANCE_CYCLE | CANCEL_FUTURE_OCCURRENCE\|COMPENSATE_WORK_ORDER_EFFECTS\|RESCHEDULE_FROM_CANONICAL_PLAN |
| PROJECT_DELIVERY_ACCEPTANCE | CANCEL_OPEN_PROJECT_TASK\|RECONCILE_COST_RECEIPT\|REOPEN_MILESTONE_OR_RISK\|REQUEST_DOMAIN_OWNER_REVERSAL |
| CONTRACT_RENEWAL | APPLY_CONTRACT_IMPACT\|SUPERSEDE_SUCCESSOR_DRAFT\|WITHDRAW_RENEWAL_QUOTE |
| SUPPLIER_RETURN_RECOVERY | RECALL_UNSHIPPED_RETURN\|RECEIVE_BACK_OR_RECONCILE\|REOPEN_REPLACEMENT_DEMAND\|REVERSE_PURCHASE_CREDIT_REFUND |
<!-- F57-SEMANTIC-TABLE:compensation_command_registry_v1:END -->

`f57check` 必须把 §8.2.0、§8.2.1 及本节三张机器表分别解析为 typed rows 后按 ObjectiveKind exact-join；人读表不得作为 fallback。Task 12 的 runtime export 必须逐字段 exact-match 15 rows，并用负例拒绝 prose token、未知/缺 policy、错误 duration unit、数组乱序/重复、Quote 非空 reopen、Receivable termination rule、未注册 guard 或一个全局 closure/timeout/termination 默认值。

`AuthorizedTerminationV1` strict fields 为 `objective_id,cycle_no,reason_code,requested_by,approved_by,decision_at,unfulfilled_obligations[],confirmed_effects[],compensation_refs[],residual_impacts[],evidence_refs[],audit_ref`；数组排序去重。requested_by 与 approved_by 必须不同，且终止能力、SoD、当前 generation 和 trusted time 均重验。任何未登记 reason、遗漏 effect/obligation、残余财务/库存/签章事实或 compensation 失败都保持 INCIDENT/RECONCILING，不得进入 ABANDONED/CLOSED。

### 8.3 Predicate 计算规则

1. 每个 Objective 实例固定 definition_version 和 generation；运行中升级只能按继续、补偿或重启策略，不热改关闭含义。
2. closure predicate 必须引用 typed fact 和 evidence digest，不允许查询一段任意脚本返回 true。
3. CLOSED 时保存 predicate_version、全部 obligation 结果、evidence refs 和 closure_digest。
4. 上游事实变化时重新计算受影响 Objective；只有登记表列明的事实可重开，未知新事实进入 INCIDENT 并阻止错误关闭。
5. T-F57-AUT-004 必须覆盖 normal waiting、Unknown→RECONCILING、conflict→INCIDENT、closure review approve/reject、registered reopen 和 ABANDONED terminal 的全部允许边及未列边负例。
6. 重开创建新的 cycle_no，继承 objective_id、原 due_at、历史责任和证据；可以新增差异 obligation，但不能删除旧 cycle。
7. 每个 cycle 必须有退出条件、检查周期、重试上限和升级 capability queue；没有固定岗位兜底。

### 8.4 Workflow 编译、升级、灰度和回滚

`WorkflowDefinitionV1` strict fields 为 `workflow_id,definition_version,generation,objective_kind,trigger_kinds[],step_graph,timeout_policy_id,compensation_graph,upgrade_policy,rollout_policy,rollback_generation,compiled_digest`。step kind 闭集为 `OWNER_COMMAND|WAIT_TYPED_FACT|TYPED_DECISION|FORK_JOIN_ALL|CLOSURE_CHECK|COMPENSATE`；condition 只能引用已登记 typed fact/field/operator，禁止任意脚本、SQL、动态网络调用、无界循环、隐式 effect 或未登记 closure。编译必须证明 step/edge 可达、每条路径有退出/超时、effect 有幂等/Unknown/compensation 契约、obligation/evidence 与 §8.2 registry exact-match、无悬空并发分支。

Workflow 不再另建 Markdown 机器表。本节是唯一 authoring rule；G0 只建立 `authoring_rule_id=f57.workflow_definition.graph_native.v1`、`WORKFLOW_DEFINITION_REGISTRY_V1` validator 和 strict wire，实际 owning task 在其 ObjectiveKind 首次 activation-due 前把 definitions 写入 CapabilityGraph 的唯一 `WORKFLOW_DEFINITION_REGISTRY`、`GRAPH_NATIVE` semantic contract binding。每行以全局唯一 `workflow_id` 为 row key，并用 `CANONICAL_JCS_OBJECT(WorkflowDefinitionV1)` 保存完整 typed step/compensation graph；不得用 ObjectiveKind 充当 row key或把 JSON 放入普通 UTF8。该 binding 的 `definition.objective_kind` coverage 必须覆盖 `F57_BUSINESS_OBJECTIVE_KIND_V1_EXACT` 的全部 15 个 ObjectiveKind，每种至少一个 definition；Objective summary 只能引用该 binding 中属于同一 ObjectiveKind 且 exact-resolve 的 ID。生成后的 JSON、运行时数据库、UI 配置和手写 Rust 都只是投影或缓存，不能反向成为定义来源。其他八种 semantic contract 禁止使用 `GRAPH_NATIVE`，workflow registry 也禁止伪装成 `CONTRACT_TABLE`。

运行实例固定原 definition_version/generation。升级决定闭集为：

| UpgradeDecision | 唯一语义 |
|---|---|
| CONTINUE_PINNED | 旧实例按旧定义直至终态；新触发才使用新定义 |
| COMPENSATE_AND_TERMINATE | 停止新 effect，确证全部 Unknown，按旧定义 compensation graph 完成并独立复核后终结旧实例；不能把未补偿结果当成功 |
| RESTART_ON_NEW_DEFINITION | 只有尚无不可逆 Confirmed effect，或全部旧 effect 已确证补偿后，才创建引用 predecessor 的新 objective/cycle；旧实例和证据不可改写 |

每个升级决定必须逐运行实例保存 impact snapshot、原因、旧/新 digest、决定人/独立审批人和证据；禁止“热替换后继续跑”或未登记第四种策略。

发布顺序固定为 `compile→deterministic simulation→fault injection→maker-checker approval→sign generation→canary→promote`。simulation 使用与 live 相同的 typed rule/closure engine 但不得 dispatch effect；fault injection 至少覆盖 checkpoint 前后崩溃、响应丢失、重复/逆序 callback、provider failure、lease loss、full disk 和 compensation failure。灰度只接收新实例，cohort 固定为 `SHA-256(deployment_id||workflow_id||subject_id||target_generation)` 的签名阈值，不能人工挑选“容易成功”的对象；旧实例仍 pinned。

任一 CRITICAL incident、重复 effect、未处置 Unknown、closure false-positive，或签名 rollout policy 的 error/reopen/SLA 阈值超限，立即停止向新 generation 分流并用 Task 9 已签名 `rollback_plan_digest` 回到前一 generation。已进入新版本的实例仍按其批准的 CONTINUE_PINNED/COMPENSATE_AND_TERMINATE/RESTART_ON_NEW_DEFINITION 决定处置，回滚不得原位改写。只有 canary 证据、业务对账和独立批准都通过才能 promote 100%。

### 8.5 Objective 与 workflow 验收

T-F57-AUT-001、T-F57-AUT-002、T-F57-AUT-003、T-F57-AUT-005、T-F57-AUT-006 必须对登记表中每个 ObjectiveKind 生成：

- happy_path_closes_with_complete_evidence
- missing_each_required_obligation_blocks_closure
- duplicate_and_out_of_order_fact_is_idempotent
- every_registered_reopen_fact_creates_next_cycle
- unregistered_fact_cannot_silently_change_terminal_state
- abandoned_requires_impact_and_cannot_erase_economic_effect
- three_upgrade_decisions_preserve_old_instance_and_evidence
- compile_simulate_fault_inject_canary_promote_is_non_skippable
- canary_failure_uses_signed_rollback_and_never_hot_mutates_running_instance

只测试合成 Objective 不足以激活业务 owner task。

## 9. Unknown 外部效果的人工处置

### 9.1 闭合决定类型

HumanEffectDecision 只能取：

| 决定 | 含义 | 后继 |
|---|---|---|
| CONFIRMED_SUCCEEDED | 独立证据证明原 effect 已成功执行 | 追加 EffectConfirmed 和对应 typed domain fact |
| CONFIRMED_NOT_EXECUTED | 独立证据证明原 effect 未执行 | 原 effect 终结为 FailedNotExecuted；允许新 effect 使用新 ID 并引用原 effect |
| CONFIRMED_COMPENSATED | 证明原 effect 曾执行且补偿已成功 | 追加 EffectCompensated 和补偿业务事实 |
| UNRESOLVED_CONTAINED | 无法证明结果，但风险已隔离且不会盲重试 | Objective 保持 INCIDENT/RECONCILING，不得作为业务成功关闭 |

禁止 MANUAL_SUCCESS、MARK_DONE、ASSUME_FAILED、IGNORE 或自由文本决定类型。

四种决定只接受当前 `UNKNOWN` effect。`CONFIRMED_SUCCEEDED`、`CONFIRMED_NOT_EXECUTED`、`CONFIRMED_COMPENSATED` 分别原子推进到 `CONFIRMED`、`FAILED_NOT_EXECUTED`、`COMPENSATED`；`UNRESOLVED_CONTAINED` 追加 containment/incident 事实但 effect 仍为 `UNKNOWN`。若决定事务内或决定后收到与目标相反的独立 callback/对账证据，则 effect 进入 `CONFLICTED`、Objective 进入 INCIDENT、所有重复 dispatch 冻结，并新建引用原 effect/evidence 的恢复责任；不得回滚或覆盖已签人工决定。

### 9.2 证据包

每次人工决定必须保存：

- effect_id、objective_id、provider_id、operation_kind；
- 原 request digest、provider idempotency key、dispatch time 和最后已知状态；
- 查询过的外部范围、时间窗口和结果；
- 外部回执/对账单/签署文件/银行或税务证明等独立证据 digest；
- decision、reason_code、发起人、独立审批人；
- reauthentication evidence、SoD decision、policy/generation/version；
- 决定时间、后续 obligation 和通知对象。

操作者记忆、截图中无法验证的文字、同一系统自生成的 Unknown 状态或单一人员声明不能作为独立证据。

### 9.3 风险门禁

1. 付款、退款、银行、开票/红冲、合同签署/签章/生效、采购单发出、客户接受和权限/配置等高风险 effect 必须双人决定，且至少一人独立于原执行者。
2. CONFIRMED_SUCCEEDED 必须有外部独立成功证据；CONFIRMED_NOT_EXECUTED 必须有 provider 明确否定或覆盖完整处理窗口的权威对账证据。
3. 不能证明时只能 UNRESOLVED_CONTAINED；不得为了清空事故箱选择较方便的结果。
4. 只有 CONFIRMED_NOT_EXECUTED 后才允许重发；新 effect 使用新 effect_id，但业务幂等关系引用原 effect，防止双重完成。
5. 后续回调与人工决定相反时立即进入 ReconciliationConflict，重开相关 Objective，冻结新增同类效果，并按资金/发票/签章等 owner 的冲正或补偿路径处置。
6. 人工决定本身不能绕过业务 owner；自动化内核只能把已批准决定交给对应 owner 的 reconciliation command。

### 9.4 Unknown 验收

T-F57-FIN-009、T-F57-MCP-003 和 T-F57-AUT-002 必须至少包含：

- unknown_has_only_four_human_decisions
- high_risk_success_requires_independent_external_evidence_and_two_people
- unresolved_contained_never_satisfies_business_closure
- retry_is_allowed_only_after_confirmed_not_executed
- late_opposite_callback_reopens_and_freezes_duplicate_effects
- effect_every_allowed_edge_is_executable_and_every_unlisted_edge_is_rejected
- effect_reconciling_and_failed_no_effect_legacy_tokens_are_rejected
- late_opposite_callback_moves_effect_to_conflicted_and_preserves_both_evidence_sets
- human_decision_cannot_write_domain_fact_without_owner_command

## 10. 客户/供应商外部门户身份生命周期

### 10.1 对象和边界

客户和供应商门户均禁止自助注册，也禁止外部联系人转邀。两种 audience 共用一套身份内核，当前对象 exact-set 为：

- PortalInvite
- PortalPrincipal
- PortalPartyBinding
- PortalAuthenticator
- PortalDevice
- PortalSession

一个 `PortalPrincipal` 表示一个已验证自然人，可以有多个 `PortalPartyBinding`。每个 binding 必须精确绑定一个 `audience`、一个 `legal_entity_id`、一个 party 和一个生效联系人关系：

- `CUSTOMER_PORTAL`：`customer_id` 与客户 `contact_id` 必填，`supplier_id` 必须为空；
- `SUPPLIER_PORTAL`：`supplier_id` 与供应商 `contact_id` 必填，`customer_id` 必须为空。

两种 audience 不得共用 binding、session、device credential 或授权投影。每个请求必须选择一个 ACTIVE binding，查询和命令不得跨 binding、party、法人或 audience 聚合；客户端提交的 party、法人或 audience 不能覆盖服务器会话上下文。

### 10.2 邀请状态机

InviteState：

- ISSUED
- ACCEPTED
- EXPIRED
- REVOKED

允许边是闭集：`ISSUED→ACCEPTED|EXPIRED|REVOKED`；`ACCEPTED`、`EXPIRED`、`REVOKED` 均为不可恢复终态，所有未列边一律拒绝。一次邀请只能产生一个最终结果，过期扫描、人工撤销和接受命令以同一 invite version 做原子 CAS；竞争者只观察到已经提交的唯一终态，不能把终态改成另一终态。

规则：

1. 邀请只能由分别具有 `customer.portal.identity.invite` 或 `supplier.portal.identity.invite` 能力的内部主体，为对应 audience 下仍生效的已登记联系人创建。
2. 邀请绑定 audience、法人、精确一个 customer/supplier party、联系人、规范化邮箱/手机号、issued_at、expires_at 和一次性随机 token digest。
3. token 明文不得入库或日志；默认 72 小时过期，配置范围 1 至 168 小时。
4. token 单次消费；重复、过期、撤销、party/联系人关系变化、audience 错配或 binding 已存在都拒绝。
5. 外部联系人不能自行邀请其他用户；POR-001、POR-002 均不包含邀请命令。

### 10.3 主体和 binding 状态

PortalPrincipalState：

- PENDING_ACTIVATION
- ACTIVE
- SUSPENDED
- REVOKED

PortalPartyBindingState：

- PENDING_APPROVAL
- ACTIVE
- SUSPENDED
- ENDED

允许边是闭集：

| 对象 | 当前状态 | 允许后继 |
|---|---|---|
| Principal | PENDING_ACTIVATION | ACTIVE、REVOKED |
| Principal | ACTIVE | SUSPENDED、REVOKED |
| Principal | SUSPENDED | ACTIVE、REVOKED |
| Principal | REVOKED | 无 |
| Binding | PENDING_APPROVAL | ACTIVE、ENDED |
| Binding | ACTIVE | SUSPENDED、ENDED |
| Binding | SUSPENDED | ACTIVE、ENDED |
| Binding | ENDED | 无 |

状态原因到目标的映射固定为：疑似被盗、临时安全隔离、party/联系人临时停用、可恢复的 MFA/device/渠道问题进入 SUSPENDED；联系人关系结束/离职、party 法律终止、主数据合并的来源 binding、永久 portal access 撤销进入 ENDED；明确的自然人身份欺诈、法律身份撤销或永久全局身份撤销进入 Principal REVOKED。`SUSPENDED→ACTIVE` 只能由恢复门禁完成；ENDED/REVOKED 不可恢复，只能按批准流程建立新 binding/principal。

Principal 是 bindings 的保守聚合：首次成功激活前为 PENDING_ACTIVATION；任一 binding 为 ACTIVE 且不存在 principal 级全局停用/撤销时为 ACTIVE；曾至少一次成功 ACTIVE、当前没有 ACTIVE 但至少一个 SUSPENDED 或全部 binding 均为 ENDED 时为 SUSPENDED；从未成功激活且全部 binding 均为 PENDING_APPROVAL 或 ENDED 时仍为 PENDING_ACTIVATION；只有依法明确的永久全局身份撤销才是 REVOKED。因而首个 PENDING_APPROVAL binding 直接 ENDED 不会制造 `PENDING_ACTIVATION→SUSPENDED` 隐式边。单一 binding 的 refresh reuse 或关系问题不得停掉同一 principal 的其他健康 binding；principal 级身份被盗/撤销必须按 §10.5 先原子提交不可回退的 authority security fence，使全部目标 binding 立即失去访问，再完成可重试的凭据清扫，不能把“状态事务回滚”误当成已经停用。

激活必须完成：

- 一次性邀请验证；
- 联系渠道验证；
- 当前密码策略；
- 至少一个 MFA 因子，首发支持 TOTP 或 WebAuthn；
- 对应 customer/supplier party 与联系人关系仍有效；
- portal terms/version 接受证据；
- 服务器创建 device/session 绑定。

首次激活只有一个权威命令 `AcceptPortalInvite`，不得由实现者拆成可分别提交的状态跳转。命令先在事务外完成必要的交互式 channel/TOTP/WebAuthn challenge，并把结果固化为短期、单次、绑定 invite/binding/device key 的验证证据；调用权威事务前先对该证据做不可回退的 consume-on-attempt CAS，失败后必须重新 challenge，消费本身不改变任何 portal domain state。随后在一个 serializable authority transaction 内按固定顺序：校验 invite 仍为 ISSUED、联系人/party/audience/法人仍匹配、密码/MFA/条款和已消费证据的 digest 有效；创建并激活 binding-scoped PASSWORD 与 MFA authenticator；使 binding 变为 ACTIVE 并据聚合规则使 principal 变为 ACTIVE；创建并激活首个 device；创建首个 ACTIVE session/refresh family；把 invite 置为 ACCEPTED；写审计和下述 activation receipt。任一步失败则除证据已消费事实外的全部 domain/session 写入回滚，invite 保持 ISSUED，外部只能看到事务前或完整事务后状态。后续对既有 principal 增加 binding 使用同一命令和事务顺序，但只改变目标 binding，并按 §10.3 重新计算 principal 聚合。

`PortalCredentialActivationReceiptV1` strict fields 恰为 `receipt_id,invite_id,principal_id,binding_id,audience,legal_entity_id,party_kind,party_id,contact_id,authenticator_ids,device_id,session_id,refresh_family_id,channel_proof_digest,mfa_proof_digest,terms_version,terms_evidence_digest,activated_at,generation,audit_ref`。`party_kind` 只能为 `CUSTOMER|SUPPLIER` 且与 audience/party_id 匹配；authenticator_ids 排序去重并恰含一个 ACTIVE PASSWORD_ARGON2ID 与至少一个 ACTIVE TOTP/WEBAUTHN，其他 credential ID 均属于同一 binding/audience/generation。每个 ACCEPTED invite 在同一事务必须恰好生成一张 immutable activation receipt。只有同一 `request_id`/`idempotency_key`、同一 principal/binding/device、原响应丢失且调用者仍被授权读取该结果的命令重试，才可由通用 command receipt 返回原 activation receipt 引用；持久层若复用通用内部列名 `command_id`，其值必须逐字等于 wire `request_id`，不得向 portal wire 增加 `command_id` 字段或别名。invite token 或 channel/MFA challenge proof 的再次提交始终以不枚举错误拒绝，不得返回 receipt、principal 或 binding 是否存在，也不得再建 session/device/receipt。receipt 只存 digest/ID，不存 token、password、TOTP secret、WebAuthn private material 或 refresh 明文。它与撤销专用 `PortalCredentialRevocationReceiptV1` 是两个不相容 schema，禁止互相代用。

### 10.4 多 party、合并、终止和恢复

1. 同一自然人可以有多个 active binding，但切换客户、供应商或 audience 必须建立新 session context；URL、载荷或客户端字段不能改变当前 binding。
2. 跨法人 binding 永不自动合并，密钥、会话和投影完全隔离。
3. 客户或供应商合并时先把旧 binding 置为 ENDED、撤销全部旧 session/device/authenticator，再由批准的主数据合并计划建立 PENDING_APPROVAL 的新 binding；历史 portal 事实继续引用旧 party 快照，禁止原位换 party_id。
4. 联系人离职、party 关系终止或永久授权撤回时 binding 立即 ENDED；可恢复的 party/联系人临时停用或临时授权冻结进入 SUSPENDED。两种结果均撤销该 binding 的全部 session、refresh token 和 device credential；开放验收、投诉、服务或采购协同责任转为内部异常任务，不自动丢失。
5. 账号恢复必须重新验证联系渠道，由有恢复能力的内部主体批准，撤销旧 MFA/device/session，再建立新 authenticator；服务台不能读取旧密码或 MFA secret。
6. 邮箱/手机号变更先验证新渠道，再使旧渠道失效并撤销全部 session。
7. 怀疑被盗时任何有安全停用能力的主体可立即 SUSPEND；恢复需要不同主体批准。停用不删除审计、客户提交事实或附件。
8. 一个曾至少一次成功 ACTIVE 的 principal 在最后一个健康 binding 结束时进入 SUSPENDED；从未成功激活的 principal 在全部 binding 结束后仍按 §10.3 保持 PENDING_ACTIVATION。两者在达到依法批准的数据处置条件前都不得物理删除。

恢复不是独立状态。恢复处理中相应 binding 必须保持 `SUSPENDED`；principal 按 §10.3 聚合，在仍有其他健康 ACTIVE binding 时保持 ACTIVE，在全局身份隔离或没有健康 binding 时为 SUSPENDED。只有该 binding 的旧 authenticator/device/session 已全部撤销、新联系渠道与 MFA 已验证、不同主体批准且当前 party 关系重新通过后，该 binding 才能回到 ACTIVE；失败或永久关系终止进入 ENDED，自然人身份欺诈/法律撤销才使 principal 进入 REVOKED。不得实现 `RECOVERY_PENDING` 或其他未登记状态。

### 10.5 会话和权限

1. portal session 的 audience 固定为所选 binding 的 `CUSTOMER_PORTAL` 或 `SUPPLIER_PORTAL`，不能跨 audience，也不能用于 Workbench、Control Center 或 core-server。
2. 每次查询/命令重新验证 principal、binding、对应 party/contact 状态、MFA、设备、session、current generation，以及该 audience 的 POR-001/POR-002 allowlist。
3. 404 用于无权对象，禁止泄露其他 party、audience 或法人的存在。
4. 客户交付/验收、投诉、服务请求、补证，以及供应商 PO 确认、交期、ASN、发票上传和资料变更，都保存 principal、binding、party、文档 digest、可信时间和设备证据。
5. 门户身份绝不产生内部动态 grant；它只取得对应固定外部 audience allowlist 和该 binding 的 party 范围。

PortalAuthenticatorState 闭集为 `PENDING_VERIFICATION|ACTIVE|REVOKED|EXPIRED`，允许边只有 `PENDING_VERIFICATION→ACTIVE|REVOKED|EXPIRED` 和 `ACTIVE→REVOKED|EXPIRED`；PortalDeviceState 使用同一状态闭集与边；PortalSessionState 闭集为 `ACTIVE|CLOSED|REVOKED|EXPIRED`，只允许从 ACTIVE 进入三个终态，终态不可恢复。邀请撤销/过期、关系变化、合并、binding 终止或批量安全撤销必须把尚在 PENDING_VERIFICATION 的 authenticator/device 置为 REVOKED；不得把待验证项伪装成 EXPIRED，也不得留下可继续激活的孤儿。每个 authenticator 的 exact binding key 为 `principal_id,binding_id,audience`，还必须保存 `authenticator_id,kind,credential_identity_digest,issued_at,expires_at,state,generation`；不得跨 binding/audience 共享密码验证记录、TOTP secret 或 WebAuthn credential。authenticator kind 闭集为 `PASSWORD_ARGON2ID|TOTP|WEBAUTHN`；每个 ACTIVE binding 必须始终有满足当前密码策略的 PASSWORD_ARGON2ID 和至少一个 ACTIVE 的 TOTP/WEBAUTHN，替换使用新 ID，旧项只能 REVOKED，不能原位覆盖 secret/public-key identity。TOTP secret 只在一次 enrollment ceremony 内显示且加密存储，WebAuthn credential id 在全 deployment 唯一、sign counter 回退/克隆检测立即撤销该 authenticator、同 binding 的 session 和 device 并 SUSPEND 该 binding；其他 binding 不受影响，除非证据升级为 principal 级身份被盗。

PortalDevice 是 binding/audience 专用 credential，字段至少绑定 `device_id,principal_id,binding_id,audience,public_key_or_webauthn_credential_id,credential_digest,issued_at,expires_at,last_used_at,state,generation`；私钥不得入库、日志或导出。`PENDING_VERIFICATION→ACTIVE` 需要当前 ACTIVE binding、联系渠道、MFA 和重新认证；ACTIVE 绝对有效期最多 90 天，同一 binding 最多 3 个 ACTIVE device，达到上限时新登记必须显式选择并撤销旧 device，不能自动挤掉。binding/principal 暂停或终止、关系变化、credential 轮换/克隆、90 天到期、管理员/用户撤销均立即使相应 device 终结；恢复只能创建新 device ID/credential。

PortalSession 只在 ACTIVE principal+binding+device、当前 generation 和成功 MFA 后创建。access credential 固定最多 15 分钟，空闲最多 30 分钟，session/refresh family 绝对最多 8 小时；三者取最早到期，续期不得滑动延长绝对期限。每个 principal+binding 最多 3 个 ACTIVE session，达到上限时拒绝并返回稳定 `PORTAL.SESSION.LIMIT_REACHED`，除非主体显式关闭一个。交付/验收确认、投诉或服务请求正式提交、PO/交期/ASN/发票提交、资料变更、device/authenticator/session 管理要求 MFA freshness 不超过 10 分钟；超时只能重新 MFA，不能靠 refresh 延长。

refresh credential 是至少 256-bit CSPRNG opaque value，只放 `Secure`、`HttpOnly`、`SameSite=Strict` cookie；数据库只存 deployment-keyed HMAC digest、`family_id`、单调 `rotation_no`、session/device/binding、issued/expires/used/revoked 时间。每次 refresh 必须 single-use 原子 CAS 并签发下一 rotation；旧值再次出现、并发双用、rotation 跳号或错 device/binding/audience 均判 reuse，原事务撤销整个 family、该 device 的全部 session/credential，SUSPEND 对应 binding，创建不可抑制安全事件，且不向调用者泄露哪一项错配。refresh/access/CSRF/device secret 均不得进 URL、localStorage、普通日志或附件。

安全、关系终止、合并或 principal-wide 批量撤销是固定的两事务 fail-closed protocol。第一步 `BeginPortalSecurityFence` 在一个 serializable authority transaction 中解析并冻结 exact binding set、按下表转换 binding/principal、递增 authority epoch，并写 immutable `PortalSecurityFenceV1`。其 strict fields 恰为 `fence_id,principal_id,scope,binding_targets,reason_code,target_principal_state,epoch_before,epoch_after,requested_by,fenced_at,generation,audit_ref`；`scope` 只能为 `BINDING|PRINCIPAL`，`binding_targets[]` 每项恰为 `binding_id,audience,from_state,target_state`，按 `(binding_id,audience)` 排序且组合唯一，PRINCIPAL scope 必须覆盖该 principal 的全部非终态 binding 和仍残留 credential 的终态 binding，BINDING scope 恰一项；epoch_after 必须等于 epoch_before+1。`from_state=target_state` 只有两类合法用途：reason 表逐字要求的 `SUSPENDED→SUSPENDED` 幂等隔离，以及把仍残留 credential 的 `ENDED→ENDED` 终态 binding 纳入 sweep；两者都不算新状态边。其他非终态同态、`ACTIVE→ACTIVE`、`PENDING_APPROVAL→PENDING_APPROVAL`、跨 reason 借用或无 residual credential 的终态同态一律拒绝。该事务一旦提交不得因后续清扫失败回退；gateway 每次请求和 refresh 都读取/订阅 authority epoch 和 fence，旧 epoch 或未完成 fence 一律拒绝，不能只相信自包含 token。

`PortalFenceReason` 与结果是闭集：

| reason_code | scope | binding_targets | target_principal_state |
|---|---|---|---|
| REFRESH_REUSE、CREDENTIAL_CLONE、BINDING_SECURITY_SUSPECTED、RELATIONSHIP_TEMPORARILY_DISABLED | BINDING | 目标 ACTIVE→SUSPENDED；已 SUSPENDED 保持 | 按 §10.3 聚合，其他健康 binding 可使其仍 ACTIVE |
| RELATIONSHIP_ENDED、PARTY_MERGE_SOURCE、BINDING_ACCESS_PERMANENTLY_REVOKED | BINDING | 目标 PENDING_APPROVAL/ACTIVE/SUSPENDED→ENDED | 按 §10.3 聚合 |
| PRINCIPAL_COMPROMISE_SUSPECTED | PRINCIPAL | ACTIVE→SUSPENDED，SUSPENDED 保持，PENDING_APPROVAL→ENDED，ENDED 保持 | 只接受曾 ACTIVE principal，结果 SUSPENDED |
| PRINCIPAL_IDENTITY_FRAUD、PRINCIPAL_LEGAL_IDENTITY_REVOKED、PRINCIPAL_GLOBAL_ACCESS_REVOKED | PRINCIPAL | PENDING_APPROVAL/ACTIVE/SUSPENDED→ENDED，ENDED 保持 | 同一第一事务显式置 REVOKED，不使用普通聚合结果 |

未知 reason、scope/target 不匹配、永久 principal reason 未把 principal 原子置 REVOKED、临时 reason 试图 REVOKED/ENDED 健康其他 binding，或 suspected compromise 用于从未 ACTIVE principal 都拒绝。PENDING_ACTIVATION principal 的邀请/待审批风险只能撤销邀请并结束 binding；只有具备后三种永久证据时才走 `PENDING_ACTIVATION→REVOKED`。

第二步 `FinalizePortalCredentialRevocation` 可幂等重试，但每次尝试必须在一个 authority transaction 中撤销 fence exact-set 内全部 ACTIVE/PENDING authenticator、device、session 和 refresh family，并写对应 `PortalCredentialRevocationReceiptV1`。其 strict fields 恰为 `receipt_id,fence_id,principal_id,binding_id,audience,authenticator_ids,device_ids,session_ids,refresh_family_ids,reason_code,revoked_at,generation,audit_ref`；四个 ID 数组排序去重，可为空但受影响对象不得漏列。receipt 粒度严格为一个 `principal_id,binding_id,audience`；N 个 binding/audience 必须按 `(binding_id,audience)` 升序写恰好 N 张 receipt，全部引用同一 fence_id，并共享 audit_ref、reason、generation 和 revoked_at。数据库约束验证 receipt binding set 与 fence exact-set 相等、每张数组与被撤销行相等；缺少、额外、重复、跨 binding 或 receipt/audit 不一致使第二事务整体回滚，但第一事务的 fence/epoch/SUSPENDED|ENDED 状态仍在，访问继续失败关闭并创建独立耐久内部异常。清扫成功后把 fence 标记 finalized；恢复只能在 finalized 后按 §10.4 建立新 credential，不能重新启用旧项。普通用户显式关闭单一 session 可在一个事务完成 `ACTIVE→CLOSED`，不冒充安全批量撤销。

### 10.6 门户身份验收

T-F57-POR-003 必须至少包含：

- no_self_registration_or_external_party_delegated_invite
- invite_is_single_use_expiring_and_binding_specific
- invite_every_allowed_edge_is_executable_and_every_unlisted_edge_is_rejected
- accept_portal_invite_is_one_atomic_bootstrap_without_visible_partial_activation
- accepted_invite_writes_exactly_one_strict_activation_receipt
- same_idempotent_command_retry_may_return_authorized_receipt_but_token_or_proof_replay_is_rejected_without_enumeration
- one_principal_multiple_audience_bindings_never_cross_query
- customer_or_supplier_merge_revokes_sessions_before_rebinding
- contact_or_relationship_end_revokes_all_credentials
- recovery_replaces_mfa_devices_and_sessions_with_independent_approval
- stolen_account_suspend_is_immediate_and_audited
- customer_and_supplier_sessions_cannot_cross_audience_or_call_workbench_control_routes
- portal_authenticator_device_and_session_state_edges_are_closed
- pending_authenticator_and_device_can_be_revoked_by_invite_or_binding_termination
- never_activated_principal_does_not_take_an_implicit_suspended_edge
- principal_and_binding_reason_to_state_mapping_is_exact_and_terminal_states_never_recover
- authenticator_is_bound_to_one_principal_binding_audience_and_recovery_is_scoped
- access_idle_absolute_and_device_ttl_use_the_earliest_deadline
- refresh_rotation_is_single_use_and_reuse_suspends_only_the_affected_binding
- mfa_freshness_and_three_session_device_limits_fail_closed
- revocation_receipt_exactly_lists_every_invalidated_credential
- principal_wide_revocation_writes_exactly_one_receipt_per_binding_in_one_transaction
- security_fence_commits_before_credential_sweep_and_survives_sweep_rollback
- portal_fence_reason_scope_binding_and_principal_targets_are_exact
- permanent_principal_fence_atomically_ends_bindings_and_sets_revoked
- gateway_rejects_old_epoch_while_fence_is_unfinalized
- revocation_retry_finalizes_exact_receipt_set_once_without_reopening_access

## 11. 动态责任解析和改派

### 11.1 WorkItemAssignment 状态

所有 WorkItemAssignment 的 `current_assignee` 以及下列 attempt 的 `assignee` 都必须直接使用 foundation 唯一 `PrincipalRefV1 {kind,id}`，不得保存裸 UUID、默认 USER kind 或另造 assignee 类型。分配只表达责任；每次接受、开始和 effect 前仍按完整 principal kind+id 重验当前 grant，分配本身不授予权限。

AssignmentState：

- UNASSIGNED
- RESOLVING
- ASSIGNED
- ACCEPTED
- IN_PROGRESS
- WAITING
- REASSIGNING
- ESCALATED_NO_CANDIDATE
- COMPLETED
- CANCELLED

允许边、命令和门禁固定为：

| 当前状态 | 允许后继 | 命令/门禁 |
|---|---|---|
| UNASSIGNED | RESOLVING、CANCELLED | `ResolveAssignment` 或业务 owner 合法取消 |
| RESOLVING | ASSIGNED、ESCALATED_NO_CANDIDATE、CANCELLED | 候选查询恰一名获选且保存解释时 ASSIGNED；空集合只能升级，无权自动扩权 |
| ASSIGNED | ACCEPTED、RESOLVING、CANCELLED | 只有被指派主体在权限/device/SoD 重新验证后可接受；接受前失效、拒绝或超时终结该 assignment attempt 并重新解析 |
| ACCEPTED | IN_PROGRESS、REASSIGNING、CANCELLED | `StartWork` 前再验当前 grant；接受后失效先 checkpoint 再改派 |
| IN_PROGRESS | WAITING、REASSIGNING、COMPLETED、CANCELLED | WAITING 只允许登记原因；完成要求 owner closure predicate；失效先 checkpoint |
| WAITING | IN_PROGRESS、REASSIGNING、COMPLETED、CANCELLED | 等待事实解除后回 IN_PROGRESS；若解除事实同时满足关闭谓词可原子进入 COMPLETED；Unknown effect 不得靠改派重试 |
| REASSIGNING | ASSIGNED、ESCALATED_NO_CANDIDATE、CANCELLED | 新候选按当前权限重新解析并创建新 attempt；不得继承 session/MFA/临时权限 |
| ESCALATED_NO_CANDIDATE | RESOLVING、CANCELLED | grant/candidate/策略改变后显式重新解析；不得直接 ASSIGNED |
| COMPLETED | 无 | 终态；后续 obligation 用新的 review/work item，不复活原 assignment |
| CANCELLED | 无 | 终态；原因和未完成影响必填 |

每次 assignment attempt 固定 `attempt_id,query_id,assignee: PrincipalRefV1 {kind,id},accepted_at,checkpoint_ref,ended_reason,row_version`；从 RESOLVING/REASSIGNING 选人必须创建新 attempt，不能改写旧 assignee。所有状态变更使用 CAS；未列边一律拒绝。

CandidateQuery 必须按 capability、legal entity/record scope、条件、有效期、设备要求、金额/风险上限、SLA、位置、负载、回避和 SoD 解析。角色/岗位只能展开成 grant 模板，不能直接成为候选。

### 11.2 失效和改派规则

| 时点 | 处理 |
|---|---|
| ASSIGNED 但未接受时失效 | 原 assignment 终结；进入 RESOLVING；SLA 起点不变 |
| ACCEPTED/IN_PROGRESS 时失效 | 先写 checkpoint；阻止新 effect；进入 REASSIGNING；保存草稿和证据，下一人显式接受 |
| WAITING 外部效果 Unknown | 不先改派执行同类 effect；先由原 Objective 进入 RECONCILING，改派人只能处理事故 |
| 已 COMPLETED、待上级 closure review | 完成事实保留；只重新解析未完成复核节点 |
| 审批人失效 | 已作出的有效决定不转移；未决定节点重新解析，继续执行 maker-checker 和历史 SoD 排除 |
| 无候选 | ESCALATED_NO_CANDIDATE；通知拥有 assignment.admin/escalation 能力的队列；不得扩大权限或跳过节点 |

显式 ReassignWorkItem 必须记录原因、原/新候选解释、当前权限和影响；新 assignment 不能超过新主体自己的 grant。新执行者不继承原执行者的会话、MFA、设备、临时权限或未提交本地草稿。

### 11.3 SLA 和草稿

1. 改派不重置 created_at、due_at、累计等待和升级次数。
2. 只有签名策略列明的 WAITING_CUSTOMER、WAITING_PART、WAITING_EXTERNAL 可暂停解决时钟；权限失效、无人候选和内部改派不暂停。
3. 已提交服务器草稿可按字段权限转交；无权字段必须裁剪，不能先发送再隐藏。
4. 端点本地草稿属于原设备/主体，不能自动转给下一人；原主体撤销后清除。
5. 无候选超过 SLA 时升级事故，但仍不自动审批、付款、签章或关闭。

### 11.4 改派验收

T-F57-AUT-004、T-F57-AUTH-003、T-F57-PLT-003 必须至少包含：

- runtime_candidate_resolution_uses_capability_not_rolecode
- revocation_during_execution_checkpoints_before_reassignment
- reassignment_never_expands_scope_or_inherits_session
- sod_excludes_maker_and_prior_incompatible_approvers
- no_candidate_escalates_without_auto_approval
- assignment_normal_chain_and_terminal_edges_are_closed
- assignment_and_current_assignee_use_full_principal_kind_and_id
- assignment_never_grants_authority
- sla_clock_does_not_reset_on_reassignment
- local_draft_is_not_transferred_and_server_draft_is_field_filtered

## 12. 指标公式登记

### 12.1 通用公式规则

1. 每个 MetricDefinition 保存 MetricID、formula_version、generation、来源事实、时间口径、过滤、分子、分母、空值规则和 drilldown contract。
2. 报表默认按业务事实的 legal_entity 和 Asia/Shanghai 业务日期分组；经营财务金额另按 F-50/F-57 经营期间归属。
3. 被冲销/撤销事实从净值中扣除但历史原值可下钻；迟到事实按当前经营期间入账并保留原业务日期。
4. 合成测试、容量探针和明确 test_data=true 的事实排除；真实取消、失败和重开不得为了美化指标排除。
5. 权限裁剪在服务器聚合前执行。无权字段不得用于排序、分组、分母或通过总计差值推断。
6. 除另有说明，比例分母为 0 时返回 NOT_APPLICABLE，不返回 0% 或 100%。
7. 所有时长使用可信 UTC 两端差值，界面按业务时区显示；墙钟异常期间没有可信证据的样本标为 DATA_QUALITY_INCIDENT。

### 12.2 当前公式

| MetricID | 公式 |
|---|---|
| MET-SERVICE-FIRST-RESPONSE-ON-TIME-RATE-V1 | 在窗口内 response_due_at 到期的有效工单中，first_accepted_at 小于等于 response_due_at 的数量 ÷ 同窗口有效工单数量。进入 TRIAGED 前合法取消的工单排除；重开不重置首次响应 |
| MET-SERVICE-RESOLUTION-ON-TIME-RATE-V1 | 在窗口内 resolution_due_at 到期的工单 cycle 中，closed_at 小于等于 due_at 的数量 ÷ 全部到期 cycle；到期仍开放计失败，获准暂停时长只按签名 SLA 策略顺延 |
| MET-SERVICE-REOPEN-RATE-30D-V1 | 已有完整 30 日观察窗的 CLOSED cycle 中，30 日内因第 6.10 节事实重开的数量 ÷ 同期具备观察窗的 CLOSED cycle 数量 |
| MET-SERVICE-FIRST-TIME-FIX-RATE-30D-V1 | 具备 30 日观察窗的 REPAIR cycle 中，关闭后 30 日内无重开、无同因复发且无需第二次现场访问的数量 ÷ 全部具备观察窗的 REPAIR cycle |
| MET-SERVICE-COST-V1 | 已确认未冲销的配件估价 + 已批准未冲销的工时成本 + 已批准未冲销的费用；逐项下钻来源，不以收费金额代替成本 |
| MET-SERVICE-CSAT-V1 | 已收到的 1 至 5 分有效响应总和 ÷ 有效响应数；不响应不记 0，同时必须展示 response_count 和 eligible_request_count |
| MET-OBJECTIVE-ON-TIME-CLOSURE-RATE-V1 | 在窗口内 due_at 到期的 Objective cycle 中，closed_at 小于等于 due_at 的数量 ÷ 全部到期 cycle；ABANDONED 和开放逾期均不算成功 |
| MET-OBJECTIVE-REOPEN-RATE-30D-V1 | 具备 30 日观察窗的 CLOSED cycle 中 30 日内重开的数量 ÷ 同期具备观察窗的 CLOSED cycle |
| MET-AUTOMATION-STRAIGHT-THROUGH-RATE-V1 | 无异常人工 work item、无人工作 override、无 HumanEffectDecision 且 CLOSED 的 eligible objective cycles ÷ 全部 eligible closed/abandoned cycles；业务设计本来要求的正常审批不算异常人工干预 |
| MET-AUTOMATION-UNKNOWN-EFFECT-RATE-V1 | 曾至少一次进入 Unknown 的唯一外部 effect 数 ÷ 已 DISPATCHED 的唯一外部 effect 数；同 effect 多次观察只计一次 |
| MET-AUTOMATION-COMPENSATION-RATE-V1 | 至少一个 compensation effect 已 Confirmed 的 objective cycle 数 ÷ 已开始的 objective cycle 数 |
| MET-AUTOMATION-MANUAL-INCIDENT-RATE-V1 | 出现 HumanEffectDecision、人工异常 override 或手工数据修复的 objective cycle 数 ÷ 已开始的 objective cycle 数；普通业务录入和设计内审批不计 |
| MET-AUTOMATION-OPEN-INCIDENT-BACKLOG-V1 | as_of 时刻状态非 RESOLVED/CLOSED 的 incident 数；必须同时按 age bucket、risk 和 owner capability queue 展示 |
| MET-PROCUREMENT-AWARD-CYCLE-HOURS-V1 | 每个 RFQRound 从 OPEN 到 AWARDED/NO_AWARD/CANCELLED/CANCELLED_BY_REVISION 的可信小时数；报 P50/P90 和样本数，取消或换轮不得从样本中隐藏 |
| MET-PROJECT-RISK-EXPOSURE-V1 | OPEN、MITIGATING、MONITORING、ACCEPTED 且 review 未过期风险的 likelihood × impact 之和；同时单列 exposure_amount_minor 总额，不能相互替代 |

### 12.3 公式变更和历史

公式只能经 draft→compile→simulate→approve→sign→publish generation 变更。新版本不覆盖旧报表结果；每次结果保存 formula_version、as_of、generation 和来源 watermark。跨版本趋势默认禁止拼接，除非报表显式重算全部期间并披露版本。

### 12.4 指标验收

T-F57-REP-002 和 T-F57-REP-003 必须至少包含：

- every_metric_matches_registered_formula_and_version
- zero_denominator_is_not_applicable
- open_overdue_items_are_not_hidden_from_rate
- reopen_observation_window_prevents_right_censor_bias
- reversals_and_late_facts_follow_source_and_period_rules
- unauthorized_rows_fields_do_not_enter_aggregate_or_inference
- every_result_drills_to_authorized_source_evidence

## 13. 四端能力矩阵

### 13.1 解释规则

“四端结果等价”是指相同主体、法人、业务事实、generation、授权、设备风险和命令载荷得到相同服务器结果、错误、审计和幂等行为。设备 posture 是授权输入，因此设备不合规造成的拒绝是预期安全结果，不是客户端功能分叉。

平台不得按固定岗位或 OS 名称授予业务权限。只有设备能力客观缺失、显示形态、批量交互和本文明确的控制面边界可以产生 UI 差异。

### 13.2 Workbench 能力

| 能力 | Windows | macOS | iOS | Android | 离线规则 |
|---|---|---|---|---|---|
| 任务、目标、异常、通知、授权查询 | 完整 | 完整 | 自适应完整 | 自适应完整 | 仅最小加密已选任务摘要；非权威 |
| 客户/合同/订单/采购/项目/服务查询 | 完整 | 完整 | 自适应完整 | 自适应完整 | 仅明确选取的最小加密投影；可撤销 |
| 创建和编辑业务草稿 | 完整 | 完整 | 分步表单 | 分步表单 | 可保存签名草稿/意图；重连重验 |
| 普通业务命令 | 在线完整 | 在线完整 | 在线完整 | 在线完整 | 不离线生效；可保存待提交意图 |
| 审批 | 在线完整 | 在线完整 | 在线完整 | 在线完整 | 只能草拟意见；最终决定在线 |
| 合同生效、付款/退款、开票/红冲、财务更正、敏感导出等高风险业务命令 | 在线，设备/金额/SoD/MFA 通过 | 同 Windows | 在线且合规设备策略允许时可提交 | 同 iOS | 永不离线生效 |
| 客户签收、现场验收、服务签字 | 可连接设备或上传证据 | 可连接设备或上传证据 | 相机/触控完整 | 相机/触控完整 | 可离线采集证据，服务器验签验权后生效 |
| 扫码、拍照、附件 | 设备存在时原生或文件上传 | 同 Windows | 原生完整 | 原生完整 | 加密临时附件；重连扫描后发布 |
| Excel/CSV 大批量导入映射和启动 | 完整 | 完整 | 只查看、审批、暂停/恢复服务端任务 | 同 iOS | 文件解析和权威执行只在服务端 |
| 大批量导出、复杂报表设计、打印模板设计 | 完整 | 完整 | 查看任务/结果；不提供复杂设计器 | 同 iOS | 不离线执行 |
| 报价/合同版本比较和文档模板编辑 | 完整 | 完整 | 只读比较、批注和审批 | 同 iOS | 草稿批注可暂存 |
| 服务工单、设备、配件、工时和现场证据 | 完整 | 完整 | 现场优化完整 | 现场优化完整 | 草稿/证据可离线，库存和成本效果在线 |
| 服务器配置、权限 generation、schema/迁移、能力包、密钥、备份、恢复 | 不属于 Workbench | 不属于 Workbench | 不属于 Workbench | 不属于 Workbench | 禁止；只在 Windows Server 控制中心 |

### 13.3 默认设备策略

1. 首发默认允许受管 Windows/macOS 执行经授权的敏感导出；iOS/Android 默认拒绝批量敏感导出，但客户可通过签名策略仅对满足 MDM、加密、屏幕锁、非 Root/Jailbreak、应用完整性和远程擦除的设备开放受限导出。
2. 付款、退款、合同生效、开票等业务命令不按 OS 永久禁用；必须在线，并由当前设备 posture、金额、风险、MFA、SoD 和 capability 决定。
3. PDF/打印模板、签章坐标和复杂版本合并只在桌面设计；四端均可查看 exact digest、发起签署请求和执行获准审批。签章私钥和实际签署效果始终在服务器/provider。
4. 配置、包、schema、密钥、备份和恢复不是业务 Workbench 权限，即使用户从 Windows Workbench 登录也不能执行。

### 13.4 四端验收

T-F57-CLI-002、T-F57-CLI-003、T-F57-CLI-005 必须至少包含：

- same_authority_context_has_same_result_error_and_audit_on_four_platforms
- device_policy_difference_is_server_explained_not_client_hardcoded
- high_risk_business_command_is_online_only_on_every_platform
- mobile_cannot_call_control_center_capabilities
- offline_money_quantity_state_permission_contract_changes_require_server_review
- desktop_mobile_layout_diff_does_not_change_business_payload
- bulk_job_can_be_monitored_on_mobile_without_local_execution

## 14. 四端员工 C/S 执行契约

### 14.1 唯一网络入口

1. 四端 Workbench 只连接签名 DeploymentManifest 中的 employee_api_origin。
2. employee API 是独立受保护 HTTPS 信任边界；core-server 继续只接受服务器内部受信路径，macOS/iOS/Android 不得连接 loopback core-server、PostgreSQL 或 Windows named pipe。
3. 客户端不得通过自定义 URL、系统代理重写、插件或模板改变 authority endpoint。
4. 员工 API gateway 只能把命令交给 Task 6 CommandPipeline，把查询交给授权投影；不得直写 repository。

### 14.2 唯一协议引用和请求信封

[F-57 客户端、生命周期与安全运营执行契约 §1](2026-08-23-f57-client-lifecycle-security-contract.md#1-员工-cs-在线协议) 是员工 API 路径、字段、会话、兼容和失败语义的唯一机器协议权威；本节只声明业务接线，不建立第二套 envelope、端点或状态。机器可读 IDL 必须从该唯一契约生成。

在线命令精确携带：`request_id`、`command_type`、`idempotency_key`、`expected_generation`、`expected_subject_version`、`generation_report`、`client_version`、`device_key_id`、`device_signature` 和类型化 `payload`。查询精确携带：`query_type`、`generation`、`generation_report`、类型化过滤条件、允许的 sort key、`page_size` 和不透明 cursor。`generation_report` 与握手/result directive 逐字段复用客户端契约 §1.2.1 的 `ClientGenerationReportV1`/`ClientGenerationDirectiveV1`，不建立业务侧别名。actor、当前法人、principal、session、设备、授权依据、policy、风险等级、MFA 与 SoD 结论全部来自服务端认证上下文；客户端不得声明或覆盖。

### 14.3 当前端点族和业务接线

第一阶段业务接线逐字复用客户端契约 §1.1 的 16 个 method/path pair：

- `POST /employee/v1/session/start`、`POST /employee/v1/session/handshake`、`POST /employee/v1/session/renew`、`POST /employee/v1/session/end`
- `POST /employee/v1/commands`、`GET /employee/v1/commands/{request_id}`、`POST /employee/v1/queries`、`GET /employee/v1/tasks/stream`
- `GET /employee/v1/ui-schema/{generation}`
- `POST /employee/v1/files/upload-sessions`、`GET /employee/v1/files/upload-sessions/{upload_id}`、`PUT /employee/v1/files/upload-sessions/{upload_id}/chunks/{chunk_no}`、`POST /employee/v1/files/upload-sessions/{upload_id}/complete`、`GET /employee/v1/files/{object_id}/versions/{version_id}`
- `POST /employee/v1/devices/{device_id}/attestations`、`POST /employee/v1/devices/{device_id}/wipe-receipts`

本清单必须与该唯一契约和 employee OpenAPI 机器相等；通配路径、可选别名、额外 method 或 `/employee/v1/schema/*` 旧称均不属于当前协议。

业务命令/查询类型属于强类型 registry，不允许任意 object/table/method 字符串。所有在线命令和重连后的 `ClientIntentV1` 都提交到 `/employee/v1/commands` 并进入 Task 6 唯一 `CommandPipeline`；不得建立 `/intents/replay` 旁路。查询只读取经服务器授权裁剪的投影。任务流只作刷新提示；断流后以权威 watermark 查询补齐，不能把流消息当业务事实。

### 14.4 结果、分页和兼容

结果精确携带：`correlation_id`、`authoritative_generation`、`generation_directive`、`subject_version`、`outcome`、`audit_ref` 和类型化值。`expected_generation`/query `generation` 必须等于 report 的 observed generation；高风险能力在 desired/observed/authoritative 或三个 digest 不兼容时零写入失败关闭。错误正文不得含表名、堆栈、SQL、路径、密钥、内部拓扑或无权对象存在性。

分页使用服务器签名 opaque cursor，至少绑定 principal、device、legal entity、query type、generation、授权摘要和过期时间；错绑定或过期明确失败，不能静默退回第一页，也不能由客户端修改 offset 绕过裁剪。

旧客户端只在客户端契约规定的签名 compatibility window 内使用；缺少必需高风险字段、generation 不兼容或 schema version 未知时失败关闭。四端 contract suite 必须从同一 IDL 和同一正反向黄金向量生成。

### 14.5 C/S 验收

T-F57-GOV-001、T-F57-CLI-002、T-F57-CLI-005 必须至少包含：

- four_clients_use_one_machine_readable_employee_api_contract
- client_cannot_assert_actor_policy_sod_or_authority_epoch
- command_route_reaches_single_command_pipeline
- cursor_is_bound_to_principal_scope_policy_and_generation
- task_stream_loss_recovers_from_authoritative_watermark
- stale_client_and_generation_fail_with_stable_actionable_error

### 14.6 API 状态域的现行补充裁决

员工、Control 和 Portal API 暴露的 `state`/`states` 不是可由 OpenAPI 作者临场命名的展示词。本表是状态语义的唯一权威；`docs/f57-api-state-domains.seed.tsv` 只保留为 G0 的历史导入快照，不得被重写、继续执行或作为第二真值。G0 建立语义合约机制后，所属节点必须把本表 exact-author 到 CapabilityGraph 的 `business_state_domain_registry_v1` 绑定，并只从该绑定生成现行机器投影、Rust、数据库约束、OpenAPI 和客户端。合同、销售订单、采购订单和设备沿用旧 PRD/阶段计划中未被 F-57 取代的精确语义，并由本节提升为 F-57 当前裁决。此前只有枚举、没有完整边或派生规则的状态域按下表收口。表内 `A→B|C` 表示从 A 只允许进入 B 或 C；未列边、终态出边、直接 SQL 改状态和用新名字替代旧状态一律拒绝。自环只在表中明确列出时允许，而且只能追加失败/重试事实，不能覆盖历史。

<!-- F57-SEMANTIC-TABLE:business_state_domain_registry_v1:BEGIN -->
| StateDomain | 语义种类与初态 | 允许边或唯一派生规则 | 终态 |
|---|---|---|---|
| `CONTRACT_V1` | 持久生命周期；`DRAFT` | `DRAFT→PENDING_APPROVAL\|VOID`；`PENDING_APPROVAL→DRAFT\|PENDING_SIGNATURE\|REJECTED`；`REJECTED→DRAFT\|VOID`；`PENDING_SIGNATURE→EFFECTIVE\|REJECTED`；`EFFECTIVE→EFFECTIVE\|IN_PERFORMANCE\|TERMINATING`；`IN_PERFORMANCE→COMPLETED\|TERMINATING`；`TERMINATING→TERMINATED\|TERMINATING`。`EFFECTIVE` 自环只表示派生失败后保持原态，`TERMINATING` 自环只表示处置失败后保持原态。 | `COMPLETED\|TERMINATED\|VOID` |
| `SALES_ORDER_V1` | 持久生命周期；`PENDING_RELEASE` | `PENDING_RELEASE→CANCELLED\|RELEASED`；`RELEASED→CANCELLED\|CHANGE_APPROVAL\|DELIVERED\|PARTIALLY_DELIVERED`；`CHANGE_APPROVAL→PARTIALLY_DELIVERED\|RELEASED`，目标取进入审批前状态；`PARTIALLY_DELIVERED→CHANGE_APPROVAL\|CLOSED\|DELIVERED`。 | `CANCELLED\|CLOSED\|DELIVERED` |
| `PURCHASE_ORDER_V1` | 持久生命周期；`DRAFT` | `DRAFT→PENDING_APPROVAL\|VOIDED`；`PENDING_APPROVAL→CLOSED\|ISSUED\|REJECTED`；`REJECTED→CLOSED\|DRAFT`；`ISSUED→CLOSED\|PENDING_SUPPLIER_CONFIRM`；`PENDING_SUPPLIER_CONFIRM→SUPPLIER_CONFIRMED\|SUPPLIER_RESCHEDULE_PROPOSED\|VOIDED`；`SUPPLIER_RESCHEDULE_PROPOSED→PENDING_SUPPLIER_CONFIRM\|SUPPLIER_CONFIRMED\|VOIDED`；`SUPPLIER_CONFIRMED→CLOSED\|COMPLETED\|PARTIALLY_RECEIVED\|PENDING_SUPPLIER_CONFIRM`；`PARTIALLY_RECEIVED→CLOSED\|COMPLETED`；`COMPLETED→CLOSED\|SUPPLIER_CONFIRMED`。（F-63 补两条边：`SUPPLIER_CONFIRMED→PENDING_SUPPLIER_CONFIRM` 承接 PRD:1582 逐字「变更后订单回到待供应商确认状态」的改单回退；`COMPLETED→SUPPLIER_CONFIRMED` 承接计划 07:1104 逐字「红字减少后…COMPLETED 订单恢复 SUPPLIER_CONFIRMED」的红冲重开；**部分收货态的改单回退无逐字依据、本轮不加边**，仍待裁。）VOIDED 守卫为尚无收货、采购发票或已批准付款效果；CLOSED 必须保留已发生事实。 | `CLOSED\|VOIDED` |
| `EQUIPMENT_V1` | 持久生命周期；`IN_STOCK` | `IN_STOCK→IN_SERVICE\|RETURNED\|SCRAPPED`；`IN_SERVICE→RETURNED\|SCRAPPED\|UNDER_REPAIR`；`UNDER_REPAIR→IN_SERVICE\|RETURNED\|SCRAPPED`。变更只留设备审计，不制造库存或财务事实。 | `RETURNED\|SCRAPPED` |
| `APPROVAL_CASE_V1` | 持久生命周期；`OPEN` | `OPEN→APPROVED\|CANCELLED\|EXPIRED\|REJECTED`。一个结论只能写一次；重提建立新 case。 | `APPROVED\|CANCELLED\|EXPIRED\|REJECTED` |
| `COST_ENTRY_V1` | 持久生命周期；`DRAFT` | `DRAFT→SUBMITTED`；`SUBMITTED→APPROVED\|REJECTED`；`APPROVED→POSTED`；`POSTED→REVERSED`。拒绝后修改建立新版本；已过账更正只能追加反向/更正事实。 | `REJECTED\|REVERSED` |
| `IMPORT_PROPOSAL_V1` | 持久生命周期；`DRAFT` | `DRAFT→FAILED_CONTAINED\|VALIDATED`；`VALIDATED→FAILED_CONTAINED\|PENDING_APPROVAL`；`PENDING_APPROVAL→APPROVED\|FAILED_CONTAINED\|REJECTED`；`APPROVED→APPLIED\|FAILED_CONTAINED`。失败隔离保留原始文件 digest、逐行结果和补偿证据。 | `APPLIED\|FAILED_CONTAINED\|REJECTED` |
| `MAINTENANCE_PLAN_V1` | 持久生命周期；`DRAFT` | `DRAFT→ACTIVE\|CANCELLED`；`ACTIVE→CANCELLED\|COMPLETED\|PAUSED`；`PAUSED→ACTIVE\|CANCELLED\|COMPLETED`。暂停不改写已生成 occurrence。 | `CANCELLED\|COMPLETED` |
| `MAINTENANCE_OCCURRENCE_V1` | 持久生命周期；通常初态 `PLANNED`；停机恢复补建的漏期 occurrence 初态直接为 `OVERDUE` | `PLANNED→CANCELLED\|DUE`；`DUE→CANCELLED\|IN_PROGRESS\|OVERDUE`；`OVERDUE→CANCELLED\|IN_PROGRESS`；`IN_PROGRESS→CANCELLED\|COMPLETED`。合同/计划提前终止只以 typed `PlanTerminationEvidence` 进入 CANCELLED；设备异常打开 incident 并保持应有未完成状态；重复计划生成新的 occurrence，不重开旧终态。 | `CANCELLED\|COMPLETED` |
| `PORTABLE_EXPORT_V1` | 持久生命周期；`PENDING_APPROVAL` | `PENDING_APPROVAL→APPROVED\|EXPIRED\|REJECTED`；`APPROVED→EXPIRED\|PREPARING`；`PREPARING→EXPIRED\|FAILED_CONTAINED\|READY`；`READY→EXPIRED`。READY 只在导出包、清单、密钥包装和审计证据全部原子发布后成立。 | `EXPIRED\|FAILED_CONTAINED\|REJECTED` |
| `PROJECT_RECEIPT_MILESTONE_V1` | 只读派生分类；初值由当前 owner facts 决定 | 禁止状态命令。优先级固定为：有效合同变更已取消节点且既有效果处置完毕=`CANCELLED`；否则有效到款核销全额覆盖=`PAID`；否则有效到款+批准减免/核销/法律消灭全额覆盖且 waiver 为正=`WAIVED`；否则有效已开票覆盖=`INVOICED`；否则前置义务未满足=`BLOCKED`；否则服务器业务日达到到期日=`DUE`；否则=`READY`。coverage 不得重复或超额。合同取消被取代、waiver 撤销/更正、付款核销释放/冲销、发票红冲或前置义务反向时按同一优先级重新派生，历史事实不改写。 | 无；包括 `CANCELLED\|PAID\|WAIVED` 在内的任何结果都可因合法反向事实重新派生 |
| `SATISFACTION_V1` | 持久生命周期；`OPEN` | `OPEN→CLOSED\|RECORDED\|WAITING_RESPONSE`；`WAITING_RESPONSE→CLOSED\|RECORDED`；`RECORDED→CLOSED`。窗口到期的受控无响应证据才可 CLOSED；迟到相反证据建立新 follow-up cycle。 | `CLOSED` |
| `SERVICE_EVIDENCE_V1` | 两阶段证据裁决；创建结果为 `VERIFIED` 或 `REJECTED` | 文件/签名/归属/时间验证失败直接创建 `REJECTED`；通过则创建 `VERIFIED`，随后只有 `VERIFIED→ACCEPTED\|REJECTED`。证据字节不可改；纠正须新建 evidence ID 并引用旧项。 | `ACCEPTED\|REJECTED` |
| `SERVICE_REQUEST_V1` | 持久生命周期；`OPEN` | `OPEN→CANCELLED\|TRIAGED`；`TRIAGED→ACCEPTED\|CANCELLED`；`ACCEPTED→CANCELLED\|CLOSED`。ACCEPTED 后的取消必须闭合已派生责任和效果，不能删除工单或证据。 | `CANCELLED\|CLOSED` |
<!-- F57-SEMANTIC-TABLE:business_state_domain_registry_v1:END -->

这四列不是运行时要再次解释的自由文本。G0 的冻结 adapter 必须 exact-match headers/codecs `StateDomain:MARKDOWN_CODE_UPPER_TOKEN`、`语义种类与初态:UTF8_EXACT`、`允许边或唯一派生规则:UTF8_EXACT`、`终态:UTF8_EXACT`，然后由唯一 `STATE_INVARIANT_REGISTRY_V1` validator 把完整四-cell tuple 归一化为 strict `StateDomainDefinitionV1` JCS object。对象至少显式含 `domain_id,mode,initial_or_output_states,transitions,derived_precedence,guard_ids,invariant_ids,terminal_states,reverse_fact_triggers`；不适用字段是 schema 定义的 exact 空数组而非遗漏/自由 prose。14 个归一化对象必须逐字节等于 G0 的独立 reviewed golden；Graph、投影、Rust、SQL/OpenAPI/client 都只消费这些对象，禁止消费三列中文原文。任何无法无损归一化的句子、未具名 guard/invariant/reverse fact、header/codec 漂移或把整段文字保存成 UTF8 semantic value 都阻止该 graph version 激活。

G0（所有权桶 `F57-01`）必须先逐字节导入并封存 `docs/f57-api-state-domains.seed.tsv`，再建立 CapabilityGraph 语义绑定、行 schema、唯一投影和漂移门；它不得为了追上本表而改写历史 seed。G4/G5 的对应 owner 必须把本表 exact-author 为 `business_state_domain_registry_v1`，使生成的现行机器合约、Rust 枚举、数据库 CHECK/派生查询、OpenAPI 和客户端逐域 exact-join。若该绑定或现行投影仍给 `MAINTENANCE_OCCURRENCE_V1` 暴露跳过状态，或未给 `PROJECT_RECEIPT_MILESTONE_V1` 暴露 WAIVED/CANCELLED，则候选失败关闭；历史 seed 保持原字节并不构成漂移。对持久生命周期须穷举每条允许边、每条未列边和每个终态出边；对派生分类须证明不存在公开状态写命令并覆盖所有事实组合、反向事实和优先级并存组合。维护验收必须额外覆盖停机跨多期逐期 OVERDUE、typed termination→CANCELLED 和设备异常→incident；收款节点验收必须覆盖七态 exact-set、closure coverage、防重复/超额、全部优先级并存组合和每类反向事实。任何现行图/投影/实现来源缺失或互相不等时，G0/所属节点失败关闭，不允许以“仅展示状态”为由放行。

## 15. 首发地域、语言、币种和数据驻留

### 15.1 冻结取值

第一阶段：

- UI 与业务文档语言：zh-CN；
- 业务币种和本位经营币种：CNY；
- 业务时区：Asia/Shanghai；
- 数据驻留档：CN_MAINLAND_ONLY_V1；
- 不支持多币种、外汇、进出口、报关、信用证和产品内容多语言。

### 15.2 驻留范围

CN_MAINLAND_ONLY_V1 覆盖：

- PostgreSQL data/WAL/temp；
- 附件、客户能力包、配置代、审计、日志、索引、报表、导出和隔离区；
- 服务器外连续备份、离线备份保管地点和恢复演练环境；
- 监控、远程支持、消息/邮件、MCP、AI、OCR 和其他 provider 可能接收的客户或可关联客户数据。

任一 provider、IaaS 区域、备份目标或支持路径不能证明位于中国大陆且符合当前外发策略时，必须保持关闭或标为不合格；不得以“客户自建”自动视为满足。

### 15.3 部署证据

DeploymentManifest/BackupEvidence 必须记录 residency_profile、country_or_region、carrier/provider/endpoint、数据类别、备份/日志路径、证据 digest、verified_at 和 expires_at。ProviderManifest 的同一承诺必须逐字段使用 [ADR-0023 §2.1](../../adr/ADR-0023-f57-provider-manifest-resource-grant.md#21-processinglocationevidencev1-exact-schema) 的 `ProcessingLocationEvidenceV1`，不得另建自由地区字符串或外部附件别名。region 缺失、`UNKNOWN`、数据类别/endpoint 未 exact-join、跨境 endpoint、重定向/代理到境外或证据过期时阻止真实客户数据和对应 capability 激活。

该地域约束是当前产品承诺，不是开发人员可选择的部署默认。未来国际化必须新建明确版本和数据迁移/合规设计，不能修改 CN_MAINLAND_ONLY_V1 的含义。

## 16. 当前延期 exact 边界和产品文字解释

### 16.1 当前不得激活或宣称可用

下表是面向产品文字和菜单扫描的 **12 行 operational alias exact-set**。它不是新增 RequirementID 集；每行必须通过 `canonical_requirement_id` 绑定现有 185 行稳定需求。机器执行的 11 行边界 RequirementID exact-set 以 [客户端、生命周期与安全运营执行契约 §11](2026-08-23-f57-client-lifecycle-security-contract.md#11-延期能力-exact-registry-gov-010) 为唯一权威，两套集合通过此列关联，不得按名称猜测或继续追加未登记 token。

| Capability token | Canonical RequirementID | 当前状态 | 允许的接口 |
|---|---|---|---|
| DEF-LEAD-MARKETING | CRM-003 | DEFERRED | CRM 可接收类型化 customer/opportunity 输入；不提供线索、活动、漏斗营销、渠道佣金、销售预测 |
| DEF-CPQ-COMPLEX | CPQ-001 | DEFERRED | 基础报价版本当前；复杂配置器、成本模型、返利和部分接受延期 |
| DEF-PROC-TENDER-VMI | PROC-003 | DEFERRED | RFQ/询比价当前；正式招投标、VMI 和复杂供应商绩效模型延期 |
| DEF-SALES-CONSIGNMENT-SUBSCRIPTION-LEASE | DEF-009 | DEFERRED_WITH_INTERFACE | 只提供 provider seam；未认证不得出现可执行菜单/route/营销声明 |
| DEF-MRP-MES-APS | DEF-002 | DEFERRED_WITH_INTERFACE | 外部生产只能创建标准采购需求；不自研完整制造 |
| DEF-WMS-ADVANCED | DEF-003 | DEFERRED_WITH_INTERFACE | 基础库存和 `SRV-006` 服务配件预留当前；高级 WMS 的波次、拣货、盘点、质检、销售分配库存预留、调拨和自动立库延期，仅保留版本化 provider 接口 |
| DEF-SERVICE-BILLING-EAM | SRV-009 | DEFERRED | 服务权益、成本和一次性收费提案当前；周期计费引擎、预测维护、完整 EAM 延期 |
| DEF-PPM-EVM | DEF-005 | DEFERRED_WITH_INTERFACE | 基础项目、风险、成本和收款节点当前；完整 WBS/资源/预算变更/EVM 延期 |
| DEF-STATUTORY-FINANCE | DEF-004 | DEFERRED_WITH_INTERFACE | 内部经营分录当前；法定总账/凭证账簿/税务/工资/法定年结走专业系统 |
| DEF-HR-GRC-LEGAL-TRAVEL-ECM-GIS-PLM-PIM-QMS | GOV-010 | DEFERRED | 不创建模块、菜单、route 或营销声明 |
| DEF-DEALER-EMPLOYEE-PORTAL | GOV-010 | DEFERRED | 客户/供应商门户当前；其他门户不恢复 |
| DEF-LOCAL-MODEL-OCR-RAG-KG | DEF-001 | DEFERRED_WITH_INTERFACE | AI/provider 契约当前，本地模型和 OCR/RAG/知识图谱实现延期 |

发布门禁必须断言本表 token 恰为上述 12 个且无增删重名，并对每行验证 canonical RequirementID 存在、没有可安装当前包、没有 current route/menu、没有默认启用 provider、没有“已支持”产品文字。受控 seam 不等于已实现；新增延期项必须先成为稳定需求并更新两份 exact registry，不能只追加文字或配置。

### 16.2 产品文字单义解释

“全过程在同一套系统内完成，不依赖外部系统或线下台账”的现行单义解释为：

> 平台原生的客户、商业、履约、经营资金、发票登记、服务、证据和管理闭环不依赖表外台账；法定财税、完整制造、银行税务在线能力等明确边界由专业系统通过受治理接口完成。

“凭证与账簿”的现行单义解释为：

> 不可变、平衡的内部经营分录、经营科目映射、试算和业务台账；不表示法定科目、法定凭证账簿、税务申报、工资或法定年结。

任何产品介绍、UI、报表、导出、合同附件和销售材料必须使用上述边界，不得以简称恢复被延期的法定或专业系统能力。

## 17. 实施 owner、测试和停止门

### 17.1 Owner task

| 契约章节 | persistence/contract owner task | 行为激活 task |
|---|---|---|
| CRM、CPQ、订单来源 | Task 3 | Task 19；经济闭环续由 Task 20 |
| 采购/RFQ/授标 | Task 4 | Task 20 |
| 服务、项目、指标 | Task 5 | Task 21 |
| Objective 登记、Unknown | Task 12 | Tasks 19–21 使用真实 handler 再认证 |
| 客户门户身份 | Task 5 | Task 22 |
| 动态改派 | Task 8/12 | Tasks 19–23 的真实业务场景 |
| 四端矩阵和 C/S | Task 17/18 | Tasks 19–23 的共享 contract suite |
| 地域/驻留 | Task 2 | Task 24/25 实机和发布证据 |
| 延期 exact gate | G0 登记 | G6/L3 最终发布门 |

### 17.2 强制停止条件

对应任务遇到以下任一情形必须停止，不得由实现者自行选择：

- 状态或命令无法映射到本文闭集；
- 来源 exact-one、数量/金额守恒或 owner 边界无法由数据库/类型系统证明；
- 业务关闭依赖人工自由文本或通用 MarkDone；
- Unknown 只能靠猜测才能清空；
- 客户门户身份不能确定到单一 binding；
- 改派需要扩大权限或固定岗位兜底；
- 指标公式缺来源、分母或版本；
- 四端需要各自发明载荷或错误语义；
- 境外/未知 region 才能激活当前数据路径；
- 延期 token 出现当前菜单、route、包或产品承诺。

出现停止条件时，允许动作只有：补充与本文一致的实现细节、登记稳定错误/事件/测试，或提交新的显式 ADR/权威裁决。禁止在业务代码、SQL、UI 或配置中暗自选择另一语义。

### 17.3 最终证据

每个 RequirementID 的最终 evidence 必须同时包含：

- 状态机正反例；
- 数据库 exact-one/守恒/同法人/并发约束；
- 动态授权和 SoD 负例；
- 重复、乱序、断电、重启、撤权和 generation 变化；
- Unknown、人工处置、补偿和相反迟到回调；
- closure predicate 每个 obligation 缺失时拒绝；
- 重开保留旧 cycle；
- 四端共享 contract 向量；
- 授权报表公式和证据下钻；
- 当前延期能力不可见、不可调用、不可宣称；
- 对应 Windows/PG16/备份/驻留证据（适用时）。

## 18. 最终冻结结论

本文已经关闭以下产品选择：

- CRM 商机/跟进与 CPQ 报价版本的对象、状态和转换；
- 客户报价首发不支持部分接受，变化必须新报价版本；
- 合同、报价、人工三种订单来源 exact-one，以及无合同时的完整商业快照；
- 六种采购来源、RFQ round、供应商报价版本、比较、部分/多供应商授标和重开；
- 五类服务工单、权益、配件、工时、成本、CAPA、满意度和周期维保；
- 项目风险、成本与收款节点边界；
- 当前业务 ObjectiveKind 的关闭、ABANDONED 和重开规则；
- Unknown 的四种人工决定和高风险证据/双人门禁；
- 客户门户邀请、主体、binding、MFA、恢复、合并和终止；
- 动态责任解析、失效、改派、无候选和 SLA；
- 服务、闭环、自动化、采购与项目指标公式；
- Windows、macOS、iOS、Android 的业务能力、自适应和离线边界；
- 员工 C/S 的唯一 HTTPS 入口和共享请求/响应契约；
- zh-CN、CNY、Asia/Shanghai 和 CN_MAINLAND_ONLY_V1；
- 当前延期能力的 exact 禁用和产品文字边界。

开放产品选择为 0。部署证书、客户 ID、实际人员、实际 provider endpoint、具体金额阈值和数据量是运行输入或证据，不是产品语义选择。当前实现状态仍为 NOT_IMPLEMENTED。
