use regex::Regex;

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

/// Formatter for whippyunits types using the new prettyprint API
#[derive(Clone)]
pub struct UnitFormatter;

impl UnitFormatter {
    pub fn new() -> Self {
        Self
    }

    /// Format whippyunits types in text with the specified configuration
    pub fn format_types(&self, text: &str, config: &DisplayConfig) -> String {
        let mut result = self.format_quantity_types(text, config.verbose, config.unicode, false);
        
        // Add raw type if requested and we actually made changes
        if config.include_raw && result != text {
            result.push_str(&format!("\n\nRaw: {}", text));
        }
        
        result
    }

    /// Format whippyunits types in text with original text for Raw section
    pub fn format_types_with_original(&self, text: &str, config: &DisplayConfig, original_text: &str) -> String {
        let mut result = self.format_quantity_types(text, config.verbose, config.unicode, false);
        
        // Add raw type if requested and we actually made changes
        if config.include_raw && result != original_text {
            // Strip markdown formatting from the original text for cleaner display
            let cleaned_original = self.strip_markdown_formatting(original_text);
            
            // Check if there's a "---" separator in the original text and insert raw text after it
            if let Some(_separator_pos) = original_text.find("---") {
                // Insert raw text with separator between formatted result and raw text
                // Add extra line break after "Raw:" and clean up extra separators
                let cleaned_original_with_breaks = self.clean_up_raw_formatting(&cleaned_original);
                result = format!("{}\n\n---\nRaw:\n\n{}", result, cleaned_original_with_breaks);
            } else {
                // No separator found, append at the end with extra line break after "Raw:"
                let cleaned_original_with_breaks = self.clean_up_raw_formatting(&cleaned_original);
                result.push_str(&format!("\n\nRaw:\n\n{}", cleaned_original_with_breaks));
            }
        }
        
        result
    }

    /// Format whippyunits types for inlay hints (compact format)
    pub fn format_types_inlay_hint(&self, text: &str) -> String {
        self.format_quantity_types(text, false, true, true)
    }

