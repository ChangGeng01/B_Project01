//! 直方图的桶累计。
//!
//! 桶边界随指标定义冻结，改桶等同于改指标，因此这里不提供任何运行期改桶入口。

/// 一条直方图样本序列的累计状态。
#[derive(Clone, Debug)]
pub struct HistogramState {
    /// 与 `MetricDef::buckets` 逐位对应的累计计数，语义为「小于等于该上界」。
    counts: Vec<u64>,
    sum: f64,
    count: u64,
}

impl HistogramState {
    pub fn new(buckets: usize) -> Self {
        Self { counts: vec![0; buckets], sum: 0.0, count: 0 }
    }

    /// 观测一个取值。上界之外的取值只进 `+Inf`，由 `count` 承载。
    pub fn observe(&mut self, bounds: &[f64], value: f64) {
        for (slot, upper) in self.counts.iter_mut().zip(bounds) {
            if value <= *upper {
                *slot += 1;
            }
        }
        self.sum += value;
        self.count += 1;
    }

    pub fn cumulative(&self) -> &[u64] {
        &self.counts
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOUNDS: [f64; 3] = [0.1, 0.5, 1.0];

    #[test]
    fn observation_lands_in_every_bucket_at_or_above_it() {
        let mut h = HistogramState::new(BOUNDS.len());
        h.observe(&BOUNDS, 0.2);
        assert_eq!(h.cumulative(), [0, 1, 1]);
        assert_eq!(h.count(), 1);
    }

    #[test]
    fn value_above_last_bound_counts_only_in_inf() {
        let mut h = HistogramState::new(BOUNDS.len());
        h.observe(&BOUNDS, 42.0);
        assert_eq!(h.cumulative(), [0, 0, 0]);
        assert_eq!(h.count(), 1, "+Inf 由 count 承载，超上界的观测不得丢失");
        assert!((h.sum() - 42.0).abs() < f64::EPSILON);
    }
}
