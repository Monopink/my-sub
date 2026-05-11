/// Settings module for subconverter-rs
///
/// This module handles loading and processing of configuration settings.
///
/// Example of template variable usage:
///
/// ```rust
/// use subconverter_rs::settings::Settings;
/// use subconverter_rs::settings::update_settings_from_content;
///
/// // YAML with template variables
/// let yaml_content = r#"
/// common:
///   template_path: "./templates"
/// template:
///   globals:
///     - key: clash_dns_port
///       value: 5353
///     - key: clash_api_port
///       value: 9090
///     - key: singbox_direct_domain
///       value: example.com
/// "#;
///
/// // Update settings
/// update_settings_from_content(yaml_content).unwrap();
///
/// // Access template variables
/// let settings = Settings::current();
/// assert_eq!(settings.template_path, "./templates");
/// assert_eq!(settings.template_vars.get("clash_dns_port"), Some(&"5353".to_string()));
/// ```
///
/// Template variables can be accessed from settings.template_vars HashMap
pub mod conversions;
pub mod ini_settings;
pub mod settings_struct;
pub mod toml_settings;
pub mod yaml_settings;

// Re-export settings types
pub use ini_settings::IniSettings;
pub use settings_struct::*;
pub use toml_settings::TomlSettings;
pub use yaml_settings::YamlSettings;
