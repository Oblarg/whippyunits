use crate::{inlay_hint_processor, quantity_detection, unit_formatter::UnitFormatter, LspProxy};
use serde_json::json;

#[test]
fn test_extract_raw_type_with_full_declaration() {
    let formatter = UnitFormatter::new();

    // Test with let declaration
    let let_declaration = "let result: Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(let_declaration);
    assert_eq!(
        raw_type,
        "let result: Quantity<Scale, Dimension<_M, _L<1>>, f64>"
    );

    // Test with let mut declaration
    let let_mut_declaration = "let mut result: Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(let_mut_declaration);
    assert_eq!(
        raw_type,
        "let mut result: Quantity<Scale, Dimension<_M, _L<1>>, f64>"
    );

    // Test with const declaration
    let const_declaration = "const result: Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(const_declaration);
    assert_eq!(
        raw_type,
        "const result: Quantity<Scale, Dimension<_M, _L<1>>, f64>"
    );

    // Test with static declaration
    let static_declaration = "static result: Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(static_declaration);
    assert_eq!(
        raw_type,
        "static result: Quantity<Scale, Dimension<_M, _L<1>>, f64>"
    );

    // Test with complex variable name
    let complex_var = "let my_complex_variable_name: Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(complex_var);
    assert_eq!(
        raw_type,
        "let my_complex_variable_name: Quantity<Scale, Dimension<_M, _L<1>>, f64>"
    );

    // Test with no declaration (should return empty)
    let no_declaration = "Quantity<Scale, Dimension<_M, _L<1>>, f64>";
    let raw_type = formatter.extract_raw_type_from_hover(no_declaration);
    assert_eq!(raw_type, "");
}

#[test]
fn test_fast_quantity_detection() {
    // Test with message containing new Quantity types with Scale<...> and Dimension<...> structs
    let message_with_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = 5.0.meters();\n```"}}}"#;
    assert!(quantity_detection::contains_quantity_types_fast(
        message_with_quantity
    ));

    // Test with message containing truncated Quantity types (Scale and Dimension with defaulted parameters)
    let message_with_truncated_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<Scale, Dimension<_M<0>, _L<1>>, f64> = 5.0.meters();\n```"}}}"#;
    assert!(quantity_detection::contains_quantity_types_fast(
        message_with_truncated_quantity
    ));

    // Test with message containing fully defaulted Quantity types (Scale and Dimension with no parameters)
    let message_with_fully_defaulted_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<Scale, Dimension, f64> = 5.0;\n```"}}}"#;
    assert!(quantity_detection::contains_quantity_types_fast(
        message_with_fully_defaulted_quantity
    ));

    // Test with message not containing Quantity types
    let message_without_quantity = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"hello\";\n```"}}}"#;
    assert!(!quantity_detection::contains_quantity_types_fast(
        message_without_quantity
    ));

    // Test with message containing "Quantity" but not in proper format
    let message_with_quantity_text = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: String = \"Quantity\";\n```"}}}"#;
    assert!(!quantity_detection::contains_quantity_types_fast(
        message_with_quantity_text
    ));

    // Test early opt-out: message with "Quantity<" but no Scale/Dimension (should be fast rejection)
    let message_with_quantity_but_no_whippyunits = r#"{"jsonrpc":"2.0","id":1,"result":{"contents":{"kind":"markdown","value":"```rust\nlet x: Quantity<SomeOtherType> = something;\n```"}}}"#;
    assert!(!quantity_detection::contains_quantity_types_fast(
        message_with_quantity_but_no_whippyunits
    ));
}

#[test]
fn test_validate_quantity_format() {
    // Test valid new Quantity format with Scale<...> and Dimension<...> structs
    let valid_quantity = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
    assert!(quantity_detection::validate_quantity_format(valid_quantity));

    // Test valid truncated Quantity format (Scale and Dimension with defaulted parameters)
    let valid_truncated_quantity = "Quantity<Scale, Dimension<_M<0>, _L<1>>, f64>";
    assert!(quantity_detection::validate_quantity_format(
        valid_truncated_quantity
    ));

    // Test valid fully defaulted Quantity format (Scale and Dimension with no parameters)
    let valid_fully_defaulted_quantity = "Quantity<Scale, Dimension, f64>";
    assert!(quantity_detection::validate_quantity_format(
        valid_fully_defaulted_quantity
    ));

    // Test invalid format without Scale<...> and Dimension<...> structs
    let invalid_quantity = "Quantity<1, 2, 3>";
    assert!(!quantity_detection::validate_quantity_format(
        invalid_quantity
    ));

    // Test with nested angle brackets
    let nested_quantity = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, Some<f64>>";
    assert!(quantity_detection::validate_quantity_format(
        nested_quantity
    ));
}

