use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use regex::Regex;


pub mod inlay_hint_processor;



/// Display configuration for whippyunits type formatting
#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub verbose: bool,
    pub unicode: bool,
    pub include_raw: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            unicode: true,
            include_raw: false,
        }
    }
}

/// LSP Message structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LspMessage {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// Hover content structure
#[derive(Debug, Serialize, Deserialize)]
pub struct HoverContent {
    pub contents: HoverContents,
    pub range: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    Single(HoverContentItem),
    Multiple(Vec<HoverContentItem>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverContentItem {
    pub language: Option<String>,
    pub value: String,
    pub kind: Option<String>,
}

/// LSP Proxy that intercepts and modifies hover responses
#[derive(Clone)]
pub struct LspProxy {
    type_converter: WhippyUnitsTypeConverter,
    display_config: DisplayConfig,
    inlay_hint_processor: inlay_hint_processor::InlayHintProcessor,
}

impl LspProxy {
    pub fn new() -> Self {
        let display_config = DisplayConfig::default();
        // Create a non-verbose config for inlay hints
        let inlay_hint_config = DisplayConfig {
            verbose: false,
            unicode: true,
            include_raw: false,
        };
        Self {
            type_converter: WhippyUnitsTypeConverter::new(),
            display_config: display_config.clone(),
            inlay_hint_processor: inlay_hint_processor::InlayHintProcessor::with_config(inlay_hint_config),
        }
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        // Create a non-verbose config for inlay hints
        let inlay_hint_config = DisplayConfig {
            verbose: false,
            unicode: display_config.unicode,
            include_raw: false,
        };
        Self {
            type_converter: WhippyUnitsTypeConverter::new(),
            display_config: display_config.clone(),
            inlay_hint_processor: inlay_hint_processor::InlayHintProcessor::with_config(inlay_hint_config),
        }
    }

    /// Process an incoming LSP message (from rust-analyzer to editor)
    /// This expects a complete LSP message with Content-Length header
    pub fn process_incoming(&self, message: &str) -> Result<String, anyhow::Error> {
        // Parse the LSP message format
        let json_payload = self.extract_json_payload(message)?;
        
        // Parse the JSON payload
        let mut lsp_msg: LspMessage = serde_json::from_str(&json_payload)?;
        

        
        // Check if this is a hover response
        if let Some(result) = &lsp_msg.result {
            if let Some(hover_content) = self.extract_hover_content(result) {
                let improved_content = self.improve_hover_content(hover_content);
                lsp_msg.result = Some(serde_json::to_value(improved_content)?);
            }
        }
        

        
        // Check if this is a refresh notification
        if self.is_refresh_notification(&lsp_msg) {
            eprintln!("*** INTERCEPTING REFRESH NOTIFICATION ***");
            // Pass through refresh notifications unchanged - they're notifications, not requests
            // The client should respond to this by re-requesting inlay hints
        }
        
        // Check if this is a resolve request
        if self.is_resolve_request(&lsp_msg) {
            eprintln!("*** INTERCEPTING RESOLVE REQUEST ***");
            // Pass through resolve requests unchanged - we'll intercept the response
        }
        
        // Check if this is an inlay hint response (including resolve responses)
        if let Some(result) = &lsp_msg.result {
            if self.is_inlay_hint_response(&lsp_msg) {
                eprintln!("*** INTERCEPTING INLAY HINT RESPONSE ***");
                let improved_result = self.process_inlay_hint_result(result)?;
                lsp_msg.result = Some(improved_result);
            }
        }
        
        // Reconstruct the LSP message format
        let new_json = serde_json::to_string(&lsp_msg)?;
        let content_length = new_json.len();
        Ok(format!("Content-Length: {}\r\n\r\n{}", content_length, new_json))
    }

    /// Process an outgoing LSP message (from editor to rust-analyzer)
    /// This expects a complete LSP message with Content-Length header
    pub fn process_outgoing(&self, message: &str) -> Result<String, anyhow::Error> {
        // Parse the LSP message format
        let json_payload = self.extract_json_payload(message)?;
        
        // Parse the JSON payload
        let lsp_msg: LspMessage = serde_json::from_str(&json_payload)?;
        
        // Check if this is a refresh notification (from client to server)
        if self.is_refresh_notification(&lsp_msg) {
            eprintln!("*** INTERCEPTING OUTGOING REFRESH NOTIFICATION ***");
            // Pass through refresh notifications unchanged
        }
        
        // Check if this is a resolve request (from client to server)
        if self.is_resolve_request(&lsp_msg) {
            eprintln!("*** INTERCEPTING OUTGOING RESOLVE REQUEST ***");
            // Pass through resolve requests unchanged
        }
        
        // For now, just pass through outgoing messages unchanged
        // We could add logging or other processing here
        Ok(message.to_string())
    }

    /// Extract JSON payload from LSP message format
    fn extract_json_payload(&self, message: &str) -> Result<String, anyhow::Error> {
        let lines: Vec<&str> = message.lines().collect();
        
        // Find the empty line that separates headers from JSON
        let mut json_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                json_start = i + 1;
                break;
            }
        }
        
        if json_start >= lines.len() {
            return Err(anyhow::anyhow!("No JSON payload found in LSP message"));
        }
        
        // Join the remaining lines as JSON
        Ok(lines[json_start..].join("\n"))
    }

    fn extract_hover_content(&self, result: &Value) -> Option<HoverContent> {
        if let Ok(hover) = serde_json::from_value::<HoverContent>(result.clone()) {
            Some(hover)
        } else {
            None
        }
    }

