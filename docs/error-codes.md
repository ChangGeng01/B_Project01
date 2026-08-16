# 错误码表

本文件是全部错误码的唯一登记处。代码侧的对应物是 `ep-foundation` 的错误码常量表，两处由 CI 项 `xtask errorcodes` 逐项比对，重复码或缺失码即构建失败。

新增错误码的顺序是先登记后实现：先在本文件加行，再在常量表加常量，最后才允许有代码返回它。反过来做会让本文件变成一份滞后的注释。

## 1. 命名与分类

错误码为三段点分大写，形如 `<MODULE>.<RESOURCE>.<REASON>`。模块段取技术基线第 1.2 节的 15 个模块码之一或 `PLATFORM`；资源段取表名的单数大写；原因段为动宾短语。

分类与其默认 HTTP 状态、可重试性按技术基线第 5.5 节：

| category | 含义 | HTTP | retryable |
|---|---|---|---|
| VALIDATION | 输入校验错误，定位到字段 | 400 | false |
| BUSINESS_CONFLICT | 业务冲突，含版本冲突、状态机非法迁移、守恒与勾稽不成立 | 409 | false |
| PERMISSION_DENIED | 权限或策略拒绝 | 403，无权访问已存在记录时统一 404 | false |
| EXTERNAL_SYSTEM | 外部系统故障，首版仅电子签章 | 502 | true |
| INFRASTRUCTURE | 基础设施故障，含数据库不可用、磁盘写满、限流 | 503，限流 429 | true |

存在性泄漏的统一处理：对当前安全上下文不可见的记录，读、写、删一律返回 404 与 `PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED`，不区分不存在与无权。只有当前用户对该对象类型完全无权时才返回 403 与 `PLATFORM.AUTHZ.OBJECT_FORBIDDEN`。`PLATFORM.ROUTE.NOT_FOUND` 只用于路由本身不存在，与上面两条不得互换。

## 2. PLATFORM 段

