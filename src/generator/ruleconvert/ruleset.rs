use std::collections::HashSet;
use std::str::FromStr;

/// Ruleset types for different proxy clients
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RulesetType {
    Surge,
    Quanx,
    ClashDomain,
    ClashIpcidr,
    ClashClassical,
}

impl FromStr for RulesetType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "surge" => Ok(RulesetType::Surge),
            "quanx" => Ok(RulesetType::Quanx),
            "clash_domain" => Ok(RulesetType::ClashDomain),
            "clash_ipcidr" => Ok(RulesetType::ClashIpcidr),
            "clash_classical" => Ok(RulesetType::ClashClassical),
            _ => Err(format!("Unknown ruleset type: {}", s)),
        }
    }
}

// Rule type lists for different proxy clients
lazy_static::lazy_static! {
    pub static ref BASIC_TYPES: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("DOMAIN");
        set.insert("DOMAIN-SUFFIX");
        set.insert("DOMAIN-KEYWORD");
        set.insert("IP-CIDR");
        set.insert("SRC-IP-CIDR");
        set.insert("GEOIP");
        set.insert("MATCH");
        set.insert("FINAL");
        set
    };

    pub static ref CLASH_RULE_TYPES: HashSet<&'static str> = {
        let mut set = BASIC_TYPES.clone();
        set.insert("IP-CIDR6");
        set.insert("SRC-PORT");
        set.insert("DST-PORT");
        set.insert("PROCESS-NAME");
        set
    };

    pub static ref SURGE2_RULE_TYPES: HashSet<&'static str> = {
        let mut set = BASIC_TYPES.clone();
        set.insert("IP-CIDR6");
        set.insert("USER-AGENT");
        set.insert("URL-REGEX");
        set.insert("PROCESS-NAME");
        set.insert("IN-PORT");
        set.insert("DEST-PORT");
        set.insert("SRC-IP");
        set
    };

    pub static ref SURGE_RULE_TYPES: HashSet<&'static str> = {
        let mut set = SURGE2_RULE_TYPES.clone();
        set.insert("AND");
        set.insert("OR");
        set.insert("NOT");
        set
    };

    pub static ref QUANX_RULE_TYPES: HashSet<&'static str> = {
        let mut set = BASIC_TYPES.clone();
        set.insert("USER-AGENT");
        set.insert("HOST");
        set.insert("HOST-SUFFIX");
        set.insert("HOST-KEYWORD");
        set
    };

    pub static ref SURF_RULE_TYPES: HashSet<&'static str> = {
        let mut set = BASIC_TYPES.clone();
        set.insert("IP-CIDR6");
        set.insert("PROCESS-NAME");
        set.insert("IN-PORT");
        set.insert("DEST-PORT");
        set.insert("SRC-IP");
        set
    };

    pub static ref SINGBOX_RULE_TYPES: HashSet<&'static str> = {
        let mut set = BASIC_TYPES.clone();
        set.insert("IP-VERSION");
        set.insert("INBOUND");
        set.insert("PROTOCOL");
        set.insert("NETWORK");
        set.insert("GEOSITE");
        set.insert("SRC-GEOIP");
        set.insert("DOMAIN-REGEX");
        set.insert("PROCESS-NAME");
        set.insert("PROCESS-PATH");
        set.insert("PACKAGE-NAME");
        set.insert("PORT");
        set.insert("PORT-RANGE");
        set.insert("SRC-PORT");
        set.insert("SRC-PORT-RANGE");
        set.insert("USER");
        set.insert("USER-ID");
        set
    };
}
