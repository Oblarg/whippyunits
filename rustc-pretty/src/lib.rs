use regex::Regex;
use anyhow::Result;
use log::{debug, warn};

use whippyunits::print::prettyprint::pretty_print_quantity_type;

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

/// Pretty printer for rustc output with whippyunits type formatting
pub struct RustcPrettyPrinter {
    type_converter: WhippyUnitsTypeConverter,
    display_config: DisplayConfig,
    quantity_regex: Regex,
    error_regex: Regex,
    warning_regex: Regex,
    note_regex: Regex,
}

impl RustcPrettyPrinter {
    pub fn new() -> Self {
        Self::with_config(DisplayConfig::default())
    }

    pub fn with_config(display_config: DisplayConfig) -> Self {
        Self {
            type_converter: WhippyUnitsTypeConverter::new(),
            display_config,
            quantity_regex: Regex::new(r"Quantity<([^>]+)>").unwrap(),
            error_regex: Regex::new(r"^error\[([^\]]+)\]: (.+)$").unwrap(),
            warning_regex: Regex::new(r"^warning\[([^\]]+)\]: (.+)$").unwrap(),
            note_regex: Regex::new(r"^(\s+)note: (.+)$").unwrap(),
        }
    }

    /// Process a complete rustc output string
    pub fn process_rustc_output(&mut self, output: &str) -> Result<String> {
        let lines: Vec<&str> = output.lines().collect();
        let mut processed_lines = Vec::new();

        for line in lines {
            let processed = self.process_line(line)?;
            processed_lines.push(processed);
        }

        Ok(processed_lines.join("\n"))
    }

    /// Process a single line of rustc output
    pub fn process_line(&mut self, line: &str) -> Result<String> {
        // Check if this line contains whippyunits types
        if self.quantity_regex.is_match(line) {
            debug!("Processing line with whippyunits types: {}", line);
            
            // Apply type conversion
            let processed = self.type_converter.convert_types_in_text_with_config(
                line, 
                &self.display_config
            );
            
            // If we made changes, log them
            if processed != line {
                debug!("Transformed: {} -> {}", line, processed);
            }
            
            Ok(processed)
        } else {
            // No whippyunits types, pass through unchanged
            Ok(line.to_string())
        }
    }

    /// Check if a line is an error message
    pub fn is_error_line(&self, line: &str) -> bool {
        self.error_regex.is_match(line)
    }

    /// Check if a line is a warning message
    pub fn is_warning_line(&self, line: &str) -> bool {
        self.warning_regex.is_match(line)
    }

    /// Check if a line is a note message
    pub fn is_note_line(&self, line: &str) -> bool {
        self.note_regex.is_match(line)
    }
}

/// Type converter for whippyunits types using the prettyprint API
#[derive(Clone)]
pub struct WhippyUnitsTypeConverter;

impl WhippyUnitsTypeConverter {
    pub fn new() -> Self {
        Self
    }

    /// Convert types in text with display configuration
    pub fn convert_types_in_text_with_config(&self, text: &str, config: &DisplayConfig) -> String {
        let mut result = if config.verbose {
            // In verbose mode, convert Quantity types to readable format
            self.convert_quantity_types_verbose(text)
        } else {
            // In clean mode, convert to unit display
            self.convert_quantity_types_clean(text, config.unicode)
        };
        
        // Add raw type if requested
        if config.include_raw && result != text {
            result.push_str(&format!("\n    Raw: {}", text));
        }
        
        result
    }

    /// Convert Quantity types to verbose const generic display
    fn convert_quantity_types_verbose(&self, text: &str) -> String {
        let quantity_regex = Regex::new(r"Quantity<([^>]+)>").unwrap();
        quantity_regex.replace_all(text, |caps: &regex::Captures| {
            let full_match = caps[0].to_string();
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i16")
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i16") || 
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
                // This is a type definition or const generic context, treat as unknown
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
                // For rustc output, we always show type info without values
                pretty_print_quantity_type(
                    params.mass_exp, params.length_exp, params.time_exp,
                    params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                    params.luminous_intensity_exp, params.angle_exp,
                    params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                    &params.generic_type,
                    true, // verbose
                    true, // show_type_in_brackets for rustc output
                )
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
            
            // Check if this is a type definition (contains parameter names like "const MASS_EXPONENT: i16")
            let is_const_generic_context = full_match.contains("const") || 
                                         full_match.contains("i16") || 
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
                // This is a type definition or const generic context, treat as unknown
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
                    use whippyunits::print::name_lookup::lookup_dimension_name;
                    use whippyunits::print::prettyprint::generate_dimension_symbols;
                    
                    let exponents = vec![params.mass_exp, params.length_exp, params.time_exp, params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp, params.luminous_intensity_exp, params.angle_exp];
                    if let Some(dimension_info) = lookup_dimension_name(exponents.clone()) {
                        dimension_info.dimension_name.to_string()
                    } else {
                        // For unrecognized composite types, show dimension symbols (M, L, T)
                        generate_dimension_symbols(exponents)
                    }
                } else {
                    // Use the new prettyprint API with verbose=false
                    pretty_print_quantity_type(
                        params.mass_exp, params.length_exp, params.time_exp,
                        params.electric_current_exp, params.temperature_exp, params.amount_of_substance_exp,
                        params.luminous_intensity_exp, params.angle_exp,
                        params.scale_p2, params.scale_p3, params.scale_p5, params.scale_pi,
                        &params.generic_type,
                        false, // not verbose
                        false, // don't show type in brackets for clean mode
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_conversion() {
        let converter = WhippyUnitsTypeConverter::new();
        let converted = converter.convert_types_in_text_with_config(
            "Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>",
            &DisplayConfig::default()
        );
        println!("Converted output: '{}'", converted);
        assert!(converted.contains("m"));
    }

    #[test]
    fn test_rustc_output_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        let rustc_output = r#"error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
5 |     let x: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807> = 5.0;
  |         ^   expected `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>`, found `{float}`
  |
  = note: expected struct `Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807>`
             found type `{float}`"#;
        
        let processed = printer.process_rustc_output(rustc_output).unwrap();
        println!("Processed output:\n{}", processed);
        
        // Should contain pretty-printed types
        assert!(processed.contains("m"));
        assert!(!processed.contains("9223372036854775807"));
    }

    #[test]
    fn test_line_processing() {
        let mut printer = RustcPrettyPrinter::new();
        
        let line = "    let x: Quantity<0, 9223372036854775807, 1, 0, 0, 9223372036854775807, 9223372036854775807, 9223372036854775807> = 5.0;";
        let processed = printer.process_line(line).unwrap();
        
        println!("Original: {}", line);
        println!("Processed: {}", processed);
        
        // Should contain pretty-printed type
        assert!(processed.contains("m"));
        assert!(!processed.contains("9223372036854775807"));
    }
}
