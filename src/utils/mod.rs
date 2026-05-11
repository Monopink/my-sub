pub mod base64;
pub mod deserialize;
pub mod file;
#[cfg(not(target_arch = "wasm32"))]
pub mod file_std;
#[cfg(target_arch = "wasm32")]
pub mod file_wasm;
pub mod http;
#[cfg(not(target_arch = "wasm32"))]
pub mod http_std;
#[cfg(target_arch = "wasm32")]
pub mod http_wasm;
pub mod ini_reader;
pub mod matcher;
pub mod memory_cache;
pub mod network;
pub mod node_manip;
pub mod regexp;
pub mod serialize;
pub mod string;
pub mod system;
pub mod tribool;
pub mod url;
pub mod useragent;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export common utilities
pub use file::{file_exists, file_get_async};
pub use http::{get_sub_info_from_header, web_get_async};
pub use ini_reader::IniReader;
pub use network::{is_ipv4, is_ipv6, is_link};
pub use node_manip::{append_type_to_remark, preprocess_nodes};
pub use regexp::{
    reg_find, reg_get_all_match, reg_get_match, reg_match, reg_replace, reg_trim, reg_valid,
};
pub use serialize::{is_empty_option_string, is_u32_option_zero};
pub use string::{
    ends_with, md5, remove_emoji, replace_all_distinct, starts_with, to_lower, trim,
    trim_whitespace,
};
pub use system::{get_env, get_system_proxy, sleep_ms};
pub use url::{url_decode, url_encode};
pub use useragent::{match_user_agent, ver_greater_equal};
#[cfg(target_arch = "wasm32")]
pub use wasm::{init_panic_hook, set_panic_hook};
