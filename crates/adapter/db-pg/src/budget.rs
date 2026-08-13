//! 五具名池的种类、规模与连接预算（裁定 C-04：四类型声明并实现在
//! ep-adapter-db-pg，不进 foundation）。
//!
//! 预算纪律（规格第 7.7 章与基线 11.6）：常驻连接合计上限 42、
//! 峰值合计上限 52。装配侧在启动时调用 [`ConnectionBudget::validate`]
//! 求和校验，超限即以退出码 78（ep-platform-runtime 的
//! `EXIT_CONFIG_OR_SELFCHECK`）拒绝启动，不带病运行。

/// 五个具名池，顺序即登记表顺序。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PoolKind {
    Rw,
    Ro,
    Worker,
    Integ,
    Ops,
}

impl PoolKind {
    pub const ALL: [PoolKind; 5] = [
        PoolKind::Rw,
        PoolKind::Ro,
        PoolKind::Worker,
        PoolKind::Integ,
        PoolKind::Ops,
    ];

    /// 指标与连接串的 pool 标签取值。
    pub const fn label(self) -> &'static str {
        match self {
            PoolKind::Rw => "rw",
            PoolKind::Ro => "ro",
            PoolKind::Worker => "worker",
            PoolKind::Integ => "integ",
            PoolKind::Ops => "ops",
        }
    }
}

/// 一个具名池的规模声明。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PoolSpec {
    pub kind: PoolKind,
    pub max_connections: u16,
}

/// 标准规模 Rw20/Ro10/Worker5/Integ5/Ops2，与配置默认值逐池一致。
pub const STANDARD_POOL_SPECS: [PoolSpec; 5] = [
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
        kind: PoolKind::Integ,
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
    /// 常驻连接总和超过 resident_max。
    ResidentOverflow { sum: u32, limit: u16 },
    /// 峰值连接总和超过 burst_max。
    PeakOverflow { sum: u32, limit: u16 },
    /// 单个池的规模超出峰值总预算，独占即超限。
    PoolOverflow {
        kind: PoolKind,
        max: u16,
        limit: u16,
    },
}

/// 连接预算（C-04）：resident_max 42、burst_max 52、五池规模表。
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConnectionBudget {
    pub resident_max: u16,
    pub burst_max: u16,
    pub per_pool: [(PoolKind, u16); 5],
}

impl ConnectionBudget {
    /// 标准预算：EP__DB__BUDGET__RESIDENT_MAX=42、EP__DB__BUDGET__PEAK_MAX=52
    /// 与标准五池规模。
    pub fn standard() -> Self {
        Self {
            resident_max: 42,
            burst_max: 52,
            per_pool: [
                (PoolKind::Rw, 20),
                (PoolKind::Ro, 10),
                (PoolKind::Worker, 5),
                (PoolKind::Integ, 5),
                (PoolKind::Ops, 2),
            ],
        }
    }

    /// 由五池规模表构造预算。顺序按 [`PoolKind::ALL`] 对齐。
    pub fn from_specs(resident_max: u16, burst_max: u16, specs: &[PoolSpec; 5]) -> Self {
        let mut per_pool = [(PoolKind::Rw, 0u16); 5];
        for (i, kind) in PoolKind::ALL.iter().enumerate() {
            per_pool[i] = (
                *kind,
                specs
                    .iter()
                    .find(|s| s.kind == *kind)
                    .map_or(0, |s| s.max_connections),
            );
        }
        Self {
            resident_max,
            burst_max,
            per_pool,
        }
    }

    fn total(&self) -> u32 {
        self.per_pool.iter().map(|(_, n)| u32::from(*n)).sum()
    }

    /// 启动求和校验。三类违例全部收集后一次返回，方便启动日志定位。
    pub fn validate(&self) -> Result<(), Vec<BudgetViolation>> {
        let mut problems = Vec::new();
        let sum = self.total();
        if sum > u32::from(self.resident_max) {
            problems.push(BudgetViolation::ResidentOverflow {
                sum,
                limit: self.resident_max,
            });
        }
        if sum > u32::from(self.burst_max) {
            problems.push(BudgetViolation::PeakOverflow {
                sum,
                limit: self.burst_max,
            });
        }
        for (kind, max) in &self.per_pool {
            if u32::from(*max) > u32::from(self.burst_max) {
                problems.push(BudgetViolation::PoolOverflow {
                    kind: *kind,
                    max: *max,
                    limit: self.burst_max,
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
    fn pool_labels_are_the_five_registered_values() {
        let labels: Vec<&str> = PoolKind::ALL.iter().map(|k| k.label()).collect();
        assert_eq!(labels, ["rw", "ro", "worker", "integ", "ops"]);
    }

    #[test]
    fn standard_budget_matches_rw20_ro10_worker5_integ5_ops2() {
        let b = ConnectionBudget::standard();
        assert_eq!(b.resident_max, 42);
        assert_eq!(b.burst_max, 52);
        assert_eq!(b.total(), 42, "标准五池常驻合计恰为 42");
        assert!(b.validate().is_ok(), "标准预算必须通过校验");
    }

    #[test]
    fn resident_overflow_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.resident_max = 41;
        let errs = b.validate().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::ResidentOverflow { sum: 42, limit: 41 })));
    }

    #[test]
    fn peak_overflow_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.burst_max = 40;
        let errs = b.validate().unwrap_err();
        assert!(errs
            .iter()
            .any(|e| matches!(e, BudgetViolation::PeakOverflow { sum: 42, limit: 40 })));
        assert!(
            !errs
                .iter()
                .any(|e| matches!(e, BudgetViolation::PoolOverflow { .. })),
            "单池最大 20 未超 40，不得误报单池越界"
        );
    }

    #[test]
    fn a_single_pool_larger_than_burst_is_rejected() {
        let mut b = ConnectionBudget::standard();
        b.per_pool[0] = (PoolKind::Rw, 60);
        b.resident_max = 100;
        b.burst_max = 52;
        let errs = b.validate().unwrap_err();
        assert!(errs.iter().any(|e| matches!(
            e,
            BudgetViolation::PoolOverflow {
                kind: PoolKind::Rw,
                max: 60,
                limit: 52
            }
        )));
    }

    #[test]
    fn from_specs_aligns_by_kind_regardless_of_input_order() {
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
            PoolSpec {
                kind: PoolKind::Integ,
                max_connections: 5,
            },
        ];
        assert_eq!(
            ConnectionBudget::from_specs(42, 52, &specs),
            ConnectionBudget::standard()
        );
    }
}
