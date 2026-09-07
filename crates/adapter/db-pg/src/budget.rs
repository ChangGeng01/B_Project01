//! 四具名池的种类、规模与连接预算（裁定 C-04：四类型声明并实现在
//! ep-adapter-db-pg，不进 foundation）。
//!
//! 当前 P340 实现种子：常驻连接合计 37、硬峰值 52。签名的全机
//! generation/digest 尚未落地前，这一组值必须逐项相等，避免三个进程从
//! 不同配置文件各自缩小“别人的池”而共同超配。装配侧在启动时调用
//! [`ConnectionBudget::validate`]
//! 求和校验，超限即以退出码 78（ep-platform-runtime 的
//! `EXIT_CONFIG_OR_SELFCHECK`）拒绝启动，不带病运行。

/// 四个具名池，顺序即登记表顺序。integration-gateway 零数据库，
/// 因此不存在 `Integ` 取值。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PoolKind {
    Rw,
    Ro,
    Worker,
    Ops,
}

/// 逻辑池在当前单机拓扑中的唯一进程持有者。
/// 跨进程无法共享内存连接池，因此必须先分配唯一所有权，
/// 再按全机四池求和；不得让每个进程各建四池。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PoolOwner {
    CoreServer,
    JobWorker,
    OpsAgent,
}

impl PoolOwner {
    pub const fn kinds(self) -> &'static [PoolKind] {
        match self {
            PoolOwner::CoreServer => &[PoolKind::Rw, PoolKind::Ro],
            PoolOwner::JobWorker => &[PoolKind::Worker],
            PoolOwner::OpsAgent => &[PoolKind::Ops],
        }
    }
}

impl PoolKind {
    pub const ALL: [PoolKind; 4] = [PoolKind::Rw, PoolKind::Ro, PoolKind::Worker, PoolKind::Ops];

    /// 指标与连接串的 pool 标签取值。
    pub const fn label(self) -> &'static str {
        match self {
            PoolKind::Rw => "rw",
            PoolKind::Ro => "ro",
            PoolKind::Worker => "worker",
            PoolKind::Ops => "ops",
        }
    }

    /// 当前 P340 单机拓扑的唯一进程所有者。
    pub const fn owner(self) -> PoolOwner {
        match self {
            PoolKind::Rw | PoolKind::Ro => PoolOwner::CoreServer,
            PoolKind::Worker => PoolOwner::JobWorker,
            PoolKind::Ops => PoolOwner::OpsAgent,
        }
    }
}

/// 一个具名池的规模声明。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoolSpec {
    pub kind: PoolKind,
    pub max_connections: u16,
}

/// 当前 P340 测量种子 Rw20/Ro10/Worker5/Ops2，与配置默认值逐池一致。
pub const STANDARD_POOL_SPECS: [PoolSpec; 4] = [
    PoolSpec {
        kind: PoolKind::Rw,
        max_connections: 20,
    },
    PoolSpec {
        kind: PoolKind::Ro,
        max_connections: 10,
    },
    PoolSpec {
        kind: PoolKind::Worker,
        max_connections: 5,
    },
    PoolSpec {
        kind: PoolKind::Ops,
        max_connections: 2,
    },
];

/// 预算违例的三种形态。装配侧把它映射为退出码 78 的启动失败。
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BudgetViolation {
    /// 四池数组不按唯一登记顺序给出；位置与凭据/超时语义不能猜测。
    PoolTopologyMismatch {
        position: usize,
        expected: PoolKind,
        actual: PoolKind,
    },
    /// 当前实现尚无跨进程签名预算工件，偏离冻结种子一律拒启。
    UncertifiedSeedDrift {
        field: &'static str,
        actual: u16,
        expected: u16,
    },
    /// 常驻连接总和超过 resident_max。
    ResidentOverflow { sum: u32, limit: u16 },
    /// 常驻池 + 临时连接 + 安全余量超过 peak_max。
    PeakOverflow { sum: u32, limit: u16 },
    /// 单个池的规模超出峰值总预算，独占即超限。
    PoolOverflow {
        kind: PoolKind,
        max: u16,
        limit: u16,
    },
}

/// 连接预算：当前 P340 常驻 37、临时 10、安全余量 5、
/// 硬峰值 52 与四池规模表。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConnectionBudget {
    pub resident_max: u16,
    pub temporary_max: u16,
    pub safety_headroom: u16,
    pub peak_max: u16,
    pub per_pool: [(PoolKind, u16); 4],
}

