use serde::Deserialize;
use serde_yaml::Value;

/// Represents a Clash configuration input structure
#[derive(Debug, Clone, Deserialize)]
pub struct ClashYamlInput {
    #[serde(default, alias = "Proxy", alias = "Proxies")]
    pub proxies: Vec<Value>,
}

impl ClashYamlInput {
    /// Extract proxies from the configuration
    pub fn extract_proxies(self) -> Vec<Value> {
        self.proxies
    }
}
