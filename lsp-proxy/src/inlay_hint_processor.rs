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
        // Get the label array from the object
        if let Some(label) = hint_obj.get_mut("label") {
            if let Some(label_array) = label.as_array_mut() {
                // Check if this hint contains a whippyunits type
                if self.contains_whippyunits_type(label_array) {
                    self.convert_whippyunits_hint(label_array)?;
                }
            }
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
            
            // Convert to pretty format using the display configuration
            let pretty_type = self.converter.convert_types_in_text_with_config(&full_type, &self.display_config);
            
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
            json!({"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"})
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
                        {"value": "<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>"}
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
                        {"value": "<1, -1, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 9223372036854775807>"}
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
}
