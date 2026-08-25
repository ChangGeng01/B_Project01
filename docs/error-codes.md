# 错误码表

> **F-57 状态：`REGISTRY_PENDING_REBASELINE`。** 既有 **495** 条错误码保持为 legacy 登记；第 14 节另预登记 **27** 条 F-57 代码，因此本文件按状态统计共 **522** 条。预登记只建立名称、分类和返回契约，不表示代码、API 或运行时已经实现，也不表示全部 F-57 门禁已经闭合。

本文件是错误码的唯一状态化登记处。legacy 行与 F-57 预登记行均须保持唯一；代码侧对应物只有在所属任务实现后才允许出现，并由 `xtask errorcodes` 逐项比对，重复码、遗漏已激活代码或未登记引用均构建失败。

新增错误码的顺序是先登记后实现：先在本文件加行，再在常量表加常量，最后才允许有代码返回它。反过来做会让本文件变成一份滞后的注释。

## 1. 命名与分类

错误码原则上为不少于三段的点分大写，形如 `<MODULE>.<RESOURCE_PATH...>.<REASON>`。模块段通常取技术基线第 1.2 节的 15 个业务模块码之一或 `PLATFORM`；F-55 的非业务技术边界另固定使用 `AI`、`MCP`、`OPS` 三个命名空间，不能据此增加 `ModuleCode`。中间一至多段是稳定的业务资源路径，末段为原因短语。资源存在明确子对象时可使用四段或更多段（例如 `PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED`），不得为了满足三段形式把不同子对象压成含义不清的单段。唯一冻结例外是 F-55 ABI 已批准的两段码 `MCP.RATE_LIMITED`；CI 必须断言两段例外集合**恰好**等于这一项，其他错误码仍须至少三段，后续不得再增加两段码。

分类与其默认 HTTP 状态、可重试性按技术基线第 5.5 节：

| category | 含义 | HTTP | retryable |
|---|---|---|---|
| VALIDATION | 输入校验错误，定位到字段 | 400 | false |
| BUSINESS_CONFLICT | 业务冲突，含版本冲突、状态机非法迁移、守恒与勾稽不成立 | 409 | false |
| PERMISSION_DENIED | 权限或策略拒绝 | 403，无权访问已存在记录时统一 404 | false |
| EXTERNAL_SYSTEM | 外部系统故障；首版业务集成仅电子签章，F-55 受控远端 MCP 也归此类 | 502 | true |
| INFRASTRUCTURE | 基础设施故障，含数据库不可用、磁盘写满、限流 | 503，限流 429 | true |

存在性泄漏的统一处理：对当前安全上下文不可见的记录，读、写、删一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与无权。只有当前用户对该对象类型完全无权时才返回 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。`PLATFORM.ROUTE.NOT_FOUND` 只用于路由本身不存在，与上面两条不得互换。

## 2. PLATFORM 段

本节基础段共 **73 条**：原分阶段登记累计 65 条（阶段 1 登记 14 条，阶段 2 任务 #12〔ep-adapter-kms〕登记 8 条、任务 #11〔ep-adapter-db-pg〕登记 3 条、任务 #14〔集成 B〕登记 10 条，阶段 3a 任务 #18 登记 1 条，阶段 4 任务 #20 与任务 #22 登记 27 条，阶段 3b 任务 #21〔ep-platform-license〕与任务 #14〔ep-platform-flow 守卫求值器〕各登记 1 条），全局收口追加 `PLATFORM.IPC.CONCURRENCY_LIMIT` 与 `PLATFORM.AUTHN.RATE_LIMITED` 2 条，F-56 再追加许可证/模块包 6 条。阶段 14 的 33 条 PLATFORM 码另见第 11 节，不混入本节 73 条。「返回方」一列写的是第一个真正会返回该码的阶段；标注为阶段 1 之后的，阶段 1 只登记不返回，任何阶段不得因为「还没人返回」而删除该行。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 |
|---|---|---|---|---|---|
| PLATFORM.SYSTEM.NOT_READY | INFRASTRUCTURE | 503 | true | 进程未就绪或自检未通过 | 阶段 1 |
| PLATFORM.SYSTEM.SYNC_TIMEOUT | INFRASTRUCTURE | 503 | true | 同步等待超过 8 秒且尚无后台任务承接 | 阶段 1 |
| PLATFORM.SYSTEM.INTERNAL_ERROR | INFRASTRUCTURE | 503 | true | 未预期错误与 panic 捕获，消息为固定占位文案 | 阶段 1 |
| PLATFORM.REQUEST.INVALID_PAYLOAD | VALIDATION | 400 | false | JSON 解析失败或字段校验失败，details 定位到字段 | 阶段 1 |
| PLATFORM.REQUEST.HEADER_MISSING | VALIDATION | 400 | false | 固定请求头缺失或格式非法 | 阶段 1 |
| PLATFORM.ROUTE.NOT_FOUND | PERMISSION_DENIED | 404 | false | 路由不存在，与无权访问已存在记录同码形态，避免存在性泄漏 | 阶段 1 |
| PLATFORM.IDEMPOTENCY.KEY_REQUIRED | VALIDATION | 400 | false | 写请求缺 `Idempotency-Key`，或该头不是合法 UUIDv7 | 阶段 1 |
| PLATFORM.CAPACITY.CONCURRENCY_LIMIT | INFRASTRUCTURE | 503 | true | 并发闸门等待超过 10 秒 | 阶段 1 |
| PLATFORM.IPC.CONCURRENCY_LIMIT | INFRASTRUCTURE | 503 | true | 已核验的管道客户端账户达到其固定活跃连接上限，服务端在读取应用帧前拒绝 | 阶段 1 |
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 同一幂等键上的请求体哈希与首次调用不一致 | 阶段 1 |
| PLATFORM.CONCURRENCY.STALE_VERSION | BUSINESS_CONFLICT | 409 | false | 乐观锁版本过期，更新影响行数为零 | 阶段 1 |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | PERMISSION_DENIED | 404 | false | 记录不存在与无权访问已存在记录，同码同形态 | 阶段 4 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | PERMISSION_DENIED | 403 | false | 对象已对当前主体可见但该动作被拒 | 阶段 4 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | BUSINESS_CONFLICT | 409 | false | 未持有迁移窗口即执行在线变更 | 阶段 13b |
| PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED | VALIDATION | 400 | false | 取号时给出的类型码不在本节第 5 章的登记表内 | 阶段 3a |
| PLATFORM.LICENSE.RESTRICTED | BUSINESS_CONFLICT | 409 | false | 当前许可为受限运行，常规业务写入、普通业务审批、集成出站或新自动化任务不在 F-56 恢复/保留闭集内 | 阶段 3b（F-56）；全部受限写入口传播 |
| PLATFORM.MODULE.TRANSITION_INVALID | BUSINESS_CONFLICT | 409 | false | 模块安装态状态机上的非法迁移，五条合法动作见 F-56 第 4.2 节 | 阶段 3b（F-56） |
| PLATFORM.MODULE.LICENSE_REQUIRED | BUSINESS_CONFLICT | 409 | false | 当前有效已验签许可未覆盖目标模块与编译期依赖闭包，拒绝部署级 INSTALL、ENABLE、UPGRADE 或 ROLLBACK_VERSION；DISABLE 不使用本码 | 阶段 3b（F-56） |
| PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE | BUSINESS_CONFLICT | 409 | false | 内外签名与 signer trust 已通过，但产品版本、模块契约摘要、维护期、历史 package identity 或兼容性不成立；密码学/信任失败不得使用本码 | 阶段 3b（F-56） |
| PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT | INFRASTRUCTURE | 503 | true | 模块独占动作等待 core/worker 共享持有者排空超过 30 秒，整笔发布已回滚 | 阶段 3b（F-56） |
| PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID | BUSINESS_CONFLICT | 409 | false | LICENSE_GRANT/MODULE_PACKAGE 不是 imported、单项、ADD、全局范围或含未知/混合内容 | 阶段 13b（F-56） |
| PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM | BUSINESS_CONFLICT | 409 | false | 对含 LICENSE_GRANT/MODULE_PACKAGE 的发布创建通用 ROLLBACK；只能发布后继签名动作 | 阶段 13b（F-56） |
| PLATFORM.FLOW.GUARD_EXPRESSION_INVALID | VALIDATION | 400 | false | 守卫条件表达式的解析、语义或上限校验未通过，含求值步数超过上限 | 阶段 3b |
| PLATFORM.KEY_DOMAIN.NOT_PROVISIONED | INFRASTRUCTURE | 503 | true | 当前法人尚无任何 `key_domains` 行；仅此零行条件使用本码 | 阶段 2 |
| PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE | INFRASTRUCTURE | 503 | true | 已有 `PROVISIONING/ACTIVE` 密钥域，但 KEK/DEK、KMS、readback、16-key 矩阵或 activation audit 缺失、不可用或不一致 | 阶段 2 |
| PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 同一域同一 purpose 已有轮换在途 | 阶段 2 |
| PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED | BUSINESS_CONFLICT | 409 | false | 销毁前核验五项任一缺失 | 阶段 2 |
| PLATFORM.KEY_DOMAIN.TRANSITION_INVALID | BUSINESS_CONFLICT | 409 | false | 密钥域或数据密钥状态机非法迁移 | 阶段 2 |
| PLATFORM.CRYPTO.DECRYPT_FAILED | INFRASTRUCTURE | 503 | true | 解密失败，含引用已销毁数据密钥的密文 | 阶段 2 |
| PLATFORM.CRYPTO.AAD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 认证标签校验失败，AAD 与当前行不符 | 阶段 2 |
| PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID | VALIDATION | 400 | false | 信封魔数、长度或算法标识非法 | 阶段 2 |
| PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED | INFRASTRUCTURE | 503 | true | 并发冲突重试用尽，或事务已产生外部可见副作用而不可重试 | 阶段 2 |
| PLATFORM.DB.REFERENCED_ROW_MISSING | VALIDATION | 400 | false | 写入引用的记录不存在，details 定位外键列与约束名 | 阶段 2 |
| PLATFORM.DB.WRITE_SCALE_VIOLATION | VALIDATION | 400 | false | 写入数值超出列声明的精度范围 | 阶段 2 |
| PLATFORM.DB.RLS_CONTEXT_MISSING | INFRASTRUCTURE | 503 | true | 会话变量缺失或写入失败，无法取得法人隔离上下文 | 阶段 2 |
| PLATFORM.DB.LEGAL_ENTITY_MISMATCH | PERMISSION_DENIED | 403 | false | 写入行的法人标识与安全上下文法人不一致 | 阶段 2 |
| PLATFORM.DB.POOL_EXHAUSTED | INFRASTRUCTURE | 503 | true | 连接池在取用超时内无可用连接 | 阶段 2 |
| PLATFORM.DB.STATEMENT_TIMEOUT | INFRASTRUCTURE | 503 | true | 语句执行超过所在池的 statement_timeout | 阶段 2 |
| PLATFORM.DB.LOCK_TIMEOUT | BUSINESS_CONFLICT | 409 | false | 锁等待超过 lock_timeout，未获得所需锁 | 阶段 2 |
| PLATFORM.DB.MIGRATION_VERSION_MISMATCH | INFRASTRUCTURE | 503 | true | 迁移历史版本与二进制期望版本不一致 | 阶段 2 |
| PLATFORM.DB.MIGRATION_WINDOW_CONFLICT | BUSINESS_CONFLICT | 409 | false | 已有开启中的迁移窗口，或关窗请求指向的窗口不符 | 阶段 2 |
| PLATFORM.DB.APPEND_ONLY_VIOLATION | BUSINESS_CONFLICT | 409 | false | 对仅追加表执行 UPDATE 或 DELETE 被拒 | 阶段 2 |
| PLATFORM.DB.ROW_VERSION_NOT_BUMPED | BUSINESS_CONFLICT | 409 | false | UPDATE 未按触发器要求将 row_version 加一 | 阶段 2 |
| PLATFORM.SENSITIVE_FIELD.NOT_REGISTERED | VALIDATION | 400 | false | 敏感字段未在登记表内即发起加密写入 | 阶段 2 |
| PLATFORM.IDEMPOTENCY.IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 同一幂等键上已有请求在途未完，并发去重拒绝 | 阶段 3a |
| PLATFORM.AUTHN.CREDENTIAL_INVALID | PERMISSION_DENIED | 403 | false | 账号或口令校验不通过，连续失败计数在同一桶内递增 | 阶段 4 |
| PLATFORM.AUTHN.ACCOUNT_LOCKED | BUSINESS_CONFLICT | 409 | false | 连续失败达到上限进入锁定窗口，窗口结束前拒绝登录 | 阶段 4 |
| PLATFORM.AUTHN.ACCOUNT_INACTIVE | BUSINESS_CONFLICT | 409 | false | 账号处于停用或停用待生效状态，拒绝登录 | 阶段 4 |
| PLATFORM.AUTHN.MFA_REQUIRED | BUSINESS_CONFLICT | 409 | false | 口令校验通过但需追加第二因子，要求提交 MFA 验证码 | 阶段 4 |
| PLATFORM.AUTHN.MFA_INVALID | VALIDATION | 400 | false | MFA 验证码格式非法或与挑战不符 | 阶段 4 |
| PLATFORM.AUTHN.MFA_CHALLENGE_EXPIRED | BUSINESS_CONFLICT | 409 | false | MFA 挑战超过有效期，需重新发起 | 阶段 4 |
| PLATFORM.AUTHN.MFA_CHALLENGE_CONSUMED | BUSINESS_CONFLICT | 409 | false | MFA 挑战已被成功消费；成功响应丢失时须重新登录 | 阶段 4 |
| PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 试图停用最后一个已登记的 MFA 因子 | 阶段 4 |
| PLATFORM.AUTHN.DEVICE_NOT_REGISTERED | PERMISSION_DENIED | 403 | false | 受管客户端的设备标识不在登记册内 | 阶段 4 |
| PLATFORM.AUTHN.RATE_LIMITED | INFRASTRUCTURE | 429 | true | 内部或门户登录、MFA 完成尝试超过认证前双维度速率上限 | 阶段 4 |
| PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED | PERMISSION_DENIED | 403 | false | 会话法人不在该账号被授予的法人集合内 | 阶段 4 |
| PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN | PERMISSION_DENIED | 403 | false | 隔离受控操作被拒，含跨法人与隔离管理面动作 | 阶段 4 |
| PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN | PERMISSION_DENIED | 403 | false | 越过应用层直接访问数据面的请求被拒 | 阶段 4 |
| PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN | VALIDATION | 400 | false | 授权配置引用的权限项码不在权限项目录内 | 阶段 4 |
| PLATFORM.AUTHZ.SCOPE_BINDING_MISSING | VALIDATION | 400 | false | 配置发布或模块启用时，权限项所需对象范围绑定缺失或指向不存在的表/锚列 | 阶段 4；阶段 13 模块启用传播 |
| PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN | VALIDATION | 400 | false | 对掩码或隐藏字段发起排序、聚合 | 阶段 4 |
| PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN | PERMISSION_DENIED | 403 | false | 内部角色经门户端点发起请求被拒 | 阶段 4 |
| PLATFORM.AUTHZ.REAUTH_REQUIRED | PERMISSION_DENIED | 403 | false | 受保护动作缺少有效且未消费的 `X-Reauth-Token` | 阶段 4；阶段 6/9/10 等调用方传播 |
| PLATFORM.SOD.DUTY_CONFLICT | BUSINESS_CONFLICT | 409 | false | 职责互斥校验命中，含五类管理员两两互斥与角色互斥规则 | 阶段 4 |
| PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 审批节点展开用户集与发起人集相交，冲突指出节点号 | 阶段 4 |
| PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED | BUSINESS_CONFLICT | 409 | false | 高危操作复核挑战已被消费或状态不符，条件更新影响行数为零 | 阶段 4 |
| PLATFORM.APPROVAL.CHAIN_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 当前法人和场景没有唯一有效活动审批链，提交失败关闭且零业务写入 | 阶段 4；全部审批调用方传播 |
| PLATFORM.APPROVAL.ACTIVE_CHAIN_AMBIGUOUS | BUSINESS_CONFLICT | 409 | false | 当前法人和场景出现多条有效活动审批链，视为安全配置冲突，不任选其一 | 阶段 4；全部审批调用方传播 |
| PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER | BUSINESS_CONFLICT | 409 | false | 运行期审批节点展开后的审批人集合为空 | 阶段 4 |
| PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED | PERMISSION_DENIED | 403 | false | 移动端发起 `PAYMENT`、`INVOICE_ISSUE`、`LEDGER_POSTING`、`PERIOD_CLOSE`、`DATA_MIGRATION` 五类受限高危操作，发起即拒 | 阶段 4 |
| PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED | BUSINESS_CONFLICT | 409 | false | 账号批量操作中部分条目失败，已成功条目不回滚 | 阶段 4 |
| PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED | BUSINESS_CONFLICT | 409 | false | 首次登录要求先登记 MFA 因子再继续 | 阶段 4 |
| PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS | BUSINESS_CONFLICT | 409 | false | 账号停用前仍有在途审批任务 | 阶段 4 |

### 2.1 阶段 1 独家登记的七条

按裁定 C-24，下列七条一律由阶段 1 登记，阶段 3a 与阶段 4 不得重复登记；其中后五条在阶段 1 只登记不返回：

1. `PLATFORM.IDEMPOTENCY.KEY_REQUIRED`
2. `PLATFORM.CAPACITY.CONCURRENCY_LIMIT`
3. `PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH`
4. `PLATFORM.CONCURRENCY.STALE_VERSION`
5. `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`
6. `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`
7. `PLATFORM.DB.MIGRATION_WINDOW_CLOSED`

阶段 3 与阶段 4 的错误码清单中删去这七个并注明由阶段 1 登记，判据即上表七行存在且未被任何阶段重复登记。

## 3. 文案

`message` 与 `advice` 面向使用者，一律简体中文，禁止出现堆栈、SQL、内部主机名、进程名、表名与密钥字样，该禁令由 CI 断言。规格第 15.1 章要求的关联编号、发生时间、可否重试与处理建议四项由封套的 `incident_no`、`occurred_at`、`retryable`、`advice` 四个字段承载，不写进 `message`。

下列文案已按 F-51 对 U-A-06 的推荐值确认冻结；实现只能逐字复制，不得在各模块另造第二套文案。

