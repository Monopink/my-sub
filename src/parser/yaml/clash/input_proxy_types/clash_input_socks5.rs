
use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a SOCKS5 proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputSocks5 {
    name: String,
    server: String,
    port: u16,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
}

impl ClashInputSocks5 {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    pub fn skip_cert_verify(&self) -> Option<bool> {
        self.skip_cert_verify
    }

    pub fn udp(&self) -> Option<bool> {
        self.udp
    }

    pub fn tfo(&self) -> Option<bool> {
        self.tfo
    }
}

impl Into<Proxy> for ClashInputSocks5 {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::Socks5;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.username = self.username;
        proxy.password = self.password;
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);
        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);

        proxy
    }
}
