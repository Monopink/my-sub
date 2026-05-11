use std::{collections::HashMap, fmt};

use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize,
};

use super::settings::yaml_settings::TemplateSettings;

/// Stream rule configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RegexMatchRuleInYaml {
    #[serde(rename = "match")]
    pub match_str: Option<String>,
    pub replace: Option<String>,
    pub script: Option<String>,
    pub import: Option<String>,
}

/// Trait for converting to INI format with a specified delimiter
pub trait ToIniWithDelimiter {
    fn to_ini_with_delimiter(&self, delimiter: &str) -> String;
}

impl ToIniWithDelimiter for RegexMatchRuleInYaml {
    fn to_ini_with_delimiter(&self, delimiter: &str) -> String {
        // Check for script first
        if let Some(script) = &self.script {
            if !script.is_empty() {
                return format!("!!script:{}", script);
            }
        }

        // Then check for import
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Finally check for match and replace
        if let (Some(match_str), Some(replace)) = (&self.match_str, &self.replace) {
            if !match_str.is_empty() && !replace.is_empty() {
                return format!("{}{}{}", match_str, delimiter, replace);
            }
        }

        // Default to empty string if nothing matches
        String::new()
    }
}

pub trait ToIni {
    fn to_ini(&self) -> String;
}

impl ToIni for RulesetConfigInYaml {
    fn to_ini(&self) -> String {
        // Check for import first
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Then check for ruleset URL
        if let Some(ruleset) = &self.ruleset {
            if !ruleset.is_empty() {
                let mut result = format!("{},{}", self.group, ruleset);
                // Add interval if provided
                if let Some(interval) = self.interval {
                    result = format!("{},{}", result, interval);
                }
                return result;
            }
        }

        // Finally check for rule
        if let Some(rule) = &self.rule {
            if !rule.is_empty() {
                return format!("{},[]{}", self.group, rule);
            }
        }

        // Default to empty string if nothing matches
        String::new()
    }
}

impl ToIni for TaskConfigInYaml {
    fn to_ini(&self) -> String {
        // Check for import first
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Otherwise join fields with backticks
        format!(
            "{}`{}`{}`{}",
            self.name, self.cronexp, self.path, self.timeout
        )
    }
}

/// Proxy group configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct ProxyGroupConfigInYaml {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: String,
    pub rule: Vec<String>,
    #[serde(default = "default_test_url")]
    pub url: Option<String>,
    #[serde(default = "default_interval")]
    pub interval: Option<u32>,
    pub tolerance: Option<u32>,
    pub timeout: Option<u32>,
    pub import: Option<String>,
}

impl ToIni for ProxyGroupConfigInYaml {
    fn to_ini(&self) -> String {
        // Check for import first
        if let Some(import) = &self.import {
            if !import.is_empty() {
                return format!("!!import:{}", import);
            }
        }

        // Create initial array with name and type
        let mut temp_array = vec![self.name.clone(), self.group_type.clone()];

        // Add all rules
        for rule in &self.rule {
            temp_array.push(rule.clone());
        }

        // Check if we have enough elements based on group type
        match self.group_type.as_str() {
            "select" => {
                if temp_array.len() < 3 {
                    return String::new();
                }
            }
            "ssid" => {
                if temp_array.len() < 4 {
                    return String::new();
                }
            }
            _ => {
                if temp_array.len() < 3 {
                    return String::new();
                }

                // Add url
                temp_array.push(
                    self.url
                        .clone()
                        .unwrap_or_else(|| "http://www.gstatic.com/generate_204".to_string()),
                );

                // Add interval, timeout, tolerance as a combined string
                let interval = self.interval.unwrap_or(300).to_string();
                let timeout = match self.timeout {
                    Some(t) => t.to_string(),
                    None => String::new(),
                };
                let tolerance = match self.tolerance {
                    Some(t) => t.to_string(),
                    None => String::new(),
                };

                temp_array.push(format!("{},{},{}", interval, timeout, tolerance));
            }
        }

        // Join all elements with backtick
        temp_array.join("`")
    }
}

fn default_test_url() -> Option<String> {
    Some("http://www.gstatic.com/generate_204".to_string())
}

fn default_interval() -> Option<u32> {
    Some(300)
}

/// Task configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct TaskConfigInYaml {
    pub name: String,
    pub cronexp: String,
    pub path: String,
    pub timeout: u32,
    pub import: Option<String>,
}

/// Ruleset configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default)]
pub struct RulesetConfigInYaml {
    pub rule: Option<String>,
    pub ruleset: Option<String>,
    pub group: String,
    pub interval: Option<u32>,
    pub import: Option<String>,
}

pub fn deserialize_template_as_template_settings<'de, D>(
    deserializer: D,
) -> Result<TemplateSettings, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct TemplateSettingsVisitor;

    #[derive(Debug, Clone, Deserialize, Default)]
    struct TemplateGlobalsVariable {
        key: String,
        #[serde(deserialize_with = "deserialize_as_string")]
        value: String,
    }

    impl<'de> Visitor<'de> for TemplateSettingsVisitor {
        type Value = TemplateSettings;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a TemplateSettings struct")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut template_settings = TemplateSettings::default();
            while let Some(key) = map.next_key::<String>()? {
                if key == "template_path" {
                    let value = map.next_value::<String>()?;
                    template_settings.template_path = value.clone();
                } else if key == "globals" {
                    let value = map.next_value::<Vec<TemplateGlobalsVariable>>()?;
                    for item in value {
                        template_settings.globals.insert(item.key, item.value);
                    }
                }
            }
            Ok(template_settings)
        }
    }

    deserializer.deserialize_any(TemplateSettingsVisitor)
}

/// Template argument
#[derive(Debug, Clone, Deserialize, Default)]
struct TemplateArgument {
    pub key: String,
    pub value: String,
}

pub fn deserialize_template_args_as_hash_map<'de, D>(
    deserializer: D,
) -> Result<Option<HashMap<String, String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct TemplateArgsVisitor;

    impl<'de> Visitor<'de> for TemplateArgsVisitor {
        type Value = Option<HashMap<String, String>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of template arguments or null")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut template_args = HashMap::new();

            while let Some(item) = seq.next_element::<TemplateArgument>()? {
                template_args.insert(item.key, item.value);
            }

            if template_args.is_empty() {
                Ok(None)
            } else {
                Ok(Some(template_args))
            }
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut template_args = HashMap::new();

            while let Some((key, value)) = map.next_entry::<String, String>()? {
                template_args.insert(key, value);
            }

            if template_args.is_empty() {
                Ok(None)
            } else {
                Ok(Some(template_args))
            }
        }
    }

    deserializer.deserialize_any(TemplateArgsVisitor)
}

pub fn deserialize_as_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, number, or boolean")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(String::new())
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(String::new())
        }
    }

    deserializer.deserialize_any(StringVisitor)
}
