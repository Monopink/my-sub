use crate::resources::{load_text, ResourceLoadOptions};
use crate::settings::Settings;
use crate::utils::http::parse_proxy;

// Import platform-specific implementations
#[cfg(not(target_arch = "wasm32"))]
mod platform {
    pub use crate::utils::file_std::{copy_file, file_exists, file_get_async, read_file_async};
}

#[cfg(target_arch = "wasm32")]
mod platform {
    pub use crate::utils::file_wasm::{copy_file, file_exists, file_get_async, read_file_async};
}

// Re-export platform-specific implementations
pub use platform::*;

// These functions are re-exported from platform-specific implementations

/// Async version of load_content
///
/// # Arguments
/// * `path` - Path to the file or URL to load
///
/// # Returns
/// * `Ok(String)` - The content
/// * `Err(String)` - Error message if loading failed
pub async fn load_content_async(path: &str) -> Result<String, String> {
    let proxy_config = {
        let settings = Settings::current();
        settings.proxy_config.clone()
    };
    let proxy = parse_proxy(&proxy_config);
    let options = ResourceLoadOptions {
        proxy: Some(&proxy),
        scope_base_path: None,
    };
    load_text(path, &options)
        .await
        .map_err(|e| format!("Failed to load content from {}: {}", path, e))
}
