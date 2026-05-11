//! URL encoding/decoding utilities


/// Encodes a string using URL encoding
///
/// # Arguments
/// * `input` - The string to encode
///
/// # Returns
/// * String containing the URL-encoded input
///
/// # Examples
/// ```
/// use subconverter_rs::utils::url::url_encode;
///
/// let encoded = url_encode("Hello World!");
/// assert_eq!(encoded, "Hello%20World%21");
/// ```
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).into_owned()
}

/// Decodes a URL-encoded string
///
/// # Arguments
/// * `input` - The URL-encoded string to decode
///
/// # Returns
/// * String containing the decoded input
/// * Returns the original string if decoding fails
///
/// # Examples
/// ```
/// use subconverter_rs::utils::url::url_decode;
///
/// let decoded = url_decode("Hello%20World%21");
/// assert_eq!(decoded, "Hello World!");
/// ```
pub fn url_decode(input: &str) -> String {
    urlencoding::decode(input)
        .map(|cow| cow.into_owned())
        .unwrap_or_else(|_| input.to_string())
}

/// Extracts a parameter value from a URL query string
///
/// # Arguments
/// * `url_params` - The URL query string containing parameters
/// * `param_name` - The name of the parameter to extract
///
/// # Returns
/// * String containing the parameter value if found, or an empty string if not found
///
/// # Examples
/// ```
/// use subconverter_rs::utils::url::get_url_arg;
///
/// let query = "host=example.com&port=443&mode=ws";
/// assert_eq!(get_url_arg(query, "host"), "example.com");
/// assert_eq!(get_url_arg(query, "port"), "443");
/// assert_eq!(get_url_arg(query, "unknown"), "");
/// ```
pub fn get_url_arg(url_params: &str, param_name: &str) -> String {
    let pattern = format!("{}=", param_name);
    let mut pos = url_params.len();

    while pos > 0 {
        // Find the pattern starting from pos, moving backward
        if let Some(found_pos) = url_params[..pos].rfind(&pattern) {
            // Check if this is a proper parameter boundary
            if found_pos == 0
                || url_params.as_bytes()[found_pos - 1] == b'&'
                || url_params.as_bytes()[found_pos - 1] == b'?'
            {
                // Extract the value
                let start = found_pos + pattern.len();
                let end = match url_params[start..].find('&') {
                    Some(ampersand_pos) => start + ampersand_pos,
                    None => url_params.len(),
                };
                return url_params[start..end].to_string();
            }
            // Move position backward to continue searching
            pos = found_pos;
        } else {
            // Pattern not found
            break;
        }

        // Prevent unsigned integer underflow
        if pos == 0 {
            break;
        }
        pos -= 1;
    }

    // Parameter not found
    String::new()
}