| 错误码 | message | advice |
|---|---|---|
| PLATFORM.SYSTEM.NOT_READY | 系统尚未就绪，暂时无法处理该请求。 | 请稍后重试；持续未就绪时联系管理员查看启动自检报告。 |
| PLATFORM.SYSTEM.SYNC_TIMEOUT | 该请求处理时间超过同步等待上限。 | 请改用后台任务方式提交该操作，或缩小单次处理范围后重试。 |
| PLATFORM.SYSTEM.INTERNAL_ERROR | 系统内部错误，本次操作未生效。 | 请记录关联编号后重试；重复出现时联系管理员。 |
| PLATFORM.REQUEST.INVALID_PAYLOAD | 请求内容不符合要求。 | 请按提示修正标出的字段后重新提交。 |
| PLATFORM.REQUEST.HEADER_MISSING | 请求缺少必需的标识信息，或其格式不正确。 | 请更新客户端到受支持的版本后重试。 |
| PLATFORM.ROUTE.NOT_FOUND | 请求的地址不存在。 | 请检查地址是否正确，或确认客户端版本与服务端一致。 |
| PLATFORM.IDEMPOTENCY.KEY_REQUIRED | 该写入请求缺少幂等标识，或标识格式不正确。 | 请由客户端为每次写入生成一个幂等标识后重试。 |
| PLATFORM.CAPACITY.CONCURRENCY_LIMIT | 当前并发请求已达上限，本次请求未被受理。 | 请稍后重试；高峰期持续出现时联系管理员调整并发上限。 |
| PLATFORM.IPC.CONCURRENCY_LIMIT | 当前内部服务连接已达安全上限，本次调用未被受理。 | 请稍后重试；持续出现时请管理员检查对应服务进程。 |
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | 同一幂等标识上提交了不同的内容。 | 请换用新的幂等标识重新提交，或核对首次提交的内容。 |
| PLATFORM.CONCURRENCY.STALE_VERSION | 该记录已被他人修改，本次修改未生效。 | 请重新打开该记录，确认最新内容后再提交。 |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 记录不存在，或您无权访问。 | 如确需访问，请联系管理员申请相应权限。 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | 您无权对该对象执行此操作。 | 如确需执行，请联系管理员申请相应权限。 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | 当前不在允许结构变更的时间窗口内。 | 请在维护窗口内重试，或联系管理员打开迁移窗口。 |
| PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED | 该单据或档案的类型未登记，无法取号。 | 请联系管理员在类型码登记表中登记该类型后重试。 |
| PLATFORM.LICENSE.RESTRICTED | 当前许可证处于受限运行，本次业务操作不可用。 | 请由管理员导入并完成有效签名许可证的受控审批与发布；查询、导出和安全处置仍可使用。 |
| PLATFORM.MODULE.TRANSITION_INVALID | 该模块的当前状态不允许此操作。 | 请刷新模块状态后按其当前状态选择可用操作。 |
| PLATFORM.MODULE.LICENSE_REQUIRED | 当前许可证未覆盖该模块及其依赖，不能安装、启用、升级或版本回退。 | 请导入适用于本部署且覆盖完整模块依赖的有效签名许可证后重试；停用不受此限制。 |
| PLATFORM.MODULE.PACKAGE_INVALID_OR_INCOMPATIBLE | 该可信模块包与当前产品或安装历史不兼容。 | 请保持模块停用，并导入适用于当前产品版本和契约的模块包。 |
| PLATFORM.MODULE.IN_FLIGHT_DRAIN_TIMEOUT | 该模块仍有正在处理的操作，本次状态变更未生效。 | 请等待在途操作完成后重试；持续出现时检查相关服务任务。 |
| PLATFORM.CONFIG_PACKAGE.SPECIAL_ITEM_SHAPE_INVALID | 该许可证或模块包的内容结构不符合要求。 | 请使用发行方提供的原始单项签名包重新导入。 |
| PLATFORM.CONFIG_RELEASE_ORDER.NON_ROLLBACKABLE_ITEM | 许可证或模块动作不能通过通用配置回退撤销。 | 请导入并审批一份新的签名续期、撤销、停用或版本回退动作。 |
| PLATFORM.FLOW.GUARD_EXPRESSION_INVALID | 流程的条件表达式无法求值。 | 请在流程设计器中检查该条件的写法与所引用的字段。 |
| PLATFORM.KEY_DOMAIN.NOT_PROVISIONED | 所需的安全服务尚未就绪。 | 请稍后重试；持续出现时联系管理员检查安全服务配置。 |
| PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE | 所需的安全材料暂时不可用。 | 请稍后重试；持续出现时联系管理员。 |
| PLATFORM.KEY_DOMAIN.ROTATION_IN_PROGRESS | 同一对象上已有一项轮换操作在途。 | 请等待在途操作完成后重试。 |
| PLATFORM.KEY_DOMAIN.DESTROY_PRECHECK_FAILED | 销毁前核验未通过，已阻止后续操作。 | 请按报告补齐缺失的核验项后重新发起。 |
| PLATFORM.KEY_DOMAIN.TRANSITION_INVALID | 当前状态不允许该操作。 | 请刷新对象状态后按其当前状态选择可用操作。 |
| PLATFORM.CRYPTO.DECRYPT_FAILED | 内容解密失败，本次读取未完成。 | 请记录关联编号后联系管理员处置。 |
| PLATFORM.CRYPTO.AAD_MISMATCH | 内容与当前位置不匹配，已拒绝读取。 | 请勿跨行搬运受保护内容；如系数据异常请联系管理员。 |
| PLATFORM.CRYPTO.CIPHERTEXT_FORMAT_INVALID | 受保护内容的格式不正确。 | 请确认内容未被截断或篡改后重试。 |
| PLATFORM.DB.SERIALIZATION_RETRY_EXHAUSTED | 多个操作同时修改同一内容，本次未能完成。 | 请稍后重试；持续出现时联系管理员检查并发负载。 |
| PLATFORM.DB.REFERENCED_ROW_MISSING | 所引用的记录不存在，本次写入未完成。 | 请核对所引用的记录是否已被移除，修正后重新提交。 |
| PLATFORM.DB.WRITE_SCALE_VIOLATION | 提交的数值超出允许的范围。 | 请按字段说明调整数值后重新提交。 |
| PLATFORM.DB.RLS_CONTEXT_MISSING | 未能取得所需的隔离上下文。 | 请稍后重试；持续出现时联系管理员。 |
| PLATFORM.DB.LEGAL_ENTITY_MISMATCH | 提交内容与当前所属主体不一致。 | 请核对内容归属后重新提交。 |
| PLATFORM.DB.POOL_EXHAUSTED | 当前没有可用的数据访问通道。 | 请稍后重试；高峰期持续出现时联系管理员。 |
| PLATFORM.DB.STATEMENT_TIMEOUT | 本次数据操作的执行时间超过上限。 | 请缩小操作范围后重试。 |
| PLATFORM.DB.LOCK_TIMEOUT | 本次操作等待所需锁的时间超过上限。 | 请稍后重试；持续出现时检查是否有长时间未完成的操作。 |
| PLATFORM.DB.MIGRATION_VERSION_MISMATCH | 数据结构版本与当前程序不一致。 | 请联系管理员完成结构升级后重试。 |
| PLATFORM.DB.MIGRATION_WINDOW_CONFLICT | 已有一个变更窗口处于冲突状态。 | 请在在途窗口关闭后重试。 |
| PLATFORM.DB.APPEND_ONLY_VIOLATION | 该内容仅允许追加，不允许修改或删除。 | 请改用新增记录的方式变更内容。 |
| PLATFORM.DB.ROW_VERSION_NOT_BUMPED | 记录版本号未按要求递增。 | 请核对提交内容后重新提交。 |
| PLATFORM.SENSITIVE_FIELD.NOT_REGISTERED | 相关字段尚未登记为受保护字段。 | 请联系管理员完成登记后重试。 |
| PLATFORM.IDEMPOTENCY.IN_PROGRESS | 同一幂等标识上已有一个请求正在处理。 | 请等待在先请求完成后重试；重试时保持幂等标识与内容不变。 |
| PLATFORM.AUTHN.CREDENTIAL_INVALID | 账号或口令不正确。 | 请核对账号与口令后重试；多次失败将临时锁定。 |
| PLATFORM.AUTHN.ACCOUNT_LOCKED | 该账号因连续失败已临时锁定。 | 请等待锁定窗口结束后重试；持续锁定时联系管理员。 |
| PLATFORM.AUTHN.ACCOUNT_INACTIVE | 该账号当前不可登录。 | 请联系管理员确认账号状态。 |
| PLATFORM.AUTHN.MFA_REQUIRED | 需要追加一次验证才能继续。 | 请按提示提交验证码。 |
| PLATFORM.AUTHN.MFA_INVALID | 验证码不正确。 | 请核对验证码后重新提交。 |
| PLATFORM.AUTHN.MFA_CHALLENGE_EXPIRED | 本次验证已超时。 | 请重新发起登录后再次提交验证码。 |
| PLATFORM.AUTHN.MFA_CHALLENGE_CONSUMED | 本次验证已完成，不能重复使用。 | 若未收到登录结果，请重新发起登录。 |
| PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN | 不允许停用最后一个验证方式。 | 请先登记另一种验证方式再停用当前方式。 |
| PLATFORM.AUTHN.DEVICE_NOT_REGISTERED | 当前设备未被允许访问。 | 请使用已登记的设备，或联系管理员登记本设备。 |
| PLATFORM.AUTHN.RATE_LIMITED | 登录尝试过于频繁，已被限流。 | 请稍后重试。 |
| PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED | 您无权访问该主体的内容。 | 如需访问，请联系管理员授予相应主体权限。 |
| PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN | 该操作涉及隔离受控内容，已被拒绝。 | 请确认操作对象后重试；确需执行时联系管理员。 |
| PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN | 该访问方式不被允许。 | 请通过正常入口执行该操作。 |
| PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN | 权限配置引用了不存在的权限项。 | 请联系管理员核对权限配置。 |
| PLATFORM.AUTHZ.SCOPE_BINDING_MISSING | 权限配置缺少有效的数据范围绑定。 | 请先完成对象范围配置，再发布配置或启用模块。 |
| PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN | 该字段不支持排序或汇总。 | 请更换排序或汇总字段后重试。 |
| PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN | 当前入口不允许执行该操作。 | 请使用对应的内部入口。 |
| PLATFORM.AUTHZ.REAUTH_REQUIRED | 该操作需要重新确认身份。 | 请按提示完成重新认证后再次提交。 |
| PLATFORM.SOD.DUTY_CONFLICT | 该操作与职责分离要求冲突。 | 请调整参与人员或角色后重新提交。 |
| PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | 审批人不得与发起人相同。 | 请按提示调整冲突节点的审批人后重新提交。 |
| PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED | 本次复核已使用或已失效。 | 请重新发起复核后再次确认。 |
| PLATFORM.APPROVAL.CHAIN_NOT_FOUND | 当前业务没有可用的审批流程。 | 请联系管理员为当前法人和业务场景启用一条审批链后重试。 |
| PLATFORM.APPROVAL.ACTIVE_CHAIN_AMBIGUOUS | 当前业务存在冲突的审批流程配置。 | 请联系安全管理员完成受控配置修复；修复前不要重复提交。 |
| PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER | 该审批节点当前没有可用审批人。 | 请联系管理员补充审批人后重试。 |
| PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED | 该操作不允许在当前入口发起。 | 请使用桌面端或内部入口发起该操作。 |
| PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED | 批量操作中部分条目未成功。 | 请按结果核对失败条目，修正后单独重试。 |
| PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED | 首次使用前需登记一种验证方式。 | 请按提示完成登记后继续。 |
| PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS | 该账号尚有未完成的审批任务。 | 请先移交或办结在途任务后再试。 |

## 4. 代码侧唯一落点

阶段 1 计划第 6.1 节与裁定 C-24 指定的常量落点是 `crates/foundation/src/error/codes.rs`。

开发时必须建立 `crates/foundation/src/error/codes.rs`，由 `crates/foundation/src/error.rs` 公开重导出；旧式 `EP-CORE-*` 常量不得保留为业务返回码。`cargo xtask errorcodes` 必须机械比较本文件、常量表和 OpenAPI 的 `x-error-codes`：任一重复、缺失、未登记引用或分类不一致均以非零退出。本文是规范真值，代码与生成物不得反向覆盖本文。

## 5. 维护纪律

- 一个错误码只能由一个阶段登记，重复登记即构建失败。
- 已登记的错误码不得改名。语义变化时新增一个码并把旧码标注为废弃，废弃行保留在本文件内。
- 代码里不内联中文错误文案，只引用常量；用户可见文案与错误码一一对应，集中在本文件。
- 后续阶段的新增错误码按模块码分段追加，段的顺序取技术基线第 1.2 节的模块码顺序，PLATFORM 段在最前。

## 6. F-50 财务一致性、发票与更正凭证段

本段 32 条由 F-50 在开发前先行登记；实现任务不得改名或用通用 4xx 代替。VALIDATION 固定为 `400/false`，BUSINESS_CONFLICT 固定为 `409/false`。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 |
|---|---|---:|---:|---|---|
| FINANCE.SETTLEMENT.EFFECT_INVALID | VALIDATION | 400 | false | effect/root/reverses 形态、方向或来源枚举非法 | 阶段 10 |
| FINANCE.SETTLEMENT.ROOT_INVARIANT_VIOLATED | BUSINESS_CONFLICT | 409 | false | 跨条目、跨根、成环、父子上限或根净额上下界不成立 | 阶段 10 |
| FINANCE.SETTLEMENT.EFFECTIVE_OPEN_CHANGED | BUSINESS_CONFLICT | 409 | false | 锁后候选集合或有效未核销容量相对请求快照已变化 | 阶段 10 |
| FINANCE.SETTLEMENT.AMOUNT_EXCEEDS_EFFECTIVE_OPEN | BUSINESS_CONFLICT | 409 | false | 请求中的单笔核销金额超过锁后有效未核销余额 | 阶段 10 |
| FINANCE.SETTLEMENT.RELEASE_ALLOCATION_MISMATCH | BUSINESS_CONFLICT | 409 | false | 红冲释放分段合计不等于锁后计算的释放总额 | 阶段 10 |
| FINANCE.REFUND.SOURCE_ALLOCATION_MISMATCH | VALIDATION | 400 | false | 来源链接金额和不等于退款额，或任一链接不满足逐来源守恒 | 阶段 10 |
| FINANCE.REFUND.SOURCE_CAP_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 任一来源链接超过对应原款项的可追溯可退上限 | 阶段 10 |
| FINANCE.CASH_DOCUMENT.DOWNSTREAM_REFUND_EXISTS | BUSINESS_CONFLICT | 409 | false | 原到款或付款仍有未冲正退款或返款 | 阶段 10 |
| FINANCE.CASH_DOCUMENT.TRACEABILITY_MISMATCH | BUSINESS_CONFLICT | 409 | false | 原款项或退款来源的锁后资金追溯守恒不成立 | 阶段 10 |
| FINANCE.CASH_DOCUMENT.POSTING_SPLIT_MISMATCH | BUSINESS_CONFLICT | 409 | false | 动态往来腿与预收预付腿不守恒或不等于原资金腿 | 阶段 10 |
| FINANCE.RECONCILIATION.BALANCE_MISMATCH | BUSINESS_CONFLICT | 409 | false | 本次写入后的子账总账或当前与最新期间勾稽差额非零 | 阶段 10 |
| INVOICE.INVOICE_LINE.HEAD_AMOUNT_FORBIDDEN | VALIDATION | 400 | false | 写请求、插件或 Excel 提交头税率或头金额字段 | 阶段 10 |
| INVOICE.INVOICE_LINE.TAX_AMOUNT_OUT_OF_TOLERANCE | VALIDATION | 400 | false | 普通行税额与 half-up 期望值差额超过配置容差 | 阶段 10 |
| INVOICE.INVOICE_LINE.AMOUNT_EQUATION_INVALID | VALIDATION | 400 | false | 行价税合计不等或服务端头行汇总断言失败 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.LINE_SOURCE_MISMATCH | VALIDATION | 400 | false | 来源行两组 id 非恰一组、方向不符、跨票或跨法人 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.EFFECT_KIND_INVALID | VALIDATION | 400 | false | quantity/pricing effect 组合、纯税特例或末次尾差分类非法 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.REMAINING_AMOUNT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 来源行累计净额、税额或价税合计超过锁后剩余额 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.REMAINING_QUANTITY_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 来源行累计冲销数量超过锁后剩余数量 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.STATE_CHANGED | BUSINESS_CONFLICT | 409 | false | 原票或来源行状态、版本变化使当前动作不再合法 | 阶段 10 |
| INVOICE.INVOICE_NUMBER.FORMAT_INVALID | VALIDATION | 400 | false | 编号制式、媒介、代码或号码不是封闭合法组合 | 阶段 10 |
| INVOICE.INVOICE_NUMBER.DUPLICATED | BUSINESS_CONFLICT | 409 | false | 同法人中央号码键已被任一蓝票或红票占用 | 阶段 10 |
| INVOICE.INVOICE_NUMBER.OWNER_MISMATCH | BUSINESS_CONFLICT | 409 | false | 号码登记 owner 与业务头方向、类型或 id 不一致 | 阶段 10 |
| INVOICE.IMPORT.TEMPLATE_VERSION_UNSUPPORTED | VALIDATION | 400 | false | 模板不是三个 v2 版本之一或仍含头金额、头税率列 | 阶段 10 |
| INVOICE.IMPORT.GROUP_HEADER_MISMATCH | VALIDATION | 400 | false | 同一 document_key 的重复头字段不一致或行号重复 | 阶段 10 |
| MDM.TRADE_HISTORY.PRICE_SOURCE_NO_LONGER_ELIGIBLE | BUSINESS_CONFLICT | 409 | false | 提交时重算发现历史成交已不可作为价格来源 | 阶段 5/6/7/10 同批接线 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED | BUSINESS_CONFLICT | 409 | false | 同法人、同供应商、同规范化发票标识已有未作废上传记录 | 阶段 7 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED | BUSINESS_CONFLICT | 409 | false | 上传不再为 UPLOADED 或 row_version 已变化 | 阶段 7/10 同批接线 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.CONTENT_MISMATCH | BUSINESS_CONFLICT | 409 | false | 正式进项登记内容与锁后上传头行不一致 | 阶段 10 |
| LEDGER.CASH_REVERSAL.SPLIT_INVALID | BUSINESS_CONFLICT | 409 | false | ledger 收到非受控来源、负金额或两腿不等于原资金腿 | 阶段 9/10 同批接线 |
| LEDGER.CORRECTION_VOUCHER.SOURCE_NOT_POSTED | BUSINESS_CONFLICT | 409 | false | 更正凭证引用的原凭证不存在、无权或未过账 | 阶段 9 |
| LEDGER.CORRECTION_VOUCHER.AMOUNT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 本次加历史累计更正金额超过原凭证对应行金额 | 阶段 9 |
| LEDGER.CORRECTION_VOUCHER.ENTRY_NOT_ALLOWED | VALIDATION | 400 | false | 更正请求改变资金或业务事实、使用自由科目或分录不平 | 阶段 9 |

