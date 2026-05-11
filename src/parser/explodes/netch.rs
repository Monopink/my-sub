use crate::models::{
    Proxy, HTTP_DEFAULT_GROUP, SOCKS_DEFAULT_GROUP, SSR_DEFAULT_GROUP, SS_DEFAULT_GROUP,
    TROJAN_DEFAULT_GROUP, V2RAY_DEFAULT_GROUP,
};
use crate::utils::base64::{base64_encode, url_safe_base64_decode};
use serde_json::Value;

/// Parse a Netch JSON configuration into a Proxy object
/// Matches C++ explodeNetch implementation
pub fn explode_netch(netch: &str, node: &mut Proxy) -> bool {
    // Handle the Netch protocol scheme and decode base64
    if !netch.starts_with("Netch://") {
        return false;
    }

    let decoded = url_safe_base64_decode(&netch[8..]);

    // Parse the JSON content
    let json: Value = match serde_json::from_str(&decoded) {
        Ok(j) => j,
        Err(_) => return false,
    };

    // Check if it's a valid JSON object
    if !json.is_object() {
        return false;
    }

    // Check if required fields exist
    let type_str = json.get("Type").and_then(Value::as_str).unwrap_or("");
    if type_str.is_empty() {
        return false;
    }

    let remark = json.get("Remark").and_then(Value::as_str).unwrap_or("");
    let server = json.get("Hostname").and_then(Value::as_str).unwrap_or("");
    let port_str = json.get("Port").and_then(Value::as_str).unwrap_or("0");

    if port_str == "0" {
        return false;
    }

    let port = port_str.parse::<u16>().unwrap_or(0);
    if port == 0 {
        return false;
    }

    // Extract optional common fields
    let group = json.get("Group").and_then(Value::as_str).unwrap_or("");
    let udp = json.get("EnableUDP").and_then(Value::as_bool);
    let tfo = json.get("EnableTFO").and_then(Value::as_bool);
    let scv = json.get("AllowInsecure").and_then(Value::as_bool);

    // Set default remark if empty
    let remark = if remark.is_empty() {
        format!("{}:{}", server, port)
    } else {
        remark.to_string()
    };

    // Process based on proxy type
    match type_str {
        "Shadowsocks" | "SS" => {
            let method = json
                .get("EncryptMethod")
                .and_then(Value::as_str)
                .or_else(|| json.get("Method").and_then(Value::as_str))
                .unwrap_or("");
            let password = json.get("Password").and_then(Value::as_str).unwrap_or("");

            if method.is_empty() || password.is_empty() {
                return false;
            }

            let plugin = json.get("Plugin").and_then(Value::as_str).unwrap_or("");
            let plugin_opts = json
                .get("PluginOption")
                .and_then(Value::as_str)
                .unwrap_or("");

            let group = if group.is_empty() {
                SS_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::ss_construct(
                group,
                &remark,
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
        "ShadowsocksR" | "SSR" => {
            let method = json
                .get("EncryptMethod")
                .and_then(Value::as_str)
                .or_else(|| json.get("Method").and_then(Value::as_str))
                .unwrap_or("");
            let password = json.get("Password").and_then(Value::as_str).unwrap_or("");
            let protocol = json.get("Protocol").and_then(Value::as_str).unwrap_or("");
            let obfs = json.get("OBFS").and_then(Value::as_str).unwrap_or("");
            let protocol_param = json
                .get("ProtocolParam")
                .and_then(Value::as_str)
                .unwrap_or("");
            let obfs_param = json.get("OBFSParam").and_then(Value::as_str).unwrap_or("");

            if method.is_empty() || password.is_empty() || protocol.is_empty() || obfs.is_empty() {
                return false;
            }

            let group = if group.is_empty() {
                SSR_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::ssr_construct(
                group,
                &remark,
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
        "SOCKS5" | "Socks5" => {
            let username = json.get("Username").and_then(Value::as_str).unwrap_or("");
            let password = json.get("Password").and_then(Value::as_str).unwrap_or("");

            let group = if group.is_empty() {
                SOCKS_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::socks_construct(
                group, &remark, server, port, username, password, udp, tfo, scv, "",
            );

            true
        }
        "HTTP" | "HTTPS" => {
            let username = json.get("Username").and_then(Value::as_str).unwrap_or("");
            let password = json.get("Password").and_then(Value::as_str).unwrap_or("");
            let is_https = type_str == "HTTPS";

            let group = if group.is_empty() {
                HTTP_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::http_construct(
                group, &remark, server, port, username, password, is_https, tfo, scv, None, "",
            );

            true
        }
        "Trojan" => {
            let password = json.get("Password").and_then(Value::as_str).unwrap_or("");
            if password.is_empty() {
                return false;
            }

            let sni = json
                .get("Host")
                .and_then(Value::as_str)
                .or_else(|| json.get("ServerName").and_then(Value::as_str))
                .map(|s| s.to_string());

            let group = if group.is_empty() {
                TROJAN_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::trojan_construct(
                group.to_string(),
                remark,
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
        "VMess" => {
            let uuid = json
                .get("UserID")
                .and_then(Value::as_str)
                .or_else(|| json.get("Id").and_then(Value::as_str))
                .unwrap_or("");
            if uuid.is_empty() {
                return false;
            }

            let alter_id = json
                .get("AlterID")
                .and_then(Value::as_u64)
                .or_else(|| json.get("AlterId").and_then(Value::as_u64))
                .unwrap_or(0) as u16;
            let network = json
                .get("TransferProtocol")
                .and_then(Value::as_str)
                .or_else(|| json.get("Network").and_then(Value::as_str))
                .unwrap_or("tcp");
            let security = json
                .get("EncryptMethod")
                .and_then(Value::as_str)
                .or_else(|| json.get("Security").and_then(Value::as_str))
                .unwrap_or("auto");
            let tls = json
                .get("TLSSecure")
                .and_then(Value::as_bool)
                .or_else(|| json.get("TLS").and_then(Value::as_bool))
                .unwrap_or(false);

            let host = json.get("Host").and_then(Value::as_str).unwrap_or("");
            let path = json.get("Path").and_then(Value::as_str).unwrap_or("/");
            let sni = json.get("ServerName").and_then(Value::as_str).unwrap_or("");
            let fake_type = json.get("FakeType").and_then(Value::as_str).unwrap_or("");
            let edge = json.get("Edge").and_then(Value::as_str).unwrap_or("");

            let group = if group.is_empty() {
                V2RAY_DEFAULT_GROUP
            } else {
                group
            };

            *node = Proxy::vmess_construct(
                group,
                &remark,
                server,
                port,
                fake_type,
                uuid,
                alter_id,
                network,
                security,
                path,
                host,
                edge,
                if tls { "tls" } else { "" },
                sni,
                udp,
                tfo,
                scv,
                None,
                "",
            );

            true
        }
        _ => false,
    }
}

/// Parse a Netch configuration file into a vector of Proxy objects
/// Matches C++ explodeNetchConf implementation
pub fn explode_netch_conf(content: &str, nodes: &mut Vec<Proxy>) -> bool {
    // Parse JSON
    let json: Value = match serde_json::from_str(content) {
        Ok(j) => j,
        Err(_) => return false,
    };

    // Check if it's a valid JSON object with Server field
    if !json.is_object() || !json.get("Server").is_some() {
        return false;
    }

    let servers = match json.get("Server") {
        Some(Value::Array(arr)) => arr,
        _ => return false,
    };

    let mut success = false;

    // Process each server
    for server in servers {
        let server_str = match serde_json::to_string(server) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Create Netch URL
        let netch_url = format!("Netch://{}", base64_encode(&server_str));

        let mut node = Proxy::default();
        if explode_netch(&netch_url, &mut node) {
            nodes.push(node);
            success = true;
        }
    }

    success
}
