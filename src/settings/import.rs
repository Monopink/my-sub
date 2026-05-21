use crate::resources::{load_text, ResourceLoadOptions};
use crate::utils::http::ProxyConfig;

/// Import items from external files or URLs
///
/// This function processes configuration items that start with "!!import:"
/// and replaces them with the content from the specified file or URL.
pub async fn import_items(
    target: &mut Vec<String>,
    scope_limit: bool,
    proxy_config: &ProxyConfig,
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let mut item_count = 0;

    for item in target.iter() {
        if !item.starts_with("!!import:") {
            result.push(item.clone());
            continue;
        }

        let path = item[9..].to_string(); // Extract path after "!!import:"
        log::info!("Trying to import items from {}", path);

        // Function to determine content line breaks
        let get_line_break = |content: &str| -> char {
            if content.contains("\r\n") {
                '\n' // Windows style but we normalize to '\n'
            } else if content.contains('\r') {
                '\r' // Old Mac style
            } else {
                '\n' // Unix style
            }
        };

        let options = ResourceLoadOptions {
            proxy: Some(proxy_config),
            scope_base_path: if scope_limit { Some(base_path) } else { None },
        };
        let content = match load_text(&path, &options).await {
            Ok(content) => content,
            Err(e) => {
                log::error!("Failed to import from {}: {}", path, e);
                return Err(Box::new(e));
            }
        };

        if content.is_empty() {
            return Err("Empty content from import source".into());
        }

        // Process content line by line
        let delimiter = get_line_break(&content);
        for line in content.split(delimiter) {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty()
                || line.starts_with(';')
                || line.starts_with('#')
                || (line.len() >= 2 && line.starts_with("//"))
            {
                continue;
            }

            result.push(line.to_string());
            item_count += 1;
        }
    }

    *target = result;
    log::info!("Imported {} item(s).", item_count);

    Ok(())
}
