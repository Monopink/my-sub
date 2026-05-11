use std::collections::HashSet;

use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a WireGuard proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputWireGuard {
    name: String,
    server: String,
    port: u16,
    #[serde(alias = "private-key")]
    private_key: String,
    #[serde(alias = "public-key")]
    public_key: String,
    #[serde(default)]
    ip: Option<String>,
    #[serde(default)]
    ipv6: Option<String>,
    #[serde(alias = "preshared-key", default)]
    preshared_key: Option<String>,
    #[serde(default)]
    dns: Option<Vec<String>>,
    #[serde(default)]
    mtu: Option<u32>,
    #[serde(alias = "allowed-ips", default)]
    allowed_ips: Option<Vec<String>>,
    #[serde(default)]
    keepalive: Option<u32>,
    #[serde(default)]
    udp: Option<bool>,
}

impl ClashInputWireGuard {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn ip(&self) -> Option<&str> {
        self.ip.as_deref()
    }

    pub fn ipv6(&self) -> Option<&str> {
        self.ipv6.as_deref()
    }

    pub fn preshared_key(&self) -> Option<&str> {
        self.preshared_key.as_deref()
    }

    pub fn dns(&self) -> Option<&Vec<String>> {
        self.dns.as_ref()
    }

    pub fn mtu(&self) -> Option<u32> {
        self.mtu
    }

    pub fn allowed_ips(&self) -> Option<&Vec<String>> {
        self.allowed_ips.as_ref()
    }

    pub fn keepalive(&self) -> Option<u32> {
        self.keepalive
    }

    pub fn udp(&self) -> Option<bool> {
        self.udp
    }
}

impl Into<Proxy> for ClashInputWireGuard {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::WireGuard;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.private_key = Some(self.private_key);
        proxy.public_key = Some(self.public_key);
        proxy.self_ip = self.ip;
        proxy.self_ipv6 = self.ipv6;
        proxy.pre_shared_key = self.preshared_key;

        // Convert Vec<String> to HashSet<String> for dns_servers
        let mut dns_set = HashSet::new();
        if let Some(dns_servers) = self.dns {
            for dns_server in dns_servers {
                dns_set.insert(dns_server);
            }
        }
        proxy.dns_servers = dns_set;

        proxy.mtu = self.mtu.unwrap_or(0) as u16;
        proxy.allowed_ips = self.allowed_ips.unwrap_or_default().join(",");
        proxy.keep_alive = self.keepalive.unwrap_or(0) as u16;
        proxy.udp.set_if_some(self.udp);

        proxy
    }
}
