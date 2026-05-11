use serde_yaml;
use std::collections::HashMap;
use toml;

use crate::models::{ProxyGroupConfig, RegexMatchConfig, RulesetConfig};
use crate::settings::Settings;
use crate::utils::file::load_content_async;
// TODO: Implement template rendering module similar to C++ render_template function

use super::ini_external::IniExternalSettings;
use super::toml_external::TomlExternalSettings;
use super::yaml_external::YamlExternalSettings;

/// External configuration structure with flattened fields
#[derive(Debug, Clone, Default)]
pub struct ExternalSettings {
    // Rule bases
    pub clash_rule_base: String,
    pub surge_rule_base: String,
    pub surfboard_rule_base: String,
    pub mellow_rule_base: String,
    pub quan_rule_base: String,
    pub quanx_rule_base: String,
    pub loon_rule_base: String,
    pub sssub_rule_base: String,
    pub singbox_rule_base: String,

    // Rule generation options
    pub enable_rule_generator: Option<bool>,
    pub overwrite_original_rules: Option<bool>,

    // Emoji settings
    pub add_emoji: Option<bool>,
    pub remove_old_emoji: Option<bool>,
    pub emojis: Vec<RegexMatchConfig>,

    // Filtering
    pub include_remarks: Vec<String>,
    pub exclude_remarks: Vec<String>,
    // #[serde(default, deserialize_with = "deserialize_rulesets")]
    pub custom_rulesets: Vec<RulesetConfig>,
    pub custom_proxy_groups: Vec<ProxyGroupConfig>,

    // Node operations
    pub rename_nodes: Vec<RegexMatchConfig>,

    // Template arguments
    pub tpl_args: Option<HashMap<String, String>>,
}

impl ExternalSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load external configuration from file or URL
    // pub fn load_from_file_sync(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     // Load content from file or URL
    //     let _content = load_content(path)?;

    //     Self::parse_content(&_content)
    // }

    /// Load external configuration from file or URL asynchronously
    pub async fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Load content from file or URL asynchronously
        let _content = load_content_async(path).await?;

        Self::parse_content(&_content).await
    }

    /// Parse the content and return an ExternalSettings object
    async fn parse_content(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement template rendering here
        // In C++: if(render_template(config, *ext.tpl_args, base_content, global.templatePath) != 0)
        //           base_content = config;

        // Try YAML format first
        if content.contains("custom:") {
            let mut yaml_settings: YamlExternalSettings = serde_yaml::from_str(content)?;
            yaml_settings.process_imports().await?;
            // Convert to ExternalSettings
            let config = Self::from(yaml_settings);
            return Ok(config);
        }

        if toml::from_str::<toml::Value>(content).is_ok() {
            let mut toml_settings: TomlExternalSettings = toml::from_str(content)?;
            toml_settings.process_imports().await?;
            // Convert to ExternalSettings
            let config = Self::from(toml_settings);
            return Ok(config);
        }

        // Fall back to INI format
        let mut ini_settings = IniExternalSettings::new();
        match ini_settings.load_from_ini(content) {
            Ok(_) => {
                // Process any imports
                ini_settings.process_imports().await?;
                // Convert to ExternalSettings
                let config = Self::from(ini_settings);
                return Ok(config);
            }
            Err(e) => Err(format!("Failed to parse external config as INI: {}", e).into()),
        }
    }

    /// Validate rulesets count
    pub fn validate_rulesets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = Settings::current();
        if settings.max_allowed_rulesets > 0
            && self.custom_rulesets.len() > settings.max_allowed_rulesets
        {
            return Err("Ruleset count in external config has exceeded limit.".into());
        }
        Ok(())
    }
}
