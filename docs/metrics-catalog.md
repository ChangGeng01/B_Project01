# 指标目录

本文件是全部指标名的唯一登记处。指标名的唯一性由 CI 校验，同一指标只能由一个阶段注册，重复登记即构建失败。

## 1. 命名与暴露

命名形如 `ep_<subsystem>_<metric>_<unit>`。计数器以 `_total` 结尾，时长以 `_seconds` 结尾，字节数以 `_bytes` 结尾。

指标由 ops-agent 在 127.0.0.1:9101 以 Prometheus 文本格式暴露，仅内网可达，可对接客户已有的 Prometheus 与 Grafana。ops-agent 的 `/metrics` 聚合本机各进程的指标端点，抓取失败的目标按 `up=0` 标记，不静默丢弃。

## 2. 标签基数纪律

- 禁止把 `user_id`、`doc_no`、`trace_id` 作为标签。这三项的取值集合随业务量无上限增长，一旦进标签即为时序爆炸。
- `legal_entity_id` 允许，理由是首版只有 2 个法人。
- `route` 一律取模板路径而不是实例路径。

本纪律对下表六项逐项成立，新增指标须在登记时逐标签核对。

## 3. 登记表

「注册方」是创建该指标并使其在指标端点上可见的阶段，「填充方」是写入非零样本的阶段。两者可以不同：按裁定 C-23，`ep_db_pool_connections` 与 `ep_db_statement_duration_seconds` 由阶段 1 注册、阶段 2 填充，其判据是指标名存在，而不是有非零样本。

| 指标名 | 类型 | 标签 | 注册方 | 填充方 | 含义 |
|---|---|---|---|---|---|
| `ep_build_info` | gauge | `version`、`git_commit` | 阶段 1 | 阶段 1 | 构建标识，取值恒为 1，信息全在标签上 |
| `ep_selfcheck_pending_items` | gauge | `process` | 阶段 1 | 阶段 1 | 该进程启动自检报告中 Pending 项的条数 |
| `ep_db_pool_connections` | gauge | `pool` | 阶段 1 | 阶段 2 | 各具名连接池的当前连接数 |
| `ep_db_statement_duration_seconds` | histogram | `pool`、`statement_kind` | 阶段 1 | 阶段 2 | 单条 SQL 的执行时长分布 |
| `ep_http_request_duration_seconds` | histogram | `route`、`method`、`status_class`、`client` | 阶段 1 | 阶段 1 | HTTP 请求时长分布，在中间件栈中填充 |
| `ep_quota_throttled_total` | counter | `route` | 阶段 1 | 阶段 1 | 被并发闸门拒绝的请求数，在闸门中填充 |

### 3.1 取值域与桶

- `pool` 取 `rw`、`ro`、`worker`、`integ`、`ops` 五值，与阶段 1 计划第 7.2 节的五个具名池一一对应。
- `client` 取 `win`、`mac`、`ios`、`android`、`portal`、`ops` 六值，即技术基线第 5.6 节 `X-Client` 头的六个取值，与 `crates/foundation/src/security/context.rs` 的 `ClientKind` 六个变体一一对应。
- `status_class` 取 `2xx`、`3xx`、`4xx`、`5xx`。
- `ep_http_request_duration_seconds` 的桶固定为 0.05、0.1、0.25、0.5、1、2、3、5、10、30，与技术基线第 9.2 节逐值一致。改桶等同于改指标，须走一次登记变更。
- `ep_db_statement_duration_seconds` 的桶由阶段 2 在填充时定死并回写本节。

## 4. 阶段 1 的登记范围

阶段 1 登记且只登记上表六项，六项一次性注册在同一处注册表内。上表之外，阶段 1 不登记任何指标名，也不登记任何与上表六项同义的别名——同义名是重复登记的主要来源，本文件不设废弃名一节，作废指标名的追溯记录留在裁定登记文件，不在本文件出现。

规格第 15.3 章要求的降级与暴露窗口台账既进数据库表也各出一个 gauge，两处不得只有其一；该 gauge 由承接降级窗口的阶段登记，不在阶段 1 范围内。

## 5. 机器判定与当前状态

指标名唯一性校验由 `xtask` 实现，判据是本文件登记表内无重名，且登记表与代码侧注册表逐项一致。

当前状态如实记录如下。阶段 1 计划第 13 节新增决定五指定的注册落点是 `crates/platform/obs/src/metrics/registry.rs`；截至本文件写成时，`crates/platform/obs/` 下只有 `src/lib.rs` 一个骨架文件，尚无该注册表，因此上表六项在代码侧尚无对应注册。`xtask` 的唯一性校验子命令当前以退出码 70 明确报「本阶段未交付」，不静默返回 0。上表是登记面，其出处是阶段 1 计划第 13 节新增决定五与退出条件 24，不因代码尚未跟上而变化；代码侧补齐时以上表为准，逐项对齐。

## 6. 维护纪律

- 一个指标只能由一个阶段注册。跨阶段复用同一指标时，追加的是标签取值而不是新指标名。
- 已登记的指标名不得改名。观测口径变化时新增指标并把旧指标标注为停止填充的版本，仪表盘的迁移在同一变更内完成。
- 新增指标的登记项必须同时给出类型、全部标签、取值域与注册填充两方，缺一不受理。
