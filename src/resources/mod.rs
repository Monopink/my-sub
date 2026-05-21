use include_dir::{include_dir, Dir};
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

use crate::utils::http::{web_get_async, ProxyConfig};

const EMBEDDED_SCHEME: &str = "embedded://";
const EMBEDDED_ROOT: &str = "base";

static EMBEDDED_BASE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/base");

#[derive(Debug, Clone)]
pub enum ResourceLocation {
    Embedded(String),
    Url(String),
    LocalPath(String),
}

#[derive(Debug, Clone, Default)]
pub struct ResourceLoadOptions<'a> {
    pub proxy: Option<&'a ProxyConfig>,
    pub scope_base_path: Option<&'a str>,
}

#[derive(thiserror::Error, Debug)]
pub enum ResourceError {
    #[error("resource reference is empty")]
    EmptyReference,
    #[error("invalid embedded path: {0}")]
    InvalidEmbeddedPath(String),
    #[error("embedded resource not found: {0}")]
    EmbeddedNotFound(String),
    #[error("embedded resource is not valid UTF-8: {0}")]
    EmbeddedNotUtf8(String),
    #[error("file path is out of allowed scope: {path} not under {scope}")]
    OutOfScope { path: String, scope: String },
    #[error("failed to load local file {path}: {reason}")]
    FileIo { path: String, reason: String },
    #[error("failed to load URL {url}: {reason}")]
    HttpRequest { url: String, reason: String },
    #[error("HTTP status {status} when loading URL {url}")]
    HttpStatus { url: String, status: u16 },
}

pub fn parse_location(reference: &str) -> Result<ResourceLocation, ResourceError> {
    let reference = reference.trim();
    if reference.is_empty() {
        return Err(ResourceError::EmptyReference);
    }

    if let Some(raw_path) = reference.strip_prefix(EMBEDDED_SCHEME) {
        let normalized = normalize_slash_path(raw_path)?;
        if normalized == EMBEDDED_ROOT {
            return Err(ResourceError::InvalidEmbeddedPath(
                "embedded://base must point to a file".to_string(),
            ));
        }
        if !normalized.starts_with("base/") {
            return Err(ResourceError::InvalidEmbeddedPath(format!(
                "embedded path must start with 'base/': {}",
                raw_path
            )));
        }
        let rel = normalized[5..].to_string();
        if rel.is_empty() {
            return Err(ResourceError::InvalidEmbeddedPath(raw_path.to_string()));
        }
        return Ok(ResourceLocation::Embedded(rel));
    }

    if reference.starts_with("http://") || reference.starts_with("https://") {
        return Ok(ResourceLocation::Url(reference.to_string()));
    }

    Ok(ResourceLocation::LocalPath(reference.to_string()))
}

pub async fn load_text(
    reference: &str,
    options: &ResourceLoadOptions<'_>,
) -> Result<String, ResourceError> {
    match parse_location(reference)? {
        ResourceLocation::Embedded(rel_path) => {
            let file = EMBEDDED_BASE
                .get_file(&rel_path)
                .ok_or_else(|| ResourceError::EmbeddedNotFound(reference.to_string()))?;
            file.contents_utf8()
                .map(ToOwned::to_owned)
                .ok_or_else(|| ResourceError::EmbeddedNotUtf8(reference.to_string()))
        }
        ResourceLocation::Url(url) => {
            let default_proxy = ProxyConfig::default();
            let proxy = options.proxy.unwrap_or(&default_proxy);
            match web_get_async(&url, proxy, None).await {
                Ok(response) if (200..300).contains(&response.status) => Ok(response.body),
                Ok(response) => Err(ResourceError::HttpStatus {
                    url,
                    status: response.status,
                }),
                Err(err) => Err(ResourceError::HttpRequest {
                    url,
                    reason: err.message,
                }),
            }
        }
        ResourceLocation::LocalPath(path) => {
            load_local_text(&path, options.scope_base_path).await
        }
    }
}

pub async fn resource_exists(reference: &str, scope_base_path: Option<&str>) -> bool {
    match parse_location(reference) {
        Ok(ResourceLocation::Embedded(rel_path)) => EMBEDDED_BASE.get_file(&rel_path).is_some(),
        Ok(ResourceLocation::Url(_)) => true,
        Ok(ResourceLocation::LocalPath(path)) => {
            local_resource_exists(&path, scope_base_path).await
        }
        Err(_) => false,
    }
}

pub fn is_url_reference(reference: &str) -> bool {
    matches!(parse_location(reference), Ok(ResourceLocation::Url(_)))
}

fn normalize_slash_path(raw_path: &str) -> Result<String, ResourceError> {
    let raw_path = raw_path.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();

    for segment in raw_path.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            if parts.pop().is_none() {
                return Err(ResourceError::InvalidEmbeddedPath(raw_path));
            }
            continue;
        }
        parts.push(segment);
    }

    if parts.is_empty() {
        return Err(ResourceError::InvalidEmbeddedPath(raw_path));
    }

    Ok(parts.join("/"))
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_local_text(path: &str, scope_base_path: Option<&str>) -> Result<String, ResourceError> {
    let file_path = validate_local_path(path, scope_base_path).await?;
    tokio::fs::read_to_string(&file_path)
        .await
        .map_err(|e| ResourceError::FileIo {
            path: path.to_string(),
            reason: e.to_string(),
        })
}

#[cfg(target_arch = "wasm32")]
async fn load_local_text(path: &str, _scope_base_path: Option<&str>) -> Result<String, ResourceError> {
    Err(ResourceError::FileIo {
        path: path.to_string(),
        reason: "local filesystem is not supported in wasm runtime".to_string(),
    })
}

#[cfg(not(target_arch = "wasm32"))]
async fn local_resource_exists(path: &str, scope_base_path: Option<&str>) -> bool {
    match validate_local_path(path, scope_base_path).await {
        Ok(file_path) => tokio::fs::metadata(file_path).await.is_ok(),
        Err(_) => false,
    }
}

#[cfg(target_arch = "wasm32")]
async fn local_resource_exists(_path: &str, _scope_base_path: Option<&str>) -> bool {
    false
}

#[cfg(not(target_arch = "wasm32"))]
async fn validate_local_path(
    path: &str,
    scope_base_path: Option<&str>,
) -> Result<PathBuf, ResourceError> {
    let file_path = tokio::fs::canonicalize(path)
        .await
        .map_err(|e| ResourceError::FileIo {
            path: path.to_string(),
            reason: e.to_string(),
        })?;

    if let Some(scope_base_path) = scope_base_path {
        let scope = tokio::fs::canonicalize(scope_base_path)
            .await
            .map_err(|e| ResourceError::FileIo {
                path: scope_base_path.to_string(),
                reason: e.to_string(),
            })?;

        if !is_path_under_scope(&file_path, &scope) {
            return Err(ResourceError::OutOfScope {
                path: path.to_string(),
                scope: scope_base_path.to_string(),
            });
        }
    }

    Ok(file_path)
}

#[cfg(not(target_arch = "wasm32"))]
fn is_path_under_scope(path: &Path, scope: &Path) -> bool {
    path.starts_with(scope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn loads_embedded_clash_base_with_dns_section() {
        let options = ResourceLoadOptions::default();
        let content = load_text("embedded://base/base/clash.yaml", &options)
            .await
            .expect("embedded clash base should be readable");
        assert!(content.contains("\ndns:") || content.starts_with("dns:"));
    }
}
