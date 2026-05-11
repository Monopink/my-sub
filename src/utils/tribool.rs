//! Tribool utility traits for Option<T>
//!
//! This module provides utility traits to extend Option<T> with tribool-like functionality,
//! similar to the C++ tribool implementation in the original subconverter.

use serde_json::{Map, Value as JsonValue};

/// Trait for types that can be applied to a JSON object conditionally
pub trait JsonApplicable {
    /// Apply a value to a JSON object with the specified key
    /// only if it meets some condition (usually non-empty or defined)
    ///
    /// # Arguments
    ///
    /// * `json` - The JSON object to modify
    /// * `key` - The key to set in the JSON object
    ///
    /// # Returns
    ///
    /// true if the value was applied, false otherwise
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool;
}

// Implementation for strings - only apply if non-empty
impl JsonApplicable for String {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if !self.is_empty() {
            if let JsonValue::Object(map) = json {
                map.insert(key.to_string(), JsonValue::String(self.clone()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

// Implementation for &str - only apply if non-empty
impl JsonApplicable for &str {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if !self.is_empty() {
            if let JsonValue::Object(map) = json {
                map.insert(key.to_string(), JsonValue::String((*self).to_string()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

// Implementation for Option<String> - only apply if Some and non-empty
impl JsonApplicable for Option<String> {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if let Some(value) = self {
            value.apply_json(json, key)
        } else {
            false
        }
    }
}

// Implementation for Option<&str> - only apply if Some and non-empty
impl JsonApplicable for Option<&str> {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if let Some(value) = self {
            value.apply_json(json, key)
        } else {
            false
        }
    }
}

// Implementation for numbers - only apply if non-zero
impl JsonApplicable for u16 {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if *self > 0 {
            if let JsonValue::Object(map) = json {
                map.insert(key.to_string(), JsonValue::Number((*self).into()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl JsonApplicable for u32 {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if *self > 0 {
            if let JsonValue::Object(map) = json {
                map.insert(key.to_string(), JsonValue::Number((*self).into()));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Extension trait for Option<T> that adds tribool-like functionality
pub trait TriboolExt<T> {
    /// Define a value if the current value is None (undefined)
    ///
    /// Similar to the tribool.define() function in C++, this method will set
    /// the value to the provided default only if the current value is None.
    ///
    /// # Arguments
    ///
    /// * `default` - The value to use if self is None
    ///
    /// # Returns
    ///
    /// The existing value if Some, otherwise the default value
    fn define(&self, default: Option<T>) -> Option<T>
    where
        T: Clone;

    /// Apply this tribool value to a JSON object if it's defined (Some)
    ///
    /// # Arguments
    ///
    /// * `obj` - The JSON object to modify
    /// * `key` - The key to set in the JSON object
    ///
    /// # Returns
    ///
    /// true if the value was applied, false if it was undefined (None)
    fn apply_to_json(&self, obj: &mut Map<String, JsonValue>, key: &str) -> bool
    where
        T: Into<JsonValue> + Clone;

    /// Apply this tribool value to a JSON Value object if it's defined (Some)
    ///
    /// # Arguments
    ///
    /// * `obj` - The JSON Value object to modify (must be an object type)
    /// * `key` - The key to set in the JSON object
    ///
    /// # Returns
    ///
    /// true if the value was applied, false if it was undefined (None) or the target wasn't a JSON object
    fn apply_to_json_value(&self, obj: &mut JsonValue, key: &str) -> bool
    where
        T: Into<JsonValue> + Clone;
}
pub trait OptionSetExt<T> {
    fn set_if_some(&mut self, src: Option<T>);
}

impl<T> OptionSetExt<T> for Option<T> {
    fn set_if_some(&mut self, src: Option<T>) {
        if let Some(value) = src {
            *self = Some(value);
        }
    }
}

impl<T> TriboolExt<T> for Option<T> {
    fn define(&self, default: Option<T>) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Some(value) => Some(value.clone()),
            None => default,
        }
    }

    fn apply_to_json(&self, obj: &mut Map<String, JsonValue>, key: &str) -> bool
    where
        T: Into<JsonValue> + Clone,
    {
        match self {
            Some(value) => {
                let json_value = value.clone().into();
                obj.insert(key.to_string(), json_value);
                true
            }
            None => false,
        }
    }

    fn apply_to_json_value(&self, obj: &mut JsonValue, key: &str) -> bool
    where
        T: Into<JsonValue> + Clone,
    {
        if let JsonValue::Object(map) = obj {
            self.apply_to_json(map, key)
        } else {
            false
        }
    }
}

// Implement JsonApplicable for Option<bool> tribools
impl JsonApplicable for Option<bool> {
    fn apply_json(&self, json: &mut JsonValue, key: &str) -> bool {
        if let Some(value) = self {
            if let JsonValue::Object(map) = json {
                map.insert(key.to_string(), JsonValue::Bool(*value));
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// Extend Option<bool> with tribool-specific methods
pub trait BoolTriboolExt: TriboolExt<bool> {
    /// Get the boolean value with a default if undefined
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to use if undefined
    ///
    /// # Returns
    ///
    /// The boolean value or the default
    fn get_or(&self, default: bool) -> bool;

    /// Check if the tribool is undefined
    ///
    /// # Returns
    ///
    /// true if the value is None, false otherwise
    fn is_undef(&self) -> bool;

    /// Get the string representation of the tribool
    ///
    /// # Returns
    ///
    /// "true" if true, "false" if false, empty string if undefined
    fn get_str(&self) -> String;
}

impl BoolTriboolExt for Option<bool> {
    fn get_or(&self, default: bool) -> bool {
        self.unwrap_or(default)
    }

    fn is_undef(&self) -> bool {
        self.is_none()
    }

    fn get_str(&self) -> String {
        match self {
            Some(true) => "true".to_string(),
            Some(false) => "false".to_string(),
            None => String::new(),
        }
    }
}
