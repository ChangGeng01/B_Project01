//! 抓取目标与聚合。
//!
//! 目标不进配置：阶段 1 计划第 8 节的配置项表没有这一项，而各进程的监听地址
//! 本来就是按进程固定的，再开一个配置键只会多出一处可以和现实不一致的地方。
//!
//! 抓取失败按 `up=0` 标记，绝不静默丢弃——丢掉一个 down 的目标，
//! 仪表盘上看到的就是「一切正常」。

use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use ep_adapter_ipc::{IpcClient, DEFAULT_MAX_FRAME_BYTES, INTEGRATION_ENDPOINT};
use ep_platform_runtime::http::client;
use serde_json::Value;

/// 抓取超时。比 ops 池的 5 秒语句超时更短：抓取拖住聚合等于整块指标不可见。
pub const SCRAPE_TIMEOUT: Duration = Duration::from_secs(2);
/// 单个目标的 Prometheus 文本硬上限。四目标加本地正文即使同时达到上限，
/// 聚合内存仍有确定上界。
pub const MAX_SCRAPE_BODY_BYTES: usize = 1024 * 1024;

/// 本机有 HTTP 指标端点的进程。integration-gateway 按安全拓扑只经 IPC；
/// plugin-host、archive-writer、backup-writer 没有 HTTP 指标端点。
pub const HTTP_TARGETS: [(&str, &str); 3] = [
    ("core-server", "http://127.0.0.1:8080/api/v1/system/metrics"),
    ("job-worker", "http://127.0.0.1:8081/metrics"),
    (
        "portal-gateway",
        "http://127.0.0.1:8090/portal/v1/system/metrics",
    ),
];
pub const TARGET_COUNT: usize = HTTP_TARGETS.len() + 1;
const INTEGRATION_JOB: &str = "integration-gateway";
const METRICS_SNAPSHOT_METHOD: &str = "metrics.snapshot.v1";
const METRICS_CONTENT_TYPE: &str = "text/plain; version=0.0.4; charset=utf-8";

/// 一个目标的抓取结果。
pub struct Scraped {
    pub job: &'static str,
    pub up: bool,
    pub body: String,
}

pub async fn scrape_all() -> Vec<Scraped> {
    let mut out = Vec::with_capacity(TARGET_COUNT);
    for (job, url) in HTTP_TARGETS {
        let scraped = match client::get(url, SCRAPE_TIMEOUT, MAX_SCRAPE_BODY_BYTES).await {
            Ok(r) if r.status == 200 => Scraped {
                job,
                up: true,
                body: r.body,
            },
            Ok(r) => Scraped {
                job,
                up: false,
                body: format!("# 抓取返回 HTTP {}\n", r.status),
            },
            Err(e) => Scraped {
                job,
                up: false,
                body: format!("# 抓取失败：{e}\n"),
            },
        };
        out.push(scraped);
    }
    out.push(scrape_integration_metrics().await);
    out
}

async fn scrape_integration_metrics() -> Scraped {
    let client = IpcClient::new(
        INTEGRATION_ENDPOINT,
        DEFAULT_MAX_FRAME_BYTES,
        SCRAPE_TIMEOUT,
    );
    match client.call(METRICS_SNAPSHOT_METHOD, Value::Null).await {
        Ok(payload) => match decode_metrics_snapshot(payload) {
            Ok(body) => Scraped {
                job: INTEGRATION_JOB,
                up: true,
                body,
            },
            Err(()) => Scraped {
                job: INTEGRATION_JOB,
                up: false,
                body: "# IPC 指标快照格式无效\n".into(),
            },
        },
        Err(_) => Scraped {
            job: INTEGRATION_JOB,
            up: false,
            body: "# IPC 指标快照不可用\n".into(),
        },
    }
}

fn decode_metrics_snapshot(payload: Value) -> Result<String, ()> {
    let Value::Object(mut object) = payload else {
        return Err(());
    };
    if object.len() != 3
        || object.remove("schema_version") != Some(Value::from(1))
        || object.remove("content_type") != Some(Value::from(METRICS_CONTENT_TYPE))
    {
        return Err(());
    }
    let Some(Value::String(body)) = object.remove("body") else {
        return Err(());
    };
    if !object.is_empty() || body.len() > MAX_SCRAPE_BODY_BYTES {
        return Err(());
    }
    Ok(body)
}

