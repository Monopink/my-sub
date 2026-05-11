use crate::models::{ProxyGroupConfig, ProxyGroupType};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

/// Serialize a ProxyGroupConfig for Clash output
///
/// This implementation follows the serialization logic from the C++ code in subexport.cpp
/// and ensures compatibility with the existing Clash configuration generation.
///
/// Specific serialization rules:
/// - All groups have "name" and "type" fields
/// - Type-specific fields are only included for relevant group types
/// - Fields with default values are omitted
/// - Empty lists are omitted
/// - Special handling for DIRECT proxy and Smart group type
pub fn serialize_proxy_group<S>(
    group: &ProxyGroupConfig,
    proxies: &[String],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(None)?;

    // Always include name and type
    map.serialize_entry("name", &group.name)?;

    // Handle type (special case for Smart type which becomes url-test)
    let type_str = if group.group_type == ProxyGroupType::Smart {
        "url-test"
    } else {
        group.type_str()
    };
    map.serialize_entry("type", &type_str)?;

    // Add type-specific fields
    match group.group_type {
        ProxyGroupType::Select | ProxyGroupType::Relay => {
            // No special fields for these types
        }
        ProxyGroupType::LoadBalance => {
            // Add strategy for load balancing
            map.serialize_entry("strategy", &group.strategy_str())?;

            // If not lazy, include the flag (false is default, so omit if true)
            if !group.lazy {
                map.serialize_entry("lazy", &group.lazy)?;
            }

            // Add URL test fields
            map.serialize_entry("url", &group.url)?;

            if group.interval > 0 {
                map.serialize_entry("interval", &group.interval)?;
            }

            if group.tolerance > 0 {
                map.serialize_entry("tolerance", &group.tolerance)?;
            }
        }
        ProxyGroupType::Smart | ProxyGroupType::URLTest => {
            // If not lazy, include the flag (true is default, so only include if false)
            if !group.lazy {
                map.serialize_entry("lazy", &group.lazy)?;
            }

            // Add URL test fields
            map.serialize_entry("url", &group.url)?;

            if group.interval > 0 {
                map.serialize_entry("interval", &group.interval)?;
            }

            if group.tolerance > 0 {
                map.serialize_entry("tolerance", &group.tolerance)?;
            }
        }
        ProxyGroupType::Fallback => {
            // Add URL test fields
            map.serialize_entry("url", &group.url)?;

            if group.interval > 0 {
                map.serialize_entry("interval", &group.interval)?;
            }

            if group.tolerance > 0 {
                map.serialize_entry("tolerance", &group.tolerance)?;
            }
        }
        ProxyGroupType::SSID => {
            // Not fully implemented in the original code
            // Skip for now or add SSID-specific fields if needed
        }
    }

    // Add optional common fields if they're not default values
    if group.disable_udp {
        map.serialize_entry("disable-udp", &group.disable_udp)?;
    }

    if group.persistent {
        map.serialize_entry("persistent", &group.persistent)?;
    }

    if group.evaluate_before_use {
        map.serialize_entry("evaluate-before-use", &group.evaluate_before_use)?;
    }

    // Add provider via "use" field if present, or filtered nodes
    if !group.using_provider.is_empty() {
        let provider_seq: Vec<&String> = group.using_provider.iter().collect();
        map.serialize_entry("use", &provider_seq)?;
    } else {
        // Add proxies list if we have any
        if !proxies.is_empty() {
            map.serialize_entry("proxies", &proxies)?;
        } else {
            // Add DIRECT if empty, as seen in the original code
            map.serialize_entry("proxies", &["DIRECT"])?;
        }
    }

    map.end()
}

/// ClashProxyGroup represents a serializable proxy group for Clash configurations
///
/// This struct is designed to be serialized directly to YAML for Clash configurations.
/// It contains all necessary fields with proper serde annotations to control when
/// fields are included in the output.
#[derive(Debug, Serialize)]
pub struct ClashProxyGroup {
    /// Name of the proxy group
    pub name: String,

