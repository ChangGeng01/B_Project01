//! AuthzSnapshot 运行形态：arc_swap 持有、按法人分片、整体替换。
//!
//! 唯一重载路径 = core-server 轮询 `authz_config_versions` 的 EFFECTIVE 版本号
//! （`EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS`，默认 2000ms）；用户维度授权集合
//! 在会话建立时读一次冻结进 SecurityContext，快照只承载策略与授予的读面。
//! checksum 不符时开 AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 降级窗口并沿用旧数据，
//! kind 取值属 ep-platform-obs 台账枚举，映射由 wiring 侧经本 crate 的
//! [`DegradationWindowOpener`] 端口完成。

use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

use arc_swap::ArcSwap;
use ep_foundation::error::AppError;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_foundation::port::tx::Tx;

use crate::types::{Action, FieldVisibility, ObjectScopeBinding, PolicyCondition, PolicyEffect};

/// 单个法人的授权数据。不可变，只整体替换，不做就地修改。
#[derive(Clone, Debug, Default)]
pub struct EntityAuthzData {
    /// `authz_config_versions.version_no` 的 EFFECTIVE 行。
    pub version_no: u64,
    /// 与版本行同载的校验和，重载时复核。
    pub checksum: Arc<str>,
    /// (角色码, 对象类型) → 授予的动作集合。由 role_permission_grants
    /// 经 permission_items.object_type 联接而成。
    pub role_grants: HashMap<(Arc<str>, Arc<str>), BTreeSet<Action>>,
    /// 门户可用角色（`roles.is_portal_role` 为真的角色码集合）。
    pub portal_roles: BTreeSet<Arc<str>>,
    /// access_policies，按 priority 升序（数值小者先生效）。
    pub policies: Vec<AccessPolicyEntry>,
    /// field_permissions。
    pub field_grants: Vec<FieldGrantEntry>,
}

/// 一条访问策略的内存形态。
#[derive(Clone, Debug)]
pub struct AccessPolicyEntry {
    /// 空表示约束全部角色。
    pub role_code: Option<Arc<str>>,
    pub object_type: Arc<str>,
    pub effect: PolicyEffect,
    pub priority: i32,
    pub condition: PolicyCondition,
}

/// 一条字段授权的内存形态。
#[derive(Clone, Debug)]
pub struct FieldGrantEntry {
    pub role_code: Arc<str>,
    pub object_type: Arc<str>,
    pub field_name: Arc<str>,
    pub visibility: FieldVisibility,
}

/// 快照映射：按法人分片。
pub type SnapshotMap = HashMap<Id<LegalEntity>, Arc<EntityAuthzData>>;

/// 快照持有者。读侧无锁（arc_swap load），写侧整体替换。
pub struct AuthzSnapshotHolder {
    entities: ArcSwap<SnapshotMap>,
    object_types: ArcSwap<BTreeSet<Arc<str>>>,
    bindings: ArcSwap<HashMap<Arc<str>, ObjectScopeBinding>>,
}

impl AuthzSnapshotHolder {
    pub fn empty() -> Self {
        Self {
            entities: ArcSwap::from_pointee(SnapshotMap::new()),
            object_types: ArcSwap::from_pointee(BTreeSet::new()),
            bindings: ArcSwap::from_pointee(HashMap::new()),
        }
    }

    /// 取法人分片；未建分片即该法人无任何授权数据。
    pub fn entity(&self, legal_entity_id: Id<LegalEntity>) -> Option<Arc<EntityAuthzData>> {
        self.entities.load().get(&legal_entity_id).cloned()
    }

    /// 对象类型是否已在 `object_scope_bindings` 登记。未登记者记录级一律拒绝。
    pub fn is_object_registered(&self, object_type: &str) -> bool {
        self.object_types
            .load()
            .iter()
            .any(|t| t.as_ref() == object_type)
    }

    /// 取对象范围绑定。
    pub fn binding(&self, object_type: &str) -> Option<ObjectScopeBinding> {
        self.bindings.load().get(object_type).cloned()
    }

