//! 安全上下文。字段顺序即技术基线第 1.4 节表格顺序，共 19 项，不得增删改名。

use std::sync::Arc;

use crate::error::{AppError, E_INVALID_ARGUMENT};
use crate::id::marker::{
    Customer, Department, LegalEntity, Position, Project, Session, UserAccount,
};
use crate::id::Id;
use crate::principal::{SYSTEM_DEVICE_ID, SYSTEM_PRINCIPAL_ID};
use crate::security::level::SecurityLevel;

/// 校验 `Arc<str>` 形态的受约束字符串字段。
fn checked(
    field: &'static str,
    raw: &str,
    min: usize,
    max: usize,
    allowed: fn(char) -> bool,
) -> Result<Arc<str>, AppError> {
    let len = raw.chars().count();
    if len < min || len > max {
        return Err(AppError::new(
            E_INVALID_ARGUMENT,
            format!("{field} 长度需在 {min}..={max}，实际 {len}"),
        ));
    }
    if let Some(bad) = raw.chars().find(|c| !allowed(*c)) {
        return Err(AppError::new(
            E_INVALID_ARGUMENT,
            format!("{field} 含非法字符 {bad:?}"),
        ));
    }
    Ok(Arc::from(raw))
}

macro_rules! arc_str_newtype {
    ($name:ident, $min:expr, $max:expr, $pred:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $name(Arc<str>);

        impl $name {
            pub fn new(raw: &str) -> Result<Self, AppError> {
                checked(stringify!($name), raw, $min, $max, $pred).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

fn is_device(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}
fn is_role(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
}
fn is_request(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}
fn is_trace(c: char) -> bool {
    c.is_ascii_digit() || ('a'..='f').contains(&c)
}

arc_str_newtype!(DeviceId, 1, 64, is_device);
arc_str_newtype!(RoleCode, 1, 64, is_role);
arc_str_newtype!(RequestId, 8, 64, is_request);
arc_str_newtype!(TraceId, 32, 32, is_trace);

/// 形态为 `<kind>:<value>`，kind 取 `[a-z0-9_-]`，value 取 `[A-Za-z0-9_-]`，总长上限 128。
///
/// 其 `Display` 与 serde 输出即公共列 `data_scope_tags text[]` 的元素形态，
/// 也是事件信封 `data_scope_tags` 的元素形态，两处不得各自编解码。
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DataScopeTag(Arc<str>);

impl DataScopeTag {
    pub fn new(raw: &str) -> Result<Self, AppError> {
        let err = |m: String| AppError::new(E_INVALID_ARGUMENT, m);
        if raw.chars().count() > 128 {
            return Err(err(format!("DataScopeTag 总长上限 128，实际 {}", raw.chars().count())));
        }
        let (kind, value) = raw
            .split_once(':')
            .ok_or_else(|| err("DataScopeTag 形态需为 <kind>:<value>".to_string()))?;
        if kind.is_empty() || value.is_empty() {
            return Err(err("DataScopeTag 的 kind 与 value 均不可为空".to_string()));
        }
        let kind_ok = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-';
        let value_ok = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '-';
        if let Some(bad) = kind.chars().find(|c| !kind_ok(*c)) {
            return Err(err(format!("DataScopeTag 的 kind 含非法字符 {bad:?}")));
        }
        if let Some(bad) = value.chars().find(|c| !value_ok(*c)) {
            return Err(err(format!("DataScopeTag 的 value 含非法字符 {bad:?}")));
        }
        Ok(Self(Arc::from(raw)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for DataScopeTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum AccountKind {
    Human,
    System,
    Portal,
}

/// 序列化取值与第 5.6 节 X-Client 头的六个取值一一对应。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum ClientKind {
    Win,
    Mac,
    Ios,
    Android,
    Portal,
    Ops,
}

/// 序列化取值与 `platform_authz.roles.duty_class` 的六个字符串逐字一致。
///
/// 该列为空的业务角色不产生任何项，`Arc<[DutyClass]>` 允许为空数组，不设 None 变体。
/// 职责分离的两两互斥关系是种子规则行的内容，不进本枚举。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum DutyClass {
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "DATA")]
    Data,
    #[serde(rename = "SECURITY")]
    Security,
    #[serde(rename = "AUDIT")]
    Audit,
    #[serde(rename = "KEY")]
    Key,
    #[serde(rename = "CONFIG")]
    Config,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, serde::Deserialize)]
pub enum RecordShareGrant {
    Read,
    Write,
}

/// 只表达「一条具体记录被显式共享给当前主体」这一事实，不含任何判定语义。
///
/// `object_type` 与事件信封的 `aggregate_type` 同形，即 `<module>.<table>` 的
/// 小写下划线形态。记录范围的编译结果与谓词类型留在 ep-platform-authz。
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RecordShare {
    pub object_type: Arc<str>,
    pub object_id: uuid::Uuid,
    pub grant: RecordShareGrant,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DepartmentScope {
    All,
    Subtree(Id<Department>),
    Explicit(Arc<[Id<Department>]>),
}

/// 19 个字段，顺序即技术基线第 1.4 节表格顺序。
#[derive(Clone, Debug)]
pub struct SecurityContext {
    pub user_id: Id<UserAccount>,
    pub account_kind: AccountKind,
    pub session_id: Id<Session>,
    pub legal_entity_id: Id<LegalEntity>,
    pub device_id: DeviceId,
    pub client: ClientKind,
    pub clearance_level: SecurityLevel,
    pub roles: Arc<[RoleCode]>,
    pub duty_classes: Arc<[DutyClass]>,
    pub department_scope: DepartmentScope,
    pub position_ids: Arc<[Id<Position>]>,
    pub project_scope: Arc<[Id<Project>]>,
    pub customer_scope: Arc<[Id<Customer>]>,
    pub record_shares: Arc<[RecordShare]>,
    pub data_scope_tags: Arc<[DataScopeTag]>,
    pub snapshot_version: u64,
    pub is_breakglass: bool,
    pub request_id: RequestId,
    pub trace_id: TraceId,
}

/// `SecurityContext::human` 的入参。19 个字段全部由调用方给出，
/// 单独成型是因为 clippy 的 too_many_arguments 与可读性，字段与上表一一对应。
#[derive(Clone, Debug)]
pub struct HumanContextInput {
    pub user_id: Id<UserAccount>,
    pub session_id: Id<Session>,
    pub legal_entity_id: Id<LegalEntity>,
    pub device_id: DeviceId,
    pub client: ClientKind,
    pub clearance_level: SecurityLevel,
    pub roles: Arc<[RoleCode]>,
    pub duty_classes: Arc<[DutyClass]>,
    pub department_scope: DepartmentScope,
    pub position_ids: Arc<[Id<Position>]>,
    pub project_scope: Arc<[Id<Project>]>,
    pub customer_scope: Arc<[Id<Customer>]>,
    pub record_shares: Arc<[RecordShare]>,
    pub data_scope_tags: Arc<[DataScopeTag]>,
    pub snapshot_version: u64,
    pub is_breakglass: bool,
    pub request_id: RequestId,
    pub trace_id: TraceId,
}

impl SecurityContext {
    /// 构造函数只有 human 与 system 两个。不提供任何 with_ 前缀的变换方法。
    pub fn human(input: HumanContextInput) -> Self {
        Self {
            user_id: input.user_id,
            account_kind: AccountKind::Human,
            session_id: input.session_id,
            legal_entity_id: input.legal_entity_id,
            device_id: input.device_id,
            client: input.client,
            clearance_level: input.clearance_level,
            roles: input.roles,
            duty_classes: input.duty_classes,
            department_scope: input.department_scope,
            position_ids: input.position_ids,
            project_scope: input.project_scope,
            customer_scope: input.customer_scope,
            record_shares: input.record_shares,
            data_scope_tags: input.data_scope_tags,
            snapshot_version: input.snapshot_version,
            is_breakglass: input.is_breakglass,
            request_id: input.request_id,
            trace_id: input.trace_id,
        }
    }

    /// 用 `SYSTEM_PRINCIPAL_ID` 与 `SYSTEM_DEVICE_ID` 填 user_id 与 device_id，
    /// account_kind 取 System。
    pub fn system(
        legal_entity_id: Id<LegalEntity>,
        request_id: RequestId,
        trace_id: TraceId,
    ) -> Self {
        let device_id = DeviceId::new(SYSTEM_DEVICE_ID)
            .expect("SYSTEM_DEVICE_ID 必须能由 &'static str 无损构造");
        Self {
            user_id: Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            account_kind: AccountKind::System,
            session_id: Id::from_uuid(SYSTEM_PRINCIPAL_ID),
            legal_entity_id,
            device_id,
            client: ClientKind::Ops,
            clearance_level: SecurityLevel::Secret,
            roles: Arc::from([] as [RoleCode; 0]),
            duty_classes: Arc::from([DutyClass::System]),
            department_scope: DepartmentScope::All,
            position_ids: Arc::from([] as [Id<Position>; 0]),
            project_scope: Arc::from([] as [Id<Project>; 0]),
            customer_scope: Arc::from([] as [Id<Customer>; 0]),
            record_shares: Arc::from([] as [RecordShare; 0]),
            data_scope_tags: Arc::from([] as [DataScopeTag; 0]),
            snapshot_version: 0,
            is_breakglass: false,
            request_id,
            trace_id,
        }
    }
}