    fn improve_hover_content(&self, mut hover: HoverContent) -> HoverContent {
        match &mut hover.contents {
            HoverContents::Single(item) => {
                let original_text = item.value.clone();
                // Apply transformations in sequence, but only if they haven't been applied already
                let mut processed_text = item.value.clone();
                
                // Only apply rescale transformation if not already processed
                if !processed_text.contains("rescale(") {
                    processed_text = self.transform_rescale_signature(&processed_text);
                }
                
                // Only apply trait signature transformation if not already processed
                if !processed_text.contains("impl Add for") && !processed_text.contains("impl Sub for") && 
                   !processed_text.contains("impl Mul<") && !processed_text.contains("impl Div<") {
                    processed_text = self.transform_add_trait_signature(&processed_text);
                }
                
                // Always apply type conversion as the final step
                processed_text = self.type_converter.convert_types_in_text_with_config_and_original(&processed_text, &self.display_config, &original_text);
                
                item.value = processed_text;
            }
            HoverContents::Multiple(items) => {
                for item in items {
                    let original_text = item.value.clone();
                    // Apply transformations in sequence, but only if they haven't been applied already
                    let mut processed_text = item.value.clone();
                    
                    // Only apply rescale transformation if not already processed
                    if !processed_text.contains("rescale(") {
                        processed_text = self.transform_rescale_signature(&processed_text);
                    }
                    
                    // Only apply trait signature transformation if not already processed
                    if !processed_text.contains("impl Add for") && !processed_text.contains("impl Sub for") && 
                       !processed_text.contains("impl Mul<") && !processed_text.contains("impl Div<") {
                        processed_text = self.transform_add_trait_signature(&processed_text);
                    }
                    
                    // Always apply type conversion as the final step
                    processed_text = self.type_converter.convert_types_in_text_with_config_and_original(&processed_text, &self.display_config, &original_text);
                    
                    item.value = processed_text;
                }
            }
        }
        hover
    }

