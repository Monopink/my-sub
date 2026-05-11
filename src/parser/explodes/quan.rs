use crate::models::{
    Proxy, HTTP_DEFAULT_GROUP, SSR_DEFAULT_GROUP, SS_DEFAULT_GROUP, TROJAN_DEFAULT_GROUP,
    V2RAY_DEFAULT_GROUP,
};

/// Parse Quantumult configuration into a vector of Proxy objects
/// Consistent with the C++ implementation in explodeQuan
pub fn explode_quan(content: &str, nodes: &mut Vec<Proxy>) -> bool {
    // Split the content into lines
    let lines: Vec<&str> = content.lines().collect();

    let mut success = false;

    for line in lines {
        // Skip empty lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        // Check if this is a proxy line
        let mut node = Proxy::default();
        if parse_quan_line(line, &mut node) {
            nodes.push(node);
            success = true;
        }
    }

    success
}

/// Parse a single line from Quantumult configuration
/// Returns true if parsing was successful
fn parse_quan_line(line: &str, node: &mut Proxy) -> bool {
    // Different formats for Quantumult configuration lines:

    // Format: [name] = [type], [params...]
    let parts: Vec<&str> = line.splitn(2, " = ").collect();
    if parts.len() != 2 {
        return false;
    }

    let name = parts[0].trim();
    let config = parts[1].trim();

    let config_parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();
    if config_parts.is_empty() {
        return false;
    }

    // Determine the proxy type and parse accordingly
    match config_parts[0] {
        "vmess" => parse_quan_vmess(name, config_parts, node),
        "shadowsocks" => parse_quan_ss(name, config_parts, node),
        "shadowsocksr" => parse_quan_ssr(name, config_parts, node),
        "http" => parse_quan_http(name, config_parts, node),
        "trojan" => parse_quan_trojan(name, config_parts, node),
        _ => false,
    }
}

/// Parse a Quantumult Shadowsocks line
fn parse_quan_ss(name: &str, config_parts: Vec<&str>, node: &mut Proxy) -> bool {
    // Format: shadowsocks, [server], [port], [method], [password], [options]
    if config_parts.len() < 5 {
        return false;
    }

    // Extract basic parameters
    let server = config_parts[1];
    let port = match config_parts[2].parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }
    let method = config_parts[3];
    let password = config_parts[4];

    // Default values
    let mut plugin = "";
    let mut plugin_opts = "";
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional options
    for i in 5..config_parts.len() {
        if config_parts[i].starts_with("obfs=") {
            plugin = "obfs";

            let obfs_parts: Vec<&str> = config_parts[i][5..].split(',').collect();
            if !obfs_parts.is_empty() {
                let mut opts = format!("obfs={}", obfs_parts[0]);

                if obfs_parts.len() > 1 {
                    opts.push_str(&format!(";obfs-host={}", obfs_parts[1]));
                }

                plugin_opts = Box::leak(opts.into_boxed_str());
            }
        } else if config_parts[i] == "udp-relay=true" {
            udp = Some(true);
        } else if config_parts[i] == "fast-open=true" {
            tfo = Some(true);
        } else if config_parts[i] == "tls-verification=false" {
            scv = Some(true);
        }
    }

    *node = Proxy::ss_construct(
        SS_DEFAULT_GROUP,
        name,
        server,
        port,
        password,
        method,
        plugin,
        plugin_opts,
        udp,
        tfo,
        scv,
        None,
        "",
    );

    true
}

/// Parse a Quantumult ShadowsocksR line
fn parse_quan_ssr(name: &str, config_parts: Vec<&str>, node: &mut Proxy) -> bool {
    // Format: shadowsocksr, [server], [port], [method], [password], [protocol], [protocol_param], [obfs], [obfs_param], [options]
    if config_parts.len() < 9 {
        return false;
    }

    // Extract basic parameters
    let server = config_parts[1];
    let port = match config_parts[2].parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }
    let method = config_parts[3];
    let password = config_parts[4];
    let protocol = config_parts[5];
    let protocol_param = config_parts[6];
    let obfs = config_parts[7];
    let obfs_param = config_parts[8];

    // Default values
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional options
    for i in 9..config_parts.len() {
        if config_parts[i] == "udp-relay=true" {
            udp = Some(true);
        } else if config_parts[i] == "fast-open=true" {
            tfo = Some(true);
        } else if config_parts[i] == "tls-verification=false" {
            scv = Some(true);
        }
    }

    *node = Proxy::ssr_construct(
        SSR_DEFAULT_GROUP,
        name,
        server,
        port,
        protocol,
        method,
        obfs,
        password,
        obfs_param,
        protocol_param,
        udp,
        tfo,
        scv,
        "",
    );

    true
}