    /// 整体替换三个分面。
    pub fn replace(&self, entities: SnapshotMap, bindings: Vec<ObjectScopeBinding>) {
        let types: BTreeSet<Arc<str>> = bindings.iter().map(|b| b.object_type.clone()).collect();
        let map: HashMap<Arc<str>, ObjectScopeBinding> = bindings
            .into_iter()
            .map(|b| (b.object_type.clone(), b))
            .collect();
        self.entities.store(Arc::new(entities));
        self.object_types.store(Arc::new(types));
        self.bindings.store(Arc::new(map));
    }
}

/// `authz_config_versions` 的 EFFECTIVE 行摘要。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EffectiveVersion {
    pub legal_entity_id: Id<LegalEntity>,
    pub version_no: u64,
    pub checksum: Arc<str>,
}

/// 授权配置版本的查询端口。SQL 执行体落 ep-adapter-db-pg（platform_authz
/// schema），本端口只冻结调用面。
#[async_trait::async_trait]
pub trait AuthzConfigVersionQuery: Send + Sync {
    /// 查每个法人的 EFFECTIVE 版本号与校验和。
    async fn effective_versions(&self, tx: &mut dyn Tx) -> Result<Vec<EffectiveVersion>, AppError>;
    /// 按版本载入单法人的完整授权数据。
    async fn load_entity(
        &self,
        tx: &mut dyn Tx,
        legal_entity_id: Id<LegalEntity>,
        version_no: u64,
    ) -> Result<EntityAuthzData, AppError>;
    /// `object_scope_bindings` 全量（登记制，无列法人）。
    async fn object_bindings(&self, tx: &mut dyn Tx) -> Result<Vec<ObjectScopeBinding>, AppError>;
}

/// 一次重载的结果。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReloadOutcome {
    /// 版本号与绑定均未变化，未做替换。
    Unchanged,
    /// 整体替换完成。
    Applied { entities: usize },
    /// 某法人 checksum 不符：已开降级窗口，沿用旧分片。
    ChecksumMismatch { legal_entity_id: Id<LegalEntity> },
}

/// AUTHZ_SNAPSHOT_CHECKSUM_MISMATCH 降级窗口的开口。
pub trait DegradationWindowOpener: Send + Sync {
    fn open_checksum_mismatch_window(
        &self,
        legal_entity_id: Id<LegalEntity>,
        expected: &str,
        actual: &str,
    );
}

/// 轮询重载器。循环本体的启动归 wiring（阶段 4 集成任务），
/// 此处交付单次重载逻辑与轮询间隔取用。
pub struct SnapshotReloader {
    holder: Arc<AuthzSnapshotHolder>,
    query: Arc<dyn AuthzConfigVersionQuery>,
    opener: Arc<dyn DegradationWindowOpener>,
    poll_interval_ms: u32,
}

impl SnapshotReloader {
    pub fn new(
        holder: Arc<AuthzSnapshotHolder>,
        query: Arc<dyn AuthzConfigVersionQuery>,
        opener: Arc<dyn DegradationWindowOpener>,
        poll_interval_ms: u32,
    ) -> Self {
        Self {
            holder,
            query,
            opener,
            poll_interval_ms,
        }
    }

    /// `EP__AUTHZ__SNAPSHOT__POLL_INTERVAL_MS` 的取用值。
    pub fn poll_interval_ms(&self) -> u32 {
        self.poll_interval_ms
    }

    pub fn holder(&self) -> &Arc<AuthzSnapshotHolder> {
        &self.holder
    }

    /// 单次重载：查 EFFECTIVE 版本号与绑定，无变化即返回；有变化整体替换。
    pub async fn reload_once(&self, tx: &mut dyn Tx) -> Result<ReloadOutcome, AppError> {
        let versions = self.query.effective_versions(tx).await?;
        let bindings = self.query.object_bindings(tx).await?;
        let current = self.holder.entities.load_full();
        if !versions_changed(&current, &versions) && !bindings_changed(&self.holder, &bindings) {
            return Ok(ReloadOutcome::Unchanged);
        }
        let mut mismatch: Option<(Id<LegalEntity>, Arc<str>, Arc<str>)> = None;
        let mut next = SnapshotMap::new();
        for v in &versions {
            self.load_one(tx, &current, v, &mut next, &mut mismatch)
                .await?;
        }
        if let Some((le, expected, actual)) = &mismatch {
            self.opener
                .open_checksum_mismatch_window(*le, expected, actual);
        }
        self.holder.replace(next.clone(), bindings);
        Ok(match mismatch {
            Some((le, _, _)) => ReloadOutcome::ChecksumMismatch {
                legal_entity_id: le,
            },
            None => ReloadOutcome::Applied {
                entities: next.len(),
            },
        })
    }

