use crate::parser::yaml::clash::clash_proxy_types::ClashProxyYamlInput;
use serde::Deserialize;

/// Represents a Clash configuration input structure
#[derive(Debug, Clone, Deserialize)]
pub struct ClashYamlInput {
    #[serde(default)]
    pub proxies: Vec<ClashProxyYamlInput>,
}

impl ClashYamlInput {
    /// Extract proxies from the configuration
    pub fn extract_proxies(self) -> Vec<ClashProxyYamlInput> {
        self.proxies
    }
}
