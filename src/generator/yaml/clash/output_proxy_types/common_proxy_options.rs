use crate::utils::is_empty_option_string;
use serde::{Deserialize, Serialize};
/// Common proxy options that can be used across different proxy types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommonProxyOptions {
    pub name: String,
    pub server: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_cert_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<bool>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub client_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub sni: Option<String>,
    // Additional fields from the Go implementations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mptcp: Option<bool>,
    #[serde(skip_serializing_if = "is_empty_option_string")]
    pub interface: Option<String>, // interface-name in ClashMeta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_mark: Option<i32>, // routing-mark in ClashMeta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_version: Option<String>, // ip-version in ClashMeta
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_proxy: Option<String>, // dialer-proxy in ClashMeta
}

impl CommonProxyOptions {
    /// Create a new CommonProxyOptions with default values
    pub fn new(name: String, server: String, port: u16) -> Self {
        Self {
            name,
            server,
            port,
            udp: None,
            tfo: None,
            skip_cert_verify: None,
            tls: None,
            fingerprint: None,
            client_fingerprint: None,
            sni: None,
            mptcp: None,
            interface: None,
            routing_mark: None,
            ip_version: None,
            dialer_proxy: None,
        }
    }

    /// Create a builder for CommonProxyOptions
    pub fn builder(name: String, server: String, port: u16) -> CommonProxyOptionsBuilder {
        CommonProxyOptionsBuilder {
            common: Self::new(name, server, port),
        }
    }
}

/// Builder for CommonProxyOptions
pub struct CommonProxyOptionsBuilder {
    common: CommonProxyOptions,
}

impl CommonProxyOptionsBuilder {
    /// Set UDP option
    pub fn udp(mut self, value: Option<bool>) -> Self {
        self.common.udp = value;
        self
    }

    /// Set TFO (TCP Fast Open) option
    pub fn tfo(mut self, value: Option<bool>) -> Self {
        self.common.tfo = value;
        self
    }

    /// Set skip_cert_verify option
    pub fn skip_cert_verify(mut self, value: Option<bool>) -> Self {
        self.common.skip_cert_verify = value;
        self
    }

    /// Set TLS option
    pub fn tls(mut self, value: Option<bool>) -> Self {
        self.common.tls = value;
        self
    }

    /// Set SNI option
    pub fn sni(mut self, value: Option<String>) -> Self {
        self.common.sni = value;
        self
    }

    /// Set fingerprint option
    pub fn fingerprint(mut self, value: Option<String>) -> Self {
        self.common.fingerprint = value;
        self
    }

    /// Set client_fingerprint option
    pub fn client_fingerprint(mut self, value: Option<String>) -> Self {
        self.common.client_fingerprint = value;
        self
    }

    /// Set mptcp option
    pub fn mptcp(mut self, value: Option<bool>) -> Self {
        self.common.mptcp = value;
        self
    }

    /// Set interface option
    pub fn interface(mut self, value: Option<String>) -> Self {
        self.common.interface = value;
        self
    }

    /// Set routing_mark option
    pub fn routing_mark(mut self, value: Option<i32>) -> Self {
        self.common.routing_mark = value;
        self
    }

    /// Set ip_version option
    pub fn ip_version(mut self, value: Option<String>) -> Self {
        self.common.ip_version = value;
        self
    }

    /// Set dialer_proxy option
    pub fn dialer_proxy(mut self, value: Option<String>) -> Self {
        self.common.dialer_proxy = value;
        self
    }

    /// Build the final CommonProxyOptions
    pub fn build(self) -> CommonProxyOptions {
        self.common
    }
}
