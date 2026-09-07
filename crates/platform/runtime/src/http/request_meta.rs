//! 服务端拥有的请求元数据与可信代理解析。
//!
//! 外部请求可以携带 `X-Forwarded-For`，但只有直接 TCP 对端命中显式
//! 可信代理网段时才解析；否则始终以 socket peer 为准。内部 request/trace
//! 标识每次请求重新生成，不接受任何入站 `x-ep-*` 值。

use std::net::{IpAddr, Ipv4Addr};

use axum::extract::{ConnectInfo, Request};
use ep_foundation::security::context::{RequestId, TraceId};
use serde::{de::Error as _, Deserialize, Deserializer};

const MAX_FORWARDED_BYTES: usize = 1024;
const MAX_FORWARDED_HOPS: usize = 16;

/// 一个规范化 IP/CIDR。无 `/prefix` 的输入按单主机处理。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TrustedProxyNet {
    network: IpAddr,
    prefix: u8,
}

impl TrustedProxyNet {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let (addr_raw, prefix_raw) = raw.split_once('/').unwrap_or((raw, ""));
        let addr: IpAddr = addr_raw
            .parse()
            .map_err(|_| format!("可信代理网段不是 IP/CIDR：{raw}"))?;
        let max = match addr {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        let prefix = if prefix_raw.is_empty() {
            max
        } else {
            prefix_raw
                .parse::<u8>()
                .map_err(|_| format!("可信代理前缀不是数字：{raw}"))?
        };
        if prefix == 0 {
            return Err(format!("可信代理网段不得覆盖整个地址空间：{raw}"));
        }
        if prefix > max {
            return Err(format!("可信代理前缀超出地址宽度：{raw}"));
        }
        Ok(Self {
            network: masked(addr, prefix),
            prefix,
        })
    }

    pub fn contains(self, addr: IpAddr) -> bool {
        match (self.network, addr) {
            (IpAddr::V4(_), IpAddr::V4(_)) | (IpAddr::V6(_), IpAddr::V6(_)) => {
                masked(addr, self.prefix) == self.network
            }
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for TrustedProxyNet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(D::Error::custom)
    }
}

fn masked(addr: IpAddr, prefix: u8) -> IpAddr {
    match addr {
        IpAddr::V4(ip) => {
            let bits = u32::from(ip);
            let mask = if prefix == 0 {
                0
            } else {
                u32::MAX << (32 - u32::from(prefix))
            };
            IpAddr::V4((bits & mask).into())
        }
        IpAddr::V6(ip) => {
            let bits = u128::from(ip);
            let mask = if prefix == 0 {
                0
            } else {
                u128::MAX << (128 - u32::from(prefix))
            };
            IpAddr::V6((bits & mask).into())
        }
    }
}

fn is_trusted(addr: IpAddr, trusted: &[TrustedProxyNet]) -> bool {
    trusted.iter().any(|net| net.contains(addr))
}

/// 解析客户端 IP。无连接信息、坏/重复/过长代理头都回落到同一保守桶，
/// 不允许攻击者借非法表示制造新键。
pub fn resolve_client_ip(req: &Request, trusted: &[TrustedProxyNet]) -> IpAddr {
    let peer = req
        .extensions()
        .get::<ConnectInfo<std::net::SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip())
        .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    if !is_trusted(peer, trusted) {
        return peer;
    }

    let mut forwarded = req.headers().get_all("x-forwarded-for").iter();
    let Some(value) = forwarded.next() else {
        return peer;
    };
    if forwarded.next().is_some() {
        return peer;
    }
    let Ok(raw) = value.to_str() else {
        return peer;
    };
    if raw.len() > MAX_FORWARDED_BYTES {
        return peer;
    }

    let mut hops = Vec::new();
    for part in raw.split(',') {
        if hops.len() == MAX_FORWARDED_HOPS {
            return peer;
        }
        let Ok(ip) = part.trim().parse::<IpAddr>() else {
            return peer;
        };
        hops.push(ip);
    }
    if hops.is_empty() {
        return peer;
    }

    let mut current = peer;
    for hop in hops.into_iter().rev() {
        if !is_trusted(current, trusted) {
            return current;
        }
        current = hop;
    }
    if is_trusted(current, trusted) {
        peer
    } else {
        current
    }
}

/// 一次请求唯一、只由服务端生成的关联元数据。
#[derive(Clone, Debug)]
pub struct RequestMeta {
    pub client_ip: IpAddr,
    pub request_id: RequestId,
    pub trace_id: TraceId,
}

impl RequestMeta {
    pub fn generate(client_ip: IpAddr) -> Self {
        let request_raw = ep_platform_obs::TraceContext::new().trace_id().to_string();
        let trace_raw = ep_platform_obs::TraceContext::new().trace_id().to_string();
        Self {
            client_ip,
            request_id: RequestId::new(&request_raw).expect("内部请求标识形态固定合法"),
            trace_id: TraceId::new(&trace_raw).expect("内部追踪标识形态固定合法"),
        }
    }
}

/// 确保请求上只有一份服务端关联元数据。core-server 的更外层会先按
/// 配置解析可信代理；其他本机进程默认不信任代理头，以传输对端为源。
/// 各中间件都可防御性调用本函数：既有扩展总是复用，绝不为同一请求
/// 再生成第二个 trace。
pub fn ensure_request_meta(req: &mut Request) -> RequestMeta {
    if let Some(meta) = req.extensions().get::<RequestMeta>() {
        return meta.clone();
    }
    // 首个公共 HTTP 边界剥离所有客户端内部头；已有 RequestMeta 表示外层
    // 可信中间件已完成边界处理，不能误删 core 注入的服务端关联头。
    let injected: Vec<_> = req
        .headers()
        .keys()
        .filter(|name| name.as_str().starts_with("x-ep-"))
        .cloned()
        .collect();
    for name in injected {
        req.headers_mut().remove(name);
    }
    let meta = RequestMeta::generate(resolve_client_ip(req, &[]));
    req.extensions_mut().insert(meta.clone());
    meta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cidr_membership_is_canonical_for_both_address_families() {
        let v4 = TrustedProxyNet::parse("10.4.9.8/8").expect("CIDR 合法");
        assert!(v4.contains("10.255.0.1".parse().unwrap()));
        assert!(!v4.contains("11.0.0.1".parse().unwrap()));
        let v6 = TrustedProxyNet::parse("2001:db8:1::7/32").expect("CIDR 合法");
        assert!(v6.contains("2001:db8:ffff::1".parse().unwrap()));
        assert!(!v6.contains("2001:db9::1".parse().unwrap()));
    }

    #[test]
    fn malformed_prefixes_are_rejected_at_config_parse_time() {
        assert!(TrustedProxyNet::parse("10.0.0.0/33").is_err());
        assert!(TrustedProxyNet::parse("2001:db8::/129").is_err());
        assert!(TrustedProxyNet::parse("not-an-ip").is_err());
        assert!(TrustedProxyNet::parse("0.0.0.0/0").is_err());
        assert!(TrustedProxyNet::parse("::/0").is_err());
    }
}