#[test]
fn test_find_matching_angle_bracket() {
    // Test simple case
    assert_eq!(
        quantity_detection::find_matching_angle_bracket("1, 2, 3>"),
        Some(7)
    );

    // Test with nested brackets
    assert_eq!(
        quantity_detection::find_matching_angle_bracket("1, 2, Some<f64>, 3>"),
        Some(18)
    );

    // Test with no closing bracket
    assert_eq!(
        quantity_detection::find_matching_angle_bracket("1, 2, 3"),
        None
    );

    // Test with multiple closing brackets
    assert_eq!(
        quantity_detection::find_matching_angle_bracket("1, 2, 3>, 4>"),
        Some(7)
    );
}

#[test]
fn test_hover_tooltip_processing() {
    let proxy = LspProxy::new();

    // Test hover response with new Quantity types (energy example)
    let hover_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": {
                "kind": "markdown",
                "value": "```rust\nlet energy_j: Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<1>, _L<2>, _T<-2>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> = 5.0.joules();\n```"
            }
        }
    });

    let response_str = serde_json::to_string(&hover_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();

    // Should contain pretty-printed type (hover format) - should be Joules (J)
    println!("Hover processed: {}", processed);
    assert!(processed.contains("Quantity<J, f64>"));
    // Should not contain the raw const generic parameters
    assert!(!processed.contains("const MASS_EXPONENT: i16"));
    assert!(!processed.contains("const LENGTH_EXPONENT: i16"));
    assert!(!processed.contains("const TIME_EXPONENT: i16"));
    // Should not contain the incorrect _A<0> generic type
    assert!(!processed.contains("_A<0>"));
}

#[test]
fn test_hover_tooltip_processing_i32() {
    let proxy = LspProxy::new();

    // Test hover response with i32 type (like in the user's case)
    let hover_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "contents": {
                "kind": "markdown",
                "value": "```rust\nlet length_i32: Quantity<Scale, Dimension<_M, _L<1>>, i32> = 10.meters();\n```"
            }
        }
    });

    let response_str = serde_json::to_string(&hover_response).unwrap();
    let processed = proxy.process_incoming(&response_str).unwrap();

    println!("Input hover: {}", response_str);
    println!("Processed hover: {}", processed);

    // Should contain pretty-printed type with i32, not f64
    assert!(processed.contains("Quantity<") && processed.contains("i32"));
    assert!(!processed.contains("f64"));
}

#[test]
fn test_type_conversion() {
    let converter = UnitFormatter::new();

    // Test basic type conversion with new format
    let input = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());
    assert!(result.contains("Quantity<m, f64>"));
    assert!(!result.contains("const"));
    assert!(!result.contains("MASS_EXPONENT"));

    // Test type conversion with truncated format
    let truncated_input = "Quantity<Scale, Dimension<_M<0>, _L<1>>, f64>";
    let truncated_result =
        converter.format_types(truncated_input, &crate::DisplayConfig::default());
    println!("Truncated input: {}", truncated_input);
    println!("Truncated result: {}", truncated_result);
    assert!(truncated_result.contains("Quantity<m, f64>"));
    assert!(!truncated_result.contains("const"));
    assert!(!truncated_result.contains("MASS_EXPONENT"));

    // Test type conversion with fully defaulted format (dimensionless)
    let fully_defaulted_input = "Quantity<Scale, Dimension, f64>";
    let fully_defaulted_result =
        converter.format_types(fully_defaulted_input, &crate::DisplayConfig::default());
    assert!(
        fully_defaulted_result.contains("Quantity<1, f64>")
            || fully_defaulted_result.contains("Quantity<dimensionless, f64>")
    );
    assert!(!fully_defaulted_result.contains("const"));
    assert!(!fully_defaulted_result.contains("MASS_EXPONENT"));
}

#[test]
fn test_composite_unresolved_type_conversion() {
    let converter = UnitFormatter::new();

    // Test composite unresolved type conversion with new format
    let input = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64> + Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());
    assert!(result.contains("m"));
    assert!(!result.contains("const"));
}

