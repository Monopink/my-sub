//! User agent matching utilities
//!
//! This module provides functionality for parsing and matching user agent strings
//! to determine target formats and settings.

use crate::models::SubconverterTarget;

/// User agent profile structure
pub struct UAProfile {
    /// Beginning of user agent string to match
    pub head: String,
    /// Version string to look for
    pub version_match: String,
    /// Target version to compare with
    pub version_target: String,
    /// Target format to use
    pub target: SubconverterTarget,
    /// Whether to use new field names in Clash
    /// None means indeterminate (equivalent to tribool indeterminate state)
    pub clash_new_name: Option<bool>,
    /// Surge version
    pub surge_ver: i32,
}

impl UAProfile {
    /// Create a new UAProfile
    pub fn new(
        head: &str,
        version_match: &str,
        version_target: &str,
        target: SubconverterTarget,
        clash_new_name: Option<bool>,
        surge_ver: i32,
    ) -> Self {
        UAProfile {
            head: head.to_string(),
            version_match: version_match.to_string(),
            version_target: version_target.to_string(),
            target,
            clash_new_name,
            surge_ver,
        }
    }
}

/// Compare two version strings to check if source version is greater than or equal to target version
///
/// # Arguments
///
/// * `src_ver` - Source version string, like "1.2.3"
/// * `target_ver` - Target version string to compare with
///
/// # Returns
///
/// `true` if the source version is greater than or equal to the target version
pub fn ver_greater_equal(src_ver: &str, target_ver: &str) -> bool {
    // Create iterators for both version strings, splitting by dots
    let src_parts = src_ver.split('.').collect::<Vec<&str>>();
    let target_parts = target_ver.split('.').collect::<Vec<&str>>();

    // Compare each part of the version
    let min_len = src_parts.len().min(target_parts.len());
    for i in 0..min_len {
        // Parse to integers, default to 0 if parsing fails
        let src_part = src_parts[i].parse::<i32>().unwrap_or(0);
        let target_part = target_parts[i].parse::<i32>().unwrap_or(0);

        // If source part is greater, return true
        if src_part > target_part {
            return true;
        }
        // If source part is less, return false
        else if src_part < target_part {
            return false;
        }
        // If equal, continue to next part
    }

    // If all parts were equal, check if source has more parts than target
    // For example, 1.2.3 is greater than 1.2
    if src_parts.len() >= target_parts.len() {
        return true;
    }

    false
}

