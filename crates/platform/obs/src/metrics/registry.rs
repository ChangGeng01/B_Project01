//! 六个指标的注册落点，出处是阶段 1 计划第 13 节新增决定五与退出条件 24。
//!
//! 标签基数纪律做成注册期判定而不是评审约定：`user_id`、`doc_no`、`trace_id`
//! 三个标签名一旦进入定义表即由测试拦下，运行期再传未登记的标签名一律返回
//! 错误而不是静默接受——静默接受会让时序库在上线后才炸。

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Mutex;

use super::histogram::HistogramState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MetricKind {
    Gauge,
    Counter,
    Histogram,
}

impl MetricKind {
    const fn as_str(self) -> &'static str {
        match self {
            MetricKind::Gauge => "gauge",
            MetricKind::Counter => "counter",
            MetricKind::Histogram => "histogram",
        }
    }
}

/// 一项指标的登记。四列与 `docs/metrics-catalog.md` 第 3 节登记表一一对应。
#[derive(Clone, Copy, Debug)]
pub struct MetricDef {
    pub name: &'static str,
    pub kind: MetricKind,
    pub labels: &'static [&'static str],
    pub help: &'static str,
    /// 只有 Histogram 用得上；其余取空切片。
    pub buckets: &'static [f64],
}

/// 技术基线第 9.2 节的十个桶，逐值一致。
pub const HTTP_BUCKETS: [f64; 10] = [0.05, 0.1, 0.25, 0.5, 1.0, 2.0, 3.0, 5.0, 10.0, 30.0];

/// 禁止作为标签的三项，取值集合随业务量无上限增长。
pub const FORBIDDEN_LABELS: [&str; 3] = ["user_id", "doc_no", "trace_id"];

pub const REGISTERED: [MetricDef; 6] = [
    MetricDef {
        name: "ep_build_info",
        kind: MetricKind::Gauge,
        labels: &["version", "git_commit"],
        help: "构建标识，取值恒为 1，信息全在标签上",
        buckets: &[],
    },
    MetricDef {
        name: "ep_selfcheck_pending_items",
        kind: MetricKind::Gauge,
        labels: &["process"],
        help: "该进程启动自检报告中 Pending 项的条数",
        buckets: &[],
    },
    MetricDef {
        name: "ep_db_pool_connections",
        kind: MetricKind::Gauge,
        labels: &["pool"],
        help: "各具名连接池的当前连接数，阶段 1 只注册不填充",
        buckets: &[],
    },
    MetricDef {
        name: "ep_db_statement_duration_seconds",
        kind: MetricKind::Histogram,
        labels: &["pool", "statement_kind"],
        help: "单条 SQL 的执行时长分布，阶段 1 只注册不填充",
        // 桶由阶段 2 在填充时定死并回写指标目录，本阶段不擅自取值。
        buckets: &[],
    },
    MetricDef {
        name: "ep_http_request_duration_seconds",
        kind: MetricKind::Histogram,
        labels: &["route", "method", "status_class", "client"],
        help: "HTTP 请求时长分布，在中间件栈中填充",
        buckets: &HTTP_BUCKETS,
    },
    MetricDef {
        name: "ep_quota_throttled_total",
        kind: MetricKind::Counter,
        labels: &["route"],
        help: "被并发闸门拒绝的请求数，在闸门中填充",
        buckets: &[],
    },
];

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MetricError {
    UnknownMetric(String),
    KindMismatch { name: &'static str, want: &'static str },
    LabelSetMismatch { name: &'static str, detail: String },
    NoBuckets(&'static str),
}

impl fmt::Display for MetricError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricError::UnknownMetric(n) => write!(f, "指标 {n} 未在登记表中，指标名必须先登记后使用"),
            MetricError::KindMismatch { name, want } => write!(f, "指标 {name} 的类型是 {want}"),
            MetricError::LabelSetMismatch { name, detail } => write!(f, "指标 {name} 的标签集不符：{detail}"),
            MetricError::NoBuckets(n) => write!(f, "指标 {n} 尚未定死桶边界，本阶段不接受观测"),
        }
    }
}

impl std::error::Error for MetricError {}

enum Sample {
    Scalar(f64),
    Hist(HistogramState),
}

