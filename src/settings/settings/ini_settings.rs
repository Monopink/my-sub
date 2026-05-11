use super::super::ini_bindings::{FromIni, FromIniWithDelimiter};
use crate::utils::http::parse_proxy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::{
    models::{
        cron::CronTaskConfigs, ruleset::RulesetConfigs, ProxyGroupConfigs, RegexMatchConfigs,
    },
    settings::import_items,
};

/// Settings structure to hold global configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct IniSettings {
    // Common settings
    #[serde(default)]
    pub api_mode: bool,
    #[serde(default)]
    pub api_access_token: String,

    #[serde(default)]
    pub default_url: String,
    #[serde(default)]
    pub insert_url: String,
    #[serde(default)]
    pub enable_insert: bool,
    #[serde(default)]
    pub prepend_insert_url: bool,

    #[serde(default)]
    pub include_remarks: Vec<String>,
    #[serde(default)]
    pub exclude_remarks: Vec<String>,
    #[serde(default)]
    pub default_ext_config: String,

    #[serde(default = "default_listen_address")]
    pub listen_address: String,
    #[serde(default = "default_listen_port")]
    pub listen_port: u32,
    #[serde(default)]
    pub managed_config_prefix: String,
    #[serde(default = "default_max_pending_conns")]
    pub max_pending_conns: u32,
    #[serde(default = "default_max_concur_threads")]
    pub max_concur_threads: u32,
    #[serde(default)]
    pub prepend_insert: bool,
    #[serde(default)]
    pub skip_failed_links: bool,
    #[serde(default)]
    pub write_managed_config: bool,
    #[serde(default = "default_true")]
    pub enable_rule_gen: bool,
    #[serde(default)]
    pub update_ruleset_on_request: bool,
    #[serde(default)]
    pub overwrite_original_rules: bool,
    #[serde(default)]
    pub print_dbg_info: bool,
    #[serde(default = "default_true")]
    pub append_sub_userinfo: bool,
    #[serde(default)]
    pub async_fetch_ruleset: bool,
    #[serde(default)]
    pub surge_resolve_hostname: bool,
    pub base_path: String,
    pub custom_group: String,
    #[serde(default = "default_log_level")]
    pub log_level: u32,
    #[serde(default = "default_max_download_size")]
    pub max_allowed_download_size: i64,
    pub template_path: String,
    #[serde(default)]
    pub template_vars: HashMap<String, String>,

    // Generator settings
    #[serde(default)]
    pub generator_mode: bool,
    pub generate_profiles: String,

    // Preferences
    #[serde(default)]
    pub reload_conf_on_request: bool,
    #[serde(default)]
    pub add_emoji: bool,
    #[serde(default)]
    pub remove_emoji: bool,
    #[serde(default)]
    pub emoji_rules: Vec<String>,

    #[serde(default)]
    pub append_type: bool,
    #[serde(default = "default_true")]
    pub filter_deprecated: bool,
    pub udp_flag: Option<bool>,
    pub tfo_flag: Option<bool>,
    pub skip_cert_verify: Option<bool>,
    pub tls13_flag: Option<bool>,
    #[serde(default)]
    pub enable_sort: bool,
    #[serde(default)]
    pub update_strict: bool,
    #[serde(default = "default_true")]
    pub clash_use_new_field: bool,
    #[serde(default)]
    pub singbox_add_clash_modes: bool,
    #[serde(default)]
    pub rename_node: Vec<String>,
    #[serde(default)]
    pub stream_rule: Vec<String>,
    #[serde(default)]
    pub time_rule: Vec<String>,

    pub clash_proxies_style: String,
    pub clash_proxy_groups_style: String,
    pub proxy_config: String,
    pub proxy_ruleset: String,
    pub proxy_subscription: String,
    #[serde(default)]
    pub update_interval: u32,
    pub sort_script: String,

    pub enable_filter: bool,
    pub filter_script: String,

    // Base configs
    pub clash_base: String,
    pub surge_base: String,
    pub surfboard_base: String,
    pub mellow_base: String,
    pub quan_base: String,
    pub quanx_base: String,
    pub loon_base: String,
    pub ssub_base: String,
    pub singbox_base: String,
    pub surge_ssr_path: String,
    pub quanx_dev_id: String,

    // Cache system
    #[serde(default)]
    pub enable_cache: bool,
    #[serde(default)]
    pub serve_cache_on_fetch_fail: bool,
    #[serde(default = "default_cache_subscription")]
    pub cache_subscription: u32,
    #[serde(default = "default_cache_config")]
    pub cache_config: u32,
    #[serde(default = "default_cache_ruleset")]
    pub cache_ruleset: u32,

    // Limits
    #[serde(default = "default_max_rulesets")]
    pub max_allowed_rulesets: usize,
    #[serde(default = "default_max_rules")]
    pub max_allowed_rules: usize,
    #[serde(default)]
    pub script_clean_context: bool,

    // Cron system
    #[serde(default)]
    pub enable_cron: bool,
    #[serde(default)]
    pub cron_tasks: Vec<String>,

    // Custom rulesets and groups
    #[serde(default)]
    pub rulesets: Vec<String>,
    #[serde(default)]
    pub custom_proxy_group: Vec<String>,

    // Webserver settings
    #[serde(default)]
    pub serve_file: bool,
    pub serve_file_root: String,

    // Aliases
    #[serde(default)]
    pub aliases: HashMap<String, String>,

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

