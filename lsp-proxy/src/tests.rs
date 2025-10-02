use crate::{LspProxy, unit_formatter::UnitFormatter, quantity_detection, inlay_hint_processor};
use serde_json::json;

#[test]
fn test_fast_quantity_detection() {
    // Test with message containing Quantity types (8+ commas for valid detection)
    let message_with_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 5.0.meters();\n```"}}}"#;
    assert!(quantity_detection::contains_quantity_types_fast(message_with_quantity));
    
    // Test with message not containing Quantity types
    let message_without_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"hello\";\n```"}}}"#;
    assert!(!quantity_detection::contains_quantity_types_fast(message_without_quantity));
    
    // Test with message containing "Quantity" but not in proper format
    let message_with_quantity_text = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"Quantity\";\n```"}}}"#;
    assert!(!quantity_detection::contains_quantity_types_fast(message_with_quantity_text));
}

#[test]
fn test_validate_quantity_format() {
    // Test valid Quantity format with 8+ commas
    let valid_quantity = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>";
    assert!(quantity_detection::validate_quantity_format(valid_quantity));
    
    // Test invalid format with insufficient commas
    let invalid_quantity = "Quantity<1, 2, 3>";
    assert!(!quantity_detection::validate_quantity_format(invalid_quantity));
    
    // Test with nested angle brackets
    let nested_quantity = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, Some<f64>>";
    assert!(quantity_detection::validate_quantity_format(nested_quantity));
}

#[test]
fn test_find_matching_angle_bracket() {
    // Test simple case
    assert_eq!(quantity_detection::find_matching_angle_bracket("1, 2, 3>"), Some(7));
    
    // Test with nested brackets
    assert_eq!(quantity_detection::find_matching_angle_bracket("1, 2, Some<f64>, 3>"), Some(18));
    
    // Test with no closing bracket
    assert_eq!(quantity_detection::find_matching_angle_bracket("1, 2, 3"), None);
    
    // Test with multiple closing brackets
    assert_eq!(quantity_detection::find_matching_angle_bracket("1, 2, 3>, 4>"), Some(7));
}

#[test]
fn test_hover_tooltip_processing() {
    let proxy = LspProxy::new();
    
    // Test hover response with Quantity types
    let hover_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": {
                "kind": "markdown",
                "value": "```rust\nlet x: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 5.0.meters();\n```"
            }
        }
    });
    
    let response_str = serde_json::to_string(&hover_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();
    
    // Should contain pretty-printed type (hover format)
    println!("Hover processed: {}", processed);
    assert!(processed.contains("Quantity<m; Length>"));
    // Should not contain the raw const generic parameters
    assert!(!processed.contains("const MASS_EXPONENT: i16"));
    assert!(!processed.contains("const LENGTH_EXPONENT: i16"));
    assert!(!processed.contains("const TIME_EXPONENT: i16"));
}

#[test]
fn test_inlay_hint_integration() {
    let proxy = LspProxy::new();
    
    // Test inlay hint response with Quantity types
    let inlay_hint_response = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": [
            {
                "position": {"line": 12, "character": 17},
                "label": [
                    {"value": ": "},
                    {"value": "Quantity"},
                    {"value": "<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"}
                ],
                "kind": 1,
                "data": {"file_id": 0, "hash": "123", "resolve_range": {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 17}}, "version": 1}
            }
        ]
    });
    
    let response_str = serde_json::to_string(&inlay_hint_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();
    
    // Should contain pretty-printed type
    assert!(processed.contains("Quantity<m, f64>"));
    // Should preserve all metadata
    assert!(processed.contains("position"));
    assert!(processed.contains("data"));
    assert!(processed.contains("resolve_range"));
}

#[test]
fn test_type_conversion() {
    let converter = UnitFormatter::new();
    
    // Test basic type conversion
    let input = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());
    assert!(result.contains("Quantity<m; Length>"));
    assert!(!result.contains("const"));
    assert!(!result.contains("MASS_EXPONENT"));
}

#[test]
fn test_text_conversion() {
    let converter = UnitFormatter::new();
    
    // Test text with multiple Quantity types
    let input = "let x: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 5.0.meters();\nlet y: Quantity<1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> = 10.0.kilograms();";
    let result = converter.format_types(input, &crate::DisplayConfig { verbose: false, unicode: false, include_raw: false });
    
    assert!(result.contains("m"));
    assert!(result.contains("kg"));
    assert!(!result.contains("const"));
    assert!(!result.contains("MASS_EXPONENT"));
}

#[test]
fn test_composite_unresolved_type_conversion() {
    let converter = UnitFormatter::new();
    
    // Test composite unresolved type conversion
    let input = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64> + Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());
    assert!(result.contains("m"));
    assert!(!result.contains("const"));
}

#[test]
fn test_verbose_partially_resolved_type() {
    let converter = UnitFormatter::new();
    
    // Test verbose partially resolved type conversion
    let input = "Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig { verbose: true, unicode: true, include_raw: false });
    println!("Verbose test result: {}", result);
    assert!(result.contains("Quantity<meter; Length"));
    assert!(!result.contains("const"));
}

