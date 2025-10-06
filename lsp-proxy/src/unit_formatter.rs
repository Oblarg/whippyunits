

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
            result.push_str(&format!("\n\nRaw:\n\n```rust\n{}\n```", text));
        }
        
        result
    }

    /// Format whippyunits types in text with original text for Raw section
    pub fn format_types_with_original(&self, text: &str, config: &DisplayConfig, original_text: &str) -> String {
        // Check if this contains a generic type definition that we passed through unchanged
        let contains_generic_definition = text.contains("T = f64") || original_text.contains("T = f64");
        
        // If we don't need raw section or it's a generic definition, just format normally
        if !config.include_raw || contains_generic_definition {
            return self.format_quantity_types(text, config.verbose, config.unicode, false);
        }
        
        // Work within the existing markdown structure
        // Find the first code block and replace its content
        if let Some(code_start) = original_text.find("```rust") {
            let after_code_start = &original_text[code_start + 7..]; // Skip "```rust"
            if let Some(code_end) = after_code_start.find("```") {
                let code_content = &after_code_start[..code_end];
                let after_code = &after_code_start[code_end..];
                
                // Format the code content
                let formatted_content = self.format_quantity_types(code_content, config.verbose, config.unicode, false);
                
                // Extract raw type from the original code content
                let raw_type = self.extract_raw_type_from_hover(&format!("let result: {}", code_content));
                
                // Replace the content in the existing code block
                let before_code = &original_text[..code_start + 7]; // Include "```rust"
                let after_code_block = &after_code_start[code_end + 3..]; // Skip closing ```
                
                let result = if !raw_type.is_empty() {
                    // Insert raw section after the first --- separator
                    if let Some(separator_pos) = after_code_block.find("---") {
                        let after_separator = &after_code_block[separator_pos..];
                        format!("{}\n{}\n\n---\nRaw:\n\n{}\n{}", 
                               before_code, formatted_content.trim(), raw_type, after_separator)
                    } else {
                        format!("{}\n{}\n\n---\nRaw:\n\n{}\n", 
                               before_code, formatted_content.trim(), raw_type)
                    }
                } else {
                    format!("{}\n{}\n{}", before_code, formatted_content.trim(), after_code_block)
                };
                
                return result;
            }
        }
        
        // Fallback to normal formatting if we can't parse the markdown structure
        self.format_quantity_types(text, config.verbose, config.unicode, false)
    }

    /// Format whippyunits types for inlay hints (compact format)
    pub fn format_types_inlay_hint(&self, text: &str) -> String {
        self.format_quantity_types(text, false, true, true)
    }

    /// Core method to format Quantity types with configurable parameters
    fn format_quantity_types(&self, text: &str, verbose: bool, unicode: bool, is_inlay_hint: bool) -> String {
        
        use whippyunits::print::prettyprint::{pretty_print_quantity_type, generate_dimension_symbols};
        use whippyunits::print::name_lookup::lookup_dimension_name;
        
        // Handle the new format with Scale and Dimension structs (both full and truncated)
        if text.contains("Scale") && text.contains("Dimension") {
            // Use a more sophisticated approach to find and replace each Quantity type
            // We'll manually find the start and end of each Quantity type by counting brackets
            let mut result = String::new();
            let mut i = 0;
            
            while i < text.len() {
                if let Some(start) = text[i..].find("Quantity<Scale") {
                    let start_pos = i + start;
                    
                    // Ensure start_pos is within bounds
                    if start_pos >= text.len() {
                        result.push_str(&text[i..]);
                        break;
                    }
                    
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
                        // The bracket counting found the end of the Quantity type
                        let actual_end = j;
                        
                        // Ensure we don't go beyond string bounds
                        let end_pos = std::cmp::min(actual_end + 2, text.len());
                        
                        // Extract the quantity type including all the '>' characters
                        let quantity_type = &text[start_pos..end_pos];
                        let formatted = self.format_new_quantity_type(quantity_type, verbose, unicode, is_inlay_hint);
                        result.push_str(&text[i..start_pos]);
                        result.push_str(&formatted);
                        i = end_pos;
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
        
        // If we reach here, no new format was found, return original text
        text.to_string()
    }

    /// Format the new Quantity type with Scale<...> and Dimension<...> structs
    fn format_new_quantity_type(&self, full_match: &str, verbose: bool, _unicode: bool, is_inlay_hint: bool) -> String {
        use whippyunits::print::prettyprint::pretty_print_quantity_type;
        
        // Check if this is a generic type definition (contains parameter names like Scale, Dimension, T)
        // rather than a concrete instantiation with actual values
        if self.is_generic_type_definition(full_match) {
            // Pass through generic definitions unchanged
            return full_match.to_string();
        }
        
        // Parse the new format: Quantity<Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>>, Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>, T>
        if let Some(params) = self.parse_new_quantity_params(full_match) {
            // Check if this is a dimensionless quantity (all dimensions are zero)
            if params.mass_exp == 0 && params.length_exp == 0 && params.time_exp == 0 && 
               params.electric_current_exp == 0 && params.temperature_exp == 0 && 
               params.amount_of_substance_exp == 0 && params.luminous_intensity_exp == 0 && 
               params.angle_exp == 0 && params.scale_p2 == 0 && params.scale_p3 == 0 && 
               params.scale_p5 == 0 && params.scale_pi == 0 {
                // Format as dimensionless quantity
                return format!("Quantity<1, {}>", params.generic_type);
            }
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
                    false, // show_type_in_brackets = false for pretty printer
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
        // Parse Scale parameters - handle both full format and truncated format
        let (scale_p2, scale_p3, scale_p5, scale_pi) = if quantity_type.contains("Scale<_2<") {
            // Full format: Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>>
            self.parse_scale_full_format(quantity_type)?
        } else if quantity_type.contains("Scale,") {
            // Truncated format: Scale, or Scale> or Scale, Dimension (all parameters default to 0)
            (0, 0, 0, 0)
        } else {
            // Unknown format
            return None;
        };
        
        // Parse Dimension parameters - handle both full format and truncated format
        let (mass_exp, length_exp, time_exp, electric_current_exp, temperature_exp, amount_of_substance_exp, luminous_intensity_exp, angle_exp) = 
            if quantity_type.contains("Dimension<_M<") && quantity_type.contains("_A<") && !quantity_type.contains("_M<0>") {
                // Full format: Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>
                self.parse_dimension_full_format(quantity_type)?
            } else if quantity_type.contains("Dimension,") || quantity_type.contains("Dimension>") {
                // Fully defaulted Dimension (dimensionless): Dimension, T or Dimension> T
                (0, 0, 0, 0, 0, 0, 0, 0)
            } else {
                // Truncated format: parse only the non-zero parameters
                // Look for patterns like Dimension<_M<0>, _L<1>> (only non-zero parameters are shown)
                self.parse_dimension_truncated_format(quantity_type)
            };
        
        
        // For resolved Scale/Dimension format, the generic type defaults to f64
        // (extract_generic_type is only for raw generic parameter names)
        let generic_type = "f64".to_string();
        
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

    /// Parse full Scale format: Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>>
    /// Handles cases where some parameters may be missing (e.g., only _2, _3, _5 without _Pi)
    fn parse_scale_full_format(&self, quantity_type: &str) -> Option<(i16, i16, i16, i16)> {
        let scale_start = quantity_type.find("Scale<_2<")?;
        let scale_content = &quantity_type[scale_start + 6..]; // Skip "Scale<"
        
        // Find the end of the Scale struct
        let scale_end = self.find_matching_bracket(scale_content, 0)?;
        let scale_params = &scale_content[..scale_end];
        
        // Parse individual parameters: _2<P2>, _3<P3>, _5<P5>, _Pi<PI>
        // Handle missing parameters by defaulting to 0
        let p2 = self.parse_scale_param(scale_params, "_2<").unwrap_or(0);
        let p3 = self.parse_scale_param(scale_params, "_3<").unwrap_or(0);
        let p5 = self.parse_scale_param(scale_params, "_5<").unwrap_or(0);
        let pi = self.parse_scale_param(scale_params, "_Pi<").unwrap_or(0);
        
        Some((p2, p3, p5, pi))
    }

    /// Parse full Dimension format: Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>
    fn parse_dimension_full_format(&self, quantity_type: &str) -> Option<(i16, i16, i16, i16, i16, i16, i16, i16)> {
        let dimension_start = quantity_type.find("Dimension<_M<")?;
        let dimension_content = &quantity_type[dimension_start + 9..]; // Skip "Dimension<"
        
        // Find the end of the Dimension struct by looking for the pattern ">, f64" or ">, T"
        // This is more reliable than bracket counting for this specific case
        let dimension_end = if let Some(pos) = dimension_content.find(">, f64") {
            pos + 1 // Include the '>'
        } else if let Some(pos) = dimension_content.find(">, ") {
            pos + 1 // Include the '>'
        } else {
            // Fallback to bracket counting
            self.find_matching_bracket(&dimension_content[1..], 0)? + 1
        };
        let dimension_params = &dimension_content[..dimension_end];
        
        // Parse individual parameters
        let mass = self.parse_dimension_param(dimension_params, "_M<")?;
        let length = self.parse_dimension_param(dimension_params, "_L<")?;
        let time = self.parse_dimension_param(dimension_params, "_T<")?;
        let current = self.parse_dimension_param(dimension_params, "_I<")?;
        let temp = self.parse_dimension_param(dimension_params, "_Θ<")?;
        let amount = self.parse_dimension_param(dimension_params, "_N<")?;
        let lum = self.parse_dimension_param(dimension_params, "_J<")?;
        let angle = self.parse_dimension_param(dimension_params, "_A<")?;
        
        Some((mass, length, time, current, temp, amount, lum, angle))
    }

    /// Parse truncated Dimension format: Dimension<_M<0>, _L<1>> (only non-zero parameters are shown)
    fn parse_dimension_truncated_format(&self, quantity_type: &str) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
        let mut mass_exp = 0;
        let mut length_exp = 0;
        let mut time_exp = 0;
        let mut electric_current_exp = 0;
        let mut temperature_exp = 0;
        let mut amount_of_substance_exp = 0;
        let mut luminous_intensity_exp = 0;
        let mut angle_exp = 0;
        
        // Parse individual dimension parameters that are present
        if let Some(value) = self.parse_dimension_param(quantity_type, "_M<") {
            mass_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_L<") {
            length_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_T<") {
            time_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_I<") {
            electric_current_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_Θ<") {
            temperature_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_N<") {
            amount_of_substance_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_J<") {
            luminous_intensity_exp = value;
        }
        if let Some(value) = self.parse_dimension_param(quantity_type, "_A<") {
            angle_exp = value;
        }
        
        (mass_exp, length_exp, time_exp, electric_current_exp, temperature_exp, amount_of_substance_exp, luminous_intensity_exp, angle_exp)
    }

    /// Parse a scale parameter like "_2<5>" and return the value
    fn parse_scale_param(&self, content: &str, prefix: &str) -> Option<i16> {
        let start = content.find(prefix)?;
        let param_start = start + prefix.len();
        let param_end = content[param_start..].find('>')?;
        let param_value = &content[param_start..param_start + param_end];
        Some(self.parse_parameter(param_value))
    }

    /// Parse a dimension parameter like "_M<1>" and return the value
    fn parse_dimension_param(&self, content: &str, prefix: &str) -> Option<i16> {
        let start = content.find(prefix)?;
        let param_start = start + prefix.len();
        let param_end = content[param_start..].find('>')?;
        let param_value = &content[param_start..param_start + param_end];
        let result = self.parse_parameter(param_value);
        Some(result)
    }

    /// Find the matching closing bracket for a given opening bracket
    fn find_matching_bracket(&self, content: &str, start_pos: usize) -> Option<usize> {
        let mut depth = 1;
        let mut i = start_pos;
        
        while i < content.len() {
            match content.chars().nth(i) {
                Some('<') => depth += 1,
                Some('>') => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                },
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Find the end of a Quantity type by looking for the closing > after the generic type parameter
    fn find_quantity_end(&self, content: &str, start_pos: usize) -> Option<usize> {
        let mut depth = 1;
        let mut i = start_pos;
        let mut found_comma = false;
        
        while i < content.len() {
            match content.chars().nth(i) {
                Some('<') => depth += 1,
                Some('>') => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                },
                Some(',') if depth == 1 => {
                    // Found a comma at the top level - this should be the separator before the generic type
                    found_comma = true;
                },
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Find the end of the Dimension struct by looking for the closing > after all parameters
    fn find_dimension_end(&self, content: &str) -> Option<usize> {
        let mut depth = 1;
        let mut i = 0;
        
        while i < content.len() {
            match content.chars().nth(i) {
                Some('<') => depth += 1,
                Some('>') => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(i);
                    }
                },
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Extract the generic type parameter from a Quantity type string
    fn extract_generic_type(&self, quantity_type: &str) -> String {
        if let Some(dimension_start) = quantity_type.find("Dimension<") {
            // Find the end of the Dimension struct
            let dimension_content = &quantity_type[dimension_start + 9..]; // Skip "Dimension<"
            if let Some(dimension_end) = self.find_matching_bracket(dimension_content, 0) {
                let after_dimension = &quantity_type[dimension_start + 9 + dimension_end + 1..];
                return self.find_type_parameter(after_dimension);
            }
        } else if let Some(dimension_start) = quantity_type.find("Dimension,") {
            // Handle Dimension, T format (fully defaulted Dimension)
            let after_dimension = &quantity_type[dimension_start + 9..]; // Skip "Dimension,"
            return self.find_type_parameter(after_dimension);
        }
        
        "f64".to_string()
    }

    /// Find the type parameter in a string, skipping alignment and trait information
    fn find_type_parameter(&self, content: &str) -> String {
        let parts: Vec<&str> = content.split(',').collect();
        
        for part in parts {
            let cleaned = part.trim().trim_end_matches('>');
            // Check if this looks like a type (not a number, not a keyword like "align", "no", "Drop")
            if !cleaned.parse::<i16>().is_ok() && 
               !cleaned.starts_with("align") && 
               !cleaned.starts_with("no") && 
               !cleaned.starts_with("Drop") &&
               !cleaned.starts_with("size") &&
               !cleaned.starts_with("0x") &&
               !cleaned.is_empty() {
                return cleaned.to_string();
            }
        }
        
        "f64".to_string()
    }

    /// Parse a parameter that could be a number or underscore placeholder
    fn parse_parameter(&self, param: &str) -> i16 {
        if param == "_" {
            i16::MIN // Unknown placeholder
        } else {
            param.parse().unwrap_or(0)
        }
    }




    

    /// Extract just the raw type information from hover content
    fn extract_raw_type_from_hover(&self, hover_text: &str) -> String {
        // Look for the pattern: let [mut] [var]: Quantity<...>
        if let Some(start) = hover_text.find(": Quantity<") {
            // Find the start of the variable declaration
            let mut var_start = start;
            while var_start > 0 && !hover_text[var_start..var_start+1].chars().next().unwrap().is_whitespace() {
                var_start -= 1;
            }
            if var_start > 0 && hover_text[var_start..var_start+1].chars().next().unwrap().is_whitespace() {
                var_start += 1; // Skip the whitespace
            }
            
            // Find the end of the type declaration (before any size/align info)
            let type_start = start + ": ".len();
            let after_type = &hover_text[type_start..];
            
            // Find the end of the Quantity type by looking for the closing >
            let mut bracket_count = 0;
            let mut end_pos = 0;
            
            for (i, ch) in after_type.char_indices() {
                match ch {
                    '<' => bracket_count += 1,
                    '>' => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            end_pos = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            if end_pos > 0 {
                let raw_type = &after_type[..end_pos];
                let var_name = &hover_text[var_start..start];
                return format!("{}: {}", var_name, raw_type);
            }
        }
        
        String::new()
    }

    /// Check if this is a generic type definition rather than a concrete instantiation
    /// Specifically looks for the pattern "T = f64" which indicates a generic type definition
    fn is_generic_type_definition(&self, text: &str) -> bool {
        // Only detect the specific case where we have "T = f64" which reliably indicates
        // a generic type definition like "Quantity<Scale, Dimension, T = f64>"
        text.contains("T = f64")
    }

    /// Check if a type is partially resolved (has sentinel values for dimensions that should be resolved)
    fn is_partially_resolved_type(&self, text: &str) -> bool {
        // Check for new Scale/Dimension format first
        if text.contains("Scale") && text.contains("Dimension") {
            // Parse the new format to check for partially resolved types
            if let Some(params) = self.parse_new_quantity_params(text) {
                // Check if any dimension has a non-zero exponent but sentinel scale values
                // This indicates a partially resolved type
                
                // Check if any dimension exponent is non-zero
                let has_non_zero_dimension = params.mass_exp != 0 || params.length_exp != 0 || 
                                           params.time_exp != 0 || params.electric_current_exp != 0 ||
                                           params.temperature_exp != 0 || params.amount_of_substance_exp != 0 ||
                                           params.luminous_intensity_exp != 0 || params.angle_exp != 0;
                
                if has_non_zero_dimension {
                    // Check if any scale parameter has sentinel value (i16::MIN indicates unknown)
                    if params.scale_p2 == i16::MIN || params.scale_p3 == i16::MIN || 
                       params.scale_p5 == i16::MIN || params.scale_pi == i16::MIN {
                        return true; // Has dimension but unknown scale
                    }
                }
            }
            return false; // Fully resolved new format type
        }
        
        // For old format, check if it contains underscore placeholders
        if text.contains("Quantity<") && text.contains("_") {
            // Simple check: if it contains underscores, it's likely partially resolved
            return true;
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
