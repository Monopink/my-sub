use std::collections::HashMap;

use super::Settings;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        cron::CronTaskConfigs, proxy_group_config::ProxyGroupConfigs, ruleset::RulesetConfigs,
        RegexMatchConfigs,
    },
    settings::{
        import_toml::import_toml_items,
        toml_deserializer::{
            deserialize_template_as_template_settings, ProxyGroupConfigInToml,
            RegexMatchRuleInToml, RulesetConfigInToml, TaskConfigInToml,
        },
    },
    utils::http::parse_proxy,
};

// 为toml::Value添加默认值函数
fn default_toml_value() -> toml::Value {
    toml::Value::String(String::new())
}

// 为常用默认值添加函数
fn default_true() -> bool {
    true
}

fn default_empty_string() -> String {
    String::new()
}

fn default_system() -> String {
    "SYSTEM".to_string()
}

fn default_none() -> String {
    "NONE".to_string()
}

fn default_listen_address() -> String {
    "127.0.0.1".to_string()
}

fn default_listen_port() -> u32 {
    25500
}

fn default_max_pending_conns() -> u32 {
    10240
}

fn default_max_concurrent_threads() -> u32 {
    4
}

fn default_info_log_level() -> String {
    "info".to_string()
}

fn default_cache_subscription() -> u32 {
    60
}

fn default_cache_config() -> u32 {
    300
}

fn default_cache_ruleset() -> u32 {
    21600
}

fn default_max_rulesets() -> usize {
    64
}

fn default_max_rules() -> usize {
    32768
}

fn default_max_download_size() -> i64 {
    32 * 1024 * 1024 // 32MB
}
/// User info settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserInfoSettings {
    pub stream_rule: Vec<RegexMatchRuleInToml>,
    pub time_rule: Vec<RegexMatchRuleInToml>,
}

/// Common settings section
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct CommonSettings {
    pub api_mode: bool,
    pub api_access_token: String,
    #[serde(rename = "default_url")]
    pub default_urls: Vec<String>,
    #[serde(default = "default_true")]
    pub enable_insert: bool,
    #[serde(rename = "insert_url")]
    pub insert_urls: Vec<String>,
    #[serde(default = "default_true")]
    pub prepend_insert_url: bool,
    pub exclude_remarks: Vec<String>,
    pub include_remarks: Vec<String>,
    pub enable_filter: bool,
    pub filter_script: String,
    pub default_external_config: String,
    #[serde(default = "default_empty_string")]
    pub base_path: String,
    pub clash_rule_base: String,
    pub surge_rule_base: String,
    pub surfboard_rule_base: String,
    pub mellow_rule_base: String,
    pub quan_rule_base: String,
    pub quanx_rule_base: String,
    pub loon_rule_base: String,
    pub sssub_rule_base: String,
    pub singbox_rule_base: String,
    #[serde(default = "default_system")]
    pub proxy_config: String,
    #[serde(default = "default_system")]
    pub proxy_ruleset: String,
    #[serde(default = "default_none")]
    pub proxy_subscription: String,
    pub append_proxy_type: bool,
    pub reload_conf_on_request: bool,
}

/// Node preferences
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct NodePreferences {
    pub udp_flag: Option<bool>,
    pub tcp_fast_open_flag: Option<bool>,
    pub skip_cert_verify_flag: Option<bool>,
    pub tls13_flag: Option<bool>,
    pub sort_flag: bool,
    pub sort_script: String,
    pub filter_deprecated_nodes: bool,
    #[serde(default = "default_true")]
    pub append_sub_userinfo: bool,
    #[serde(default = "default_true")]
    pub clash_use_new_field_name: bool,
    #[serde(default = "default_empty_string")]
    pub clash_proxies_style: String,
    #[serde(default = "default_empty_string")]
    pub clash_proxy_groups_style: String,
    pub singbox_add_clash_modes: bool,
    pub rename_node: Vec<RegexMatchRuleInToml>,
}

/// Managed config settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ManagedConfigSettings {
    #[serde(default = "default_true")]
    pub write_managed_config: bool,
    #[serde(default = "default_listen_address")]
    pub managed_config_prefix: String,
    #[serde(default = "default_update_interval")]
    pub config_update_interval: u32,
    pub config_update_strict: bool,
    pub quanx_device_id: String,
}

fn default_update_interval() -> u32 {
    86400 // 24 hours
}

/// Surge external proxy settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SurgeExternalProxySettings {
    pub surge_ssr_path: String,
    pub resolve_hostname: bool,
}

/// Emoji settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct EmojiSettings {
    pub add_emoji: bool,
    #[serde(default = "default_true")]
    pub remove_old_emoji: bool,
    pub emoji: Vec<RegexMatchRuleInToml>,
}

/// Ruleset settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct RulesetSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub overwrite_original_rules: bool,
    pub update_ruleset_on_request: bool,
}

/// Template variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub key: String,
    #[serde(default = "default_toml_value")]
    pub value: toml::Value,
}

