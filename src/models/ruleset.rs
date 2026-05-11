use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Enum defining the type of ruleset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RulesetType {
    Surge,
    Quanx,
    ClashDomain,
    ClashIpcidr,
    ClashClassical,
}

impl Default for RulesetType {
    fn default() -> Self {
        RulesetType::Surge
    }
}

/// Mapping from URL prefix to ruleset type
pub type RulesetMapping = HashMap<String, RulesetType>;

/// Available ruleset types with their prefixes
pub static RULESET_TYPES: once_cell::sync::Lazy<RulesetMapping> =
    once_cell::sync::Lazy::new(|| {
        let mut types = RulesetMapping::new();
        types.insert("clash-domain:".to_string(), RulesetType::ClashDomain);
        types.insert("clash-ipcidr:".to_string(), RulesetType::ClashIpcidr);
        types.insert("clash-classical:".to_string(), RulesetType::ClashClassical);
        types.insert("quanx:".to_string(), RulesetType::Quanx);
        types.insert("surge:".to_string(), RulesetType::Surge);
        types
    });

/// Find a ruleset type based on a URL
///
/// Similar to the C++ implementation, this function looks for a matching
/// ruleset type based on the prefix of a URL
pub fn get_ruleset_type_from_url(url: &str) -> Option<RulesetType> {
    for (prefix, ruleset_type) in RULESET_TYPES.iter() {
        if url.starts_with(prefix) {
            return Some(ruleset_type.clone());
        }
    }
    None
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RulesetConfig {
    pub group: String,
    pub url: String,
    pub interval: u32,
}

pub type RulesetConfigs = Vec<RulesetConfig>;

/// Represents a ruleset with its metadata and content
/// Matches the C++ struct RulesetContent:
/// ```cpp
/// struct RulesetContent {
///     std::string rule_group;
///     std::string rule_path;
///     std::string rule_path_typed;
///     int rule_type = RULESET_SURGE;
///     std::shared_future<std::string> rule_content;
///     int update_interval = 0;
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RulesetContent {
    pub group: String,           // rule_group in C++
    pub rule_path: String,       // rule_path in C++
    pub rule_path_typed: String, // rule_path_typed in C++
    pub rule_type: RulesetType,  // rule_type in C++

    // Similar to std::shared_future<std::string> in C++
    // Arc provides shared ownership, RwLock provides interior mutability,
    // Option allows for content to be present or not
    pub rule_content: Arc<RwLock<Option<String>>>,

    pub update_interval: u32, // update_interval in C++
}

impl RulesetContent {
    /// Create a new empty ruleset
    pub fn new(rule_path: &str, group: &str) -> Self {
        RulesetContent {
            group: group.to_string(),
            rule_path: rule_path.to_string(),
            rule_path_typed: rule_path.to_string(),
            rule_type: RulesetType::default(),
            rule_content: Arc::new(RwLock::new(None)),
            update_interval: 0,
        }
    }

    /// Get rule content - simulates the std::shared_future<std::string> rule_content.get() in C++
    /// Returns a reference to the actual content or an empty string if not available
    pub fn get_rule_content(&self) -> String {
        // Try to read the content, return empty string if lock can't be acquired or content is None
        match self.rule_content.read() {
            Ok(guard) => match &*guard {
                Some(content) => content.clone(),
                None => String::new(),
            },
            Err(_) => String::new(),
        }
    }

    /// Set the rule content
    /// Simulates setting the promise value that would fulfill the future in C++
    pub fn set_rule_content(&mut self, content: &str) {
        if let Ok(mut guard) = self.rule_content.write() {
            *guard = Some(content.to_string());
        }
    }

    /// Check if rule content has been set
    /// Simulates std::shared_future::valid() in C++
    pub fn has_rule_content(&self) -> bool {
        match self.rule_content.read() {
            Ok(guard) => guard.is_some(),
            Err(_) => false,
        }
    }
}

/// Parse a ruleset file
pub fn parse_ruleset(content: &str, group: &str) -> RulesetContent {
    let mut ruleset = RulesetContent::new("", group);
    ruleset.set_rule_content(content);
    ruleset
}
