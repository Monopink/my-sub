use serde::Deserialize;
use std::collections::HashMap;

use super::super::ini_bindings::{FromIni, FromIniWithDelimiter};
use crate::models::ruleset::RulesetConfigs;
use crate::models::{ProxyGroupConfigs, RegexMatchConfig, RegexMatchConfigs};
use crate::settings::yaml_deserializer::deserialize_template_args_as_hash_map;
use crate::{settings::import_items, utils::http::parse_proxy, Settings};

// Default value functions
fn default_true() -> bool {
    true
}

/// Rule bases settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RuleBasesSettings {
    pub clash_rule_base: String,
    pub surge_rule_base: String,
    pub surfboard_rule_base: String,
    pub mellow_rule_base: String,
    pub quan_rule_base: String,
    pub quanx_rule_base: String,
    pub loon_rule_base: String,
    pub sssub_rule_base: String,
    pub singbox_rule_base: String,
}

/// Rule generation options
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RuleGenerationSettings {
    #[serde(default = "default_true")]
    pub enable_rule_generator: bool,
    pub overwrite_original_rules: bool,
}

/// Emoji settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct EmojiSettings {
    pub add_emoji: Option<bool>,
    pub remove_old_emoji: Option<bool>,
    pub emoji: Vec<RegexMatchConfig>,
}

/// Filtering settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct FilteringSettings {
    pub include_remarks: Vec<String>,
    pub exclude_remarks: Vec<String>,
}

/// Custom settings
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct CustomSettings {
    // Include the custom settings from RuleBasesSettings
    #[serde(flatten)]
    pub rule_bases: RuleBasesSettings,

    // Rule generation options
    #[serde(flatten)]
    pub rule_generation: RuleGenerationSettings,

    // Emoji settings
    #[serde(flatten)]
    pub emoji_settings: EmojiSettings,

    // Filtering settings
    #[serde(flatten)]
    pub filtering: FilteringSettings,

    // Emoji and rename rules
    #[serde(alias = "emoji")]
    pub emojis: Vec<String>,
    pub rename_nodes: Vec<String>,

    // Custom rulesets and proxy groups
    #[serde(alias = "surge_ruleset")]
    pub rulesets: Vec<String>,
    #[serde(alias = "custom_proxy_group")]
    pub proxy_groups: Vec<String>,
}

/// Main YAML external settings structure
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct YamlExternalSettings {
    pub custom: CustomSettings,
    #[serde(deserialize_with = "deserialize_template_args_as_hash_map")]
    pub tpl_args: Option<HashMap<String, String>>,

    // Processed fields
    #[serde(skip)]
    pub parsed_custom_proxy_groups: ProxyGroupConfigs,
    #[serde(skip)]
    pub parsed_rulesets: RulesetConfigs,
    #[serde(skip)]
    pub parsed_rename: Vec<RegexMatchConfig>,
    #[serde(skip)]
    pub parsed_emojis: Vec<RegexMatchConfig>,
}

impl YamlExternalSettings {
    pub async fn process_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let global = Settings::current();
        let proxy_config = parse_proxy(&global.proxy_config);

        // Process rename nodes
        import_items(
            &mut self.custom.rename_nodes,
            false,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_rename =
            RegexMatchConfigs::from_ini_with_delimiter(&self.custom.rename_nodes, "@");

        // Process emoji rules
        import_items(
            &mut self.custom.emojis,
            false,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_emojis = RegexMatchConfigs::from_ini_with_delimiter(&self.custom.emojis, ",");

        // Process imports for rulesets
        import_items(
            &mut self.custom.rulesets,
            global.api_mode,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_rulesets = RulesetConfigs::from_ini(&self.custom.rulesets);

        // Process imports for proxy groups
        import_items(
            &mut self.custom.proxy_groups,
            global.api_mode,
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_custom_proxy_groups = ProxyGroupConfigs::from_ini(&self.custom.proxy_groups);

        Ok(())
    }
}