#[test]
fn test_inlay_hint_unresolved_types() {
    let proxy = LspProxy::new();
    
    // Test inlay hint with unresolved types
    let inlay_hint_response = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": [
            {
                "position": {"line": 12, "character": 17},
                "label": [
                    {"value": ": "},
                    {"value": "Quantity"},
                    {"value": "<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"}
                ],
                "kind": 1,
                "data": {"file_id": 0, "hash": "123", "resolve_range": {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 17}}, "version": 1}
            }
        ]
    });
    
    let response_str = serde_json::to_string(&inlay_hint_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();
    
    // Extract JSON payload from LSP message format
    let json_payload = if processed.starts_with("Content-Length:") {
        // Extract JSON from LSP message format
        let json_start = processed.find("\r\n\r\n").unwrap() + 4;
        &processed[json_start..]
    } else {
        &processed
    };
    
    // Parse the processed result to verify the transformation
    let processed_json: serde_json::Value = serde_json::from_str(json_payload).unwrap();
    let result_array = processed_json["result"].as_array().unwrap();
    let hint = &result_array[0];
    let label_array = hint["label"].as_array().unwrap();
    
    // Should have 2 parts now (removed generic params)
    assert_eq!(label_array.len(), 2);
    
    // First part should be ": "
    assert_eq!(label_array[0]["value"], ": ");
    
    // Second part should be pretty-printed
    let second_part = &label_array[1];
    let label_str = second_part["value"].as_str().unwrap();
    // The processor should transform "Quantity" to a pretty-printed Quantity<unit, type> format
    // So we should check for the pretty-printed result instead
    assert!(label_str.contains("Quantity<m, f64>") || label_str.contains("Quantity<kg, f64>") || label_str.contains("Quantity<s, f64>"));
}

#[test]
fn test_add_sub_trait_signature_transformation() {
    let proxy = LspProxy::new();
    
    // Test Add trait
    let add_hover_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": {
                "kind": "markdown",
                "value": "```rust\nimpl<const MASS_EXPONENT: i16, const LENGTH_EXPONENT: i16, const TIME_EXPONENT: i16, const CURRENT_EXPONENT: i16, const TEMPERATURE_EXPONENT: i16, const AMOUNT_EXPONENT: i16, const LUMINOSITY_EXPONENT: i16, const ANGLE_EXPONENT: i16, const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16, T> Add for Quantity<MASS_EXPONENT, LENGTH_EXPONENT, TIME_EXPONENT, CURRENT_EXPONENT, TEMPERATURE_EXPONENT, AMOUNT_EXPONENT, LUMINOSITY_EXPONENT, ANGLE_EXPONENT, SCALE_P2, SCALE_P3, SCALE_P5, SCALE_PI, T>\n```"
            }
        }
    });
    
    let response_str = serde_json::to_string(&add_hover_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();
    
    // Extract JSON payload from LSP message format
    let json_payload = if processed.starts_with("Content-Length:") {
        // Extract JSON from LSP message format
        let json_start = processed.find("\r\n\r\n").unwrap() + 4;
        &processed[json_start..]
    } else {
        &processed
    };
    
    // Parse the processed result to verify the transformation
    let processed_json: serde_json::Value = serde_json::from_str(json_payload).unwrap();
    let contents = &processed_json["result"]["contents"];
    let contents_str = contents["value"].as_str().unwrap();
    
    println!("Processed Add trait: {}", contents_str);
    // Should show a simplified Add trait signature
    assert!(contents_str.contains("impl Add for"));
    assert!(contents_str.contains("Quantity<"));
    // Should not contain the const generic parameters
    assert!(!contents_str.contains("const MASS_EXPONENT: i16"));
    assert!(!contents_str.contains("const LENGTH_EXPONENT: i16"));
    assert!(!contents_str.contains("const TIME_EXPONENT: i16"));
}



// Inlay hint processor tests (moved from inlay_hint_processor.rs)

#[test]
fn test_inlay_hint_contains_whippyunits_type() {
    let processor = inlay_hint_processor::InlayHintProcessor::new();
    
    // Test with whippyunits type
    let label_with_quantity = vec![
        json!({"value": ": "}),
        json!({"value": "Quantity", "location": {"uri": "file://test.rs", "range": {"start": {"line": 1, "character": 0}, "end": {"line": 1, "character": 8}}}}),
        json!({"value": "<1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"})
    ];
    assert!(processor.contains_whippyunits_type(&label_with_quantity));
    
    // Test without whippyunits type
    let label_without_quantity = vec![
        json!({"value": ": "}),
        json!({"value": "String"}),
        json!({"value": "()"})
    ];
    assert!(!processor.contains_whippyunits_type(&label_without_quantity));
}