    /// 载入单法人：版本与校验和均未变则复用旧分片；checksum 不符时
    /// 记录不符三元组并沿用旧分片（不放入 next 即由 replace 前的旧值承接，
    /// 此处显式回填旧分片）。
    async fn load_one(
        &self,
        tx: &mut dyn Tx,
        current: &SnapshotMap,
        v: &EffectiveVersion,
        next: &mut SnapshotMap,
        mismatch: &mut Option<(Id<LegalEntity>, Arc<str>, Arc<str>)>,
    ) -> Result<(), AppError> {
        if let Some(old) = current.get(&v.legal_entity_id) {
            if old.version_no == v.version_no && old.checksum == v.checksum {
                next.insert(v.legal_entity_id, old.clone());
                return Ok(());
            }
        }
        let data = self
            .query
            .load_entity(tx, v.legal_entity_id, v.version_no)
            .await?;
        if data.checksum != v.checksum {
            *mismatch = Some((v.legal_entity_id, v.checksum.clone(), data.checksum.clone()));
            if let Some(old) = current.get(&v.legal_entity_id) {
                next.insert(v.legal_entity_id, old.clone());
            }
            return Ok(());
        }
        next.insert(v.legal_entity_id, Arc::new(data));
        Ok(())
    }
}

fn versions_changed(current: &SnapshotMap, versions: &[EffectiveVersion]) -> bool {
    if current.len() != versions.len() {
        return true;
    }
    versions.iter().any(|v| {
        current
            .get(&v.legal_entity_id)
            .map(|d| d.version_no != v.version_no || d.checksum != v.checksum)
            .unwrap_or(true)
    })
}

