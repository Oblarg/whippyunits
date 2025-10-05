use serde_json::{json, Value};
use crate::{unit_formatter::{UnitFormatter, DisplayConfig}, quantity_detection};
use anyhow::Result;

/// Process inlay hint responses to pretty-print whippyunits types
#[derive(Clone)]
pub struct InlayHintProcessor {
    formatter: UnitFormatter,
    display_config: DisplayConfig,
}

impl InlayHintProcessor {
    pub fn new() -> Self {
        Self {
            formatter: UnitFormatter::new(),
            display_config: DisplayConfig::default(),
        }
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            formatter: UnitFormatter::new(),
            display_config,
        }
    }

    /// Process an inlay hint response, converting whippyunits types to pretty format
    pub fn process_inlay_hint_response(&self, message: &str) -> Result<String> {
        
        // Fast string search to detect if this message contains Quantity types
        if !self.contains_quantity_types_fast(message) {
            // No Quantity types detected, return original message unchanged
            return Ok(message.to_string());
        }
        
        
        // Parse the JSON message only if we detected Quantity types
        let mut json_value: Value = serde_json::from_str(message)?;
        
        // Check if this is an inlay hint response with results
        if let Some(result) = json_value.get_mut("result") {
            
            // Handle both array (inlay hint requests) and object (resolve responses)
            if let Some(results_array) = result.as_array_mut() {
                // Process each inlay hint in the results array
                for (i, hint) in results_array.iter_mut().enumerate() {
                    self.process_single_hint(hint)?;
                }
            } else if let Some(single_hint) = result.as_object_mut() {
                // Process a single inlay hint object (resolve response)
                self.process_single_hint_object(single_hint)?;
            }
        } else {
        }
        
        // Convert back to string
        Ok(serde_json::to_string(&json_value)?)
    }

    /// Process a single inlay hint, converting whippyunits types if present
    fn process_single_hint(&self, hint: &mut Value) -> Result<()> {
        
        // Get the label array
        if let Some(label) = hint.get_mut("label") {
            if let Some(label_array) = label.as_array_mut() {
                
                // Log the original label content
                let original_label: Vec<String> = label_array.iter()
                    .filter_map(|part| part.get("value").and_then(|v| v.as_str()).map(|s| s.to_string()))
                    .collect();
                
                // Check if this hint contains a whippyunits type
                if self.contains_whippyunits_type(label_array) {
                    self.convert_whippyunits_hint(label_array)?;
                    
                    // Log the converted label content
                    let converted_label: Vec<String> = label_array.iter()
                        .filter_map(|part| part.get("value").and_then(|v| v.as_str()).map(|s| s.to_string()))
                        .collect();
                } else {
                }
            } else {
            }
        } else {
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

    /// Fast string search to detect Quantity types without deserialization
    /// This performs a performant string search for "Quantity<" patterns
    fn contains_quantity_types_fast(&self, json_payload: &str) -> bool {
        quantity_detection::contains_quantity_types_fast(json_payload)
    }

    /// Check if a label array contains a whippyunits type
    pub fn contains_whippyunits_type(&self, label_array: &[Value]) -> bool {
        quantity_detection::contains_whippyunits_type(label_array)
    }

    /// Convert a whippyunits inlay hint to pretty format
    pub fn convert_whippyunits_hint(&self, label_array: &mut Vec<Value>) -> Result<()> {
        
        // Find the Quantity part
        let mut quantity_part_index = None;
        for (i, part) in label_array.iter().enumerate() {
            if let Some(value) = part.get("value") {
                if let Some(text) = value.as_str() {
                    if text == "Quantity" {
                        quantity_part_index = Some(i);
                        break;
                    }
                }
            }
        }
        
        // If we found the Quantity part, collect all the generic parameters
        if let Some(quantity_idx) = quantity_part_index {
            
            // Collect all parts after Quantity that form the generic parameters
            let mut generic_parts = Vec::new();
            let mut bracket_depth = 0;
            let mut found_opening_bracket = false;
            
            for i in (quantity_idx + 1)..label_array.len() {
                if let Some(value) = label_array[i].get("value") {
                    if let Some(text) = value.as_str() {
                        // Track bracket depth to know when we've reached the end
                        for ch in text.chars() {
                            if ch == '<' {
                                bracket_depth += 1;
                                found_opening_bracket = true;
                            } else if ch == '>' {
                                bracket_depth -= 1;
                            }
                        }
                        
                        generic_parts.push(text);
                        
                        // Stop when we've closed all brackets
                        if found_opening_bracket && bracket_depth == 0 {
                            break;
                        }
                    }
                }
            }
            
            if !generic_parts.is_empty() {
                // Create the complete generic parameters string
                let generic_params = generic_parts.join("");
                
                // Construct the full type string
                let full_type = format!("Quantity{}", generic_params);
                
                // For inlay hints, use the full Quantity<unit, type> format
                let pretty_type = self.formatter.format_types_inlay_hint(&full_type);
                
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
                
                // Remove all the generic parameters parts
                let mut parts_to_remove = Vec::new();
                let mut bracket_depth = 0;
                let mut found_opening_bracket = false;
                
                for i in (quantity_idx + 1)..label_array.len() {
                    if let Some(value) = label_array[i].get("value") {
                        if let Some(text) = value.as_str() {
                            // Track bracket depth to know when we've reached the end
                            for ch in text.chars() {
                                if ch == '<' {
                                    bracket_depth += 1;
                                    found_opening_bracket = true;
                                } else if ch == '>' {
                                    bracket_depth -= 1;
                                }
                            }
                            
                            parts_to_remove.push(i);
                            
                            // Stop when we've closed all brackets
                            if found_opening_bracket && bracket_depth == 0 {
                                break;
                            }
                        }
                    }
                }
                
                // Remove parts in reverse order to maintain indices
                for &i in parts_to_remove.iter().rev() {
                    label_array.remove(i);
                }
                
            } else {
            }
        } else {
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


        // Extract the datatype suffix if present
        let (unit_part, datatype) = self.extract_unit_and_datatype(&pretty_type);
        
        // Generate the appropriate unit! macro format based on datatype
        let unit_macro = if datatype == "f64" {
            // For f64, use the simple format: unit!(mm)
            format!("unit!({})", unit_part)
        } else {
            // For other datatypes, use the type-specified format: unit!(mm, i32)
            format!("unit!({}, {})", unit_part, datatype)
        };
        
        
        // Return just the type annotation with the unit! macro
        Ok(format!(": {}", unit_macro))
    }

    /// Extract unit part and datatype from a pretty-printed type with suffix
    fn extract_unit_and_datatype(&self, pretty_type: &str) -> (String, String) {
        
        // Remove the "Unresolved type - " prefix if present
        let clean_type = if pretty_type.starts_with("Unresolved type - ") {
            &pretty_type[18..] // Skip "Unresolved type - "
        } else {
            pretty_type
        };
        
        
        // Check for new Quantity<unit, datatype> format
        if clean_type.starts_with("Quantity<") && clean_type.ends_with('>') {
            let inner = &clean_type[9..clean_type.len()-1]; // Remove "Quantity<" and ">"
            
            if let Some(comma_pos) = inner.rfind(',') {
                let unit_part = self.convert_pretty_type_to_unit_macro(inner[..comma_pos].trim());
                let datatype = inner[comma_pos + 1..].trim().to_string();
                return (unit_part, datatype);
            } else {
                // No comma found, this might be a malformed Quantity type
                // Try to extract just the unit part without datatype
                let unit_part = self.convert_pretty_type_to_unit_macro(inner.trim());
                return (unit_part, "f64".to_string());
            }
        }
        
        // Check for old backing datatype suffix format (fallback for compatibility)
        if let Some(underscore_pos) = clean_type.rfind('_') {
            let suffix = &clean_type[underscore_pos + 1..];
            if suffix == "f64" || suffix == "f32" || suffix == "i64" || suffix == "i32" || 
               suffix == "i16" || suffix == "i8" || suffix == "u64" || suffix == "u32" || 
               suffix == "u16" || suffix == "u8" || suffix == "isize" || suffix == "usize" {
                let unit_part = self.convert_pretty_type_to_unit_macro(&clean_type[..underscore_pos]);
                return (unit_part, suffix.to_string());
            }
        }
        
        // No datatype suffix found, treat as f64 (default)
        let unit_part = self.convert_pretty_type_to_unit_macro(clean_type);
        (unit_part, "f64".to_string())
    }

    /// Convert pretty-printed type to unit! macro format
    fn convert_pretty_type_to_unit_macro(&self, pretty_type: &str) -> String {
        // Convert pretty-printed types like "mmˀ" to unit macro format like "mm^"
        // Remove the ? and place cursor after the caret
        pretty_type
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
    pub fn prune_inlay_hint_exponents(&self, pretty_type: &str) -> String {
        // In our pretty-printed output, we use Unicode superscripts like ¹, ², ³, etc.
        // We want to remove ¹ (superscript 1) but keep all other superscripts
        // For negative exponents, we want to preserve the full -1 to make it clear
        let mut result = pretty_type.to_string();
        
        // Remove standalone ¹ (but preserve ⁻¹ as it represents a meaningful negative exponent)
        // Since ⁻¹ is two separate Unicode characters (⁻ + ¹), we need to handle this carefully
        // First, temporarily replace ⁻¹ with a placeholder to protect it
        result = result.replace("⁻¹", "PLACEHOLDER_MINUS_ONE");
        
        // Then remove standalone ¹
        result = result.replace("¹", "");
        
        // Finally, restore the ⁻¹
        result = result.replace("PLACEHOLDER_MINUS_ONE", "⁻¹");
        
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

