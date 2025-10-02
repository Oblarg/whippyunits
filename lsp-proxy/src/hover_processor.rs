use serde_json::Value;
use crate::{lsp_structures::{HoverContent, HoverContents}, unit_formatter::{UnitFormatter, DisplayConfig}};

/// Processor for hover content to improve whippyunits type display
#[derive(Clone)]
pub struct HoverProcessor {
    unit_formatter: UnitFormatter,
    display_config: DisplayConfig,
}

impl HoverProcessor {
    pub fn new(display_config: DisplayConfig) -> Self {
        Self {
            unit_formatter: UnitFormatter::new(),
            display_config,
        }
    }

    /// Extract hover content from LSP result
    pub fn extract_hover_content(&self, result: &Value) -> Option<HoverContent> {
        if let Ok(hover) = serde_json::from_value::<HoverContent>(result.clone()) {
            Some(hover)
        } else {
            None
        }
    }

    /// Improve hover content by formatting whippyunits types
    pub fn improve_hover_content(&self, mut hover: HoverContent) -> HoverContent {
        match &mut hover.contents {
            HoverContents::Single(item) => {
                let original_text = item.value.clone();
                // Apply transformations in sequence, but only if they haven't been applied already
                let mut processed_text = item.value.clone();
                
                
                // Only apply trait signature transformation if not already processed
                if !processed_text.contains("impl Add for") && !processed_text.contains("impl Sub for") && 
                   !processed_text.contains("impl Mul<") && !processed_text.contains("impl Div<") {
                    processed_text = self.transform_add_trait_signature(&processed_text);
                }
                
                // Always apply type conversion as the final step
                processed_text = self.unit_formatter.format_types_with_original(&processed_text, &self.display_config, &original_text);
                
                item.value = processed_text;
            }
            HoverContents::Multiple(items) => {
                for item in items {
                    let original_text = item.value.clone();
                    // Apply transformations in sequence, but only if they haven't been applied already
                    let mut processed_text = item.value.clone();
                    
                    
                    // Only apply trait signature transformation if not already processed
                    if !processed_text.contains("impl Add for") && !processed_text.contains("impl Sub for") && 
                       !processed_text.contains("impl Mul<") && !processed_text.contains("impl Div<") {
                        processed_text = self.transform_add_trait_signature(&processed_text);
                    }
                    
                    // Always apply type conversion as the final step
                    processed_text = self.unit_formatter.format_types_with_original(&processed_text, &self.display_config, &original_text);
                    
                    item.value = processed_text;
                }
            }
        }
        hover
    }


    /// Transform the Add/Sub/Mul/Div trait implementation signatures to be more readable
    fn transform_add_trait_signature(&self, text: &str) -> String {
        // Only process text that contains trait implementations
        if !text.contains("impl<") || (!text.contains("for Quantity<") && !text.contains("for f64")) {
            return text.to_string();
        }

        // Look for specific trait patterns and simplify them
        let mut result = text.to_string();
        
        // Pattern 1: impl<const PARAMS...> Add for Quantity<...>
        if result.contains("impl<") && result.contains("Add for") {
            result = self.simplify_trait_signature(&result, "Add", "Add for");
        }
        
        // Pattern 2: impl<const PARAMS...> Sub for Quantity<...>
        if result.contains("impl<") && result.contains("Sub for") {
            result = self.simplify_trait_signature(&result, "Sub", "Sub for");
        }
        
        // Pattern 3: impl<const PARAMS...> Mul<f64> for Quantity<...> (scalar multiplication)
        if result.contains("impl<") && result.contains("Mul<f64>") && result.contains("for Quantity<") {
            result = self.simplify_scalar_signature(&result, "Mul");
        }
        
        // Pattern 4: impl<const PARAMS...> Div<f64> for Quantity<...> (scalar division)
        if result.contains("impl<") && result.contains("Div<f64>") && result.contains("for Quantity<") {
            result = self.simplify_scalar_signature(&result, "Div");
        }
        
        // Pattern 5: impl<const PARAMS...> Mul for f64 (reverse scalar multiplication: f64 * Quantity)
        if result.contains("impl<") && result.contains("Mul for") && result.contains("for f64") {
            result = self.simplify_reverse_scalar_signature(&result, "Mul");
        }
        
        // Pattern 6: impl<const PARAMS...> Div for f64 (reverse scalar division: f64 / Quantity)
        if result.contains("impl<") && result.contains("Div for") && result.contains("for f64") {
            result = self.simplify_reverse_scalar_signature(&result, "Div");
        }
        
        // Pattern 7: impl<const PARAMS...> Mul for Quantity<...> (ambiguous case - check function signature)
        if result.contains("impl<") && result.contains("Mul for") && !result.contains("Mul<") && !result.contains("Mul<f64>") && !result.contains("for f64") {
            // Check if this is actually a Quantity-Quantity multiplication by looking at the function signature
            if result.contains("fn mul(self, other: Quantity<") {
                result = self.simplify_mul_div_signature(&result, "Mul");
            } else if result.contains("fn mul(self, other: f64)") {
                // This is a scalar multiplication
                result = self.simplify_scalar_signature(&result, "Mul");
            } else {
                // Default to quantity-quantity multiplication for impl<...> Mul for Quantity<...>
                result = self.simplify_mul_div_signature(&result, "Mul");
            }
        }
        
        // Pattern 8: impl<const PARAMS...> Div for Quantity<...> (ambiguous case - check function signature)
        if result.contains("impl<") && result.contains("Div for") && !result.contains("Div<") && !result.contains("Div<f64>") && !result.contains("for f64") {
            // Check if this is actually a Quantity-Quantity division by looking at the function signature
            if result.contains("fn div(self, other: Quantity<") {
                result = self.simplify_mul_div_signature(&result, "Div");
            } else if result.contains("fn div(self, other: f64)") {
                // This is a scalar division
                result = self.simplify_scalar_signature(&result, "Div");
            } else {
                // Default to quantity-quantity division for impl<...> Div for Quantity<...>
                result = self.simplify_mul_div_signature(&result, "Div");
            }
        }
        
        result
    }
    