#[test]
fn test_inlay_hint_convert_whippyunits_hint() {
    let processor = inlay_hint_processor::InlayHintProcessor::new();
    
    let mut label_array = vec![
        json!({"value": ": "}),
        json!({
            "value": "Quantity", 
            "location": {
                "uri": "file://test.rs", 
                "range": {
                    "start": {"line": 1, "character": 0}, 
                    "end": {"line": 1, "character": 8}
                }
            }
        }),
        json!({"value": "<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"})
    ];
    
    processor.convert_whippyunits_hint(&mut label_array).unwrap();
    
    // Should have 2 parts now (removed generic params)
    assert_eq!(label_array.len(), 2);
    
    // First part should still be ": "
    assert_eq!(label_array[0]["value"], ": ");
    
    // Second part should be pretty-printed and have location preserved
    let second_part = &label_array[1];
    assert!(second_part["value"].as_str().unwrap().contains("Quantity<m, f64>"));
    assert!(second_part.get("location").is_some());
}

#[test]
fn test_inlay_hint_process_inlay_hint_response() {
    let processor = inlay_hint_processor::InlayHintProcessor::new();
    
    let response = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": [
            {
                "position": {"line": 12, "character": 17},
                "label": [
                    {"value": ": "},
                    {
                        "value": "Quantity",
                        "location": {
                            "uri": "file://test.rs",
                            "range": {
                                "start": {"line": 1, "character": 0},
                                "end": {"line": 1, "character": 8}
                            }
                        }
                    },
                    {"value": "<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"}
                ],
                "kind": 1,
                "data": {"file_id": 0, "hash": "123", "resolve_range": {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 17}}, "version": 1}
            }
        ]
    });
    
    let response_str = serde_json::to_string(&response).unwrap();
    let processed = processor.process_inlay_hint_response(&response_str).unwrap();
    
    // Should contain pretty-printed type
    assert!(processed.contains("Quantity<m, f64>"));
    // Should preserve all metadata
    assert!(processed.contains("location"));
    assert!(processed.contains("resolve_range"));
    assert!(processed.contains("data"));
}

#[test]
fn test_inlay_hint_real_inlay_hint_transformation() {
    let processor = inlay_hint_processor::InlayHintProcessor::new();
    
    // Real inlay hint response from our test output
    let real_response = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "result": [
            {
                "position": {"line": 12, "character": 17},
                "label": [
                    {"value": ": "},
                    {
                        "value": "Quantity",
                        "location": {
                            "uri": "file:///workspace/src/lib.rs",
                            "range": {
                                "start": {"line": 64, "character": 11},
                                "end": {"line": 64, "character": 19}
                            }
                        }
                    },
                    {"value": "<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>"}
                ],
                "kind": 1,
                "paddingLeft": false,
                "paddingRight": false,
                "data": {
                    "file_id": 0,
                    "hash": "11030576060064372646",
                    "resolve_range": {
                        "start": {"line": 12, "character": 8},
                        "end": {"line": 12, "character": 17}
                    },
                    "version": 1
                }
            }
        ]
    });
    
    let response_str = serde_json::to_string(&real_response).unwrap();
    let processed = processor.process_inlay_hint_response(&response_str).unwrap();
    
    // Parse the processed result to verify the transformation
    let processed_json: serde_json::Value = serde_json::from_str(&processed).unwrap();
    let result_array = processed_json["result"].as_array().unwrap();
    let hint = &result_array[0];
    let label_array = hint["label"].as_array().unwrap();
    
    // Should have 2 parts now (removed generic params)
    assert_eq!(label_array.len(), 2);
    
    // First part should be ": "
    assert_eq!(label_array[0]["value"], ": ");
    
    // Second part should be pretty-printed and have location preserved
    let second_part = &label_array[1];
    let pretty_value = second_part["value"].as_str().unwrap();
    
    // Should contain the pretty-printed type
    assert!(pretty_value.contains("Quantity<m, f64>"));
    println!("Original: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>");
    println!("Pretty: '{}'", pretty_value);
    
    // Should preserve the location for click-to-source
    assert!(second_part.get("location").is_some());
    let location = &second_part["location"];
    assert!(location["uri"].as_str().unwrap().ends_with("/src/lib.rs"));
    
    // Should preserve all other metadata
    assert!(hint.get("position").is_some());
    assert!(hint.get("kind").is_some());
    assert!(hint.get("data").is_some());
    assert!(hint["data"].get("resolve_range").is_some());
    
    println!("Original: Quantity<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, f64>");
    println!("Pretty: '{}'", pretty_value);
    println!("Pretty value length: {}", pretty_value.len());
}

#[test]
fn test_inlay_hint_exponent_pruning() {
    let processor = inlay_hint_processor::InlayHintProcessor::new();
    
    // Test that ^1 exponents are pruned but meaningful exponents are preserved
    let test_cases = vec![
        ("mm¹", "mm"),           // ^1 should be removed
        ("mm²", "mm²"),          // ^2 should be preserved
        ("mm³", "mm³"),          // ^3 should be preserved
        ("mm⁻¹", "mm⁻¹"),        // ^-1 should be preserved (meaningful negative exponent)
        ("m¹s²", "ms²"),         // ^1 should be removed, ^2 preserved
        ("kg¹m²s⁻²", "kgm²s⁻²"), // ^1 should be removed, others preserved
    ];
    
    for (input, expected) in test_cases {
        let result = processor.prune_inlay_hint_exponents(input);
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}