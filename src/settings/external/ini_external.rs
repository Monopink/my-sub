use serde::Deserialize;
use std::collections::HashMap;

use super::super::ini_bindings::{FromIni, FromIniWithDelimiter};
use crate::models::ruleset::RulesetConfigs;
use crate::models::{ProxyGroupConfigs, RegexMatchConfig, RegexMatchConfigs, RulesetConfig};
use crate::settings::{import_items, Settings};
use crate::utils::http::parse_proxy;

/// INI external settings structure
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct IniExternalSettings {
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
    pub enable_rule_generator: bool,
    pub overwrite_original_rules: bool,

    // Emoji options
    pub add_emoji: Option<bool>,
    pub remove_old_emoji: Option<bool>,
    pub emojis: Vec<String>,

    // Filtering options
    pub include_remarks: Vec<String>,
    pub exclude_remarks: Vec<String>,

    // Rulesets and proxy groups (stored as raw strings)
    pub rulesets: Vec<String>,
    pub custom_proxy_groups: Vec<String>,

    // fields
    pub rename_nodes: Vec<String>,
    // Rename rules

    // Template arguments
    pub tpl_args: Option<HashMap<String, String>>,

    // processed fields
    #[serde(skip)]
    pub parsed_custom_proxy_groups: ProxyGroupConfigs,
    #[serde(skip)]
    pub parsed_rulesets: Vec<RulesetConfig>,
    #[serde(skip)]
    pub parsed_rename: Vec<RegexMatchConfig>,
    #[serde(skip)]
    pub parsed_emojis: Vec<RegexMatchConfig>,
}

impl IniExternalSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load settings from INI format
    pub fn load_from_ini(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_section = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
                continue;
            }

            // Check for section header
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = trimmed[1..trimmed.len() - 1].to_string();
                continue;
            }

            // Process key-value pairs
            if let Some(pos) = trimmed.find('=') {
                let key = trimmed[..pos].trim();
                let value = trimmed[pos + 1..].trim();

                match current_section.as_str() {
                    "custom" => self.process_custom_section(key, value),
                    "template" => self.process_template_section(key, value),
                    _ => {} // Ignore unknown sections
                }
            }
        }
        Ok(())
    }

    fn process_custom_section(&mut self, key: &str, value: &str) {
        match key {
            "clash_rule_base" => self.clash_rule_base = value.to_string(),
            "surge_rule_base" => self.surge_rule_base = value.to_string(),
            "surfboard_rule_base" => self.surfboard_rule_base = value.to_string(),
            "mellow_rule_base" => self.mellow_rule_base = value.to_string(),
            "quan_rule_base" => self.quan_rule_base = value.to_string(),
            "quanx_rule_base" => self.quanx_rule_base = value.to_string(),
            "loon_rule_base" => self.loon_rule_base = value.to_string(),
            "sssub_rule_base" => self.sssub_rule_base = value.to_string(),
            "singbox_rule_base" => self.singbox_rule_base = value.to_string(),
            "enable_rule_generator" => {
                self.enable_rule_generator = parse_bool_with_true_default(value)
            }
            "overwrite_original_rules" => self.overwrite_original_rules = parse_bool(value),
            "add_emoji" => self.add_emoji = Some(parse_bool(value)),
            "remove_old_emoji" => self.remove_old_emoji = Some(parse_bool(value)),
            "include_remarks" => {
                self.include_remarks = value.split(',').map(|s| s.trim().to_string()).collect();
            }
            "exclude_remarks" => {
                self.exclude_remarks = value.split(',').map(|s| s.trim().to_string()).collect();
            }
            "ruleset" | "surge_ruleset" => {
                self.rulesets.push(value.to_string());
            }
            "custom_proxy_group" => {
                self.custom_proxy_groups.push(value.to_string());
            }
            "emoji" => {
                self.emojis.push(value.to_string());
            }
            "rename" => {
                self.rename_nodes.push(value.to_string());
            }
            _ => {}
        }
    }

    fn process_template_section(&mut self, key: &str, value: &str) {
        // Initialize tpl_args if it's None
        if self.tpl_args.is_none() {
            self.tpl_args = Some(HashMap::new());
        }

        // Add the key-value pair to the template arguments
        if let Some(ref mut args) = self.tpl_args {
            args.insert(key.to_string(), value.to_string());
        }
    }

    pub async fn process_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let global = Settings::current();
        let proxy_config = parse_proxy(&global.proxy_config);
        // Process rename nodes
        import_items(
            &mut self.rename_nodes,
            false,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_rename = RegexMatchConfigs::from_ini_with_delimiter(&self.rename_nodes, "@");

        // Process emoji rules
        import_items(&mut self.emojis, false, &proxy_config, &global.base_path).await?;
        self.parsed_emojis = RegexMatchConfigs::from_ini_with_delimiter(&self.emojis, ",");

        // Process imports for rulesets
        import_items(
            &mut self.rulesets,
            global.api_mode,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_rulesets = RulesetConfigs::from_ini(&self.rulesets);
        // Process imports for proxy groups
        let mut custom_proxy_groups = self.custom_proxy_groups.clone();
        import_items(
            &mut custom_proxy_groups,
            global.api_mode,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_custom_proxy_groups = ProxyGroupConfigs::from_ini(&custom_proxy_groups);

        Ok(())
    }
}

/// Parse a string as boolean
fn parse_bool(value: &str) -> bool {
    value.to_lowercase() == "true" || value == "1"
}

fn parse_bool_with_true_default(value: &str) -> bool {
    if value.is_empty() {
        true
    } else {
        parse_bool(value)
    }
}