泄漏规则：VALIDATION 的 `details` 只含本请求字段路径；守恒类错误只返回当前有权单据内的安全金额和 `incident_no`，不返回表名、约束名或其他资金根；号码重复仅在当前主体有原单据读取权限时返回业务链接，否则 `details` 为空；不可见对象仍统一返回 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`。

### 6.1 用户文案

| 错误码 | message | advice |
|---|---|---|
| FINANCE.SETTLEMENT.EFFECT_INVALID | 核销明细的方向或来源不符合规则。 | 请刷新单据并重新选择核销内容。 |
| FINANCE.SETTLEMENT.ROOT_INVARIANT_VIOLATED | 核销关系不满足资金守恒，本次操作未生效。 | 请记录关联编号并联系财务管理员核查。 |
| FINANCE.SETTLEMENT.EFFECTIVE_OPEN_CHANGED | 可核销余额已经发生变化。 | 请刷新最新余额后重新提交。 |
| FINANCE.SETTLEMENT.AMOUNT_EXCEEDS_EFFECTIVE_OPEN | 本次核销金额超过当前可核销余额。 | 请刷新余额并调整核销金额后重新提交。 |
| FINANCE.SETTLEMENT.RELEASE_ALLOCATION_MISMATCH | 本次释放金额无法完整分配。 | 请刷新相关单据；持续出现时记录关联编号并联系管理员。 |
| FINANCE.REFUND.SOURCE_ALLOCATION_MISMATCH | 退款金额与所选原款项分配不一致。 | 请逐项核对来源金额后重新提交。 |
| FINANCE.REFUND.SOURCE_CAP_EXCEEDED | 某笔原款项的可退余额不足。 | 请调整该来源的退款金额或重新选择原款项。 |
| FINANCE.CASH_DOCUMENT.DOWNSTREAM_REFUND_EXISTS | 该款项仍有关联退款，暂不能冲正。 | 请先处理列出的关联退款后再试。 |
| FINANCE.CASH_DOCUMENT.TRACEABILITY_MISMATCH | 该款项的当前去向无法完整核对。 | 请记录关联编号并联系财务管理员核查。 |
| FINANCE.CASH_DOCUMENT.POSTING_SPLIT_MISMATCH | 本次冲正的资金分配不平衡。 | 请记录关联编号并联系财务管理员核查。 |
| FINANCE.RECONCILIATION.BALANCE_MISMATCH | 本次操作未通过账务一致性校验。 | 请记录关联编号并联系财务管理员核查。 |
| INVOICE.INVOICE_LINE.HEAD_AMOUNT_FORBIDDEN | 发票金额和税率必须按行填写。 | 请更新客户端或模板并删除头部金额、税率字段。 |
| INVOICE.INVOICE_LINE.TAX_AMOUNT_OUT_OF_TOLERANCE | 发票行税额超出允许差额。 | 请核对不含税金额、税率和税额后重新提交。 |
| INVOICE.INVOICE_LINE.AMOUNT_EQUATION_INVALID | 发票行的价税金额不一致。 | 请确保价税合计等于不含税金额加税额。 |
| INVOICE.INVOICE_REVERSAL.LINE_SOURCE_MISMATCH | 红字行与所选原发票不匹配。 | 请重新选择原发票行后提交。 |
| INVOICE.INVOICE_REVERSAL.EFFECT_KIND_INVALID | 红字行的数量和价格更正方式不匹配。 | 请按实际业务选择数量减少或金额更正。 |
| INVOICE.INVOICE_REVERSAL.REMAINING_AMOUNT_EXCEEDED | 红字金额超过原行当前可冲金额。 | 请刷新原票并调整本次红字金额。 |
| INVOICE.INVOICE_REVERSAL.REMAINING_QUANTITY_EXCEEDED | 红字数量超过原行当前可冲数量。 | 请刷新原票并调整本次红字数量。 |
| INVOICE.INVOICE_REVERSAL.STATE_CHANGED | 原发票状态已经变化。 | 请刷新原票后按当前状态重新操作。 |
| INVOICE.INVOICE_NUMBER.FORMAT_INVALID | 发票代码或号码格式不正确。 | 请按所选编号制式核对代码和号码。 |
| INVOICE.INVOICE_NUMBER.DUPLICATED | 该发票标识已经登记。 | 请核对发票原件；如有权限可打开已有单据。 |
| INVOICE.INVOICE_NUMBER.OWNER_MISMATCH | 发票号码与业务单据不匹配。 | 请记录关联编号并联系管理员核查。 |
| INVOICE.IMPORT.TEMPLATE_VERSION_UNSUPPORTED | 该导入模板版本不受支持。 | 请下载最新 v2 模板后重新导入。 |
| INVOICE.IMPORT.GROUP_HEADER_MISMATCH | 同一发票分组的头部信息不一致。 | 请统一该分组的头字段并检查行号。 |
| MDM.TRADE_HISTORY.PRICE_SOURCE_NO_LONGER_ELIGIBLE | 所选历史成交已不能作为当前价格来源。 | 请刷新历史成交并重新选择。 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.DUPLICATED | 该供应商发票已经上传。 | 请核对发票标识并打开已有上传记录。 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.STATE_CHANGED | 该发票上传记录已经被处理。 | 请刷新上传状态后再操作。 |
| PORTAL.SUPPLIER_INVOICE_UPLOAD.CONTENT_MISMATCH | 正式登记内容与供应商上传内容不一致。 | 请按锁定的上传内容登记，或先退回供应商重传。 |
| LEDGER.CASH_REVERSAL.SPLIT_INVALID | 资金冲正的分配不符合规则。 | 请记录关联编号并联系财务管理员核查。 |
| LEDGER.CORRECTION_VOUCHER.SOURCE_NOT_POSTED | 原凭证尚未过账，不能更正。 | 请核对原凭证状态后再试。 |
| LEDGER.CORRECTION_VOUCHER.AMOUNT_EXCEEDED | 累计更正金额超过原凭证金额。 | 请刷新原凭证并调整本次更正金额。 |
| LEDGER.CORRECTION_VOUCHER.ENTRY_NOT_ALLOWED | 本次内容不适用总账更正凭证。 | 请按实际错误类型改用发票红冲或资金单据冲正。 |

### 6.2 阶段 7 门户前置登记

下列 1 条是阶段 7 门户通用闸门的既有错误码，不计入上面的 F-50 32 条；因 F-50 门户 OpenAPI 会直接引用，故在开发前同批先行登记。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 |
|---|---|---:|---:|---|---|
| PORTAL.PORTAL_USER.CAPABILITY_NOT_GRANTED | PERMISSION_DENIED | 403 | false | 请求不是门户客户端、主体不是门户账号，或未授予端点所需门户能力 | 阶段 7/10 门户端点 |

| 错误码 | message | advice |
|---|---|---|
| PORTAL.PORTAL_USER.CAPABILITY_NOT_GRANTED | 当前门户账号不能使用该功能。 | 请确认登录入口与账号权限；如确需使用请联系管理员授权。 |

### 6.3 阶段 10 完整契约沿用码

下列 37 条是阶段 10 原有且仍生效的自有码（其中新增付款状态机的显式非法迁移码），与第 6 节 F-50 新登记的阶段 10 自有码 24 条合计为阶段 10 精确 61 条（FINANCE 31 + INVOICE 30）。两份阶段 10 OpenAPI 只引用实际可由公开入口返回的子集；内部回滚守卫使用的 `INVOICE.INVOICE_APPLICATION.RATIO_ROLLBACK_OVERFLOW` 同样保留登记。被 F-50 替代的 15 条旧码不在本表，任何新实现不得返回。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 |
|---|---|---:|---:|---|---|
| FINANCE.CASH_ACCOUNT.ACCOUNT_TYPE_LEDGER_MISMATCH | VALIDATION | 400 | false | 资金账户类型与所选总账科目分类不匹配 | 阶段 10 |
| FINANCE.CASH_ACCOUNT.LEDGER_ACCOUNT_LOCKED | BUSINESS_CONFLICT | 409 | false | 账户已有资金流水后修改法人或总账科目 | 阶段 10 |
| FINANCE.SETTLEMENT.PARTY_REQUIRED | VALIDATION | 400 | false | 核销建议查询未提供方向对应的往来方 | 阶段 10 |
| FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT | VALIDATION | 400 | false | 核销行合计超过本次到款或付款金额 | 阶段 10 |
| FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE | VALIDATION | 400 | false | 到款金额小于或等于零 | 阶段 10 |
| FINANCE.RECEIPT.CASH_ACCOUNT_DEACTIVATED | BUSINESS_CONFLICT | 409 | false | 到款或付款引用的资金账户已停用 | 阶段 10 |
| FINANCE.RECEIPT.DATE_IN_FUTURE | VALIDATION | 400 | false | 到款日期晚于当前业务日期 | 阶段 10 |
| FINANCE.RECEIPT.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 到款单当前状态不允许取消 | 阶段 10 |
| FINANCE.PAYMENT.EXCEEDS_REQUEST_AMOUNT | BUSINESS_CONFLICT | 409 | false | 累计付款金额超过付款申请可付余额 | 阶段 10 |
| FINANCE.PAYMENT.REQUEST_NOT_APPROVED | BUSINESS_CONFLICT | 409 | false | 付款申请尚未审批通过或已失效 | 阶段 10 |
| FINANCE.PAYMENT.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 付款单当前状态不允许撤回或取消 | 阶段 10 |
| FINANCE.REFUND.RETURN_DOC_REQUIRED | VALIDATION | 400 | false | 退款或返款未提供方向匹配的退货单引用 | 阶段 10 |
| FINANCE.REFUND.INVOICE_REVERSAL_REQUIRED | VALIDATION | 400 | false | 已开票退货退款未提供对应发票冲销引用 | 阶段 10 |
| FINANCE.REFUND.CASH_ACCOUNT_DEACTIVATED | BUSINESS_CONFLICT | 409 | false | 退款或返款引用的资金账户已停用 | 阶段 10 |
| FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_ALREADY_REVERSED | BUSINESS_CONFLICT | 409 | false | 原资金单据已经被有效冲正 | 阶段 10 |
| FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_NOT_REGISTERED | BUSINESS_CONFLICT | 409 | false | 原资金单据不存在、未登记或状态不允许冲正 | 阶段 10 |
| FINANCE.OPENING_BALANCE.PERIOD_NOT_FIRST | BUSINESS_CONFLICT | 409 | false | 期初余额导入目标不是该法人的首个会计期间 | 阶段 10 |
| FINANCE.OPENING_BALANCE.ROW_LIMIT_EXCEEDED | VALIDATION | 400 | false | 单次期初余额导入行数超过上限 | 阶段 10 |
| FINANCE.OPENING_BALANCE.PARTY_NOT_FOUND | VALIDATION | 400 | false | 期初余额行引用的客户或供应商不存在或不可用 | 阶段 10 |
| FINANCE.OVERBILLING_ENTRY.WRITTEN_OFF_REQUIRES_REVERSAL | BUSINESS_CONFLICT | 409 | false | 已核销超量开票挂账在继续匹配前未先冲回核销 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.CONTRACT_NOT_EFFECTIVE | BUSINESS_CONFLICT | 409 | false | 发票申请引用的合同未生效 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.CUMULATIVE_RATIO_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 合同累计申请开票比例超过允许上限 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.EXPECTED_DATE_BEFORE_APPLICATION_DATE | VALIDATION | 400 | false | 预计收款日期早于申请日期 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 发票申请当前状态不允许提交审批 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.APPROVAL_ALREADY_STARTED | BUSINESS_CONFLICT | 409 | false | 发票申请审批已开始后请求撤回 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.ISSUED_INVOICE_EXISTS | BUSINESS_CONFLICT | 409 | false | 发票申请已有已开具发票后请求取消 | 阶段 10 |
| INVOICE.INVOICE_APPLICATION.RATIO_ROLLBACK_OVERFLOW | BUSINESS_CONFLICT | 409 | false | 发票冲销回增后申请剩余比例超过原申请比例及容差 | 阶段 10 |
| INVOICE.SALES_INVOICE.RATIO_EXCEEDS_REMAINING | BUSINESS_CONFLICT | 409 | false | 本次开票比例超过申请单锁后剩余可开比例 | 阶段 10 |
| INVOICE.SALES_INVOICE.ISSUE_DATE_IN_FUTURE | VALIDATION | 400 | false | 销项发票开票日期晚于当前业务日期 | 阶段 10 |
| INVOICE.SALES_INVOICE.POSTING_DATE_IN_FUTURE | VALIDATION | 400 | false | 销项发票记账日期晚于允许日期 | 阶段 10 |
| INVOICE.IMPORT_BATCH.ROW_LIMIT_EXCEEDED | VALIDATION | 400 | false | 发票导入批次行数超过上限 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.SOURCE_INVOICE_NOT_REGISTERED | BUSINESS_CONFLICT | 409 | false | 冲销引用的原发票尚未登记或状态不允许冲销 | 阶段 10 |
| INVOICE.INVOICE_REVERSAL.RECEIPT_PLAN_ISSUED_AMOUNT_NEGATIVE | BUSINESS_CONFLICT | 409 | false | 冲销后收款计划累计已开金额将变为负数 | 阶段 10 |
| INVOICE.PURCHASE_INVOICE.RECEIPT_LINE_ALREADY_INVOICED | BUSINESS_CONFLICT | 409 | false | 采购收货行已被其他有效进项发票占用 | 阶段 10 |
| INVOICE.PURCHASE_INVOICE.QUANTITY_EXCEEDS_RECEIPT | BUSINESS_CONFLICT | 409 | false | 进项发票累计数量超过锁后可开票收货数量 | 阶段 10 |
| INVOICE.PURCHASE_INVOICE.AMOUNT_MISMATCH_WITH_ORDER | VALIDATION | 400 | false | 进项发票行金额与采购订单或允许价差口径不一致 | 阶段 10 |
| INVOICE.PURCHASE_INVOICE.POSTING_DATE_IN_FUTURE | VALIDATION | 400 | false | 进项发票记账日期晚于允许日期 | 阶段 10 |

| 错误码 | message | advice |
|---|---|---|
| FINANCE.CASH_ACCOUNT.ACCOUNT_TYPE_LEDGER_MISMATCH | 资金账户类型与总账科目不匹配。 | 请改选与银行或现金类型一致的科目。 |
| FINANCE.CASH_ACCOUNT.LEDGER_ACCOUNT_LOCKED | 该账户已有资金流水，不能修改所属主体或总账科目。 | 请保留原设置；如需调整请新建账户。 |
| FINANCE.SETTLEMENT.PARTY_REQUIRED | 生成核销建议前必须选择往来方。 | 请按应收或应付方向选择客户或供应商。 |
| FINANCE.SETTLEMENT.TOTAL_EXCEEDS_DOCUMENT_AMOUNT | 核销合计超过本次款项金额。 | 请调低核销行金额后重新提交。 |
| FINANCE.RECEIPT.AMOUNT_NOT_POSITIVE | 到款金额必须大于零。 | 请修正到款金额后重新提交。 |
| FINANCE.RECEIPT.CASH_ACCOUNT_DEACTIVATED | 所选资金账户已停用。 | 请改选启用中的资金账户。 |
| FINANCE.RECEIPT.DATE_IN_FUTURE | 到款日期不能晚于当前业务日期。 | 请修正到款日期后重新提交。 |
| FINANCE.RECEIPT.INVALID_TRANSITION | 当前到款单状态不能取消。 | 请刷新单据状态后选择可用操作。 |
| FINANCE.PAYMENT.EXCEEDS_REQUEST_AMOUNT | 付款金额超过付款申请的可付余额。 | 请刷新付款申请并调整付款金额。 |
| FINANCE.PAYMENT.REQUEST_NOT_APPROVED | 付款申请尚未审批通过或已失效。 | 请完成审批或重新选择有效申请。 |
| FINANCE.PAYMENT.INVALID_TRANSITION | 当前付款单状态不能执行该操作。 | 请刷新付款单后选择当前状态允许的动作。 |
| FINANCE.REFUND.RETURN_DOC_REQUIRED | 退款必须关联对应退货单。 | 请补充与退款方向一致的退货单。 |
| FINANCE.REFUND.INVOICE_REVERSAL_REQUIRED | 已开票退货必须关联发票冲销单。 | 请先完成发票冲销并补充关联。 |
| FINANCE.REFUND.CASH_ACCOUNT_DEACTIVATED | 退款使用的资金账户已停用。 | 请改选启用中的资金账户。 |
| FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_ALREADY_REVERSED | 该资金单据已经冲正。 | 请打开既有冲正记录核对。 |
| FINANCE.CASH_DOCUMENT_REVERSAL.SOURCE_NOT_REGISTERED | 原资金单据不能冲正。 | 请刷新原单据并确认其已登记。 |
| FINANCE.OPENING_BALANCE.PERIOD_NOT_FIRST | 期初余额只能导入首个会计期间。 | 请改选该主体的首个期间。 |
| FINANCE.OPENING_BALANCE.ROW_LIMIT_EXCEEDED | 期初余额导入行数超过上限。 | 请拆分批次后重新导入。 |
| FINANCE.OPENING_BALANCE.PARTY_NOT_FOUND | 期初余额行引用的往来方不可用。 | 请修正客户或供应商后重试该行。 |
| FINANCE.OVERBILLING_ENTRY.WRITTEN_OFF_REQUIRES_REVERSAL | 该挂账已核销，继续匹配前必须先冲回。 | 请先发起核销冲回审批。 |
| INVOICE.INVOICE_APPLICATION.CONTRACT_NOT_EFFECTIVE | 所选合同尚未生效。 | 请改选已生效合同或先完成合同生效。 |
| INVOICE.INVOICE_APPLICATION.CUMULATIVE_RATIO_EXCEEDED | 累计申请开票比例超过允许上限。 | 请刷新合同余额并调低本次比例。 |
| INVOICE.INVOICE_APPLICATION.EXPECTED_DATE_BEFORE_APPLICATION_DATE | 预计收款日期不能早于申请日期。 | 请修正预计收款日期。 |
| INVOICE.INVOICE_APPLICATION.INVALID_TRANSITION | 当前发票申请状态不能提交审批。 | 请刷新申请状态后选择可用操作。 |
| INVOICE.INVOICE_APPLICATION.APPROVAL_ALREADY_STARTED | 审批已经开始，不能撤回申请。 | 请在审批流程中继续处理。 |
| INVOICE.INVOICE_APPLICATION.ISSUED_INVOICE_EXISTS | 该申请已有已开具发票，不能取消。 | 请先按规定处理已开具发票。 |
| INVOICE.INVOICE_APPLICATION.RATIO_ROLLBACK_OVERFLOW | 冲销后的剩余开票比例不一致。 | 请记录关联编号并联系财务管理员核查。 |
| INVOICE.SALES_INVOICE.RATIO_EXCEEDS_REMAINING | 本次开票比例超过剩余可开比例。 | 请刷新申请单并调低开票比例。 |
| INVOICE.SALES_INVOICE.ISSUE_DATE_IN_FUTURE | 开票日期不能晚于当前业务日期。 | 请修正开票日期。 |
| INVOICE.SALES_INVOICE.POSTING_DATE_IN_FUTURE | 记账日期晚于允许日期。 | 请改选允许的会计日期。 |
| INVOICE.IMPORT_BATCH.ROW_LIMIT_EXCEEDED | 发票导入行数超过上限。 | 请拆分文件后重新导入。 |
| INVOICE.INVOICE_REVERSAL.SOURCE_INVOICE_NOT_REGISTERED | 原发票不能执行本次冲销。 | 请刷新原发票并确认其已登记。 |
| INVOICE.INVOICE_REVERSAL.RECEIPT_PLAN_ISSUED_AMOUNT_NEGATIVE | 冲销会使收款计划已开金额小于零。 | 请核对原发票与收款计划关联。 |
| INVOICE.PURCHASE_INVOICE.RECEIPT_LINE_ALREADY_INVOICED | 所选收货行已被其他发票占用。 | 请刷新收货行并选择当前可开票数量。 |
| INVOICE.PURCHASE_INVOICE.QUANTITY_EXCEEDS_RECEIPT | 发票数量超过可开票收货数量。 | 请刷新收货数量并调整发票行。 |
| INVOICE.PURCHASE_INVOICE.AMOUNT_MISMATCH_WITH_ORDER | 发票金额与采购依据不一致。 | 请核对订单价格、允许价差与发票行金额。 |
| INVOICE.PURCHASE_INVOICE.POSTING_DATE_IN_FUTURE | 记账日期晚于允许日期。 | 请改选允许的会计日期。 |

## 7. 阶段 11 成本、经营指标与报表段

本段 36 条是阶段 11 的完整新增集合。表内同时冻结分类、用户文案与处理建议，不允许在实现中使用省略前缀的短名。

| 错误码 | category | HTTP | retryable | 触发条件 | message | advice |
|---|---|---:|---:|---|---|---|
| COSTING.COST_ENTRY.PERIOD_RANGE_REQUIRED | VALIDATION | 400 | false | 成本查询未给期间起止 | 成本查询必须指定起止期间。 | 请补充起止期间后重试。 |
| COSTING.COST_ENTRY.PERIOD_RANGE_INVALID | VALIDATION | 400 | false | 期间顺序非法或跨度超过 36 | 查询期间范围不符合要求。 | 请缩短范围并确认起止顺序。 |
| COSTING.COST_ENTRY.DIMENSION_REQUIRED | VALIDATION | 400 | false | 成本查询未给归集维度 | 成本查询必须指定归集维度。 | 请选择一个支持的维度后重试。 |
| COSTING.COST_ENTRY.DIMENSION_NOT_SUPPORTED | VALIDATION | 400 | false | 维度不在首版五值内 | 当前归集维度不受支持。 | 请改用客户、产品、合同、销售订单或项目维度。 |
| COSTING.COST_ENTRY.RESULT_TOO_LARGE | VALIDATION | 400 | false | 成本明细超过查询上限 | 成本查询结果超过允许范围。 | 请缩短期间或增加筛选条件。 |
| COSTING.COST_ENTRY.DUPLICATE_CAPTURE | BUSINESS_CONFLICT | 409 | false | 同一来源成本已捕获 | 该来源成本已经归集。 | 请刷新来源单据并核对既有归集记录。 |
| COSTING.COST_ENTRY.ACCOUNT_NOT_COST_ACCOUNT | VALIDATION | 400 | false | 总账科目不在成本科目集合 | 该科目不能作为成本归集科目。 | 请核对科目分类后重新过账。 |
| COSTING.COST_ENTRY.SOURCE_DIMENSION_CONFLICT | BUSINESS_CONFLICT | 409 | false | 来源单据与维度引用不一致 | 成本来源与归集维度不一致。 | 请核对来源单据的客户、产品、合同与订单引用。 |
| COSTING.COST_ENTRY.RETURN_MARK_NOT_APPLICABLE | BUSINESS_CONFLICT | 409 | false | 退货标注无唯一可用成本根或已被其他审批占用 | 当前退货不能标记为尚未冲回成本。 | 请刷新成本链并核对退货与采购发票关联。 |
| COSTING.REVENUE_ENTRY.DUPLICATE_CAPTURE | BUSINESS_CONFLICT | 409 | false | 同一来源收入已捕获 | 该来源收入已经归集。 | 请刷新来源单据并核对既有归集记录。 |
| COSTING.REVENUE_ENTRY.ACCOUNT_NOT_REVENUE_ACCOUNT | VALIDATION | 400 | false | 总账科目不在收入科目集合 | 该科目不能作为收入归集科目。 | 请核对科目分类后重新过账。 |
| REPORTING.DATASET.NOT_REGISTERED | VALIDATION | 400 | false | 报表引用未登记数据集 | 所选数据集尚未登记。 | 请改用已登记数据集或联系报表管理员。 |
| REPORTING.DATASET_FIELD.NOT_VISIBLE | PERMISSION_DENIED | 403 | false | 字段超出当前权限或密级 | 您无权使用所选字段。 | 请移除该字段或申请相应权限。 |
| REPORTING.DATASET_FIELD.NOT_AGGREGATABLE | VALIDATION | 400 | false | 对禁止汇总字段执行聚合 | 所选字段不支持汇总。 | 请移除该汇总或改用可汇总字段。 |
| REPORTING.REPORT_OBJECT.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 报表对象状态迁移非法或治理通道不匹配 | 当前报表状态不允许此操作。 | 请刷新报表状态后选择可用操作。 |
| REPORTING.REPORT_OBJECT.SELF_APPROVAL_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 企业报表提交人与审批人相同 | 企业报表不能由提交人自行审批。 | 请由其他管理审批人处理。 |
| REPORTING.REPORT_OBJECT.DEPENDENCY_BROKEN | BUSINESS_CONFLICT | 409 | false | 数据集或字段依赖失效 | 报表依赖已经失效，无法运行。 | 请修复列出的字段依赖并发布新版本。 |
| REPORTING.REPORT_OBJECT.NOT_PUBLISHED | BUSINESS_CONFLICT | 409 | false | 企业对象没有有效发布版本 | 该企业报表尚未发布。 | 请先完成审批和发布。 |
| REPORTING.REPORT_OBJECT.DEACTIVATED | BUSINESS_CONFLICT | 409 | false | 企业对象处于停用态 | 该企业报表已停用。 | 请联系报表管理员重新启用。 |
| REPORTING.REPORT_OBJECT_VERSION.SPEC_INVALID | VALIDATION | 400 | false | 定义不符合对象类型 JSON Schema | 报表定义不符合要求。 | 请按字段提示修正定义。 |
| REPORTING.REPORT_OBJECT_VERSION.EXPRESSION_PARSE_FAILED | VALIDATION | 400 | false | 计算表达式解析失败 | 报表表达式无法解析。 | 请按标出的行列位置修正表达式。 |
| REPORTING.REPORT_OBJECT_VERSION.ADVANCED_SQL_NOT_ALLOWED | VALIDATION | 400 | false | 高级 SQL 已关闭或对象不允许使用 | 当前报表不能使用高级查询。 | 请改用拖拽式取数，或联系管理员确认配置。 |
| REPORTING.REPORT_OBJECT_VERSION.ADVANCED_SQL_PARSE_FAILED | VALIDATION | 400 | false | 高级只读 SQL 解析或白名单校验失败 | 高级查询不符合只读规则。 | 请按提示移除不允许的语句或对象。 |
| REPORTING.REPORT_OBJECT_VERSION.ADVANCED_SQL_LIMIT_EXCEEDED | VALIDATION | 400 | false | SQL 字节、连接或子查询深度超限 | 高级查询复杂度超过上限。 | 请简化查询后重试。 |
| REPORTING.ANALYTIC_QUERY.STATEMENT_TIMEOUT | INFRASTRUCTURE | 503 | true | 分析 SQL 超过 statement_timeout | 分析查询执行超时。 | 请缩短期间或增加筛选条件后重试。 |
| REPORTING.ANALYTIC_QUERY.RESOURCE_LIMIT_EXCEEDED | INFRASTRUCTURE | 503 | true | work_mem 或临时文件上限命中 | 分析查询超过可用资源上限。 | 请简化查询后重试；持续出现时联系管理员。 |
| REPORTING.ANALYTIC_QUERY.RESULT_TOO_LARGE | VALIDATION | 400 | false | 分析查询预计结果超过上限 | 分析查询结果超过允许范围。 | 请增加筛选条件或减少明细字段。 |
| REPORTING.AGING_BUCKET_PROFILE.NOT_FOUND | VALIDATION | 400 | false | 账龄分档不存在或不可用 | 所选账龄分档不存在或不可用。 | 请刷新分档列表后重新选择。 |
| REPORTING.AGING_BUCKET_PROFILE.RANGE_GAP | VALIDATION | 400 | false | 分档范围存在断档 | 账龄分档存在未覆盖区间。 | 请补齐连续区间后保存。 |
| REPORTING.AGING_BUCKET_PROFILE.RANGE_OVERLAP | VALIDATION | 400 | false | 分档范围相互重叠 | 账龄分档区间发生重叠。 | 请调整边界使各区间互不重叠。 |
| REPORTING.RENDER_TASK.ROW_LIMIT_EXCEEDED | VALIDATION | 400 | false | 导出或打印超过 50000 行 | 导出或打印数据超过五万行。 | 请缩小范围后重新创建任务。 |
| REPORTING.RENDER_TASK.FORMAT_NOT_SUPPORTED | VALIDATION | 400 | false | 输出格式不是 XLSX、CSV、PDF | 所选输出格式不受支持。 | 请选择 XLSX、CSV 或 PDF。 |
| REPORTING.RENDER_TASK.CLIENT_NOT_SUPPORTED | VALIDATION | 400 | false | 移动端请求打印渲染 | 当前客户端不支持打印渲染。 | 请改用 Windows 或 macOS 桌面端。 |
| REPORTING.RENDER_TASK.ARTIFACT_EXPIRED | BUSINESS_CONFLICT | 409 | false | 渲染产物已超过保留期 | 该导出或打印文件已经过期。 | 请重新创建渲染任务。 |
| REPORTING.OPERATING_METRIC.DIMENSION_NOT_SUPPORTED | VALIDATION | 400 | false | 指标不支持所选下钻维度 | 当前指标不支持所选下钻维度。 | 请改用该指标支持的维度。 |
| REPORTING.OPERATING_METRIC.PERIOD_SCOPE_MISMATCH | VALIDATION | 400 | false | 指标与请求期间口径不一致 | 指标与所选期间范围不匹配。 | 请按指标口径调整期间后重试。 |

## 8. 阶段 6 合同、销售与价格权限段

本段 34 条是阶段 6 的完整新增集合；平台通用码仍只引用第 2 节，不在本段重复登记。

| 错误码 | category | HTTP | retryable | 触发条件 | message | advice |
|---|---|---:|---:|---|---|---|
| CLM.CONTRACT.AMEND_ON_NON_EFFECTIVE | BUSINESS_CONFLICT | 409 | false | 非生效合同发起变更 | 当前合同状态不能发起变更。 | 请刷新合同并确认其已生效。 |
| CLM.CONTRACT.CUSTOMER_INACTIVE | BUSINESS_CONFLICT | 409 | false | 合同客户不可引用 | 合同客户当前不可用。 | 请重新选择已生效且启用的客户。 |
| CLM.CONTRACT.DERIVATION_IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 派生批次尚未结束时再次变更 | 合同派生任务仍在处理中。 | 请等待派生完成后重试。 |
| CLM.CONTRACT.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 合同状态迁移非法 | 当前合同状态不允许此操作。 | 请刷新合同状态后选择可用操作。 |
| CLM.CONTRACT.LINE_DELIVERY_DATE_OUT_OF_RANGE | VALIDATION | 400 | false | 合同行交付日在有效期外 | 合同行交付日期不在合同有效期内。 | 请调整交付日期或合同有效期。 |
| CLM.CONTRACT.MERGE_CUSTOMER_MISMATCH | VALIDATION | 400 | false | 待合并合同客户不同 | 不同客户的合同不能合并。 | 请选择同一客户的合同。 |
| CLM.CONTRACT.MERGE_SOURCE_NOT_ELIGIBLE | BUSINESS_CONFLICT | 409 | false | 来源合同状态不允许合并 | 所选合同当前不能参与合并。 | 请刷新来源合同并移除不符合条件的合同。 |
| CLM.CONTRACT.PAYMENT_SCHEDULE_SUM_MISMATCH | VALIDATION | 400 | false | 期次比例或金额合计不等于合同总额 | 收付款期次合计与合同金额不一致。 | 请核对各期比例或金额后重试。 |
| CLM.CONTRACT.PRODUCT_INACTIVE | BUSINESS_CONFLICT | 409 | false | 合同行产品不可引用 | 合同行产品当前不可用。 | 请重新选择已生效且可销售的产品。 |
| CLM.CONTRACT.RENEW_SOURCE_NOT_ELIGIBLE | BUSINESS_CONFLICT | 409 | false | 来源合同状态不允许续签 | 当前合同不能作为续签来源。 | 请刷新合同并确认其处于允许续签的状态。 |
| CLM.CONTRACT.SIGNATURE_NOT_COMPLETED | BUSINESS_CONFLICT | 409 | false | 生效前签署或用印证据不完整 | 合同签署尚未完成。 | 请完成签署或用印登记后再生效。 |
| CLM.CONTRACT.THREE_INFO_INCOMPLETE | VALIDATION | 400 | false | 条款、期次或合同正文附件缺失 | 合同关键信息尚未填写完整。 | 请补齐条款、收付款期次和合同正文附件。 |
| CLM.CONTRACT_TEMPLATE.VERSION_NOT_PUBLISHED | BUSINESS_CONFLICT | 409 | false | 所选模板版本未发布 | 所选合同模板版本尚未发布。 | 请改用已发布版本。 |
| CLM.SEAL_USAGE.SCAN_REQUIRED | VALIDATION | 400 | false | 实体用印登记缺扫描件 | 实体用印登记必须附带扫描件。 | 请上传扫描件后重新提交。 |
| CLM.SIGNATURE_REQUEST.EXTERNAL_UNAVAILABLE | EXTERNAL_SYSTEM | 502 | true | 电子签章服务不可用 | 电子签章服务暂时不可用。 | 请稍后重试，或按流程使用人工签署入口。 |
| CLM.SIGNATURE_REQUEST.VERIFY_FAILED | BUSINESS_CONFLICT | 409 | false | 已签文件验签失败 | 已签署文件未通过验证。 | 请核对签署结果并重新获取有效文件。 |
| CPQ.PRICE_AUTHORITY.MULTIPLE_PRICE_LIST_HITS | BUSINESS_CONFLICT | 409 | false | 同级价格权限命中多个价目表 | 当前订单行命中了多个同级价目表。 | 请按提示选择价格来源或修正价目表范围。 |
| CPQ.PRICE_AUTHORITY.NOT_CONFIGURED | BUSINESS_CONFLICT | 409 | false | 用户、岗位、角色三级均无价格权限 | 当前人员尚未配置价格权限。 | 请联系销售管理员配置价格权限。 |
| SALES.DELIVERY_CONFIRMATION.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 交付确认单状态迁移非法 | 当前交付确认单状态不允许此操作。 | 请刷新单据状态后重试。 |
| SALES.DELIVERY_CONFIRMATION.QTY_EXCEEDS_SCHEDULED | BUSINESS_CONFLICT | 409 | false | 确认数量超过分批待交量 | 本次确认数量超过可交付数量。 | 请刷新分批交付行并调整数量。 |
| SALES.DELIVERY_SCHEDULE.NOT_SPLITTABLE | BUSINESS_CONFLICT | 409 | false | 非待交状态执行拆分或合并 | 当前分批交付行不能拆分或合并。 | 请刷新分批状态后选择可操作行。 |
| SALES.DELIVERY_SCHEDULE.SPLIT_SUM_MISMATCH | VALIDATION | 400 | false | 拆分数量合计不等于订单行数量 | 分批数量合计与订单行数量不一致。 | 请调整各分批数量后重新提交。 |
| SALES.EXCHANGE_LINK.ALREADY_LINKED | BUSINESS_CONFLICT | 409 | false | 退货行或替换交付行已配对 | 退货或替换交付行已经配对。 | 请刷新换货关联并选择未配对记录。 |
| SALES.EXCHANGE_LINK.SCOPE_MISMATCH | BUSINESS_CONFLICT | 409 | false | 法人、原订单、客户或产品不一致 | 退货与替换交付不属于同一换货范围。 | 请核对原订单、客户和产品。 |
| SALES.SALES_ORDER.CHANGE_IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 同一订单已有变更在审 | 该销售订单已有变更正在处理。 | 请等待当前变更结束后重试。 |
| SALES.SALES_ORDER.CREDIT_LIMIT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 三桶信用占用超过额度 | 本次业务将超过客户可用信用额度。 | 请查看占用明细并按策略调整或提交审批。 |
| SALES.SALES_ORDER.DELIVERED_QTY_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 变更后数量小于已交付数量 | 订单数量不能低于已交付数量。 | 请保留已交付数量并只调整未交付部分。 |
| SALES.SALES_ORDER.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 销售订单状态迁移非法 | 当前销售订单状态不允许此操作。 | 请刷新订单状态后选择可用操作。 |
| SALES.SALES_ORDER.PRICE_CHANGE_NOT_ALLOWED | BUSINESS_CONFLICT | 409 | false | 部分交付后试图修改价格 | 已部分交付的订单不能修改价格。 | 请保留原价格或按允许字段发起变更。 |
| SALES.SALES_ORDER.STOCK_NOT_AVAILABLE | BUSINESS_CONFLICT | 409 | false | 库存或交期校验未通过且动作要求阻断 | 当前库存或交期不能满足订单。 | 请调整仓库、数量或交期后重试。 |
| SALES.SALES_RETURN.DELIVERY_LINK_REQUIRED | VALIDATION | 400 | false | 退货行未关联交付确认行 | 销售退货必须关联原交付记录。 | 请为每条退货明细选择交付确认行。 |
| SALES.SALES_RETURN.INVOICE_NOT_CREDIT_NOTED | BUSINESS_CONFLICT | 409 | false | 已开票部分尚未完成红字冲销 | 退货对应的已开票部分尚未冲销。 | 请先完成红字发票处理。 |
| SALES.SALES_RETURN.QTY_EXCEEDS_DELIVERED | BUSINESS_CONFLICT | 409 | false | 退货数量超过已交付未退数量 | 退货数量超过当前可退数量。 | 请刷新交付与退货数量后调整本次退货。 |
| SALES.SALES_RETURN.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 销售退货状态迁移非法 | 当前销售退货状态不允许此操作。 | 请刷新退货单状态后选择可用操作。 |

## 9. 阶段 12 售后、项目与设备段

本段 36 条是阶段 12 的完整新增集合；序列号不存在、字段缺失与上限类输入错误均返回 400，状态、配对、数量占用与终态冲突均返回 409。

| 错误码 | category | HTTP | retryable | 触发条件 | message | advice |
|---|---|---:|---:|---|---|---|
| PROJECT.PROJECT.OPEN_TASKS_EXIST | BUSINESS_CONFLICT | 409 | false | 项目完成或关闭时仍有未结任务 | 项目仍有未完成任务。 | 请先完成或取消列出的任务。 |
| PROJECT.PROJECT_TASK.ASSIGNEE_REQUIRED | VALIDATION | 400 | false | 启动任务但未指定负责人 | 启动任务前必须指定负责人。 | 请先指派负责人。 |
| PROJECT.PROJECT_TASK.DERIVATION_LIMIT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 单次合同派生项超过配置上限 | 合同派生的项目任务数量超过上限。 | 请核对合同义务配置后重新触发。 |
| PROJECT.PROJECT_TASK.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 项目任务状态迁移非法 | 当前项目任务状态不允许此操作。 | 请刷新任务状态后选择可用操作。 |
| PROJECT.PROJECT_TASK.REQUISITION_ALREADY_LINKED | BUSINESS_CONFLICT | 409 | false | 任务已关联采购需求 | 该任务已经提交采购需求。 | 请打开既有关联记录。 |
| PROJECT.PROJECT_TASK.REQUISITION_PENDING | BUSINESS_CONFLICT | 409 | false | 任务的采购需求正在异步受理且尚未建立最终引用 | 该任务的采购需求正在处理中。 | 请等待处理完成；若状态变为失败，请按页面提示重试或处理死信。 |
| PROJECT.PROJECT_TASK.TERMINAL_READ_ONLY | BUSINESS_CONFLICT | 409 | false | 修改已完成或已取消任务 | 终态项目任务不能修改。 | 如需补充工作，请新建任务。 |
| SERVICE.CUSTOMER_COMPLAINT.ALREADY_ESCALATED | BUSINESS_CONFLICT | 409 | false | 同一投诉已升级为工单 | 该投诉已经生成工单。 | 请打开已有工单继续处理。 |
| SERVICE.CUSTOMER_COMPLAINT.HANDLING_NOTE_REQUIRED | VALIDATION | 400 | false | 关闭投诉时缺处理说明 | 关闭投诉必须填写处理说明。 | 请补充处理说明后重试。 |
| SERVICE.CUSTOMER_COMPLAINT.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 投诉状态迁移非法 | 当前投诉状态不允许此操作。 | 请刷新投诉状态后选择可用操作。 |
| SERVICE.CUSTOMER_COMPLAINT.TERMINAL_READ_ONLY | BUSINESS_CONFLICT | 409 | false | 修改已关闭或已取消投诉 | 终态投诉不能修改。 | 如需继续处理，请新建投诉或工单。 |
| SERVICE.EQUIPMENT_RECORD.BATCH_LIMIT_EXCEEDED | VALIDATION | 400 | false | 批量建档超过 200 台 | 单次设备建档数量超过上限。 | 请拆分批次后重新提交。 |
| SERVICE.EQUIPMENT_RECORD.DELIVERY_DATE_IN_FUTURE | VALIDATION | 400 | false | 交付日期晚于当前自然日 | 设备交付日期不能晚于今天。 | 请核对交付日期。 |
| SERVICE.EQUIPMENT_RECORD.INSTALL_BEFORE_DELIVERY | VALIDATION | 400 | false | 安装日期早于交付日期 | 设备安装日期不能早于交付日期。 | 请核对交付和安装日期。 |
| SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_ALREADY_LINKED | BUSINESS_CONFLICT | 409 | false | 库存序列号已被其他设备引用 | 该序列号已经关联设备。 | 请核对设备档案或选择其他序列号。 |
| SERVICE.EQUIPMENT_RECORD.SERIAL_STATE_NOT_FOUND | VALIDATION | 400 | false | 库存权威序列号不存在 | 未找到该库存序列号。 | 请确认序列号已由库存模块登记。 |
| SERVICE.EQUIPMENT_RECORD.STATUS_UNKNOWN | VALIDATION | 400 | false | 设备状态码不存在或已停用 | 所选设备状态不可用。 | 请刷新状态选项后重新选择。 |
| SERVICE.EQUIPMENT_RECORD.WARRANTY_EDIT_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 无维护权限或不允许修改保修快照 | 当前设备的保修信息不能修改。 | 请确认权限并按维护入口操作。 |
| SERVICE.EQUIPMENT_RECORD.WARRANTY_RANGE_INVALID | VALIDATION | 400 | false | 保修起止或期限字段不一致 | 设备保修范围不符合要求。 | 请核对保修起止日期和期限。 |
| SERVICE.WORK_ORDER.ASSIGNEE_REQUIRED | VALIDATION | 400 | false | 受理或开工时无处理人 | 工单必须指定处理人。 | 请先指派具备技术员角色的人员。 |
| SERVICE.WORK_ORDER.CONCLUSION_REQUIRED | VALIDATION | 400 | false | 完成工单时缺结论 | 完成工单必须填写处理结论。 | 请补充处理结论后重试。 |
| SERVICE.WORK_ORDER.CUSTOMER_MISMATCH | BUSINESS_CONFLICT | 409 | false | 设备、订单行或合同客户与工单不同 | 工单关联对象的客户不一致。 | 请核对设备、订单行和合同。 |
| SERVICE.WORK_ORDER.EQUIPMENT_TERMINAL_STATUS_CONFIRM_REQUIRED | BUSINESS_CONFLICT | 409 | false | 选择终止状态设备但未确认 | 该设备处于终止状态，需要管理人员确认。 | 请由项目经理确认后重新提交。 |
| SERVICE.WORK_ORDER.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 工单状态迁移非法 | 当前工单状态不允许此操作。 | 请刷新工单状态后选择可用操作。 |
| SERVICE.WORK_ORDER.MAX_LINES_EXCEEDED | VALIDATION | 400 | false | 登记行数量超过工单上限 | 工单登记行数量超过上限。 | 请减少登记行或拆分工单。 |
| SERVICE.WORK_ORDER.OPEN_LINES_EXIST | BUSINESS_CONFLICT | 409 | false | 完成或取消时仍有未结登记行 | 工单仍有未结清登记行。 | 请先完成或作废列出的登记行。 |
| SERVICE.WORK_ORDER.SERIAL_STATE_MISMATCH | BUSINESS_CONFLICT | 409 | false | 请求序列号与设备引用不一致 | 工单序列号与设备档案不一致。 | 请核对设备和库存序列号。 |
| SERVICE.WORK_ORDER.TERMINAL_READ_ONLY | BUSINESS_CONFLICT | 409 | false | 修改已完成或已取消工单 | 终态工单不能修改。 | 返修请创建跟进工单。 |
| SERVICE.WORK_ORDER_LINE.ALREADY_LINKED | BUSINESS_CONFLICT | 409 | false | 登记行已关联业务单据 | 该登记行已经完成关联。 | 请刷新登记行并打开既有关联。 |
| SERVICE.WORK_ORDER_LINE.EXCHANGE_PAIR_REQUIRED | VALIDATION | 400 | false | 换货动作只提供退货侧或替换侧之一 | 换货必须同时提供退货行和替换交付行。 | 请补齐两侧记录后重新提交。 |
| SERVICE.WORK_ORDER_LINE.EXCHANGE_SCOPE_MISMATCH | BUSINESS_CONFLICT | 409 | false | 换货两侧客户、原订单或产品不一致 | 所选退货与替换交付不匹配。 | 请核对原订单、客户和产品。 |
| SERVICE.WORK_ORDER_LINE.INVALID_STATE_TRANSITION | BUSINESS_CONFLICT | 409 | false | 登记行状态迁移非法 | 当前登记行状态不允许此操作。 | 请刷新登记行状态后重试。 |
| SERVICE.WORK_ORDER_LINE.PROCESSING_METHOD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 非维修登记行调用完成维修 | 当前登记行不是维修处理。 | 请使用与处理方式匹配的动作。 |
| SERVICE.WORK_ORDER_LINE.QUANTITY_EXCEEDS_RETURNABLE | BUSINESS_CONFLICT | 409 | false | 请求数量超过已交付减已登记数量 | 本次登记数量超过当前可退数量。 | 请刷新交付与登记数量后调整。 |
| SERVICE.WORK_ORDER_LINE.SALES_ORDER_LINE_REQUIRED | VALIDATION | 400 | false | 退货或换货登记缺销售订单行 | 退货或换货必须关联销售订单行。 | 请选择原销售订单行。 |
| SERVICE.WORK_ORDER_LINE.SALES_RETURN_REJECTED | BUSINESS_CONFLICT | 409 | false | sales 权威复核拒绝创建退货 | 销售模块拒绝了本次退货登记。 | 请刷新交付、退货和发票状态后处理。 |

## 10. 阶段 5 主数据与价目表段

本段 50 条把阶段 5 通用资源占位符展开为可直接实现的字面错误码。仓库没有统一社会信用代码和同名确认分支，因此不登记对应两类错误码。

| 错误码 | category | HTTP | retryable | 触发条件 | message | advice |
|---|---|---:|---:|---|---|---|
| MDM.CUSTOMER.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 客户编码法人内重复 | 客户编码已经存在。 | 请改用其他编码或打开已有客户。 |
| MDM.CUSTOMER.USCC_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 客户统一社会信用代码法人内重复 | 该统一社会信用代码已经登记。 | 请核对客户主体并打开已有档案。 |
| MDM.CUSTOMER.NAME_DUPLICATE_UNCONFIRMED | BUSINESS_CONFLICT | 409 | false | 存在启用同名客户且未确认 | 存在同名客户，需要确认后继续。 | 请核对提示的客户并填写确认说明。 |
| MDM.CUSTOMER.REFERENCE_UNAVAILABLE | VALIDATION | 400 | false | 客户请求引用不存在或不可用对象 | 客户档案引用的记录不可用。 | 请按字段提示重新选择。 |
| MDM.CUSTOMER.USCC_CHECKSUM_INVALID | VALIDATION | 400 | false | 客户统一社会信用代码校验失败 | 统一社会信用代码格式或校验位不正确。 | 请核对十八位代码后重试。 |
| MDM.CUSTOMER.VERSION_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 指定客户版本不存在 | 未找到指定的客户版本。 | 请刷新版本列表后重新选择。 |
| MDM.CUSTOMER.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 客户状态迁移非法 | 当前客户状态不允许此操作。 | 请刷新客户状态后选择可用操作。 |
| MDM.CUSTOMER.NOT_REFERENCEABLE | BUSINESS_CONFLICT | 409 | false | 客户未生效或已停用 | 当前客户不能被新单据引用。 | 请选择已生效且启用的客户。 |
| MDM.SUPPLIER.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 供应商编码法人内重复 | 供应商编码已经存在。 | 请改用其他编码或打开已有供应商。 |
| MDM.SUPPLIER.USCC_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 供应商统一社会信用代码法人内重复 | 该供应商统一社会信用代码已经登记。 | 请核对供应商主体并打开已有档案。 |
| MDM.SUPPLIER.NAME_DUPLICATE_UNCONFIRMED | BUSINESS_CONFLICT | 409 | false | 存在启用同名供应商且未确认 | 存在同名供应商，需要确认后继续。 | 请核对提示的供应商并填写确认说明。 |
| MDM.SUPPLIER.REFERENCE_UNAVAILABLE | VALIDATION | 400 | false | 供应商请求引用不存在或不可用对象 | 供应商档案引用的记录不可用。 | 请按字段提示重新选择。 |
| MDM.SUPPLIER.USCC_CHECKSUM_INVALID | VALIDATION | 400 | false | 供应商统一社会信用代码校验失败 | 供应商统一社会信用代码格式或校验位不正确。 | 请核对十八位代码后重试。 |
| MDM.SUPPLIER.VERSION_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 指定供应商版本不存在 | 未找到指定的供应商版本。 | 请刷新版本列表后重新选择。 |
| MDM.SUPPLIER.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 供应商状态迁移非法 | 当前供应商状态不允许此操作。 | 请刷新供应商状态后选择可用操作。 |
| MDM.SUPPLIER.NOT_REFERENCEABLE | BUSINESS_CONFLICT | 409 | false | 供应商未生效或已停用 | 当前供应商不能被新单据引用。 | 请选择已生效且启用的供应商。 |
| MDM.MATERIAL.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 物料编码法人内重复 | 物料编码已经存在。 | 请改用其他编码或打开已有物料。 |
| MDM.MATERIAL.NAME_DUPLICATE_UNCONFIRMED | BUSINESS_CONFLICT | 409 | false | 存在启用同名物料且未确认 | 存在同名物料，需要确认后继续。 | 请核对提示的物料并填写确认说明。 |
| MDM.MATERIAL.REFERENCE_UNAVAILABLE | VALIDATION | 400 | false | 物料请求引用不存在或不可用对象 | 物料档案引用的记录不可用。 | 请按字段提示重新选择。 |
| MDM.MATERIAL.VERSION_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 指定物料版本不存在 | 未找到指定的物料版本。 | 请刷新版本列表后重新选择。 |
| MDM.MATERIAL.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 物料状态迁移非法 | 当前物料状态不允许此操作。 | 请刷新物料状态后选择可用操作。 |
| MDM.MATERIAL.NOT_REFERENCEABLE | BUSINESS_CONFLICT | 409 | false | 物料未生效或已停用 | 当前物料不能被新单据引用。 | 请选择已生效且启用的物料。 |
| MDM.MATERIAL.STOCK_MOVEMENT_EXISTS | BUSINESS_CONFLICT | 409 | false | 已有库存流水时修改冻结字段 | 该物料已有库存流水，冻结字段不能修改。 | 请保留原字段或新建物料。 |
| MDM.PRODUCT.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 产品编码法人内重复 | 产品编码已经存在。 | 请改用其他编码或打开已有产品。 |
| MDM.PRODUCT.NAME_DUPLICATE_UNCONFIRMED | BUSINESS_CONFLICT | 409 | false | 存在启用同名产品且未确认 | 存在同名产品，需要确认后继续。 | 请核对提示的产品并填写确认说明。 |
| MDM.PRODUCT.REFERENCE_UNAVAILABLE | VALIDATION | 400 | false | 产品请求引用不存在或不可用对象 | 产品档案引用的记录不可用。 | 请按字段提示重新选择。 |
| MDM.PRODUCT.VERSION_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 指定产品版本不存在 | 未找到指定的产品版本。 | 请刷新版本列表后重新选择。 |
| MDM.PRODUCT.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 产品状态迁移非法 | 当前产品状态不允许此操作。 | 请刷新产品状态后选择可用操作。 |
| MDM.PRODUCT.NOT_REFERENCEABLE | BUSINESS_CONFLICT | 409 | false | 产品未生效、已停用或不可销售 | 当前产品不能被新单据引用。 | 请选择已生效、启用且可销售的产品。 |
| MDM.PRODUCT.SALES_REFERENCE_EXISTS | BUSINESS_CONFLICT | 409 | false | 已被生效合同或销售订单引用时修改冻结物料 | 产品已被销售业务引用，关联物料不能修改。 | 请保留原关联或新建产品。 |
| MDM.WAREHOUSE.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 仓库编码法人内重复 | 仓库编码已经存在。 | 请改用其他编码或打开已有仓库。 |
| MDM.WAREHOUSE.REFERENCE_UNAVAILABLE | VALIDATION | 400 | false | 仓库请求引用不存在或不可用对象 | 仓库档案引用的记录不可用。 | 请按字段提示重新选择。 |
| MDM.WAREHOUSE.VERSION_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 指定仓库版本不存在 | 未找到指定的仓库版本。 | 请刷新版本列表后重新选择。 |
| MDM.WAREHOUSE.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 仓库状态迁移非法 | 当前仓库状态不允许此操作。 | 请刷新仓库状态后选择可用操作。 |
| MDM.WAREHOUSE.NOT_REFERENCEABLE | BUSINESS_CONFLICT | 409 | false | 仓库未生效或已停用 | 当前仓库不能被新单据引用。 | 请选择已生效且启用的仓库。 |
| MDM.WAREHOUSE.NONZERO_STOCK | BUSINESS_CONFLICT | 409 | false | 停用仓库时仍有库存结存 | 仓库仍有库存，不能停用。 | 请先完成库存清理或移库。 |
| MDM.WAREHOUSE.OPEN_DOCUMENTS_EXIST | BUSINESS_CONFLICT | 409 | false | 停用仓库时仍有未完成来源单据 | 仓库仍被未完成单据使用。 | 请先处理列出的未完成单据。 |
| MDM.WAREHOUSE.DEACTIVATION_CHECK_UNAVAILABLE | INFRASTRUCTURE | 503 | true | 已启用来源模块缺少停用检查器 | 仓库停用检查暂不可用。 | 请联系管理员检查模块接线后重试。 |
| MDM.CHANGE_REQUEST.ALREADY_OPEN | BUSINESS_CONFLICT | 409 | false | 同一对象已有在途变更申请 | 该档案已有变更申请正在处理。 | 请打开既有申请或等待其结束。 |
| MDM.CHANGE_REQUEST.BASE_DRIFTED | BUSINESS_CONFLICT | 409 | false | 审批期间档案基线或子表已变化 | 档案内容在审批期间发生了变化。 | 请刷新档案并重新提交变更。 |
| MDM.CHANGE_REQUEST.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 变更申请状态迁移非法 | 当前变更申请状态不允许此操作。 | 请刷新申请状态后选择可用操作。 |
| MDM.CHANGE_REQUEST.SELF_APPROVAL_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 申请人试图审批自己提交的申请 | 变更申请不能由申请人自行审批。 | 请由其他审批人处理。 |
| MDM.MASTER_RECORD.FROZEN_FIELD_MODIFIED | BUSINESS_CONFLICT | 409 | false | 修改已冻结档案字段 | 该档案字段已冻结，不能修改。 | 请保留原值或按规则新建档案。 |
| CPQ.PRICE_LIST.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | 价目表状态迁移非法 | 当前价目表状态不允许此操作。 | 请刷新价目表状态后选择可用操作。 |
| CPQ.PRICE_LIST.PERIOD_INVALID | VALIDATION | 400 | false | 价目表起止日期非法 | 价目表有效期不符合要求。 | 请核对生效和失效日期。 |
| CPQ.PRICE_LIST.SCOPE_CUSTOMER_REQUIRED | VALIDATION | 400 | false | 客户范围价目表未指定客户 | 客户专属价目表必须指定客户。 | 请选择适用客户。 |
| CPQ.PRICE_LIST_LINE.DUPLICATED | BUSINESS_CONFLICT | 409 | false | 同一价目表中产品、单位与区间重复 | 价目表中存在重复明细。 | 请合并或调整重复明细。 |
| CPQ.PRICE_LIST_LINE.FLOOR_PRICE_ABOVE_UNIT_PRICE | VALIDATION | 400 | false | 底价高于标准单价 | 底价不能高于标准单价。 | 请调整底价或标准单价。 |
| CPQ.PRICE_LIST_LINE.PRODUCT_NOT_SELLABLE | BUSINESS_CONFLICT | 409 | false | 明细产品不可销售 | 所选产品当前不可销售。 | 请改用可销售产品。 |
| CPQ.PRICE_QUOTE.LINE_LIMIT_EXCEEDED | VALIDATION | 400 | false | 批量取价超过 200 行 | 单次取价行数超过上限。 | 请拆分请求后重试。 |

## 11. F-53 阶段 14 运维与历史数据迁移段

本段 33 条由 F-53 在开发前登记。`VALIDATION` 固定为 `400/false`，`BUSINESS_CONFLICT` 固定为 `409/false`，`INFRASTRUCTURE` 固定为 `503/true`；不得用未登记的 413、通用 4xx 或“返回错误但同时成功”的双重语义替代。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 |
|---|---|---:|---:|---|---|
| PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE | BUSINESS_CONFLICT | 409 | false | 对不可抑制的暴露窗口发起抑制或静音 | 阶段 14 |
| PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED | BUSINESS_CONFLICT | 409 | false | 对已关闭窗口再次执行抑制、取消抑制或关闭动作 | 阶段 14 |
| PLATFORM.OFFSITE_SINK.NOT_CONFIGURED | BUSINESS_CONFLICT | 409 | false | 需要服务器之外落点的动作在未配置落点时发起 | 阶段 14 |
| PLATFORM.OFFSITE_SINK.UNWRITABLE | INFRASTRUCTURE | 503 | true | 经连续阈值判定服务器之外落点当前不可写 | 阶段 14 |
| PLATFORM.OFFSITE_SINK.MEDIA_TYPE_OFFLINE | BUSINESS_CONFLICT | 409 | false | 当前介质为离线介质而请求需要在线读写能力 | 阶段 14 |
| PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED | BUSINESS_CONFLICT | 409 | false | 落点访问控制尚无有效核验结论 | 阶段 14 |
| PLATFORM.ARCHIVE_CHANNEL.SLOT_INVALIDATED | BUSINESS_CONFLICT | 409 | false | 归档所用复制槽已失效，需重建恢复基线 | 阶段 14 |
| PLATFORM.ARCHIVE_CHANNEL.SUSPENDED | BUSINESS_CONFLICT | 409 | false | 归档通道处于暂停态，当前动作不允许 | 阶段 14 |
| PLATFORM.BACKUP_SET.CONCURRENT_RUN | BUSINESS_CONFLICT | 409 | false | 已有互斥的全量备份或基线重建在运行 | 阶段 14 |
| PLATFORM.BACKUP_SET.VERIFY_FAILED | BUSINESS_CONFLICT | 409 | false | 备份自动校验任一必需方法失败 | 阶段 14 |
| PLATFORM.BACKUP_SET.SPILL_LIMIT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 本机备份暂存达到冻结上限，任务已中止 | 阶段 14 |
| PLATFORM.BACKUP_ENCRYPTION.KEY_UNAVAILABLE | INFRASTRUCTURE | 503 | true | 部署级备份加密材料当前不可用 | 阶段 14 |
| PLATFORM.KEY_RECOVERY_MATERIAL.SHARD_PICKUP_SLA_MISSING | BUSINESS_CONFLICT | 409 | false | 未约定恢复材料分片取回时限，不能声明恢复目标 | 阶段 14 |
| PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE | BUSINESS_CONFLICT | 409 | false | 无法取得数据库与附件共同可恢复水位 | 阶段 14 |
| PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING | BUSINESS_CONFLICT | 409 | false | 恢复后存在附件元数据但找不到对应正文 | 阶段 14 |
| PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH | BUSINESS_CONFLICT | 409 | false | 恢复后的附件正文摘要与元数据登记不一致 | 阶段 14 |
| PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT | BUSINESS_CONFLICT | 409 | false | 同类演练的同一次序号已登记 | 阶段 14 |
| PLATFORM.CAPACITY.DISK_WATERMARK_EXCEEDED | INFRASTRUCTURE | 503 | true | 磁盘占用达到冻结容量水位，当前重负载动作不可安全继续 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.INVALID_TEMPLATE | VALIDATION | 400 | false | 迁移模板签名、版本、结构、字段映射或清洗规则非法 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.OBJECT_KIND_UNSUPPORTED | VALIDATION | 400 | false | 模板引用的迁移对象不在 25 项封闭集合内 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.SOURCE_CHANGED | BUSINESS_CONFLICT | 409 | false | 来源结构、清单摘要或已处理记录在批准后发生变化 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.WINDOW_CLOSED | BUSINESS_CONFLICT | 409 | false | 当前不在该批次批准的迁移窗口内 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.STATE_CONFLICT | BUSINESS_CONFLICT | 409 | false | 批次当前状态不允许所请求的迁移动作 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE | BUSINESS_CONFLICT | 409 | false | 数据、涉及模块或财务审批未按当前动作要求齐备 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.READ_ONLY_PROOF_FAILED | BUSINESS_CONFLICT | 409 | false | 来源只读能力或冻结证据未通过核验 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.CHUNK_TOO_LARGE | VALIDATION | 400 | false | 单块超过 1000 行、规范化 JSON 请求体超过 524288 字节或完整 HTTP 请求超过 1048576 字节 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.RECORD_TOO_LARGE | VALIDATION | 400 | false | 单条规范化记录自身超过 524288 字节，无法放入合法迁移块 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.RECORD_INVALID | VALIDATION | 400 | false | 来源记录未通过必填、格式、映射或业务前置校验 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.RECONCILIATION_FAILED | BUSINESS_CONFLICT | 409 | false | 迁移对账仍有不可豁免或未批准的差异 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.KNOWN_DIFFERENCE_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 试图为不可豁免类别登记或批准已知差异 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.CUTOVER_NOT_READY | BUSINESS_CONFLICT | 409 | false | 最终对账、冻结证据、批准或切换材料未齐 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.REVERSAL_NOT_PLANNABLE | BUSINESS_CONFLICT | 409 | false | 任一涉及模块无法形成完整整批冲销计划 | 阶段 14 |
| PLATFORM.DATA_MIGRATION.SOURCE_CONNECTION_FAILED | INFRASTRUCTURE | 503 | true | 经批准的只读来源当前无法连接或读取 | 阶段 14 |

### 11.1 用户文案

| 错误码 | message | advice |
|---|---|---|
| PLATFORM.DEGRADATION_WINDOW.NOT_SUPPRESSIBLE | 该告警不允许静音。 | 请先处理告警所指问题，系统将在条件消除后自动关闭。 |
| PLATFORM.DEGRADATION_WINDOW.ALREADY_CLOSED | 该告警记录已经关闭。 | 请刷新页面查看最新状态。 |
| PLATFORM.OFFSITE_SINK.NOT_CONFIGURED | 尚未配置服务器之外的备份落点。 | 请由管理员完成落点配置与核验后重试。 |
| PLATFORM.OFFSITE_SINK.UNWRITABLE | 服务器之外的备份落点当前不可写。 | 请检查落点可用性后稍后重试。 |
| PLATFORM.OFFSITE_SINK.MEDIA_TYPE_OFFLINE | 当前介质不支持这项在线操作。 | 请装载约定介质，或改用支持在线访问的落点。 |
| PLATFORM.OFFSITE_SINK.ACCESS_CONTROL_NOT_ATTESTED | 备份落点的访问控制尚未核验。 | 请由安全管理员完成核验并登记证据。 |
| PLATFORM.ARCHIVE_CHANNEL.SLOT_INVALIDATED | 连续归档链已失效。 | 请按恢复手册重建恢复基线；完成校验前不要声明新的恢复点。 |
| PLATFORM.ARCHIVE_CHANNEL.SUSPENDED | 连续归档当前已暂停。 | 请先恢复备份落点，再按恢复手册完成重建。 |
| PLATFORM.BACKUP_SET.CONCURRENT_RUN | 已有一项互斥的备份任务正在运行。 | 请等待在途任务结束后重试。 |
| PLATFORM.BACKUP_SET.VERIFY_FAILED | 备份校验未通过。 | 请查看关联报告并重新生成、校验备份。 |
| PLATFORM.BACKUP_SET.SPILL_LIMIT_EXCEEDED | 备份暂存空间已达到上限。 | 请释放容量或调整备份安排后重新执行。 |
| PLATFORM.BACKUP_ENCRYPTION.KEY_UNAVAILABLE | 备份保护材料暂时不可用。 | 请稍后重试；持续出现时联系安全管理员。 |
| PLATFORM.KEY_RECOVERY_MATERIAL.SHARD_PICKUP_SLA_MISSING | 尚未约定恢复材料的取回时限。 | 请完成约定与登记后再声明恢复目标。 |
| PLATFORM.RECOVERY.WATERMARK_UNAVAILABLE | 当前无法确定完整可恢复时间点。 | 请修复归档或附件写出链路后重新判定。 |
| PLATFORM.RECOVERY.ATTACHMENT_CONTENT_MISSING | 恢复数据中有附件正文缺失。 | 请停止放行并按恢复报告补齐或重新恢复。 |
| PLATFORM.RECOVERY.ATTACHMENT_CHECKSUM_MISMATCH | 恢复后的附件内容校验未通过。 | 请停止放行并重新读取受保护副本。 |
| PLATFORM.RECOVERY_DRILL.DUPLICATE_ATTEMPT | 该次演练已经登记。 | 请打开已有记录，或使用新的演练序号。 |
| PLATFORM.CAPACITY.DISK_WATERMARK_EXCEEDED | 当前磁盘容量不足以安全继续。 | 请扩容或按已批准的处置流程释放空间后重试。 |
| PLATFORM.DATA_MIGRATION.INVALID_TEMPLATE | 数据迁移模板不符合要求。 | 请按提示修正模板、重新签名并升版本后重试。 |
| PLATFORM.DATA_MIGRATION.OBJECT_KIND_UNSUPPORTED | 模板中包含首版不支持的迁移对象。 | 请移除该对象，或改用已登记的对象类型。 |
| PLATFORM.DATA_MIGRATION.SOURCE_CHANGED | 来源数据或结构已发生变化。 | 请停止切换，重新生成清单并执行完整试运行。 |
| PLATFORM.DATA_MIGRATION.WINDOW_CLOSED | 当前不在批准的数据迁移窗口内。 | 请在新窗口获批后重新发起。 |
| PLATFORM.DATA_MIGRATION.STATE_CONFLICT | 当前批次状态不允许此操作。 | 请刷新批次状态并按可用步骤继续。 |
| PLATFORM.DATA_MIGRATION.APPROVAL_INCOMPLETE | 数据迁移审批尚未齐备。 | 请补齐所列责任人的审批后重试。 |
| PLATFORM.DATA_MIGRATION.READ_ONLY_PROOF_FAILED | 来源只读或冻结证据未通过。 | 请先在来源侧落实只读控制并重新取证。 |
| PLATFORM.DATA_MIGRATION.CHUNK_TOO_LARGE | 本批数据超过单次允许大小。 | 请拆成每批不超过 1000 行、规范化请求体不超过 512 KiB 后再提交。 |
| PLATFORM.DATA_MIGRATION.RECORD_TOO_LARGE | 单条来源记录超过迁移接口允许大小。 | 请把大附件改为批准的文件清单引用，并缩小该记录后重试。 |
| PLATFORM.DATA_MIGRATION.RECORD_INVALID | 部分来源记录不符合迁移规则。 | 请导出错误摘要、修正来源后以新的运行批次重试。 |
| PLATFORM.DATA_MIGRATION.RECONCILIATION_FAILED | 数据迁移对账未通过。 | 请处理不可豁免差异，并完成其他差异的规定审批。 |
| PLATFORM.DATA_MIGRATION.KNOWN_DIFFERENCE_FORBIDDEN | 该差异不允许作为已知差异放行。 | 请修正来源或目标，使该差异归零后重新对账。 |
| PLATFORM.DATA_MIGRATION.CUTOVER_NOT_READY | 当前批次尚未达到切换条件。 | 请补齐最终对账、冻结证据与全部批准后重试。 |
| PLATFORM.DATA_MIGRATION.REVERSAL_NOT_PLANNABLE | 当前无法为整批数据形成完整冲销计划。 | 请先解决所列模块的冲销问题；不要执行部分冲销。 |
| PLATFORM.DATA_MIGRATION.SOURCE_CONNECTION_FAILED | 当前无法读取已批准的数据来源。 | 请检查来源可用性与只读访问后稍后重试。 |


## 12. F-54 全局引用差集收口

本段 **128 条**是对阶段 3、7、8、9、13 现行正文的 referenced-minus-registered 差集收口；加上此前 332 条，至本段为止的首版现行错误码总数为 **460 条**。阶段 9 在传播平台重新认证与自审码后，自有终态为 36 条（32 条本段既有码加 F-50 四码）；阶段 13 只保留 30 条具名新增码，旧“37 条”未具名配额撤销。已由 F-50 替代的 15 个旧码与已撤销的 `LEDGER.POSTING_TRIGGER_EVENT_TYPE.REGISTRY_MISMATCH` 不在本段，不得实现。

触发条件的完整业务谓词以“返回方”所列阶段正文为准；本表冻结其字面码、分类、HTTP、重试性与固定用户文案。CI 必须同时验证阶段正文/OpenAPI 引用差集为零、代码常量差集为零、每码恰有一条用户文案。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 | message | advice |
|---|---|---:|---:|---|---|---|---|
| PLATFORM.IMPACT_ASSESSMENT.REPLAY_NOT_ALLOWED | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.IMPACT_ASSESSMENT.REPLAY_NOT_ALLOWED` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.IMPACT_COMPLETION_PORT.NOT_REGISTERED | BUSINESS_CONFLICT | 409 | false | 启动、模块启用或批次闭合时，`(source_module,source_event_type)` 没有且只有一个真实 `ImpactSourceCompletionPort` | 阶段 3 | 影响处置完成通道未正确安装，系统已停止自动闭合。 | 请联系管理员修复模块安装或注册配置；不要重复提交终止操作。 |
| INVENTORY.SERIAL_STATE.ALREADY_IN_STOCK | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.ALREADY_IN_STOCK` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.SERIAL_STATE.COUNT_MISMATCH | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.COUNT_MISMATCH` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.SERIAL_STATE.DUPLICATE_IN_LINE | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.DUPLICATE_IN_LINE` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.SERIAL_STATE.MATERIAL_MISMATCH | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.MATERIAL_MISMATCH` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.SERIAL_STATE.NOT_IN_STOCK | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.NOT_IN_STOCK` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.SERIAL_STATE.WAREHOUSE_MISMATCH | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.SERIAL_STATE.WAREHOUSE_MISMATCH` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_BALANCE.WAREHOUSE_HAS_STOCK | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_BALANCE.WAREHOUSE_HAS_STOCK` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_MOVEMENT.BATCH_NOT_ALLOWED | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.BATCH_NOT_ALLOWED` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.STOCK_MOVEMENT.BATCH_NOT_FOUND | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.BATCH_NOT_FOUND` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_MOVEMENT.BATCH_REQUIRED | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.BATCH_REQUIRED` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.STOCK_MOVEMENT.DUPLICATE_SOURCE_DOCUMENT | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.DUPLICATE_SOURCE_DOCUMENT` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_MOVEMENT.LINE_LIMIT_EXCEEDED | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.LINE_LIMIT_EXCEEDED` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.STOCK_MOVEMENT.MATERIAL_INACTIVE | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.MATERIAL_INACTIVE` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_MOVEMENT.MIGRATION_NOT_EMPTY | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.MIGRATION_NOT_EMPTY` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_MOVEMENT.PERIOD_REF_MISMATCH | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.PERIOD_REF_MISMATCH` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.STOCK_MOVEMENT.QUANTITY_NOT_POSITIVE | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.QUANTITY_NOT_POSITIVE` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.STOCK_MOVEMENT.WAREHOUSE_INACTIVE | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_MOVEMENT.WAREHOUSE_INACTIVE` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_QTY_BALANCE.INSUFFICIENT_BALANCE | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_QTY_BALANCE.INSUFFICIENT_BALANCE` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_QTY_BALANCE.NEGATIVE_RESULT | BUSINESS_CONFLICT | 409 | false | 阶段 8正文中 `INVENTORY.STOCK_QTY_BALANCE.NEGATIVE_RESULT` 对应的精确守卫命中 | 阶段 8 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| INVENTORY.STOCK_VALUE_BALANCE.ORIGINAL_PRICE_ALLOCATION_MISMATCH | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.STOCK_VALUE_BALANCE.ORIGINAL_PRICE_ALLOCATION_MISMATCH` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| INVENTORY.VARIANCE_SPLIT.MATCHED_QUANTITY_NOT_POSITIVE | VALIDATION | 400 | false | 阶段 8正文中 `INVENTORY.VARIANCE_SPLIT.MATCHED_QUANTITY_NOT_POSITIVE` 对应的精确守卫命中 | 阶段 8 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNT.BOUND_TO_EVENT_ROLE | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.ACCOUNT.BOUND_TO_EVENT_ROLE` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.ACCOUNT.CATEGORY_DIRECTION_MISMATCH | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNT.CATEGORY_DIRECTION_MISMATCH` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNT.CODE_DUPLICATED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.ACCOUNT.CODE_DUPLICATED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.ACCOUNT.HAS_ACTIVE_CHILDREN | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.ACCOUNT.HAS_ACTIVE_CHILDREN` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.ACCOUNT.HAS_POSTED_VOUCHERS | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.ACCOUNT.HAS_POSTED_VOUCHERS` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.ACCOUNT.LEVEL_EXCEEDED | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNT.LEVEL_EXCEEDED` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNT.NOT_POSTABLE | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.ACCOUNT.NOT_POSTABLE` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.ACCOUNT.PARENT_IS_LEVEL_TWO | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNT.PARENT_IS_LEVEL_TWO` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNT.PARENT_NOT_FOUND | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNT.PARENT_NOT_FOUND` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNTING_PERIOD.BEFORE_FIRST_PERIOD | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNTING_PERIOD.BEFORE_FIRST_PERIOD` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.ACCOUNTING_PERIOD.POSTING_DATE_IN_FUTURE | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.ACCOUNTING_PERIOD.POSTING_DATE_IN_FUTURE` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.EVENT_ACCOUNT_BINDING.ACCOUNT_INACTIVE | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.EVENT_ACCOUNT_BINDING.ACCOUNT_INACTIVE` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.EVENT_ACCOUNT_BINDING.ROLE_UNBOUND | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.EVENT_ACCOUNT_BINDING.ROLE_UNBOUND` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.OPENING_BALANCE_BATCH.ACCOUNT_DUPLICATED | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.OPENING_BALANCE_BATCH.ACCOUNT_DUPLICATED` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.OPENING_BALANCE_BATCH.ALREADY_CONFIRMED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.OPENING_BALANCE_BATCH.ALREADY_CONFIRMED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.OPENING_BALANCE_BATCH.UNBALANCED | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.OPENING_BALANCE_BATCH.UNBALANCED` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.OPENING_BALANCE_BATCH.VOUCHER_EXISTS | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.OPENING_BALANCE_BATCH.VOUCHER_EXISTS` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.ANOTHER_REQUEST_IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.ANOTHER_REQUEST_IN_PROGRESS` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.CONCLUSION_ALREADY_PRODUCED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.NOT_EARLIEST_OPEN_PERIOD | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.NOT_EARLIEST_OPEN_PERIOD` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.PENDING_POSTING_BACKLOG` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.PERIOD_ALREADY_CLOSED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.PERIOD_ALREADY_CLOSED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.UNREPAIRED_DEAD_LETTERS` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_DISCREPANCY | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_DISCREPANCY` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_INCOMPLETE | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.PERIOD_CLOSE_REQUEST.VALIDATION_INCOMPLETE` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.POSTING.MEASURE_INVALID | VALIDATION | 400 | false | 阶段 9正文中 `LEDGER.POSTING.MEASURE_INVALID` 对应的精确守卫命中 | 阶段 9 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| LEDGER.VOUCHER.IMMUTABLE | PERMISSION_DENIED | 403 | false | 阶段 9正文中 `LEDGER.VOUCHER.IMMUTABLE` 对应的精确守卫命中 | 阶段 9 | 当前账号不能执行该操作。 | 请改用获授权的入口，或联系管理员核对权限。 |
| LEDGER.VOUCHER.LINE_LIMIT_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.VOUCHER.LINE_LIMIT_EXCEEDED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.VOUCHER.UNBALANCED | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.VOUCHER.UNBALANCED` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.YEAR_END_CLOSING.NOT_FISCAL_YEAR_LAST_PERIOD | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.YEAR_END_CLOSING.NOT_FISCAL_YEAR_LAST_PERIOD` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.YEAR_END_CLOSING.PERIOD_NOT_POSTABLE` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| LEDGER.YEAR_END_CLOSING.ROLE_UNBOUND | BUSINESS_CONFLICT | 409 | false | 阶段 9正文中 `LEDGER.YEAR_END_CLOSING.ROLE_UNBOUND` 对应的精确守卫命中 | 阶段 9 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.AUDIT_EVENT.SEGMENT_LOCK_TIMEOUT | INFRASTRUCTURE | 503 | true | 阶段 3正文中 `PLATFORM.AUDIT_EVENT.SEGMENT_LOCK_TIMEOUT` 对应的精确守卫命中 | 阶段 3 | 相关服务暂时无法完成该操作。 | 请稍后重试；持续出现时记录关联编号并联系管理员。 |
| PLATFORM.AUDIT_VERIFICATION.RANGE_TOO_WIDE | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.AUDIT_VERIFICATION.RANGE_TOO_WIDE` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.BRAND_PROFILE.ASSET_INVALID | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.BRAND_PROFILE.ASSET_INVALID` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.BRAND_PROFILE.CURRENT_UNAVAILABLE | INFRASTRUCTURE | 503 | false | 当前品牌配置为零行或检测到违反单值不变量的多行；服务端必须 fail-closed，不能任选或回退到内置默认 | 阶段 13 | 当前品牌配置尚未安全就绪。 | 请联系平台管理员检查品牌激活状态与数据库完整性。 |
| PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.BRAND_PROFILE.STORE_POLICY_CHECK_FAILED` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT | PERMISSION_DENIED | 403 | false | 阶段 13正文中 `PLATFORM.CLIENT_CAPABILITY.WRITE_NOT_AVAILABLE_ON_CLIENT` 对应的精确守卫命中 | 阶段 13 | 当前账号不能执行该操作。 | 请改用获授权的入口，或联系管理员核对权限。 |
| PLATFORM.CLIENT_RELEASE.FORCED_UPDATE_REQUIRED | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CLIENT_RELEASE.FORCED_UPDATE_REQUIRED` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CONFIG_EDIT_LOCK.HELD_BY_ANOTHER_USER` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_PACKAGE.AUTOTEST_NOT_PASSED | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CONFIG_PACKAGE.AUTOTEST_NOT_PASSED` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CONFIG_PACKAGE.ITEM_HASH_MISMATCH` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CONFIG_PACKAGE.ITEM_LIMIT_EXCEEDED | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.CONFIG_PACKAGE.ITEM_LIMIT_EXCEEDED` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CONFIG_PACKAGE.PLATFORM_VERSION_TOO_LOW | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CONFIG_PACKAGE.PLATFORM_VERSION_TOO_LOW` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CONFIG_PACKAGE.SIGNATURE_INVALID` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CONFIG_PACKAGE.SIGNER_NOT_TRUSTED` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CONFIG_RELEASE_ORDER.APPLIER_NOT_REGISTERED | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.CONFIG_RELEASE_ORDER.APPLIER_NOT_REGISTERED` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CONFIG_RELEASE_ORDER.CONCURRENT_RELEASE_IN_PROGRESS` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_RELEASE_ORDER.DERIVED_STORE_REBUILD_REQUIRED | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CONFIG_RELEASE_ORDER.DERIVED_STORE_REBUILD_REQUIRED` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CONFIG_RELEASE_ORDER.ROLLBACK_TARGET_EXPIRED` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.DOC_TYPE_CODE_CONFLICT` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.CUSTOM_OBJECT.INDEX_KIND_NOT_IN_BASELINE | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.INDEX_KIND_NOT_IN_BASELINE` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CUSTOM_OBJECT.QUOTA_EXCEEDED | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.QUOTA_EXCEEDED` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CUSTOM_OBJECT.RESERVED_NAME | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.RESERVED_NAME` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CUSTOM_OBJECT.SECURITY_LEVEL_REQUIRED | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.SECURITY_LEVEL_REQUIRED` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.CUSTOM_OBJECT.TYPE_NOT_IN_BASELINE | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.CUSTOM_OBJECT.TYPE_NOT_IN_BASELINE` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW | BUSINESS_CONFLICT | 409 | false | 阶段 13正文中 `PLATFORM.DDL_PLAN.REQUIRES_MAINTENANCE_WINDOW` 对应的精确守卫命中 | 阶段 13 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.DEAD_LETTER.DISCARD_APPROVAL_REQUIRED | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.DEAD_LETTER.DISCARD_APPROVAL_REQUIRED` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.DEAD_LETTER.STATE_INVALID | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.DEAD_LETTER.STATE_INVALID` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.DISPOSAL.NOT_DELIVERED | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.DISPOSAL.NOT_DELIVERED` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.EXTENSION.CAPABILITY_DENIED | PERMISSION_DENIED | 403 | false | 阶段 13正文中 `PLATFORM.EXTENSION.CAPABILITY_DENIED` 对应的精确守卫命中 | 阶段 13 | 当前账号不能执行该操作。 | 请改用获授权的入口，或联系管理员核对权限。 |
| PLATFORM.EXTENSION.DISABLED | PERMISSION_DENIED | 403 | false | 阶段 13正文中 `PLATFORM.EXTENSION.DISABLED` 对应的精确守卫命中 | 阶段 13 | 当前账号不能执行该操作。 | 请改用获授权的入口，或联系管理员核对权限。 |
| PLATFORM.EXTENSION.HOST_UNAVAILABLE | INFRASTRUCTURE | 429 | true | 阶段 13正文中 `PLATFORM.EXTENSION.HOST_UNAVAILABLE` 对应的精确守卫命中 | 阶段 13 | 相关服务暂时无法完成该操作。 | 请稍后重试；持续出现时记录关联编号并联系管理员。 |
| PLATFORM.EXTENSION.MANIFEST_MISMATCH | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.EXTENSION.MANIFEST_MISMATCH` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED | INFRASTRUCTURE | 503 | true | 阶段 13正文中 `PLATFORM.EXTENSION.RESOURCE_LIMIT_EXCEEDED` 对应的精确守卫命中 | 阶段 13 | 相关服务暂时无法完成该操作。 | 请稍后重试；持续出现时记录关联编号并联系管理员。 |
| PLATFORM.EXTENSION.SIGNATURE_INVALID | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.EXTENSION.SIGNATURE_INVALID` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.OUTBOX.ENVELOPE_INCOMPLETE | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.OUTBOX.ENVELOPE_INCOMPLETE` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.OUTBOX.EVENT_TYPE_NOT_REGISTERED | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.OUTBOX.EVENT_TYPE_NOT_REGISTERED` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.PROCESS_TASK.NOT_ASSIGNED | PERMISSION_DENIED | 403 | false | 阶段 3正文中 `PLATFORM.PROCESS_TASK.NOT_ASSIGNED` 对应的精确守卫命中 | 阶段 3 | 当前账号不能执行该操作。 | 请改用获授权的入口，或联系管理员核对权限。 |
| PLATFORM.PUSH_REGISTRATION.TOKEN_INVALID | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.PUSH_REGISTRATION.TOKEN_INVALID` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.RULE.AST_LIMIT_EXCEEDED | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.RULE.AST_LIMIT_EXCEEDED` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.RULE.EXPRESSION_PARSE_FAILED | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.RULE.EXPRESSION_PARSE_FAILED` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.SEQUENCE.ALLOCATION_TIMEOUT | INFRASTRUCTURE | 503 | true | 阶段 3正文中 `PLATFORM.SEQUENCE.ALLOCATION_TIMEOUT` 对应的精确守卫命中 | 阶段 3 | 相关服务暂时无法完成该操作。 | 请稍后重试；持续出现时记录关联编号并联系管理员。 |
| PLATFORM.SEQUENCE.CODE_ALREADY_EXISTS | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.SEQUENCE.CODE_ALREADY_EXISTS` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PLATFORM.SEQUENCE.MANUAL_CODE_NOT_ALLOWED | VALIDATION | 400 | false | 阶段 3正文中 `PLATFORM.SEQUENCE.MANUAL_CODE_NOT_ALLOWED` 对应的精确守卫命中 | 阶段 3 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.UI_LAYOUT.CAPABILITY_VALUE_NOT_MODIFIABLE` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL | VALIDATION | 400 | false | 阶段 13正文中 `PLATFORM.UI_LAYOUT.FIELD_HIDING_NOT_ACCESS_CONTROL` 对应的精确守卫命中 | 阶段 13 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PLATFORM.UPLOAD_SESSION.PART_HASH_MISMATCH | BUSINESS_CONFLICT | 409 | false | 阶段 3正文中 `PLATFORM.UPLOAD_SESSION.PART_HASH_MISMATCH` 对应的精确守卫命中 | 阶段 3 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.GOODS_RECEIPT.BATCH_NO_REQUIRED | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.BATCH_NO_REQUIRED` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.GOODS_RECEIPT.ORDER_NOT_RECEIVABLE | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.ORDER_NOT_RECEIVABLE` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.GOODS_RECEIPT.OVER_RECEIPT_APPROVAL_REQUIRED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.OVER_RECEIPT_APPROVAL_REQUIRED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.GOODS_RECEIPT.POSTING_DATE_IN_FUTURE | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.POSTING_DATE_IN_FUTURE` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.GOODS_RECEIPT.SERIAL_COUNT_MISMATCH | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.SERIAL_COUNT_MISMATCH` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.GOODS_RECEIPT.SERIAL_NO_DUPLICATED | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.GOODS_RECEIPT.SERIAL_NO_DUPLICATED` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PAYMENT_REQUEST.AMOUNT_EXCEEDS_OPEN_BALANCE` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PAYMENT_REQUEST.DUPLICATE_INVOICE_RESERVATION | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PAYMENT_REQUEST.DUPLICATE_INVOICE_RESERVATION` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PAYMENT_REQUEST.ILLEGAL_STATUS_TRANSITION | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PAYMENT_REQUEST.ILLEGAL_STATUS_TRANSITION` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PAYMENT_REQUEST.PAYEE_ACCOUNT_MISSING | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PAYMENT_REQUEST.PAYEE_ACCOUNT_MISSING` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PAYMENT_REQUEST.SUPPLIER_TERMINATED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PAYMENT_REQUEST.SUPPLIER_TERMINATED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_ORDER.BATCH_QUANTITY_MISMATCH | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.BATCH_QUANTITY_MISMATCH` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.PURCHASE_ORDER.COST_DIMENSION_REQUIRED | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.COST_DIMENSION_REQUIRED` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.PURCHASE_ORDER.ILLEGAL_STATUS_TRANSITION | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.ILLEGAL_STATUS_TRANSITION` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_ORDER.RECEIVED_LINE_NOT_REVISABLE | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.RECEIVED_LINE_NOT_REVISABLE` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_ORDER.SUPPLIER_NOT_ADMITTED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.SUPPLIER_NOT_ADMITTED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_ORDER.SUPPLIER_QUALIFICATION_EXPIRED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.SUPPLIER_QUALIFICATION_EXPIRED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_ORDER.VOID_NOT_ALLOWED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_ORDER.VOID_NOT_ALLOWED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_REQUISITION.ILLEGAL_STATUS_TRANSITION | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_REQUISITION.ILLEGAL_STATUS_TRANSITION` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_REQUISITION.ORDERED_QUANTITY_EXCEEDED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_REQUISITION.ORDERED_QUANTITY_EXCEEDED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_REQUISITION.SOURCE_LINE_CLOSED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_REQUISITION.SOURCE_LINE_CLOSED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_RETURN.BATCH_OR_SERIAL_NOT_IN_RECEIPT | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.BATCH_OR_SERIAL_NOT_IN_RECEIPT` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |
| PROCURE.PURCHASE_RETURN.ILLEGAL_STATUS_TRANSITION | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.ILLEGAL_STATUS_TRANSITION` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_RETURN.NEGATIVE_STOCK_BLOCKED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.NEGATIVE_STOCK_BLOCKED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_RETURN.QUANTITY_EXCEEDS_RETURNABLE | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.QUANTITY_EXCEEDS_RETURNABLE` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_RETURN.RECEIPT_NOT_POSTED | BUSINESS_CONFLICT | 409 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.RECEIPT_NOT_POSTED` 对应的精确守卫命中 | 阶段 7 | 当前状态或关联数据不允许完成该操作。 | 请刷新数据，处理页面列出的前置事项后重试。 |
| PROCURE.PURCHASE_RETURN.SALES_RETURN_LINK_REQUIRED | VALIDATION | 400 | false | 阶段 7正文中 `PROCURE.PURCHASE_RETURN.SALES_RETURN_LINK_REQUIRED` 对应的精确守卫命中 | 阶段 7 | 提交内容不符合该操作的要求。 | 请按字段提示修正内容后重新提交。 |

