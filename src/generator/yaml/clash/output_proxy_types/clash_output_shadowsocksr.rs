use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::is_empty_option_string;
use serde::{Deserialize, Serialize};

/// ShadowsocksR proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShadowsocksRProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub cipher: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub protocol_param: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub obfs_param: Option<String>,
}

impl ShadowsocksRProxy {
    /// Create a new ShadowsocksR proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            cipher: None,
            password: None,
            protocol: None,
            protocol_param: None,
            obfs: None,
            obfs_param: None,
        }
    }
}

impl From<Proxy> for ShadowsocksRProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut ssr = ShadowsocksRProxy::new(common);

        ssr.cipher = proxy.encrypt_method;
        ssr.password = proxy.password;
        ssr.protocol = proxy.protocol;
        ssr.protocol_param = proxy.protocol_param;
        ssr.obfs = proxy.obfs;
        ssr.obfs_param = proxy.obfs_param;

        ssr
    }
}
