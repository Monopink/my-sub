use crate::{
    models::{Proxy, SNELL_DEFAULT_GROUP},
    utils::url_decode,
};
use std::collections::HashMap;
use url::Url;

/// Parse a Snell link into a Proxy object
pub fn explode_snell(snell: &str, node: &mut Proxy) -> bool {
    // Check if the link starts with snell://
    if !snell.starts_with("snell://") {
        return false;
    }

    // Try to parse as URL
    let url = match Url::parse(snell) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract host and port
    let host = match url.host_str() {
        Some(host) => host,
        None => return false,
    };
    let port = url.port().unwrap_or(8388);
    if port == 0 {
        return false; // Skip if port is 0
    }

    // Extract password (username in URL)
    let password = url.username();
    if password.is_empty() {
        return false;
    }

    // Extract parameters from the query string
    let mut params = HashMap::new();
    for (key, value) in url.query_pairs() {
        params.insert(key.to_string(), url_decode(&value));
    }

    // Extract obfs
    let obfs = params.get("obfs").map(|s| s.as_str()).unwrap_or("none");

    // Extract host
    let host_param = params.get("host").map(|s| s.as_str()).unwrap_or("");

    // Extract version
    let version = params
        .get("version")
        .map(|s| s.parse::<u16>().unwrap_or(1))
        .unwrap_or(1);

    // Extract UDP, TFO, and allow_insecure flags
    let udp = params.get("udp").map(|s| s == "true" || s == "1");
    let tfo = params.get("tfo").map(|s| s == "true" || s == "1");
    let allow_insecure = params
        .get("skip-cert-verify")
        .map(|s| s == "true" || s == "1");

    // Extract remark from the fragment
    let remark = url_decode(url.fragment().unwrap_or(""));
    let formatted_remark = if remark.is_empty() {
        format!("{} ({})", host, port)
    } else {
        remark.to_string()
    };

    // Create the proxy object
    *node = Proxy::snell_construct(
        SNELL_DEFAULT_GROUP.to_string(),
        formatted_remark,
        host.to_string(),
        port,
        password.to_string(),
        obfs.to_string(),
        host_param.to_string(),
        version,
        udp,
        tfo,
        allow_insecure,
        None,
    );

    true
}

/// Parse a Snell configuration in Surge format
/// Format: snell = server, port, psk=password, obfs=obfs, obfs-host=host, version=version
pub fn explode_snell_surge(surge: &str, node: &mut Proxy) -> bool {
    if !surge.starts_with("snell") {
        return false;
    }

    // Split the line by commas and remove spaces
    let parts: Vec<&str> = surge.split(',').map(|s| s.trim()).collect();
    if parts.len() < 3 {
        return false;
    }

    // Extract server and port
    let server_part = parts[0];
    let server = if server_part.contains('=') {
        server_part
            .split('=')
            .nth(1)
            .unwrap_or("")
            .trim()
            .to_string()
    } else {
        server_part.replace("snell", "").trim().to_string()
    };
    if server.is_empty() {
        return false;
    }

    let port_str = parts[1];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false;
    }

    // Default values
    let mut password = String::new();
    let mut obfs = String::new();
    let mut obfs_host = String::new();
    let mut version = 1u16;
    let mut udp = None;
    let mut tfo = None;
    let mut allow_insecure = None;

    // Parse additional parameters
    for i in 2..parts.len() {
        let param_parts: Vec<&str> = parts[i].split('=').collect();
        if param_parts.len() != 2 {
            continue;
        }
        let key = param_parts[0].trim();
        let value = param_parts[1].trim();

        match key {
            "psk" => password = value.to_string(),
            "obfs" => obfs = value.to_string(),
            "obfs-host" => obfs_host = value.to_string(),
            "version" => version = value.parse::<u16>().unwrap_or(1),
            "udp" | "udp-relay" => udp = Some(value == "true" || value == "1"),
            "tfo" => tfo = Some(value == "true" || value == "1"),
            "skip-cert-verify" => allow_insecure = Some(value == "true" || value == "1"),
            _ => {}
        }
    }

    if password.is_empty() {
        return false;
    }

    // Create remark
    let remark = format!("{} ({})", server, port);

    // Create the proxy object
    *node = Proxy::snell_construct(
        SNELL_DEFAULT_GROUP.to_string(),
        remark,
        server,
        port,
        password,
        obfs,
        obfs_host,
        version,
        udp,
        tfo,
        allow_insecure,
        None,
    );

    true
}
