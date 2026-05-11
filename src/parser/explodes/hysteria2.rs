use crate::{models::HYSTERIA2_DEFAULT_GROUP, utils::url_decode, Proxy};
use url::Url;

/// Parse a Hysteria2 link into a Proxy object
pub fn explode_hysteria2(hysteria2: &str, node: &mut Proxy) -> bool {
    // Check if the link starts with hysteria2://
    if !hysteria2.starts_with("hysteria2://") && !hysteria2.starts_with("hy2://") {
        return false;
    }

    // Parse the URL
    let url = match Url::parse(hysteria2) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract host and port
    let host = url.host_str().unwrap_or("");
    let port = url.port().unwrap_or(443);

    // Extract password (username in URL)
    let password = url.username();

    // Extract parameters from the query string
    let mut up_speed = None;
    let mut down_speed = None;
    let mut obfs = String::new();
    let mut obfs_param = String::new();
    let mut sni = String::new();
    let mut fingerprint = String::new();
    let mut ca = String::new();
    let mut ca_str = String::new();
    let mut cwnd = None;
    let mut allow_insecure = None;
    let mut ports = String::new();
    let mut alpn = Vec::new();

    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "up" => up_speed = value.parse::<u32>().ok(),
            "down" => down_speed = value.parse::<u32>().ok(),
            "obfs" => obfs = url_decode(&value),
            "obfs-password" => obfs_param = url_decode(&value),
            "sni" => sni = url_decode(&value),
            "insecure" => {
                allow_insecure =
                    Some(value.as_ref() == "1" || value.as_ref().to_lowercase() == "true")
            }
            "fingerprint" => fingerprint = url_decode(&value),
            "ca" => ca = url_decode(&value),
            "caStr" => ca_str = url_decode(&value),
            "ports" => ports = url_decode(&value),
            "mport" => ports = url_decode(&value),
            "cwnd" => cwnd = value.parse::<u32>().ok(),
            "alpn" => {
                for a in url_decode(&value).split(',') {
                    alpn.push(a.to_string());
                }
            }
            _ => {}
        }
    }

    // Extract remark from the fragment
    let remark = url_decode(url.fragment().unwrap_or(""));

    // Create formatted strings
    let remark_str = if remark.is_empty() {
        format!("{} ({})", host, port)
    } else {
        remark.to_string()
    };

    // Create the proxy object
    *node = Proxy::hysteria2_construct(
        HYSTERIA2_DEFAULT_GROUP.to_string(),
        remark_str,
        host.to_string(),
        port,
        Some(ports),
        up_speed,
        down_speed,
        password.to_string(),
        Some(obfs),
        Some(obfs_param),
        Some(sni),
        Some(fingerprint),
        alpn,
        Some(ca),
        Some(ca_str),
        cwnd,
        None,
        allow_insecure,
        None,
    );

    true
}

/// Parse a standard Hysteria2 link into a Proxy object (handles hy2:// scheme)
pub fn explode_std_hysteria2(hysteria2: &str, node: &mut Proxy) -> bool {
    // Check if the link starts with hy2://
    if !hysteria2.starts_with("hy2://") {
        return false;
    }

    // Parse the URL
    let url = match Url::parse(hysteria2) {
        Ok(url) => url,
        Err(_) => return false,
    };

    // Extract host and port
    let host = url.host_str().unwrap_or("");
    let port = url.port().unwrap_or(443);

    // Extract password (username in URL)
    let password = url.username();

    // Extract parameters from the query string
    let mut up_speed = None;
    let mut down_speed = None;
    let mut obfs = String::new();
    let mut obfs_param = String::new();
    let mut sni = String::new();
    let mut fingerprint = String::new();
    let mut ca = String::new();
    let ca_str = String::new();
    let mut cwnd = None;
    let mut allow_insecure = None;
    let mut ports = String::new();
    let mut alpn = Vec::new();

    for (key, value) in url.query_pairs() {
        let value_decoded = url_decode(&value);
        match key.as_ref() {
            "bandwidth" => {
                let parts: Vec<&str> = value_decoded.split(',').collect();
                if parts.len() >= 1 {
                    up_speed = parts[0].parse::<u32>().ok();
                }
                if parts.len() >= 2 {
                    down_speed = parts[1].parse::<u32>().ok();
                }
            }
            "obfs" => obfs = value_decoded,
            "obfs-password" => obfs_param = value_decoded,
            "sni" => sni = value_decoded,
            "insecure" => {
                allow_insecure =
                    Some(value.as_ref() == "1" || value.as_ref().to_lowercase() == "true")
            }
            "pinSHA256" => fingerprint = value_decoded,
            "ca" => ca = value_decoded,
            "ports" => ports = value_decoded,
            "cwnd" => cwnd = value.parse::<u32>().ok(),
            "alpn" => {
                for a in value_decoded.split(',') {
                    alpn.push(a.to_string());
                }
            }
            _ => {}
        }
    }

    // Extract remark from the fragment
    let remark = url_decode(url.fragment().unwrap_or(""));

    // Create formatted strings
    let remark_str = if remark.is_empty() {
        format!("{} ({})", host, port)
    } else {
        remark.to_string()
    };

    // Create the proxy object
    *node = Proxy::hysteria2_construct(
        HYSTERIA2_DEFAULT_GROUP.to_string(),
        remark_str,
        host.to_string(),
        port,
        Some(ports),
        up_speed,
        down_speed,
        password.to_string(),
        Some(obfs),
        Some(obfs_param),
        Some(sni),
        Some(fingerprint),
        alpn,
        Some(ca),
        Some(ca_str),
        cwnd,
        None,
        allow_insecure,
        None,
    );

    true
}
