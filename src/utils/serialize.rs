
pub fn is_empty_option_string(s: &Option<String>) -> bool {
    s.is_none() || s.as_ref().unwrap().is_empty()
}

pub fn is_u32_option_zero(u: &Option<u32>) -> bool {
    if let Some(u) = u {
        *u == 0
    } else {
        true
    }
}
