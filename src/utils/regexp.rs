//! Regular expression utilities
//!
//! This module provides utility functions for working with regular expressions,
//! similar to the C++ implementation in subconverter.

use regex::{Regex, RegexBuilder};

/// Checks if a regular expression pattern is valid
///
/// # Arguments
///
/// * `reg` - The regular expression pattern to validate
///
/// # Returns
///
/// `true` if the pattern is valid, `false` otherwise
pub fn reg_valid(reg: &str) -> bool {
    Regex::new(reg).is_ok()
}

/// Finds if a pattern matches anywhere in the string
///
/// # Arguments
///
/// * `src` - The source string to search in
/// * `match_pattern` - The pattern to search for
///
/// # Returns
///
/// `true` if the pattern is found, `false` otherwise
pub fn reg_find(src: &str, match_pattern: &str) -> bool {
    let (pattern, case_insensitive) = if match_pattern.starts_with("(?i)") {
        (&match_pattern[4..], true)
    } else {
        (match_pattern, false)
    };

    if let Ok(regex) = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(true)
        .build()
    {
        regex.is_match(src)
    } else {
        false
    }
}

/// Replaces matches of a pattern with a replacement string
///
/// # Arguments
///
/// * `src` - The source string
/// * `match_pattern` - The pattern to match
/// * `rep` - The replacement string
/// * `global` - Whether to replace all occurrences or just the first one
/// * `multiline` - Whether to enable multiline mode
///
/// # Returns
///
/// The string with replacements made
pub fn reg_replace(
    src: &str,
    match_pattern: &str,
    rep: &str,
    global: bool,
    multiline: bool,
) -> String {
    let (pattern, case_insensitive) = if match_pattern.starts_with("(?i)") {
        (&match_pattern[4..], true)
    } else {
        (match_pattern, false)
    };

    if let Ok(regex) = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(multiline)
        .build()
    {
        if global {
            regex.replace_all(src, rep).to_string()
        } else {
            regex.replace(src, rep).to_string()
        }
    } else {
        src.to_string()
    }
}

/// Checks if a string fully matches a pattern
///
/// # Arguments
///
/// * `src` - The source string
/// * `match_pattern` - The pattern to match
///
/// # Returns
///
/// `true` if the string fully matches the pattern, `false` otherwise
pub fn reg_match(src: &str, match_pattern: &str) -> bool {
    let (pattern, case_insensitive) = if match_pattern.starts_with("(?i)") {
        (&match_pattern[4..], true)
    } else {
        (match_pattern, false)
    };

    if let Ok(regex) = RegexBuilder::new(&format!("^{}$", pattern))
        .case_insensitive(case_insensitive)
        .build()
    {
        regex.is_match(src)
    } else {
        false
    }
}

/// Gets the capturing groups from a regex match
///
/// # Arguments
///
/// * `src` - The source string
/// * `match_pattern` - The pattern to match with capturing groups
///
/// # Returns
///
/// A vector of matched capturing groups, or an empty vector if no match
pub fn reg_get_match(src: &str, match_pattern: &str) -> Vec<String> {
    let (pattern, case_insensitive) = if match_pattern.starts_with("(?i)") {
        (&match_pattern[4..], true)
    } else {
        (match_pattern, false)
    };

    if let Ok(regex) = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(true)
        .build()
    {
        if let Some(caps) = regex.captures(src) {
            let mut results = Vec::new();
            for i in 0..caps.len() {
                if let Some(m) = caps.get(i) {
                    results.push(m.as_str().to_string());
                }
            }
            results
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

/// Gets all matches for a regex pattern
///
/// # Arguments
///
/// * `src` - The source string
/// * `match_pattern` - The pattern to match
/// * `group_only` - Whether to return only capturing groups
///
/// # Returns
///
/// A vector of matched strings
pub fn reg_get_all_match(src: &str, match_pattern: &str, group_only: bool) -> Vec<String> {
    let (pattern, case_insensitive) = if match_pattern.starts_with("(?i)") {
        (&match_pattern[4..], true)
    } else {
        (match_pattern, false)
    };

    let mut results = Vec::new();

    if let Ok(regex) = RegexBuilder::new(pattern)
        .case_insensitive(case_insensitive)
        .multi_line(true)
        .build()
    {
        if group_only {
            for caps in regex.captures_iter(src) {
                // Skip the 0th capture (the full match) when group_only is true
                for i in 1..caps.len() {
                    if let Some(m) = caps.get(i) {
                        results.push(m.as_str().to_string());
                    }
                }
            }
        } else {
            for caps in regex.captures_iter(src) {
                for i in 0..caps.len() {
                    if let Some(m) = caps.get(i) {
                        results.push(m.as_str().to_string());
                    }
                }
            }
        }
    }

    results
}

/// Trims whitespace from a string
///
/// # Arguments
///
/// * `src` - The source string
///
/// # Returns
///
/// The trimmed string
pub fn reg_trim(src: &str) -> String {
    src.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_valid() {
        assert!(reg_valid(r"^\d+$"));
        assert!(!reg_valid(r"[\d+"));
    }

    #[test]
    fn test_reg_find() {
        assert!(reg_find("hello world", r"world"));
        assert!(reg_find("HELLO world", r"(?i)hello"));
        assert!(!reg_find("hello world", r"universe"));
    }

    #[test]
    fn test_reg_replace() {
        assert_eq!(
            reg_replace("hello world", r"world", "universe", false, false),
            "hello universe"
        );
        assert_eq!(
            reg_replace("hello world world", r"world", "universe", true, false),
            "hello universe universe"
        );
        assert_eq!(
            reg_replace("hello world world", r"world", "universe", false, false),
            "hello universe world"
        );
    }

    #[test]
    fn test_reg_match() {
        assert!(reg_match("12345", r"^\d+$"));
        assert!(!reg_match("12345a", r"^\d+$"));
        assert!(reg_match("HELLO", r"(?i)hello"));
    }

    #[test]
    fn test_reg_get_match() {
        let result = reg_get_match("hello 12345 world", r"(\d+)");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "12345");
        assert_eq!(result[1], "12345");
    }

    #[test]
    fn test_reg_get_all_match() {
        let result = reg_get_all_match("hello 123 world 456", r"(\d+)", false);
        assert_eq!(result.len(), 4); // 2 matches, each with full match and group

        let group_only = reg_get_all_match("hello 123 world 456", r"(\d+)", true);
        assert_eq!(group_only.len(), 2);
        assert_eq!(group_only[0], "123");
        assert_eq!(group_only[1], "456");
    }

    #[test]
    fn test_reg_trim() {
        assert_eq!(reg_trim("  hello world  "), "hello world");
    }
}
