use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::is_empty_option_string;
use serde::{Deserialize, Serialize};

/// HTTP proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HttpProxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub password: Option<String>,
}

impl HttpProxy {
    /// Create a new HTTP proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            username: None,
            password: None,
        }
    }
}

impl From<Proxy> for HttpProxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .tls(Some(proxy.proxy_type == crate::models::ProxyType::HTTPS))
                .build();

        let mut http = HttpProxy::new(common);

        http.username = proxy.username;
        http.password = proxy.password;

        http
    }
}