    /// Transform the rescale function signature to be more readable
    fn transform_rescale_signature(&self, text: &str) -> String {
        // Look for the rescale function signature pattern
        // The actual signature has const generic parameters and complex Quantity types
        // Pattern: pub fn rescale<const PARAMS...>(quantity: Quantity<...>) -> Quantity<...>
        // We need a more robust regex that can handle the complex generic parameters
        
        // The issue with the old regex is that [^>]+ stops at the first >, but we have nested
        // Quantity<...> types that contain > characters. We need to handle this differently.
        
        // First, let's try to find the rescale function and extract the parts we need
        if text.contains("pub fn rescale<") {
            // We need to find where the generic parameters end and replace the entire signature
            // Look for the pattern: pub fn rescale<...>(quantity: ...) -> ...
            // We need to find the first > that's not part of a nested type
            
            let mut depth = 0;
            let mut generic_end = None;
            
            // Start from the opening < after "pub fn rescale<"
            let search_start = text.find("pub fn rescale<").unwrap() + "pub fn rescale<".len();
            
            for (i, ch) in text[search_start..].char_indices() {
                match ch {
                    '<' => depth += 1,
                    '>' => {
                        if depth == 0 {
                            generic_end = Some(search_start + i);
                            break;
                        }
                        depth -= 1;
                    }
                    _ => {}
                }
            }
            
            if let Some(generic_end) = generic_end {
                // Find the opening parenthesis
                if let Some(param_start) = text[generic_end..].find('(') {
                    let param_start = generic_end + param_start;
                    
                    // Find the closing parenthesis
                    if let Some(param_end) = text[param_start..].find(')') {
                        let param_end = param_start + param_end;
                        
                        // Find the return type arrow
                        if let Some(arrow_start) = text[param_end..].find(" -> ") {
                            let arrow_start = param_end + arrow_start;
                            
                            // Find the end of the return type (before the next { or newline)
                            let return_end = text[arrow_start..].find('{').unwrap_or_else(|| {
                                text[arrow_start..].find('\n').unwrap_or(text.len() - arrow_start)
                            });
                            let return_end = arrow_start + return_end;
                            
                            // Extract the actual parameter and return types (the parts that were pretty-printed)
                            // The parameter type is in "quantity: Quantity<...>", not in the generic params
                            let param_type_start = text[param_start..].find("quantity: ").unwrap_or(0) + param_start + "quantity: ".len();
                            let param_type_end = param_end;
                            let param_type = &text[param_type_start..param_type_end];
                            
                            let return_type = &text[arrow_start + 4..return_end];
                            
                            // Now construct the simplified signature that keeps the meaningful type info
                            let before = &text[..text.find("pub fn rescale<").unwrap()];
                            let after = &text[return_end..];
                            let simplified = format!("pub fn rescale<const FROM: {}, const TO: {}>(quantity: FROM) -> TO", param_type, return_type);
                            
                            return format!("{}{}{}", before, simplified, after);
                        }
                    }
                }
            }
        }
        
        text.to_string()
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
            eprintln!("DEBUG: Pattern 3 matched - scalar Mul<f64>");
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
            eprintln!("DEBUG: Pattern 7 matched - ambiguous Mul for");
            // Check if this is actually a Quantity-Quantity multiplication by looking at the function signature
            if result.contains("fn mul(self, other: Quantity<") {
                eprintln!("DEBUG: Pattern 7 - Quantity-Quantity case");
                result = self.simplify_mul_div_signature(&result, "Mul");
            } else if result.contains("fn mul(self, other: f64)") {
                eprintln!("DEBUG: Pattern 7 - scalar case");
                // This is a scalar multiplication
                result = self.simplify_scalar_signature(&result, "Mul");
            } else {
                eprintln!("DEBUG: Pattern 7 - default case");
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
                
                // Create simplified signature
                let simplified = format!("impl {} for {}", trait_name, trait_type);
                
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

    /// Extract a quantity type from generic parameters based on the scale suffix
    fn extract_quantity_type(&self, generic_params: &str, scale_suffix: &str) -> String {
        // Parse the generic parameters to extract values
        let mut mass_exp = 0;
        let mut mass_scale = 0;
        let mut length_exp = 0;
        let mut length_scale = 0;
        let mut time_exp = 0;
        let mut time_p2 = 0;
        let mut time_p3 = 0;
        let mut time_p5 = 0;
        
        // Extract MASS_EXPONENT
        if let Some(cap) = Regex::new(r"const MASS_EXPONENT: i8 = (\d+)").unwrap().captures(generic_params) {
            mass_exp = cap[1].parse().unwrap_or(0);
        }
        
        // Extract MASS_SCALE_P10 with the specified suffix
        let mass_scale_pattern = format!(r"const MASS_SCALE_P10_{}: i8 = (-?\d+)", scale_suffix);
        if let Some(cap) = Regex::new(&mass_scale_pattern).unwrap().captures(generic_params) {
            mass_scale = cap[1].parse().unwrap_or(0);
        }
        
        // Extract LENGTH_EXPONENT
        if let Some(cap) = Regex::new(r"const LENGTH_EXPONENT: i8 = (\d+)").unwrap().captures(generic_params) {
            length_exp = cap[1].parse().unwrap_or(0);
        }
        
        // Extract LENGTH_SCALE_P10 with the specified suffix
        let length_scale_pattern = format!(r"const LENGTH_SCALE_P10_{}: i8 = (-?\d+)", scale_suffix);
        if let Some(cap) = Regex::new(&length_scale_pattern).unwrap().captures(generic_params) {
            length_scale = cap[1].parse().unwrap_or(0);
        }
        
        // Extract TIME_EXPONENT
        if let Some(cap) = Regex::new(r"const TIME_EXPONENT: i8 = (\d+)").unwrap().captures(generic_params) {
            time_exp = cap[1].parse().unwrap_or(0);
        }
        
        // Extract TIME_SCALE_P2 with the specified suffix
        let time_p2_pattern = format!(r"const TIME_SCALE_P2_{}: i8 = (-?\d+)", scale_suffix);
        if let Some(cap) = Regex::new(&time_p2_pattern).unwrap().captures(generic_params) {
            time_p2 = cap[1].parse().unwrap_or(0);
        }
        
        // Extract TIME_SCALE_P3 with the specified suffix
        let time_p3_pattern = format!(r"const TIME_SCALE_P3_{}: i8 = (-?\d+)", scale_suffix);
        if let Some(cap) = Regex::new(&time_p3_pattern).unwrap().captures(generic_params) {
            time_p3 = cap[1].parse().unwrap_or(0);
        }
        
        // Extract TIME_SCALE_P5 with the specified suffix
        let time_p5_pattern = format!(r"const TIME_SCALE_P5_{}: i8 = (-?\d+)", scale_suffix);
        if let Some(cap) = Regex::new(&time_p5_pattern).unwrap().captures(generic_params) {
            time_p5 = cap[1].parse().unwrap_or(0);
        }
        
        // Use the prettyprint API to generate a readable type
        use whippyunits::print::prettyprint::pretty_print_quantity_type;
        pretty_print_quantity_type(
            mass_exp, mass_scale,
            length_exp, length_scale,
            time_exp, time_p2, time_p3, time_p5,
            false, // not verbose
        )
    }



    /// Check if this is an inlay hint response (has result with inlay hint data)
    fn is_inlay_hint_response(&self, lsp_msg: &LspMessage) -> bool {
        // Check if the result contains inlay hint data structure
        if let Some(result) = &lsp_msg.result {
            // Check if result is an array (typical for inlay hint requests)
            if result.is_array() {
                // Check if any item in the array has inlay hint structure
                if let Some(array) = result.as_array() {
                    for item in array {
                        if let Some(item_obj) = item.as_object() {
                            if item_obj.contains_key("position") && item_obj.contains_key("label") {
                                return true;
                            }
                        }
                    }
                }
            }
            
            // Check if result is an object (typical for inlay hint resolve responses)
            if result.is_object() {
                if let Some(obj) = result.as_object() {
                    if obj.contains_key("position") && obj.contains_key("label") {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    /// Check if this is a refresh notification
    fn is_refresh_notification(&self, lsp_msg: &LspMessage) -> bool {
        if let Some(method) = &lsp_msg.method {
            method == "workspace/inlayHint/refresh"
        } else {
            false
        }
    }

    /// Check if this is a resolve request
    fn is_resolve_request(&self, lsp_msg: &LspMessage) -> bool {
        if let Some(method) = &lsp_msg.method {
            method == "inlayHint/resolve"
        } else {
            false
        }
    }





    /// Process inlay hint result to pretty-print whippyunits types
    fn process_inlay_hint_result(&self, result: &Value) -> Result<Value, anyhow::Error> {
        // Create a full message structure for the inlay hint processor
        let full_message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": result
        });
        
        // Convert to string for processing
        let message_str = serde_json::to_string(&full_message)?;
        
        // Process the inlay hint response using our instance processor
        let processed_str = self.inlay_hint_processor.process_inlay_hint_response(&message_str)?;
        
        // Parse back to Value
        let processed_value: Value = serde_json::from_str(&processed_str)?;
        
        // Extract just the result part (remove the jsonrpc wrapper)
        if let Some(processed_result) = processed_value.get("result") {
            Ok(processed_result.clone())
        } else {
            // If no result field, return the original
            Ok(result.clone())
        }
    }
}

/// Type converter for whippyunits types using the new prettyprint API
#[derive(Clone)]
pub struct WhippyUnitsTypeConverter;

impl WhippyUnitsTypeConverter {
    pub fn new() -> Self {
        Self
    }

    /// Convert types in text with display configuration
    pub fn convert_types_in_text_with_config(&self, text: &str, config: &DisplayConfig) -> String {
        use whippyunits::print::prettyprint::pretty_print_quantity_type;
        
        let mut result = if config.verbose {
            // In verbose mode, convert Quantity types to readable format
            self.convert_quantity_types_verbose(text)
        } else {
            // In clean mode, convert to unit display
            self.convert_quantity_types_clean(text, config.unicode)
        };
        
        // Add raw type if requested
        // Only add raw type if requested AND if we actually made changes
        if config.include_raw && result != text {
            result.push_str(&format!("\n\nRaw: {}", text));
        }
        
        result
    }

    /// Convert types in text with display configuration and original text for Raw section
    pub fn convert_types_in_text_with_config_and_original(&self, text: &str, config: &DisplayConfig, original_text: &str) -> String {
        use whippyunits::print::prettyprint::pretty_print_quantity_type;
        
        let mut result = if config.verbose {
            // In verbose mode, convert Quantity types to readable format
            self.convert_quantity_types_verbose(text)
        } else {
            // In clean mode, convert to unit display
            self.convert_quantity_types_clean(text, config.unicode)
        };
        
        // Only add raw type if requested AND if we actually made changes
        if config.include_raw && result != original_text {
            result.push_str(&format!("\n\nRaw: {}", original_text));
        }
        
        result
    }

    /// Convert types in text (legacy method for backward compatibility)
    pub fn convert_types_in_text(&self, text: &str) -> String {
        self.convert_types_in_text_with_config(text, &DisplayConfig::default())
    }

    /// Convert types in text with verbose mode
    pub fn convert_types_in_text_verbose(&self, text: &str) -> String {
        let config = DisplayConfig {
            verbose: true,
            unicode: true,
            include_raw: false,
        };
        self.convert_types_in_text_with_config(text, &config)
    }

    /// Convert Quantity types to verbose const generic display
    fn convert_quantity_types_verbose(&self, text: &str) -> String {
        use whippyunits::print::prettyprint::{pretty_print_quantity_type, generate_dimension_symbols};
        use whippyunits::print::name_lookup::{lookup_dimension_name, generate_systematic_unit_name};
        
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i8")
            // Also check if we're in a context that suggests const generic parameters (like rescale functions)
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i8") || 
                                         text.contains("pub fn rescale<") ||
                                         text.contains("const FROM:") ||
                                         text.contains("const TO:") ||
                                         text.contains("impl Add for") ||
                                         text.contains("impl Sub for") ||
                                         text.contains("impl Mul<") ||
                                         text.contains("impl Div<") ||
                                         text.contains("impl Mul for") ||
                                         text.contains("impl Div for");
            
            let params = if is_const_generic_context {
                // This is a type definition or const generic context, treat as unknown (all i8::MIN placeholders)
                Some(QuantityParams {
                    mass_exp: i8::MIN,
                    mass_scale: i8::MIN,
                    length_exp: i8::MIN,
                    length_scale: i8::MIN,
                    time_exp: i8::MIN,
                    time_p2: i8::MIN,
                    time_p3: i8::MIN,
                    time_p5: i8::MIN,
                    generic_type: "f64".to_string(),
                })
            } else {
                self.parse_quantity_params(&full_match)
            };
            
            if let Some(params) = params {
                // Check if this is a partially resolved type
                if self.is_partially_resolved_type(&full_match) {
                    // For partially resolved types, use the full pretty-printed format
                    // but with unresolved scale indicators
                    pretty_print_quantity_type(
                        params.mass_exp, params.mass_scale,
                        params.length_exp, params.length_scale,
                        params.time_exp, params.time_p2, params.time_p3, params.time_p5,
                        true, // verbose
                    )
                } else {
                    // Use the new prettyprint API with verbose=true
                    pretty_print_quantity_type(
                        params.mass_exp, params.mass_scale,
                        params.length_exp, params.length_scale,
                        params.time_exp, params.time_p2, params.time_p3, params.time_p5,
                        true, // verbose
                    )
                }
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Convert Quantity types to clean unit display
    fn convert_quantity_types_clean(&self, text: &str, unicode: bool) -> String {
        use whippyunits::print::prettyprint::{pretty_print_quantity_type, generate_dimension_symbols};
        use whippyunits::print::name_lookup::{lookup_dimension_name, generate_systematic_unit_name};
        
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i8")
            // Also check if we're in a context that suggests const generic parameters (like rescale functions)
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i8") || 
                                         text.contains("pub fn rescale<") ||
                                         text.contains("const FROM:") ||
                                         text.contains("const TO:") ||
                                         text.contains("impl Add for") ||
                                         text.contains("impl Sub for") ||
                                         text.contains("impl Mul<") ||
                                         text.contains("impl Div<") ||
                                         text.contains("impl Mul for") ||
                                         text.contains("impl Div for");
            
            let params = if is_const_generic_context {
                // This is a type definition or const generic context, treat as unknown (all i8::MIN placeholders)
                Some(QuantityParams {
                    mass_exp: i8::MIN,
                    mass_scale: i8::MIN,
                    length_exp: i8::MIN,
                    length_scale: i8::MIN,
                    time_exp: i8::MIN,
                    time_p2: i8::MIN,
                    time_p3: i8::MIN,
                    time_p5: i8::MIN,
                    generic_type: "f64".to_string(),
                })
            } else {
                self.parse_quantity_params(&full_match)
            };
            
            if let Some(params) = params {
                // Check if this is a partially resolved type
                if self.is_partially_resolved_type(&full_match) {
                    // For partially resolved types, leverage existing prettyprint logic
                    // First try to look up recognized dimension names
                    if let Some(dimension_info) = lookup_dimension_name(params.mass_exp, params.length_exp, params.time_exp) {
                        dimension_info.dimension_name.to_string()
                    } else {
                        // For unrecognized composite types, show dimension symbols (M, L, T)
                        generate_dimension_symbols(params.mass_exp, params.length_exp, params.time_exp)
                    }
                } else {
                    // Use the new prettyprint API with verbose=false
                    pretty_print_quantity_type(
                        params.mass_exp, params.mass_scale,
                        params.length_exp, params.length_scale,
                        params.time_exp, params.time_p2, params.time_p3, params.time_p5,
                        false, // not verbose
                    )
                }
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Convert Quantity types to ultra-terse inlay hint format
    pub fn convert_types_in_text_inlay_hint(&self, text: &str) -> String {
        use whippyunits::print::prettyprint::{pretty_print_quantity_inlay_hint, generate_dimension_symbols};
        use whippyunits::print::name_lookup::{lookup_dimension_name, generate_systematic_unit_name};
        
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i8")
            // Also check if this is a type definition or const generic context (like rescale functions)
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i8") || 
                                         text.contains("pub fn rescale<") ||
                                         text.contains("const FROM:") ||
                                         text.contains("const TO:") ||
                                         text.contains("impl Add for") ||
                                         text.contains("impl Sub for") ||
                                         text.contains("impl Mul<") ||
                                         text.contains("impl Div<") ||
                                         text.contains("impl Mul for") ||
                                         text.contains("impl Div for");
            
            let params = if is_const_generic_context {
                // This is a type definition or const generic context, treat as unknown (all i8::MIN placeholders)
                Some(QuantityParams {
                    mass_exp: i8::MIN,
                    mass_scale: i8::MIN,
                    length_exp: i8::MIN,
                    length_scale: i8::MIN,
                    time_exp: i8::MIN,
                    time_p2: i8::MIN,
                    time_p3: i8::MIN,
                    time_p5: i8::MIN,
                    generic_type: "f64".to_string(),
                })
            } else {
                self.parse_quantity_params(&full_match)
            };
            
            if let Some(params) = params {
                // Check if this is a partially resolved type
                if self.is_partially_resolved_type(&full_match) {
                    // For partially resolved types, leverage existing prettyprint logic
                    // First try to look up recognized dimension names
                    if let Some(dimension_info) = lookup_dimension_name(params.mass_exp, params.length_exp, params.time_exp) {
                        dimension_info.dimension_name.to_string()
                    } else {
                        // For unrecognized composite types, show dimension symbols (M, L, T)
                        generate_dimension_symbols(params.mass_exp, params.length_exp, params.time_exp)
                    }
                } else {
                    // Use the new ultra-terse inlay hint API
                    pretty_print_quantity_inlay_hint(
                        params.mass_exp, params.mass_scale,
                        params.length_exp, params.length_scale,
                        params.time_exp, params.time_p2, params.time_p3, params.time_p5,
                    )
                }
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }
    
    fn parse_quantity_params(&self, quantity_type: &str) -> Option<QuantityParams> {
        // Extract const generic parameters from Quantity<...>
        let re = Regex::new(r"Quantity<([^>]*)>").unwrap();
        let captures = re.captures(quantity_type)?;
        let params_str = captures.get(1)?.as_str();
        
        // Parse comma-separated parameters, handling _ placeholders
        let params: Vec<Option<i8>> = params_str
            .split(',')
            .map(|s| {
                let s = s.trim();
                if s == "_" {
                    Some(i8::MIN) // Unknown placeholder
                } else if s == "9223372036854775807" {
                    Some(i8::MAX) // Unused value (original meaning)
                } else {
                    s.parse::<i8>().ok()
                }
            })
            .collect();
        
        if params.len() >= 9 {
            // Extract the generic type parameter (last parameter)
            let generic_type = if params.len() > 9 {
                // If we have more than 9 parameters, the last one is the generic type
                params_str.split(',').nth(8).unwrap_or("f64").trim().to_string()
            } else {
                "f64".to_string() // Default to f64
            };
            
            Some(QuantityParams {
                // New API uses (mass, length, time) order
                mass_exp: params[0].unwrap_or(0),
                mass_scale: params[1].unwrap_or(i8::MAX),
                length_exp: params[2].unwrap_or(0),
                length_scale: params[3].unwrap_or(i8::MAX),
                time_exp: params[4].unwrap_or(0),
                time_p2: params[5].unwrap_or(i8::MAX),
                time_p3: params[6].unwrap_or(i8::MAX),
                time_p5: params[7].unwrap_or(i8::MAX),
                generic_type,
            })
        } else if params.len() >= 8 {
            // Handle legacy 8-parameter format (backward compatibility)
            Some(QuantityParams {
                // New API uses (mass, length, time) order
                mass_exp: params[0].unwrap_or(0),
                mass_scale: params[1].unwrap_or(i8::MAX),
                length_exp: params[2].unwrap_or(0),
                length_scale: params[3].unwrap_or(i8::MAX),
                time_exp: params[4].unwrap_or(0),
                time_p2: params[5].unwrap_or(i8::MAX),
                time_p3: params[6].unwrap_or(i8::MAX),
                time_p5: params[7].unwrap_or(i8::MAX),
                generic_type: "f64".to_string(), // Default to f64 for legacy format
            })
        } else {
            None
        }
    }

    /// Check if a type is partially resolved (has sentinel values for dimensions that should be resolved)
    fn is_partially_resolved_type(&self, text: &str) -> bool {
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        if let Some(caps) = quantity_regex.captures(text) {
            let params_str = &caps[1];
            let params: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
            
            if params.len() >= 8 {
                // Check if any dimension has a non-zero exponent but sentinel scale values
                // This indicates a partially resolved type
                
                // Mass dimension: params[0] = exp, params[1] = scale
                let mass_exp = params[0].parse::<i8>().unwrap_or(0);
                let mass_scale = if params[1] == "_" { i8::MIN } else { params[1].parse::<i8>().unwrap_or(i8::MAX) };
                if mass_exp != 0 && mass_scale == i8::MIN {
                    return true; // Mass has exponent but unknown scale
                }
                
                // Length dimension: params[2] = exp, params[3] = scale  
                let length_exp = params[2].parse::<i8>().unwrap_or(0);
                let length_scale = if params[3] == "_" { i8::MIN } else { params[3].parse::<i8>().unwrap_or(i8::MAX) };
                if length_exp != 0 && length_scale == i8::MIN {
                    return true; // Length has exponent but unknown scale
                }
                
                // Time dimension: params[4] = exp, params[5-7] = p2, p3, p5
                let time_exp = params[4].parse::<i8>().unwrap_or(0);
                let time_p2 = if params[5] == "_" { i8::MIN } else { params[5].parse::<i8>().unwrap_or(i8::MAX) };
                let time_p3 = if params[6] == "_" { i8::MIN } else { params[6].parse::<i8>().unwrap_or(i8::MAX) };
                let time_p5 = if params[7] == "_" { i8::MIN } else { params[7].parse::<i8>().unwrap_or(i8::MAX) };
                if time_exp != 0 && (time_p2 == i8::MIN || time_p3 == i8::MIN || time_p5 == i8::MIN) {
                    return true; // Time has exponent but unknown scale
                }
            }
        }
        
        false // Fully resolved type
    }



    /// Generate compact notation for unresolved types with _ placeholders
    fn generate_ambiguous_matches(&self, text: &str, config: &DisplayConfig) -> String {
        let ambiguous_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        ambiguous_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this contains unresolved placeholders
            if full_match.contains('_') || full_match.contains("9223372036854775807") {
                let mut result = if config.verbose {
                    // In verbose mode, show "Unresolved type" with raw type
                    format!("Unresolved type - {}", full_match)
                } else {
                    // In clean mode, just show "Unresolved"
                    "Unresolved".to_string()
                };
                
                // Add raw type if requested
                if config.include_raw {
                    result.push_str(&format!("\n\nRaw: {}", full_match));
                }
                
                result
            } else {
                full_match
            }
        }).to_string()
    }
}

#[derive(Debug)]
struct QuantityParams {
    length_exp: i8,
    length_scale: i8,
    mass_exp: i8,
    mass_scale: i8,
    time_exp: i8,
    time_p2: i8,
    time_p3: i8,
    time_p5: i8,
    generic_type: String, // New field for the generic type parameter
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        let converted = converter.convert_types_in_text("Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>");
        println!("Converted output: '{}'", converted);
        assert!(converted.contains("m"));
    }

    #[test]
    fn test_text_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        let text = "let x: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807> = 5.0.meters();";
        let converted = converter.convert_types_in_text(text);
        println!("Converted text: '{}'", converted);
        // The Quantity type should be converted to pretty format, but still contain "Quantity<"
        assert!(converted.contains("Quantity<"));
        assert!(converted.contains("m"));
    }

    #[test]
    fn test_unresolved_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        // Test the case from the user's example: Quantity<0, _, 1, _, 0, _, _, _>
        let text = "let distance1: Quantity<0, _, 1, _, 0, _, _, _> = 5.0.millimeters();";
        let converted = converter.convert_types_in_text(text);
        println!("Converted unresolved type: '{}'", converted);
        // Should show "Length" for unresolved length type
        assert!(converted.contains("Length"));
        assert!(!converted.contains("Unresolved type"));
    }

    #[test]
    fn test_composite_unresolved_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        // Test composite type: Quantity<1, _, 1, _, 0, _, _, _> (mass × length)
        let text = "let force: Quantity<1, _, 1, _, 0, _, _, _> = 5.0.newtons();";
        let converted = converter.convert_types_in_text(text);
        println!("Converted composite unresolved type: '{}'", converted);
        // Should show "M·L" for unresolved mass × length type
        assert!(converted.contains("M·L"));
        assert!(!converted.contains("Unresolved type"));
    }

    #[test]
    fn test_recognized_composite_unresolved_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        // Test recognized composite type: Quantity<1, _, 2, _, -2, _, _, _> (Energy)
        let text = "let energy: Quantity<1, _, 2, _, -2, _, _, _> = 5.0.joules();";
        let converted = converter.convert_types_in_text(text);
        println!("Converted recognized composite unresolved type: '{}'", converted);
        // Should show "Energy" for recognized energy type
        assert!(converted.contains("Energy"));
        assert!(!converted.contains("Unresolved type"));
    }

    #[test]
    fn test_verbose_partially_resolved_type() {
        let converter = WhippyUnitsTypeConverter::new();
        // Test verbose mode for partially resolved type: Quantity<0, _, 1, _, 0, _, _, _> (Length)
        let text = "let distance: Quantity<0, _, 1, _, 0, _, _, _> = 5.0.millimeters();";
        let converted = converter.convert_types_in_text_verbose(text);
        println!("Converted verbose partially resolved type: '{}'", converted);
        // Should show the full pretty-printed format with unresolved scale indicators
        assert!(converted.contains("Quantity<"));
        assert!(converted.contains("Length"));
        assert!(converted.contains("(10ˀ)"));
        assert!(converted.contains("(2ˀ, 3ˀ, 5ˀ)"));
    }

    #[test]
    fn test_inlay_hint_integration() {
        let proxy = LspProxy::new();
        
        // Create a mock inlay hint response
        let inlay_hint_response = json!({
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
                    "kind": 1
                }
            ]
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&inlay_hint_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let result = processed_value["result"].as_array().unwrap();
        let label = &result[0]["label"];
        
        // The label should contain the converted type
        let label_str = serde_json::to_string(label).unwrap();
        println!("Processed label: {}", label_str);
        // This is a fully resolved type, so it should contain the ultra-terse format (just the unit literal)
        assert!(label_str.contains("m"));
        assert!(!label_str.contains("Quantity<"));
    }

    #[test]
    fn test_hover_tooltip_processing() {
        let proxy = LspProxy::new();
        
        // Create a mock hover response
        let hover_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nlet distance1: Quantity<0, _, 1, _, 0, _, _, _> = 5.0.millimeters();\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed hover contents: {}", contents_str);
        // Should show "Length" for unresolved length type (capitalized from dimension name)
        assert!(contents_str.contains("Length"));
        assert!(!contents_str.contains("Unresolved type"));
    }

    #[test]
    fn test_inlay_hint_unresolved_types() {
        let proxy = LspProxy::new();
        
        // Test with the exact format from the user's example
        let inlay_hint_response = json!({
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
                        {"value": "<0, _, 1, _, 0, _, _, _>"}
                    ],
                    "kind": 1
                }
            ]
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&inlay_hint_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let result = processed_value["result"].as_array().unwrap();
        let label = &result[0]["label"];
        
        // The label should contain "Length" for the unresolved length type
        let label_str = serde_json::to_string(label).unwrap();
        println!("Processed unresolved type label: {}", label_str);
        assert!(label_str.contains("Length"));
        assert!(!label_str.contains("Unresolved type"));
    }

    #[test]
    fn test_rescale_signature_transformation() {
        let proxy = LspProxy::new();
        
        // Test with the actual rescale function signature format
        let hover_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\npub fn rescale<const MASS_EXPONENT: i8, const MASS_SCALE_P10_FROM: i8, const MASS_SCALE_P10_TO: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10_FROM: i8, const LENGTH_SCALE_P10_TO: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2_FROM: i8, const TIME_SCALE_P3_FROM: i8, const TIME_SCALE_P5_FROM: i8, const TIME_SCALE_P2_TO: i8, const TIME_SCALE_P3_TO: i8, const TIME_SCALE_P5_TO: i8>(quantity: Quantity<Unknown; [mass⁰(unused), length⁰(unused), time⁰(unused))]>) -> Quantity<Unknown; [mass⁰(unused), length⁰(unused), time⁰(unused))]>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed rescale signature: {}", contents_str);
        // Should show a simplified rescale signature
        assert!(contents_str.contains("pub fn rescale<const FROM:"));
        assert!(contents_str.contains("-> TO"));
        assert!(contents_str.contains("quantity: FROM"));
        assert!(!contents_str.contains("MASS_EXPONENT"));
        assert!(!contents_str.contains("LENGTH_EXPONENT"));
        assert!(!contents_str.contains("TIME_EXPONENT"));
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
                    "value": "```rust\nimpl<const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Add for Quantity<MASS_EXPONENT, MASS_SCALE_P10, LENGTH_EXPONENT, LENGTH_SCALE_P10, TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&add_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Add trait signature: {}", contents_str);
        // Should show a simplified Add trait signature
        assert!(contents_str.contains("impl Add for"));
        // The type converter should process the Quantity type and show the pretty output
        // We expect something like Quantity<?> or the full pretty-printed format
        assert!(contents_str.contains("Quantity<"));
        assert!(!contents_str.contains("const MASS_EXPONENT: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT: i8"));
        
