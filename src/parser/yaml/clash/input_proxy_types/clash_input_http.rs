
use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents an HTTP/HTTPS proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputHttp {
    name: String,
    server: String,
    port: u16,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(default)]
    tls: Option<bool>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
}

impl ClashInputHttp {
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

    pub fn tls(&self) -> Option<bool> {
        self.tls
    }

    pub fn skip_cert_verify(&self) -> Option<bool> {
        self.skip_cert_verify
    }
}

impl Into<Proxy> for ClashInputHttp {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = if self.tls.unwrap_or(false) {
            ProxyType::HTTPS
        } else {
            ProxyType::HTTP
        };
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.username = self.username;
        proxy.password = self.password;
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);

        proxy
    }
}
