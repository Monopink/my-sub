//! Ruleset to Sing-Box conversion
//!
//! This module provides functionality to convert rulesets to Sing-Box format.

use crate::models::RulesetContent;
use crate::utils::string::{find_str, starts_with, to_lower};
use crate::utils::trim;
use crate::Settings;
use log::warn;
use serde_json::{json, Map, Value};

use super::convert_ruleset::convert_ruleset;
use super::ruleset::SINGBOX_RULE_TYPES;

/// Converts rulesets to Sing-Box format and updates the JSON configuration
///
/// # Arguments
///
/// * `base_rule` - The base Sing-Box configuration as JSON
/// * `ruleset_content_array` - Array of ruleset contents to process
/// * `overwrite_original_rules` - Whether to overwrite original rules
pub fn ruleset_to_sing_box(
    base_rule: &mut Value,
    ruleset_content_array: &[RulesetContent],
    overwrite_original_rules: bool,
) {
    // Get global settings
    let settings = Settings::current();

    // Create rules array
    let mut rules = Value::Array(Vec::new());

    // If not overwriting, copy existing rules
    if !overwrite_original_rules {
        if let Some(route) = base_rule.get("route") {
            if let Some(existing_rules) = route.get("rules") {
                if existing_rules.is_array() {
                    rules = existing_rules.clone();
                }
            }
        }
    }

    // Add Clash modes if enabled
    if settings.singbox_add_clash_modes {
        let global_object = json!({
            "clash_mode": "Global",
            "outbound": "GLOBAL"
        });

        let direct_object = json!({
            "clash_mode": "Direct",
            "outbound": "DIRECT"
        });

        if let Some(rules_array) = rules.as_array_mut() {
            rules_array.push(global_object);
            rules_array.push(direct_object);
        }
    }

    // Add DNS rule
    let dns_object = json!({
        "protocol": "dns",
        "outbound": "dns-out"
    });

    if let Some(rules_array) = rules.as_array_mut() {
        rules_array.push(dns_object);
    }

    // Process each ruleset
    let mut total_rules = 0;
    let mut final_rule = String::new();

    for ruleset in ruleset_content_array {
        // Check if we've reached the maximum number of rules
        if settings.max_allowed_rules > 0 && total_rules >= settings.max_allowed_rules {
            break;
        }

        let rule_group = &ruleset.group;
        let retrieved_rules = ruleset.get_rule_content();

        if retrieved_rules.is_empty() {
            warn!(
                "Failed to fetch ruleset or ruleset is empty: '{}'!",
                ruleset.rule_path
            );
            continue;
        }

        // Special case for rules that start with "[]"
        if starts_with(&retrieved_rules, "[]") {
            let str_line = &retrieved_rules[2..];

            if starts_with(str_line, "FINAL") || starts_with(str_line, "MATCH") {
                final_rule = rule_group.clone();
                continue;
            }

            // Transform rule to SingBox format
            let parts: Vec<&str> = str_line.split(',').collect();
            if parts.len() < 2 {
                continue;
            }

            let rule_type = to_lower(parts[0]);
            let rule_value = to_lower(parts[1]);

            let mut rule_obj = Map::new();

            // Convert type names from Clash format to SingBox format
            let rule_type = rule_type
                .replace("-", "_")
                .replace("ip_cidr6", "ip_cidr")
                .replace("src_", "source_");

            if rule_type == "match" || rule_type == "final" {
                rule_obj.insert("outbound".to_string(), Value::String(rule_value));
            } else {
                rule_obj.insert(rule_type, Value::String(rule_value));
                rule_obj.insert(
                    "outbound".to_string(),
                    Value::String(rule_group.to_string()),
                );
            }

            if let Some(rules_array) = rules.as_array_mut() {
                rules_array.push(Value::Object(rule_obj));
                total_rules += 1;
            }

            continue;
        }

        // Convert ruleset based on its type
        let converted_rules = convert_ruleset(&retrieved_rules, ruleset.rule_type);

        // Create a new rule object for this ruleset
        let mut rule_obj = Map::new();

        // Process each rule line
        for line in converted_rules.lines() {
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

            // Remove inline comments
            if let Some(comment_pos) = find_str(&str_line, "//") {
                str_line = str_line[..comment_pos].to_string();
                str_line = trim(&str_line).to_string();
            }

            // Process the rule
            let rule_parts: Vec<&str> = str_line.split(',').collect();
            if rule_parts.len() < 2 {
                continue;
            }

            let rule_type = rule_parts[0];

            // Skip if rule type is not supported
            if !SINGBOX_RULE_TYPES.contains(rule_type) {
                continue;
            }

            let real_type = to_lower(rule_type)
                .replace("-", "_")
                .replace("ip_cidr6", "ip_cidr")
                .replace("src_", "source_");

            let rule_value = to_lower(rule_parts[1]);

            // Add to rule object
            let values = rule_obj
                .entry(real_type)
                .or_insert_with(|| Value::Array(Vec::new()));

            if let Value::Array(ref mut arr) = values {
                arr.push(Value::String(rule_value));
                total_rules += 1;
            }
        }

        // Only add if rule object is not empty
        if !rule_obj.is_empty() {
            // Add outbound to the rule object
            rule_obj.insert("outbound".to_string(), Value::String(rule_group.clone()));

            if let Some(rules_array) = rules.as_array_mut() {
                rules_array.push(Value::Object(rule_obj));
            }
        }
    }

    // Ensure "route" section exists in the base rule
    if base_rule.get("route").is_none() {
        base_rule["route"] = json!({});
    }

    // Update the rules array in the base rule
    if let Some(route) = base_rule.get_mut("route") {
        if let Some(route_obj) = route.as_object_mut() {
            route_obj.insert("rules".to_string(), rules);
            route_obj.insert("final".to_string(), Value::String(final_rule));
        }
    }
}
