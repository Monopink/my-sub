use crate::models::Proxy;
use crate::parser::settings::ParseSettings;
use crate::parser::subparser::add_nodes;
use std::collections::HashMap;

/// Options for parsing subscriptions
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Remarks to include in parsing
    pub include_remarks: Vec<String>,

    /// Remarks to exclude from parsing
    pub exclude_remarks: Vec<String>,

    /// Whether the request is authorized
    pub authorized: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            include_remarks: Vec::new(),
            exclude_remarks: Vec::new(),
            authorized: false,
        }
    }
}

impl Clone for ParseOptions {
    fn clone(&self) -> Self {
        Self {
            include_remarks: self.include_remarks.clone(),
            exclude_remarks: self.exclude_remarks.clone(),
            authorized: self.authorized,
        }
    }
}

/// Parse a subscription URL and return a vector of proxies
///
/// # Arguments
/// * `url` - The subscription URL to parse
/// * `options` - Options for parsing
///
/// # Returns
/// * `Ok(Vec<Proxy>)` - The parsed proxies
/// * `Err(String)` - Error message if parsing fails
pub fn parse_subscription(url: &str, options: ParseOptions) -> Result<Vec<Proxy>, String> {
    // Create a new parse settings instance
    let mut parse_settings = ParseSettings::default();

    // Set options from the provided config
    if !options.include_remarks.is_empty() {
        parse_settings.include_remarks = Some(options.include_remarks.clone());
    }

    if !options.exclude_remarks.is_empty() {
        parse_settings.exclude_remarks = Some(options.exclude_remarks.clone());
    }

    parse_settings.authorized = options.authorized;

    // Create a vector to hold the nodes
    let mut nodes = Vec::new();

    // Call add_nodes to do the actual parsing
    // We use group_id = 0 since we don't care about it in this context
    add_nodes(url.to_string(), &mut nodes, 0, &mut parse_settings)?;

    Ok(nodes)
}
