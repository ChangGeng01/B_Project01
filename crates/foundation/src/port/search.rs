//! 检索端口。
//!
//! 阶段 1 只建空文件。SearchDocument、SearchQuery、SearchHit 与
//! SearchIndexPort、SearchQueryPort 由阶段 3b 按 A-07 补齐，
//! 实现落在 ep-adapter-search，索引按法人分区，写入一律经 job-worker
//! 消费 Outbox 事件触发，不在业务事务内调用。