        // Test Sub trait
        let sub_hover_response = json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Sub for Quantity<MASS_EXPONENT, MASS_SCALE_P10, LENGTH_EXPONENT, LENGTH_SCALE_P10, TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&sub_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Sub trait signature: {}", contents_str);
        // Should show a simplified Sub trait signature
        assert!(contents_str.contains("impl Sub for"));
        // The type converter should process the Quantity type and show the pretty output
        // We expect something like Quantity<?> or the full pretty-printed format
        assert!(contents_str.contains("Quantity<"));
        assert!(!contents_str.contains("const MASS_EXPONENT: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT: i8"));
        
        // Test Mul trait
        let mul_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT1: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT1: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT1: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Mul for Quantity<MASS_EXPONENT1, MASS_SCALE_P10, LENGTH_EXPONENT1, LENGTH_SCALE_P10, TIME_EXPONENT1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&mul_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Mul trait signature: {}", contents_str);
        // Should show a simplified Mul trait signature
        assert!(contents_str.contains("impl Mul for"));
        assert!(contents_str.contains("Quantity<"));
        // The type converter should process the Quantity types and show the pretty output
        assert!(!contents_str.contains("const MASS_EXPONENT1: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT1: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT1: i8"));
        
        // Test Div trait
        let div_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 4,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT1: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT1: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT1: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Div for Quantity<MASS_EXPONENT1, MASS_SCALE_P10, LENGTH_EXPONENT1, LENGTH_SCALE_P10, TIME_EXPONENT1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&div_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Div trait signature: {}", contents_str);
        // Should show a simplified Div trait signature
        assert!(contents_str.contains("impl Div for"));
        assert!(contents_str.contains("Quantity<"));
        // The type converter should process the Quantity types and show the pretty output
        assert!(!contents_str.contains("const MASS_EXPONENT1: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT1: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT1: i8"));
        
