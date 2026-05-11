use serde::Deserialize;
use std::collections::HashMap;

use crate::models::{ProxyGroupConfigs, RegexMatchConfig, RulesetConfig};
use crate::settings::import_toml::import_toml_items;
use crate::settings::toml_deserializer::*;
use crate::settings::Settings;
use crate::utils::http::parse_proxy;

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
    pub emoji: Vec<RegexMatchRuleInToml>,
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
    pub rename_node: Vec<RegexMatchRuleInToml>,

    // Custom rulesets and proxy groups
    pub custom_rulesets: Vec<RulesetConfigInToml>,
    pub custom_proxy_groups: Vec<ProxyGroupConfigInToml>,
}

/// Main TOML external settings structure
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct TomlExternalSettings {
    pub custom: CustomSettings,
    #[serde(deserialize_with = "deserialize_template_args_as_hash_map")]
    pub tpl_args: Option<HashMap<String, String>>,

    // Processed fields
    #[serde(skip)]
    pub parsed_custom_proxy_groups: ProxyGroupConfigs,
    #[serde(skip)]
    pub parsed_rulesets: Vec<RulesetConfig>,
    #[serde(skip)]
    pub parsed_rename: Vec<RegexMatchConfig>,
    #[serde(skip)]
    pub parsed_emojis: Vec<RegexMatchConfig>,
}

impl TomlExternalSettings {
    pub async fn process_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let global = Settings::current();
        let proxy_config = parse_proxy(&global.proxy_config);

        import_toml_items(
            &mut self.custom.rename_node,
            false,
            "rename_node",
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_rename = self
            .custom
            .rename_node
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process emoji rules
        import_toml_items(
            &mut self.custom.emoji_settings.emoji,
            false,
            "emoji",
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_emojis = self
            .custom
            .emoji_settings
            .emoji
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process imports for rulesets
        import_toml_items(
            &mut self.custom.custom_rulesets,
            global.api_mode,
            "rulesets",
            &proxy_config,
            &global.base_path,
        )
        .await?;
        if global.max_allowed_rulesets > 0
            && self.custom.custom_rulesets.len() > global.max_allowed_rulesets
        {
            return Err(format!(
                "Number of rulesets exceeds the maximum allowed: {}",
                global.max_allowed_rulesets
            )
            .into());
        }

        self.parsed_rulesets = self
            .custom
            .custom_rulesets
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process imports for proxy groups
        import_toml_items(
            &mut self.custom.custom_proxy_groups,
            global.api_mode,
            "custom_group",
            &proxy_config,
            &global.base_path,
        )
        .await?;
        self.parsed_custom_proxy_groups = self
            .custom
            .custom_proxy_groups
            .iter()
            .map(|r| r.clone().into())
            .collect();

        Ok(())
    }
}
