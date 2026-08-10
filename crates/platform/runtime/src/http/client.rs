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

/// 对 `http://host:port/path` 发一次 GET。超时由调用方给，不设隐含默认值。
pub async fn get(url: &str, timeout: Duration) -> Result<FetchResponse, FetchError> {
    tokio::time::timeout(timeout, get_inner(url))
        .await
        .map_err(|_| FetchError(format!("GET {url} 超时（{} 毫秒）", timeout.as_millis())))?
}

async fn get_inner(url: &str) -> Result<FetchResponse, FetchError> {
    let uri: hyper::Uri = url.parse().map_err(|e| FetchError(format!("地址 {url} 不合法：{e}")))?;
    if uri.scheme_str() != Some("http") {
        return Err(FetchError(format!("只支持回环上的明文 http，实际 {url}")));
    }
    let host = uri.host().ok_or_else(|| FetchError(format!("地址 {url} 缺主机段")))?;
    let port = uri.port_u16().unwrap_or(80);
    let authority = format!("{host}:{port}");

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

    let resp = sender.send_request(req).await.map_err(|e| FetchError(format!("请求 {url} 失败：{e}")))?;
    let status = resp.status().as_u16();
    let bytes = resp
        .into_body()
        .collect()
        .await
        .map_err(|e| FetchError(format!("读取 {url} 响应体失败：{e}")))?
        .to_bytes();
    Ok(FetchResponse { status, body: String::from_utf8_lossy(&bytes).to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn non_http_scheme_is_rejected() {
        let e = get("https://127.0.0.1:8080/x", Duration::from_millis(100)).await.unwrap_err();
        assert!(e.0.contains("只支持回环上的明文 http"));
    }

    // 负样例断言的是「抓取失败必须报错」这条规则本身：连不上时不得返回一个
    // 空响应假装成功，否则 ops-agent 会把 down 的目标标成 up。
    #[tokio::test]
    async fn unreachable_target_is_an_error_not_an_empty_success() {
        // 端口 1 在回环上不会有监听者。
        let e = get("http://127.0.0.1:1/metrics", Duration::from_millis(500)).await.unwrap_err();
        assert!(e.0.contains("失败") || e.0.contains("超时"), "{}", e.0);
    }
}
