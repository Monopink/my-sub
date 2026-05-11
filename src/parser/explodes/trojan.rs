use crate::{models::TROJAN_DEFAULT_GROUP, utils::url_decode, Proxy};
use std::collections::HashMap;
use url::Url;

/// Parse a Trojan link into a Proxy object
pub fn explode_trojan(trojan: &str, node: &mut Proxy) -> bool {
    // Check if the link starts with trojan://
    if !trojan.starts_with("trojan://") {
        return false;
    }

    // Try to parse as URL
    let url = match Url::parse(trojan) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract password
    let password = url.username();
    if password.is_empty() {
        return false;
    }

    // Extract host and port
    let host = match url.host_str() {
        Some(host) => host,
        None => return false,
    };
    let port = url.port().unwrap_or(443);

    // Skip if port is 0
    if port == 0 {
        return false;
    }

    // Extract parameters from the query string
    let mut params = HashMap::new();
    for (key, value) in url.query_pairs() {
        params.insert(key.to_string(), url_decode(&value));
    }

    // Extract SNI - check for both "sni" and "peer" parameters (like in C++)
    let sni = params
        .get("sni")
        .map(|s| s.to_string())
        .or_else(|| params.get("peer").map(|s| s.to_string()));

    // Extract TLS verification setting
    let skip_cert_verify = params
        .get("allowInsecure")
        .map(|s| s == "1" || s.to_lowercase() == "true");

    // Extract TCP Fast Open setting
    let tfo = params
        .get("tfo")
        .map(|s| s == "1" || s.to_lowercase() == "true");

    // Extract group parameter
    let group = params
        .get("group")
        .map(|s| url_decode(s))
        .unwrap_or_else(|| TROJAN_DEFAULT_GROUP.to_string());

    // Handle WebSocket support
    let mut network = None;
    let mut path = None;

    if params.get("ws").map(|s| s == "1").unwrap_or(false) {
        network = Some("ws".to_string());
        path = params.get("wspath").map(|s| s.to_string());
    } else if params.get("type").map(|s| s == "ws").unwrap_or(false) {
        network = Some("ws".to_string());
        if let Some(p) = params.get("path") {
            let p_str = p.to_string();
            if p_str.starts_with("%2F") {
                path = Some(url_decode(&p_str));
            } else {
                path = Some(p_str);
            }
        }
    }

    // Extract remark from the fragment
    let remark = url_decode(&url.fragment().unwrap_or(""));
    let formatted_remark = if remark.is_empty() {
        format!("{} ({})", host, port)
    } else {
        remark.to_string()
    };

    // Create the proxy object
    *node = Proxy::trojan_construct(
        group,
        formatted_remark,
        host.to_string(),
        port,
        password.to_string(),
        network,
        sni.clone(),
        path,
        sni,
        true,             // tls_secure
        None,             // udp
        tfo,              // tfo
        skip_cert_verify, // allow_insecure
        None,             // tls13
        None,             // underlying_proxy
    );

    true
}

/// Parse a Trojan-Go link into a Proxy object
pub fn explode_trojan_go(trojan_go: &str, node: &mut Proxy) -> bool {
    // Check if the link starts with trojan-go://
    if !trojan_go.starts_with("trojan-go://") {
        return false;
    }

    // Try to parse as URL
    let url = match Url::parse(trojan_go) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract password
    let password = url.username();
    if password.is_empty() {
        return false;
    }

    // Extract host and port
    let host = match url.host_str() {
        Some(host) => host,
        None => return false,
    };
    let port = url.port().unwrap_or(443);

    // Skip if port is 0
    if port == 0 {
        return false;
    }

    // Extract parameters from the query string
    let mut params = HashMap::new();
    for (key, value) in url.query_pairs() {
        params.insert(key.to_string(), url_decode(&value));
    }

    // Extract network, host, path
    let network = params.get("type").map(|s| s.to_string());
    let host_param = params.get("host").map(|s| s.to_string());
    let path = params.get("path").map(|s| s.to_string());
    let sni = params.get("sni").map(|s| s.to_string());
    // Extract TLS verification setting
    let skip_cert_verify = params
        .get("allowInsecure")
        .map(|s| s == "1" || s.to_lowercase() == "true");

    // Extract TFO setting
    let tfo = params
        .get("tfo")
        .map(|s| s == "1" || s.to_lowercase() == "true");

    // Extract group parameter
    let group = params
        .get("group")
        .map_or_else(|| TROJAN_DEFAULT_GROUP, |s| s);

    // Extract remark from the fragment
    let remark = url_decode(url.fragment().unwrap_or(""));
    let formatted_remark = if remark.is_empty() {
        format!("{} ({})", host, port)
    } else {
        remark.to_string()
    };

    // Create the proxy object
    *node = Proxy::trojan_construct(
        group.to_string(),
        formatted_remark,
        host.to_string(),
        port,
        password.to_string(),
        network,
        host_param,
        path,
        sni,
        true,             // tls_secure
        None,             // udp
        tfo,              // tfo
        skip_cert_verify, // allow_insecure
        None,             // tls13
        None,             // underlying_proxy
    );

    true
}
