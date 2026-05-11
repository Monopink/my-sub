use std::collections::HashSet;

use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::deserialize::deserialize_string_or_number;
use crate::utils::tribool::OptionSetExt;

/// Represents a Hysteria2 proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputHysteria2 {
    name: String,
    server: String,
    port: u16,
    password: String,
    #[serde(default)]
    ports: Option<String>,
    #[serde(alias = "hop-interval", default)]
    hop_interval: Option<u32>,
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    up: Option<String>,
    #[serde(default, deserialize_with = "deserialize_string_or_number")]
    down: Option<String>,
    #[serde(default)]
    obfs: Option<String>,
    #[serde(alias = "obfs-password", default)]
    obfs_password: Option<String>,
    #[serde(default)]
    fingerprint: Option<String>,
    #[serde(default)]
    alpn: Option<String>,
    #[serde(default)]
    ca: Option<String>,
    #[serde(alias = "ca-str", default)]
    ca_str: Option<String>,
    #[serde(default)]
    cwnd: Option<u32>,
    #[serde(alias = "udp-mtu", default)]
    udp_mtu: Option<u32>,
    #[serde(default)]
    sni: Option<String>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(alias = "fast-open", default)]
    fast_open: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
}

impl ClashInputHysteria2 {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn ports(&self) -> Option<&str> {
        self.ports.as_deref()
    }

    pub fn hop_interval(&self) -> Option<u32> {
        self.hop_interval
    }

    pub fn up(&self) -> Option<&str> {
        self.up.as_deref()
    }

    pub fn down(&self) -> Option<&str> {
        self.down.as_deref()
    }

    pub fn obfs(&self) -> Option<&str> {
        self.obfs.as_deref()
    }

    pub fn obfs_password(&self) -> Option<&str> {
        self.obfs_password.as_deref()
    }

    pub fn fingerprint(&self) -> Option<&str> {
        self.fingerprint.as_deref()
    }

    pub fn alpn(&self) -> Option<&str> {
        self.alpn.as_deref()
    }

    pub fn ca(&self) -> Option<&str> {
        self.ca.as_deref()
    }

    pub fn ca_str(&self) -> Option<&str> {
        self.ca_str.as_deref()
    }

    pub fn cwnd(&self) -> Option<u32> {
        self.cwnd
    }

    pub fn udp_mtu(&self) -> Option<u32> {
        self.udp_mtu
    }

    pub fn sni(&self) -> Option<&str> {
        self.sni.as_deref()
    }

    pub fn skip_cert_verify(&self) -> Option<bool> {
        self.skip_cert_verify
    }

    pub fn fast_open(&self) -> Option<bool> {
        self.fast_open
    }

    pub fn tfo(&self) -> Option<bool> {
        self.tfo
    }
}

impl Into<Proxy> for ClashInputHysteria2 {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::Hysteria2;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.password = Some(self.password);
        proxy.ports = self.ports;
        proxy.hop_interval = self.hop_interval.unwrap_or(0);

        // Handle upload/download speed
        if let Some(up_value) = self.up {
            proxy.up_speed = up_value.replace("Mbps", "").parse().unwrap_or(0);
        }

        if let Some(down_value) = self.down {
            proxy.down_speed = down_value.replace("Mbps", "").parse().unwrap_or(0);
        }

        // Set obfuscation options
        proxy.obfs = self.obfs;
        proxy.obfs_param = self.obfs_password;

        // Set TLS related fields
        proxy.fingerprint = self.fingerprint;

        // Handle alpn as a comma-separated string to HashSet
        if let Some(alpn_value) = self.alpn {
            let mut alpn_set = HashSet::new();
            for value in alpn_value.split(',').map(|s| s.trim().to_string()) {
                if !value.is_empty() {
                    alpn_set.insert(value);
                }
            }
            proxy.alpn = alpn_set;
        }

        proxy.ca = self.ca;
        proxy.ca_str = self.ca_str;
        proxy.sni = self.sni;

        // Set other network parameters
        proxy.cwnd = self.cwnd.unwrap_or(0);
        proxy.mtu = self.udp_mtu.unwrap_or(0) as u16;

        // Set boolean options
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);
        proxy.tcp_fast_open.set_if_some(self.fast_open.or(self.tfo));

        proxy
    }
}
