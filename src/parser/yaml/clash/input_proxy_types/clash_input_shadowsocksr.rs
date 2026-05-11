
use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a ShadowsocksR proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputShadowsocksR {
    name: String,
    server: String,
    port: u16,
    cipher: String,
    password: String,
    protocol: String,
    obfs: String,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(alias = "protocol-param", default)]
    protocol_param: Option<String>,
    #[serde(alias = "obfs-param", default)]
    obfs_param: Option<String>,
}

impl ClashInputShadowsocksR {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn cipher(&self) -> &str {
        &self.cipher
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn obfs(&self) -> &str {
        &self.obfs
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

    pub fn protocol_param(&self) -> Option<&str> {
        self.protocol_param.as_deref()
    }

    pub fn obfs_param(&self) -> Option<&str> {
        self.obfs_param.as_deref()
    }
}

impl Into<Proxy> for ClashInputShadowsocksR {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::ShadowsocksR;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.encrypt_method = Some(self.cipher);
        proxy.password = Some(self.password);
        proxy.protocol = Some(self.protocol);
        proxy.obfs = Some(self.obfs);
        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);
        proxy.protocol_param = self.protocol_param;
        proxy.obfs_param = self.obfs_param;

        proxy
    }
}