impl ConnectionBudget {
    pub const STANDARD_SAFETY_HEADROOM: u16 = 5;

    /// 当前 P340 测量种子：resident 37、peak 52 与标准四池规模。
    pub fn standard() -> Self {
        Self {
            resident_max: 37,
            temporary_max: 10,
            safety_headroom: Self::STANDARD_SAFETY_HEADROOM,
            peak_max: 52,
            per_pool: [
                (PoolKind::Rw, 20),
                (PoolKind::Ro, 10),
                (PoolKind::Worker, 5),
                (PoolKind::Ops, 2),
            ],
        }
    }

    /// 由四池规模表构造预算。保留调用方给出的顺序，让 [`Self::validate`]
    /// 能拒绝乱序、重复或缺项；不得搜索后补零来掩盖配置错误。
    pub fn from_specs(
        resident_max: u16,
        temporary_max: u16,
        peak_max: u16,
        specs: &[PoolSpec; 4],
    ) -> Self {
        let per_pool = specs.map(|spec| (spec.kind, spec.max_connections));
        Self {
            resident_max,
            temporary_max,
            safety_headroom: Self::STANDARD_SAFETY_HEADROOM,
            peak_max,
            per_pool,
        }
    }

    fn total(&self) -> u32 {
        self.per_pool.iter().map(|(_, n)| u32::from(*n)).sum()
    }

