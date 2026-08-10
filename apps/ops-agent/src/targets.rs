//! 抓取目标与聚合。
//!
//! 目标不进配置：阶段 1 计划第 8 节的配置项表没有这一项，而各进程的监听地址
//! 本来就是按进程固定的，再开一个配置键只会多出一处可以和现实不一致的地方。
//!
//! 抓取失败按 `up=0` 标记，绝不静默丢弃——丢掉一个 down 的目标，
//! 仪表盘上看到的就是「一切正常」。

use std::time::Duration;

use ep_platform_runtime::http::client;

/// 抓取超时。比 ops 池的 5 秒语句超时更短：抓取拖住聚合等于整块指标不可见。
pub const SCRAPE_TIMEOUT: Duration = Duration::from_secs(2);

/// 本机有指标端点的进程。plugin-host、archive-writer、backup-writer 三个
/// 进程按第 6.2 节无监听，因此没有指标端点，不在目标之列。
pub const TARGETS: [(&str, &str); 4] = [
    ("core-server", "http://127.0.0.1:8080/api/v1/system/metrics"),
    ("job-worker", "http://127.0.0.1:8081/metrics"),
    ("integration-gateway", "http://127.0.0.1:8082/metrics"),
    ("portal-gateway", "http://127.0.0.1:8090/portal/v1/system/metrics"),
];

/// 一个目标的抓取结果。
pub struct Scraped {
    pub job: &'static str,
    pub up: bool,
    pub body: String,
}

pub async fn scrape_all() -> Vec<Scraped> {
    let mut out = Vec::with_capacity(TARGETS.len());
    for (job, url) in TARGETS {
        let scraped = match client::get(url, SCRAPE_TIMEOUT).await {
            Ok(r) if r.status == 200 => Scraped { job, up: true, body: r.body },
            Ok(r) => Scraped { job, up: false, body: format!("# 抓取返回 HTTP {}\n", r.status) },
            Err(e) => Scraped { job, up: false, body: format!("# 抓取失败：{e}\n") },
        };
        out.push(scraped);
    }
    out
}

/// 汇总为一份 Prometheus 文本。每个目标一行 `up`，抓到的正文原样附在其后。
pub fn render(local: &str, scraped: &[Scraped]) -> String {
    let mut out = String::new();
    out.push_str("# HELP up 目标是否可抓取，抓取失败为 0\n# TYPE up gauge\n");
    for s in scraped {
        out.push_str(&format!("up{{job=\"{}\"}} {}\n", s.job, u8::from(s.up)));
    }
    out.push_str("up{job=\"ops-agent\"} 1\n");
    out.push_str(local);
    for s in scraped.iter().filter(|s| s.up) {
        out.push_str(&s.body);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_target_is_a_loopback_address() {
        for (_, url) in TARGETS {
            assert!(url.starts_with("http://127.0.0.1:"), "{url} 必须只在回环上抓");
        }
    }

    #[test]
    fn up_is_one_for_reachable_targets() {
        let scraped = [Scraped { job: "core-server", up: true, body: "ep_build_info 1\n".into() }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"core-server\"} 1"));
        assert!(text.contains("ep_build_info 1"));
    }

    // 负样例断言的是「抓取失败按 up=0 标记，不静默丢弃」这条规则本身。
    #[test]
    fn a_failed_target_is_marked_down_rather_than_omitted() {
        let scraped = [Scraped { job: "job-worker", up: false, body: "# 抓取失败：连接被拒\n".into() }];
        let text = render("", &scraped);
        assert!(text.contains("up{job=\"job-worker\"} 0"), "{text}");
        assert!(!text.contains("连接被拒"), "失败目标的正文不进聚合结果，只留 up=0");
    }

    #[tokio::test]
    async fn scraping_an_unreachable_local_target_yields_down_not_an_error() {
        // 没有任何进程在这些端口上时，抓取必须给出 up=0 的结果而不是中止聚合。
        let scraped = scrape_all().await;
        assert_eq!(scraped.len(), TARGETS.len());
    }
}
