//! VMess link construction utilities
//!
//! This module provides functionality for building VMess links.

use serde_json::json;

use crate::utils::base64::base64_encode;

/// Constructs a VMess link from the provided parameters
///
/// # Arguments
///
/// * `remarks` - Remarks (name) for the VMess link
/// * `add` - Server address
/// * `port` - Server port
/// * `type_str` - Connection type (e.g., "tcp", "ws")
/// * `id` - UUID
/// * `aid` - AlterID
/// * `net` - Network protocol
/// * `path` - WebSocket path or other path-like parameter
/// * `host` - Host header
/// * `tls` - TLS setting
///
/// # Returns
///
/// A VMess URI string
pub fn vmess_link_construct(
    remarks: &str,
    add: &str,
    port: &str,
    type_str: &str,
    id: &str,
    aid: &str,
    net: &str,
    path: &str,
    host: &str,
    tls: &str,
) -> String {
    // Create the JSON object
    let json_obj = json!({
        "v": "2",
        "ps": remarks,
        "add": add,
        "port": port,
        "id": id,
        "aid": aid,
        "net": net,
        "type": type_str,
        "host": host,
        "path": path,
        "tls": tls
    });

    // Convert to string
    let json_string = serde_json::to_string(&json_obj).unwrap_or_default();

    // Encode to Base64
    let encoded = base64_encode(&json_string);

    // Return vmess:// URL
    format!("vmess://{}", encoded)
}
