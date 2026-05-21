use crate::{models::Proxy, parser::yaml::clash::parse_clash_yaml};

/// Parse a Clash YAML configuration into a vector of Proxy objects.
///
/// The parser uses the unified typed Clash parser and intentionally avoids
/// legacy fallback branches to keep parsing behavior deterministic.
pub fn explode_clash(content: &str, nodes: &mut Vec<Proxy>) -> bool {
    match parse_clash_yaml(content) {
        Ok(mut proxies) => {
            if proxies.is_empty() {
                return false;
            }
            nodes.append(&mut proxies);
            true
        }
        Err(_) => false,
    }
}
