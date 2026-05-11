use crate::models::{Proxy, SOCKS_DEFAULT_GROUP};
use crate::utils::base64::url_safe_base64_decode;
use std::collections::HashMap;
use url::Url;

/// Parse a SOCKS link into a Proxy object
pub fn explode_socks(link: &str, node: &mut Proxy) -> bool {
    // Check if it's a v2rayN style socks link
    if link.starts_with("socks://") {
        return parse_v2rayn_socks(link, node);
    }
    // Check if it's a Telegram style socks link
    else if link.starts_with("https://t.me/socks") || link.starts_with("tg://socks") {
        return parse_telegram_socks(link, node);
    }

    false
}

/// Parse a v2rayN style socks link
/// Format: socks://BASE64(username:password@server:port)#remarks
fn parse_v2rayn_socks(link: &str, node: &mut Proxy) -> bool {
    // Extract remarks if present
    let mut remarks = String::new();
    let mut trimmed_link = link.to_string();
    if let Some(pos) = link.find('#') {
        remarks = link[pos + 1..].to_string();
        trimmed_link = link[..pos].to_string();
    }

    // Decode the base64 part
    let decoded = url_safe_base64_decode(&trimmed_link[8..]);
    if decoded.is_empty() {
        return false;
    }

    // Parse the decoded content
    let mut username = String::new();
    let mut password = String::new();
    let mut _server = String::new();
    let mut _port = 0;

    if decoded.contains('@') {
        let parts: Vec<&str> = decoded.split('@').collect();
        if parts.len() < 2 {
            return false;
        }

        // Parse userinfo
        let userinfo: Vec<&str> = parts[0].split(':').collect();
        if userinfo.len() < 2 {
            return false;
        }
        username = userinfo[0].to_string();
        password = userinfo[1].to_string();

        // Parse server and port
        let server_port: Vec<&str> = parts[1].split(':').collect();
        if server_port.len() < 2 {
            return false;
        }
        _server = server_port[0].to_string();
        _port = match server_port[1].parse::<u16>() {
            Ok(p) => p,
            Err(_) => return false,
        };
    } else {
        // No authentication, just server and port
        let server_port: Vec<&str> = decoded.split(':').collect();
        if server_port.len() < 2 {
            return false;
        }
        _server = server_port[0].to_string();
        _port = match server_port[1].parse::<u16>() {
            Ok(p) => p,
            Err(_) => return false,
        };
    }

    if _port == 0 {
        return false;
    }

    // Use default remark if none provided
    if remarks.is_empty() {
        remarks = format!("{} ({})", _server, _port);
    }

    // Create the proxy object
    *node = Proxy::socks_construct(
        SOCKS_DEFAULT_GROUP,
        &remarks,
        &_server,
        _port,
        &username,
        &password,
        None,
        None,
        None,
        "",
    );

    true
}

/// Parse a Telegram style socks link
/// Format: tg://socks?server=x&port=x&user=x&pass=x&remarks=x&group=x
/// or https://t.me/socks?server=x&port=x&user=x&pass=x&remarks=x&group=x
fn parse_telegram_socks(link: &str, node: &mut Proxy) -> bool {
    // Try to parse as URL
    let url = match Url::parse(link) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract query parameters
    let query_pairs: HashMap<String, String> = url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    // Get required parameters
    let server = match query_pairs.get("server") {
        Some(s) => s,
        None => return false,
    };

    let port_str = match query_pairs.get("port") {
        Some(p) => p,
        None => return false,
    };

    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };

    if port == 0 {
        return false;
    }

    // Get optional parameters
    let username = query_pairs.get("user").map_or("", |s| s);
    let password = query_pairs.get("pass").map_or("", |s| s);

    let group = query_pairs.get("group").map_or(SOCKS_DEFAULT_GROUP, |s| s);

    let remarks = if let Some(r) = query_pairs.get("remarks") {
        if !r.is_empty() {
            r.as_str()
        } else {
            &format!("{} ({})", server, port)
        }
    } else {
        &format!("{} ({})", server, port)
    };

    // Create the proxy object
    *node = Proxy::socks_construct(
        group, remarks, server, port, username, password, None, None, None, "",
    );

    true
}
