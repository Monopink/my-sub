use super::external_struct::ExternalSettings;
use super::ini_external::IniExternalSettings;
use super::toml_external::TomlExternalSettings;
use super::yaml_external::YamlExternalSettings;

// TODO: Implement template handling and global settings like in C++
// In C++, there is a template rendering system and global settings
// that need to be addressed in the conversion process

// Conversion from YamlExternalSettings to ExternalSettings
impl From<YamlExternalSettings> for ExternalSettings {
    fn from(yaml_settings: YamlExternalSettings) -> Self {
        let mut settings = ExternalSettings::default();

        // Copy rule bases
        settings.clash_rule_base = yaml_settings.custom.rule_bases.clash_rule_base;
        settings.surge_rule_base = yaml_settings.custom.rule_bases.surge_rule_base;
        settings.surfboard_rule_base = yaml_settings.custom.rule_bases.surfboard_rule_base;
        settings.mellow_rule_base = yaml_settings.custom.rule_bases.mellow_rule_base;
        settings.quan_rule_base = yaml_settings.custom.rule_bases.quan_rule_base;
        settings.quanx_rule_base = yaml_settings.custom.rule_bases.quanx_rule_base;
        settings.loon_rule_base = yaml_settings.custom.rule_bases.loon_rule_base;
        settings.sssub_rule_base = yaml_settings.custom.rule_bases.sssub_rule_base;
        settings.singbox_rule_base = yaml_settings.custom.rule_bases.singbox_rule_base;

        // Rule generation options
        settings.enable_rule_generator =
            Some(yaml_settings.custom.rule_generation.enable_rule_generator);
        settings.overwrite_original_rules = Some(
            yaml_settings
                .custom
                .rule_generation
                .overwrite_original_rules,
        );

        // Emoji options
        settings.add_emoji = yaml_settings.custom.emoji_settings.add_emoji;
        settings.remove_old_emoji = yaml_settings.custom.emoji_settings.remove_old_emoji;

        // Filtering options
        settings.include_remarks = yaml_settings.custom.filtering.include_remarks;
        settings.exclude_remarks = yaml_settings.custom.filtering.exclude_remarks;

        // Copy processed fields
        settings.custom_rulesets = yaml_settings.parsed_rulesets;
        settings.custom_proxy_groups = yaml_settings.parsed_custom_proxy_groups;
        settings.rename_nodes = yaml_settings.parsed_rename;
        settings.emojis = yaml_settings.parsed_emojis;

        // Copy template arguments
        settings.tpl_args = yaml_settings.tpl_args;

        settings
    }
}

// Conversion from TomlExternalSettings to ExternalSettings
impl From<TomlExternalSettings> for ExternalSettings {
    fn from(toml_settings: TomlExternalSettings) -> Self {
        let mut settings = ExternalSettings::default();

        // Copy rule bases
        settings.clash_rule_base = toml_settings.custom.rule_bases.clash_rule_base;
        settings.surge_rule_base = toml_settings.custom.rule_bases.surge_rule_base;
        settings.surfboard_rule_base = toml_settings.custom.rule_bases.surfboard_rule_base;
        settings.mellow_rule_base = toml_settings.custom.rule_bases.mellow_rule_base;
        settings.quan_rule_base = toml_settings.custom.rule_bases.quan_rule_base;
        settings.quanx_rule_base = toml_settings.custom.rule_bases.quanx_rule_base;
        settings.loon_rule_base = toml_settings.custom.rule_bases.loon_rule_base;
        settings.sssub_rule_base = toml_settings.custom.rule_bases.sssub_rule_base;
        settings.singbox_rule_base = toml_settings.custom.rule_bases.singbox_rule_base;

        // Rule generation options
        settings.enable_rule_generator =
            Some(toml_settings.custom.rule_generation.enable_rule_generator);
        settings.overwrite_original_rules = Some(
            toml_settings
                .custom
                .rule_generation
                .overwrite_original_rules,
        );

        // Emoji options
        settings.add_emoji = toml_settings.custom.emoji_settings.add_emoji;
        settings.remove_old_emoji = toml_settings.custom.emoji_settings.remove_old_emoji;

        // Filtering options
        settings.include_remarks = toml_settings.custom.filtering.include_remarks;
        settings.exclude_remarks = toml_settings.custom.filtering.exclude_remarks;

        // Copy processed fields
        settings.custom_rulesets = toml_settings.parsed_rulesets;
        settings.custom_proxy_groups = toml_settings.parsed_custom_proxy_groups;
        settings.rename_nodes = toml_settings.parsed_rename;
        settings.emojis = toml_settings.parsed_emojis;

        // Copy template arguments
        settings.tpl_args = toml_settings.tpl_args;

        settings
    }
}

// Conversion from IniExternalSettings to ExternalSettings
impl From<IniExternalSettings> for ExternalSettings {
    fn from(ini_settings: IniExternalSettings) -> Self {
        let mut settings = ExternalSettings::default();

        // Copy rule bases
        settings.clash_rule_base = ini_settings.clash_rule_base;
        settings.surge_rule_base = ini_settings.surge_rule_base;
        settings.surfboard_rule_base = ini_settings.surfboard_rule_base;
        settings.mellow_rule_base = ini_settings.mellow_rule_base;
        settings.quan_rule_base = ini_settings.quan_rule_base;
        settings.quanx_rule_base = ini_settings.quanx_rule_base;
        settings.loon_rule_base = ini_settings.loon_rule_base;
        settings.sssub_rule_base = ini_settings.sssub_rule_base;
        settings.singbox_rule_base = ini_settings.singbox_rule_base;

        // Rule generation options
        settings.enable_rule_generator = Some(ini_settings.enable_rule_generator);
        settings.overwrite_original_rules = Some(ini_settings.overwrite_original_rules);

        // Emoji options
        settings.add_emoji = ini_settings.add_emoji;
        settings.remove_old_emoji = ini_settings.remove_old_emoji;

        // Filtering options
        settings.include_remarks = ini_settings.include_remarks;
        settings.exclude_remarks = ini_settings.exclude_remarks;

        // Copy processed fields
        settings.custom_rulesets = ini_settings.parsed_rulesets;
        settings.custom_proxy_groups = ini_settings.parsed_custom_proxy_groups;
        settings.rename_nodes = ini_settings.parsed_rename;
        settings.emojis = ini_settings.parsed_emojis;

        // Copy template arguments
        settings.tpl_args = ini_settings.tpl_args;

        settings
    }
}
