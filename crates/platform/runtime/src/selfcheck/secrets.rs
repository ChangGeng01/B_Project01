//! `secrets-resolvable` 的两段式自检（02 计划 §7.1 与 E-11）。
//!
//! 该项按基线十项登记为 Blocking 一个档位，但按 02 计划 §7.1 必须拆两段：
//! 一、机密可解引用且 KMS 或 HSM 可用，取 Blocking 语义——失败即 [`Verdict::Fail`]，
//! 由档位映射为 FAILED，拒绝启动；
//! 二、每个法人的数据加密密钥域存在，取 Degrading 语义——缺域不阻断启动，
//! 经 [`DegradationLedger::open`] 逐法人登记降级窗口并上报 [`Verdict::Degraded`]。
//! 后一段必须降级而非阻断：建立密钥域的唯一入口是 A-03 端点，由 core-server
//! 承载，若缺域即拒绝启动则该端点永远不可达，形成自锁。
//!
//! 缺域开窗暂用 `PORT_NOT_IMPLEMENTED` 取值：本阶段定义的三个初始 kind 中
//! 只有它是通用缺位形态，表上 CHECK 约束限定不得另起取值（已向 leader 请示，
//! 待阶段 14 扩充 kind 清单后同批迁移），偏离已登记。
//!
//! 前提分流与 SQL 四项一致：四进程（portal-gateway、plugin-host、archive-writer、
//! backup-writer）不持常规数据库连接与密钥域，一律 NotApplicable；探针未装配
//! 报 Pending（未覆盖），绝不判通过。

use std::sync::Arc;

use ep_foundation::error::codes::PLATFORM_DB_MIGRATION_WINDOW_CONFLICT;
use ep_foundation::id::marker::LegalEntity;
use ep_foundation::id::Id;
use ep_platform_obs::degradation::{DegradationKind, DegradationLedger, WindowOpenRequest};

use crate::process::ProcessKind;
use crate::selfcheck::probe::ProbeError;
use crate::selfcheck::registry::{SelfCheckRun, Verdict};

/// 缺域降级窗口的 subject 取值。`PORT_NOT_IMPLEMENTED` 的窗口按约定由
/// subject 记下缺位对象的完整标识，这里记密钥域建立能力的归属端点。
pub const KEY_DOMAIN_WINDOW_SUBJECT: &str = "platform.key_domain.provisioning";

/// `secrets-resolvable` 的被测对象端口。实现由 apps 在 `wiring/` 目录下注入：
/// 第一段经 KMS 后端，第二段经法人目录与密钥域快照的组合读取。
#[async_trait::async_trait]
pub trait SecretsProbe: Send + Sync {
    /// 第一段（Blocking）：机密全部可解引用且密钥后端可用。
    async fn backend_available(&self) -> Result<(), ProbeError>;

    /// 第二段（Degrading）：返回缺失密钥域的存续法人清单。空清单即全部齐备。
    async fn legal_entities_missing_key_domain(&self) -> Result<Vec<Id<LegalEntity>>, ProbeError>;
}

/// `secrets-resolvable` 的判定。探针与降级台账都由装配侧注入：
/// 台账缺席时缺域仍如实上报降级结论，只是无窗可登。
pub struct SecretsResolvable {
    pub process: ProcessKind,
    pub probe: Option<Arc<dyn SecretsProbe>>,
    pub ledger: Option<Arc<dyn DegradationLedger>>,
}

#[async_trait::async_trait]
impl SelfCheckRun for SecretsResolvable {
    async fn run(&self) -> Verdict {
        if !self.process.holds_sql_session() {
            return Verdict::NotApplicable(format!(
                "{} 不持有常规数据库连接与密钥域，机密自检项不成立",
                self.process.name()
            ));
        }
        let Some(probe) = self.probe.as_ref() else {
            return Verdict::Pending(
                "未装机密探针：两段判定逻辑已就位，取数实现由装配侧提供，本项未覆盖".into(),
            );
        };
        if let Err(e) = probe.backend_available().await {
            return Verdict::Fail(format!("机密不可解引用或密钥后端不可用：{e}"));
        }
        let missing = match probe.legal_entities_missing_key_domain().await {
            Ok(m) => m,
            // 读不到不等于缺域，但同样无法证明齐备：按降级如实上报，不误判失败。
            Err(e) => return Verdict::Degraded(format!("无法核验密钥域齐备性：{e}")),
        };
        if missing.is_empty() {
            return Verdict::Pass("机密全部可解引用且各法人密钥域齐备".into());
        }
        let mut detail = format!("{} 个法人缺失密钥域", missing.len());
        match self.ledger.as_ref() {
            Some(ledger) => {
                let mut registered = 0usize;
                for id in &missing {
                    match ledger.open(missing_domain_window(*id)).await {
                        Ok(()) => registered += 1,
                        // 同一窗口已登记（重复启动）按幂等约定视为已在台账，不计失败。
                        Err(e) if e.code == PLATFORM_DB_MIGRATION_WINDOW_CONFLICT => {
                            registered += 1;
                        }
                        Err(e) => detail.push_str(&format!("；法人 {id} 的降级窗口登记失败：{e}")),
                    }
                }
                detail.push_str(&format!("，已登记降级窗口 {registered} 个"));
            }
            None => detail.push_str("；降级台账未装配，未登记窗口"),
        }
        Verdict::Degraded(detail)
    }
}

