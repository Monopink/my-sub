use std::collections::HashMap;

use serde::Deserialize;

use crate::models::proxy::Proxy;
use crate::models::proxy::ProxyType;
use crate::utils::tribool::OptionSetExt;

/// Represents a VMess proxy in Clash configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub struct ClashInputVMess {
    name: String,
    server: String,
    port: u16,
    uuid: String,
    #[serde(alias = "alterId", default)]
    alter_id: u32,
    cipher: String,
    #[serde(default)]
    udp: Option<bool>,
    #[serde(default)]
    tfo: Option<bool>,
    #[serde(alias = "skip-cert-verify", default)]
    skip_cert_verify: Option<bool>,
    #[serde(default)]
    network: Option<String>,
    #[serde(alias = "ws-path", default)]
    ws_path: Option<String>,
    #[serde(alias = "ws-headers", default)]
    ws_headers: Option<HashMap<String, String>>,
    #[serde(default)]
    tls: Option<bool>,
    #[serde(alias = "servername", default)]
    servername: Option<String>,
}

impl ClashInputVMess {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn server(&self) -> &str {
        &self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn alter_id(&self) -> u32 {
        self.alter_id
    }

    pub fn cipher(&self) -> &str {
        &self.cipher
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

    pub fn network(&self) -> Option<&str> {
        self.network.as_deref()
    }

    pub fn ws_path(&self) -> Option<&str> {
        self.ws_path.as_deref()
    }

    pub fn ws_headers(&self) -> Option<&HashMap<String, String>> {
        self.ws_headers.as_ref()
    }

    pub fn tls(&self) -> Option<bool> {
        self.tls
    }

    pub fn servername(&self) -> Option<&str> {
        self.servername.as_deref()
    }
}

impl Into<Proxy> for ClashInputVMess {
    fn into(self) -> Proxy {
        let mut proxy = Proxy::default();
        proxy.proxy_type = ProxyType::VMess;
        proxy.remark = self.name;
        proxy.hostname = self.server.clone();
        proxy.port = self.port;
        proxy.user_id = Some(self.uuid);
        proxy.alter_id = self.alter_id as u16;
        proxy.encrypt_method = Some(self.cipher);
        proxy.udp.set_if_some(self.udp);
        proxy.tcp_fast_open.set_if_some(self.tfo);
        proxy.allow_insecure.set_if_some(self.skip_cert_verify);
        proxy.tls_secure = self.tls.unwrap_or(false);
        proxy.server_name = self.servername;

        // Network protocol handling
        if let Some(net) = self.network {
            proxy.transfer_protocol = Some(net.clone());
            match net.as_str() {
                "ws" => {
                    if let Some(path) = self.ws_path {
                        proxy.path = Some(path);
                    }
                    if let Some(headers) = self.ws_headers {
                        if let Some(host) = headers.get("Host") {
                            proxy.host = Some(host.clone());
                        }
                        if let Some(edge) = headers.get("Edge") {
                            proxy.edge = Some(edge.clone());
                        }
                    }
                }
                "http" => {
                    if let Some(path) = self.ws_path {
                        proxy.path = Some(path);
                    }
                    if let Some(headers) = self.ws_headers {
                        if let Some(host) = headers.get("Host") {
                            proxy.host = Some(host.clone());
                        }
                        if let Some(edge) = headers.get("Edge") {
                            proxy.edge = Some(edge.clone());
                        }
                    }
                }
                "h2" => {
                    if let Some(path) = self.ws_path {
                        proxy.path = Some(path);
                    }
                    if let Some(headers) = self.ws_headers {
                        if let Some(host) = headers.get("Host") {
                            proxy.host = Some(host.clone());
                        }
                    }
                }
                "grpc" => {
                    if let Some(path) = self.ws_path {
                        proxy.path = Some(path);
                    }
                    proxy.host = Some(self.server);
                }
                _ => {}
            }
        }

        proxy
    }
}