#[derive(Clone, Default)]
struct MetricMetadata {
    help: Option<String>,
    kind: Option<String>,
}

struct ParsedSample {
    name: String,
    labels: Vec<(String, String)>,
    tail: String,
}

impl ParsedSample {
    fn series_key(&self) -> (String, Vec<(String, String)>) {
        (self.name.clone(), self.labels.clone())
    }

    fn render(&self, out: &mut String) {
        out.push_str(&self.name);
        out.push('{');
        for (index, (name, value)) in self.labels.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(name);
            out.push_str("=\"");
            out.push_str(&escape_label_value(value));
            out.push('"');
        }
        out.push_str("} ");
        out.push_str(&self.tail);
        out.push('\n');
    }
}

#[derive(Default)]
struct ParsedBody {
    metadata: BTreeMap<String, MetricMetadata>,
    samples: Vec<ParsedSample>,
}

#[derive(Default)]
struct Aggregate {
    metadata: BTreeMap<String, MetricMetadata>,
    series: BTreeSet<(String, Vec<(String, String)>)>,
    samples: Vec<ParsedSample>,
}

impl Aggregate {
    fn accepts(&self, body: &ParsedBody) -> bool {
        let metadata_compatible = body.metadata.iter().all(|(name, candidate)| {
            self.metadata.get(name).is_none_or(|existing| {
                fields_compatible(&existing.help, &candidate.help)
                    && fields_compatible(&existing.kind, &candidate.kind)
            })
        });
        metadata_compatible
            && body
                .samples
                .iter()
                .all(|sample| !self.series.contains(&sample.series_key()))
    }

    fn merge(&mut self, body: ParsedBody) {
        for (name, candidate) in body.metadata {
            let metadata = self.metadata.entry(name).or_default();
            if metadata.help.is_none() {
                metadata.help = candidate.help;
            }
            if metadata.kind.is_none() {
                metadata.kind = candidate.kind;
            }
        }
        for sample in body.samples {
            self.series.insert(sample.series_key());
            self.samples.push(sample);
        }
    }
}

fn fields_compatible(existing: &Option<String>, candidate: &Option<String>) -> bool {
    match (existing, candidate) {
        (Some(left), Some(right)) => left == right,
        _ => true,
    }
}

#[derive(Clone, Copy, Debug)]
struct ParseError;

fn parse_body(body: &str, job: &str) -> Result<ParsedBody, ParseError> {
    let mut parsed = ParsedBody::default();
    let mut series = BTreeSet::new();
    for raw_line in body.lines() {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with('#') {
            parse_comment(line, &mut parsed.metadata)?;
            continue;
        }

        let sample = parse_sample(line, job)?;
        // `up` 属于聚合器本身；下游正文不能覆盖或追加同名 series。
        if sample.name == "up" {
            continue;
        }
        if !series.insert(sample.series_key()) {
            return Err(ParseError);
        }
        parsed.samples.push(sample);
    }
    Ok(parsed)
}