    /// 启动求和校验。三类违例全部收集后一次返回，方便启动日志定位。
    pub fn validate(&self) -> Result<(), Vec<BudgetViolation>> {
        let mut problems = Vec::new();
        let standard = Self::standard();
        for (position, expected_kind) in PoolKind::ALL.iter().enumerate() {
            let (actual_kind, actual_max) = self.per_pool[position];
            if actual_kind != *expected_kind {
                problems.push(BudgetViolation::PoolTopologyMismatch {
                    position,
                    expected: *expected_kind,
                    actual: actual_kind,
                });
            }
            let expected_max = standard.per_pool[position].1;
            if actual_max != expected_max {
                problems.push(BudgetViolation::UncertifiedSeedDrift {
                    field: expected_kind.label(),
                    actual: actual_max,
                    expected: expected_max,
                });
            }
        }
        for (field, actual, expected) in [
            ("resident_max", self.resident_max, standard.resident_max),
            ("temporary_max", self.temporary_max, standard.temporary_max),
            (
                "safety_headroom",
                self.safety_headroom,
                standard.safety_headroom,
            ),
            ("peak_max", self.peak_max, standard.peak_max),
        ] {
            if actual != expected {
                problems.push(BudgetViolation::UncertifiedSeedDrift {
                    field,
                    actual,
                    expected,
                });
            }
        }
        let sum = self.total();
        if sum > u32::from(self.resident_max) {
            problems.push(BudgetViolation::ResidentOverflow {
                sum,
                limit: self.resident_max,
            });
        }
        let peak_required = sum + u32::from(self.temporary_max) + u32::from(self.safety_headroom);
        if peak_required > u32::from(self.peak_max) {
            problems.push(BudgetViolation::PeakOverflow {
                sum: peak_required,
                limit: self.peak_max,
            });
        }
        for (kind, max) in &self.per_pool {
            if u32::from(*max) > u32::from(self.peak_max) {
                problems.push(BudgetViolation::PoolOverflow {
                    kind: *kind,
                    max: *max,
                    limit: self.peak_max,
                });
            }
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_labels_are_the_four_registered_values() {
        let labels: Vec<&str> = PoolKind::ALL.iter().map(|k| k.label()).collect();
        assert_eq!(labels, ["rw", "ro", "worker", "ops"]);
    }

    #[test]
    fn four_logical_pools_have_one_disjoint_process_owner() {
        assert_eq!(PoolKind::Rw.owner(), PoolOwner::CoreServer);
        assert_eq!(PoolKind::Ro.owner(), PoolOwner::CoreServer);
        assert_eq!(PoolKind::Worker.owner(), PoolOwner::JobWorker);
        assert_eq!(PoolKind::Ops.owner(), PoolOwner::OpsAgent);
        assert_eq!(PoolOwner::CoreServer.kinds(), &[PoolKind::Rw, PoolKind::Ro]);
        assert_eq!(PoolOwner::JobWorker.kinds(), &[PoolKind::Worker]);
        assert_eq!(PoolOwner::OpsAgent.kinds(), &[PoolKind::Ops]);
    }

    #[test]
    fn standard_budget_matches_rw20_ro10_worker5_ops2() {
        let b = ConnectionBudget::standard();
        assert_eq!(b.resident_max, 37);
        assert_eq!(b.temporary_max, 10);
        assert_eq!(b.safety_headroom, 5);
        assert_eq!(b.peak_max, 52);
        assert_eq!(b.total(), 37, "当前四池常驻合计恰为 37");
        assert!(b.validate().is_ok(), "标准预算必须通过校验");
    }

    #[test]
    fn resident_overflow_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.resident_max = 36;
        let errs = b.validate().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::ResidentOverflow { sum: 37, limit: 36 })));
    }

    #[test]
    fn peak_overflow_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.peak_max = 51;
        let errs = b.validate().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::PeakOverflow { sum: 52, limit: 51 })));
        assert!(
            !errs
                .iter()
                .any(|e| matches!(e, BudgetViolation::PoolOverflow { .. })),
            "单池最大 20 未超 51，不得误报单池越界"
        );
    }

    #[test]
    fn temporary_and_safety_capacity_are_both_reserved_at_peak() {
        let mut b = ConnectionBudget::standard();
        b.temporary_max = 11;
        let errs = b.validate().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::PeakOverflow { sum: 53, limit: 52 })));
    }

    #[test]
    fn a_single_pool_larger_than_burst_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.per_pool[0] = (PoolKind::Rw, 130);
        b.resident_max = 200;
        b.peak_max = 120;
        let errs = b.validate().unwrap_err();
        assert!(errs.iter().any(|e| matches!(
            e,
            BudgetViolation::PoolOverflow {
                kind: PoolKind::Rw,
                max: 130,
                limit: 120
            }
        )));
    }

    #[test]
    fn reordered_specs_are_rejected_instead_of_silently_realigned() {
        let specs = [
            PoolSpec {
                kind: PoolKind::Ops,
                max_connections: 2,
            },
            PoolSpec {
                kind: PoolKind::Rw,
                max_connections: 20,
            },
            PoolSpec {
                kind: PoolKind::Ro,
                max_connections: 10,
            },
            PoolSpec {
                kind: PoolKind::Worker,
                max_connections: 5,
            },
        ];
        let errs = ConnectionBudget::from_specs(37, 10, 52, &specs)
            .validate()
            .unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::PoolTopologyMismatch { position: 0, .. })));
    }

    #[test]
    fn duplicate_kind_and_missing_kind_are_rejected() {
        let mut specs = STANDARD_POOL_SPECS;
        specs[3].kind = PoolKind::Worker;
        let errs = ConnectionBudget::from_specs(37, 10, 52, &specs)
            .validate()
            .unwrap_err();
        assert!(errs.iter().any(|e| matches!(
            e,
            BudgetViolation::PoolTopologyMismatch {
                position: 3,
                expected: PoolKind::Ops,
                actual: PoolKind::Worker
            }
        )));
    }

    #[test]
    fn three_processes_cannot_each_hide_drift_in_other_pools() {
        for drifted in [
            [
                (PoolKind::Rw, 22),
                (PoolKind::Ro, 8),
                (PoolKind::Worker, 5),
                (PoolKind::Ops, 2),
            ],
            [
                (PoolKind::Rw, 20),
                (PoolKind::Ro, 8),
                (PoolKind::Worker, 7),
                (PoolKind::Ops, 2),
            ],
            [
                (PoolKind::Rw, 20),
                (PoolKind::Ro, 10),
                (PoolKind::Worker, 3),
                (PoolKind::Ops, 4),
            ],
        ] {
            let mut budget = ConnectionBudget::standard();
            budget.per_pool = drifted;
            assert!(budget
                .validate()
                .unwrap_err()
                .iter()
                .any(|e| matches!(e, BudgetViolation::UncertifiedSeedDrift { .. })));
        }
    }

    #[test]
    fn configured_peak_cannot_exceed_live_probe_floor_without_a_new_generation() {
        let mut budget = ConnectionBudget::standard();
        budget.peak_max = 53;
        assert!(budget.validate().unwrap_err().iter().any(|e| matches!(
            e,
            BudgetViolation::UncertifiedSeedDrift {
                field: "peak_max",
                actual: 53,
                expected: 52
            }
        )));
    }
}
