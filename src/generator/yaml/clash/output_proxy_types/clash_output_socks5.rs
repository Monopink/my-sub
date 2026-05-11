use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::is_empty_option_string;
use serde::{Deserialize, Serialize};

/// SOCKS5 proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Socks5Proxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub password: Option<String>,
}

impl Socks5Proxy {
    /// Create a new SOCKS5 proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            username: None,
            password: None,
        }
    }
}

impl From<Proxy> for Socks5Proxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut socks5 = Socks5Proxy::new(common);

        socks5.username = proxy.username;
        socks5.password = proxy.password;

        socks5
    }
}
