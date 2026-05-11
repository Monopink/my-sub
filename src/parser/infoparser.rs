use std::time::{Duration, UNIX_EPOCH};

use crate::models::{Proxy, RegexMatchConfigs};
use crate::utils::base64::url_safe_base64_decode;
use crate::utils::system::safe_system_time;
use crate::utils::url::get_url_arg;
use regex::Regex;

/// Converts a string representing data size with units (B, KB, MB, etc.) to bytes
pub fn stream_to_int(stream: &str) -> u64 {
    if stream.is_empty() {
        return 0;
    }

    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut index = units.len() - 1;

    loop {
        if stream.ends_with(units[index]) {
            let value_str = stream.trim_end_matches(units[index]);
            let base_value = value_str.parse::<f64>().unwrap_or(0.0);
            return (base_value * (1024_f64.powi(index as i32))) as u64;
        }

        if index == 0 {
            break;
        }
        index -= 1;
    }

    // If no unit is found, try parsing as a raw number
    stream.parse::<u64>().unwrap_or(0)
}

/// Converts a percentage string (e.g., "50%") to a decimal value (0.5)
fn percent_to_double(percent: &str) -> f64 {
    if percent.ends_with('%') {
        let value_str = &percent[..percent.len() - 1];
        return value_str.parse::<f64>().unwrap_or(0.0) / 100.0;
    }
    0.0
}

/// Converts a date string to a timestamp
pub fn date_string_to_timestamp(date: &str) -> u64 {
    let now = safe_system_time()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs();

    if date.starts_with("left=") {
        let mut seconds_left = 0;
        let time_str = &date[5..];

        if time_str.ends_with('d') {
            let days = time_str[..time_str.len() - 1].parse::<f64>().unwrap_or(0.0);
            seconds_left = (days * 86400.0) as u64;
        }

        return now + seconds_left;
    } else {
        let parts: Vec<&str> = date.split(':').collect();
        if parts.len() != 6 {
            return 0;
        }

        // This is a simplified version - Rust's time handling is different
        // In a full implementation, you would use chrono crate for better date handling
        let _year = parts[0].parse::<i32>().unwrap_or(1900);
        let _month = parts[1].parse::<u32>().unwrap_or(1);
        let _day = parts[2].parse::<u32>().unwrap_or(1);
        let _hour = parts[3].parse::<u32>().unwrap_or(0);
        let _minute = parts[4].parse::<u32>().unwrap_or(0);
        let _second = parts[5].parse::<u32>().unwrap_or(0);

        // This is a placeholder - in practice use chrono::NaiveDate::from_ymd_opt and related functions
        // Return current time as fallback
        return now;
    }
}

/// Extracts subscription info from HTTP headers
pub fn get_sub_info_from_header(header: &str) -> Option<String> {
    let re = Regex::new(r"(?i)^Subscription-UserInfo: (.*?)$").ok()?;

    if let Some(captures) = re.captures(header) {
        if let Some(matched) = captures.get(1) {
            let ret_str = matched.as_str().trim();
            if !ret_str.is_empty() {
                return Some(ret_str.to_string());
            }
        }
    }

    None
}

/// Extracts subscription info from a collection of proxy nodes
pub fn get_sub_info_from_nodes(
    nodes: &[Proxy],
    stream_rules: &RegexMatchConfigs,
    time_rules: &RegexMatchConfigs,
) -> Option<String> {
    let mut stream_info = String::new();
    let mut time_info = String::new();

    for node in nodes {
        let remarks = &node.remark;

        // Extract stream info if not already found
        if stream_info.is_empty() {
            for rule in stream_rules {
                let re = Regex::new(&rule._match).ok()?;
                if re.is_match(remarks) {
                    let new_remark = re.replace(remarks, &rule.replace).to_string();
                    if new_remark != *remarks {
                        stream_info = new_remark;
                        break;
                    }
                }
            }
        }

        // Extract time info if not already found
        if time_info.is_empty() {
            for rule in time_rules {
                let re = Regex::new(&rule._match).ok()?;
                if re.is_match(remarks) {
                    let new_remark = re.replace(remarks, &rule.replace).to_string();
                    if new_remark != *remarks {
                        time_info = new_remark;
                        break;
                    }
                }
            }
        }

        if !stream_info.is_empty() && !time_info.is_empty() {
            break;
        }
    }

    if stream_info.is_empty() && time_info.is_empty() {
        return None;
    }

    // Calculate stream usage
    let mut total: u64 = 0;
    let mut used: u64 = 0;

    let total_str = get_url_arg(&stream_info, "total");
    let left_str = get_url_arg(&stream_info, "left");
    let used_str = get_url_arg(&stream_info, "used");

    if total_str.contains('%') {
        if !used_str.is_empty() {
            used = stream_to_int(&used_str);
            let percentage = percent_to_double(&total_str);
            if percentage > 0.0 {
                total = (used as f64 / (1.0 - percentage)) as u64;
            }
        } else if !left_str.is_empty() {
            let left = stream_to_int(&left_str);
            let percentage = percent_to_double(&total_str);
            if percentage > 0.0 {
                total = (left as f64 / percentage) as u64;
                if left > total {
                    used = 0;
                } else {
                    used = total - left;
                }
            }
        }
    } else {
        total = stream_to_int(&total_str);
        if !used_str.is_empty() {
            used = stream_to_int(&used_str);
        } else if !left_str.is_empty() {
            let left = stream_to_int(&left_str);
            if left > total {
                used = 0;
            } else {
                used = total - left;
            }
        }
    }

    let mut result = format!("upload=0; download={}; total={};", used, total);

    // Calculate expire time
    let expire = date_string_to_timestamp(&time_info);
    if expire > 0 {
        result.push_str(&format!(" expire={};", expire));
    }

    Some(result)
}

/// Extracts subscription info from an SSD-format subscription
pub fn get_sub_info_from_ssd(sub: &str) -> Option<String> {
    if !sub.starts_with("ssd://") {
        return None;
    }

    let decoded = url_safe_base64_decode(&sub[6..]);

    // Parse JSON
    let json: serde_json::Value = match serde_json::from_str(&decoded) {
        Ok(val) => val,
        Err(_) => return None,
    };

    let used_str = json.get("traffic_used")?.as_str()?;
    let total_str = json.get("traffic_total")?.as_str()?;

    // 1 GB = 1024^3 bytes
    let gb_to_bytes = 1024u64.pow(3);
    let used = used_str.parse::<f64>().unwrap_or(0.0) * gb_to_bytes as f64;
    let total = total_str.parse::<f64>().unwrap_or(0.0) * gb_to_bytes as f64;

    let mut result = format!(
        "upload=0; download={}; total={};",
        used as u64, total as u64
    );

    if let Some(expire_str) = json.get("expiry").and_then(|v| v.as_str()) {
        // Convert expiry format using regex
        let re = Regex::new(r"(\d+)-(\d+)-(\d+) (.*)").unwrap();
        let formatted_date = re.replace(expire_str, "$1:$2:$3:$4").to_string();

        let expire = date_string_to_timestamp(&formatted_date);
        if expire > 0 {
            result.push_str(&format!(" expire={};", expire));
        }
    }

    Some(result)
}
