use serde::{Deserialize, Serialize};
use serde_json::Value;
use regex::Regex;

/// Macro to format the bracket notation consistently across different contexts
macro_rules! format_bracket_details {
    ($length_exp:expr, $length_scale:expr, $length_scale_name:expr,
     $mass_exp:expr, $mass_scale:expr, $mass_scale_name:expr,
     $time_exp:expr, $time_p2:expr, $time_p3:expr, $time_p5:expr, $time_scale_name:expr) => {
        format!(
            "Length: Exponent {} [Scale Index {}; {}], Mass: Exponent {} [Scale Index {}; {}], Time: Exponent {} [Prime Factors p2:{}, p3:{}, p5:{}; {}]",
            $length_exp,
            $length_scale,
            $length_scale_name,
            $mass_exp,
            $mass_scale,
            $mass_scale_name,
            $time_exp,
            $time_p2,
            $time_p3,
            $time_p5,
            $time_scale_name
        )
    };
}

#[cfg(test)]
mod integration_test;

#[cfg(test)]
mod minimal_hover_test;

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
}

impl LspProxy {
    pub fn new() -> Self {
        Self {
            type_converter: WhippyUnitsTypeConverter::new(),
            display_config: DisplayConfig::default(),
        }
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            type_converter: WhippyUnitsTypeConverter::new(),
            display_config,
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
        
        // Reconstruct the LSP message format
        let new_json = serde_json::to_string(&lsp_msg)?;
        let content_length = new_json.len();
        Ok(format!("Content-Length: {}\r\n\r\n{}", content_length, new_json))
    }

    /// Process an outgoing LSP message (from editor to rust-analyzer)
    /// This expects a complete LSP message with Content-Length header
    pub fn process_outgoing(&self, message: &str) -> Result<String, anyhow::Error> {
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
}

/// Type converter for whippyunits types
#[derive(Clone)]
pub struct WhippyUnitsTypeConverter;

impl WhippyUnitsTypeConverter {
    pub fn new() -> Self {
        Self
    }

