use std::collections::HashMap;

use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a Snell proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputSnell {
    name: String,
    server: String,
    port: u16,
    psk: String,
    #[serde(default)]
    version: Option<u32>,
    #[serde(default)]
    obfs: Option<String>,
    #[serde(rename = "obfs-opts", default)]
    obfs_opts: Option<HashMap<String, String>>,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
}

impl ClashInputSnell {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn psk(&self) -> &str {
        &self.psk
    }

    pub fn version(&self) -> Option<u32> {
        self.version
    }

    pub fn obfs(&self) -> Option<&str> {
        self.obfs.as_deref()
    }

    pub fn obfs_opts(&self) -> Option<&HashMap<String, String>> {
        self.obfs_opts.as_ref()
    }

    pub fn udp(&self) -> Option<bool> {
        self.udp
    }

    pub fn tfo(&self) -> Option<bool> {
        self.tfo
    }
}

impl Into<Proxy> for ClashInputSnell {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::Snell;
        proxy.remark = self.name;
        proxy.hostname = self.server;
        proxy.port = self.port;
        proxy.password = Some(self.psk);
        proxy.snell_version = self.version.unwrap_or(1) as u16;
        proxy.obfs = self.obfs;

        if let Some(opts) = self.obfs_opts {
            let mut obfs_opts_str = String::new();
            for (key, value) in opts {
                if !obfs_opts_str.is_empty() {
                    obfs_opts_str.push(';');
                }
                obfs_opts_str.push_str(&format!("{}={}", key, value));
            }
            proxy.plugin_option = Some(obfs_opts_str);
        }

        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);

        proxy
    }
}
