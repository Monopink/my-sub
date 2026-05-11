use std::collections::HashMap;

use crate::models::RegexMatchConfigs;
use crate::utils::http::{parse_proxy, ProxyConfig};
use crate::Settings;
use case_insensitive_string::CaseInsensitiveString;

/// Rust equivalent of the parse_settings struct in C++
/// Used for controlling the behavior of parsing functions
#[derive(Debug, Clone)]
pub struct ParseSettings {
    /// Proxy to use for downloading subscriptions
    pub proxy: ProxyConfig,

    /// Array of remarks to exclude
    pub exclude_remarks: Option<Vec<String>>,

    /// Array of remarks to include
    pub include_remarks: Option<Vec<String>>,

    /// Rules for stream matching
    pub stream_rules: Option<RegexMatchConfigs>,

    /// Rules for time matching
    pub time_rules: Option<RegexMatchConfigs>,

    /// Subscription information
    pub sub_info: Option<String>,

    /// Whether operations requiring authorization are allowed
    pub authorized: bool,

    /// HTTP request headers
    pub request_header: Option<HashMap<CaseInsensitiveString, String>>,

    /// JavaScript runtime - optional depending on feature flags
    #[cfg(feature = "js-runtime")]
    pub js_runtime: Option<()>, // Placeholder for actual JS runtime type

    /// JavaScript context - optional depending on feature flags
    #[cfg(feature = "js-runtime")]
    pub js_context: Option<()>, // Placeholder for actual JS context type
}

impl Default for ParseSettings {
    fn default() -> Self {
        // Get global settings
        let settings = Settings::current();

        ParseSettings {
            proxy: parse_proxy(&settings.proxy_subscription),
            exclude_remarks: if settings.exclude_remarks.is_empty() {
                None
            } else {
                Some(settings.exclude_remarks.clone())
            },
            include_remarks: if settings.include_remarks.is_empty() {
                None
            } else {
                Some(settings.include_remarks.clone())
            },
            stream_rules: None, // TODO: Get from global settings
            time_rules: None,   // TODO: Get from global settings
            sub_info: None,
            authorized: !settings.api_access_token.is_empty(),
            request_header: None,
            #[cfg(feature = "js-runtime")]
            js_runtime: None,
            #[cfg(feature = "js-runtime")]
            js_context: None,
        }
    }
}

/// Create a new ParseSettings instance with defaults from global settings
pub fn create_parse_settings() -> ParseSettings {
    ParseSettings::default()
}

/// Create a new ParseSettings instance with authorization
pub fn create_authorized_settings() -> ParseSettings {
    let mut settings = ParseSettings::default();
    settings.authorized = true;
    settings
}