    /// Generic helper to find where clause position and determine where to cut off the text
    fn find_where_clause_boundary(&self, search_text: &str) -> Option<usize> {
        // Look for "where " or "where\n" - these are the common patterns in hover text
        search_text.find("where ")
            .or_else(|| search_text.find("where\n"))
    }

    /// Helper function to simplify Add/Sub trait signatures
    fn simplify_trait_signature(&self, text: &str, trait_name: &str, trait_pattern: &str) -> String {
        if let Some(impl_start) = text.find("impl<") {
            if let Some(trait_start) = text[impl_start..].find(trait_pattern) {
                let trait_start = impl_start + trait_start;
                
                // Find the end of the trait type, stopping before any where clause or function body
                let search_text = &text[trait_start..];
                
                // Use the generic helper to find where clause boundary
                let where_pos = self.find_where_clause_boundary(search_text);
                
                let brace_pos = search_text.find('{');
                let newline_pos = search_text.find('\n');
                
                let type_end = trait_start + match (where_pos, brace_pos, newline_pos) {
                    (Some(w), Some(b), Some(n)) => w.min(b).min(n),
                    (Some(w), Some(b), None) => w.min(b),
                    (Some(w), None, Some(n)) => w.min(n),
                    (None, Some(b), Some(n)) => b.min(n),
                    (Some(w), None, None) => w,
                    (None, Some(b), None) => b,
                    (None, None, Some(n)) => n,
                    (None, None, None) => search_text.len(),
                };
                
                // Extract the type after the trait
                let trait_type = &text[trait_start + trait_pattern.len()..type_end];
                
                // Create simplified signature - just show the trait name and a placeholder for the type
                let simplified = format!("impl {} for Quantity<...>", trait_name);
                
                // If there's a where clause, don't include anything after the trait type
                // If no where clause, include everything after the type
                if where_pos.is_some() {
                    // Stop at the where clause - don't include anything after
                    return format!("{}{}", &text[..impl_start], simplified);
                } else {
                    // No where clause, include everything after the type
                    let after = &text[type_end..];
                    return format!("{}{}{}", &text[..impl_start], simplified, after);
                }
            }
        }
        text.to_string()
    }
    
    /// Helper function to simplify Mul/Div trait signatures with Quantity parameters
    fn simplify_mul_div_signature(&self, text: &str, trait_name: &str) -> String {
        if let Some(impl_start) = text.find("impl<") {
            // Handle the real case: impl<const PARAMS...> Mul for Quantity<...>
            if let Some(trait_start) = text[impl_start..].find(trait_name) {
                let trait_start = impl_start + trait_start;
                
                // Find "for Quantity<" after the trait name
                if let Some(for_start) = text[trait_start..].find("for Quantity<") {
                    let for_start = trait_start + for_start;
                    
                    // Find the end of the Quantity type
                    let search_text = &text[for_start..];
                    
                    // Use the generic helper to find where clause boundary
                    let where_pos = self.find_where_clause_boundary(search_text);
                    
                    let brace_pos = search_text.find('{');
                    let newline_pos = search_text.find('\n');
                    
                    let type_end = for_start + match (where_pos, brace_pos, newline_pos) {
                        (Some(w), Some(b), Some(n)) => w.min(b).min(n),
                        (Some(w), Some(b), None) => w.min(b),
                        (Some(w), None, Some(n)) => w.min(n),
                        (None, Some(b), Some(n)) => b.min(n),
                        (Some(w), None, None) => w,
                        (None, Some(b), None) => b,
                        (None, None, Some(n)) => n,
                        (None, None, None) => search_text.len(),
                    };
                    
                    let trait_type = &text[for_start + "for ".len()..type_end];
                    
                    // For quantity-quantity operations, show the actual types
                    let simplified = format!("impl {} for {}", trait_name, trait_type);
                    
                    // Strip the where clause but keep the function definition
                    if where_pos.is_some() {
                        // Include everything from the trait type to the where clause (which includes the function)
                        let where_start = for_start + where_pos.unwrap();
                        let after = &text[type_end..where_start];
                        return format!("{}{}{}", &text[..impl_start], simplified, after);
                    } else {
                        // No where clause, include everything after the type
                        let after = &text[type_end..];
                        return format!("{}{}{}", &text[..impl_start], simplified, after);
                    }
                }
            }
        }
        text.to_string()
    }
    
