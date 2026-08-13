# ADR-0012 workspace axum 依赖条目新增 `query` feature

- 状态：已接受
- 出处：阶段 2 任务 #14 实施期偏离登记第十三条

## 背景

workspace 根 `Cargo.toml` 的 axum 条目以 `default-features = false` 加白名单 feature 的形态冻结，既有取值为 `http1`、`json`、`tokio`、`matched-path`。阶段 2 的九个平台端点中，清单类端点（密钥域清单的过滤与分页、敏感字段清单的过滤）带查询串，需要 axum 的 `Query` 提取器，而该提取器在 `query` feature 之后才参与编译。

## 决定

在 workspace axum 条目的 feature 白名单追加 `"query"`，最终形态为 `["http1", "json", "tokio", "matched-path", "query"]`。不启用 axum 的默认 feature 集，不为单端点改用手工解析查询串。

## 理由

选「白名单追加一个 feature」而不是「手工解析查询串」：不采用本决定的代价是在装配层自写一份 URL 解码与键值解析，该实现要自行处理百分号编码、重复键与空值三类边界，而这三类正是 axum `Query` 提取器已被上游测试覆盖的部分；自写版还会绕过 workspace 统一的提取器错误形态，使参数错误的退出路径与其余端点不一致。

选「白名单追加」而不是「放开默认 feature」：默认 feature 会带入本仓不用的 form、multipart 等能力面，扩大依赖编译面与审计面，且与冻结条目「逐项点名」的纪律冲突。追加单项使 feature 清单仍是完整的能力声明。

## 后果

正面：查询串解析、解码与错误形态全部由 axum 承载，清单类端点的参数错误与其它端点同一退出路径；feature 清单一处可读。

负面（如实记录）：编译产物多入一小段 serde_urlencoded 依赖路径；该依赖随 axum 版本走，升级 axum 时需按冻结纪律走一次变更登记。

## 影响范围

- workspace 根 `Cargo.toml` 第 37 行 axum 条目；
- `apps/core-server` 的密钥域清单与敏感字段清单两个查询串提取点；
- 02 计划 §12 实施期偏离登记第十三条。
