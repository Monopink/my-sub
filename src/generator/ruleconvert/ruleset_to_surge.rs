//! Ruleset to Surge conversion
//!
//! This module provides functionality to convert rulesets to Surge format.

use crate::models::RulesetContent;
use crate::utils::base64::url_safe_base64_encode;
use crate::utils::ini_reader::IniReader;
use crate::utils::network::is_link;
use crate::utils::string::{find_str, starts_with};
use crate::utils::{file_exists, trim};
use crate::Settings;
use lazy_static::lazy_static;
use log::warn;
use std::collections::HashSet;

use super::common::transform_rule_to_common;
use super::convert_ruleset::convert_ruleset;

lazy_static! {
    static ref QUANX_RULE_TYPES: HashSet<&'static str> = {
        let mut types = HashSet::new();
        // Basic types
        types.insert("DOMAIN");
        types.insert("DOMAIN-SUFFIX");
        types.insert("DOMAIN-KEYWORD");
        types.insert("IP-CIDR");
        types.insert("SRC-IP-CIDR");
        types.insert("GEOIP");
        types.insert("MATCH");
        types.insert("FINAL");
        // QuanX specific types
        types.insert("USER-AGENT");
        types.insert("HOST");
        types.insert("HOST-SUFFIX");
        types.insert("HOST-KEYWORD");
        types
    };

    static ref SURF_RULE_TYPES: HashSet<&'static str> = {
        let mut types = HashSet::new();
        // Basic types
        types.insert("DOMAIN");
        types.insert("DOMAIN-SUFFIX");
        types.insert("DOMAIN-KEYWORD");
        types.insert("IP-CIDR");
        types.insert("SRC-IP-CIDR");
        types.insert("GEOIP");
        types.insert("MATCH");
        types.insert("FINAL");
        // Surfboard specific types
        types.insert("IP-CIDR6");
        types.insert("PROCESS-NAME");
        types.insert("IN-PORT");
        types.insert("DEST-PORT");
        types.insert("SRC-IP");
        types
    };

    static ref SURGE2_RULE_TYPES: HashSet<&'static str> = {
        let mut types = HashSet::new();
        // Basic types
        types.insert("DOMAIN");
        types.insert("DOMAIN-SUFFIX");
        types.insert("DOMAIN-KEYWORD");
        types.insert("IP-CIDR");
        types.insert("SRC-IP-CIDR");
        types.insert("GEOIP");
        types.insert("MATCH");
        types.insert("FINAL");
        // Surge2 specific types
        types.insert("IP-CIDR6");
        types.insert("USER-AGENT");
        types.insert("URL-REGEX");
        types.insert("PROCESS-NAME");
        types.insert("IN-PORT");
        types.insert("DEST-PORT");
        types.insert("SRC-IP");
        types
    };

    static ref SURGE_RULE_TYPES: HashSet<&'static str> = {
        let mut types = HashSet::new();
        // Basic types
        types.insert("DOMAIN");
        types.insert("DOMAIN-SUFFIX");
        types.insert("DOMAIN-KEYWORD");
        types.insert("IP-CIDR");
        types.insert("SRC-IP-CIDR");
        types.insert("GEOIP");
        types.insert("MATCH");
        types.insert("FINAL");
        // Surge specific types
        types.insert("IP-CIDR6");
        types.insert("USER-AGENT");
        types.insert("URL-REGEX");
        types.insert("AND");
        types.insert("OR");
        types.insert("NOT");
        types.insert("PROCESS-NAME");
        types.insert("IN-PORT");
        types.insert("DEST-PORT");
        types.insert("SRC-IP");
        types
    };
}