/// Match user agent string to determine target format and settings
///
/// # Arguments
///
/// * `user_agent` - User agent string to match
/// * `target` - Output parameter for target format
/// * `clash_new_name` - Output parameter for Clash new name setting (None = indeterminate)
/// * `surge_ver` - Output parameter for Surge version
///
/// # Returns
///
/// Updates the target, clash_new_name, and surge_ver parameters based on matching
pub fn match_user_agent(
    user_agent: &str,
    target: &mut SubconverterTarget,
    clash_new_name: &mut Option<bool>,
    surge_ver: &mut i32,
) {
    // Define user agent profiles to match C++ UAMatchList
    let ua_profiles = vec![
        // ClashForAndroid profiles
        UAProfile::new(
            "clashforandroid",
            "\\/([0-9.]+)",
            "2.0",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        UAProfile::new(
            "clashforandroid",
            "\\/([0-9.]+)r",
            "",
            SubconverterTarget::ClashR,
            Some(false), // False
            -1,
        ),
        UAProfile::new(
            "clashforandroid",
            "",
            "",
            SubconverterTarget::Clash,
            Some(false), // False
            -1,
        ),
        // ClashForWindows profiles
        UAProfile::new(
            "clashforwindows",
            "\\/([0-9.]+)",
            "0.11",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        UAProfile::new(
            "clashforwindows",
            "",
            "",
            SubconverterTarget::Clash,
            Some(false), // False
            -1,
        ),
        // Clash Verge
        UAProfile::new(
            "clash-verge",
            "",
            "",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        // ClashX Pro
        UAProfile::new(
            "clashx pro",
            "",
            "",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        // ClashX
        UAProfile::new(
            "clashx",
            "\\/([0-9.]+)",
            "0.13",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        // Generic Clash
        UAProfile::new(
            "clash",
            "",
            "",
            SubconverterTarget::Clash,
            Some(true), // True
            -1,
        ),
        // Kitsunebi
        UAProfile::new(
            "kitsunebi",
            "",
            "",
            SubconverterTarget::V2Ray,
            None, // Indeterminate
            -1,
        ),
        // Loon
        UAProfile::new(
            "loon",
            "",
            "",
            SubconverterTarget::Loon,
            None, // Indeterminate
            -1,
        ),
        // Pharos
        UAProfile::new(
            "pharos",
            "",
            "",
            SubconverterTarget::Mixed,
            None, // Indeterminate
            -1,
        ),
        // Potatso
        UAProfile::new(
            "potatso",
            "",
            "",
            SubconverterTarget::Mixed,
            None, // Indeterminate
            -1,
        ),
        // Quantumult X
        UAProfile::new(
            "quantumult%20x",
            "",
            "",
            SubconverterTarget::QuantumultX,
            None, // Indeterminate
            -1,
        ),
        // Quantumult
        UAProfile::new(
            "quantumult",
            "",
            "",
            SubconverterTarget::Quantumult,
            None, // Indeterminate
            -1,
        ),
        // Qv2ray
        UAProfile::new(
            "qv2ray",
            "",
            "",
            SubconverterTarget::V2Ray,
            None, // Indeterminate
            -1,
        ),
        // Shadowrocket
        UAProfile::new(
            "shadowrocket",
            "",
            "",
            SubconverterTarget::Mixed, // In original C++ it's "mixed"
            None,                      // Indeterminate
            -1,
        ),
        // Surfboard
        UAProfile::new(
            "surfboard",
            "",
            "",
            SubconverterTarget::Surfboard,
            None, // Indeterminate
            -1,
        ),
        // Surge Mac x86
        UAProfile::new(
            "surge",
            "\\/([0-9.]+).*x86",
            "906",
            SubconverterTarget::Surge(4),
            Some(false), // False
            4,
        ),
        UAProfile::new(
            "surge",
            "\\/([0-9.]+).*x86",
            "368",
            SubconverterTarget::Surge(3),
            Some(false), // False
            3,
        ),
        // Surge iOS
        UAProfile::new(
            "surge",
            "\\/([0-9.]+)",
            "1419",
            SubconverterTarget::Surge(4),
            Some(false), // False
            4,
        ),
        UAProfile::new(
            "surge",
            "\\/([0-9.]+)",
            "900",
            SubconverterTarget::Surge(3),
            Some(false), // False
            3,
        ),
        // Fallback for any Surge version
        UAProfile::new(
            "surge",
            "",
            "",
            SubconverterTarget::Surge(2),
            Some(false), // False
            2,
        ),
        // Trojan-Qt5
        UAProfile::new(
            "trojan-qt5",
            "",
            "",
            SubconverterTarget::Trojan,
            None, // Indeterminate
            -1,
        ),
        // V2rayU
        UAProfile::new(
            "v2rayu",
            "",
            "",
            SubconverterTarget::V2Ray,
            None, // Indeterminate
            -1,
        ),
        // V2RayX
        UAProfile::new(
            "v2rayx",
            "",
            "",
            SubconverterTarget::V2Ray,
            None, // Indeterminate
            -1,
        ),
        // SingBox (not in original C++ list but keep it)
        UAProfile::new(
            "sing-box",
            "",
            "",
            SubconverterTarget::SingBox,
            None, // Indeterminate
            -1,
        ),
    ];

    // Convert the user agent to lowercase for case-insensitive matching
    let user_agent_lower = user_agent.to_lowercase();

    for profile in ua_profiles {
        if user_agent_lower.contains(&profile.head) {
            // If a version string is specified, check if it matches and is greater than or equal to target version
            if !profile.version_match.is_empty() && !profile.version_target.is_empty() {
                if let Some(version_pos) = user_agent_lower.find(&profile.version_match) {
                    let version_begin = version_pos + profile.version_match.len();
                    if let Some(version_end) = user_agent_lower[version_begin..].find(' ') {
                        let version =
                            &user_agent_lower[version_begin..(version_begin + version_end)];
                        if ver_greater_equal(version, &profile.version_target) {
                            *target = profile.target;
                            *clash_new_name = profile.clash_new_name;
                            *surge_ver = profile.surge_ver;
                            return;
                        }
                    } else {
                        let version = &user_agent_lower[version_begin..];
                        if ver_greater_equal(version, &profile.version_target) {
                            *target = profile.target;
                            *clash_new_name = profile.clash_new_name;
                            *surge_ver = profile.surge_ver;
                            return;
                        }
                    }
                }
            } else {
                // If no version string specified, just match the head
                *target = profile.target;
                *clash_new_name = profile.clash_new_name;
                *surge_ver = profile.surge_ver;
                return;
            }
        }
    }
}