/// 一个缺域法人的开窗请求。kind 暂用 `PortNotImplemented`，见模块头。
fn missing_domain_window(entity: Id<LegalEntity>) -> WindowOpenRequest {
    WindowOpenRequest {
        kind: DegradationKind::PortNotImplemented,
        subject: Some(KEY_DOMAIN_WINDOW_SUBJECT.to_string()),
        scope_key: format!("legal-entity:{entity}"),
        scope_legal_entity_id: Some(entity),
        scope_accounting_period_id: None,
        basis: "启动自检 secrets-resolvable 密钥域段发现该法人缺失数据加密密钥域".to_string(),
        closing_condition: "经 A-03 端点为该法人建立密钥域后关闭窗口".to_string(),
        is_suppressible: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_platform_obs::degradation::WindowRef;

    /// 内存台账：记录开窗请求，供断言使用。
    #[derive(Default)]
    struct MemLedger(tokio::sync::Mutex<Vec<WindowOpenRequest>>);

    #[async_trait::async_trait]
    impl DegradationLedger for MemLedger {
        async fn open(&self, req: WindowOpenRequest) -> Result<(), ep_foundation::error::AppError> {
            self.0.lock().await.push(req);
            Ok(())
        }
        async fn close(&self, _w: WindowRef) -> Result<(), ep_foundation::error::AppError> {
            Ok(())
        }
        async fn open_count(&self) -> Result<usize, ep_foundation::error::AppError> {
            Ok(self.0.lock().await.len())
        }
    }

    struct FixedProbe {
        backend_ok: bool,
        missing: Vec<Id<LegalEntity>>,
    }

    #[async_trait::async_trait]
    impl SecretsProbe for FixedProbe {
        async fn backend_available(&self) -> Result<(), ProbeError> {
            self.backend_ok
                .then_some(())
                .ok_or_else(|| ProbeError("后端不可用".into()))
        }
        async fn legal_entities_missing_key_domain(
            &self,
        ) -> Result<Vec<Id<LegalEntity>>, ProbeError> {
            Ok(self.missing.clone())
        }
    }

    fn item(
        probe: Option<Arc<dyn SecretsProbe>>,
        ledger: Option<Arc<dyn DegradationLedger>>,
    ) -> SecretsResolvable {
        SecretsResolvable {
            process: ProcessKind::CoreServer,
            probe,
            ledger,
        }
    }

    #[tokio::test]
    async fn four_processes_without_sql_session_are_not_applicable() {
        for p in [
            ProcessKind::PortalGateway,
            ProcessKind::PluginHost,
            ProcessKind::ArchiveWriter,
            ProcessKind::BackupWriter,
        ] {
            let v = SecretsResolvable {
                process: p,
                probe: None,
                ledger: None,
            }
            .run()
            .await;
            assert!(matches!(v, Verdict::NotApplicable(_)), "{}", p.name());
        }
    }

    #[tokio::test]
    async fn absent_probe_is_pending_not_passed() {
        let v = item(None, None).run().await;
        assert!(matches!(v, Verdict::Pending(d) if d.contains("未覆盖")));
    }

    #[tokio::test]
    async fn backend_failure_is_blocking_fail() {
        let probe = Arc::new(FixedProbe {
            backend_ok: false,
            missing: Vec::new(),
        });
        let v = item(Some(probe), None).run().await;
        assert!(matches!(v, Verdict::Fail(_)));
    }

    #[tokio::test]
    async fn all_domains_present_passes() {
        let probe = Arc::new(FixedProbe {
            backend_ok: true,
            missing: Vec::new(),
        });
        let v = item(Some(probe), None).run().await;
        assert!(matches!(v, Verdict::Pass(_)));
    }

    #[tokio::test]
    async fn missing_domain_degrades_and_opens_a_window_per_entity() {
        let probe = Arc::new(FixedProbe {
            backend_ok: true,
            missing: vec![
                Id::from_uuid(uuid::Uuid::from_u128(1)),
                Id::from_uuid(uuid::Uuid::from_u128(2)),
            ],
        });
        let ledger = Arc::new(MemLedger::default());
        let v = item(Some(probe), Some(ledger.clone())).run().await;
        let Verdict::Degraded(detail) = v else {
            panic!("缺域必须报降级而非失败");
        };
        assert!(detail.contains("2 个法人缺失密钥域"));
        let windows = ledger.0.lock().await;
        assert_eq!(windows.len(), 2, "逐法人开窗");
        for w in windows.iter() {
            assert_eq!(w.kind, DegradationKind::PortNotImplemented);
            assert_eq!(w.subject.as_deref(), Some(KEY_DOMAIN_WINDOW_SUBJECT));
            assert!(!w.is_suppressible);
        }
    }
}