type SampleKey = (&'static str, Vec<String>);

/// 进程级指标注册表。一个进程一个实例，在装配时建好后只读地共享。
pub struct MetricsRegistry {
    samples: Mutex<BTreeMap<SampleKey, Sample>>,
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self { samples: Mutex::new(BTreeMap::new()) }
    }

    pub fn definition(name: &str) -> Option<&'static MetricDef> {
        REGISTERED.iter().find(|d| d.name == name)
    }

    fn resolve(name: &str, kind: MetricKind) -> Result<&'static MetricDef, MetricError> {
        let def = Self::definition(name).ok_or_else(|| MetricError::UnknownMetric(name.to_string()))?;
        if def.kind != kind {
            return Err(MetricError::KindMismatch { name: def.name, want: def.kind.as_str() });
        }
        Ok(def)
    }

    /// 标签按定义表的顺序取值，缺一个或多一个都算不符。
    fn label_values(def: &'static MetricDef, given: &[(&str, &str)]) -> Result<Vec<String>, MetricError> {
        if given.len() != def.labels.len() {
            return Err(MetricError::LabelSetMismatch {
                name: def.name,
                detail: format!("登记 {} 个标签，实传 {} 个", def.labels.len(), given.len()),
            });
        }
        def.labels
            .iter()
            .map(|want| {
                given
                    .iter()
                    .find(|(k, _)| k == want)
                    .map(|(_, v)| (*v).to_string())
                    .ok_or_else(|| MetricError::LabelSetMismatch {
                        name: def.name,
                        detail: format!("缺标签 {want}"),
                    })
            })
            .collect()
    }

    pub fn set_gauge(&self, name: &str, labels: &[(&str, &str)], value: f64) -> Result<(), MetricError> {
        let def = Self::resolve(name, MetricKind::Gauge)?;
        let key = (def.name, Self::label_values(def, labels)?);
        let mut guard = self.lock();
        guard.insert(key, Sample::Scalar(value));
        Ok(())
    }

    pub fn inc_counter(&self, name: &str, labels: &[(&str, &str)], by: f64) -> Result<(), MetricError> {
        let def = Self::resolve(name, MetricKind::Counter)?;
        let key = (def.name, Self::label_values(def, labels)?);
        let mut guard = self.lock();
        match guard.entry(key).or_insert(Sample::Scalar(0.0)) {
            Sample::Scalar(v) => *v += by,
            Sample::Hist(_) => unreachable!("类型已由 resolve 判定"),
        }
        Ok(())
    }

    pub fn observe(&self, name: &str, labels: &[(&str, &str)], seconds: f64) -> Result<(), MetricError> {
        let def = Self::resolve(name, MetricKind::Histogram)?;
        if def.buckets.is_empty() {
            return Err(MetricError::NoBuckets(def.name));
        }
        let key = (def.name, Self::label_values(def, labels)?);
        let mut guard = self.lock();
        let entry = guard.entry(key).or_insert_with(|| Sample::Hist(HistogramState::new(def.buckets.len())));
        match entry {
            Sample::Hist(h) => h.observe(def.buckets, seconds),
            Sample::Scalar(_) => unreachable!("类型已由 resolve 判定"),
        }
        Ok(())
    }

    /// 中毒的锁不吞：指标写入方 panic 过一次，后续取值不可信，这里恢复内层值
    /// 并继续，理由是指标不该把进程带死，但该事实由 panic 日志本身留痕。
    fn lock(&self) -> std::sync::MutexGuard<'_, BTreeMap<SampleKey, Sample>> {
        self.samples.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Prometheus 文本格式。无样本的指标同样输出 HELP 与 TYPE 两行，
    /// 因此「指标名存在」这条判据不依赖有没有非零样本（裁定 C-23）。
    pub fn encode_text(&self) -> String {
        let guard = self.lock();
        let mut out = String::new();
        for def in REGISTERED.iter() {
            out.push_str(&format!("# HELP {} {}\n# TYPE {} {}\n", def.name, def.help, def.name, def.kind.as_str()));
            for ((name, values), sample) in guard.iter().filter(|((n, _), _)| *n == def.name) {
                let labels = render_labels(def.labels, values);
                match sample {
                    Sample::Scalar(v) => out.push_str(&format!("{name}{labels} {v}\n")),
                    Sample::Hist(h) => encode_histogram(&mut out, name, def, values, h),
                }
            }
        }
        out
    }
}