本段共 60 条：阶段 1 登记 13 条，阶段 2 任务 #12（ep-adapter-kms）登记 8 条，阶段 2 任务 #11（ep-adapter-db-pg）登记 3 条，阶段 2 任务 #14（集成 B）登记 10 条，阶段 3a 任务 #18 登记 1 条，阶段 4 任务 #20 与任务 #22 登记 23 条（AUTHN 段 9 条、AUTHZ 段 6 条、SOD 段 2 条、REAUTH 段 1 条、APPROVAL 段 1 条、HIGH_RISK_REQUEST 段 1 条、USER_ACCOUNT 段 3 条），阶段 3b 任务 #21（ep-platform-license）登记 1 条。「返回方」一列写的是第一个真正会返回该码的阶段；标注为阶段 1 之后的，阶段 1 只登记不返回，任何阶段不得因为「还没人返回」而删除该行。

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
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 同一幂等键上的请求体哈希与首次调用不一致 | 阶段 1 |
| PLATFORM.CONCURRENCY.STALE_VERSION | BUSINESS_CONFLICT | 409 | false | 乐观锁版本过期，更新影响行数为零 | 阶段 1 |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | PERMISSION_DENIED | 404 | false | 记录不存在与无权访问已存在记录，同码同形态 | 阶段 4 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | PERMISSION_DENIED | 403 | false | 对象已对当前主体可见但该动作被拒 | 阶段 4 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | BUSINESS_CONFLICT | 409 | false | 未持有迁移窗口即执行在线变更 | 阶段 13b |
| PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED | VALIDATION | 400 | false | 取号时给出的类型码不在本节第 5 章的登记表内 | 阶段 3a |
| PLATFORM.MODULE.TRANSITION_INVALID | BUSINESS_CONFLICT | 409 | false | 模块安装态状态机上的非法迁移，四条合法边见阶段 3 计划第 3.4.11 节 | 阶段 3b |
| PLATFORM.KEY_DOMAIN.NOT_PROVISIONED | INFRASTRUCTURE | 503 | true | 密钥域尚未建立或 KMS 载体不可用 | 阶段 2 |
| PLATFORM.KEY_DOMAIN.KEY_UNAVAILABLE | INFRASTRUCTURE | 503 | true | 所需数据密钥缺失或无法解封 | 阶段 2 |
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
| PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 试图停用最后一个已登记的 MFA 因子 | 阶段 4 |
| PLATFORM.AUTHN.DEVICE_NOT_REGISTERED | PERMISSION_DENIED | 403 | false | 受管客户端的设备标识不在登记册内 | 阶段 4 |
| PLATFORM.AUTHN.RATE_LIMITED | INFRASTRUCTURE | 503 | true | 登录尝试超过速率上限，限流按第 1 节口径取 503 | 阶段 4 |
| PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED | PERMISSION_DENIED | 403 | false | 会话法人不在该账号被授予的法人集合内 | 阶段 4 |
| PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN | PERMISSION_DENIED | 403 | false | 隔离受控操作被拒，含跨法人与隔离管理面动作 | 阶段 4 |
| PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN | PERMISSION_DENIED | 403 | false | 越过应用层直接访问数据面的请求被拒 | 阶段 4 |
| PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN | VALIDATION | 400 | false | 授权配置引用的权限项码不在权限项目录内 | 阶段 4 |
| PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN | VALIDATION | 400 | false | 对掩码或隐藏字段发起排序、聚合 | 阶段 4 |
| PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN | PERMISSION_DENIED | 403 | false | 内部角色经门户端点发起请求被拒 | 阶段 4 |
| PLATFORM.SOD.DUTY_CONFLICT | BUSINESS_CONFLICT | 409 | false | 职责互斥校验命中，含五类管理员两两互斥与角色互斥规则 | 阶段 4 |
| PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | BUSINESS_CONFLICT | 409 | false | 审批节点展开用户集与发起人集相交，冲突指出节点号 | 阶段 4 |
| PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED | BUSINESS_CONFLICT | 409 | false | 高危操作复核挑战已被消费或状态不符，条件更新影响行数为零 | 阶段 4 |
| PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER | BUSINESS_CONFLICT | 409 | false | 运行期审批节点展开后的审批人集合为空 | 阶段 4 |
| PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED | PERMISSION_DENIED | 403 | false | 移动端发起四类受限高危操作，发起即拒 | 阶段 4 |
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