fn parse_comment(
    line: &str,
    metadata: &mut BTreeMap<String, MetricMetadata>,
) -> Result<(), ParseError> {
    let content = line
        .strip_prefix('#')
        .expect("调用方只传注释行")
        .trim_start();
    let (directive, rest) = split_token(content).unwrap_or((content, ""));
    match directive {
        "HELP" => {
            let (name, help) = split_token(rest).ok_or(ParseError)?;
            if !valid_metric_name(name) || !valid_help_text(help) {
                return Err(ParseError);
            }
            if name != "up" {
                set_metadata_field(
                    &mut metadata.entry(name.to_string()).or_default().help,
                    help,
                )?;
            }
        }
        "TYPE" => {
            let mut fields = rest.split_whitespace();
            let name = fields.next().ok_or(ParseError)?;
            let kind = fields.next().ok_or(ParseError)?;
            if fields.next().is_some()
                || !valid_metric_name(name)
                || !matches!(
                    kind,
                    "counter" | "gauge" | "histogram" | "summary" | "untyped"
                )
            {
                return Err(ParseError);
            }
            if name != "up" {
                set_metadata_field(
                    &mut metadata.entry(name.to_string()).or_default().kind,
                    kind,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn set_metadata_field(field: &mut Option<String>, value: &str) -> Result<(), ParseError> {
    match field {
        Some(existing) if existing != value => Err(ParseError),
        Some(_) => Ok(()),
        None => {
            *field = Some(value.to_string());
            Ok(())
        }
    }
}

fn split_token(value: &str) -> Option<(&str, &str)> {
    let value = value.trim_start();
    if value.is_empty() {
        return None;
    }
    let end = value.find(char::is_whitespace).unwrap_or(value.len());
    Some((&value[..end], value[end..].trim_start()))
}

fn parse_sample(line: &str, job: &str) -> Result<ParsedSample, ParseError> {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() && valid_metric_name_byte(bytes[cursor], cursor == 0) {
        cursor += 1;
    }
    if cursor == 0 {
        return Err(ParseError);
    }
    let name = &line[..cursor];
    let mut labels = Vec::new();
    if bytes.get(cursor) == Some(&b'{') {
        let parsed = parse_labels(line, cursor)?;
        labels = parsed.0;
        cursor = parsed.1;
    }
    if !bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        return Err(ParseError);
    }
    let tail = line[cursor..].trim();
    if !valid_sample_tail(tail) {
        return Err(ParseError);
    }

    labels.retain(|(label, _)| label != "job");
    labels.push(("job".to_string(), job.to_string()));
    labels.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    Ok(ParsedSample {
        name: name.to_string(),
        labels,
        tail: tail.to_string(),
    })
}

fn parse_labels(line: &str, open: usize) -> Result<(Vec<(String, String)>, usize), ParseError> {
    let bytes = line.as_bytes();
    let mut cursor = open + 1;
    let mut labels = Vec::new();
    let mut names = BTreeSet::new();
    loop {
        skip_ascii_whitespace(bytes, &mut cursor);
        if bytes.get(cursor) == Some(&b'}') {
            return Ok((labels, cursor + 1));
        }

        let name_start = cursor;
        while cursor < bytes.len() && valid_label_name_byte(bytes[cursor], cursor == name_start) {
            cursor += 1;
        }
        if cursor == name_start {
            return Err(ParseError);
        }
        let name = &line[name_start..cursor];
        if !names.insert(name.to_string()) {
            return Err(ParseError);
        }
        skip_ascii_whitespace(bytes, &mut cursor);
        if bytes.get(cursor) != Some(&b'=') {
            return Err(ParseError);
        }
        cursor += 1;
        skip_ascii_whitespace(bytes, &mut cursor);
        if bytes.get(cursor) != Some(&b'"') {
            return Err(ParseError);
        }
        cursor += 1;

        let mut value = String::new();
        loop {
            match bytes.get(cursor).copied() {
                Some(b'"') => {
                    cursor += 1;
                    break;
                }
                Some(b'\\') => {
                    cursor += 1;
                    match bytes.get(cursor).copied() {
                        Some(b'\\') => value.push('\\'),
                        Some(b'"') => value.push('"'),
                        Some(b'n') => value.push('\n'),
                        _ => return Err(ParseError),
                    }
                    cursor += 1;
                }
                Some(_) => {
                    let ch = line[cursor..].chars().next().ok_or(ParseError)?;
                    value.push(ch);
                    cursor += ch.len_utf8();
                }
                None => return Err(ParseError),
            }
        }
        labels.push((name.to_string(), value));
        skip_ascii_whitespace(bytes, &mut cursor);
        match bytes.get(cursor) {
            Some(b',') => cursor += 1,
            Some(b'}') => return Ok((labels, cursor + 1)),
            _ => return Err(ParseError),
        }
    }
}

fn skip_ascii_whitespace(bytes: &[u8], cursor: &mut usize) {
    while bytes.get(*cursor).is_some_and(u8::is_ascii_whitespace) {
        *cursor += 1;
    }
}

fn valid_metric_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .enumerate()
            .all(|(index, byte)| valid_metric_name_byte(byte, index == 0))
}

fn valid_metric_name_byte(byte: u8, first: bool) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte == b':' || (!first && byte.is_ascii_digit())
}

fn valid_label_name_byte(byte: u8, first: bool) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || (!first && byte.is_ascii_digit())
}

fn valid_sample_tail(tail: &str) -> bool {
    let mut fields = tail.split_whitespace();
    let Some(value) = fields.next() else {
        return false;
    };
    if !valid_prometheus_number(value) {
        return false;
    }
    if fields
        .next()
        .is_some_and(|timestamp| timestamp.parse::<i64>().is_err())
    {
        return false;
    }
    fields.next().is_none()
}

fn valid_help_text(value: &str) -> bool {
    let mut escaped = false;
    for ch in value.chars() {
        if escaped {
            if !matches!(ch, '\\' | 'n') {
                return false;
            }
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if matches!(ch, '\r' | '\n') {
            return false;
        }
    }
    !escaped
}

fn valid_prometheus_number(value: &str) -> bool {
    if matches!(value, "+Inf" | "-Inf" | "NaN") {
        return true;
    }
    let bytes = value.as_bytes();
    let mut cursor = usize::from(bytes.first().is_some_and(|b| matches!(b, b'+' | b'-')));
    let integer_start = cursor;
    while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
        cursor += 1;
    }
    let mut has_digit = cursor > integer_start;
    if bytes.get(cursor) == Some(&b'.') {
        cursor += 1;
        let fraction_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        has_digit |= cursor > fraction_start;
    }
    if !has_digit {
        return false;
    }
    if bytes.get(cursor).is_some_and(|b| matches!(b, b'e' | b'E')) {
        cursor += 1;
        if bytes.get(cursor).is_some_and(|b| matches!(b, b'+' | b'-')) {
            cursor += 1;
        }
        let exponent_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        if cursor == exponent_start {
            return false;
        }
    }
    cursor == bytes.len() && value.parse::<f64>().is_ok_and(|parsed| parsed.is_finite())
}

fn escape_label_value(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

struct Source<'a> {
    job: &'a str,
    advertised_up: bool,
    body: &'a str,
}

/// 汇总为一份 Prometheus 文本。每个来源先独立解析并注入稳定 `job` 标签；
/// 格式错误、重复 series 或元数据冲突只会隔离该来源，并把它的 `up` 置 0。
pub fn render(local: &str, scraped: &[Scraped]) -> String {
    let mut sources = Vec::with_capacity(scraped.len() + 1);
    sources.push(Source {
        job: "ops-agent",
        advertised_up: true,
        body: local,
    });
    sources.extend(scraped.iter().map(|source| Source {
        job: source.job,
        advertised_up: source.up,
        body: &source.body,
    }));

    let mut job_counts = BTreeMap::new();
    for source in &sources {
        *job_counts.entry(source.job).or_insert(0usize) += 1;
    }
    let mut aggregate = Aggregate::default();
    let mut statuses = Vec::with_capacity(sources.len());
    for source in &sources {
        let unique_job = job_counts.get(source.job) == Some(&1);
        let accepted = source.advertised_up
            && source.body.len() <= MAX_SCRAPE_BODY_BYTES
            && unique_job
            && parse_body(source.body, source.job).is_ok_and(|body| {
                if aggregate.accepts(&body) {
                    aggregate.merge(body);
                    true
                } else {
                    false
                }
            });
        statuses.push(accepted);
    }

    let mut out = String::new();
    out.push_str("# HELP up 目标是否可抓取，抓取失败为 0\n# TYPE up gauge\n");
    let mut emitted_jobs = BTreeSet::new();
    for (source, accepted) in sources.iter().zip(statuses) {
        if emitted_jobs.insert(source.job) {
            out.push_str(&format!(
                "up{{job=\"{}\"}} {}\n",
                escape_label_value(source.job),
                u8::from(accepted)
            ));
        }
    }
    for (name, metadata) in aggregate.metadata {
        if let Some(help) = metadata.help {
            out.push_str(&format!("# HELP {name} {help}\n"));
        }
        if let Some(kind) = metadata.kind {
            out.push_str(&format!("# TYPE {name} {kind}\n"));
        }
    }
    for sample in aggregate.samples {
        sample.render(&mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_targets_are_loopback_and_integration_uses_fixed_local_ipc() {
        for (_, url) in HTTP_TARGETS {
            assert!(
                url.starts_with("http://127.0.0.1:"),
                "{url} 必须只在回环上抓"
            );
        }
        assert!(!INTEGRATION_ENDPOINT.starts_with("http"));
        assert_eq!(TARGET_COUNT, HTTP_TARGETS.len() + 1);
    }

    #[test]
    fn ipc_metrics_snapshot_schema_is_exact_and_bounded() {
        let body = decode_metrics_snapshot(serde_json::json!({
            "schema_version": 1,
            "content_type": METRICS_CONTENT_TYPE,
            "body": "ep_build_info 1\n",
        }))
        .unwrap();
        assert_eq!(body, "ep_build_info 1\n");

        for invalid in [
            serde_json::json!({"schema_version": 1, "content_type": METRICS_CONTENT_TYPE}),
            serde_json::json!({"schema_version": 2, "content_type": METRICS_CONTENT_TYPE, "body": ""}),
            serde_json::json!({"schema_version": 1, "content_type": "text/html", "body": ""}),
            serde_json::json!({"schema_version": 1, "content_type": METRICS_CONTENT_TYPE, "body": "", "extra": true}),
        ] {
            assert!(decode_metrics_snapshot(invalid).is_err());
        }
        assert!(decode_metrics_snapshot(serde_json::json!({
            "schema_version": 1,
            "content_type": METRICS_CONTENT_TYPE,
            "body": "x".repeat(MAX_SCRAPE_BODY_BYTES + 1),
        }))
        .is_err());
    }

    #[test]
    fn up_is_one_for_reachable_targets() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "ep_build_info 1\n".into(),
        }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"core-server\"} 1"));
        assert!(
            text.contains("ep_build_info{job=\"core-server\"} 1"),
            "{text}"
        );
    }

    #[test]
    fn shared_metadata_is_emitted_once_and_every_sample_gets_its_source_job() {
        let local = "# HELP ep_shared shared metric\n# TYPE ep_shared gauge\nep_shared 1\n";
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: local.into(),
        }];

        let text = render(local, &scraped);

        assert_eq!(text.matches("# HELP ep_shared ").count(), 1, "{text}");
        assert_eq!(text.matches("# TYPE ep_shared ").count(), 1, "{text}");
        assert!(text.contains("ep_shared{job=\"ops-agent\"} 1"), "{text}");
        assert!(text.contains("ep_shared{job=\"core-server\"} 1"), "{text}");
    }

    #[test]
    fn an_existing_job_label_is_replaced_by_the_stable_target_job() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "ep_requests{job=\"forged\",zone=\"a\"} 2\n".into(),
        }];

        let text = render("", &scraped);

        assert!(
            text.contains("ep_requests{job=\"core-server\",zone=\"a\"} 2"),
            "{text}"
        );
        assert!(!text.contains("forged"), "{text}");
    }

    #[test]
    fn a_malformed_target_is_isolated_and_marked_down() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "ep_valid 1\nep_broken{label=\"unterminated} 2\n".into(),
        }];

        let text = render("", &scraped);

        assert!(text.contains("up{job=\"core-server\"} 0"), "{text}");
        assert!(!text.contains("ep_valid"), "{text}");
        assert!(!text.contains("ep_broken"), "{text}");
    }

    #[test]
    fn duplicate_series_in_one_target_are_rejected_as_a_conflict() {
        let scraped = [Scraped {
            job: "job-worker",
            up: true,
            body: "ep_jobs{queue=\"default\"} 1\nep_jobs{queue=\"default\"} 2\n".into(),
        }];

        let text = render("", &scraped);

        assert!(text.contains("up{job=\"job-worker\"} 0"), "{text}");
        assert!(!text.contains("ep_jobs"), "{text}");
    }

    #[test]
    fn conflicting_metadata_rejects_only_the_later_source() {
        let local = "# HELP ep_shared canonical\n# TYPE ep_shared gauge\nep_shared 1\n";
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "# HELP ep_shared conflicting\n# TYPE ep_shared gauge\nep_shared 2\n".into(),
        }];

        let text = render(local, &scraped);

        assert!(text.contains("up{job=\"ops-agent\"} 1"), "{text}");
        assert!(text.contains("up{job=\"core-server\"} 0"), "{text}");
        assert!(text.contains("ep_shared{job=\"ops-agent\"} 1"), "{text}");
        assert!(!text.contains("ep_shared{job=\"core-server\"}"), "{text}");
        assert!(!text.contains("conflicting"), "{text}");
    }

    #[test]
    fn incoming_up_samples_cannot_override_the_aggregator_up_series() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "# HELP up forged\n# TYPE up counter\nup 99\nep_other 1\n".into(),
        }];

        let text = render("", &scraped);

        assert_eq!(text.matches("# HELP up ").count(), 1, "{text}");
        assert_eq!(text.matches("up{job=\"core-server\"}").count(), 1, "{text}");
        assert!(text.contains("up{job=\"core-server\"} 1"), "{text}");
        assert!(!text.contains("up 99"), "{text}");
        assert!(text.contains("ep_other{job=\"core-server\"} 1"), "{text}");
    }

    #[test]
    fn malformed_local_metrics_do_not_hide_healthy_remote_targets() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "ep_remote 1\n".into(),
        }];

        let text = render("ep_local 1\nnot-a-sample\n", &scraped);

        assert!(text.contains("up{job=\"ops-agent\"} 0"), "{text}");
        assert!(text.contains("up{job=\"core-server\"} 1"), "{text}");
        assert!(!text.contains("ep_local"), "{text}");
        assert!(text.contains("ep_remote{job=\"core-server\"} 1"), "{text}");
    }

    #[test]
    fn invalid_help_escape_is_rejected_instead_of_reemitted() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "# HELP ep_metric invalid\\qescape\n# TYPE ep_metric gauge\nep_metric 1\n".into(),
        }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"core-server\"} 0"), "{text}");
        assert!(!text.contains("invalid\\qescape"), "{text}");
    }

    #[test]
    fn only_canonical_prometheus_special_numbers_are_accepted() {
        for invalid in ["inf", "Inf", "+inf", "nan", "NaN(payload)", "1e999"] {
            let scraped = [Scraped {
                job: "core-server",
                up: true,
                body: format!("ep_metric {invalid}\n"),
            }];
            let text = render("", &scraped);
            assert!(
                text.contains("up{job=\"core-server\"} 0"),
                "{invalid}: {text}"
            );
            assert!(!text.contains("ep_metric{"), "{invalid}: {text}");
        }
        let text = render(
            "ep_positive +Inf\nep_negative -Inf\nep_nan NaN\nep_decimal -1.25e+3\n",
            &[],
        );
        assert!(text.contains("up{job=\"ops-agent\"} 1"), "{text}");
    }

    #[test]
    fn oversized_source_is_isolated_before_parsing() {
        let scraped = [Scraped {
            job: "core-server",
            up: true,
            body: "x".repeat(MAX_SCRAPE_BODY_BYTES + 1),
        }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"core-server\"} 0"), "{text}");
    }

    // 负样例断言的是「抓取失败按 up=0 标记，不静默丢弃」这条规则本身。
    #[test]
    fn a_failed_target_is_marked_down_rather_than_omitted() {
        let scraped = [Scraped {
            job: "job-worker",
            up: false,
            body: "# 抓取失败：连接被拒\n".into(),
        }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"job-worker\"} 0"), "{text}");
        assert!(
            !text.contains("连接被拒"),
            "失败目标的正文不进聚合结果，只留 up=0"
        );
    }

    #[tokio::test]
    async fn scraping_an_unreachable_local_target_yields_down_not_an_error() {
        // 没有任何进程在这些端口上时，抓取必须给出 up=0 的结果而不是中止聚合。
        let scraped = scrape_all().await;
        assert_eq!(scraped.len(), TARGET_COUNT);
    }
}