fn bindings_changed(holder: &AuthzSnapshotHolder, bindings: &[ObjectScopeBinding]) -> bool {
    let current = holder.object_types.load();
    if current.len() != bindings.len() {
        return true;
    }
    bindings
        .iter()
        .any(|b| !current.iter().any(|t| t.as_ref() == b.object_type.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct FixtureTx;
    impl Tx for FixtureTx {
        fn tx_id(&self) -> ep_foundation::port::tx::TxId {
            ep_foundation::port::tx::TxId(uuid::Uuid::nil())
        }
        fn isolation(&self) -> ep_foundation::port::tx::IsolationKind {
            ep_foundation::port::tx::IsolationKind::ReadCommitted
        }
        fn legal_entity_id(&self) -> Id<LegalEntity> {
            le(1)
        }
        fn as_any_mut(&mut self) -> &mut (dyn core::any::Any + Send) {
            self
        }
    }

    fn le(n: u128) -> Id<LegalEntity> {
        Id::from_uuid(uuid::Uuid::from_u128(n))
    }

    /// 固定返回的版本查询载体：versions 与 checksum 可预先布置。
    struct FixtureVersionQuery {
        versions: Vec<EffectiveVersion>,
        loaded: Vec<EntityAuthzData>,
        bindings: Vec<ObjectScopeBinding>,
    }

    #[async_trait::async_trait]
    impl AuthzConfigVersionQuery for FixtureVersionQuery {
        async fn effective_versions(
            &self,
            _: &mut dyn Tx,
        ) -> Result<Vec<EffectiveVersion>, AppError> {
            Ok(self.versions.clone())
        }
        async fn load_entity(
            &self,
            _: &mut dyn Tx,
            legal_entity_id: Id<LegalEntity>,
            version_no: u64,
        ) -> Result<EntityAuthzData, AppError> {
            self.loaded
                .iter()
                .find(|d| d.version_no == version_no)
                .cloned()
                .ok_or_else(|| {
                    AppError::new(
                        ep_foundation::error::codes::PLATFORM_SYSTEM_INTERNAL_ERROR,
                        format!("版本 {version_no} 无数据，法人 {legal_entity_id}"),
                    )
                })
        }
        async fn object_bindings(
            &self,
            _: &mut dyn Tx,
        ) -> Result<Vec<ObjectScopeBinding>, AppError> {
            Ok(self.bindings.clone())
        }
    }

    #[derive(Default)]
    struct RecordingWindowOpener {
        calls: Mutex<Vec<(Id<LegalEntity>, String, String)>>,
    }

    impl DegradationWindowOpener for RecordingWindowOpener {
        fn open_checksum_mismatch_window(
            &self,
            legal_entity_id: Id<LegalEntity>,
            expected: &str,
            actual: &str,
        ) {
            self.calls.lock().unwrap_or_else(|p| p.into_inner()).push((
                legal_entity_id,
                expected.to_string(),
                actual.to_string(),
            ));
        }
    }

    fn binding(object_type: &str) -> ObjectScopeBinding {
        ObjectScopeBinding {
            object_type: Arc::from(object_type),
            schema_name: Arc::from("platform"),
            table_name: Arc::from("user_accounts"),
            owner_user_col: None,
            owning_dept_col: None,
            project_col: None,
            customer_col: None,
            security_level_col: Arc::from("security_level"),
            valid_from_col: None,
            valid_to_col: None,
        }
    }

    fn data(version_no: u64, checksum: &str) -> EntityAuthzData {
        EntityAuthzData {
            version_no,
            checksum: Arc::from(checksum),
            ..EntityAuthzData::default()
        }
    }

    fn reloader(
        query: FixtureVersionQuery,
    ) -> (
        SnapshotReloader,
        Arc<AuthzSnapshotHolder>,
        Arc<RecordingWindowOpener>,
    ) {
        let holder = Arc::new(AuthzSnapshotHolder::empty());
        let opener = Arc::new(RecordingWindowOpener::default());
        let r = SnapshotReloader::new(holder.clone(), Arc::new(query), opener.clone(), 2_000);
        (r, holder, opener)
    }

    #[tokio::test]
    async fn first_reload_applies_and_second_is_unchanged() {
        let query = FixtureVersionQuery {
            versions: vec![EffectiveVersion {
                legal_entity_id: le(1),
                version_no: 7,
                checksum: Arc::from("c7"),
            }],
            loaded: vec![data(7, "c7")],
            bindings: vec![binding("platform.user_accounts")],
        };
        let (r, holder, opener) = reloader(query);
        let mut tx = FixtureTx;
        assert_eq!(
            r.reload_once(&mut tx).await.expect("首次重载"),
            ReloadOutcome::Applied { entities: 1 }
        );
        assert!(holder.is_object_registered("platform.user_accounts"));
        assert!(holder.entity(le(1)).expect("分片存在").version_no == 7);
        assert_eq!(
            r.reload_once(&mut tx).await.expect("二次重载"),
            ReloadOutcome::Unchanged
        );
        assert!(opener
            .calls
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .is_empty());
        assert_eq!(r.poll_interval_ms(), 2_000);
    }

    #[tokio::test]
    async fn checksum_mismatch_opens_window_and_keeps_old_shard() {
        let query = FixtureVersionQuery {
            versions: vec![EffectiveVersion {
                legal_entity_id: le(1),
                version_no: 8,
                checksum: Arc::from("expected"),
            }],
            loaded: vec![data(8, "tampered")],
            bindings: vec![binding("platform.user_accounts")],
        };
        let holder = Arc::new(AuthzSnapshotHolder::empty());
        holder.replace(
            {
                let mut m = SnapshotMap::new();
                m.insert(le(1), Arc::new(data(7, "c7")));
                m
            },
            vec![binding("platform.user_accounts")],
        );
        let opener = Arc::new(RecordingWindowOpener::default());
        let r = SnapshotReloader::new(holder.clone(), Arc::new(query), opener.clone(), 2_000);
        let mut tx = FixtureTx;
        let outcome = r.reload_once(&mut tx).await.expect("重载完成");
        assert_eq!(
            outcome,
            ReloadOutcome::ChecksumMismatch {
                legal_entity_id: le(1)
            }
        );
        let kept = holder.entity(le(1)).expect("沿用旧分片");
        assert_eq!(kept.version_no, 7, "旧分片不被篡改数据替换");
        let calls = opener.calls.lock().unwrap_or_else(|p| p.into_inner());
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, "expected");
        assert_eq!(calls[0].2, "tampered");
    }
}
