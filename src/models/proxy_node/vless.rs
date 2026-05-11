use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlessProxy {
    pub uuid: String,
    pub flow: Option<String>,
    pub tls: bool,
    pub alpn: HashSet<String>,
    pub udp: bool,
    pub packet_addr: Option<bool>,
    pub xudp: Option<bool>,
    pub packet_encoding: Option<String>,
    pub network: Option<String>,
    pub reality_public_key: Option<String>,
    pub reality_short_id: Option<String>,
    pub http_method: Option<String>,
    pub http_path: Option<String>,
    pub http_headers: Option<HashMap<String, Vec<String>>>,
    pub h2_host: Option<Vec<String>>,
    pub h2_path: Option<String>,
    pub grpc_service_name: Option<String>,
    pub ws_path: Option<String>,
    pub ws_headers: Option<HashMap<String, String>>,
    pub skip_cert_verify: Option<bool>,
    pub fingerprint: Option<String>,
    pub servername: Option<String>,
    pub client_fingerprint: Option<String>,
}

impl Default for VlessProxy {
    fn default() -> Self {
        Self {
            uuid: String::new(),
            flow: None,
            tls: false,
            alpn: HashSet::new(),
            udp: true,
            packet_addr: None,
            xudp: None,
            packet_encoding: None,
            network: None,
            reality_public_key: None,
            reality_short_id: None,
            http_method: None,
            http_path: None,
            http_headers: None,
            h2_host: None,
            h2_path: None,
            grpc_service_name: None,
            ws_path: None,
            ws_headers: None,
            skip_cert_verify: None,
            fingerprint: None,
            servername: None,
            client_fingerprint: None,
        }
    }
}