    /// Convert types in text with display configuration
    pub fn convert_types_in_text_with_config(&self, text: &str, config: &DisplayConfig) -> String {
        // Check for truly unresolved types with _ placeholders
        if text.contains('_') {
            return self.generate_ambiguous_matches(text, config);
        }
        
        // Check for partially resolved types (mix of specific values and sentinel values)
        if text.contains("9223372036854775807") {
            // Parse the type to see if it's fully resolved or partially resolved
            if self.is_partially_resolved_type(text) {
                return self.generate_ambiguous_matches(text, config);
            }
        }
        
        let mut result = if config.verbose {
            // In verbose mode, just convert Quantity types to readable format
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

    /// Convert Quantity types to verbose const generic display
    fn convert_quantity_types_verbose(&self, text: &str) -> String {
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            let params = self.parse_quantity_params(&full_match);
            if let Some(params) = params {
                // Get the clean unit display first
                let clean_display = self.build_unit_strings_unicode(&params);
                // Get the detailed breakdown
                let details = self.build_type_details(&params);
                // Combine them: clean display + detailed breakdown
                format!("{} - {}", clean_display, details)
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Convert Quantity types to clean unit display
    fn convert_quantity_types_clean(&self, text: &str, unicode: bool) -> String {
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            let params = self.parse_quantity_params(&full_match);
            if let Some(params) = params {
                if unicode {
                    self.build_unit_strings_unicode(&params)
                } else {
                    self.build_unit_strings_ascii(&params)
                }
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Build unit strings with Unicode superscripts
    fn build_unit_strings_unicode(&self, params: &QuantityParams) -> String {
        let mut units = Vec::new();
        
        // Length
        if params.length_exp != 0 {
            let unit = match params.length_scale {
                -1 => "mm",
                0 => "m",
                1 => "km",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                let superscript = self.to_unicode_superscript(params.length_exp);
                units.push(format!("{}{}", unit, superscript));
            }
        }

        // Mass
        if params.mass_exp != 0 {
            let unit = match params.mass_scale {
                -1 => "mg",
                0 => "g",
                1 => "kg",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                let superscript = self.to_unicode_superscript(params.mass_exp);
                units.push(format!("{}{}", unit, superscript));
            }
        }

        // Time
        if params.time_exp != 0 {
            let unit = match params.time_scale_order {
                -1 => "ms",
                0 => "s",
                1 => "min",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                let superscript = self.to_unicode_superscript(params.time_exp);
                units.push(format!("{}{}", unit, superscript));
            }
        }

        if units.is_empty() {
            "dimensionless".to_string()
        } else {
            units.join("·")
        }
    }

    /// Build unit strings with ASCII notation
    fn build_unit_strings_ascii(&self, params: &QuantityParams) -> String {
        let mut units = Vec::new();
        
        // Length
        if params.length_exp != 0 {
            let unit = match params.length_scale {
                -1 => "mm",
                0 => "m",
                1 => "km",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                if params.length_exp == 1 {
                    units.push(unit.to_string());
                } else {
                    units.push(format!("{}^{}", unit, params.length_exp));
                }
            }
        }

        // Mass
        if params.mass_exp != 0 {
            let unit = match params.mass_scale {
                -1 => "mg",
                0 => "g",
                1 => "kg",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                if params.mass_exp == 1 {
                    units.push(unit.to_string());
                } else {
                    units.push(format!("{}^{}", unit, params.mass_exp));
                }
            }
        }

        // Time
        if params.time_exp != 0 {
            let unit = match params.time_scale_order {
                -1 => "ms",
                0 => "s",
                1 => "min",
                isize::MAX => "",
                _ => "unknown",
            };
            if !unit.is_empty() {
                if params.time_exp == 1 {
                    units.push(unit.to_string());
                } else {
                    units.push(format!("{}^{}", unit, params.time_exp));
                }
            }
        }

        if units.is_empty() {
            "dimensionless".to_string()
        } else {
            units.join("·")
        }
    }

    /// Convert number to Unicode superscript
    fn to_unicode_superscript(&self, n: isize) -> String {
        if n == 1 {
            return "".to_string(); // No superscript for 1
        }
        
        n.to_string()
            .chars()
            .map(|c| match c {
                '0' => '⁰',
                '1' => '¹',
                '2' => '²',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                '-' => '⁻',
                _ => c,
            })
            .collect()
    }

    /// Get superscript question mark for unresolved exponents
    fn superscript_question_mark(&self) -> &'static str {
        "ˀ" // Unicode superscript question mark
    }
    
    fn parse_quantity_params(&self, quantity_type: &str) -> Option<QuantityParams> {
        // Extract const generic parameters from Quantity<...>
        let re = Regex::new(r"Quantity<([^>]*)>").unwrap();
        let captures = re.captures(quantity_type)?;
        let params_str = captures.get(1)?.as_str();
        
        // Parse comma-separated parameters
        let params: Vec<Result<isize, _>> = params_str
            .split(',')
            .map(|s| s.trim().parse::<isize>())
            .collect();
        
        // Check if any parameter failed to parse (contains _ or other non-numeric)
        if params.iter().any(|r| r.is_err()) {
            return None;
        }
        
        let params: Vec<isize> = params.into_iter().map(|r| r.unwrap()).collect();
        
        if params.len() >= 9 {
            Some(QuantityParams {
                length_exp: params[0],
                length_scale: params[1],
                mass_exp: params[2],
                mass_scale: params[3],
                time_exp: params[4],
                time_p2: params[5],
                time_p3: params[6],
                time_p5: params[7],
                time_scale_order: params[8],
            })
        } else {
            None
        }
    }
    

    
    fn build_type_details(&self, params: &QuantityParams) -> String {
        format_bracket_details!(
            params.length_exp,
            match params.length_scale {
                isize::MAX => "MAX".to_string(),
                _ => params.length_scale.to_string(),
            },
            self.scale_name(params.length_scale, "length"),
            params.mass_exp,
            match params.mass_scale {
                isize::MAX => "MAX".to_string(),
                _ => params.mass_scale.to_string(),
            },
            self.scale_name(params.mass_scale, "mass"),
            params.time_exp,
            match params.time_p2 {
                isize::MAX => "MAX".to_string(),
                _ => params.time_p2.to_string(),
            },
            match params.time_p3 {
                isize::MAX => "MAX".to_string(),
                _ => params.time_p3.to_string(),
            },
            match params.time_p5 {
                isize::MAX => "MAX".to_string(),
                _ => params.time_p5.to_string(),
            },
            self.scale_name(params.time_scale_order, "time")
        )
    }
    
    fn scale_name_from_str(&self, scale_str: &str, dimension: &str) -> &'static str {
        // Parse the scale string, handling _ and sentinel values
        if scale_str == "_" {
            return "unresolved";
        }
        
        if scale_str == "9223372036854775807" {
            return "unused";
        }
        
        if let Ok(scale) = scale_str.parse::<isize>() {
            self.scale_name(scale, dimension)
        } else {
            "unknown"
        }
    }

    fn scale_name(&self, scale: isize, dimension: &str) -> &'static str {
        match dimension {
            "length" => match scale {
                -1 => "millimeter",
                0 => "meter",
                1 => "kilometer",
                isize::MAX => "unused",
                _ => "unknown",
            },
            "mass" => match scale {
                -1 => "milligram",
                0 => "gram",
                1 => "kilogram",
                isize::MAX => "unused",
                _ => "unknown",
            },
            "time" => match scale {
                -1 => "millisecond",
                0 => "second",
                1 => "minute",
                isize::MAX => "unused",
                _ => "unknown",
            },
            _ => "unknown",
        }
    }

    /// Check if a type is partially resolved (has sentinel values for dimensions that should be resolved)
    fn is_partially_resolved_type(&self, text: &str) -> bool {
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        if let Some(caps) = quantity_regex.captures(text) {
            let params_str = &caps[1];
            let params: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
            
            if params.len() >= 9 {
                // Check if any dimension has a non-zero exponent but sentinel scale values
                // This indicates a partially resolved type
                
                // Length dimension: params[0] = exp, params[1] = scale
                let length_exp = params[0].parse::<isize>().unwrap_or(0);
                let length_scale = params[1].parse::<isize>().unwrap_or(isize::MAX);
                if length_exp != 0 && length_scale == isize::MAX {
                    return true; // Length has exponent but unresolved scale
                }
                
                // Mass dimension: params[2] = exp, params[3] = scale  
                let mass_exp = params[2].parse::<isize>().unwrap_or(0);
                let mass_scale = params[3].parse::<isize>().unwrap_or(isize::MAX);
                if mass_exp != 0 && mass_scale == isize::MAX {
                    return true; // Mass has exponent but unresolved scale
                }
                
                // Time dimension: params[4] = exp, params[8] = scale_order
                let time_exp = params[4].parse::<isize>().unwrap_or(0);
                let time_scale_order = params[8].parse::<isize>().unwrap_or(isize::MAX);
                if time_exp != 0 && time_scale_order == isize::MAX {
                    return true; // Time has exponent but unresolved scale
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
                let constraint_description = self.build_constraint_description(&full_match);
                let mut result = if config.verbose {
                    // In verbose mode, add detailed metadata
                    let details = self.build_unresolved_type_details(&full_match);
                    format!("Unresolved type - {} - {}", constraint_description, details)
                } else {
                    // In clean mode, just show the constraint description
                    format!("Unresolved type - {}", constraint_description)
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



    /// Build detailed metadata for unresolved types
    fn build_unresolved_type_details(&self, quantity_type: &str) -> String {
        let params_match = Regex::new(r"Quantity<([^>]+)>").unwrap();
        
        if let Some(captures) = params_match.captures(quantity_type) {
            let params_str = &captures[1];
            let params: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
            
            if params.len() >= 9 {
                format_bracket_details!(
                    params[0], // length_exp
                    match params[1] {
                        "9223372036854775807" => "MAX".to_string(),
                        _ => params[1].to_string(),
                    },
                    self.scale_name_from_str(params[1], "length"),
                    params[2], // mass_exp
                    match params[3] {
                        "9223372036854775807" => "MAX".to_string(),
                        _ => params[3].to_string(),
                    },
                    self.scale_name_from_str(params[3], "mass"),
                    params[4], // time_exp
                    match params[5] {
                        "9223372036854775807" => "MAX".to_string(),
                        _ => params[5].to_string(),
                    },
                    match params[6] {
                        "9223372036854775807" => "MAX".to_string(),
                        _ => params[6].to_string(),
                    },
                    match params[7] {
                        "9223372036854775807" => "MAX".to_string(),
                        _ => params[7].to_string(),
                    },
                    self.scale_name_from_str(params[8], "time")
                )
            } else {
                "invalid type format".to_string()
            }
        } else {
            "invalid type format".to_string()
        }
    }

    /// Build a compact unit notation showing resolved vs unresolved parts
    fn build_constraint_description(&self, quantity_type: &str) -> String {
        let params_match = Regex::new(r"Quantity<([^>]+)>").unwrap();
        
        if let Some(captures) = params_match.captures(quantity_type) {
            let params_str = &captures[1];
            let params: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
            
            if params.len() >= 9 {
                let mut units = Vec::new();
                
                // Length dimension (index 0: exp, 1: scale)
                // Prune if exponent is 0 OR scale is MAX (dimension not involved)
                let length_exp = params[0].parse::<isize>().ok();
                let length_scale = params[1].parse::<isize>().ok();
                let length_scale_is_max = length_scale == Some(isize::MAX);
                
                if length_exp != Some(0) && !length_scale_is_max {
                    if let Some(exp) = length_exp {
                        if let Some(scale) = length_scale {
                            let unit = match scale {
                                -1 => "mm",
                                0 => "m",
                                1 => "km",
                                _ => "m", // fallback
                            };
                            if exp == 1 {
                                units.push(unit.to_string());
                            } else {
                                units.push(format!("{}{}", unit, self.to_unicode_superscript(exp)));
                            }
                        } else {
                            units.push(format!("m{}", self.to_unicode_superscript(exp)));
                        }
                    } else if let Some(scale) = length_scale {
                        let unit = match scale {
                            -1 => "mm",
                            0 => "m",
                            1 => "km",
                            _ => "m", // fallback
                        };
                        units.push(format!("{}{}", unit, self.superscript_question_mark()));
                    }
                }
                
                // Mass dimension (index 2: exp, 3: scale)
                // Prune if exponent is 0 OR scale is MAX (dimension not involved)
                let mass_exp = params[2].parse::<isize>().ok();
                let mass_scale = params[3].parse::<isize>().ok();
                let mass_scale_is_max = mass_scale == Some(isize::MAX);
                
                if mass_exp != Some(0) && !mass_scale_is_max {
                    if let Some(exp) = mass_exp {
                        if let Some(scale) = mass_scale {
                            let unit = match scale {
                                -1 => "mg",
                                0 => "g",
                                1 => "kg",
                                _ => "g", // fallback
                            };
                            if exp == 1 {
                                units.push(unit.to_string());
                            } else {
                                units.push(format!("{}{}", unit, self.to_unicode_superscript(exp)));
                            }
                        } else {
                            units.push(format!("g{}", self.to_unicode_superscript(exp)));
                        }
                    } else if let Some(scale) = mass_scale {
                        let unit = match scale {
                            -1 => "mg",
                            0 => "g",
                            1 => "kg",
                            _ => "g", // fallback
                        };
                        units.push(format!("{}{}", unit, self.superscript_question_mark()));
                    }
                }
                
                // Time dimension (index 4: exp, 8: scale_order)
                // Prune if exponent is 0 OR scale_order is MAX (dimension not involved)
                let time_exp = params[4].parse::<isize>().ok();
                let time_scale_order = params[8].parse::<isize>().ok();
                let time_scale_is_max = time_scale_order == Some(isize::MAX);
                
                if time_exp != Some(0) && !time_scale_is_max {
                    if let Some(exp) = time_exp {
                        if let Some(scale_order) = time_scale_order {
                            let unit = match scale_order {
                                -1 => "ms",
                                0 => "s",
                                1 => "min",
                                _ => "s", // fallback
                            };
                            if exp == 1 {
                                units.push(unit.to_string());
                            } else {
                                units.push(format!("{}{}", unit, self.to_unicode_superscript(exp)));
                            }
                        } else {
                            units.push(format!("s{}", self.to_unicode_superscript(exp)));
                        }
                    } else if let Some(scale_order) = time_scale_order {
                        let unit = match scale_order {
                            -1 => "ms",
                            0 => "s",
                            1 => "min",
                            _ => "s", // fallback
                        };
                        units.push(format!("{}{}", unit, self.superscript_question_mark()));
                    }
                }
                
                if units.is_empty() {
                    "Unresolved".to_string()
                } else {
                    units.join("·")
                }
            } else {
                "Unresolved".to_string()
            }
        } else {
            "Unresolved".to_string()
        }
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
    time_scale_order: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        let result = converter.convert_quantity_type("Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0>");
        assert!(result.is_some());
        let converted = result.unwrap();
        assert!(converted.contains("meter"));
    }

    #[test]
    fn test_text_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        let text = "let x: Quantity<1, 0, 0, 9223372036854775807, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807, 0> = 5.0.meters();";
        let converted = converter.convert_types_in_text(text);
        assert!(converted.contains("Quantity<"));
        assert!(converted.contains("meter"));
    }
}
