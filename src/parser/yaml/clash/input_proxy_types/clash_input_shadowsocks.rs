use std::collections::HashMap;

use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a Shadowsocks proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputShadowsocks {
    name: String,
    server: String,
    port: u16,
    cipher: String,
    password: String,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(default)]
    plugin: Option<String>,
    #[serde(alias = "plugin-opts", default)]
    plugin_opts: Option<HashMap<String, String>>,
}

impl ClashInputShadowsocks {
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

    pub fn udp(&self) -> Option<bool> {
        self.udp
    }

    pub fn tfo(&self) -> Option<bool> {
        self.tfo
    }

    pub fn skip_cert_verify(&self) -> Option<bool> {
        self.skip_cert_verify
    }

    pub fn plugin(&self) -> Option<&str> {
        self.plugin.as_deref()
    }

    pub fn plugin_opts(&self) -> Option<&HashMap<String, String>> {
        self.plugin_opts.as_ref()
    }
}

impl Into<Proxy> for ClashInputShadowsocks {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::Shadowsocks;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.encrypt_method = Some(self.cipher);
        proxy.password = Some(self.password);
        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);

        if let Some(plugin_name) = self.plugin {
            proxy.plugin = Some(plugin_name);
            if let Some(opts) = self.plugin_opts {
                let mut plugin_opts_str = String::new();
                for (key, value) in opts {
                    if !plugin_opts_str.is_empty() {
                        plugin_opts_str.push(';');
                    }
                    plugin_opts_str.push_str(&format!("{}={}", key, value));
                }
                proxy.plugin_option = Some(plugin_opts_str);
            }
        }

        proxy
    }
}
