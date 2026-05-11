
/// Match a range against a target integer value
///
/// This function checks if a target value is within a specified range.
/// The range can be defined in different formats like "1", "1-100", ">100", etc.
///
/// # Arguments
/// * `range` - Range specification string
/// * `target` - Target integer value to check
///
/// # Returns
/// `true` if target is within the specified range, `false` otherwise
pub fn match_range(range: &str, target: i32) -> bool {
    // Empty range matches everything
    if range.is_empty() {
        return true;
    }

    // Direct equality check
    if let Ok(value) = range.parse::<i32>() {
        return target == value;
    }

    // Range with dash: "1-100"
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            let start = parts[0].parse::<i32>().unwrap_or(i32::MIN);
            let end = parts[1].parse::<i32>().unwrap_or(i32::MAX);
            return target >= start && target <= end;
        }
    }

    // Greater than: ">100"
    if range.starts_with('>') {
        if let Ok(value) = range[1..].parse::<i32>() {
            return target > value;
        }
    }

    // Greater than or equal: ">=100"
    if range.starts_with(">=") {
        if let Ok(value) = range[2..].parse::<i32>() {
            return target >= value;
        }
    }

    // Less than: "<100"
    if range.starts_with('<') {
        if let Ok(value) = range[1..].parse::<i32>() {
            return target < value;
        }
    }

    // Less than or equal: "<=100"
    if range.starts_with("<=") {
        if let Ok(value) = range[2..].parse::<i32>() {
            return target <= value;
        }
    }

    false
}
