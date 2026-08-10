//! plugin-host 的装配。IPC 方法表在这里注入。
//!
//! 本阶段只有 `system.ping` 与 `system.version` 两个方法，没有 WASM 宿主。

use std::sync::Arc;

use ep_adapter_ipc::{IpcMethod, MethodTable};
use ep_platform_runtime::http::SystemState;
use serde_json::{json, Value};

pub const METHODS: [&str; 2] = ["system.ping", "system.version"];

pub struct SystemPing {
    state: Arc<SystemState>,
}

#[async_trait::async_trait]
impl IpcMethod for SystemPing {
    async fn call(&self, _payload: Value) -> Result<Value, String> {
        Ok(json!({ "process": self.state.process().name(), "version": self.state.build().version }))
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
        }))
    }
}

pub fn method_table(state: Arc<SystemState>) -> MethodTable {
    MethodTable::new()
        .with(METHODS[0], Arc::new(SystemPing { state: state.clone() }))
        .with(METHODS[1], Arc::new(SystemVersion { state }))
}