#[test]
fn test_verbose_partially_resolved_type() {
    let converter = UnitFormatter::new();

    // Test verbose partially resolved type conversion with new format
    let input = "Quantity<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
    let result = converter.format_types(
        input,
        &crate::DisplayConfig {
            verbose: true,
            unicode: true,
            include_raw: false,
        },
    );
    assert!(result.contains("Quantity<meter"));
    assert!(result.contains("f64"));
    assert!(!result.contains("const"));
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
                "value": "```rust\nimpl<const MASS_EXPONENT: i16, const LENGTH_EXPONENT: i16, const TIME_EXPONENT: i16, const CURRENT_EXPONENT: i16, const TEMPERATURE_EXPONENT: i16, const AMOUNT_EXPONENT: i16, const LUMINOSITY_EXPONENT: i16, const ANGLE_EXPONENT: i16, const SCALE_P2: i16, const SCALE_P3: i16, const SCALE_P5: i16, const SCALE_PI: i16, T> Add for Quantity<Scale<_2<SCALE_P2>, _3<SCALE_P3>, _5<SCALE_P5>, _Pi<SCALE_PI>>, Dimension<_M<MASS_EXPONENT>, _L<LENGTH_EXPONENT>, _T<TIME_EXPONENT>, _I<CURRENT_EXPONENT>, _Θ<TEMPERATURE_EXPONENT>, _N<AMOUNT_EXPONENT>, _J<LUMINOSITY_EXPONENT>, _A<ANGLE_EXPONENT>>, T>\n```"
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
        json!({"value": "<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<1>, _L<0>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>"}),
    ];
    assert!(processor.contains_whippyunits_type(&label_with_quantity));

    // Test without whippyunits type
    let label_without_quantity = vec![
        json!({"value": ": "}),
        json!({"value": "String"}),
        json!({"value": "()"}),
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
        json!({"value": "<Scale<_2<0>, _3<0>, _5<0>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<0>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>"}),
    ];

    processor
        .convert_whippyunits_hint(&mut label_array)
        .unwrap();

    // Should have 2 parts now (removed generic params)
    assert_eq!(label_array.len(), 2);

    // First part should still be ": "
    assert_eq!(label_array[0]["value"], ": ");

    // Second part should be pretty-printed and have location preserved
    let second_part = &label_array[1];
    assert!(second_part["value"]
        .as_str()
        .unwrap()
        .contains("Quantity<m, f64>"));
    assert!(second_part.get("location").is_some());
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

#[test]
fn test_scale_parsing_with_missing_pi_parameter() {
    let converter = UnitFormatter::new();

    // Test the user's specific case: Scale<_2<-3>, _3<0>, _5<-3>> (missing _Pi parameter)
    let input = "Quantity<Scale<_2<-3>, _3<0>, _5<-3>>, Dimension<_M<1>>, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());

    println!("Input: {}", input);
    println!("Result: {}", result);

    // Should successfully parse and format the type
    assert!(result.contains("Quantity<"));
    assert!(!result.contains("Scale<_2<-3>"));
    assert!(!result.contains("const"));
}

#[test]
fn test_wholly_unresolved_type_formatting() {
    let converter = UnitFormatter::new();

    // Test the user's specific case: wholly unresolved type with all parameters as _
    // This matches the exact format from the IDE hover
    let input = "Quantity<Scale<_2<_>, _3<_>, _5<_>, _Pi<_>>, Dimension<_M<_>, _L<_>, _T<_>, _I<_>, _Θ<_>, _N<_>, _J<_>, _A<_>>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());

    // Should format as wholly unresolved type
    assert!(result.contains("Quantity<?, f64>"));
    assert!(!result.contains("Scale<_2<_>"));
    assert!(!result.contains("Dimension<_M<_>"));
    assert!(!result.contains("const"));

    // Test with inlay hint formatting
    let inlay_result = converter.format_types_inlay_hint(input, &crate::DisplayConfig::default());
    println!("Inlay hint result: {}", inlay_result);
    assert!(inlay_result.contains("Quantity<?, f64>"));
}