    /// Core method to format Quantity types with configurable parameters
    fn format_quantity_types(&self, text: &str, verbose: bool, unicode: bool, is_inlay_hint: bool) -> String {
        
        use whippyunits::print::prettyprint::{pretty_print_quantity_type, generate_dimension_symbols};
        use whippyunits::print::name_lookup::lookup_dimension_name;
        
        // Handle the new format with Scale<...> and Dimension<...> structs
        if text.contains("Scale<") && text.contains("Dimension<") {
            // Use a more sophisticated approach to find and replace each Quantity type
            // We'll manually find the start and end of each Quantity type by counting brackets
            let mut result = String::new();
            let mut i = 0;
            
            while i < text.len() {
                if let Some(start) = text[i..].find("Quantity<Scale<") {
                    let start_pos = i + start;
                    let mut bracket_count = 0;
                    let mut found_end = false;
                    
                    // Count brackets to find the matching end
                    // Start counting from the first '<' after "Quantity"
                    let quantity_start = start_pos + 8; // Skip "Quantity"
                    let mut j = quantity_start;
                    
                    while j < text.len() {
                        match text.chars().nth(j) {
                            Some('<') => bracket_count += 1,
                            Some('>') => {
                                bracket_count -= 1;
                                if bracket_count == 0 {
                                    found_end = true;
                                    break;
                                }
                            },
                            _ => {}
                        }
                        j += 1;
                    }
                    
                    if found_end {
                        // The bracket counting found the first '>', but we need to find the actual end
                        // of the Quantity type. Look for the last '>' that closes the Quantity<...> structure
                        let mut actual_end = j;
                        
                        // Continue looking for the final '>' that closes the entire Quantity<...> structure
                        // We need to find the last '>' that's not part of nested generics
                        while actual_end < text.len() {
                            if let Some(ch) = text.chars().nth(actual_end) {
                                if ch == '>' {
                                    // Check if this is the final '>' by looking ahead
                                    // If the next non-whitespace character is not part of a generic type,
                                    // then this is the final '>'
                                    let remaining = &text[actual_end + 1..];
                                    let next_non_whitespace = remaining.chars().find(|c| !c.is_whitespace());
                                    
                                    match next_non_whitespace {
                                        Some(',') | Some(')') | Some(']') | Some('}') | Some(' ') | Some('\n') | Some('\t') | None => {
                                            // This is the final '>'
                                            break;
                                        }
                                        _ => {
                                            // This is not the final '>', continue
                                            actual_end += 1;
                                        }
                                    }
                                } else {
                                    actual_end += 1;
                                }
                            } else {
                                break;
                            }
                        }
                        
                        // Extract the quantity type including all the '>' characters
                        let quantity_type = &text[start_pos..actual_end+1];
                        let formatted = self.format_new_quantity_type(quantity_type, verbose, unicode, is_inlay_hint);
                        result.push_str(&text[i..start_pos]);
                        result.push_str(&formatted);
                        i = actual_end + 1;
                    } else {
                        result.push_str(&text[i..]);
                        break;
                    }
                } else {
                    result.push_str(&text[i..]);
                    break;
                }
            }
            
            return result;
        }
        
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i16")
            // Also check if we're in a context that suggests const generic parameters (like rescale functions)
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i16") || 
                                         (text.contains("pub fn rescale<") && !text.contains("const FROM_TYPE:")) ||
                                         (text.contains("const FROM:") && !text.contains("const TO_TYPE:")) ||
                                         (text.contains("const TO:") && !text.contains("const TO_TYPE:")) ||
                                         text.contains("impl Add for") ||
                                         text.contains("impl Sub for") ||
                                         text.contains("impl Mul<") ||
                                         text.contains("impl Div<") ||
                                         text.contains("impl Mul for") ||
                                         text.contains("impl Div for");
            
            let params = if is_const_generic_context {
                // This is a type definition or const generic context
                // Try to parse the actual values first, fall back to unknown if parsing fails
                self.parse_quantity_params(&full_match).or_else(|| {
                    // If parsing fails, treat as unknown (all i16::MIN placeholders)
                    Some(QuantityParams {
                        mass_exp: i16::MIN,
                        length_exp: i16::MIN,
                        time_exp: i16::MIN,
                        electric_current_exp: 0,
                        temperature_exp: 0,
                        amount_of_substance_exp: 0,
                        luminous_intensity_exp: 0,
                        angle_exp: 0,
                        scale_p2: i16::MIN,
                        scale_p3: i16::MIN,
                        scale_p5: i16::MIN,
                        scale_pi: i16::MIN,
                        generic_type: "f64".to_string(),
                    })
                })
            } else {
                self.parse_quantity_params(&full_match)
            };
            
            if let Some(params) = params {
                // Check if this is a partially resolved type
                if self.is_partially_resolved_type(&full_match) {
                    // For partially resolved types, leverage existing prettyprint logic
                    // First try to look up recognized dimension names
                    let exponents = vec![params.mass_exp, params.length_exp, params.time_exp, params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp, params.luminous_intensity_exp, params.angle_exp];
                    let base_type = if let Some(dimension_info) = lookup_dimension_name(exponents.clone()) {
                        dimension_info.dimension_name.to_string()
                    } else {
                        // For unrecognized composite types, show dimension symbols (M, L, T)
                        generate_dimension_symbols(exponents)
                    };
                    
                    if is_inlay_hint {
                        // Return typedef format for partially resolved types too
                        format!("Quantity<{}, {}>", base_type, params.generic_type)
                    } else {
                        base_type
                    }
                } else {
                    if is_inlay_hint {
                        // Use the main pretty print function with verbose=false to get the unit literal
                        let full_output = pretty_print_quantity_type(
                            params.mass_exp, params.length_exp, params.time_exp,
                            params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                            params.luminous_intensity_exp, params.angle_exp,
                            params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                            &params.generic_type,
                            false, // Non-verbose mode for inlay hints
                            false, // Don't show type in brackets
                        );
                        
                        // The pretty_print_quantity_type already returns the correct format
                        // Just return it directly without double-formatting
                        full_output
                    } else {
                        // Use the new prettyprint API with configurable parameters
                        pretty_print_quantity_type(
                            params.mass_exp, params.length_exp, params.time_exp,
                            params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                            params.luminous_intensity_exp, params.angle_exp,
                            params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                            &params.generic_type,
                            verbose,
                            true, // show_type_in_brackets for LSP hover
                        )
                    }
                }
            } else {
                caps[0].to_string()
            }
        }).to_string()
    }

    /// Format the new Quantity type with Scale<...> and Dimension<...> structs
    fn format_new_quantity_type(&self, full_match: &str, verbose: bool, _unicode: bool, is_inlay_hint: bool) -> String {
        use whippyunits::print::prettyprint::pretty_print_quantity_type;
        
        
        // Parse the new format: Quantity<Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>>, Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>, T>
        if let Some(params) = self.parse_new_quantity_params(full_match) {
            if is_inlay_hint {
                // Use the main pretty print function with verbose=false to get the unit literal
                let full_output = pretty_print_quantity_type(
                    params.mass_exp, params.length_exp, params.time_exp,
                    params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                    params.luminous_intensity_exp, params.angle_exp,
                    params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                    &params.generic_type,
                    false, // Non-verbose mode for inlay hints
                    false, // Don't show type in brackets
                );
                
                
                // The pretty_print_quantity_type already returns the correct format
                // Just return it directly without double-formatting
                full_output
            } else {
                // Use the prettyprint API with configurable parameters
                let result = pretty_print_quantity_type(
                    params.mass_exp, params.length_exp, params.time_exp,
                    params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                    params.luminous_intensity_exp, params.angle_exp,
                    params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                    &params.generic_type,
                    verbose,
                    true, // show_type_in_brackets for LSP hover
                );
                result
            }
        } else {
            // If parsing fails, return the original
            full_match.to_string()
        }
    }

    /// Parse the new Quantity type format with Scale<...> and Dimension<...> structs
    fn parse_new_quantity_params(&self, quantity_type: &str) -> Option<QuantityParams> {
        
        // Parse Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>> directly from the full string
        // Handle both numbers and underscore placeholders
        let scale_re = Regex::new(r"Scale<_2<(-?\d+|_)>, _3<(-?\d+|_)>, _5<(-?\d+|_)>, _Pi<(-?\d+|_)>>").unwrap();
        let scale_captures = scale_re.captures(quantity_type)?;
        let scale_p2: i16 = self.parse_parameter(scale_captures.get(1)?.as_str());
        let scale_p3: i16 = self.parse_parameter(scale_captures.get(2)?.as_str());
        let scale_p5: i16 = self.parse_parameter(scale_captures.get(3)?.as_str());
        let scale_pi: i16 = self.parse_parameter(scale_captures.get(4)?.as_str());
        
        
        // Parse Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>> directly from the full string
        // Handle both numbers and underscore placeholders
        let dimension_re = Regex::new(r"Dimension<_M<(-?\d+|_)>, _L<(-?\d+|_)>, _T<(-?\d+|_)>, _I<(-?\d+|_)>, _Θ<(-?\d+|_)>, _N<(-?\d+|_)>, _J<(-?\d+|_)>, _A<(-?\d+|_)>>").unwrap();
        let dimension_captures = dimension_re.captures(quantity_type)?;
        let mass_exp: i16 = self.parse_parameter(dimension_captures.get(1)?.as_str());
        let length_exp: i16 = self.parse_parameter(dimension_captures.get(2)?.as_str());
        let time_exp: i16 = self.parse_parameter(dimension_captures.get(3)?.as_str());
        let electric_current_exp: i16 = self.parse_parameter(dimension_captures.get(4)?.as_str());
        let temperature_exp: i16 = self.parse_parameter(dimension_captures.get(5)?.as_str());
        let amount_of_substance_exp: i16 = self.parse_parameter(dimension_captures.get(6)?.as_str());
        let luminous_intensity_exp: i16 = self.parse_parameter(dimension_captures.get(7)?.as_str());
        let angle_exp: i16 = self.parse_parameter(dimension_captures.get(8)?.as_str());
        
        
        // Extract the generic type parameter (after the Dimension struct)
        // The format is: Quantity<Scale<...>, Dimension<...>, T>
        let generic_type = if let Some(dimension_end) = quantity_type.find("Dimension<") {
            // Find the end of the Dimension struct by counting brackets
            let mut bracket_count = 0;
            let mut i = dimension_end + 9; // Skip "Dimension<"
            let mut found_end = false;
            
            while i < quantity_type.len() {
                match quantity_type.chars().nth(i) {
                    Some('<') => bracket_count += 1,
                    Some('>') => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            found_end = true;
                            break;
                        }
                    },
                    _ => {}
                }
                i += 1;
            }
            
            if found_end {
                // Look for the comma after the Dimension struct
                if let Some(comma_pos) = quantity_type[i..].find(',') {
                    let after_comma = &quantity_type[i + comma_pos + 1..];
                    let trimmed = after_comma.trim();
                    
                    // Find the actual type parameter by looking for the first non-numeric, non-keyword part
                    // Skip over alignment and trait information like "align = 0x8, no Drop"
                    let parts: Vec<&str> = trimmed.split(',').collect();
                    let mut generic_type = "f64".to_string();
                    
                    for part in parts {
                        let cleaned = part.trim().trim_end_matches('>');
                        // Check if this looks like a type (not a number, not a keyword like "align", "no", "Drop")
                        if !cleaned.parse::<i16>().is_ok() && 
                           !cleaned.starts_with("align") && 
                           !cleaned.starts_with("no") && 
                           !cleaned.starts_with("Drop") &&
                           !cleaned.is_empty() {
                            generic_type = cleaned.to_string();
                            break;
                        }
                    }
                    
                    generic_type
                } else {
                    "f64".to_string()
                }
            } else {
                "f64".to_string()
            }
        } else {
            "f64".to_string()
        };
        
        Some(QuantityParams {
            mass_exp,
            length_exp,
            time_exp,
            electric_current_exp,
            temperature_exp,
            amount_of_substance_exp,
            luminous_intensity_exp,
            angle_exp,
            scale_p2,
            scale_p3,
            scale_p5,
            scale_pi,
            generic_type,
        })
    }

    /// Parse a parameter that could be a number or underscore placeholder
    fn parse_parameter(&self, param: &str) -> i16 {
        if param == "_" {
            i16::MIN // Unknown placeholder
        } else {
            param.parse().unwrap_or(0)
        }
    }

    /// Strip markdown formatting from text for cleaner raw display
    fn strip_markdown_formatting(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Remove code block markers
        result = result.replace("```rust", "").replace("```", "");
        
        // Remove other common markdown formatting
        result = result.replace("**", "").replace("*", "");
        result = result.replace("`", "");
        
        // Clean up any extra whitespace
        result.trim().to_string()
    }

    /// Clean up raw formatting by removing extra separators and vertical space
    fn clean_up_raw_formatting(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Remove extra "---" separators that appear before size/align information
        // Look for patterns like "---\n\nsize = " or "---\n\n---\nsize = "
        result = result.replace("---\n\n---", "---");
        
        // Remove multiple consecutive newlines (more than 2)
        while result.contains("\n\n\n") {
            result = result.replace("\n\n\n", "\n\n");
        }
        
        // Clean up any trailing whitespace
        result.trim().to_string()
    }

    
    fn parse_quantity_params(&self, quantity_type: &str) -> Option<QuantityParams> {
        // Extract const generic parameters from Quantity<...>
        let re = Regex::new(r"Quantity<([^>]*)>").unwrap();
        let captures = re.captures(quantity_type)?;
        let params_str = captures.get(1)?.as_str();
        
        // Parse comma-separated parameters, handling _ placeholders
        let params: Vec<Option<i16>> = params_str
            .split(',')
            .map(|s| {
                let s = s.trim();
                if s == "_" {
                    Some(i16::MIN) // Unknown placeholder
                } else if s == "9223372036854775807" {
                    Some(i16::MAX) // Unused value (original meaning)
                } else {
                    s.parse::<i16>().ok()
                }
            })
            .collect();
        
        // Extract the generic type parameter (last parameter if it's not a number)
        let generic_type = if params.len() > 0 {
            let last_param = params_str.split(',').last().unwrap_or("f64").trim();
            if last_param.parse::<i16>().is_err() && last_param != "_" && last_param != "9223372036854775807" {
                last_param.to_string()
            } else {
                "f64".to_string() // Default to f64
            }
        } else {
            "f64".to_string()
        };
        
        // Handle new 13-parameter format (8 dimension exponents + 4 aggregated scale parameters + 1 type)
        if params.len() >= 13 {
            Some(QuantityParams {
                mass_exp: params[0].unwrap_or(0),
                length_exp: params[1].unwrap_or(0),
                time_exp: params[2].unwrap_or(0),
                electric_current_exp: params[3].unwrap_or(0),
                temperature_exp: params[4].unwrap_or(0),
                amount_of_substance_exp: params[5].unwrap_or(0),
                luminous_intensity_exp: params[6].unwrap_or(0),
                angle_exp: params[7].unwrap_or(0),
                scale_p2: params[8].unwrap_or(0),
                scale_p3: params[9].unwrap_or(0),
                scale_p5: params[10].unwrap_or(0),
                scale_pi: params[11].unwrap_or(0),
                generic_type,
            })
        } else if params.len() >= 12 {
            // Handle 12-parameter format (8 dimension exponents + 4 scale parameters, no explicit type)
            Some(QuantityParams {
                mass_exp: params[0].unwrap_or(0),
                length_exp: params[1].unwrap_or(0),
                time_exp: params[2].unwrap_or(0),
                electric_current_exp: params[3].unwrap_or(0),
                temperature_exp: params[4].unwrap_or(0),
                amount_of_substance_exp: params[5].unwrap_or(0),
                luminous_intensity_exp: params[6].unwrap_or(0),
                angle_exp: params[7].unwrap_or(0),
                scale_p2: params[8].unwrap_or(0),
                scale_p3: params[9].unwrap_or(0),
                scale_p5: params[10].unwrap_or(0),
                scale_pi: params[11].unwrap_or(0),
                generic_type,
            })
        } else if params.len() >= 8 {
            // Handle 8-parameter format (8 dimension exponents only, no scale parameters)
            Some(QuantityParams {
                mass_exp: params[0].unwrap_or(0),
                length_exp: params[1].unwrap_or(0),
                time_exp: params[2].unwrap_or(0),
                electric_current_exp: params[3].unwrap_or(0),
                temperature_exp: params[4].unwrap_or(0),
                amount_of_substance_exp: params[5].unwrap_or(0),
                luminous_intensity_exp: params[6].unwrap_or(0),
                angle_exp: params[7].unwrap_or(0),
                scale_p2: 0,
                scale_p3: 0,
                scale_p5: 0,
                scale_pi: 0,
                generic_type,
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
            
            if params.len() >= 14 {
                // Check if any dimension has a non-zero exponent but sentinel scale values
                // This indicates a partially resolved type
                
                // Check dimension exponents (params[0-7])
                for i in 0..8 {
                    let exp = params[i].parse::<i16>().unwrap_or(0);
                    if exp != 0 {
                        // Check if any scale parameter has sentinel value
                        for j in 8..13 { // scale parameters are at indices 8-12
                            let scale = if params[j] == "_" { i16::MIN } else { params[j].parse::<i16>().unwrap_or(i16::MAX) };
                            if scale == i16::MIN {
                                return true; // Has exponent but unknown scale
                            }
                        }
                    }
                }
            } else if params.len() >= 12 {
                // 12-parameter format (8 dimension exponents + 4 scale parameters)
                // Check if any dimension has a non-zero exponent but sentinel scale values
                for i in 0..8 {
                    let exp = params[i].parse::<i16>().unwrap_or(0);
                    if exp != 0 {
                        // Check if any scale parameter has sentinel value (indices 8-11)
                        for j in 8..12 {
                            let scale = if params[j] == "_" { i16::MIN } else { params[j].parse::<i16>().unwrap_or(i16::MAX) };
                            if scale == i16::MIN {
                                return true; // Has exponent but unknown scale
                            }
                        }
                    }
                }
            } else if params.len() >= 8 {
                // 8-parameter format (8 dimension exponents only, no scale parameters)
                // Check if any dimension has a non-zero exponent
                for i in 0..8 {
                    let exp = params[i].parse::<i16>().unwrap_or(0);
                    if exp != 0 {
                        // For 8-parameter format, we don't have scale parameters to check
                        // This is a fully resolved type
                        return false;
                    }
                }
            }
        }
        
        false // Fully resolved type
    }

}

#[derive(Debug)]
struct QuantityParams {
    mass_exp: i16,
    length_exp: i16,
    time_exp: i16,
    electric_current_exp: i16,
    temperature_exp: i16,
    amount_of_substance_exp: i16,
    luminous_intensity_exp: i16,
    angle_exp: i16,
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_pi: i16,
    generic_type: String,
}
