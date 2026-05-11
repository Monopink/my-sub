use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowsocksProxy {
    pub server: String,
    pub port: u16,
    pub password: String,
    // cipher is the alias for encryption method.
    pub cipher: String,
    pub udp: Option<bool>,
    pub tfo: Option<bool>,
    pub skip_cert_verify: Option<bool>,
    pub plugin: Option<String>,
    pub plugin_opts: Option<String>,
    pub udp_over_tcp: Option<bool>,
    pub udp_over_tcp_version: Option<u8>,
    pub client_fingerprint: Option<String>,
}

impl Default for ShadowsocksProxy {
    fn default() -> Self {
        Self {
            server: String::new(),
            port: 0,
            password: String::new(),
            cipher: String::new(),
            udp: None,
            tfo: None,
            skip_cert_verify: None,
            plugin: None,
            plugin_opts: None,
            udp_over_tcp: None,
            udp_over_tcp_version: None,
            client_fingerprint: None,
        }
    }
}