#[test]
fn test_partially_resolved_type_formatting() {
    let converter = UnitFormatter::new();

    // Test partially resolved type: some dimensions known, some scales unknown
    let input = "Quantity<Scale<_2<_>, _3<0>, _5<_>, _Pi<0>>, Dimension<_M<0>, _L<1>, _T<0>, _I<_>, _Θ<0>, _N<0>, _J<0>, _A<0>>, f64>";
    let result = converter.format_types(input, &crate::DisplayConfig::default());

    println!("Input: {}", input);
    println!("Result: {}", result);

    // Should format with best-effort guesses and unicode question marks for unresolved parts
    assert!(result.contains("Quantity<"));
    assert!(result.contains("f64"));
    assert!(!result.contains("Scale<_2<_>"));
    assert!(!result.contains("Dimension<_M<_>"));
    assert!(!result.contains("const"));

    // The result should contain some resolved parts (like length dimension) and question marks for unresolved parts
    // This tests that the partial resolution logic works correctly
}

#[test]
fn test_pid_controller_nested_quantity_types() {
    let converter = UnitFormatter::new();

    // Test complex nested type with multiple Quantity types in PIDController
    // This tests that the algorithm finds and transforms ALL Quantity types, not just the first one
    let input = "let mut controller: PIDController<Quantity<Scale, Dimension<_M, _L<1>>>, Quantity<Scale<_2<_>, _3<_>, _5<_>, _Pi<_>>, Dimension<_M<_>, _L<_>, _T<_>, _I<_>, _Θ<_>, _N<_>, _J<_>, _A<_>>>, Quantity<Scale<_2<-3>, _3, _5<-3>>, Dimension<_M, _L, _T<1>>>, Quantity<Scale, Dimension<_M<1>, _L<1>, _T<-3>, _I<-1>>>, Quantity<Scale<_2<3>, _3, _5<3>>, Dimension<_M<1>, _L<1>, _T<-4>, _I<-1>>>, Quantity<Scale<_2<-3>, _3, _5<-3>>, Dimension<_M<1>, _L<1>, _T<-2>, _I<-1>>>, Quantity<Scale<_2<_>, _3<_>, _5<_>, _Pi<_>>, Dimension<_M<_>, _L<_>, _T<_>, _I<_>, _Θ<_>, _N<_>, _J<_>, _A<_>>>, Quantity<Scale<_2<_>, _3<_>, _5<_>, _Pi<_>>, Dimension<_M<_>, _L<_>, _T<_>, _I<_>, _Θ<_>, _N<_>, _J<_>, _A<_>>>>";

    let result = converter.format_types(input, &crate::DisplayConfig::default());

    println!("Input: {}", input);
    println!("Result: {}", result);

    // Verify that all Quantity types were transformed (not left as raw Scale/Dimension format)
    // Count how many "Quantity<Scale" appear in the result - should be 0 (all transformed)
    let raw_quantity_count = result.matches("Quantity<Scale").count();
    if raw_quantity_count > 0 {
        // Find where the untransformed types are
        let mut positions = Vec::new();
        let mut search_pos = 0;
        while let Some(pos) = result[search_pos..].find("Quantity<Scale") {
            positions.push(search_pos + pos);
            search_pos += pos + 1;
        }
        panic!("Found {} untransformed Quantity<Scale types at positions {:?}. All should be transformed.\nResult: {}", raw_quantity_count, positions, result);
    }

    // Verify that we have formatted Quantity types (should contain "Quantity<" with formatted units)
    assert!(
        result.contains("Quantity<"),
        "Result should contain formatted Quantity types"
    );

    // Verify that the PIDController structure is preserved
    assert!(
        result.contains("PIDController<"),
        "Result should preserve PIDController structure"
    );

    // Count how many formatted Quantity types appear - should be 8 (one for each parameter)
    let formatted_quantity_count = result.matches("Quantity<").count();
    assert_eq!(
        formatted_quantity_count, 8,
        "Expected 8 formatted Quantity types, found {}",
        formatted_quantity_count
    );

    // Verify that unresolved types are formatted as "Quantity<?"
    let unresolved_count = result.matches("Quantity<?").count();
    assert_eq!(
        unresolved_count, 3,
        "Expected 3 unresolved Quantity types (the ones with _ placeholders), found {}",
        unresolved_count
    );

    // Verify that resolved types are formatted with actual units (not raw Scale/Dimension)
    assert!(
        !result.contains("Scale<_2<"),
        "Result should not contain raw Scale parameters"
    );
    assert!(
        !result.contains("Dimension<_M<"),
        "Result should not contain raw Dimension parameters"
    );
}
