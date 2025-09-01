use serde_json::{json, Value};
use crate::{WhippyUnitsTypeConverter, DisplayConfig};
use anyhow::Result;

/// Process inlay hint responses to pretty-print whippyunits types
#[derive(Clone)]
pub struct InlayHintProcessor {
    converter: WhippyUnitsTypeConverter,
    display_config: DisplayConfig,
}

impl InlayHintProcessor {
    pub fn new() -> Self {
        Self {
            converter: WhippyUnitsTypeConverter::new(),
            display_config: DisplayConfig::default(),
        }
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            converter: WhippyUnitsTypeConverter::new(),
            display_config,
        }
    }

    /// Process an inlay hint response, converting whippyunits types to pretty format
    pub fn process_inlay_hint_response(&self, message: &str) -> Result<String> {
        // Parse the JSON message
        let mut json_value: Value = serde_json::from_str(message)?;
        
        // Check if this is an inlay hint response with results
        if let Some(result) = json_value.get_mut("result") {
            // Handle both array (inlay hint requests) and object (resolve responses)
            if let Some(results_array) = result.as_array_mut() {
                // Process each inlay hint in the results array
                for hint in results_array {
                    self.process_single_hint(hint)?;
                }
            } else if let Some(single_hint) = result.as_object_mut() {
                // Process a single inlay hint object (resolve response)
                self.process_single_hint_object(single_hint)?;
            }
        }
        
        // Convert back to string
        Ok(serde_json::to_string(&json_value)?)
    }

    /// Process a single inlay hint, converting whippyunits types if present
    fn process_single_hint(&self, hint: &mut Value) -> Result<()> {
        // Get the label array
        if let Some(label) = hint.get_mut("label") {
            if let Some(label_array) = label.as_array_mut() {
                // Check if this hint contains a whippyunits type
                if self.contains_whippyunits_type(label_array) {
                    self.convert_whippyunits_hint(label_array)?;
                }
            }
        }
        Ok(())
    }

    /// Process a single inlay hint object (for resolve responses)
    fn process_single_hint_object(&self, hint_obj: &mut serde_json::Map<String, Value>) -> Result<()> {
        // Check if this hint contains a whippyunits type first
        let has_whippyunits = if let Some(label) = hint_obj.get("label") {
            if let Some(label_array) = label.as_array() {
                self.contains_whippyunits_type(label_array)
            } else {
                false
            }
        } else {
            false
        };

        if has_whippyunits {
            // Get the label array from the object and process it
            if let Some(label) = hint_obj.get_mut("label") {
                if let Some(label_array) = label.as_array_mut() {
                    self.convert_whippyunits_hint(label_array)?;
                }
            }
            
            // Always add the unit! macro completion, regardless of resolution state
            self.add_seeded_unit_macro_text_edit(hint_obj)?;
        }
        Ok(())
    }

    /// Check if a label array contains a whippyunits type
    fn contains_whippyunits_type(&self, label_array: &[Value]) -> bool {
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

    /// Convert a whippyunits inlay hint to pretty format
    fn convert_whippyunits_hint(&self, label_array: &mut Vec<Value>) -> Result<()> {
        // Find the parts we need to work with
        let mut quantity_part_index = None;
        let mut generic_params_part_index = None;
        
        for (i, part) in label_array.iter().enumerate() {
            if let Some(value) = part.get("value") {
                if let Some(text) = value.as_str() {
                    if text == "Quantity" {
                        quantity_part_index = Some(i);
                    } else if text.starts_with('<') && text.ends_with('>') {
                        generic_params_part_index = Some(i);
                    }
                }
            }
        }

        // If we found both parts, process them
        if let (Some(quantity_idx), Some(generic_idx)) = (quantity_part_index, generic_params_part_index) {
            // Extract the generic parameters
            let generic_params = if let Some(value) = label_array[generic_idx].get("value") {
                value.as_str().unwrap_or("")
            } else {
                return Ok(());
            };

            // Construct the full type string
            let full_type = format!("Quantity{}", generic_params);
            
            // For inlay hints, use the ultra-terse format that shows only the unit literal
            let pretty_type = self.converter.convert_types_in_text_inlay_hint(&full_type);
            
            // For inlay hints specifically, prune ^1 exponents while keeping meaningful ones
            let pretty_type = self.prune_inlay_hint_exponents(&pretty_type);
            
            // Replace the "Quantity" part with the pretty version
            // Preserve the location information if it exists
            let quantity_part = &label_array[quantity_idx];
            let mut new_quantity_part = json!({
                "value": pretty_type
            });
            
            // Copy over the location if it exists
            if let Some(location) = quantity_part.get("location") {
                new_quantity_part["location"] = location.clone();
            }
            
            label_array[quantity_idx] = new_quantity_part;
            
            // Remove the generic parameters part
            label_array.remove(generic_idx);
        }
        
        Ok(())
    }



    /// Check if this is an unresolved type that could benefit from a seeded unit macro
    fn is_unresolved_type(&self, hint_obj: &serde_json::Map<String, Value>) -> bool {
        // Look for unresolved types in the label
        if let Some(label) = hint_obj.get("label") {
            if let Some(label_array) = label.as_array() {
                for part in label_array {
                    if let Some(value) = part.get("value") {
                        if let Some(text) = value.as_str() {
                            // Check for unresolved patterns
                            if text.contains("9223372036854775807") || text.contains("_") {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Add a unit! macro text edit to the hint
    fn add_seeded_unit_macro_text_edit(&self, hint_obj: &mut serde_json::Map<String, Value>) -> Result<()> {
        // Generate the unit! macro text
        let seeded_text = self.generate_seeded_unit_macro(hint_obj)?;
        
        // Extract position before any mutable borrows
        let position = if let Some(pos) = hint_obj.get("position") {
            pos.clone()
        } else {
            return Ok(());
        };
        
        // Replace existing textEdits with our seeded unit macro
        if let Some(existing_text_edits) = hint_obj.get_mut("textEdits") {
            if let Some(text_edits_array) = existing_text_edits.as_array_mut() {
                // Clear existing text edits and add our unit! macro one
                text_edits_array.clear();
                
                // Get the range from the first existing text edit (if any)
                let range = if let Some(first_edit) = text_edits_array.first() {
                    if let Some(range) = first_edit.get("range") {
                        range.clone()
                    } else {
                        // Fallback: use position as range
                        json!({
                            "start": position,
                            "end": position
                        })
                    }
                } else {
                    // No existing text edits, use position as range
                    json!({
                        "start": position,
                        "end": position
                    })
                };
                
                // Add our unit! macro text edit
                let text_edit = json!({
                    "range": range,
                    "newText": seeded_text
                });
                text_edits_array.push(text_edit);
            }
        } else {
            // No existing textEdits, create new array
            let range = json!({
                "start": position,
                "end": position
            });
            
            let text_edit = json!({
                "range": range,
                "newText": seeded_text
            });
            hint_obj.insert("textEdits".to_string(), json!([text_edit]));
        }

        Ok(())
    }

    /// Generate a seeded unit macro based on the unresolved type
    fn generate_seeded_unit_macro(&self, hint_obj: &serde_json::Map<String, Value>) -> Result<String> {
        // We need to reconstruct the original unresolved type from the pretty-printed version
        // Look for the pretty-printed type in the label
        let mut pretty_type = String::new();
        
        if let Some(label) = hint_obj.get("label") {
            if let Some(label_array) = label.as_array() {
                for part in label_array {
                    if let Some(value) = part.get("value") {
                        if let Some(text) = value.as_str() {
                            // Skip the ": " part
                            if text != ": " {
                                pretty_type.push_str(text);
                            }
                        }
                    }
                }
            }
        }

        // Convert the pretty-printed type to a unit! macro format
        let unit_macro = format!("unit!({})", self.convert_pretty_type_to_unit_macro(&pretty_type));
        
        // Return just the type annotation with the unit! macro
        Ok(format!(": {}", unit_macro))
    }

    /// Convert pretty-printed type to unit! macro format
    fn convert_pretty_type_to_unit_macro(&self, pretty_type: &str) -> String {
        // Remove the "Unresolved type - " prefix if present
        let clean_type = if pretty_type.starts_with("Unresolved type - ") {
            &pretty_type[18..] // Skip "Unresolved type - "
        } else {
            pretty_type
        };
        
        // Convert pretty-printed types like "mmˀ" to unit macro format like "mm^"
        // Remove the ? and place cursor after the caret
        clean_type
            .replace("ˀ", "^")  // Replace superscript question mark with just ^
            .replace("⁻", "^-")  // Replace superscript minus with ^-
            .replace("¹", "^1")  // Replace superscript 1 with ^1
            .replace("²", "^2")  // Replace superscript 2 with ^2
            .replace("³", "^3")  // Replace superscript 3 with ^3
            .replace("⁴", "^4")  // Replace superscript 4 with ^4
            .replace("⁵", "^5")  // Replace superscript 5 with ^5
            .replace("⁶", "^6")  // Replace superscript 6 with ^6
            .replace("⁷", "^7")  // Replace superscript 7 with ^7
            .replace("⁸", "^8")  // Replace superscript 8 with ^8
            .replace("⁹", "^9")  // Replace superscript 9 with ^9
            .replace("⁰", "^0")  // Replace superscript 0 with ^0
    }

    /// Prune ^1 exponents from inlay hint display while keeping meaningful exponents
    fn prune_inlay_hint_exponents(&self, pretty_type: &str) -> String {
        // In our pretty-printed output, we use Unicode superscripts like ¹, ², ³, etc.
        // We want to remove ¹ (superscript 1) but keep all other superscripts
        // Note: ⁻¹ is a single Unicode character for "superscript negative one"
        // We need to handle this specially to avoid breaking it
        let mut result = pretty_type.to_string();
        
        // First, replace ⁻¹ with ⁻ (superscript minus) to preserve the negative sign
        result = result.replace("⁻¹", "⁻");
        
        // Then remove standalone ¹
        result = result.replace("¹", "");
        
        result
    }

    /// Convert unresolved type to unit! macro format
    fn convert_to_unit_macro_format(&self, unresolved_type: &str) -> String {
        // This is a simplified conversion - could be enhanced with more sophisticated parsing
        if unresolved_type.contains("9223372036854775807") {
            // Replace unresolved parts with placeholders
            unresolved_type
                .replace("9223372036854775807", "?")
                .replace("Quantity<", "")
                .replace(">", "")
                .replace(", ", " * ")
                .replace(" * 0", "") // Remove zero exponents
                .replace(" * 1", "") // Remove unit exponents
                .replace(" * ?", "?") // Clean up unresolved placeholders
        } else {
            unresolved_type.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_whippyunits_type() {
        let processor = InlayHintProcessor::new();
        
        // Test with whippyunits type
        let label_with_quantity = vec![
            json!({"value": ": "}),
            json!({"value": "Quantity", "location": {"uri": "file://test.rs", "range": {"start": {"line": 1, "character": 0}, "end": {"line": 1, "character": 8}}}}),
            json!({"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"})
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
    fn test_convert_whippyunits_hint() {
        let processor = InlayHintProcessor::new();
        
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
            json!({"value": "<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>"})
        ];
        
        processor.convert_whippyunits_hint(&mut label_array).unwrap();
        
        // Should have 2 parts now (removed generic params)
        assert_eq!(label_array.len(), 2);
        
        // First part should still be ": "
        assert_eq!(label_array[0]["value"], ": ");
        
        // Second part should be pretty-printed and have location preserved
        let second_part = &label_array[1];
        assert!(second_part["value"].as_str().unwrap().contains("m"));
        assert!(second_part.get("location").is_some());
    }

    #[test]
    fn test_process_inlay_hint_response() {
        let processor = InlayHintProcessor::new();
        
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
                        {"value": "<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>"}
                    ],
                    "kind": 1,
                    "data": {"file_id": 0, "hash": "123", "resolve_range": {"start": {"line": 12, "character": 8}, "end": {"line": 12, "character": 17}}, "version": 1}
                }
            ]
        });
        
        let response_str = serde_json::to_string(&response).unwrap();
        let processed = processor.process_inlay_hint_response(&response_str).unwrap();
        
        // Should contain pretty-printed type
        assert!(processed.contains("m"));
        // Should preserve all metadata
        assert!(processed.contains("location"));
        assert!(processed.contains("resolve_range"));
        assert!(processed.contains("data"));
    }

    #[test]
    fn test_real_inlay_hint_transformation() {
        let processor = InlayHintProcessor::new();
        
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
                                "uri": "file:///Users/emichaelbarnettgmail.com/Developer/whippyunits/src/lib.rs",
                                "range": {
                                    "start": {"line": 64, "character": 11},
                                    "end": {"line": 64, "character": 19}
                                }
                            }
                        },
                        {"value": "<0, 9223372036854775807, 1, -1, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>"}
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
        let processed_json: Value = serde_json::from_str(&processed).unwrap();
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
        assert!(pretty_value.contains("m"));
        println!("Original: Quantity<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
        println!("Pretty: '{}'", pretty_value);
        
        // Should preserve the location for click-to-source
        assert!(second_part.get("location").is_some());
        let location = &second_part["location"];
        assert_eq!(location["uri"], "file:///Users/emichaelbarnettgmail.com/Developer/whippyunits/src/lib.rs");
        
        // Should preserve all other metadata
        assert!(hint.get("position").is_some());
        assert!(hint.get("kind").is_some());
        assert!(hint.get("data").is_some());
        assert!(hint["data"].get("resolve_range").is_some());
        
        println!("Original: Quantity<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
        println!("Pretty: '{}'", pretty_value);
        println!("Pretty value length: {}", pretty_value.len());
    }

    #[test]
    fn test_inlay_hint_exponent_pruning() {
        let processor = InlayHintProcessor::new();
        
        // Test that ^1 exponents are pruned but meaningful exponents are preserved
        let test_cases = vec![
            ("mm¹", "mm"),           // ^1 should be removed
            ("mm²", "mm²"),          // ^2 should be preserved
            ("mm³", "mm³"),          // ^3 should be preserved
            ("mm⁻¹", "mm⁻"),         // ^-1 becomes ^- (the 1 is removed)
            ("m¹s²", "ms²"),         // ^1 should be removed, ^2 preserved
            ("kg¹m²s⁻²", "kgm²s⁻²"), // ^1 should be removed, others preserved
        ];
        
        for (input, expected) in test_cases {
            let result = processor.prune_inlay_hint_exponents(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