    /// Type of the proxy group (select, url-test, fallback, load-balance, etc.)
    #[serde(rename = "type")]
    pub group_type: String,

    /// List of proxy names in this group
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub proxies: Vec<String>,

    /// List of provider names used by this group
    #[serde(rename = "use", skip_serializing_if = "Vec::is_empty")]
    pub using_provider: Vec<String>,

    /// URL for testing (for url-test, fallback, and load-balance types)
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,

    /// Interval in seconds between tests (for url-test, fallback, and load-balance types)
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub interval: u32,

    /// Timeout in seconds for tests
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub timeout: u32,

    /// Tolerance value for tests
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub tolerance: u32,

    /// Strategy for load balancing (for load-balance type)
    #[serde(skip_serializing_if = "String::is_empty")]
    pub strategy: String,

    /// Whether to use lazy loading
    #[serde(skip_serializing_if = "is_true")]
    pub lazy: bool,

    /// Whether to disable UDP support
    #[serde(rename = "disable-udp", skip_serializing_if = "is_false")]
    pub disable_udp: bool,

    /// Whether to persist connections
    #[serde(skip_serializing_if = "is_false")]
    pub persistent: bool,

    /// Whether to evaluate before use
    #[serde(rename = "evaluate-before-use", skip_serializing_if = "is_false")]
    pub evaluate_before_use: bool,
}

// Helper functions for serde skip conditions
fn is_zero_u32(val: &u32) -> bool {
    *val == 0
}

fn is_true(val: &bool) -> bool {
    *val
}

fn is_false(val: &bool) -> bool {
    !*val
}

impl From<&ProxyGroupConfig> for ClashProxyGroup {
    fn from(config: &ProxyGroupConfig) -> Self {
        // Special handling for Smart type which becomes url-test
        let type_str = if config.group_type == ProxyGroupType::Smart {
            "url-test".to_string()
        } else {
            config.type_str().to_string()
        };

        // Create a basic proxy group with common fields
        let mut clash_group = ClashProxyGroup {
            name: config.name.clone(),
            group_type: type_str,
            proxies: config.proxies.clone(),
            using_provider: config.using_provider.clone(),
            url: String::new(),
            interval: 0,
            timeout: 0,
            tolerance: 0,
            strategy: String::new(),
            lazy: true, // Default to true
            disable_udp: config.disable_udp,
            persistent: config.persistent,
            evaluate_before_use: config.evaluate_before_use,
        };

        // Add type-specific fields
        match config.group_type {
            ProxyGroupType::LoadBalance => {
                clash_group.strategy = config.strategy_str().to_string();
                clash_group.lazy = config.lazy;
                clash_group.url = config.url.clone();
                clash_group.interval = config.interval;
                clash_group.tolerance = config.tolerance;
            }
            ProxyGroupType::URLTest | ProxyGroupType::Smart | ProxyGroupType::Fallback => {
                clash_group.url = config.url.clone();
                clash_group.interval = config.interval;
                clash_group.tolerance = config.tolerance;

                // Only URLTest and Smart use lazy loading
                if matches!(
                    config.group_type,
                    ProxyGroupType::URLTest | ProxyGroupType::Smart
                ) {
                    clash_group.lazy = config.lazy;
                }
            }
            _ => {} // No special fields for other types
        }

        // If proxies list is empty and no providers, add DIRECT
        if clash_group.proxies.is_empty() && clash_group.using_provider.is_empty() {
            clash_group.proxies = vec!["DIRECT".to_string()];
        }

        clash_group
    }
}

