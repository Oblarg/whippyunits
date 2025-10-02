//! Shared Quantity type detection logic for LSP proxy

/// Fast string search to detect Quantity types without deserialization
/// This performs a performant string search for "Quantity<" patterns
pub fn contains_quantity_types_fast(json_payload: &str) -> bool {
    // Look for the specific pattern "Quantity<" followed by angle brackets
    // This is much faster than deserializing the entire JSON
    if !json_payload.contains("Quantity<") {
        // Also check for split format where "Quantity" and "<...>" are separate
        if json_payload.contains("\"Quantity\"") && json_payload.contains("<") && json_payload.contains(">") {
            // This might be the split format, validate it
            return validate_split_quantity_format(json_payload);
        }
        return false;
    }
    
    // Additional validation: ensure we have proper Quantity<...> format
    // Count the number of commas between angle brackets to validate the format
    validate_quantity_format(json_payload)
}

/// Validate that Quantity<...> has the expected format with proper comma count
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
                    // Count commas to validate this is likely a Quantity type
                    // We expect at least 8 commas for the basic format (8 dimension exponents)
                    let comma_count = inner_content.matches(',').count();
                    if comma_count >= 8 {
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
    let mut depth = 0;
    for (i, ch) in text.char_indices() {
        match ch {
            '<' => depth += 1,
            '>' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

/// Validate split Quantity format where "Quantity" and "<...>" are separate JSON values
pub fn validate_split_quantity_format(json_payload: &str) -> bool {
    // Simple approach: look for the pattern "Quantity" followed by a value with angle brackets
    // and count commas in the angle bracket content
    if json_payload.contains("\"Quantity\"") && json_payload.contains("<") && json_payload.contains(">") {
        // Look for angle bracket content with enough commas
        let mut pos = 0;
        while let Some(bracket_start) = json_payload[pos..].find('<') {
            let absolute_pos = pos + bracket_start;
            let after_bracket = &json_payload[absolute_pos + 1..];
            
            if let Some(bracket_end) = after_bracket.find('>') {
                let inner_content = &after_bracket[..bracket_end];
                
                // Count commas to validate this is likely a Quantity type
                let comma_count = inner_content.matches(',').count();
                if comma_count >= 8 {
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
        // Test with message containing Quantity types (8+ commas for valid detection)
        let message_with_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0> = 5.0.meters();\n```"}}}"#;
        assert!(contains_quantity_types_fast(message_with_quantity));
        
        // Test with message not containing Quantity types
        let message_without_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"hello\";\n```"}}}"#;
        assert!(!contains_quantity_types_fast(message_without_quantity));
        
        // Test with message containing "Quantity" but not in proper format
        let message_with_quantity_text = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity = some_value;\n```"}}}"#;
        assert!(!contains_quantity_types_fast(message_with_quantity_text));
        
        // Test with message containing "Quantity<" but with insufficient commas
        let message_with_insufficient_commas = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<1, 2, 3> = some_value;\n```"}}}"#;
        assert!(!contains_quantity_types_fast(message_with_insufficient_commas));
    }

    #[test]
    fn test_validate_quantity_format() {
        // Test valid Quantity format with 8+ commas
        let valid_quantity = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>";
        assert!(validate_quantity_format(valid_quantity));
        
        // Test invalid format with insufficient commas
        let invalid_quantity = "Quantity<1, 2, 3>";
        assert!(!validate_quantity_format(invalid_quantity));
        
        // Test with nested angle brackets
        let nested_quantity = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Some<f64>>";
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
            json!({"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"})
        ];
        assert!(contains_whippyunits_type(&label_with_quantity));
        
        // Test without whippyunits type
        let label_without_quantity = vec![
            json!({"value": ": "}),
            json!({"value": "String"}),
            json!({"value": "()"})
        ];
        assert!(!contains_whippyunits_type(&label_without_quantity));
    }
}
