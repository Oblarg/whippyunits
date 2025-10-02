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
            result.push_str(&format!("\n\nRaw: {}", original_text));
        }
        
        result
    }

    /// Format whippyunits types for inlay hints (compact format)
    pub fn format_types_inlay_hint(&self, text: &str) -> String {
        self.format_quantity_types(text, false, true, true)
    }

    /// Core method to format Quantity types with configurable parameters
    fn format_quantity_types(&self, text: &str, verbose: bool, unicode: bool, is_inlay_hint: bool) -> String {
        use whippyunits::print::prettyprint::{pretty_print_quantity_type, pretty_print_quantity_inlay_hint, generate_dimension_symbols};
        use whippyunits::print::name_lookup::lookup_dimension_name;
        
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
                // This is a type definition or const generic context, treat as unknown (all i16::MIN placeholders)
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
                        // Use the ultra-terse inlay hint API to get the unit literal
                        let unit_literal = pretty_print_quantity_inlay_hint(
                            params.mass_exp, params.length_exp, params.time_exp,
                            params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                            params.luminous_intensity_exp, params.angle_exp,
                            params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                        );
                        
                        // Return typedef format (e.g., "Quantity<mN, f64>", "Quantity<kg, i32>")
                        format!("Quantity<{}, {}>", unit_literal, params.generic_type)
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
            // Handle legacy 3-dimension format (backward compatibility)
            Some(QuantityParams {
                mass_exp: params[0].unwrap_or(0),
                length_exp: params[2].unwrap_or(0),
                time_exp: params[4].unwrap_or(0),
                electric_current_exp: 0,
                temperature_exp: 0,
                amount_of_substance_exp: 0,
                luminous_intensity_exp: 0,
                angle_exp: 0,
                scale_p2: params[5].unwrap_or(0),
                scale_p3: params[6].unwrap_or(0),
                scale_p5: params[7].unwrap_or(0),
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
                // Legacy format - check old structure
                let mass_exp = params[0].parse::<i16>().unwrap_or(0);
                let mass_scale = if params[1] == "_" { i16::MIN } else { params[1].parse::<i16>().unwrap_or(i16::MAX) };
                if mass_exp != 0 && mass_scale == i16::MIN {
                    return true;
                }
                
                let length_exp = params[2].parse::<i16>().unwrap_or(0);
                let length_scale = if params[3] == "_" { i16::MIN } else { params[3].parse::<i16>().unwrap_or(i16::MAX) };
                if length_exp != 0 && length_scale == i16::MIN {
                    return true;
                }
                
                let time_exp = params[4].parse::<i16>().unwrap_or(0);
                let time_p2 = if params[5] == "_" { i16::MIN } else { params[5].parse::<i16>().unwrap_or(i16::MAX) };
                let time_p3 = if params[6] == "_" { i16::MIN } else { params[6].parse::<i16>().unwrap_or(i16::MAX) };
                let time_p5 = if params[7] == "_" { i16::MIN } else { params[7].parse::<i16>().unwrap_or(i16::MAX) };
                if time_exp != 0 && (time_p2 == i16::MIN || time_p3 == i16::MIN || time_p5 == i16::MIN) {
                    return true;
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
