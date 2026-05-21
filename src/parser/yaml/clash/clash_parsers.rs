use crate::models::Proxy;
use crate::parser::yaml::clash::clash_proxy_types::ClashProxyYamlInput;
use log::warn;
use serde_yaml::{Mapping, Value};

use super::ClashYamlInput;

/// Parse Clash configuration from YAML string
///
/// This function is the Rust equivalent of the C++ `explodeClash` function.
/// The key improvements in this Rust implementation are:
/// 1. Type safety through enum variants in ClashProxyYamlInput
/// 2. Proper error handling with Result type
/// 3. Automatic deserialization using serde
/// 4. Cleaner pattern matching compared to C++ if/else chains
pub fn parse_clash_yaml(content: &str) -> Result<Vec<Proxy>, String> {
    let clash_input: ClashYamlInput = match serde_yaml::from_str(content) {
        Ok(input) => input,
        Err(e) => return Err(format!("Failed to parse Clash YAML: {}", e)),
    };

    let mut proxies = Vec::new();

    for (index, mut raw_proxy) in clash_input.extract_proxies().into_iter().enumerate() {
        normalize_proxy_value(&mut raw_proxy);

        let proxy = match serde_yaml::from_value::<ClashProxyYamlInput>(raw_proxy) {
            Ok(value) => value,
            Err(e) => {
                warn!("Skipping invalid Clash proxy at index {}: {}", index, e);
                continue;
            }
        };

        match proxy {
            ClashProxyYamlInput::Shadowsocks(ss) => {
                proxies.push(ss.into());
            }
            ClashProxyYamlInput::ShadowsocksR(ssr) => {
                proxies.push(ssr.into());
            }
            ClashProxyYamlInput::VMess(vmess) => {
                proxies.push(vmess.into());
            }
            ClashProxyYamlInput::Trojan(trojan) => {
                proxies.push(trojan.into());
            }
            ClashProxyYamlInput::Http(http) => {
                proxies.push(http.into());
            }
            ClashProxyYamlInput::Socks5(socks5) => {
                proxies.push(socks5.into());
            }
            ClashProxyYamlInput::Snell(snell) => {
                proxies.push(snell.into());
            }
            ClashProxyYamlInput::WireGuard(wg) => {
                proxies.push(wg.into());
            }
            ClashProxyYamlInput::Hysteria(hysteria) => {
                proxies.push(hysteria.into());
            }
            ClashProxyYamlInput::Hysteria2(hysteria2) => {
                proxies.push(hysteria2.into());
            }
            ClashProxyYamlInput::VLess(vless) => {
                proxies.push(vless.into());
            }
            ClashProxyYamlInput::AnyTls(anytls) => {
                proxies.push(anytls.into());
            }
            ClashProxyYamlInput::Unknown => {
                // Skip unknown proxy types
                warn!("Skipping unknown Clash proxy type at index {}", index);
            }
        }
    }

    Ok(proxies)
}

fn normalize_proxy_value(proxy: &mut Value) {
    let Some(map) = proxy.as_mapping_mut() else {
        return;
    };

    let Some(type_key) = find_case_insensitive_key(map, "type") else {
        return;
    };

    let raw_type = map
        .get(&type_key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase);

    let Some(raw_type) = raw_type else {
        return;
    };

    let normalized_type = match raw_type.as_str() {
        "shadowsocks" => "ss",
        "shadowsocksr" => "ssr",
        "socks" => "socks5",
        "any-tls" => "anytls",
        "https" => {
            set_case_insensitive_bool(map, "tls", true);
            "http"
        }
        _ => raw_type.as_str(),
    };

    map.insert(type_key, Value::String(normalized_type.to_string()));
}

fn find_case_insensitive_key(map: &Mapping, expected: &str) -> Option<Value> {
    map.keys().find_map(|key| match key {
        Value::String(actual) if actual.eq_ignore_ascii_case(expected) => Some(key.clone()),
        _ => None,
    })
}

fn set_case_insensitive_bool(map: &mut Mapping, key_name: &str, value: bool) {
    let target_key = find_case_insensitive_key(map, key_name)
        .unwrap_or_else(|| Value::String(key_name.to_string()));
    map.insert(target_key, Value::Bool(value));
}

#[cfg(test)]
mod tests {
    use super::parse_clash_yaml;
    use crate::models::proxy::ProxyType;

    #[test]
    fn parses_vless_with_string_port() {
        let yaml = r#"
proxies:
  - name: jp-vless
    type: vless
    server: example.com
    port: '443'
    uuid: 123-123-123-123-123
    flow: xtls-rprx-vision
    tls: true
    udp: true
    servername: example.com
"#;

        let proxies = parse_clash_yaml(yaml).expect("clash yaml should parse");
        assert_eq!(proxies.len(), 1);
        assert_eq!(proxies[0].proxy_type, ProxyType::Vless);
        assert_eq!(proxies[0].remark, "jp-vless");
        assert_eq!(proxies[0].port, 443);
    }

    #[test]
    fn keeps_valid_proxies_when_one_proxy_is_invalid() {
        let yaml = r#"
proxies:
  - name: broken-vless
    type: vless
    server: example.com
    port: abc
    uuid: 123-123-123-123-123
  - name: us-trojan
    type: trojan
    server: example.com
    port: 443
    password: 123-123-123-123-123
"#;

        let proxies = parse_clash_yaml(yaml).expect("clash yaml should parse");
        assert_eq!(proxies.len(), 1);
        assert_eq!(proxies[0].proxy_type, ProxyType::Trojan);
        assert_eq!(proxies[0].remark, "us-trojan");
    }

    #[test]
    fn supports_legacy_root_key_and_type_aliases() {
        let yaml = r#"
Proxy:
  - name: ss-legacy
    type: shadowsocks
    server: example.com
    port: "8388"
    cipher: aes-128-gcm
    password: p1
  - name: socks-legacy
    type: socks
    server: example.com
    port: 1080
  - name: https-legacy
    type: https
    server: example.com
    port: "443"
"#;

        let proxies = parse_clash_yaml(yaml).expect("legacy clash yaml should parse");
        assert_eq!(proxies.len(), 3);
        assert_eq!(proxies[0].proxy_type, ProxyType::Shadowsocks);
        assert_eq!(proxies[1].proxy_type, ProxyType::Socks5);
        assert_eq!(proxies[2].proxy_type, ProxyType::HTTPS);
    }

    #[test]
    fn parses_hysteria2_with_alpn_array() {
        let yaml = r#"
proxies:
  - type: hysteria2
    name: hk-hy2
    server: example.com
    port: 10030
    password: 123-123-123-123-123
    auth: 123
    udp: true
    alpn:
      - h3
    sni: example.com
    obfs: salamander
    obfs-password: 123
"#;

        let proxies = parse_clash_yaml(yaml).expect("hysteria2 yaml should parse");
        assert_eq!(proxies.len(), 1);
        assert_eq!(proxies[0].proxy_type, ProxyType::Hysteria2);
        assert_eq!(proxies[0].remark, "hk-hy2");
        assert!(proxies[0].alpn.contains("h3"));
    }
}