下列文案在文案定稿（未决项 U-A-06）之前为占位取值，定稿时只改本表与常量表两处，错误码本身不变。

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
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | 同一幂等标识上提交了不同的内容。 | 请换用新的幂等标识重新提交，或核对首次提交的内容。 |
| PLATFORM.CONCURRENCY.STALE_VERSION | 该记录已被他人修改，本次修改未生效。 | 请重新打开该记录，确认最新内容后再提交。 |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | 记录不存在，或您无权访问。 | 如确需访问，请联系管理员申请相应权限。 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | 您无权对该对象执行此操作。 | 如确需执行，请联系管理员申请相应权限。 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | 当前不在允许结构变更的时间窗口内。 | 请在维护窗口内重试，或联系管理员打开迁移窗口。 |
| PLATFORM.SEQUENCE.TYPE_CODE_NOT_REGISTERED | 该单据或档案的类型未登记，无法取号。 | 请联系管理员在类型码登记表中登记该类型后重试。 |
| PLATFORM.MODULE.TRANSITION_INVALID | 该模块的当前状态不允许此操作。 | 请刷新模块状态后按其当前状态选择可用操作。 |
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
| PLATFORM.AUTHN.MFA_LAST_FACTOR_FORBIDDEN | 不允许停用最后一个验证方式。 | 请先登记另一种验证方式再停用当前方式。 |
| PLATFORM.AUTHN.DEVICE_NOT_REGISTERED | 当前设备未被允许访问。 | 请使用已登记的设备，或联系管理员登记本设备。 |
| PLATFORM.AUTHN.RATE_LIMITED | 登录尝试过于频繁，已被限流。 | 请稍后重试。 |
| PLATFORM.AUTHZ.LEGAL_ENTITY_NOT_GRANTED | 您无权访问该主体的内容。 | 如需访问，请联系管理员授予相应主体权限。 |
| PLATFORM.AUTHZ.ISOLATION_CONTROL_FORBIDDEN | 该操作涉及隔离受控内容，已被拒绝。 | 请确认操作对象后重试；确需执行时联系管理员。 |
| PLATFORM.AUTHZ.DIRECT_DB_ACCESS_FORBIDDEN | 该访问方式不被允许。 | 请通过正常入口执行该操作。 |
| PLATFORM.AUTHZ.PERMISSION_ITEM_UNKNOWN | 权限配置引用了不存在的权限项。 | 请联系管理员核对权限配置。 |
| PLATFORM.AUTHZ.SORT_FIELD_FORBIDDEN | 该字段不支持排序或汇总。 | 请更换排序或汇总字段后重试。 |
| PLATFORM.AUTHZ.INTERNAL_ROLE_FOR_PORTAL_FORBIDDEN | 当前入口不允许执行该操作。 | 请使用对应的内部入口。 |
| PLATFORM.SOD.DUTY_CONFLICT | 该操作与职责分离要求冲突。 | 请调整参与人员或角色后重新提交。 |
| PLATFORM.SOD.SELF_APPROVAL_FORBIDDEN | 审批人不得与发起人相同。 | 请按提示调整冲突节点的审批人后重新提交。 |
| PLATFORM.REAUTH.TOKEN_ALREADY_CONSUMED | 本次复核已使用或已失效。 | 请重新发起复核后再次确认。 |
| PLATFORM.APPROVAL.NODE_HAS_NO_APPROVER | 该审批节点当前没有可用审批人。 | 请联系管理员补充审批人后重试。 |
| PLATFORM.HIGH_RISK_REQUEST.CLIENT_NOT_ALLOWED | 该操作不允许在当前入口发起。 | 请使用桌面端或内部入口发起该操作。 |
| PLATFORM.USER_ACCOUNT.BATCH_PARTIAL_FAILED | 批量操作中部分条目未成功。 | 请按结果核对失败条目，修正后单独重试。 |
| PLATFORM.USER_ACCOUNT.MFA_ENROLLMENT_REQUIRED | 首次使用前需登记一种验证方式。 | 请按提示完成登记后继续。 |
| PLATFORM.USER_ACCOUNT.PENDING_APPROVAL_TASKS | 该账号尚有未完成的审批任务。 | 请先移交或办结在途任务后再试。 |

## 4. 代码侧落点与当前状态

阶段 1 计划第 6.1 节与裁定 C-24 指定的常量落点是 `crates/foundation/src/error/codes.rs`。

截至本文件写成时，`crates/foundation/` 下的实际形态是单文件 `crates/foundation/src/error.rs`，其中有 `ErrorCode`、`AppError` 两个类型和一个常量 `E_INVALID_ARGUMENT`，取值为 `EP-CORE-0001`。该取值不是三段点分大写形态，也不在上表 13 行之内；它当前的唯一用途是 `crates/foundation/src/security/context.rs` 中受约束字段的构造校验。

因此本文件与代码常量表之间目前存在两处不一致：常量表的落点是单文件而不是 `error/codes.rs`；常量表内容是一个 `EP-CORE-0001` 而不是上表 13 条。`xtask errorcodes` 尚未实现（当前以退出码 70 报「本阶段未交付」），该不一致因此尚未被机器检出。两处以谁为准由裁定方决定，本文件不自行取舍，也不因此删减上表任何一行——上表 13 行的出处是阶段 1 计划第 6.1 节与退出条件 7，不因代码尚未跟上而变化。

## 5. 维护纪律

- 一个错误码只能由一个阶段登记，重复登记即构建失败。
- 已登记的错误码不得改名。语义变化时新增一个码并把旧码标注为废弃，废弃行保留在本文件内。
- 代码里不内联中文错误文案，只引用常量；用户可见文案与错误码一一对应，集中在本文件。
- 后续阶段的新增错误码按模块码分段追加，段的顺序取技术基线第 1.2 节的模块码顺序，PLATFORM 段在最前。
