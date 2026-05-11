//! System utilities for cross-platform functionality

use std::env;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[cfg(target_os = "windows")]
use winapi::shared::winerror::IS_ERROR;
#[cfg(target_os = "windows")]
use winapi::{
    shared::minwindef::{BYTE, DWORD, HKEY},
    um::{
        winnt::KEY_ALL_ACCESS,
        winreg::{RegEnumValueW, RegOpenKeyExW, RegQueryInfoKeyW, HKEY_CURRENT_USER},
    },
};

/// Get the current system time safely, even in WebAssembly environments
///
/// This function handles the limitations of WebAssembly regarding system time access
/// and provides a fallback mechanism to prevent panics.
///
/// # Returns
///
/// The current system time or a default time if it can't be determined
pub fn safe_system_time() -> SystemTime {
    #[cfg(target_arch = "wasm32")]
    {
        // In WebAssembly, use the JavaScript Date API
        let now_ms = Date::now();
        let seconds = (now_ms / 1000.0) as u64;
        let nanos = ((now_ms % 1000.0) * 1_000_000.0) as u32;

        // Create a SystemTime from the UNIX_EPOCH plus the calculated duration
        UNIX_EPOCH
            .checked_add(Duration::new(seconds, nanos))
            .unwrap_or(UNIX_EPOCH) // Fallback to UNIX_EPOCH if addition overflows
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // In native environments, use the standard SystemTime::now()
        SystemTime::now()
    }
}

/// Get the current timestamp in seconds since UNIX epoch
///
/// This function safely gets the current time and converts it to seconds
/// since the UNIX epoch, handling potential errors gracefully.
///
/// # Returns
///
/// The number of seconds since the UNIX epoch, or 0 if it can't be determined
pub fn safe_unix_timestamp() -> u64 {
    safe_system_time()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Sleep for a specified number of milliseconds
///
/// # Arguments
///
/// * `interval` - The number of milliseconds to sleep
pub fn sleep_ms(interval: u64) {
    thread::sleep(Duration::from_millis(interval));
}

/// Get environment variable value
///
/// # Arguments
///
/// * `name` - The name of the environment variable
///
/// # Returns
///
/// The value of the environment variable or empty string if not found
pub fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_default()
}

/// Get system proxy settings
///
/// # Returns
///
/// The system proxy server string or empty string if not found
pub fn get_system_proxy() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // In WASM environment, we can't access system proxy settings
        // Return empty string to indicate no proxy
        return String::new();
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        use std::ptr;

        unsafe {
            // Open registry key
            let subkey = "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";
            let mut hkey: HKEY = ptr::null_mut();
            let wide_subkey: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();

            let ret = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                wide_subkey.as_ptr(),
                0,
                KEY_ALL_ACCESS,
                &mut hkey,
            );

            if IS_ERROR(ret) {
                return String::new();
            }

            // Query key info
            let mut values_count: DWORD = 0;
            let mut max_value_name_len: DWORD = 0;
            let mut max_value_len: DWORD = 0;

            let ret = RegQueryInfoKeyW(
                hkey,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut values_count,
                &mut max_value_name_len,
                &mut max_value_len,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if IS_ERROR(ret) {
                return String::new();
            }

            // max_value_name_len does not include null terminator
            max_value_name_len += 1;

            // Extract registry values
            let mut proxy_enable: DWORD = 0;
            let mut proxy_server = String::new();

            for i in 0..values_count {
                let mut name_buf = vec![0u16; max_value_name_len as usize];
                let mut name_len = max_value_name_len;
                let mut value_type: DWORD = 0;
                let mut value_len: DWORD = 0;

                // First call to get the size
                let ret = RegEnumValueW(
                    hkey,
                    i,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    ptr::null_mut(),
                    &mut value_type,
                    ptr::null_mut(),
                    &mut value_len,
                );

                if IS_ERROR(ret) {
                    continue;
                }

                let mut value_buf = vec![0u8; value_len as usize];
                name_len = max_value_name_len;

                // Second call to get the value
                let ret = RegEnumValueW(
                    hkey,
                    i,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    ptr::null_mut(),
                    &mut value_type,
                    value_buf.as_mut_ptr() as *mut BYTE,
                    &mut value_len,
                );

                if IS_ERROR(ret) {
                    continue;
                }

                // Convert name to string
                let name_len = name_len as usize;
                let name_os_string = OsString::from_wide(&name_buf[0..name_len]);
                let name_string = name_os_string.to_string_lossy().to_string();

                // Check if this is the ProxyEnable or ProxyServer entry
                if name_string == "ProxyEnable" && value_len >= 4 {
                    proxy_enable = u32::from_ne_bytes([
                        value_buf[0],
                        value_buf[1],
                        value_buf[2],
                        value_buf[3],
                    ]);
                } else if name_string == "ProxyServer" {
                    // For REG_SZ strings, convert the buffer to a string
                    if value_len > 0 {
                        // Ensure the buffer is valid UTF-8
                        if let Ok(s) =
                            String::from_utf8(value_buf[0..(value_len as usize)].to_vec())
                        {
                            proxy_server = s.trim_end_matches('\0').to_string();
                        }
                    }
                }
            }

            if proxy_enable != 0 && !proxy_server.is_empty() {
                return proxy_server;
            }

            String::new()
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(target_os = "windows"))]
    {
        let proxy_env = [
            "all_proxy",
            "ALL_PROXY",
            "http_proxy",
            "HTTP_PROXY",
            "https_proxy",
            "HTTPS_PROXY",
        ];

        for var in &proxy_env {
            if let Ok(proxy) = env::var(var) {
                if !proxy.is_empty() {
                    return proxy;
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_ms() {
        // This is more of a smoke test
        sleep_ms(1);
    }

    #[test]
    fn test_get_env() {
        // Test against a common environment variable
        let path = get_env("PATH");
        assert!(!path.is_empty());
    }
}
