use crate::models::{
    Proxy, HTTP_DEFAULT_GROUP, SNELL_DEFAULT_GROUP, SOCKS_DEFAULT_GROUP, SS_DEFAULT_GROUP,
    TROJAN_DEFAULT_GROUP, V2RAY_DEFAULT_GROUP,
};

/// Parse a Surge configuration into a vector of Proxy objects
pub fn explode_surge(content: &str, nodes: &mut Vec<Proxy>) -> bool {
    // Split the content into lines
    let lines: Vec<&str> = content.lines().collect();

    // Track the section we're currently in
    let mut in_proxy_section = false;
    let mut success = false;

    for line in lines {
        // Skip empty lines and comments
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Check section headers
        if line.starts_with('[') && line.ends_with(']') {
            in_proxy_section = line == "[Proxy]";
            continue;
        }

        // Only process lines in the [Proxy] section
        if !in_proxy_section {
            continue;
        }

        // Split by = to get name and configuration
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let name = parts[0].trim();
        let config = parts[1].trim();

        // Skip direct, reject, and reject-tinygif
        if config.starts_with("direct")
            || config.starts_with("reject")
            || config.starts_with("reject-tinygif")
        {
            continue;
        }

        // Parse the proxy based on the configuration format
        let mut node = Proxy::default();

        if config.starts_with("custom,") {
            // Surge 2 style custom proxy (essentially a shadowsocks proxy)
            if parse_surge_custom_ss(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("ss,") || config.starts_with("shadowsocks,") {
            // Surge 3 style ss proxy
            if parse_surge_ss(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("socks5") || config.starts_with("socks5-tls") {
            if parse_surge_socks(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("vmess,") {
            // Surge 4 style vmess proxy
            if parse_surge_vmess(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("http") || config.starts_with("https") {
            if parse_surge_http(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("trojan") {
            if parse_surge_trojan(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        } else if config.starts_with("snell") {
            if parse_surge_snell(config, name, &mut node) {
                nodes.push(node);
                success = true;
            }
        }
    }

    success
}

/// Parse a Surge 2 custom Shadowsocks configuration line
fn parse_surge_custom_ss(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts (custom,server,port,method,password,module)
    if parts.len() < 5 {
        return false;
    }

    // Extract the server, port, method, and password
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false;
    }

    let method = parts[3];
    let password = parts[4];

    // Default values
    let mut plugin = String::new();
    let mut plugin_opts = String::new();
    let mut pluginopts_mode = String::new();
    let mut pluginopts_host = String::new();
    let mut udp = None;
    let mut tfo = None;
    let scv = None;

    // Parse additional parameters
    for i in 6..parts.len() {
        if parts[i].contains('=') {
            let param_parts: Vec<&str> = parts[i].split('=').collect();
            if param_parts.len() != 2 {
                continue;
            }
            let key = param_parts[0].trim();
            let value = param_parts[1].trim();

            match key {
                "obfs" => {
                    plugin = "simple-obfs".to_string();
                    pluginopts_mode = value.to_string();
                }
                "obfs-host" => {
                    pluginopts_host = value.to_string();
                }
                "udp-relay" => {
                    udp = Some(value == "true" || value == "1");
                }
                "tfo" => {
                    tfo = Some(value == "true" || value == "1");
                }
                _ => {}
            }
        }
    }

    // Build plugin options if plugin is not empty
    if !plugin.is_empty() {
        plugin_opts = format!("obfs={}", pluginopts_mode);
        if !pluginopts_host.is_empty() {
            plugin_opts.push_str(&format!(";obfs-host={}", pluginopts_host));
        }
    }

    // Create the proxy object
    *node = Proxy::ss_construct(
        SS_DEFAULT_GROUP,
        name,
        server,
        port,
        password,
        method,
        &plugin,
        &plugin_opts,
        udp,
        tfo,
        scv,
        None,
        "",
    );

    true
}

/// Parse a Surge Shadowsocks configuration line
fn parse_surge_ss(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 3 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false;
    }

    // Default values
    let mut method = String::new();
    let mut password = String::new();
    let mut plugin = String::new();
    let mut plugin_opts = String::new();
    let mut pluginopts_mode = String::new();
    let mut pluginopts_host = String::new();
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    for i in 3..parts.len() {
        if parts[i].contains('=') {
            let param_parts: Vec<&str> = parts[i].split('=').collect();
            if param_parts.len() != 2 {
                continue;
            }
            let key = param_parts[0].trim();
            let value = param_parts[1].trim();

            match key {
                "encrypt-method" => {
                    method = value.to_string();
                }
                "password" => {
                    password = value.to_string();
                }
                "obfs" => {
                    plugin = "simple-obfs".to_string();
                    pluginopts_mode = value.to_string();
                }
                "obfs-host" => {
                    pluginopts_host = value.to_string();
                }
                "udp-relay" => {
                    udp = Some(value == "true" || value == "1");
                }
                "tfo" => {
                    tfo = Some(value == "true" || value == "1");
                }
                "skip-cert-verify" => {
                    scv = Some(value == "true" || value == "1");
                }
                _ => {}
            }
        }
    }

    // Build plugin options if plugin is not empty
    if !plugin.is_empty() {
        plugin_opts = format!("obfs={}", pluginopts_mode);
        if !pluginopts_host.is_empty() {
            plugin_opts.push_str(&format!(";obfs-host={}", pluginopts_host));
        }
    }

    // Create the proxy object
    *node = Proxy::ss_construct(
        SS_DEFAULT_GROUP,
        name,
        server,
        port,
        &password,
        &method,
        &plugin,
        &plugin_opts,
        udp,
        tfo,
        scv,
        None,
        "",
    );

    true
}

/// Parse a Surge HTTP/HTTPS configuration line
fn parse_surge_http(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 3 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Determine if it's HTTPS
    let is_https = parts[0] == "https";

    // Default values
    let mut username = "";
    let mut password = "";
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    for i in 3..parts.len() {
        if parts[i].starts_with("username=") {
            username = &parts[i][9..];
        } else if parts[i].starts_with("password=") {
            password = &parts[i][9..];
        } else if parts[i] == "tfo=true" {
            tfo = Some(true);
        } else if parts[i] == "skip-cert-verify=true" {
            scv = Some(true);
        }
    }

    // Create the proxy object
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

/// Parse a Surge SOCKS5 configuration line
fn parse_surge_socks(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 3 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Default values
    let mut username = "";
    let mut password = "";
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    if parts.len() >= 5 {
        username = parts[3];
        password = parts[4];
    }

    // Parse additional parameters
    for i in 5..parts.len() {
        if parts[i].contains('=') {
            let param_parts: Vec<&str> = parts[i].split('=').collect();
            if param_parts.len() != 2 {
                continue;
            }
            let key = param_parts[0].trim();
            let value = param_parts[1].trim();

            match key {
                "udp-relay" => {
                    udp = Some(value == "true" || value == "1");
                }
                "tfo" => {
                    tfo = Some(value == "true" || value == "1");
                }
                "skip-cert-verify" => {
                    scv = Some(value == "true" || value == "1");
                }
                _ => {}
            }
        }
    }

    // Create the proxy object
    *node = Proxy::socks_construct(
        SOCKS_DEFAULT_GROUP,
        name,
        server,
        port,
        username,
        password,
        udp,
        tfo,
        scv,
        "",
    );

    true
}

/// Parse a Surge VMess configuration line
fn parse_surge_vmess(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 3 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false;
    }

    // Default values
    let mut id = String::new();
    let mut net = "tcp".to_string();
    let method = "auto".to_string();
    let mut path = String::new();
    let mut host = String::new();
    let mut edge = String::new();
    let mut tls = String::new();
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;
    let mut tls13 = None;
    let mut aead = "1".to_string(); // Default to 1 for non-AEAD mode

    // Parse additional parameters
    for i in 3..parts.len() {
        if parts[i].contains('=') {
            let param_parts: Vec<&str> = parts[i].split('=').collect();
            if param_parts.len() != 2 {
                continue;
            }
            let key = param_parts[0].trim();
            let value = param_parts[1].trim();

            match key {
                "username" => {
                    id = value.to_string();
                }
                "ws" => {
                    net = if value == "true" {
                        "ws".to_string()
                    } else {
                        "tcp".to_string()
                    };
                }
                "tls" => {
                    tls = if value == "true" {
                        "tls".to_string()
                    } else {
                        String::new()
                    };
                }
                "ws-path" => {
                    path = value.to_string();
                }
                "obfs-host" => {
                    host = value.to_string();
                }
                "ws-headers" => {
                    // Parse headers in the format "Host:example.com|Edge:example.edge"
                    let headers: Vec<&str> = value.split('|').collect();
                    for header in headers {
                        let header_parts: Vec<&str> = header.split(':').collect();
                        if header_parts.len() == 2 {
                            let header_name = header_parts[0].trim().to_lowercase();
                            let header_value = header_parts[1].trim();
                            if header_name == "host" {
                                host = header_value.trim_matches('"').to_string();
                            } else if header_name == "edge" {
                                edge = header_value.trim_matches('"').to_string();
                            }
                        }
                    }
                }
                "udp-relay" => {
                    udp = Some(value == "true" || value == "1");
                }
                "tfo" => {
                    tfo = Some(value == "true" || value == "1");
                }
                "skip-cert-verify" => {
                    scv = Some(value == "true" || value == "1");
                }
                "tls13" => {
                    tls13 = Some(value == "true" || value == "1");
                }
                "vmess-aead" => {
                    aead = if value == "true" {
                        "0".to_string()
                    } else {
                        "1".to_string()
                    };
                }
                _ => {}
            }
        }
    }

    // Create the proxy object
    *node = Proxy::vmess_construct(
        V2RAY_DEFAULT_GROUP,
        name,
        server,
        port,
        "",
        &id,
        aead.parse::<u16>().unwrap_or(0),
        &net,
        &method,
        &path,
        &host,
        &edge,
        &tls,
        "",
        udp,
        tfo,
        scv,
        tls13,
        "",
    );

    true
}

/// Parse a Surge Trojan configuration line
fn parse_surge_trojan(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 4 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false;
    }

    // Default values
    let mut password = String::new();
    let mut host = String::new();
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    for i in 3..parts.len() {
        if parts[i].contains('=') {
            let param_parts: Vec<&str> = parts[i].split('=').collect();
            if param_parts.len() != 2 {
                continue;
            }
            let key = param_parts[0].trim();
            let value = param_parts[1].trim();

            match key {
                "password" => {
                    password = value.to_string();
                }
                "sni" => {
                    host = value.to_string();
                }
                "udp-relay" => {
                    udp = Some(value == "true" || value == "1");
                }
                "tfo" => {
                    tfo = Some(value == "true" || value == "1");
                }
                "skip-cert-verify" => {
                    scv = Some(value == "true" || value == "1");
                }
                _ => {}
            }
        }
    }

    // If password parameter not found, use the 4th part directly
    if password.is_empty() {
        password = parts[3].to_string();
        // Check if it has password= prefix
        if password.starts_with("password=") {
            password = password[9..].to_string();
        }
    }

    // Create the proxy object
    *node = Proxy::trojan_construct(
        TROJAN_DEFAULT_GROUP.to_string(),
        name.to_string(),
        server.to_string(),
        port,
        password,
        None,
        if host.is_empty() { None } else { Some(host) },
        None,
        None,
        true,
        udp,
        tfo,
        scv,
        None,
        None,
    );

    true
}

/// Parse a Surge Snell configuration line
fn parse_surge_snell(config: &str, name: &str, node: &mut Proxy) -> bool {
    // Split the configuration into parts
    let parts: Vec<&str> = config.split(',').map(|s| s.trim()).collect();

    // Check minimum required parts
    if parts.len() < 3 {
        return false;
    }

    // Extract the server and port
    let server = parts[1];
    let port_str = parts[2];
    let port = match port_str.parse::<u16>() {
        Ok(p) => p,
        Err(_) => return false,
    };
    if port == 0 {
        return false; // Skip if port is 0
    }

    // Default values
    let mut password = String::new();
    let mut plugin = String::new();
    let mut host = String::new();
    let mut version = String::new();
    let mut udp = None;
    let mut tfo = None;
    let mut scv = None;

    // Parse additional parameters
    for i in 3..parts.len() {
        // Split by equals sign
        let param_parts: Vec<&str> = parts[i].split('=').collect();
        if param_parts.len() != 2 {
            continue;
        }
        let key = param_parts[0].trim();
        let value = param_parts[1].trim();

        match key {
            "psk" => password = value.to_string(),
            "obfs" => plugin = value.to_string(),
            "obfs-host" => host = value.to_string(),
            "udp-relay" => udp = Some(value == "true" || value == "1"),
            "tfo" => tfo = Some(value == "true" || value == "1"),
            "skip-cert-verify" => scv = Some(value == "true" || value == "1"),
            "version" => version = value.to_string(),
            _ => {}
        }
    }

    if password.is_empty() {
        return false;
    }

    // Create the proxy object
    *node = Proxy::snell_construct(
        SNELL_DEFAULT_GROUP.to_string(),
        name.to_string(),
        server.to_string(),
        port,
        password.to_string(),
        plugin,
        host,
        version.parse::<u16>().unwrap_or(1),
        udp,
        tfo,
        scv,
        None,
    );

    true
}
