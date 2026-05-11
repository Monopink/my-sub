use crate::models::{
    Proxy, SOCKS_DEFAULT_GROUP, SSR_DEFAULT_GROUP, SS_CIPHERS, SS_DEFAULT_GROUP,
};
use serde_json::{from_str, Value};

/// Parse a SSTap JSON configuration into a vector of Proxy objects
/// Based on the C++ implementation in explodeSSTap function
pub fn explode_sstap(content: &str, nodes: &mut Vec<Proxy>) -> bool {
    // Parse the JSON content
    let json: Value = match from_str(content) {
        Ok(json) => json,
        Err(_) => return false,
    };

    // Check if it has configs array
    if !json["configs"].is_array() {
        return false;
    }

    let configs = json["configs"].as_array().unwrap();
    if configs.is_empty() {
        return false;
    }

    let mut index = nodes.len();
    let mut success = false;

    for config in configs {
        // Extract common fields
        let group = config["group"].as_str().unwrap_or("");
        let remarks = config["remarks"].as_str().unwrap_or("");
        let server = config["server"].as_str().unwrap_or("");
        let port = config["server_port"].as_u64().unwrap_or(0) as u16;

        // Skip if port is 0
        if port == 0 {
            continue;
        }

        // Use server:port as remark if not provided
        let final_remarks = if remarks.is_empty() {
            format!("{} ({})", server, port)
        } else {
            remarks.to_string()
        };

        // Extract password
        let password = config["password"].as_str().unwrap_or("");

        // Get config type
        let config_type = config["type"].as_u64().unwrap_or(0);

        match config_type {
            5 => {
                // Socks 5
                let username = config["username"].as_str().unwrap_or("");

                // Create Socks5 proxy
                let mut node = Proxy::socks_construct(
                    if group.is_empty() {
                        SOCKS_DEFAULT_GROUP
                    } else {
                        group
                    },
                    &final_remarks,
                    server,
                    port,
                    username,
                    password,
                    None,
                    None,
                    None,
                    "",
                );

                node.id = index as u32;
                nodes.push(node);
                index += 1;
                success = true;
            }
            6 => {
                // SS/SSR
                let protocol = config["protocol"].as_str().unwrap_or("");
                let obfs = config["obfs"].as_str().unwrap_or("");
                let method = config["method"].as_str().unwrap_or("");

                // Check if it's SS or SSR
                if SS_CIPHERS.iter().any(|c| *c == method)
                    && protocol == "origin"
                    && obfs == "plain"
                {
                    // Is Shadowsocks
                    let mut node = Proxy::ss_construct(
                        if group.is_empty() {
                            SS_DEFAULT_GROUP
                        } else {
                            group
                        },
                        &final_remarks,
                        server,
                        port,
                        password,
                        method,
                        "",
                        "",
                        None,
                        None,
                        None,
                        None,
                        "",
                    );

                    node.id = index as u32;
                    nodes.push(node);
                    index += 1;
                    success = true;
                } else {
                    // Is ShadowsocksR
                    let obfs_param = config["obfsparam"].as_str().unwrap_or("");
                    let proto_param = config["protocolparam"].as_str().unwrap_or("");

                    let mut node = Proxy::ssr_construct(
                        if group.is_empty() {
                            SSR_DEFAULT_GROUP
                        } else {
                            group
                        },
                        &final_remarks,
                        server,
                        port,
                        protocol,
                        method,
                        obfs,
                        password,
                        obfs_param,
                        proto_param,
                        None,
                        None,
                        None,
                        "",
                    );

                    node.id = index as u32;
                    nodes.push(node);
                    index += 1;
                    success = true;
                }
            }
            _ => continue, // Skip unknown type
        }
    }

    success
}
