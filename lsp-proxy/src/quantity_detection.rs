//! Shared Quantity type detection logic for LSP proxy

/// Fast string search to detect Quantity types without deserialization
/// This performs a performant string search for "Quantity<" patterns
pub fn contains_quantity_types_fast(json_payload: &str) -> bool {
    // Check for the basic Quantity pattern first
    if !json_payload.contains("Quantity") {
        return false;
    }

    // For initial inlay hints (full type in one chunk), validate the Quantity< pattern
    if json_payload.contains("Quantity<") {
        return validate_quantity_format(json_payload);
    }

    // For mouseover/resolved hints (deconstructed type), look for separate Scale and Dimension
    if json_payload.contains("Scale") && json_payload.contains("Dimension") {
        return true;
    }

    false
}

/// Validate that Quantity<...> has the expected format with proper structuref
pub fn validate_quantity_format(json_payload: &str) -> bool {
    // Use a simple state machine to find and validate Quantity<...> patterns
    let mut chars = json_payload.char_indices().peekable();

    while let Some((pos, ch)) = chars.next() {
        if ch == 'Q' {
            // Check if this could be the start of "Quantity<"
            let remaining = &json_payload[pos..];
            if remaining.starts_with("Quantity<") {
                // Found a potential Quantity type, validate the format
                if let Some(close_pos) = find_matching_angle_bracket(&json_payload[pos + 9..]) {
                    let inner_content = &json_payload[pos + 9..pos + 9 + close_pos];
                    // Check for the new format with Scale and Dimension structs
                    // Handle both full format (Scale<...>, Dimension<...>) and truncated format (Scale, Dimension<...>)
                    // Also handle fully defaulted Dimension (Scale, Dimension, T) and Dimension> (no brackets)
                    // Note: Dimension> might appear as ", Dimension>" or " Dimension>" with whitespace
                    let has_scale = inner_content.contains("Scale<")
                        || inner_content.contains("Scale,")
                        || inner_content.contains("Scale");
                    let has_dimension = inner_content.contains("Dimension<")
                        || inner_content.contains("Dimension,")
                        || inner_content.contains("Dimension>")
                        || inner_content.contains("Dimension"); // Also check for just "Dimension" in case of whitespace

                    if has_scale && has_dimension {
                        return true; // Found a valid Quantity type
                    }
                }
            }
        }
    }

    false
}

/// Find the matching closing angle bracket for a Quantity type
pub fn find_matching_angle_bracket(text: &str) -> Option<usize> {
    let mut depth = 1; // Start at depth 1 since we're already inside the first <
    for (i, ch) in text.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// Validate split Quantity format where "Quantity" and "<...>" are separate JSON values
pub fn validate_split_quantity_format(json_payload: &str) -> bool {
    // Simple approach: look for the pattern "Quantity" followed by a value with angle brackets
    // and check for Scale<...> and Dimension<...> structs
    if json_payload.contains("\"Quantity\"")
        && json_payload.contains("<")
        && json_payload.contains(">")
    {
        // Look for angle bracket content with Scale<...> and Dimension<...> structs
        let mut pos = 0;
        while let Some(bracket_start) = json_payload[pos..].find('<') {
            let absolute_pos = pos + bracket_start;
            let after_bracket = &json_payload[absolute_pos + 1..];

            // Use bracket counting to find the matching closing bracket
            if let Some(bracket_end) = find_matching_angle_bracket(after_bracket) {
                let inner_content = &after_bracket[..bracket_end];

                // Check for new format with Scale and Dimension structs
                // In the split format, these appear as separate JSON values like "Scale"},{"value":"<"
                if inner_content.contains("\"Scale\"") && inner_content.contains("\"Dimension\"") {
                    return true; // Found a valid Quantity type in split format
                }
            }

            pos = absolute_pos + 1;
        }
    }

    false
}

/// Check if a label array contains a whippyunits type
pub fn contains_whippyunits_type(label_array: &[serde_json::Value]) -> bool {
    // Look for "Quantity" as a literal value in any label part
    for part in label_array {
        if let Some(value) = part.get("value") {
            if let Some(text) = value.as_str() {
                if text == "Quantity" {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_contains_quantity_types_fast() {
        // Test with message containing new Quantity types with Scale<...> and Dimension<...> structs
        let message_with_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = 5.0.meters();\n```"}}}"#;
        assert!(contains_quantity_types_fast(message_with_quantity));

        // Test with message not containing Quantity types
        let message_without_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"hello\";\n```"}}}"#;
        assert!(!contains_quantity_types_fast(message_without_quantity));

        // Test with message containing "Quantity" but not in proper format
        let message_with_quantity_text = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity = some_value;\n```"}}}"#;
        assert!(!contains_quantity_types_fast(message_with_quantity_text));

        // Test with message containing "Quantity<" but without Scale<...> and Dimension<...> structs
        let message_with_insufficient_commas = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<1, 2, 3> = some_value;\n```"}}}"#;
        assert!(!contains_quantity_types_fast(
            message_with_insufficient_commas
        ));
    }

    #[test]
    fn test_validate_quantity_format() {
        // Test valid new Quantity format with Scale<...> and Dimension<...> structs
        let valid_quantity = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
        assert!(validate_quantity_format(valid_quantity));

        // Test invalid format without Scale<...> and Dimension<...> structs
        let invalid_quantity = "Quantity<1, 2, 3>";
        assert!(!validate_quantity_format(invalid_quantity));

        // Test with nested angle brackets
        let nested_quantity = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, Some<f64>>";
        assert!(validate_quantity_format(nested_quantity));
    }

    #[test]
    fn test_find_matching_angle_bracket() {
        // Test simple case
        assert_eq!(find_matching_angle_bracket("1, 2, 3>"), Some(7));

        // Test with nested brackets
        assert_eq!(find_matching_angle_bracket("1, 2, Some<f64>, 3>"), Some(18));

        // Test with no closing bracket
        assert_eq!(find_matching_angle_bracket("1, 2, 3"), None);

        // Test with multiple closing brackets
        assert_eq!(find_matching_angle_bracket("1, 2, 3>, 4>"), Some(7));
    }

    #[test]
    fn test_contains_whippyunits_type() {
        // Test with whippyunits type
        let label_with_quantity = vec![
            json!({"value": ": "}),
            json!({"value": "Quantity", "location": {"uri": "file://test.rs", "range": {"start": {"line": 1, "character": 0}, "end": {"line": 1, "character": 8}}}}),
            json!({"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"}),
        ];
        assert!(contains_whippyunits_type(&label_with_quantity));

        // Test without whippyunits type
        let label_without_quantity = vec![
            json!({"value": ": "}),
            json!({"value": "String"}),
            json!({"value": "()"}),
        ];
        assert!(!contains_whippyunits_type(&label_without_quantity));
    }
}
