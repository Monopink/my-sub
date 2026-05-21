use serde::de::{self, Deserializer};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PortValue {
    Number(u16),
    String(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringListValue {
    Single(String),
    List(Vec<String>),
}

pub fn deserialize_port<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let value = PortValue::deserialize(deserializer)?;
    let port = match value {
        PortValue::Number(port) => port,
        PortValue::String(raw) => parse_port_str(&raw).map_err(de::Error::custom)?,
    };

    if port == 0 {
        return Err(de::Error::custom("port must be within 1..=65535"));
    }

    Ok(port)
}

fn parse_port_str(input: &str) -> Result<u16, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("port cannot be empty".to_string());
    }

    let parsed = trimmed
        .parse::<u16>()
        .map_err(|_| format!("invalid port value: {trimmed}"))?;

    if parsed == 0 {
        return Err("port must be within 1..=65535".to_string());
    }

    Ok(parsed)
}

pub fn deserialize_optional_string_list<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<StringListValue>::deserialize(deserializer)?;
    let Some(value) = value else {
        return Ok(None);
    };

    let items = match value {
        StringListValue::Single(text) => text
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>(),
        StringListValue::List(list) => list
            .into_iter()
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect::<Vec<_>>(),
    };

    if items.is_empty() {
        return Ok(None);
    }

    Ok(Some(items))
}