/// Converts rulesets to Surge format and updates the INI configuration
///
/// # Arguments
///
/// * `base_rule` - The base Surge configuration as IniReader
/// * `ruleset_content_array` - Array of ruleset contents to process
/// * `surge_ver` - Surge version (or negative for other clients)
/// * `overwrite_original_rules` - Whether to overwrite original rules
/// * `remote_path_prefix` - Prefix for remote ruleset URLs
///
/// # Returns
///
/// Status code indicating success or failure
pub async fn ruleset_to_surge(
    base_rule: &mut IniReader,
    ruleset_content_array: &[RulesetContent],
    surge_ver: i32,
    overwrite_original_rules: bool,
    remote_path_prefix: &str,
) {
    // Get global settings
    let settings = Settings::current();

    // Set the appropriate section based on surge_ver
    match surge_ver {
        0 => base_rule.set_current_section("RoutingRule"), // Mellow
        -1 => base_rule.set_current_section("filter_local"), // Quantumult X
        -2 => base_rule.set_current_section("TCP"),        // Quantumult
        _ => base_rule.set_current_section("Rule"),
    }

    // Handle overwriting original rules
    if overwrite_original_rules {
        base_rule.erase_section();
        match surge_ver {
            -1 => base_rule.erase_section_by_name("filter_remote"),
            -4 => base_rule.erase_section_by_name("Remote Rule"),
            _ => {}
        }
    }

    // Keep track of all rules to add
    let mut all_rules = Vec::new();
    let mut total_rules = 0;

    // Process each ruleset
    for ruleset in ruleset_content_array {
        // Check if we've reached the maximum number of rules
        if settings.max_allowed_rules > 0 && total_rules >= settings.max_allowed_rules {
            break;
        }

        let rule_group = &ruleset.group;
        let rule_path = &ruleset.rule_path;
        let rule_path_typed = &ruleset.rule_path_typed;

        if rule_path.is_empty() {
            // Special case for rules that start with "[]"
            let mut str_line = ruleset.get_rule_content()[2..].to_string();
            if str_line == "MATCH" {
                str_line = "FINAL".to_string();
            }

            if surge_ver == -1 || surge_ver == -2 {
                str_line = transform_rule_to_common(&str_line, rule_group, true);
            } else {
                if !starts_with(&str_line, "AND")
                    && !starts_with(&str_line, "OR")
                    && !starts_with(&str_line, "NOT")
                {
                    str_line = transform_rule_to_common(&str_line, rule_group, false);
                }
            }

            // Replace double commas with single comma
            str_line = str_line.replace(",,", ",");
            all_rules.push(str_line);
            total_rules += 1;
            continue;
        } else {
            // Handle file or URL paths
            if surge_ver == -1
                && ruleset.rule_type == crate::models::RulesetType::Quanx
                && is_link(rule_path)
            {
                let str_line = format!(
                    "{}, tag={}, force-policy={}, enabled=true",
                    rule_path, rule_group, rule_group
                );
                let _ = base_rule.set("filter_remote", "{NONAME}", &str_line);
                continue;
            }

            if file_exists(rule_path).await {
                if surge_ver > 2 && !remote_path_prefix.is_empty() {
                    let mut str_line = format!(
                        "RULE-SET,{}/getruleset?type=1&url={},{}",
                        remote_path_prefix,
                        url_safe_base64_encode(rule_path_typed),
                        rule_group
                    );

                    if ruleset.update_interval > 0 {
                        str_line.push_str(&format!(",update-interval={}", ruleset.update_interval));
                    }

                    all_rules.push(str_line);
                    continue;
                } else if surge_ver == -1 && !remote_path_prefix.is_empty() {
                    let str_line = format!(
                        "{}/getruleset?type=2&url={}&group={}, tag={}, enabled=true",
                        remote_path_prefix,
                        url_safe_base64_encode(rule_path_typed),
                        url_safe_base64_encode(rule_group),
                        rule_group
                    );

                    let _ = base_rule.set("filter_remote", "{NONAME}", &str_line);
                    continue;
                } else if surge_ver == -4 && !remote_path_prefix.is_empty() {
                    let str_line = format!(
                        "{}/getruleset?type=1&url={},{}",
                        remote_path_prefix,
                        url_safe_base64_encode(rule_path_typed),
                        rule_group
                    );

                    let _ = base_rule.set("Remote Rule", "{NONAME}", &str_line);
                    continue;
                }
            } else if is_link(rule_path) {
                if surge_ver > 2 {
                    if ruleset.rule_type != crate::models::RulesetType::Surge {
                        if !remote_path_prefix.is_empty() {
                            let mut str_line = format!(
                                "RULE-SET,{}/getruleset?type=1&url={},{}",
                                remote_path_prefix,
                                url_safe_base64_encode(rule_path_typed),
                                rule_group
                            );

                            if ruleset.update_interval > 0 {
                                str_line.push_str(&format!(
                                    ",update-interval={}",
                                    ruleset.update_interval
                                ));
                            }

                            all_rules.push(str_line);
                        }
                        continue;
                    } else {
                        let mut str_line = format!("RULE-SET,{},{}", rule_path, rule_group);

                        if ruleset.update_interval > 0 {
                            str_line
                                .push_str(&format!(",update-interval={}", ruleset.update_interval));
                        }

                        all_rules.push(str_line);
                        continue;
                    }
                } else if surge_ver == -1 && !remote_path_prefix.is_empty() {
                    let str_line = format!(
                        "{}/getruleset?type=2&url={}&group={}, tag={}, enabled=true",
                        remote_path_prefix,
                        url_safe_base64_encode(rule_path_typed),
                        url_safe_base64_encode(rule_group),
                        rule_group
                    );

                    let _ = base_rule.set("filter_remote", "{NONAME}", &str_line);
                    continue;
                } else if surge_ver == -4 {
                    let str_line = format!("{},{}", rule_path, rule_group);
                    let _ = base_rule.set("Remote Rule", "{NONAME}", &str_line);
                    continue;
                }
            } else {
                continue;
            }

            // Process the rules content
            let retrieved_rules = ruleset.get_rule_content();
            if retrieved_rules.is_empty() {
                warn!(
                    "Failed to fetch ruleset or ruleset is empty: '{}'!",
                    rule_path
                );
                continue;
            }

            // Convert the ruleset based on its type
            let converted_rules = convert_ruleset(&retrieved_rules, ruleset.rule_type);
            // let line_break = if converted_rules.contains("\r\n") {
            //     '\r'
            // } else {
            //     '\n'
            // };

            // Process each rule line
            for line in converted_rules.lines() {
                // Check if we've reached the maximum number of rules
                if settings.max_allowed_rules > 0 && total_rules >= settings.max_allowed_rules {
                    break;
                }

                let mut str_line = trim(line).to_string();
                let line_size = str_line.len();

                // Skip empty lines and comments
                if line_size == 0
                    || (line_size >= 1 && (str_line.starts_with(';') || str_line.starts_with('#')))
                    || (line_size >= 2 && str_line.starts_with("//"))
                {
                    continue;
                }

                // Check if rule type is supported by the target
                let rule_supported = match surge_ver {
                    -2 => {
                        if starts_with(&str_line, "IP-CIDR6") {
                            false
                        } else {
                            QUANX_RULE_TYPES
                                .iter()
                                .any(|&rule_type| starts_with(&str_line, rule_type))
                        }
                    }
                    -1 => QUANX_RULE_TYPES
                        .iter()
                        .any(|&rule_type| starts_with(&str_line, rule_type)),
                    -3 => SURF_RULE_TYPES
                        .iter()
                        .any(|&rule_type| starts_with(&str_line, rule_type)),
                    _ => {
                        if surge_ver > 2 {
                            SURGE_RULE_TYPES
                                .iter()
                                .any(|&rule_type| starts_with(&str_line, rule_type))
                        } else {
                            SURGE2_RULE_TYPES
                                .iter()
                                .any(|&rule_type| starts_with(&str_line, rule_type))
                        }
                    }
                };

                if !rule_supported {
                    continue;
                }

                // Remove inline comments
                if let Some(comment_pos) = find_str(&str_line, "//") {
                    str_line = str_line[..comment_pos].to_string();
                    str_line = trim(&str_line).to_string();
                }

                // Transform the rule based on target type
                if surge_ver == -1 || surge_ver == -2 {
                    if starts_with(&str_line, "IP-CIDR6") {
                        str_line = str_line.replacen("IP-CIDR6", "IP6-CIDR", 1);
                    }
                    str_line = transform_rule_to_common(&str_line, rule_group, true);
                } else {
                    if !starts_with(&str_line, "AND")
                        && !starts_with(&str_line, "OR")
                        && !starts_with(&str_line, "NOT")
                    {
                        str_line = transform_rule_to_common(&str_line, rule_group, false);
                    }
                }

                all_rules.push(str_line);
                total_rules += 1;
            }
        }
    }

    // Add all collected rules to the INI
    for rule in all_rules {
        let _ = base_rule.set_current("{NONAME}", &rule);
    }
}
