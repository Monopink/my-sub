use crate::utils::system::safe_system_time;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Global in-memory cache for storing ruleset content and other frequently accessed data
static MEMORY_CACHE: Lazy<Arc<Mutex<MemoryCache>>> =
    Lazy::new(|| Arc::new(Mutex::new(MemoryCache::new())));

/// Structure to hold cached content along with metadata
#[derive(Clone)]
struct CachedItem {
    /// The content stored in the cache
    content: String,
    /// When this item was stored
    timestamp: SystemTime,
}

/// Memory cache manager
struct MemoryCache {
    /// Map of cache keys to cached content
    cache: HashMap<String, CachedItem>,
}

impl MemoryCache {
    /// Create a new empty memory cache
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

/// Store content in the in-memory cache
///
/// # Arguments
/// * `key` - Cache key (usually a path or URL)
/// * `content` - Content to store
///
/// # Returns
/// * `Ok(())` - Content was successfully stored
/// * `Err(String)` - Error storing content
pub fn store(key: &str, content: &str) -> Result<(), String> {
    let mut cache = match MEMORY_CACHE.lock() {
        Ok(cache) => cache,
        Err(e) => return Err(format!("Failed to lock memory cache: {}", e)),
    };

    // Store the content with current timestamp
    cache.cache.insert(
        key.to_string(),
        CachedItem {
            content: content.to_string(),
            timestamp: safe_system_time(),
        },
    );

    Ok(())
}

/// Retrieve content from the in-memory cache
///
/// # Arguments
/// * `key` - Cache key (usually a path or URL)
///
/// # Returns
/// * `Some(String)` - Content was found
/// * `None` - Content was not found
pub fn get(key: &str) -> Option<String> {
    let cache = match MEMORY_CACHE.lock() {
        Ok(cache) => cache,
        Err(_) => return None,
    };

    cache.cache.get(key).map(|item| item.content.clone())
}

/// Check if a key exists in the in-memory cache
///
/// # Arguments
/// * `key` - Cache key to check
///
/// # Returns
/// * `true` - Key exists in cache
/// * `false` - Key does not exist in cache
pub fn exists(key: &str) -> bool {
    let cache = match MEMORY_CACHE.lock() {
        Ok(cache) => cache,
        Err(_) => return false,
    };

    cache.cache.contains_key(key)
}

/// Check if a key exists in the in-memory cache and is not expired
///
/// # Arguments
/// * `key` - Cache key to check
/// * `max_age` - Maximum age in seconds
///
/// # Returns
/// * `true` - Key exists and is not expired
/// * `false` - Key does not exist or is expired
pub fn is_valid(key: &str, max_age: u32) -> bool {
    let cache = match MEMORY_CACHE.lock() {
        Ok(cache) => cache,
        Err(_) => return false,
    };

    if let Some(item) = cache.cache.get(key) {
        let now = safe_system_time();
        if let Ok(elapsed) = now.duration_since(item.timestamp) {
            return elapsed.as_secs() < u64::from(max_age);
        }
    }

    false
}

/// Retrieve content from the in-memory cache if it exists and is not expired
///
/// # Arguments
/// * `key` - Cache key (usually a path or URL)
/// * `max_age` - Maximum age in seconds
///
/// # Returns
/// * `Some(String)` - Content was found and is not expired
/// * `None` - Content was not found or is expired
pub fn get_if_valid(key: &str, max_age: u32) -> Option<String> {
    let cache = match MEMORY_CACHE.lock() {
        Ok(cache) => cache,
        Err(_) => return None,
    };

    if let Some(item) = cache.cache.get(key) {
        let now = safe_system_time();
        if let Ok(elapsed) = now.duration_since(item.timestamp) {
            if elapsed.as_secs() < u64::from(max_age) {
                return Some(item.content.clone());
            }
        }
    }

    None
}

/// Remove an item from the cache
///
/// # Arguments
/// * `key` - Cache key to remove
pub fn remove(key: &str) {
    if let Ok(mut cache) = MEMORY_CACHE.lock() {
        cache.cache.remove(key);
    }
}

/// Clear all items from the cache
pub fn clear() {
    if let Ok(mut cache) = MEMORY_CACHE.lock() {
        cache.cache.clear();
    }
}

/// Get the number of items in the cache
pub fn size() -> usize {
    if let Ok(cache) = MEMORY_CACHE.lock() {
        return cache.cache.len();
    }
    0
}

/// Clear expired items from the cache
///
/// # Arguments
/// * `max_age` - Maximum age in seconds
pub fn clean_expired(max_age: u32) {
    if let Ok(mut cache) = MEMORY_CACHE.lock() {
        let now = safe_system_time();
        let max_duration = Duration::from_secs(u64::from(max_age));

        // Use retain to keep only non-expired items
        cache.cache.retain(|_, item| {
            if let Ok(elapsed) = now.duration_since(item.timestamp) {
                elapsed <= max_duration
            } else {
                // If we can't calculate duration (clock went backwards), keep the item
                true
            }
        });
    }
}