    /// Helper function to simplify scalar Mul/Div trait signatures
    fn simplify_scalar_signature(&self, text: &str, trait_name: &str) -> String {
        if let Some(impl_start) = text.find("impl<") {
            // Look for both patterns: "Mul<f64> for" and "Mul for"
            let trait_pattern1 = format!("{}<f64> for", trait_name);
            let trait_pattern2 = format!("{} for", trait_name);
            
            let trait_start = if let Some(start) = text[impl_start..].find(&trait_pattern1) {
                Some(impl_start + start)
            } else if let Some(start) = text[impl_start..].find(&trait_pattern2) {
                Some(impl_start + start)
            } else {
                None
            };
            
            if let Some(trait_start) = trait_start {
                // Find the end of the trait type
                let search_text = &text[trait_start..];
                
                // Use the generic helper to find where clause boundary
                let where_pos = self.find_where_clause_boundary(search_text);
                
                let brace_pos = search_text.find('{');
                let newline_pos = search_text.find('\n');
                
                let type_end = trait_start + match (where_pos, brace_pos, newline_pos) {
                    (Some(w), Some(b), Some(n)) => w.min(b).min(n),
                    (Some(w), Some(b), None) => w.min(b),
                    (Some(w), None, Some(n)) => w.min(n),
                    (None, Some(b), Some(n)) => b.min(n),
                    (Some(w), None, None) => w,
                    (None, Some(b), None) => b,
                    (None, None, Some(n)) => n,
                    (None, None, None) => search_text.len(),
                };
                
                // Extract the type after the trait
                // Handle both "Mul<f64> for" and "Mul for" patterns
                let trait_type = if text[trait_start..].starts_with(&trait_pattern1) {
                    &text[trait_start + trait_pattern1.len() + 1..type_end] // +1 for space
                } else {
                    &text[trait_start + trait_pattern2.len() + 1..type_end] // +1 for space
                };
                
                // Create simplified signature for scalar operations
                let simplified = format!("impl {}<f64> for {}", trait_name, trait_type);
                
                // If there's a where clause, don't include anything after the trait type
                // If no where clause, include everything after the type
                if where_pos.is_some() {
                    // Stop at the where clause - don't include anything after
                    return format!("{}{}", &text[..impl_start], simplified);
                } else {
                    // No where clause, include everything after the type
                    let after = &text[type_end..];
                    return format!("{}{}{}", &text[..impl_start], simplified, after);
                }
            }
        }
        text.to_string()
    }
    
    /// Helper function to simplify reverse scalar Mul/Div trait signatures (f64 * Quantity)
    fn simplify_reverse_scalar_signature(&self, text: &str, trait_name: &str) -> String {
        if let Some(impl_start) = text.find("impl<") {
            let trait_pattern = format!("{} for", trait_name);
            
            if let Some(trait_start) = text[impl_start..].find(&trait_pattern) {
                let trait_start = impl_start + trait_start;
                
                // Find the end of the trait type
                let search_text = &text[trait_start..];
                
                // Use the generic helper to find where clause boundary
                let where_pos = self.find_where_clause_boundary(search_text);
                
                let brace_pos = search_text.find('{');
                let newline_pos = search_text.find('\n');
                
                let type_end = trait_start + match (where_pos, brace_pos, newline_pos) {
                    (Some(w), Some(b), Some(n)) => w.min(b).min(n),
                    (Some(w), Some(b), None) => w.min(b),
                    (Some(w), None, Some(n)) => w.min(n),
                    (None, Some(b), Some(n)) => b.min(n),
                    (Some(w), None, None) => w,
                    (None, Some(b), None) => b,
                    (None, None, Some(n)) => n,
                    (None, None, None) => search_text.len(),
                };
                
                // Extract the type after the trait
                let trait_type = &text[trait_start + trait_pattern.len() + 1..type_end]; // +1 for space
                
                // Create simplified signature for reverse scalar operations
                let simplified = format!("impl {}<Quantity<...>> for {}", trait_name, trait_type);
                
                // Strip the where clause but keep the function definition
                if where_pos.is_some() {
                    // Include everything from the trait type to the where clause (which includes the function)
                    let where_start = trait_start + where_pos.unwrap();
                    let after = &text[type_end..where_start];
                    return format!("{}{}{}", &text[..impl_start], simplified, after);
                } else {
                    // No where clause, include everything after the type
                    let after = &text[type_end..];
                    return format!("{}{}{}", &text[..impl_start], simplified, after);
                }
            }
        }
        text.to_string()
    }
}
