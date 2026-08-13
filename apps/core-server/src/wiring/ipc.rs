//! IPC 服务端的方法装配。
//!
//! 传输实现在 `ep-adapter-ipc`，进程状态在 `ep-platform-runtime`，两者在这里汇合：
//! 平台层不依赖适配层，所以这一次注入只能发生在 apps 里。

use std::sync::Arc;

use ep_adapter_ipc::{IpcMethod, MethodTable};
use ep_platform_runtime::http::SystemState;
use serde_json::{json, Value};

/// 本阶段只有两个方法。基线第 2 节的四类上报由阶段 14 连同方法名一次定义，
/// 这里不预留方法名，也不在协议文档中占位。
pub const METHODS: [&str; 2] = ["system.ping", "system.version"];

pub struct SystemPing {
    state: Arc<SystemState>,
}

#[async_trait::async_trait]
impl IpcMethod for SystemPing {
    async fn call(&self, _payload: Value) -> Result<Value, String> {
        Ok(json!({
            "process": self.state.process().name(),
            "version": self.state.build().version,
        }))
    }
}

pub struct SystemVersion {
    state: Arc<SystemState>,
}

#[async_trait::async_trait]
impl IpcMethod for SystemVersion {
    async fn call(&self, _payload: Value) -> Result<Value, String> {
        let b = self.state.build();
        Ok(json!({
            "process": self.state.process().name(),
            "version": b.version,
            "git_commit": b.git_commit,
            "source_date_epoch": b.source_date_epoch,
            "migration_manifest_sha256": b.migration_manifest_sha256,
        }))
    }
}

pub fn method_table(state: Arc<SystemState>) -> MethodTable {
    MethodTable::new()
        .with(
            METHODS[0],
            Arc::new(SystemPing {
                state: state.clone(),
            }),
        )
        .with(METHODS[1], Arc::new(SystemVersion { state }))
}