fn render_labels(names: &[&str], values: &[String]) -> String {
    if names.is_empty() {
        return String::new();
    }
    let pairs: Vec<String> = names
        .iter()
        .zip(values)
        .map(|(k, v)| format!("{k}=\"{}\"", v.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect();
    format!("{{{}}}", pairs.join(","))
}

fn encode_histogram(out: &mut String, name: &str, def: &MetricDef, values: &[String], h: &HistogramState) {
    for (upper, count) in def.buckets.iter().zip(h.cumulative()) {
        let mut names: Vec<&str> = def.labels.to_vec();
        names.push("le");
        let mut vals: Vec<String> = values.to_vec();
        vals.push(format!("{upper}"));
        out.push_str(&format!("{name}_bucket{} {count}\n", render_labels(&names, &vals)));
    }
    let mut names: Vec<&str> = def.labels.to_vec();
    names.push("le");
    let mut vals: Vec<String> = values.to_vec();
    vals.push("+Inf".to_string());
    out.push_str(&format!("{name}_bucket{} {}\n", render_labels(&names, &vals), h.count()));
    let base = render_labels(def.labels, values);
    out.push_str(&format!("{name}_sum{base} {}\n", h.sum()));
    out.push_str(&format!("{name}_count{base} {}\n", h.count()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_registered_names_are_unique() {
        assert_eq!(REGISTERED.len(), 6, "阶段 1 登记且只登记六项");
        let mut names: Vec<&str> = REGISTERED.iter().map(|d| d.name).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "指标名不得重复登记");
    }

    #[test]
    fn no_forbidden_label_in_any_definition() {
        for def in REGISTERED.iter() {
            for label in def.labels {
                assert!(!FORBIDDEN_LABELS.contains(label), "{} 上出现了禁止标签 {label}", def.name);
            }
        }
    }

    #[test]
    fn every_registered_name_appears_in_text_output() {
        let text = MetricsRegistry::new().encode_text();
        for def in REGISTERED.iter() {
            assert!(text.contains(&format!("# TYPE {} ", def.name)), "{} 未出现在指标端点上", def.name);
        }
    }

    // 负样例：断言的是注册表这条规则本身——未登记的指标名不得被接受。
    #[test]
    fn unregistered_name_is_rejected() {
        let reg = MetricsRegistry::new();
        let err = reg.inc_counter("ep_db_retries_total", &[("pool", "rw")], 1.0).unwrap_err();
        assert_eq!(err, MetricError::UnknownMetric("ep_db_retries_total".into()));
    }

    #[test]
    fn wrong_kind_is_rejected() {
        let reg = MetricsRegistry::new();
        let err = reg.set_gauge("ep_quota_throttled_total", &[("route", "/x")], 1.0).unwrap_err();
        assert_eq!(err, MetricError::KindMismatch { name: "ep_quota_throttled_total", want: "counter" });
    }

    #[test]
    fn label_set_mismatch_is_rejected() {
        let reg = MetricsRegistry::new();
        let err = reg.set_gauge("ep_selfcheck_pending_items", &[("proc", "core-server")], 3.0).unwrap_err();
        assert!(matches!(err, MetricError::LabelSetMismatch { name: "ep_selfcheck_pending_items", .. }));
    }

    #[test]
    fn unfilled_histogram_refuses_observation_instead_of_inventing_buckets() {
        let reg = MetricsRegistry::new();
        let err = reg
            .observe("ep_db_statement_duration_seconds", &[("pool", "rw"), ("statement_kind", "select")], 0.1)
            .unwrap_err();
        assert_eq!(err, MetricError::NoBuckets("ep_db_statement_duration_seconds"));
    }

    #[test]
    fn http_histogram_encodes_buckets_sum_and_count() {
        let reg = MetricsRegistry::new();
        let labels = [("route", "/api/v1/system/health"), ("method", "GET"), ("status_class", "2xx"), ("client", "ops")];
        reg.observe("ep_http_request_duration_seconds", &labels, 0.2).expect("观测应被接受");
        let text = reg.encode_text();
        assert!(text.contains("ep_http_request_duration_seconds_bucket"));
        assert!(text.contains("le=\"+Inf\""));
        assert!(text.contains("ep_http_request_duration_seconds_count"));
        assert!(text.contains("route=\"/api/v1/system/health\""));
    }
}