/// Parse a Quantumult VMess line
/// This implementation follows the C++ version closely
fn parse_quan_vmess(name: &str, config_parts: Vec<&str>, node: &mut Proxy) -> bool {
    // Format: vmess, [server], [port], [method], [uuid], [options]
    if config_parts.len() < 6 {
        return false;
    }

    // Extract basic parameters
    let server = config_parts[1];
    let port = match config_parts[2].parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }
    let cipher = config_parts[3];
    let uuid = config_parts[4].replace("\"", ""); // Remove quotes as in C++ replaceAllDistinct

    // Default values
    let mut group = V2RAY_DEFAULT_GROUP.to_string();
    let aid = "0".to_string();
    let mut net = "tcp".to_string();
    let mut path = String::new();
    let mut host = String::new();
    let mut edge = String::new();
    let mut tls = String::new();
    let fake_type = "none".to_string();

    // Parse additional options exactly like the C++ version
    for i in 5..config_parts.len() {
        let option_parts: Vec<&str> = config_parts[i].splitn(2, "=").collect();
        if option_parts.len() < 2 {
            continue;
        }

        let item_name = option_parts[0].trim();
        let item_val = option_parts[1].trim();

        match item_name {
            "group" => group = item_val.to_string(),
            "over-tls" => {
                tls = if item_val == "true" {
                    "tls".to_string()
                } else {
                    String::new()
                }
            }
            "tls-host" => host = item_val.to_string(),
            "obfs-path" => path = item_val.replace("\"", ""), // Remove quotes as in C++ replaceAllDistinct
            "obfs-header" => {
                // Parse headers similar to the C++ implementation
                let processed_val = item_val
                    .replace("\"", "")
                    .replace("\r\n", "|")
                    .replace("\n", "|");
                let headers: Vec<&str> = processed_val.split('|').collect();

                for header in headers {
                    if header.to_lowercase().starts_with("host: ") {
                        host = header[6..].to_string();
                    } else if header.to_lowercase().starts_with("edge: ") {
                        edge = header[6..].to_string();
                    }
                }
            }
            "obfs" => {
                if item_val == "ws" {
                    net = "ws".to_string();
                }
            }
            _ => {}
        }
    }

    // Set default path if empty
    if path.is_empty() {
        path = "/".to_string();
    }

    *node = Proxy::vmess_construct(
        &group,
        name,
        server,
        port,
        &fake_type,
        &uuid,
        aid.parse::<u16>().unwrap_or(0),
        &net,
        cipher,
        &path,
        &host,
        &edge,
        &tls,
        "", // SNI not set in C++ version
        None,
        None,
        None,
        None,
        "",
    );

    true
}

/// Parse a Quantumult HTTP/HTTPS line
fn parse_quan_http(name: &str, config_parts: Vec<&str>, node: &mut Proxy) -> bool {
    // Format: http, [server], [port], [username], [password], [options]
    if config_parts.len() < 3 {
        return false;
    }

    // Extract basic parameters
    let server = config_parts[1];
    let port = match config_parts[2].parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }

    // Default values
    let mut username = "";
    let mut password = "";
    let mut is_https = false;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    if config_parts.len() > 3 {
        username = config_parts[3];
    }

    if config_parts.len() > 4 {
        password = config_parts[4];
    }

    // Parse additional options
    for i in 5..config_parts.len() {
        if config_parts[i] == "over-tls=true" {
            is_https = true;
        } else if config_parts[i] == "fast-open=true" {
            tfo = Some(true);
        } else if config_parts[i] == "tls-verification=false" {
            scv = Some(true);
        }
    }

    *node = Proxy::http_construct(
        HTTP_DEFAULT_GROUP,
        name,
        server,
        port,
        username,
        password,
        is_https,
        tfo,
        scv,
        None,
        "",
    );

    true
}

/// Parse a Quantumult Trojan line
fn parse_quan_trojan(name: &str, config_parts: Vec<&str>, node: &mut Proxy) -> bool {
    // Format: trojan, [server], [port], [password], [options]
    if config_parts.len() < 4 {
        return false;
    }

    // Extract basic parameters
    let server = config_parts[1];
    let port = match config_parts[2].parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }
    let password = config_parts[3];

    // Default values
    let mut sni = None;
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional options
    for i in 4..config_parts.len() {
        if config_parts[i].starts_with("tls-host=") {
            sni = Some(config_parts[i][9..].to_string());
        } else if config_parts[i] == "udp-relay=true" {
            udp = Some(true);
        } else if config_parts[i] == "fast-open=true" {
            tfo = Some(true);
        } else if config_parts[i] == "tls-verification=false" {
            scv = Some(true);
        }
    }

    *node = Proxy::trojan_construct(
        TROJAN_DEFAULT_GROUP.to_string(),
        name.to_string(),
        server.to_string(),
        port,
        password.to_string(),
        None,
        sni.clone(),
        None,
        sni,
        true,
        udp,
        tfo,
        scv,
        None,
        None,
    );

    true
}
