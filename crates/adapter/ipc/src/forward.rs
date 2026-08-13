//! 落 spool 与补写。两个写出进程的形态完全相同，因此只有一份实现。
//!
//! 顺序是先补写再发当前帧：颠倒过来会让恢复后的第一帧插到历史帧前面，
//! 而写出进程的上报是有先后语义的。

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::client::IpcClient;
use crate::spool::{Spool, SpoolError};

/// 一条待上报的记录。落盘的就是它，不是完整的请求帧——
/// 请求 id 每次重发都应是新的，把旧 id 一起落盘会让重发看起来像重放。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pending {
    pub method: String,
    pub payload: Value,
}

#[derive(Debug, PartialEq)]
pub enum ForwardOutcome {
    Sent,
    /// 对端不可用，已落盘。`evicted` 非零时调用方必须记 ERROR。
    Spooled {
        evicted: usize,
        reason: String,
    },
    /// 连盘都落不下。这是最坏的一档，必须让调用方看见。
    Lost {
        reason: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum ReplayOutcome {
    Nothing,
    /// 全部补写成功，spool 已截断。
    Replayed {
        count: usize,
    },
    /// 补写到第 `ok` 条时失败，其余保留在 spool 中等下一轮。
    Partial {
        ok: usize,
        remaining: usize,
        reason: String,
    },
    Broken {
        reason: String,
    },
}

pub struct Forwarder {
    client: IpcClient,
    spool: Spool,
}

impl Forwarder {
    pub fn new(client: IpcClient, spool: Spool) -> Self {
        Self { client, spool }
    }

    pub fn spool(&self) -> &Spool {
        &self.spool
    }

    /// 先补写历史帧，再发当前帧；当前帧发不出去就落盘。
    pub async fn send(&self, pending: &Pending) -> (ReplayOutcome, ForwardOutcome) {
        let replay = self.replay().await;
        let forward = match self
            .client
            .call(&pending.method, pending.payload.clone())
            .await
        {
            Ok(_) => ForwardOutcome::Sent,
            Err(e) => self.spool_it(pending, &e.to_string()),
        };
        (replay, forward)
    }

    fn spool_it(&self, pending: &Pending, reason: &str) -> ForwardOutcome {
        let line = match serde_json::to_string(pending) {
            Ok(l) => l,
            Err(e) => {
                return ForwardOutcome::Lost {
                    reason: format!("待上报记录不可序列化：{e}"),
                }
            }
        };
        match self.spool.append(&line) {
            Ok(outcome) => ForwardOutcome::Spooled {
                evicted: outcome.evicted,
                reason: reason.to_string(),
            },
            Err(e) => ForwardOutcome::Lost {
                reason: format!("{e}"),
            },
        }
    }

    /// 按写入顺序补写。全部成功才截断，部分成功保留其余。
    pub async fn replay(&self) -> ReplayOutcome {
        let lines = match self.spool.read_lines() {
            Ok(l) => l,
            Err(e) => {
                return ReplayOutcome::Broken {
                    reason: e.to_string(),
                }
            }
        };
        if lines.is_empty() {
            return ReplayOutcome::Nothing;
        }
        let total = lines.len();
        let mut sent = 0;
        for line in &lines {
            let pending: Pending = match serde_json::from_str(line) {
                Ok(p) => p,
                // 坏行不能永远卡住队列，但也不能假装发过：单独丢弃并计入已处理，
                // 由返回值里的 reason 让调用方记 ERROR。
                Err(e) => {
                    sent += 1;
                    let _ = self.drop_sent(sent);
                    return ReplayOutcome::Partial {
                        ok: sent,
                        remaining: total - sent,
                        reason: format!("spool 中有不可解析的记录，已丢弃：{e}"),
                    };
                }
            };
            if let Err(e) = self.client.call(&pending.method, pending.payload).await {
                if sent > 0 {
                    if let Err(se) = self.drop_sent(sent) {
                        return ReplayOutcome::Broken {
                            reason: se.to_string(),
                        };
                    }
                }
                return ReplayOutcome::Partial {
                    ok: sent,
                    remaining: total - sent,
                    reason: e.to_string(),
                };
            }
            sent += 1;
        }
        match self.spool.truncate() {
            Ok(()) => ReplayOutcome::Replayed { count: sent },
            Err(e) => ReplayOutcome::Broken {
                reason: e.to_string(),
            },
        }
    }

    fn drop_sent(&self, n: usize) -> Result<(), SpoolError> {
        self.spool.drop_first(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::{IpcMethod, IpcServer, MethodTable};
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;

    struct Accept;

    #[async_trait::async_trait]
    impl IpcMethod for Accept {
        async fn call(&self, _payload: Value) -> Result<Value, String> {
            Ok(Value::Null)
        }
    }

    fn dirs(name: &str) -> (PathBuf, PathBuf) {
        let base = std::env::temp_dir().join(format!("ep-fwd-{}-{name}", std::process::id()));
        std::fs::remove_dir_all(&base).ok();
        std::fs::create_dir_all(base.join("spool")).unwrap();
        (base.join("s.sock"), base.join("spool"))
    }

    fn pending(n: u32) -> Pending {
        Pending {
            method: "system.ping".into(),
            payload: serde_json::json!({ "n": n }),
        }
    }

    #[tokio::test]
    async fn frames_are_spooled_while_the_peer_is_down_and_replayed_after_it_returns() {
        let (sock, spool_dir) = dirs("replay");
        let client = IpcClient::new(&sock, 4096, Duration::from_millis(300));
        let fwd = Forwarder::new(client, Spool::new(&spool_dir, 4096));

        // 对端不可用：两帧都落盘。
        for n in 0..2 {
            let (_, out) = fwd.send(&pending(n)).await;
            assert!(
                matches!(out, ForwardOutcome::Spooled { evicted: 0, .. }),
                "{out:?}"
            );
        }
        assert_eq!(fwd.spool().read_lines().unwrap().len(), 2);

        // 对端恢复：补写并截断。
        let server = IpcServer::new(
            &sock,
            4096,
            MethodTable::new().with("system.ping", Arc::new(Accept)),
        );
        let listener = server.bind().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            server
                .serve(listener, async move {
                    let _ = rx.await;
                })
                .await;
        });

        let (replay, out) = fwd.send(&pending(2)).await;
        assert_eq!(replay, ReplayOutcome::Replayed { count: 2 });
        assert_eq!(out, ForwardOutcome::Sent);
        assert!(
            fwd.spool().read_lines().unwrap().is_empty(),
            "补写成功后必须截断"
        );

        let _ = tx.send(());
        let _ = handle.await;
    }

    // 负样例断言的是「发不出去必须落盘且被看见」这条规则本身：
    // 落盘也失败时不得返回 Sent。
    #[tokio::test]
    async fn a_record_that_cannot_be_spooled_is_reported_as_lost() {
        let (sock, spool_dir) = dirs("lost");
        let client = IpcClient::new(&sock, 4096, Duration::from_millis(100));
        // spool 上限比一条记录还小，落盘必然失败。
        let fwd = Forwarder::new(client, Spool::new(&spool_dir, 4));
        let (_, out) = fwd.send(&pending(1)).await;
        assert!(matches!(out, ForwardOutcome::Lost { .. }), "{out:?}");
    }

    #[tokio::test]
    async fn nothing_to_replay_is_not_an_error() {
        let (sock, spool_dir) = dirs("empty");
        let client = IpcClient::new(&sock, 4096, Duration::from_millis(100));
        let fwd = Forwarder::new(client, Spool::new(&spool_dir, 4096));
        assert_eq!(fwd.replay().await, ReplayOutcome::Nothing);
    }

    #[tokio::test]
    async fn an_unparsable_spool_line_is_dropped_and_reported() {
        let (sock, spool_dir) = dirs("bad");
        let spool = Spool::new(&spool_dir, 4096);
        spool.append("这不是 JSON").unwrap();
        let client = IpcClient::new(&sock, 4096, Duration::from_millis(100));
        let fwd = Forwarder::new(client, spool);
        match fwd.replay().await {
            ReplayOutcome::Partial {
                ok: 1,
                remaining: 0,
                reason,
            } => {
                assert!(reason.contains("不可解析"), "{reason}");
            }
            other => panic!("坏行必须被丢弃并如实上报，实际 {other:?}"),
        }
    }
}
