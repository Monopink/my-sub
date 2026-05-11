//! Peer generation utilities
//!
//! This module provides functionality for generating peer configurations.


use crate::models::Proxy;

/// Generates a peer configuration string for a proxy node
///
/// # Arguments
///
/// * `node` - The proxy node to generate a peer config for
/// * `client_id_as_reserved` - Whether to use the client ID as the reserved field
///
/// # Returns
///
/// A string containing the peer configuration
pub fn generate_peer(node: &Proxy, client_id_as_reserved: bool) -> String {
    let mut result = String::new();

    // Add public key
    if let Some(public_key) = &node.public_key {
        result.push_str(&format!("public-key = {}", public_key));
    }

    // Add endpoint
    result.push_str(&format!(", endpoint = {}:{}", node.hostname, node.port));

    // Add allowed IPs if not empty
    if !node.allowed_ips.is_empty() {
        result.push_str(&format!(", allowed-ips = \"{}\"", node.allowed_ips));
    }

    // Add client ID if present
    if let Some(client_id) = &node.client_id {
        if !client_id.is_empty() {
            if client_id_as_reserved {
                result.push_str(&format!(", reserved = [{}]", client_id));
            } else {
                result.push_str(&format!(", client-id = {}", client_id));
            }
        }
    }

    result
}
