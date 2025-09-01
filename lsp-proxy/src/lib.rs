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
                item.value = self.type_converter.convert_types_in_text_with_config(&item.value, &self.display_config);
            }
            HoverContents::Multiple(items) => {
                for item in items {
                    item.value = self.type_converter.convert_types_in_text_with_config(&item.value, &self.display_config);
                }
            }
        }
        hover
    }

    /// Check if this is an inlay hint related message
    fn is_inlay_hint_message(&self, lsp_msg: &LspMessage) -> bool {
        // Check if the method is any inlay hint related method
        if let Some(method) = &lsp_msg.method {
            if method == "textDocument/inlayHint" ||
               method == "inlayHint/resolve" ||
               method == "workspace/inlayHint/refresh" {
                return true;
            }
        }
        
        // Check if the result contains inlay hint data structure (for responses)
        if let Some(result) = &lsp_msg.result {
            // Check if result is an array (typical for inlay hints)
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
        }
        
        false
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
        if config.include_raw {
            result.push_str(&format!("\n\nRaw: {}", text));
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
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: isize")
            let params = if full_match.contains("const") || full_match.contains("isize") {
                // This is a type definition, treat as fully unresolved (all _ placeholders)
                Some(QuantityParams {
                    mass_exp: isize::MIN,
                    mass_scale: isize::MIN,
                    length_exp: isize::MIN,
                    length_scale: isize::MIN,
                    time_exp: isize::MIN,
                    time_p2: isize::MIN,
                    time_p3: isize::MIN,
                    time_p5: isize::MIN,
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
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: isize")
            let params = if full_match.contains("const") || full_match.contains("isize") {
                // This is a type definition, treat as fully unresolved (all _ placeholders)
                Some(QuantityParams {
                    mass_exp: isize::MIN,
                    mass_scale: isize::MIN,
                    length_exp: isize::MIN,
                    length_scale: isize::MIN,
                    time_exp: isize::MIN,
                    time_p2: isize::MIN,
                    time_p3: isize::MIN,
                    time_p5: isize::MIN,
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
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: isize")
            let params = if full_match.contains("const") || full_match.contains("isize") {
                // This is a type definition, treat as fully unresolved (all _ placeholders)
                Some(QuantityParams {
                    mass_exp: isize::MIN,
                    mass_scale: isize::MIN,
                    length_exp: isize::MIN,
                    length_scale: isize::MIN,
                    time_exp: isize::MIN,
                    time_p2: isize::MIN,
                    time_p3: isize::MIN,
                    time_p5: isize::MIN,
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
        let params: Vec<Option<isize>> = params_str
            .split(',')
            .map(|s| {
                let s = s.trim();
                if s == "_" {
                    Some(isize::MIN) // Unknown placeholder
                } else if s == "9223372036854775807" {
                    Some(isize::MAX) // Unused value (original meaning)
                } else {
                    s.parse::<isize>().ok()
                }
            })
            .collect();
        
        if params.len() >= 8 {
            Some(QuantityParams {
                // New API uses (mass, length, time) order
                mass_exp: params[0].unwrap_or(0),
                mass_scale: params[1].unwrap_or(isize::MAX),
                length_exp: params[2].unwrap_or(0),
                length_scale: params[3].unwrap_or(isize::MAX),
                time_exp: params[4].unwrap_or(0),
                time_p2: params[5].unwrap_or(isize::MAX),
                time_p3: params[6].unwrap_or(isize::MAX),
                time_p5: params[7].unwrap_or(isize::MAX),
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
                let mass_exp = params[0].parse::<isize>().unwrap_or(0);
                let mass_scale = if params[1] == "_" { isize::MIN } else { params[1].parse::<isize>().unwrap_or(isize::MAX) };
                if mass_exp != 0 && mass_scale == isize::MIN {
                    return true; // Mass has exponent but unknown scale
                }
                
                // Length dimension: params[2] = exp, params[3] = scale  
                let length_exp = params[2].parse::<isize>().unwrap_or(0);
                let length_scale = if params[3] == "_" { isize::MIN } else { params[3].parse::<isize>().unwrap_or(isize::MAX) };
                if length_exp != 0 && length_scale == isize::MIN {
                    return true; // Length has exponent but unknown scale
                }
                
                // Time dimension: params[4] = exp, params[5-7] = p2, p3, p5
                let time_exp = params[4].parse::<isize>().unwrap_or(0);
                let time_p2 = if params[5] == "_" { isize::MIN } else { params[5].parse::<isize>().unwrap_or(isize::MAX) };
                let time_p3 = if params[6] == "_" { isize::MIN } else { params[6].parse::<isize>().unwrap_or(isize::MAX) };
                let time_p5 = if params[7] == "_" { isize::MIN } else { params[7].parse::<isize>().unwrap_or(isize::MAX) };
                if time_exp != 0 && (time_p2 == isize::MIN || time_p3 == isize::MIN || time_p5 == isize::MIN) {
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
    length_exp: isize,
    length_scale: isize,
    mass_exp: isize,
    mass_scale: isize,
    time_exp: isize,
    time_p2: isize,
    time_p3: isize,
    time_p5: isize,
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
        // Should show "length" for unresolved length type
        assert!(converted.contains("length"));
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
        
        // The label should contain "length" for the unresolved length type
        let label_str = serde_json::to_string(label).unwrap();
        println!("Processed unresolved type label: {}", label_str);
        assert!(label_str.contains("length"));
        assert!(!label_str.contains("Unresolved type"));
    }
}
