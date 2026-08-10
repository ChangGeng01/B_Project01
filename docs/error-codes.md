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

本段共 13 条，全部由阶段 1 登记。「返回方」一列写的是第一个真正会返回该码的阶段；标注为阶段 1 之后的，阶段 1 只登记不返回，任何阶段不得因为「还没人返回」而删除该行。

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
| PLATFORM.IDEMPOTENCY.PAYLOAD_MISMATCH | BUSINESS_CONFLICT | 409 | false | 同一幂等键上的请求体哈希与首次调用不一致 | 阶段 3a |
| PLATFORM.CONCURRENCY.STALE_VERSION | BUSINESS_CONFLICT | 409 | false | 乐观锁版本过期，更新影响行数为零 | 阶段 3a |
| PLATFORM.AUTHZ.NOT_FOUND_OR_DENIED | PERMISSION_DENIED | 404 | false | 记录不存在与无权访问已存在记录，同码同形态 | 阶段 4 |
| PLATFORM.AUTHZ.OBJECT_FORBIDDEN | PERMISSION_DENIED | 403 | false | 对象已对当前主体可见但该动作被拒 | 阶段 4 |
| PLATFORM.DB.MIGRATION_WINDOW_CLOSED | BUSINESS_CONFLICT | 409 | false | 未持有迁移窗口即执行在线变更 | 阶段 13b |

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

## 4. 代码侧落点与当前状态

阶段 1 计划第 6.1 节与裁定 C-24 指定的常量落点是 `crates/foundation/src/error/codes.rs`。

截至本文件写成时，`crates/foundation/` 下的实际形态是单文件 `crates/foundation/src/error.rs`，其中有 `ErrorCode`、`AppError` 两个类型和一个常量 `E_INVALID_ARGUMENT`，取值为 `EP-CORE-0001`。该取值不是三段点分大写形态，也不在上表 13 行之内；它当前的唯一用途是 `crates/foundation/src/security/context.rs` 中受约束字段的构造校验。

因此本文件与代码常量表之间目前存在两处不一致：常量表的落点是单文件而不是 `error/codes.rs`；常量表内容是一个 `EP-CORE-0001` 而不是上表 13 条。`xtask errorcodes` 尚未实现（当前以退出码 70 报「本阶段未交付」），该不一致因此尚未被机器检出。两处以谁为准由裁定方决定，本文件不自行取舍，也不因此删减上表任何一行——上表 13 行的出处是阶段 1 计划第 6.1 节与退出条件 7，不因代码尚未跟上而变化。

## 5. 维护纪律

- 一个错误码只能由一个阶段登记，重复登记即构建失败。
- 已登记的错误码不得改名。语义变化时新增一个码并把旧码标注为废弃，废弃行保留在本文件内。
- 代码里不内联中文错误文案，只引用常量；用户可见文案与错误码一一对应，集中在本文件。
- 后续阶段的新增错误码按模块码分段追加，段的顺序取技术基线第 1.2 节的模块码顺序，PLATFORM 段在最前。