## 13. F-55 本地 AI、MCP 与部署 carrier 段

本段恰好 35 条（AI 11、MCP 20、OPS 4），是 F-55 的完整历史新增集合；与前 460 条合并后形成保留的 **495 条 legacy 登记**，不表示这些代码在 F-57 中全部现行或已经实现。`AI`、`MCP` 与 `OPS` 是非业务技术边界命名空间，不增加 `ModuleCode`。触发谓词以 F-55 第 8 节及对应 exact API/ABI 为历史来源；本表同时保留用户文案。不存在“先返回通用错误、以后再补专码”的过渡形态。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 | message | advice |
|---|---|---:|---:|---|---|---|---|
| AI.MODEL_PACKAGE.SIGNATURE_INVALID | BUSINESS_CONFLICT | 409 | false | 模型包签名、manifest、Runtime ABI 或任一文件摘要不符 | 阶段 13c（F-55） | 模型包未通过完整性验证。 | 请停用该包并安装来源可信、签名有效的新版本。 |
| AI.MODEL_PACKAGE.NOT_ACTIVE | INFRASTRUCTURE | 503 | true | 没有唯一 ACTIVE 且已经认证的模型包 | 阶段 13c（F-55） | 本地分析能力尚未就绪。 | 请管理员检查模型包状态和认证报告后重试。 |
| AI.QUERY_PLAN.INVALID | VALIDATION | 400 | false | 模型计划违反单数据集、字段、算子、解码、大小或 limit 闭集 | 阶段 13c（F-55） | 未能形成安全有效的查询计划。 | 请换一种更明确的问法后重试。 |
| AI.INPUT.CONTEXT_LIMIT_EXCEEDED | VALIDATION | 400 | false | prompt、目录、问题与固定最大新 token 超过签名模型/GGUF 较小 context 上限 | 阶段 13c（F-55） | 输入内容超过当前本地模型的安全处理上限。 | 请缩小问题或可见目录范围后重试。 |
| AI.QUERY_PLAN.NOT_VISIBLE_OR_DENIED | PERMISSION_DENIED | 404 | false | 仅 compose：结构合法的模型计划引用本轮权限裁剪目录中不存在或不可见的 dataset/field code；execute 的 token/当前事实变化分别用 TOKEN_INVALID_OR_EXPIRED/SECURITY_CONTEXT_CHANGED | 阶段 13c（F-55） | 查询对象不存在，或您无权访问。 | 请调整查询范围，或联系管理员核对权限。 |
| AI.QUERY_PLAN.CONFIRMATION_REQUIRED | BUSINESS_CONFLICT | 409 | false | execute 未明确提交 `confirmed=true` | 阶段 13c（F-55） | 执行前需要您明确确认查询内容。 | 请查看查询说明并确认后重新提交。 |
| AI.QUERY_PLAN.TOKEN_INVALID_OR_EXPIRED | BUSINESS_CONFLICT | 409 | false | 计划 token 的签名、版本、摘要或五分钟时限无效 | 阶段 13c（F-55） | 查询计划已失效。 | 请重新生成并确认查询计划。 |
| AI.QUERY_PLAN.SECURITY_CONTEXT_CHANGED | BUSINESS_CONFLICT | 409 | false | compose 后权限、密级、范围、目录、模型或提示事实变化 | 阶段 13c（F-55） | 您的访问条件已发生变化，本次查询未执行。 | 请重新生成并确认查询计划。 |
| AI.QUERY_PLAN.IDEMPOTENCY_NOT_ALLOWED | VALIDATION | 400 | false | AI 只读 compose 或 execute 携带 `Idempotency-Key` | 阶段 13c（F-55） | 该只读查询不接受幂等标识。 | 请移除幂等标识后重新提交。 |
| AI.INFERENCE.CONCURRENCY_LIMIT | INFRASTRUCTURE | 429 | true | AI 固定运行或排队上限命中 | 阶段 13c（F-55） | 当前本地分析请求较多，本次未被受理。 | 请稍后重试。 |
| AI.RESOURCE.BASELINE_NOT_CERTIFIED | INFRASTRUCTURE | 503 | true | 生产启用时资源认证证据缺失或与当前制品/硬件不匹配 | 阶段 13c（F-55） | 当前设备尚未通过本地分析资源认证。 | 请管理员完成资源认证或安装已认证的较小模型包。 |
| MCP.PROTOCOL.VERSION_UNSUPPORTED | VALIDATION | 400 | false | 协议版本不是 `2026-07-28` | 阶段 13c（F-55） | 对方使用的协议版本不受支持。 | 请把客户端或连接器升级到本系统指定的协议版本。 |
| MCP.REQUEST.INVALID | VALIDATION | 400 | false | 非法 JSON-RPC、batch/notification/response、非法 id/params 或额外业务字段 | 阶段 13c（F-55） | 请求格式不符合协议约定。 | 请按本系统固定的 JSON-RPC 形状重新发送。 |
| MCP.PAYLOAD.TOO_LARGE | VALIDATION | 413 | false | HTTP/IPC request、manifest、chunk、declared length 或外部 terminal response bytes 超过固定上限 | 阶段 13c（F-55） | 本次协议报文超过允许大小。 | 请缩小单次请求或返回内容后重试。 |
| MCP.PROTOCOL.HEADER_MISMATCH | VALIDATION | 400 | false | 必需 transport header 缺失、额外、非法 sentinel 或与 body 不符 | 阶段 13c（F-55） | 协议头与请求内容不一致。 | 请按固定协议版本和方法头重新发送。 |
| MCP.METHOD.NOT_ALLOWED | PERMISSION_DENIED | 404 | false | 请求方法不在六方法闭集或命中禁用 capability | 阶段 13c（F-55） | 该协议方法未获允许。 | 请改用已批准的方法。 |
| MCP.MANIFEST.INVALID_OR_UNSIGNED | BUSINESS_CONFLICT | 409 | false | manifest 形状、规范摘要、签名、附件或 origin 无效 | 阶段 13c（F-55） | 连接器清单未通过验证。 | 请修正并重新签名一个更高版本的清单。 |
| MCP.MANIFEST.CAPABILITY_DENIED | PERMISSION_DENIED | 403 | false | 方法、工具、资源或字段不在活动 manifest | 阶段 13c（F-55） | 该连接器未获授此项能力。 | 请使用清单中已批准的能力，或提交新版本审批。 |
| MCP.GRANT.INVALID_OR_EXPIRED | PERMISSION_DENIED | 403 | false | grant 摘要、时限、次数、会话、设备、法人或 manifest 任一无效 | 阶段 13c（F-55） | 本次临时授权无效或已过期。 | 请由已登录用户重新签发所需的最小授权。 |
| MCP.DEVICE_PROOF.INVALID | PERMISSION_DENIED | 403 | false | 设备证明签名、公钥、counter、timestamp 无效或重放/乱序 | 阶段 13c（F-55） | 本次设备证明无效。 | 请撤销该临时授权，并从受信设备重新签发。 |
| MCP.TOOL.NOT_VISIBLE_OR_DENIED | PERMISSION_DENIED | 404 | false | 工具不存在或当前用户不可见，统一防枚举 | 阶段 13c（F-55） | 工具不存在，或当前授权不可使用。 | 请刷新可用工具列表后重试。 |
| MCP.RESOURCE.NOT_VISIBLE_OR_DENIED | PERMISSION_DENIED | 404 | false | 资源或资源模板不存在或当前用户不可见，统一防枚举 | 阶段 13c（F-55） | 资源不存在，或当前授权不可使用。 | 请刷新可用资源列表后重试。 |
| MCP.TOOL.HIGH_RISK_FORBIDDEN | PERMISSION_DENIED | 403 | false | 七类高风险（含 `DATA_MIGRATION`）、合同终止或审批结论动作命中绝对禁区 | 阶段 13c（F-55） | 该高风险操作不能通过此协议执行。 | 请改用受支持的业务端并完成规定的人工流程。 |
| MCP.IDEMPOTENCY.REQUIRED | VALIDATION | 400 | false | ExistingCommand 未带 `Idempotency-Key` | 阶段 13c（F-55） | 该写入调用缺少幂等标识。 | 请为本次调用生成唯一幂等标识后重试。 |
| MCP.CREDENTIAL.REF_INVALID | INFRASTRUCTURE | 503 | true | 凭据引用不存在、ACL 不符或读取失败 | 阶段 13c（F-55） | 连接器凭据尚未就绪。 | 请管理员在受控窗口内修复凭据引用并重新探测。 |
| MCP.REMOTE.UNAVAILABLE | EXTERNAL_SYSTEM | 502 | true | 已批准远端 origin 超时、TLS、SPKI 或协议失败 | 阶段 13c（F-55） | 远端连接器暂时不可用。 | 请稍后重试；持续失败时检查已批准的连接配置。 |
| MCP.RESPONSE.SCHEMA_INVALID | EXTERNAL_SYSTEM | 502 | true | 远端或本地 terminal response 不超过 8 MiB，但 JSON、JSON-RPC schema 或允许字段不合法 | 阶段 13c（F-55） | 连接器返回的内容不符合约定。 | 请停用异常版本并由管理员检查其响应契约。 |
| MCP.LOCAL.CONTAINMENT_FAILED | INFRASTRUCTURE | 503 | true | 本地子进程签名、资源、网络、文件或容器收容未成立 | 阶段 13c（F-55） | 本地连接器未能在安全边界内启动。 | 请保持停用并检查签名包与收容报告。 |
| MCP.AUDIT.UNAVAILABLE | INFRASTRUCTURE | 503 | false | completion slot、审计 ATTEMPT/COMPLETION 写入、flush/replay 或结果确认不可用；调用可能尚未执行，也可能已经产生外部副作用 | 阶段 13c（F-55） | 系统未能安全记录或确认本次连接器调用结果。 | 请勿自动重试；请记录关联编号并由管理员核对审计与目标系统状态。 |
| MCP.RATE_LIMITED | INFRASTRUCTURE | 429 | true | connector 的每分钟或在途固定上限命中 | 阶段 13c（F-55） | 当前连接器请求较多，本次未被受理。 | 请稍后重试。 |
| MCP.CALL.TIMEOUT | EXTERNAL_SYSTEM | 504 | true | MCP 调用超过 30 秒绝对时限 | 阶段 13c（F-55） | 连接器调用已超时。 | 请稍后重试；持续出现时检查连接器健康状态。 |
| OPS.DEPLOYMENT.CARRIER_NOT_ALLOWED | BUSINESS_CONFLICT | 409 | false | 除 region/vTPM/backup 专码外，carrier policy/evidence 的 strict shape、签名/链/吊销、ref/digest、双人授权、部署/policy/SKU 绑定、当前事实探针或 legacy/current guard 任一无效，carrier 不在两值闭集，或发现托管组件 | 阶段 14（F-55） | 当前部署形态不在批准范围内。 | 请改用客户自控的单机物理服务器或境内云主机。 |
| OPS.DEPLOYMENT.REGION_NOT_DOMESTIC | BUSINESS_CONFLICT | 409 | false | region jurisdiction 与客户数据驻留法域不一致 | 阶段 14（F-55） | 当前区域不满足数据驻留要求。 | 请迁移到合同批准的境内区域并重新取证。 |
| OPS.DEPLOYMENT.VTPM_EVIDENCE_MISSING | BUSINESS_CONFLICT | 409 | false | IaaS VM 缺少 vTPM 或 attestation 证据 | 阶段 14（F-55） | 云主机的可信启动证据不完整。 | 请补齐可信平台模块及证明后重新认证。 |
| OPS.BACKUP.FAILURE_DOMAIN_NOT_SEPARATE | BUSINESS_CONFLICT | 409 | false | 备份落点与生产故障域、账户凭据域或介质未满足隔离规则 | 阶段 14（F-55） | 备份副本没有与生产环境充分隔离。 | 请改用独立故障域和独立凭据控制的离站副本。 |

