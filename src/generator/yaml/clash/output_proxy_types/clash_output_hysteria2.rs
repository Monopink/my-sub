use super::CommonProxyOptions;
use crate::models::Proxy;
use crate::utils::{is_empty_option_string, is_u32_option_zero};
use serde::{Deserialize, Serialize};

/// Hysteria2 proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hysteria2Proxy {
    #[serde(flatten)]
    pub common: CommonProxyOptions,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub obfs: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub obfs_password: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub ports: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub up: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub down: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub ca: Option<String>,
    #[serde(rename = "ca-str", skip_serializing_if = "is_empty_option_string")]
    pub ca_str: Option<String>,
    #[serde(skip_serializing_if = "is_u32_option_zero")]
    pub cwnd: Option<u32>,
    #[serde(rename = "udp-mtu", skip_serializing_if = "is_u32_option_zero")]
    pub udp_mtu: Option<u32>,
    #[serde(
        rename = "recv-window-conn",
        skip_serializing_if = "is_u32_option_zero"
    )]
    pub recv_window_conn: Option<u32>,
    #[serde(rename = "recv-window", skip_serializing_if = "is_u32_option_zero")]
    pub recv_window: Option<u32>,
    #[serde(
        rename = "disable-mtu-discovery",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_mtu_discovery: Option<bool>,
    #[serde(rename = "fast-open", skip_serializing_if = "Option::is_none")]
    pub fast_open: Option<bool>,
    #[serde(rename = "hop-interval", skip_serializing_if = "is_u32_option_zero")]
    pub hop_interval: Option<u32>,
}

impl Hysteria2Proxy {
    /// Create a new Hysteria2 proxy
    pub fn new(common: CommonProxyOptions) -> Self {
        Self {
            common,
            password: None,
            obfs: None,
            obfs_password: None,
            ports: None,
            up: None,
            down: None,
            fingerprint: None,
            alpn: None,
            ca: None,
            ca_str: None,
            cwnd: None,
            udp_mtu: None,
            recv_window_conn: None,
            recv_window: None,
            disable_mtu_discovery: None,
            fast_open: None,
            hop_interval: None,
        }
    }
}

impl From<Proxy> for Hysteria2Proxy {
    fn from(proxy: Proxy) -> Self {
        let common =
            CommonProxyOptions::builder(proxy.remark.clone(), proxy.hostname.clone(), proxy.port)
                .udp(proxy.udp)
                .tfo(proxy.tcp_fast_open)
                .skip_cert_verify(proxy.allow_insecure)
                .sni(proxy.sni.clone())
                .build();

        let mut hysteria2 = Hysteria2Proxy::new(common);

        if let Some(ca_str) = &proxy.ca_str {
            hysteria2.ca_str = Some(ca_str.to_owned());
        }

        hysteria2.password = proxy.password;
        hysteria2.obfs = proxy.obfs;
        hysteria2.obfs_password = proxy.obfs_param;
        hysteria2.ports = proxy.ports;

        if proxy.up_speed > 0 {
            hysteria2.up = Some(format!("{}Mbps", proxy.up_speed));
        }

        if proxy.down_speed > 0 {
            hysteria2.down = Some(format!("{}Mbps", proxy.down_speed));
        }

        hysteria2.fingerprint = proxy.fingerprint;

        if !proxy.alpn.is_empty() {
            hysteria2.alpn = Some(proxy.alpn.into_iter().collect());
        }

        hysteria2.ca = proxy.ca;
        hysteria2.ca_str = proxy.ca_str;

        if proxy.cwnd > 0 {
            hysteria2.cwnd = Some(proxy.cwnd);
        }

        if proxy.mtu > 0 {
            hysteria2.udp_mtu = Some(proxy.mtu as u32);
        }

        if proxy.recv_window_conn > 0 {
            hysteria2.recv_window_conn = Some(proxy.recv_window_conn);
        }

        if proxy.recv_window > 0 {
            hysteria2.recv_window = Some(proxy.recv_window);
        }

        hysteria2.disable_mtu_discovery = proxy.disable_mtu_discovery;
        hysteria2.fast_open = proxy.tcp_fast_open;

        if proxy.hop_interval > 0 {
            hysteria2.hop_interval = Some(proxy.hop_interval);
        }

        hysteria2
    }
}