/// Converts ProxyGroupConfigs to a vector of ClashProxyGroup objects
pub fn convert_proxy_groups(
    group_configs: &[ProxyGroupConfig],
    filtered_nodes_map: Option<&HashMap<String, Vec<String>>>,
) -> Vec<ClashProxyGroup> {
    let mut clash_groups = Vec::with_capacity(group_configs.len());

    for group in group_configs {
        let mut clash_group = ClashProxyGroup::from(group);

        // Replace proxies with filtered nodes if available
        if let Some(filtered_map) = filtered_nodes_map {
            if let Some(filtered_nodes) = filtered_map.get(&group.name) {
                clash_group.proxies = filtered_nodes.clone();

                // If proxies list is empty and no providers, add DIRECT
                if clash_group.proxies.is_empty() && clash_group.using_provider.is_empty() {
                    clash_group.proxies = vec!["DIRECT".to_string()];
                }
            }
        }

        clash_groups.push(clash_group);
    }

    clash_groups
}

/// Example function showing how to use the ClashProxyGroup
///
/// This demonstrates how to create and serialize proxy group configurations
/// for Clash.
///
/// # Example
///
/// ```rust
/// use crate::generator::yaml::proxy_group_output::{ClashProxyGroup, convert_proxy_groups};
/// use crate::models::{ProxyGroupConfig, ProxyGroupType};
/// use std::collections::HashMap;
///
/// // Create some proxy groups
/// let mut groups = Vec::new();
///
/// // Add a simple selector group
/// let mut selector = ProxyGroupConfig::new("Proxy".to_string(), ProxyGroupType::Select);
/// selector.proxies = vec!["Server1".to_string(), "Server2".to_string()];
/// groups.push(selector);
///
/// // Add a URL test group
/// let mut url_test = ProxyGroupConfig::new("Auto".to_string(), ProxyGroupType::URLTest);
/// url_test.url = "http://www.gstatic.com/generate_204".to_string();
/// url_test.interval = 300;
/// url_test.proxies = vec!["Server1".to_string(), "Server2".to_string()];
/// groups.push(url_test);
///
/// // Create a Fallback group
/// let mut fallback_group = ProxyGroupConfig::new("Fallback".to_string(), ProxyGroupType::Fallback);
/// fallback_group.url = "http://www.gstatic.com/generate_204".to_string();
/// fallback_group.interval = 300;
/// fallback_group.proxies = vec!["Hong Kong".to_string(), "Singapore".to_string(), "US".to_string()];
/// groups.push(fallback_group);
///
/// // Convert to Clash format
/// let clash_groups = convert_proxy_groups(&groups, None);
///
/// // Serialize to YAML
/// let yaml = serde_yaml::to_string(&clash_groups).unwrap();
/// println!("{}", yaml);
/// ```
pub fn example_clash_groups() -> Vec<ClashProxyGroup> {
    // Create sample proxy groups
    let mut groups = Vec::new();

    // Create a Select group (Manual selection)
    let mut select_group = ProxyGroupConfig::new("Proxy".to_string(), ProxyGroupType::Select);
    select_group.proxies = vec![
        "Hong Kong".to_string(),
        "Singapore".to_string(),
        "US".to_string(),
    ];
    groups.push(select_group);

    // Create a URL-Test group (Auto selection by latency)
    let mut urltest_group = ProxyGroupConfig::new("Auto".to_string(), ProxyGroupType::URLTest);
    urltest_group.url = "http://www.gstatic.com/generate_204".to_string();
    urltest_group.interval = 300;
    urltest_group.proxies = vec!["Hong Kong".to_string(), "Singapore".to_string()];
    groups.push(urltest_group);

    // Create a Fallback group
    let mut fallback_group =
        ProxyGroupConfig::new("Fallback".to_string(), ProxyGroupType::Fallback);
    fallback_group.url = "http://www.gstatic.com/generate_204".to_string();
    fallback_group.interval = 300;
    fallback_group.proxies = vec![
        "Hong Kong".to_string(),
        "Singapore".to_string(),
        "US".to_string(),
    ];
    groups.push(fallback_group);

    // Convert to Clash format
    convert_proxy_groups(&groups, None)
}
