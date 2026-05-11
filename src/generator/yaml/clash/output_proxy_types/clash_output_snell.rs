use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::{is_empty_option_string, is_u32_option_zero};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Snell proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SnellProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub psk: Option<String>,
    #[serde(skip_serializing_if = "is_u32_option_zero")]
    pub version: Option<u32>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfs_opts: Option<HashMap<String, String>>,
}

impl SnellProxy {
    /// Create a new Snell proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            psk: None,
            version: None,
            obfs: None,
            obfs_opts: None,
        }
    }
}

impl From<Proxy> for SnellProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut snell = SnellProxy::new(common);

        snell.psk = proxy.password;
        snell.version = Some(proxy.snell_version as u32);
        snell.obfs = proxy.obfs;

        if let Some(obfs_opts) = proxy.obfs_param {
            let mut opts = HashMap::new();

            for opt in obfs_opts.split(';') {
                let parts: Vec<&str> = opt.split('=').collect();
                if parts.len() == 2 {
                    opts.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            snell.obfs_opts = Some(opts);
        }

        snell
    }
}