        // Test scalar Mul trait (Quantity * f64)
        let scalar_mul_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 5,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Mul<f64> for Quantity<MASS_EXPONENT, MASS_SCALE_P10, LENGTH_EXPONENT, LENGTH_SCALE_P10, TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&scalar_mul_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed scalar Mul trait signature: {}", contents_str);
        // Should show a simplified scalar Mul trait signature
        assert!(contents_str.contains("impl Mul<f64>"));
        assert!(contents_str.contains("for Quantity<"));
        // The type converter should process the Quantity type and show the pretty output
        assert!(contents_str.contains("Quantity<"));
        assert!(!contents_str.contains("const MASS_EXPONENT: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT: i8"));
        
        // Test Mul for pattern (where LSP doesn't preserve generic parameters)
        let mul_for_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 6,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Mul for Quantity<LENGTH_EXPONENT, LENGTH_SCALE_P10, MASS_EXPONENT, MASS_SCALE_P10, TIME_EXPONENT, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\nfn mul(self, other: f64) -> Self::Output\nPerforms the * operation.\n\nExample\nassert_eq!(12 * 2, 24);\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&mul_for_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Mul for trait signature: {}", contents_str);
        // Should show a simplified Mul trait signature with f64 parameter detected from function signature
        assert!(contents_str.contains("impl Mul<f64>"));
        assert!(contents_str.contains("for Quantity<"));
        // The type converter should process the Quantity type and show the pretty output
        assert!(contents_str.contains("Quantity<"));
        assert!(!contents_str.contains("const MASS_EXPONENT: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT: i8"));
        
