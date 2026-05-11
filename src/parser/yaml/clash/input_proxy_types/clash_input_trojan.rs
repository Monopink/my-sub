
use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a Trojan proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputTrojan {
    name: String,
    server: String,
    port: u16,
    password: String,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(default)]
    network: Option<String>,
    #[serde(default)]
    sni: Option<String>,
}

impl ClashInputTrojan {
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

    pub fn udp(&self) -> Option<bool> {
        self.udp
    }

    pub fn tfo(&self) -> Option<bool> {
        self.tfo
    }

    pub fn skip_cert_verify(&self) -> Option<bool> {
        self.skip_cert_verify
    }

    pub fn network(&self) -> Option<&str> {
        self.network.as_deref()
    }

    pub fn sni(&self) -> Option<&str> {
        self.sni.as_deref()
    }
}

impl Into<Proxy> for ClashInputTrojan {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::Trojan;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.password = Some(self.password);
        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);
        proxy.sni = self.sni;

        if let Some(net) = self.network {
            proxy.transfer_protocol = Some(net);
        }

        proxy
    }
}
