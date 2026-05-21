use crate::resources::{load_text, ResourceLoadOptions};
use crate::utils::http::ProxyConfig;

use super::toml_deserializer::ImportableInToml;

/// Import items from external files or URLs
///
/// This function processes configuration items that start with "!!import:"
/// and replaces them with the content from the specified file or URL.
pub async fn import_toml_items<T: ImportableInToml>(
    target: &mut Vec<T>,
    scope_limit: bool,
    import_key: &str,
    proxy_config: &ProxyConfig,
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    let mut item_count = 0;

    for item in target.iter() {
        if !item.is_import_node() {
            result.push(item.clone());
            continue;
        }

        let path = item.get_import_path().unwrap();
        log::info!("Trying to import items from {}", path);

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

        let toml_root_node = toml::from_str::<toml::Value>(&content)?;
        if let Some(sub_nodes) = toml_root_node.get(import_key) {
            if let Some(array) = sub_nodes.as_array() {
                for sub_node in array {
                    result.push(T::try_from_toml_value(sub_node)?);
                    item_count += 1;
                }
            } else {
                return Err(
                    format!("Import key {} is not an array in {}", import_key, path).into(),
                );
            }
        } else {
            return Err(format!("Import key {} not found in {}", import_key, path).into());
        }
    }

    *target = result;
    log::info!("Imported {} item(s).", item_count);

    Ok(())
}
