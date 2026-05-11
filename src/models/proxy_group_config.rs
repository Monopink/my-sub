/// Type of proxy group
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProxyGroupType {
    Select,
    URLTest,
    Fallback,
    LoadBalance,
    Relay,
    SSID,
    Smart,
}

impl ProxyGroupType {
    /// Get string representation of the proxy group type
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyGroupType::Select => "select",
            ProxyGroupType::URLTest => "url-test",
            ProxyGroupType::LoadBalance => "load-balance",
            ProxyGroupType::Fallback => "fallback",
            ProxyGroupType::Relay => "relay",
            ProxyGroupType::SSID => "ssid",
            ProxyGroupType::Smart => "smart",
        }
    }
}

/// Load balancing strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BalanceStrategy {
    ConsistentHashing,
    RoundRobin,
}

impl BalanceStrategy {
    /// Get string representation of the balance strategy
    pub fn as_str(&self) -> &'static str {
        match self {
            BalanceStrategy::ConsistentHashing => "consistent-hashing",
            BalanceStrategy::RoundRobin => "round-robin",
        }
    }
}

/// Configuration for a proxy group
#[derive(Debug, Clone)]
pub struct ProxyGroupConfig {
    /// Name of the proxy group
    pub name: String,
    /// Type of the proxy group
    pub group_type: ProxyGroupType,
    /// List of proxy names in this group
    pub proxies: Vec<String>,
    /// List of provider names used by this group
    pub using_provider: Vec<String>,
    /// URL for testing
    pub url: String,
    /// Interval in seconds between tests
    pub interval: u32,
    /// Timeout in seconds for tests
    pub timeout: u32,
    /// Tolerance value for tests
    pub tolerance: u32,
    /// Strategy for load balancing
    pub strategy: BalanceStrategy,
    /// Whether to use lazy loading
    pub lazy: bool,
    /// Whether to disable UDP support
    pub disable_udp: bool,
    /// Whether to persist connections
    pub persistent: bool,
    /// Whether to evaluate before use
    pub evaluate_before_use: bool,
}

impl Default for ProxyGroupConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            group_type: ProxyGroupType::Select,
            proxies: Vec::new(),
            using_provider: Vec::new(),
            url: String::new(),
            interval: 0,
            timeout: 0,
            tolerance: 0,
            strategy: BalanceStrategy::ConsistentHashing,
            lazy: false,
            disable_udp: false,
            persistent: false,
            evaluate_before_use: false,
        }
    }
}

impl ProxyGroupConfig {
    /// Create a new proxy group config
    pub fn new(name: String, group_type: ProxyGroupType) -> Self {
        Self {
            name,
            group_type,
            ..Default::default()
        }
    }

    /// Get string representation of the group type
    pub fn type_str(&self) -> &'static str {
        self.group_type.as_str()
    }

    /// Get string representation of the balance strategy
    pub fn strategy_str(&self) -> &'static str {
        self.strategy.as_str()
    }
}

/// A collection of proxy group configurations
pub type ProxyGroupConfigs = Vec<ProxyGroupConfig>;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

impl Serialize for ProxyGroupConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Figure out how many fields we'll have based on the proxy group type
        let mut field_count = 2; // name and type always present

        // Count conditional fields based on group type
        match self.group_type {
            ProxyGroupType::LoadBalance => {
                field_count += 4; // strategy, url, interval, tolerance
                if !self.lazy {
                    field_count += 1; // lazy
                }
            }
            ProxyGroupType::URLTest | ProxyGroupType::Smart => {
                field_count += 2; // url, interval
                if !self.lazy {
                    field_count += 1; // lazy
                }
                if self.tolerance > 0 {
                    field_count += 1; // tolerance
                }
            }
            ProxyGroupType::Fallback => {
                field_count += 2; // url, interval
                if self.tolerance > 0 {
                    field_count += 1; // tolerance
                }
            }
            _ => {}
        }

        // Add count for other optional fields
        if self.disable_udp {
            field_count += 1;
        }
        if self.persistent {
            field_count += 1;
        }
        if self.evaluate_before_use {
            field_count += 1;
        }

        // Add fields for proxies and provider
        if !self.proxies.is_empty() {
            field_count += 1;
        }
        if !self.using_provider.is_empty() {
            field_count += 1;
        }

        // Create serialization struct
        let mut state = serializer.serialize_struct("ProxyGroup", field_count)?;

        // Always include name
        state.serialize_field("name", &self.name)?;

        // Handle type (with special case for Smart)
        let type_str = if self.group_type == ProxyGroupType::Smart {
            "url-test"
        } else {
            self.type_str()
        };
        state.serialize_field("type", &type_str)?;

        // Add fields based on type
        match self.group_type {
            ProxyGroupType::LoadBalance => {
                // Load balancing specific fields
                state.serialize_field("strategy", &self.strategy_str())?;
                if !self.lazy {
                    state.serialize_field("lazy", &self.lazy)?;
                }
                state.serialize_field("url", &self.url)?;
                if self.interval > 0 {
                    state.serialize_field("interval", &self.interval)?;
                }
                if self.tolerance > 0 {
                    state.serialize_field("tolerance", &self.tolerance)?;
                }
            }
            ProxyGroupType::URLTest | ProxyGroupType::Smart => {
                // URL-test specific fields
                if !self.lazy {
                    state.serialize_field("lazy", &self.lazy)?;
                }
                state.serialize_field("url", &self.url)?;
                if self.interval > 0 {
                    state.serialize_field("interval", &self.interval)?;
                }
                if self.tolerance > 0 {
                    state.serialize_field("tolerance", &self.tolerance)?;
                }
            }
            ProxyGroupType::Fallback => {
                // Fallback specific fields
                state.serialize_field("url", &self.url)?;
                if self.interval > 0 {
                    state.serialize_field("interval", &self.interval)?;
                }
                if self.tolerance > 0 {
                    state.serialize_field("tolerance", &self.tolerance)?;
                }
            }
            _ => {}
        }

        // Add optional common fields
        if self.disable_udp {
            state.serialize_field("disable-udp", &self.disable_udp)?;
        }
        if self.persistent {
            state.serialize_field("persistent", &self.persistent)?;
        }
        if self.evaluate_before_use {
            state.serialize_field("evaluate-before-use", &self.evaluate_before_use)?;
        }

        // Add proxies list if not empty
        if !self.proxies.is_empty() {
            state.serialize_field("proxies", &self.proxies)?;
        }

        // Add provider via "use" field if present
        if !self.using_provider.is_empty() {
            state.serialize_field("use", &self.using_provider)?;
        }

        state.end()
    }
}
