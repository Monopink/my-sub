use crate::models::Proxy;
use crate::parser::yaml::clash::clash_proxy_types::ClashProxyYamlInput;
use log::warn;

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

    for (index, raw_proxy) in clash_input.extract_proxies().into_iter().enumerate() {
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
}
