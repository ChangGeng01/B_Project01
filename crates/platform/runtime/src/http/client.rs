//! 回环 HTTP 客户端。
//!
//! 只服务两个用途：portal-gateway 探测 core-server 的健康端点，
//! ops-agent 抓取本机各进程的指标端点。因此不做连接池、不做重定向、
//! 不做 TLS——回环上这三样都是没有被测对象的复杂度。

use std::time::Duration;

use http_body_util::BodyExt;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FetchError(pub String);

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for FetchError {}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FetchResponse {
    pub status: u16,
    pub body: String,
}

/// 对 `http://host:port/path` 发一次有界 GET。超时和响应体硬上限都由调用方给，
/// Content-Length 与流式累计任一超限即失败，不能在超时窗口内无限分配内存。
pub async fn get(
    url: &str,
    timeout: Duration,
    maximum_body_bytes: usize,
) -> Result<FetchResponse, FetchError> {
    if maximum_body_bytes == 0 {
        return Err(FetchError("响应体上限必须大于零".into()));
    }
    tokio::time::timeout(timeout, get_inner(url, maximum_body_bytes))
        .await
        .map_err(|_| FetchError(format!("GET {url} 超时（{} 毫秒）", timeout.as_millis())))?
}

async fn get_inner(url: &str, maximum_body_bytes: usize) -> Result<FetchResponse, FetchError> {
    let uri: hyper::Uri = url
        .parse()
        .map_err(|e| FetchError(format!("地址 {url} 不合法：{e}")))?;
    if uri.scheme_str() != Some("http") {
        return Err(FetchError(format!("只支持回环上的明文 http，实际 {url}")));
    }
    let host = uri
        .host()
        .ok_or_else(|| FetchError(format!("地址 {url} 缺主机段")))?;
    let host_ip: std::net::IpAddr = host
        .parse()
        .map_err(|_| FetchError(format!("地址 {url} 的主机必须是回环 IP 字面量")))?;
    if !host_ip.is_loopback() {
        return Err(FetchError(format!("地址 {url} 不是回环目标")));
    }
    let port = uri.port_u16().unwrap_or(80);
    let authority = std::net::SocketAddr::new(host_ip, port).to_string();

    let stream = TcpStream::connect(&authority)
        .await
        .map_err(|e| FetchError(format!("连接 {authority} 失败：{e}")))?;
    let (mut sender, conn) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
        .await
        .map_err(|e| FetchError(format!("与 {authority} 握手失败：{e}")))?;
    // 连接驱动任务的结束是正常收尾，其错误不覆盖请求本身的结论。
    tokio::spawn(async move {
        let _ = conn.await;
    });

    let path = uri.path_and_query().map(|p| p.as_str()).unwrap_or("/");
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.clone())
        .body(String::new())
        .map_err(|e| FetchError(format!("构造请求失败：{e}")))?;

    let resp = sender
        .send_request(req)
        .await
        .map_err(|e| FetchError(format!("请求 {url} 失败：{e}")))?;
    let status = resp.status().as_u16();
    if let Some(content_length) = resp.headers().get(hyper::header::CONTENT_LENGTH) {
        let content_length = content_length
            .to_str()
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .ok_or_else(|| FetchError(format!("读取 {url} 响应长度失败：Content-Length 非法")))?;
        if content_length > maximum_body_bytes as u64 {
            return Err(body_too_large(url, maximum_body_bytes));
        }
    }
    let mut body = resp.into_body();
    let mut bytes = Vec::with_capacity(maximum_body_bytes.min(16 * 1024));
    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|e| FetchError(format!("读取 {url} 响应体失败：{e}")))?;
        if let Ok(data) = frame.into_data() {
            extend_bounded(&mut bytes, &data, maximum_body_bytes)
                .map_err(|_| body_too_large(url, maximum_body_bytes))?;
        }
    }
    let body = String::from_utf8(bytes)
        .map_err(|_| FetchError(format!("读取 {url} 响应体失败：不是合法 UTF-8")))?;
    Ok(FetchResponse { status, body })
}

fn body_too_large(url: &str, maximum_body_bytes: usize) -> FetchError {
    FetchError(format!(
        "读取 {url} 响应体失败：超过 {maximum_body_bytes} bytes 硬上限"
    ))
}

fn extend_bounded(target: &mut Vec<u8>, chunk: &[u8], limit: usize) -> Result<(), ()> {
    let next = target.len().checked_add(chunk.len()).ok_or(())?;
    if next > limit {
        return Err(());
    }
    target.extend_from_slice(chunk);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn non_http_scheme_is_rejected() {
        let e = get("https://127.0.0.1:8080/x", Duration::from_millis(100), 1024)
            .await
            .unwrap_err();
        assert!(e.0.contains("只支持回环上的明文 http"));
    }

    // 负样例断言的是「抓取失败必须报错」这条规则本身：连不上时不得返回一个
    // 空响应假装成功，否则 ops-agent 会把 down 的目标标成 up。
    #[tokio::test]
    async fn unreachable_target_is_an_error_not_an_empty_success() {
        // 端口 1 在回环上不会有监听者。
        let e = get(
            "http://127.0.0.1:1/metrics",
            Duration::from_millis(500),
            1024,
        )
        .await
        .unwrap_err();
        assert!(e.0.contains("失败") || e.0.contains("超时"), "{}", e.0);
    }

    #[test]
    fn streaming_body_limit_is_checked_before_growth() {
        let mut body = b"1234".to_vec();
        assert!(extend_bounded(&mut body, b"56", 6).is_ok());
        assert_eq!(body, b"123456");
        assert!(extend_bounded(&mut body, b"7", 6).is_err());
        assert_eq!(body, b"123456", "超限块不能部分写入");
        assert!(extend_bounded(&mut Vec::new(), b"x", 0).is_err());
    }

    #[tokio::test]
    async fn zero_body_limit_is_rejected_before_network_access() {
        let e = get("http://127.0.0.1:1/metrics", Duration::from_millis(500), 0)
            .await
            .unwrap_err();
        assert!(e.0.contains("上限必须大于零"));
    }

    #[tokio::test]
    async fn non_loopback_and_dns_targets_are_rejected_before_connect() {
        for url in ["http://192.0.2.1/metrics", "http://localhost:8080/metrics"] {
            let e = get(url, Duration::from_millis(100), 1024)
                .await
                .unwrap_err();
            assert!(e.0.contains("回环"), "{url}: {e}");
        }
    }
}