        // Test Mul trait with where clause (Quantity-Quantity case)
        let mul_where_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 7,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT1: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT1: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT1: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Mul for Quantity<MASS_EXPONENT1, MASS_SCALE_P10, LENGTH_EXPONENT1, LENGTH_SCALE_P10, TIME_EXPONENT1, TIME_SCALE_P2, TIME_SCALE_P3, TIME_SCALE_P5>\nwhere\n    MASS_EXPONENT1: i8,\n    LENGTH_EXPONENT1: i8,\n    TIME_EXPONENT1: i8,\n    MASS_SCALE_P10: i8,\n    LENGTH_SCALE_P10: i8,\n    TIME_SCALE_P2: i8,\n    TIME_SCALE_P3: i8,\n    TIME_SCALE_P5: i8,\n{\n```"
                }
            }
        });
        

        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&mul_where_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed Mul trait with where clause: {}", contents_str);
        // Should show a simplified Mul trait signature without the where clause
        assert!(contents_str.contains("impl Mul for"));
        assert!(contents_str.contains("Quantity<"));
        // The type converter should process the Quantity types and show the pretty output
        assert!(!contents_str.contains("const MASS_EXPONENT1: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT1: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT1: i8"));
        // Should not contain the where clause
        assert!(!contents_str.contains("where"));
        assert!(!contents_str.contains("MASS_EXPONENT1: i8"));
    }

    #[test]
    fn test_reverse_scalar_mul() {
        let proxy = LspProxy::new();
        
        // Test reverse scalar Mul trait (f64 * Quantity)
        let reverse_scalar_mul_hover_response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "contents": {
                    "kind": "markdown",
                    "value": "```rust\nimpl<const MASS_EXPONENT: i8, const MASS_SCALE_P10: i8, const LENGTH_EXPONENT: i8, const LENGTH_SCALE_P10: i8, const TIME_EXPONENT: i8, const TIME_SCALE_P2: i8, const TIME_SCALE_P3: i8, const TIME_SCALE_P5: i8> Mul for f64\nfn mul(self: f64, other: Self::Output) -> Self::Output\nPerforms the * operation.\n\nExample\nassert_eq!(12 * 2, 24);\n```"
                }
            }
        });
        
        // Convert to LSP message format
        let json_str = serde_json::to_string(&reverse_scalar_mul_hover_response).unwrap();
        let lsp_message = format!("Content-Length: {}\r\n\r\n{}", json_str.len(), json_str);
        
        // Process the message
        let processed = proxy.process_incoming(&lsp_message).unwrap();
        
        // Extract the JSON payload from the processed message
        let lines: Vec<&str> = processed.lines().collect();
        let json_start = lines.iter().position(|line| line.trim().is_empty()).unwrap() + 1;
        let processed_json = lines[json_start..].join("\n");
        
        // Parse and verify the result
        let processed_value: Value = serde_json::from_str(&processed_json).unwrap();
        let contents = &processed_value["result"]["contents"]["value"];
        let contents_str = contents.as_str().unwrap();
        
        println!("Processed reverse scalar Mul trait: {}", contents_str);
        // Should show a simplified reverse scalar Mul trait signature
        assert!(contents_str.contains("impl Mul<Quantity<"));
        assert!(contents_str.contains("for f64"));
        // Should preserve the function definition and documentation
        assert!(contents_str.contains("fn mul(self: f64, other: Self::Output) -> Self::Output"));
        assert!(contents_str.contains("Performs the * operation."));
        // Should not contain the const generic parameters
        assert!(!contents_str.contains("const MASS_EXPONENT: i8"));
        assert!(!contents_str.contains("const LENGTH_EXPONENT: i8"));
        assert!(!contents_str.contains("const TIME_EXPONENT: i8"));
    }
}
