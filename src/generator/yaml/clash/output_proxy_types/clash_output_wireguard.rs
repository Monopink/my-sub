use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::{is_empty_option_string, is_u32_option_zero};
use serde::{Deserialize, Serialize};

/// WireGuard proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WireGuardProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub private_key: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub public_key: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub ipv6: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub preshared_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_u32_option_zero")]
    pub mtu: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_u32_option_zero")]
    pub keepalive: Option<u32>,
}

impl WireGuardProxy {
    /// Create a new WireGuard proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            private_key: None,
            public_key: None,
            ip: None,
            ipv6: None,
            preshared_key: None,
            dns: None,
            mtu: None,
            allowed_ips: None,
            keepalive: None,
        }
    }
}

impl From<Proxy> for WireGuardProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut wg = WireGuardProxy::new(common);

        wg.private_key = proxy.private_key.clone();
        wg.public_key = proxy.public_key.clone();
        wg.ip = proxy.self_ip.clone();
        wg.ipv6 = proxy.self_ipv6.clone();
        wg.preshared_key = proxy.pre_shared_key.clone();

        if !proxy.dns_servers.is_empty() {
            wg.dns = Some(proxy.dns_servers.iter().cloned().collect());
        }

        wg.mtu = Some(proxy.mtu as u32);

        if !proxy.allowed_ips.is_empty() {
            wg.allowed_ips = Some(
                proxy
                    .allowed_ips
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            );
        }

        wg.keepalive = Some(proxy.keep_alive as u32);

        wg
    }
}
