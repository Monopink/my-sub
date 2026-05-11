/// Transforms a rule to a common format for use in different proxy clients
///
/// # Arguments
///
/// * `input` - The rule to transform
/// * `group` - The proxy group to assign
/// * `no_resolve_only` - Whether to only keep no-resolve parameter
///
/// # Returns
///
/// The transformed rule as a string
pub fn transform_rule_to_common(input: &str, group: &str, no_resolve_only: bool) -> String {
    let mut parts = ["", "", "", ""]; // Pre-allocate array with 4 elements like C++ version
    let mut part_count = 0;

    // Split the input by comma and fill the parts array
    for (i, part) in input.split(',').enumerate() {
        if i < 4 {
            parts[i] = part;
            part_count = i + 1;
        } else {
            break;
        }
    }

    if part_count < 2 {
        // Single part rule, just add group
        format!("{},{}", parts[0], group)
    } else {
        // Multi-part rule
        let mut result = format!("{},{},{}", parts[0], parts[1], group);

        // Add options like no-resolve if present and applicable
        if part_count > 2 && (!no_resolve_only || parts[2] == "no-resolve") {
            result = format!("{},{}", result, parts[2]);
        }

        result
    }
}
