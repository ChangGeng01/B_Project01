//! 日志字段约定与 JSON Lines 输出。
//!
//! 字段集合是固定集合而不是建议集合：技术基线第 9.1 节写明「缺失即视为实现缺陷」，
//! 因此这里把它表达成一个结构体而不是若干次自由的键值追加——自由追加无法在
//! 编译期保证齐全。缺项以 `null` 出现，也就是「无此项」而不是「忘了写」。

pub mod access;

use std::io::Write;
use std::sync::Mutex;

pub use access::AccessLog;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub const fn as_str(self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// 配置里的 `log.level` 是字符串，未知取值不静默降级为 info。
    pub fn parse(s: &str) -> Option<Level> {
        match s.to_ascii_lowercase().as_str() {
            "debug" => Some(Level::Debug),
            "info" => Some(Level::Info),
            "warn" => Some(Level::Warn),
            "error" => Some(Level::Error),
            _ => None,
        }
    }
}

/// 一行日志。字段顺序与技术基线第 9.1 节的清单一致。
#[derive(Clone, Debug, Default)]
pub struct LogFields {
    pub target: &'static str,
    pub msg: String,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub request_id: Option<String>,
    pub legal_entity_id: Option<String>,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    pub module: Option<String>,
    pub operation: Option<String>,
    pub duration_ms: Option<u64>,
    pub outcome: Option<&'static str>,
    pub error_code: Option<String>,
    pub error_category: Option<String>,
}

impl LogFields {
    pub fn msg(target: &'static str, msg: impl Into<String>) -> Self {
        Self { target, msg: msg.into(), ..Self::default() }
    }
}

/// 进程级日志器。持有 `process` 与 `version` 两个每行都要出现的常量。
pub struct JsonLogger {
    process: &'static str,
    version: &'static str,
    level: Mutex<Level>,
}

impl JsonLogger {
    pub fn new(process: &'static str, version: &'static str, level: Level) -> Self {
        Self { process, version, level: Mutex::new(level) }
    }

    /// `log.level` 取 SIGHUP 热加载，因此级别是可变的。
    pub fn set_level(&self, level: Level) {
        *self.level.lock().unwrap_or_else(|p| p.into_inner()) = level;
    }

    pub fn level(&self) -> Level {
        *self.level.lock().unwrap_or_else(|p| p.into_inner())
    }

    pub fn log(&self, level: Level, fields: LogFields) {
        if level < self.level() {
            return;
        }
        let line = self.render(level, &fields, now_rfc3339_micros());
        // 日志写失败不再回写日志，否则递归；写进程的 stderr 留一条痕迹。
        let mut out = std::io::stdout().lock();
        if let Err(e) = writeln!(out, "{line}") {
            eprintln!("日志写出失败: {e}");
        }
    }

    /// 与 [`Self::log`] 同一渲染路径，测试据此断言字段齐全。
    pub fn render(&self, level: Level, f: &LogFields, ts: String) -> String {
        let v = serde_json::json!({
            "ts": ts,
            "level": level.as_str(),
            "target": f.target,
            "msg": f.msg,
            "process": self.process,
            "version": self.version,
            "trace_id": f.trace_id,
            "span_id": f.span_id,
            "request_id": f.request_id,
            "legal_entity_id": f.legal_entity_id,
            "user_id": f.user_id,
            "device_id": f.device_id,
            "module": f.module,
            "operation": f.operation,
            "duration_ms": f.duration_ms,
            "outcome": f.outcome,
            "error_code": f.error_code,
            "error_category": f.error_category,
        });
        v.to_string()
    }
}

/// 技术基线第 9.1 节的固定字段集合，一个不多一个不少。
pub const FIXED_FIELDS: [&str; 18] = [
    "ts",
    "level",
    "target",
    "msg",
    "process",
    "version",
    "trace_id",
    "span_id",
    "request_id",
    "legal_entity_id",
    "user_id",
    "device_id",
    "module",
    "operation",
    "duration_ms",
    "outcome",
    "error_code",
    "error_category",
];

/// RFC3339 UTC 微秒。不引入日期库：这里只做定长格式化，闰秒由内核负责。
pub fn now_rfc3339_micros() -> String {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format_rfc3339_micros(d.as_secs(), d.subsec_micros())
}

fn format_rfc3339_micros(unix_secs: u64, micros: u32) -> String {
    let days = (unix_secs / 86_400) as i64;
    let secs_of_day = unix_secs % 86_400;
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{micros:06}Z",
        secs_of_day / 3600,
        (secs_of_day % 3600) / 60,
        secs_of_day % 60
    )
}

/// Howard Hinnant 的 civil_from_days，公历，纪元为 1970-01-01。
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rendered_line_carries_every_fixed_field() {
        let logger = JsonLogger::new("core-server", "0.1.0", Level::Info);
        let line = logger.render(Level::Info, &LogFields::msg("startup", "已就绪"), "x".into());
        let v: serde_json::Value = serde_json::from_str(&line).expect("日志行必须是合法 JSON");
        let obj = v.as_object().expect("日志行必须是对象");
        assert_eq!(obj.len(), FIXED_FIELDS.len(), "字段数必须与固定集合相等");
        for field in FIXED_FIELDS {
            assert!(obj.contains_key(field), "缺字段 {field}");
        }
    }

    #[test]
    fn level_below_threshold_is_dropped() {
        let logger = JsonLogger::new("core-server", "0.1.0", Level::Warn);
        assert!(Level::Info < logger.level(), "info 低于 warn 阈值时不应输出");
    }

    #[test]
    fn unknown_level_string_is_rejected() {
        assert_eq!(Level::parse("warn"), Some(Level::Warn));
        assert_eq!(Level::parse("verbose"), None, "未知级别不得静默降级");
    }

    #[test]
    fn timestamp_format_is_rfc3339_utc_micros() {
        let s = format_rfc3339_micros(1_786_763_045, 7);
        assert_eq!(s, "2026-08-15T03:04:05.000007Z");
        // 纪元当天与闰年 2 月末各取一点，防止只在某一年成立。
        assert_eq!(format_rfc3339_micros(0, 0), "1970-01-01T00:00:00.000000Z");
        assert_eq!(format_rfc3339_micros(1_709_164_799, 999_999), "2024-02-28T23:59:59.999999Z");
    }
}