## 14. F-57 预登记（尚未实现）

本节恰好预登记 27 条 F-57 代码。它们与前述 495 条 legacy 登记组成状态化总数 **522 条**；`PRE_REGISTERED_NOT_IMPLEMENTED` 表示所属任务可以在实现时采用这些完整代码，但本文存在本身不证明任何返回路径、HTTP API、Windows 门禁、恢复流程或产品功能已完成。实现不得改用缩短形式或另造同义码。

| 错误码 | category | HTTP | retryable | 触发条件 | 返回方 | 用户安全文案 | 运维指导 |
|---|---|---:|---:|---|---|---|---|
| PLATFORM.AUTHORITY.STALE_EPOCH | BUSINESS_CONFLICT | 409 | false | 写事务携带的 `AuthorityEpoch` 已被 fencing 后的新权威代次取代，旧权威不得提交 | F-57 Task 24 authority transaction fence（预登记） | 写入权威已切换，本次操作未生效。 | 核对提升与 fencing 证据，确认只有一个写权威；客户端刷新当前代次后重新发起完整命令。 |
| PLATFORM.AUTHZ.GRANT_REVOKED | PERMISSION_DENIED | 403 | false | 离线意图或延迟命令在服务端重验时，其引用 grant 已撤销或不再有效 | F-57 Task 18 client sync revalidator（预登记） | 本次操作所依据的授权已失效。 | 保留拒绝 receipt，核对撤销审计与当前授权；确需继续时签发新的最小授权并重新提交，不得复用旧意图。 |
| PLATFORM.AUTHZ.SOD_VIOLATION | BUSINESS_CONFLICT | 409 | false | 配置代、能力包或其他高风险动作的作者、批准者或参与主体违反 F-57 exact-set 职责分离规则 | F-57 Task 16 Control Center authorization boundary（预登记） | 本次操作不满足职责分离要求。 | 查看拒绝证据中的冲突主体与角色，改由符合 exact-set 和重新认证要求的独立批准者处理。 |
| PLATFORM.BACKUP.CUT_BEFORE_MIN_RECOVERY_POINT | BUSINESS_CONFLICT | 409 | false | 待签发 recovery cut 的 LSN 早于已验证 base backup 的 `min_recovery_point` | F-57 Task 24 recovery-cut coordinator（预登记） | 无法在当前恢复基线之前建立恢复切点。 | 选择不早于已验证最小恢复点的 WAL 位置，或先生成并验证新的 base backup；不得强行签发该 cut。 |
| PLATFORM.CONTROL.UNEXPECTED_AUTHORITY_FIELD | VALIDATION | 400 | false | Control Center 请求携带只能由服务器从认证会话或当前策略导出的 authority 字段，例如 actor 或 policy version | F-57 Task 16 Control Center HTTP boundary（预登记） | 请求包含不允许由客户端指定的控制字段。 | 检查客户端序列化和代理改写，移除所有服务器权威字段；服务端继续从认证上下文和当前 generation 推导。 |
| PLATFORM.DEPLOYMENT.SIGNATURE_INVALID | BUSINESS_CONFLICT | —（启动前门禁） | false | 部署 manifest 的签名、规范摘要、受信链或已签字段校验失败，必须在任何数据库打开前失败关闭 | F-57 Task 2 deployment-manifest bootstrap verifier（预登记） | 部署清单未通过完整性验证，服务未启动。 | 隔离可疑制品，使用受信发布源恢复已签 manifest 并核对签名证据；验证通过前不得打开数据库或绕过门禁。 |
| PLATFORM.IPC.PEER_UNTRUSTED | PERMISSION_DENIED | —（Windows IPC） | false | named-pipe peer 的 SID、进程签名、本机性或允许身份校验任一失败，在进入 command bus 前拒绝 | F-57 Task 24 Windows IPC admission（预登记） | 内部服务连接未通过身份验证。 | 核对服务虚拟账户、ACL、进程签名和远程管道禁用策略；保留安全事件，禁止降级为匿名或仅路径信任。 |
| PLATFORM.STORAGE.SOFTWARE_VOLUME_DATA_FORBIDDEN | BUSINESS_CONFLICT | —（启动前门禁） | false | 任一客户内容、可关联客户的持久数据或衍生数据解析到软件 SSD，而非已验证 HDD data root | F-57 Task 2 storage-policy validator（预登记） | 数据存储位置不符合当前部署的安全要求，服务未启动。 | 按实际 volume identity 修正所有持久化类别的路由并重新扫描；不得用盘符、junction 或路径字符串例外绕过 HDD 门禁。 |
| SERVICE.WORK_ORDER.CLOSURE_EVIDENCE_MISSING | BUSINESS_CONFLICT | 409 | false | 工单关闭时仍有未履行 obligation，或缺少客户签名等该工单类型要求的 closure evidence | F-57 Task 21 service work-order closure（预登记） | 工单缺少完成所需的履约或验收证据，暂不能关闭。 | 核对未完成责任、附件、工时、用料和客户验收要求，补齐可验证证据后重新执行关闭。 |
| PLATFORM.CLIENT.GENERATION_INCOMPATIBLE | BUSINESS_CONFLICT | 409 | false | 客户端 observed/desired/authoritative generation 或三份契约摘要不满足当前兼容窗口 | F-57 Tasks 16/18/22 Control、Employee、Portal generation guard（预登记） | 客户端版本与当前配置代不兼容。 | 刷新配置代并按服务器指令升级或回滚客户端；不得绕过摘要校验继续提交。 |
| PLATFORM.APPROVAL.REQUIRED | BUSINESS_CONFLICT | 409 | false | 动作风险策略要求独立审批而当前请求尚无可消费的已批准 case | F-57 Tasks 16/23 command boundary（预登记） | 本次操作需要先完成审批。 | 创建或继续对应审批 case；不得把客户端声明当作审批结论。 |
| PLATFORM.APPROVAL.REQUEST_INVALID | VALIDATION | 400 | false | approval case 的 subject/action/request digest/evidence 形状或 policy exact binding 无效 | F-57 Task 23 approval case command（预登记） | 审批请求不符合当前策略。 | 按字段提示修正对象、动作或证据；能力、职责分离和期限由服务器策略推导。 |
| PLATFORM.APPROVAL.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | approval case 决策或状态迁移不在当前状态的允许边内 | F-57 Task 23 approval case command（预登记） | 当前审批状态不允许此操作。 | 刷新审批 case 与 row version 后选择允许的动作。 |
| PLATFORM.LEGAL_HOLD.REQUEST_INVALID | VALIDATION | 400 | false | legal-hold scope、法律依据、保留下限、原因或证据形状无效 | F-57 Task 23 legal-hold command（预登记） | 法律保留请求不符合要求。 | 核对受控 scope、法律依据、保留下限及证据后重新提交。 |
| PLATFORM.LEGAL_HOLD.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | hold/approval/cancel/activate 状态迁移或决策 enum 不在允许闭集 | F-57 Task 23 legal-hold command（预登记） | 当前法律保留状态不允许此操作。 | 刷新 hold 与 row version，按状态图执行允许动作。 |
| PLATFORM.LEGAL_HOLD.RELEASE_DENIED | BUSINESS_CONFLICT | 409 | false | release 未经独立批准、仍受更高保留约束或 scope/evidence 已漂移 | F-57 Task 23 legal-hold release command（预登记） | 当前法律保留不能释放。 | 完成独立释放审批并消除更高保留约束；scope 改变时新建请求。 |
| PLATFORM.DISPOSITION.REQUEST_INVALID | VALIDATION | 400 | false | disposition scope、method、policy ref 或 evidence 形状无效 | F-57 Task 23 disposition command（预登记） | 数据处置请求不符合要求。 | 按受控 scope、方法和策略引用修正请求。 |
| PLATFORM.DISPOSITION.IMPACT_INVALID | BUSINESS_CONFLICT | 409 | false | impact digest、blocking refs 或当前 scope/version 不一致 | F-57 Task 23 disposition impact command（预登记） | 数据处置影响评估与当前范围不一致。 | 重新生成影响评估；范围或策略变化时新建 case。 |
| PLATFORM.DISPOSITION.INVALID_TRANSITION | BUSINESS_CONFLICT | 409 | false | disposition case 的审批、关闭或其他状态迁移不在允许边内 | F-57 Task 23 disposition command（预登记） | 当前数据处置状态不允许此操作。 | 刷新 case 与 row version 后按状态图继续。 |
| PLATFORM.DISPOSITION.EXECUTION_DENIED | PERMISSION_DENIED | 403 | false | 未持有效批准、approved scope digest 不符、受 legal hold 阻断或执行能力不足 | F-57 Task 23 disposition executor（预登记） | 当前数据处置不能执行。 | 核对批准、法律保留、范围摘要和执行权限；禁止缩小错误为不存在。 |
| PLATFORM.DISPOSITION.VERIFICATION_FAILED | BUSINESS_CONFLICT | 409 | false | 处置执行后的删除/crypto-erase/保留事实或恢复不复活证据未通过 | F-57 Task 23 disposition verifier（预登记） | 数据处置结果未通过验证。 | 保持 case 未关闭并进入受控修复；补齐验证与恢复演练证据。 |
| PLATFORM.DISPOSITION.RESUME_DENIED | BUSINESS_CONFLICT | 409 | false | FAILED_CONTAINED case 的 scope/approval 已变、解决证据不足或请求目标不是原 EXECUTING scope | F-57 Task 23 disposition resume command（预登记） | 当前数据处置不能恢复执行。 | 仅在原批准范围和充分 containment 解决证据下恢复；否则新建 case。 |
| PLATFORM.SEARCH.DEFINITION_INVALID | VALIDATION | 400 | false | 搜索 AST、筛选/排序/投影字段或页上限违反安全闭集 | F-57 Task 23 search definition publisher（预登记） | 搜索定义不符合安全规则。 | 使用登记字段与受限 AST；不得提交 SQL、对象名或任意表达式。 |
| PLATFORM.IDENTITY.EXTERNAL_LINK_CONFLICT | BUSINESS_CONFLICT | 409 | false | 外部主体已唯一绑定到不同用户/法人，或候选链接存在歧义 | F-57 Task 23 external identity link command（预登记） | 外部身份已存在冲突绑定。 | 人工核对 provider subject 与本地账户；禁止自动合并或改绑。 |
| PLATFORM.IDENTITY.EXTERNAL_LINK_INVALID | BUSINESS_CONFLICT | 409 | false | 撤销目标、版本、provider/link proof 或当前 link 状态无效 | F-57 Task 23 external identity link command（预登记） | 外部身份链接当前不能修改。 | 刷新链接状态和 row version，核对 provider 证明后重试。 |
| PLATFORM.EXPORT.POLICY_DENIED | PERMISSION_DENIED | 403 | false | 便携导出 scope、格式、脱敏/保留策略或批准不允许当前主体导出 | F-57 Task 23 portable export command（预登记） | 当前策略不允许该导出。 | 缩小范围、应用批准的脱敏策略或完成独立审批。 |
| PLATFORM.IMPORT.PROPOSAL_INVALID | VALIDATION | 400 | false | 已发布附件、模板/version、proposal digest 或导入 mode 不符合 strict contract | F-57 Task 23 import proposal command（预登记） | 导入建议不符合要求。 | 使用已发布且已扫描附件与当前模板重新生成建议；不得直接写入对象。 |