impl Default for TemplateVariable {
    fn default() -> Self {
        Self {
            key: String::new(),
            value: default_toml_value(),
        }
    }
}

/// Template settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TemplateSettings {
    pub template_path: String,
    pub globals: HashMap<String, String>,
}

/// Alias configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AliasConfig {
    pub uri: String,
    pub target: String,
}

/// Server settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ServerSettings {
    #[serde(default = "default_listen_address")]
    pub listen: String,
    #[serde(default = "default_listen_port")]
    pub port: u32,
    pub serve_file_root: String,
}

/// Advanced settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AdvancedSettings {
    #[serde(default = "default_info_log_level")]
    pub log_level: String,
    pub print_debug_info: bool,
    #[serde(default = "default_max_pending_conns")]
    pub max_pending_connections: u32,
    #[serde(default = "default_max_concurrent_threads")]
    pub max_concurrent_threads: u32,
    #[serde(default = "default_max_rulesets")]
    pub max_allowed_rulesets: usize,
    #[serde(default = "default_max_rules")]
    pub max_allowed_rules: usize,
    #[serde(default = "default_max_download_size")]
    pub max_allowed_download_size: i64,
    pub enable_cache: bool,
    #[serde(default = "default_cache_subscription")]
    pub cache_subscription: u32,
    #[serde(default = "default_cache_config")]
    pub cache_config: u32,
    #[serde(default = "default_cache_ruleset")]
    pub cache_ruleset: u32,
    pub script_clean_context: bool,
    pub async_fetch_ruleset: bool,
    pub skip_failed_links: bool,
}

/// Main TOML settings structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct TomlSettings {
    pub version: i32,
    pub common: CommonSettings,
    pub userinfo: UserInfoSettings,
    #[serde(rename = "node_pref")]
    pub node_pref: NodePreferences,
    #[serde(rename = "managed_config")]
    pub managed_config: ManagedConfigSettings,
    #[serde(rename = "surge_external_proxy")]
    pub surge_external_proxy: SurgeExternalProxySettings,
    pub emojis: EmojiSettings,
    pub ruleset: RulesetSettings,
    pub rulesets: Vec<RulesetConfigInToml>,
    #[serde(rename = "custom_groups")]
    pub custom_proxy_groups: Vec<ProxyGroupConfigInToml>,
    #[serde(
        rename = "template.globals",
        deserialize_with = "deserialize_template_as_template_settings"
    )]
    pub template: TemplateSettings,
    pub aliases: Vec<AliasConfig>,
    pub tasks: Vec<TaskConfigInToml>,
    pub server: ServerSettings,
    pub advanced: AdvancedSettings,
    // Internal fields not present in TOML file
    #[serde(skip)]
    pub parsed_rename: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_stream_rule: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_time_rule: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_emoji_rules: RegexMatchConfigs,
    #[serde(skip)]
    pub parsed_proxy_group: ProxyGroupConfigs,
    #[serde(skip)]
    pub parsed_ruleset: RulesetConfigs,
    #[serde(skip)]
    pub parsed_tasks: CronTaskConfigs,
}

impl TomlSettings {
    pub async fn process_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let global = Settings::current();
        let proxy_config = parse_proxy(&self.common.proxy_config);

        // Process rename nodes
        import_toml_items(
            &mut self.node_pref.rename_node,
            false,
            "rename_node",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_rename = self
            .node_pref
            .rename_node
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process stream rules
        import_toml_items(
            &mut self.userinfo.stream_rule,
            false,
            "stream_rule",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_stream_rule = self
            .userinfo
            .stream_rule
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process time rules
        import_toml_items(
            &mut self.userinfo.time_rule,
            false,
            "time_rule",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_time_rule = self
            .userinfo
            .time_rule
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process emoji rules
        import_toml_items(
            &mut self.emojis.emoji,
            false,
            "emoji",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_emoji_rules = self.emojis.emoji.iter().map(|r| r.clone().into()).collect();

        // Process rulesets
        import_toml_items(
            &mut self.rulesets,
            global.api_mode,
            "rulesets",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;

        // Check ruleset count limit
        if global.max_allowed_rulesets > 0 && self.rulesets.len() > global.max_allowed_rulesets {
            return Err(format!(
                "Number of rulesets exceeds the maximum allowed: {}",
                global.max_allowed_rulesets
            )
            .into());
        }

        self.parsed_ruleset = self.rulesets.iter().map(|r| r.clone().into()).collect();

        // Process proxy groups
        import_toml_items(
            &mut self.custom_proxy_groups,
            global.api_mode,
            "custom_groups",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_proxy_group = self
            .custom_proxy_groups
            .iter()
            .map(|r| r.clone().into())
            .collect();

        // Process tasks
        import_toml_items(
            &mut self.tasks,
            false,
            "tasks",
            &proxy_config,
            &self.common.base_path,
        )
        .await?;
        self.parsed_tasks = self.tasks.iter().map(|r| r.clone().into()).collect();

        Ok(())
    }
}
