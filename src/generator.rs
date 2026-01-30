use crate::cli::UuidArgs;
use anyhow::{Context, Result, anyhow};
use uuid::Uuid;

/// Generate a UUID based on the provided arguments
pub fn generate_uuid(args: &UuidArgs) -> Result<Uuid> {
    let version = args.version.as_deref().unwrap_or("v7");

    match version {
        "v1" => generate_v1(args),
        "v3" => generate_v3(args),
        "v4" => Ok(Uuid::new_v4()),
        "v5" => generate_v5(args),
        "v6" => generate_v6(args),
        "v7" => Ok(Uuid::now_v7()),
        "v8" => generate_v8(args),
        _ => Err(anyhow!("Unsupported UUID version: {}", version)),
    }
}

/// Generate UUID v1 (time-based)
fn generate_v1(args: &UuidArgs) -> Result<Uuid> {
    let node_id = get_node_id(args)?;
    Ok(Uuid::now_v1(&node_id))
}

/// Generate UUID v6 (sortable time-based)
fn generate_v6(args: &UuidArgs) -> Result<Uuid> {
    let node_id = get_node_id(args)?;
    Ok(Uuid::now_v6(&node_id))
}

/// Get node ID for v1/v6, either from --node-id or generate random
fn get_node_id(args: &UuidArgs) -> Result<[u8; 6]> {
    if let Some(node_id_hex) = &args.node_id {
        // Parse hex string to 6-byte array
        if node_id_hex.len() != 12 {
            return Err(anyhow!(
                "Node ID must be exactly 12 hex characters, got {}",
                node_id_hex.len()
            ));
        }

        let mut node_id = [0u8; 6];
        for (i, chunk) in node_id_hex.as_bytes().chunks(2).enumerate() {
            let hex_str = std::str::from_utf8(chunk).context("Invalid UTF-8 in node ID")?;
            node_id[i] = u8::from_str_radix(hex_str, 16).context("Invalid hex in node ID")?;
        }
        Ok(node_id)
    } else {
        // Generate random node ID with multicast bit set (RFC 9562)
        let mut node_id = [0u8; 6];
        node_id.copy_from_slice(&Uuid::new_v4().as_bytes()[..6]);
        node_id[0] |= 0x01; // Set multicast bit
        Ok(node_id)
    }
}

/// Generate UUID v3 (MD5 hash-based)
fn generate_v3(args: &UuidArgs) -> Result<Uuid> {
    let (namespace, name) = get_namespace_and_name(args)?;
    Ok(Uuid::new_v3(&namespace, name.as_bytes()))
}

/// Generate UUID v5 (SHA-1 hash-based)
fn generate_v5(args: &UuidArgs) -> Result<Uuid> {
    let (namespace, name) = get_namespace_and_name(args)?;
    Ok(Uuid::new_v5(&namespace, name.as_bytes()))
}

/// Get namespace and name for v3/v5
fn get_namespace_and_name(args: &UuidArgs) -> Result<(Uuid, String)> {
    let name = args
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("--name is required for v3/v5"))?
        .clone();

    let namespace_str = args
        .namespace
        .as_ref()
        .ok_or_else(|| anyhow!("--namespace is required for v3/v5"))?;

    let namespace = match namespace_str.as_str() {
        "dns" => Uuid::NAMESPACE_DNS,
        "url" => Uuid::NAMESPACE_URL,
        "oid" => Uuid::NAMESPACE_OID,
        "x500" => Uuid::NAMESPACE_X500,
        custom => Uuid::parse_str(custom).context("Invalid namespace UUID")?,
    };

    Ok((namespace, name))
}

/// Generate UUID v8 (custom)
fn generate_v8(args: &UuidArgs) -> Result<Uuid> {
    let bytes_hex = args
        .bytes
        .as_ref()
        .ok_or_else(|| anyhow!("--bytes is required for v8"))?;

    if bytes_hex.len() != 32 {
        return Err(anyhow!(
            "Bytes must be exactly 32 hex characters, got {}",
            bytes_hex.len()
        ));
    }

    let mut bytes = [0u8; 16];
    for (i, chunk) in bytes_hex.as_bytes().chunks(2).enumerate() {
        let hex_str = std::str::from_utf8(chunk).context("Invalid UTF-8 in bytes")?;
        bytes[i] = u8::from_str_radix(hex_str, 16).context("Invalid hex in bytes")?;
    }

    Ok(Uuid::new_v8(bytes))
}
