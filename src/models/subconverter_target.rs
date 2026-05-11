use crate::settings::external::ExternalSettings;

/// The output format for subconverter
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum SubconverterTarget {
    Auto,
    Clash,
    ClashR,
    Surge(i32), // Surge version as parameter
    Surfboard,
    Mellow,
    SSSub,
    SS,
    SSR,
    V2Ray,
    Trojan,
    Mixed,
    Quantumult,
    QuantumultX,
    Loon,
    SSD,
    SingBox,
}

impl SubconverterTarget {
    /// Convert string to target enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Some(SubconverterTarget::Auto),
            "clash" => Some(SubconverterTarget::Clash),
            "clashr" => Some(SubconverterTarget::ClashR),
            "surge" => Some(SubconverterTarget::Surge(3)), // Default to Surge 3
            "surfboard" => Some(SubconverterTarget::Surfboard),
            "mellow" => Some(SubconverterTarget::Mellow),
            "sssub" => Some(SubconverterTarget::SSSub),
            "ss" => Some(SubconverterTarget::SS),
            "ssr" => Some(SubconverterTarget::SSR),
            "v2ray" => Some(SubconverterTarget::V2Ray),
            "trojan" => Some(SubconverterTarget::Trojan),
            "mixed" => Some(SubconverterTarget::Mixed),
            "quan" => Some(SubconverterTarget::Quantumult),
            "quanx" => Some(SubconverterTarget::QuantumultX),
            "loon" => Some(SubconverterTarget::Loon),
            "ssd" => Some(SubconverterTarget::SSD),
            "singbox" => Some(SubconverterTarget::SingBox),
            // Map shadowrocket to Mixed
            "shadowrocket" => Some(SubconverterTarget::Mixed),
            // Map surfboardios to regular Surfboard
            "surfboardios" => Some(SubconverterTarget::Surfboard),
            _ => None,
        }
    }

    /// Convert target enum to string
    pub fn to_str(&self) -> String {
        match self {
            SubconverterTarget::Auto => "auto".to_string(),
            SubconverterTarget::Clash => "clash".to_string(),
            SubconverterTarget::ClashR => "clashr".to_string(),
            SubconverterTarget::Surge(ver) => format!("surge{}", ver),
            SubconverterTarget::Surfboard => "surfboard".to_string(),
            SubconverterTarget::Mellow => "mellow".to_string(),
            SubconverterTarget::SSSub => "sssub".to_string(),
            SubconverterTarget::SS => "ss".to_string(),
            SubconverterTarget::SSR => "ssr".to_string(),
            SubconverterTarget::V2Ray => "v2ray".to_string(),
            SubconverterTarget::Trojan => "trojan".to_string(),
            SubconverterTarget::Mixed => "mixed".to_string(),
            SubconverterTarget::Quantumult => "quan".to_string(),
            SubconverterTarget::QuantumultX => "quanx".to_string(),
            SubconverterTarget::Loon => "loon".to_string(),
            SubconverterTarget::SSD => "ssd".to_string(),
            SubconverterTarget::SingBox => "singbox".to_string(),
        }
    }

    pub fn is_clash(&self) -> bool {
        matches!(self, SubconverterTarget::Clash | SubconverterTarget::ClashR)
    }

    /// Returns true if the target represents a simple type (e.g. ss, ssr, trojan)
    pub fn is_simple(&self) -> bool {
        matches!(
            self,
            SubconverterTarget::SS
                | SubconverterTarget::SSR
                | SubconverterTarget::SSD
                | SubconverterTarget::V2Ray
                | SubconverterTarget::Trojan
        )
    }

    /// Gets the base content for this target from the external config
    pub fn get_base_content_from_external(&self, external: &ExternalSettings) -> Option<String> {
        match self {
            SubconverterTarget::Clash | SubconverterTarget::ClashR => {
                if !external.clash_rule_base.is_empty() {
                    Some(external.clash_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::Surge(_) => {
                if !external.surge_rule_base.is_empty() {
                    Some(external.surge_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::Surfboard => {
                if !external.surfboard_rule_base.is_empty() {
                    Some(external.surfboard_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::Mellow => {
                if !external.mellow_rule_base.is_empty() {
                    Some(external.mellow_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::Quantumult => {
                if !external.quan_rule_base.is_empty() {
                    Some(external.quan_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::QuantumultX => {
                if !external.quanx_rule_base.is_empty() {
                    Some(external.quanx_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::Loon => {
                if !external.loon_rule_base.is_empty() {
                    Some(external.loon_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::SSSub => {
                if !external.sssub_rule_base.is_empty() {
                    Some(external.sssub_rule_base.clone())
                } else {
                    None
                }
            }
            SubconverterTarget::SingBox => {
                if !external.singbox_rule_base.is_empty() {
                    Some(external.singbox_rule_base.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