// Default value functions
fn default_listen_address() -> String {
    "127.0.0.1".to_string()
}

fn default_listen_port() -> u32 {
    25500
}

fn default_max_pending_conns() -> u32 {
    10240
}

fn default_max_concur_threads() -> u32 {
    4
}

fn default_true() -> bool {
    true
}

fn default_log_level() -> u32 {
    1 // LOG_LEVEL_INFO
}

fn default_max_download_size() -> i64 {
    32 * 1024 * 1024 // 32MB
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

impl IniSettings {
    /// Create a new settings instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Process imports in configuration
    pub async fn process_imports(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let proxy_config = parse_proxy(&self.proxy_config);

        // Process rule rename_node
        import_items(&mut self.rename_node, false, &proxy_config, &self.base_path).await?;
        self.parsed_rename = RegexMatchConfigs::from_ini_with_delimiter(&self.rename_node, "@");

        // Process stream rules
        import_items(&mut self.stream_rule, false, &proxy_config, &self.base_path).await?;
        self.parsed_stream_rule =
            RegexMatchConfigs::from_ini_with_delimiter(&self.stream_rule, "|");

        // Process time rules
        import_items(&mut self.time_rule, false, &proxy_config, &self.base_path).await?;
        self.parsed_time_rule = RegexMatchConfigs::from_ini_with_delimiter(&self.time_rule, "|");

        // Process emoji rules
        import_items(&mut self.emoji_rules, false, &proxy_config, &self.base_path).await?;
        self.parsed_emoji_rules =
            RegexMatchConfigs::from_ini_with_delimiter(&self.emoji_rules, ",");

        // Process custom_proxy_group
        import_items(
            &mut self.custom_proxy_group,
            false,
            &proxy_config,
            &self.base_path,
        )
        .await?;
        self.parsed_proxy_group = ProxyGroupConfigs::from_ini(&self.custom_proxy_group);

        // Process rulesets
        import_items(&mut self.rulesets, false, &proxy_config, &self.base_path).await?;
        self.parsed_ruleset = RulesetConfigs::from_ini(&self.rulesets);

        // Process cron tasks
        import_items(&mut self.cron_tasks, false, &proxy_config, &self.base_path).await?;
        self.parsed_tasks = CronTaskConfigs::from_ini(&self.cron_tasks);

        Ok(())
    }

    /// Load settings from file or URL
    pub async fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let mut settings = Self::default();
        settings.load_from_ini(&content)?;

        // Process any imports in the configuration
        settings.process_imports().await?;

        // Ensure listen_address is not empty
        if settings.listen_address.is_empty() {
            settings.listen_address = default_listen_address();
        }

        Ok(settings)
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

                // Process sections in the same order as C++ readConf
                match current_section.as_str() {
                    "common" => self.process_common_section(key, value),
                    "surge_external_proxy" => self.process_surge_external_section(key, value),
                    "node_pref" => self.process_node_pref_section(key, value),
                    "userinfo" => self.process_userinfo_section(key, value),
                    "managed_config" => self.process_managed_config_section(key, value),
                    "rulesets" | "ruleset" => self.process_ruleset_section(key, value),
                    "proxy_groups" | "clash_proxy_group" => {
                        self.process_proxy_group_section(key, value)
                    }
                    "template" => self.process_template_section(key, value),
                    "aliases" => self.process_aliases_section(key, value),
                    "tasks" => self.process_tasks_section(key, value),
                    "server" => self.process_server_section(key, value),
                    "advanced" => self.process_advanced_section(key, value),
                    "emojis" => self.process_emoji_section(key, value),
                    _ => {} // Ignore unknown sections
                }
            }
        }
        Ok(())
    }

    fn process_common_section(&mut self, key: &str, value: &str) {
        match key {
            "api_mode" => self.api_mode = parse_bool(value),
            "api_access_token" => self.api_access_token = value.to_string(),
            "default_url" => self.default_url = value.to_string(),
            "enable_insert" => self.enable_insert = parse_bool(value),
            "insert_url" => self.insert_url = value.to_string(),
            "prepend_insert_url" => self.prepend_insert = parse_bool(value),
            "exclude_remarks" => self.exclude_remarks.push(value.to_owned()),
            "include_remarks" => self.include_remarks.push(value.to_owned()),
            "enable_filter" => self.enable_filter = parse_bool(value),
            "filter_script" => self.filter_script = value.to_string(),
            "base_path" => self.base_path = value.to_string(),
            "clash_rule_base" => self.clash_base = value.to_string(),
            "surge_rule_base" => self.surge_base = value.to_string(),
            "surfboard_rule_base" => self.surfboard_base = value.to_string(),
            "mellow_rule_base" => self.mellow_base = value.to_string(),
            "quan_rule_base" => self.quan_base = value.to_string(),
            "quanx_rule_base" => self.quanx_base = value.to_string(),
            "loon_rule_base" => self.loon_base = value.to_string(),
            "sssub_rule_base" => self.ssub_base = value.to_string(),
            "singbox_rule_base" => self.singbox_base = value.to_string(),
            "default_external_config" => self.default_ext_config = value.to_string(),
            "append_proxy_type" => self.append_type = parse_bool(value),
            "proxy_config" => self.proxy_config = value.to_string(),
            "proxy_ruleset" => self.proxy_ruleset = value.to_string(),
            "proxy_subscription" => self.proxy_subscription = value.to_string(),
            "reload_conf_on_request" => self.reload_conf_on_request = parse_bool(value),
            _ => {}
        }
    }

    fn process_surge_external_section(&mut self, key: &str, value: &str) {
        match key {
            "surge_ssr_path" => self.surge_ssr_path = value.to_string(),
            "resolve_hostname" => self.surge_resolve_hostname = parse_bool(value),
            _ => {}
        }
    }

    fn process_node_pref_section(&mut self, key: &str, value: &str) {
        match key {
            "udp_flag" => self.udp_flag = Some(parse_bool(value)),
            "tcp_fast_open_flag" => self.tfo_flag = Some(parse_bool(value)),
            "skip_cert_verify_flag" => self.skip_cert_verify = Some(parse_bool(value)),
            "tls13_flag" => self.tls13_flag = Some(parse_bool(value)),
            "sort_flag" => self.enable_sort = parse_bool(value),
            "sort_script" => self.sort_script = value.to_string(),
            "filter_deprecated" => self.filter_deprecated = parse_bool(value),
            "append_sub_userinfo" => self.append_sub_userinfo = parse_bool(value),
            "clash_use_new_field_name" => self.clash_use_new_field = parse_bool(value),
            "clash_proxies_style" => self.clash_proxies_style = value.to_string(),
            "clash_proxy_groups_style" => self.clash_proxy_groups_style = value.to_string(),
            "singbox_add_clash_modes" => self.singbox_add_clash_modes = parse_bool(value),
            "rename_node" => self.rename_node.push(value.to_string()),
            _ => {}
        }
    }

    fn process_userinfo_section(&mut self, key: &str, value: &str) {
        match key {
            "stream_rule" => self.stream_rule.push(value.to_string()),
            "time_rule" => self.time_rule.push(value.to_string()),
            _ => {}
        }
    }

    fn process_managed_config_section(&mut self, key: &str, value: &str) {
        match key {
            "write_managed_config" => self.write_managed_config = parse_bool(value),
            "managed_config_prefix" => self.managed_config_prefix = value.to_string(),
            "config_update_interval" => {
                if let Ok(val) = value.parse() {
                    self.update_interval = val
                }
            }
            "config_update_strict" => self.update_strict = parse_bool(value),
            "quanx_device_id" => self.quanx_dev_id = value.to_string(),
            _ => {}
        }
    }

    fn process_emoji_section(&mut self, key: &str, value: &str) {
        match key {
            "add_emoji" => self.add_emoji = parse_bool(value),
            "remove_old_emoji" => self.remove_emoji = parse_bool(value),
            "rule" => self.emoji_rules.push(value.to_string()),
            _ => {}
        }
    }

    fn process_ruleset_section(&mut self, key: &str, value: &str) {
        match key {
            "enabled" => self.enable_rule_gen = parse_bool(value),
            "overwrite_original_rules" => self.overwrite_original_rules = parse_bool(value),
            "update_ruleset_on_request" => self.update_ruleset_on_request = parse_bool(value),
            "ruleset" | "surge_ruleset" => {
                self.rulesets.push(value.to_string());
            }
            _ => {}
        }
    }

    fn process_proxy_group_section(&mut self, key: &str, value: &str) {
        match key {
            "custom_proxy_group" => {
                self.custom_proxy_group.push(value.to_string());
            }
            _ => {}
        }
    }

    // TODO: 后处理template字段
    fn process_template_section(&mut self, key: &str, value: &str) {
        match key {
            "template_path" => self.template_path = value.to_string(),
            _ => {
                // Store template variables - ensure the key is not empty
                if !key.is_empty() {
                    self.template_vars
                        .insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    fn process_aliases_section(&mut self, key: &str, value: &str) {
        if !key.is_empty() && !value.is_empty() {
            self.aliases.insert(key.to_string(), value.to_string());
        }
    }

    fn process_tasks_section(&mut self, key: &str, value: &str) {
        match key {
            "task" => {
                if !value.is_empty() {
                    // Format is typically "name`cronexp`path`timeout"
                    self.enable_cron = true;
                    // Tasks will be processed in process_imports
                }
            }
            _ => {}
        }
    }

    fn process_server_section(&mut self, key: &str, value: &str) {
        match key {
            "listen" => self.listen_address = value.to_string(),
            "port" => {
                if let Ok(val) = value.parse() {
                    self.listen_port = val
                }
            }
            "serve_file_root" => {
                self.serve_file_root = value.to_string();
                self.serve_file = !self.serve_file_root.is_empty();
            }
            _ => {}
        }
    }

    fn process_advanced_section(&mut self, key: &str, value: &str) {
        match key {
            "log_level" => {
                if let Ok(val) = value.parse() {
                    self.log_level = val
                }
            }
            "print_debug_info" => self.print_dbg_info = parse_bool(value),
            "max_pending_connections" => {
                if let Ok(val) = value.parse() {
                    self.max_pending_conns = val
                }
            }
            "max_concurrent_threads" => {
                if let Ok(val) = value.parse() {
                    self.max_concur_threads = val
                }
            }
            "max_allowed_rulesets" => {
                if let Ok(val) = value.parse() {
                    self.max_allowed_rulesets = val
                }
            }
            "max_allowed_rules" => {
                if let Ok(val) = value.parse() {
                    self.max_allowed_rules = val
                }
            }
            "max_allowed_download_size" => {
                if let Ok(val) = value.parse() {
                    self.max_allowed_download_size = val
                }
            }
            "enable_cache" => {
                self.enable_cache = parse_bool(value);
            }
            "cache_subscription" => {
                if let Ok(val) = value.parse() {
                    self.cache_subscription = val
                }
            }
            "cache_config" => {
                if let Ok(val) = value.parse() {
                    self.cache_config = val
                }
            }
            "cache_ruleset" => {
                if let Ok(val) = value.parse() {
                    self.cache_ruleset = val
                }
            }
            "serve_cache_on_fetch_fail" => self.serve_cache_on_fetch_fail = parse_bool(value),
            "script_clean_context" => self.script_clean_context = parse_bool(value),
            "async_fetch_ruleset" => self.async_fetch_ruleset = parse_bool(value),
            "skip_failed_links" => self.skip_failed_links = parse_bool(value),
            _ => {}
        }
    }
}

/// Parse a string as boolean
fn parse_bool(value: &str) -> bool {
    value.to_lowercase() == "true" || value == "1"
}
