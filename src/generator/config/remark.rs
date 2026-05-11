//! Remark processing utilities
//!
//! This module provides functionality for processing proxy remarks.

use lazy_static::lazy_static;
use regex::Regex;

/// Processes a remark string according to a list of remark rules
///
/// # Arguments
///
/// * `remark` - The remark to process (will be modified in-place)
/// * `remarks_list` - List of already processed remarks to avoid duplicates
/// * `proc_comma` - Whether to process comma replacements
///
/// # Returns
///
/// Nothing, modifies the remark in-place
pub fn process_remark(remark: &mut String, remarks_list: &Vec<String>, proc_comma: bool) {
    // Replace every '=' with '-' in the remark string to avoid parse errors from clients
    *remark = remark.replace('=', "-");

    if proc_comma {
        // If the remark contains a comma, wrap it in quotes
        if remark.contains(',') {
            *remark = format!("\"{}\"", remark);
        }
    }

    // Ensure uniqueness by adding a number suffix if needed
    let mut temp_remark = remark.clone();
    let mut cnt = 2;
    while remarks_list.contains(&temp_remark) {
        temp_remark = format!("{} {}", remark, cnt);
        cnt += 1;
    }
    *remark = temp_remark;

    // Filter-related processing was added in the Rust implementation but isn't in
    // the original C++ processRemark function, so we'll keep it for additional functionality
    // Since it's specific to the Rust implementation, we'll run it after the C++ behavior
    // process_filters(remark, remarks_list);
}

/// Process filters in the remark string
pub fn process_filters(remark: &mut String, remarks_list: &Vec<String>) {
    lazy_static! {
        static ref SCRIPT_REGEX: Regex = Regex::new(r"\s*\[([^\]]*?)\]$").unwrap();
    }

    // Filter related
    for item in remarks_list {
        if item.starts_with("filter ") || item.starts_with("aerr ") {
            let left = item.find(' ').map(|pos| pos + 1).unwrap_or(0);
            if left >= item.len() {
                continue;
            }

            // Get filter arguments
            let arguments = &item[left..];

            // Match against the remark
            if arguments.starts_with("script:") {
                // TODO: Script-based filter (not fully implemented here)
                let _script_arg = &arguments[7..];
                // In C++ this would use a script engine
                // Here we'll simulate by just capturing any text in brackets
                if let Some(captures) = SCRIPT_REGEX.captures(remark) {
                    if let Some(_) = captures.get(1) {
                        // Remove the matched script section
                        *remark = remark[..remark.len() - captures.get(0).unwrap().as_str().len()]
                            .trim()
                            .to_string();
                    }
                }
            } else if arguments.starts_with("regex:") {
                // Regex-based filter
                let regex_arg = &arguments[6..];
                if let Ok(re) = Regex::new(regex_arg) {
                    if re.is_match(remark) {
                        if item.starts_with("filter") {
                            // For filter, we just remove the matched part
                            *remark = re.replace_all(remark, "").to_string();
                        } else {
                            // For aerr, we'd typically apply a different operation
                            // This is just a placeholder
                        }
                    }
                }
            } else {
                // Simple substring filter
                *remark = remark.replace(arguments, "");
            }
        }
    }

    // Remove duplicate spaces
    lazy_static! {
        static ref MULTI_SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    }
    *remark = MULTI_SPACE_REGEX.replace_all(remark, " ").to_string();
    *remark = remark.trim().to_string();
}
