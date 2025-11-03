use syn::{parse_str, TypePath};
use whippyunits_core::{
    dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
};

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
            result.push_str(&format!("\n\nRaw:\n\n{}", text));
        }

        result
    }

    /// Format whippyunits types in text with original text for Raw section
    pub fn format_types_with_original(
        &self,
        text: &str,
        config: &DisplayConfig,
        original_text: &str,
    ) -> String {
        // Check if this contains a generic type definition that we passed through unchanged
        let contains_generic_definition =
            text.contains("T = f64") || original_text.contains("T = f64");

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

                // Format the code content
                let formatted_content =
                    self.format_quantity_types(code_content, config.verbose, config.unicode, false);

                // Extract raw type from the original code content
                let raw_type = self.extract_raw_type_from_hover(code_content);

                // Replace the content in the existing code block
                let before_code = &original_text[..code_start + 7]; // Include "```rust"
                let after_code_block = &after_code_start[code_end + 3..]; // Skip closing ```

                let result = if !raw_type.is_empty() {
                    // Insert raw section after the first --- separator
                    if let Some(separator_pos) = after_code_block.find("---") {
                        let after_separator = &after_code_block[separator_pos..];
                        format!(
                            "{}\n{}\n```\n\n---\nRaw:\n\n```rust\n{}\n```\n{}",
                            before_code,
                            formatted_content.trim(),
                            raw_type,
                            after_separator
                        )
                    } else {
                        format!(
                            "{}\n{}\n```\n\n---\nRaw:\n\n```rust\n{}\n```\n",
                            before_code,
                            formatted_content.trim(),
                            raw_type
                        )
                    }
                } else {
                    format!(
                        "{}\n{}\n```{}",
                        before_code,
                        formatted_content.trim(),
                        after_code_block
                    )
                };

                return result;
            }
        }

        // Fallback to normal formatting if we can't parse the markdown structure
        self.format_quantity_types(text, config.verbose, config.unicode, false)
    }

    /// Format whippyunits types for inlay hints (compact format)
    pub fn format_types_inlay_hint(&self, text: &str, config: &DisplayConfig) -> String {
        self.format_quantity_types(text, config.verbose, config.unicode, true)
    }

    /// Core method to format Quantity types with configurable parameters
    fn format_quantity_types(
        &self,
        text: &str,
        verbose: bool,
        unicode: bool,
        is_inlay_hint: bool,
    ) -> String {
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
                            }
                            _ => {}
                        }
                        j += 1;
                    }

                    if found_end || (bracket_count == 1 && j >= text.len()) {
                        // The bracket counting found the end of the Quantity type
                        // or we've reached the end of the string with bracket_count = 1 (no generic type parameter)
                        let actual_end = if found_end { j } else { text.len() };

                        // Ensure we don't go beyond string bounds
                        let end_pos = std::cmp::min(actual_end + 2, text.len());

                        // Extract the quantity type including all the '>' characters
                        let quantity_type = &text[start_pos..end_pos];
                        let formatted = self.format_new_quantity_type(
                            quantity_type,
                            verbose,
                            unicode,
                            is_inlay_hint,
                        );
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
    fn format_new_quantity_type(
        &self,
        full_match: &str,
        verbose: bool,
        _unicode: bool,
        is_inlay_hint: bool,
    ) -> String {
        use whippyunits::print::prettyprint::pretty_print_quantity_type;

        // Check if this is a generic type definition (contains parameter names like Scale, Dimension, T)
        // rather than a concrete instantiation with actual values
        if self.is_generic_type_definition(full_match) {
            // Pass through generic definitions unchanged
            return full_match.to_string();
        }

        // Parse the new format: Quantity<Scale<_2<P2>, _3<P3>, _5<P5>, _Pi<PI>>, Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>, T>
        if let Some(params) = self.parse_new_quantity_params(full_match) {
            // Check if this is a wholly unresolved type (all parameters are sentinel values)
            let all_dimensions_unresolved = params.dimensions.0.iter().all(|&exp| exp == i16::MIN);
            let all_scales_unresolved = params.scale.0.iter().all(|&exp| exp == i16::MIN);

            // Get Brand name for passing to prettyprint function
            let brand_name = params.brand.as_deref();

            if all_dimensions_unresolved && all_scales_unresolved {
                // Format as wholly unresolved type
                let mut result = format!("Quantity<?, {}>", params.generic_type);
                if let Some(brand) = brand_name {
                    if brand != "()" {
                        result = format!("{}, {}>", &result[..result.len() - 1], brand);
                    }
                }
                return result;
            }

            // Check if this is a dimensionless quantity (all dimensions are zero)
            if params.dimensions == DynDimensionExponents::ZERO
                && params.scale == ScaleExponents::IDENTITY
            {
                // Format as dimensionless quantity
                let mut result = format!("Quantity<1, {}>", params.generic_type);
                if let Some(brand) = brand_name {
                    if brand != "()" {
                        result = format!("{}, {}>", &result[..result.len() - 1], brand);
                    }
                }
                return result;
            }
            if is_inlay_hint {
                // Use the main pretty print function with verbose=false to get the unit literal
                let full_output = pretty_print_quantity_type(
                    params.dimensions,
                    params.scale,
                    &params.generic_type,
                    false, // Non-verbose mode for inlay hints
                    false, // Don't show type in brackets
                    brand_name,
                );

                // Check if the pretty print function returned just "?" for wholly unresolved types
                if full_output == "?" {
                    let mut result = format!("Quantity<?, {}>", params.generic_type);
                    if let Some(brand) = brand_name {
                        if brand != "()" {
                            result = format!("{}, {}>", &result[..result.len() - 1], brand);
                        }
                    }
                    return result;
                }

                full_output
            } else {
                // Use the prettyprint API with configurable parameters
                let result = pretty_print_quantity_type(
                    params.dimensions,
                    params.scale,
                    &params.generic_type,
                    verbose,
                    false, // show_type_in_brackets = false for pretty printer
                    brand_name,
                );

                // Check if the pretty print function returned just "?" for wholly unresolved types
                if result == "?" {
                    let mut formatted = format!("Quantity<?, {}>", params.generic_type);
                    if let Some(brand) = brand_name {
                        if brand != "()" {
                            formatted =
                                format!("{}, {}>", &formatted[..formatted.len() - 1], brand);
                        }
                    }
                    return formatted;
                }

                result
            }
        } else {
            // If parsing fails, return the original
            full_match.to_string()
        }
    }

    /// Parse the new Quantity type format with Scale<...> and Dimension<...> structs
    fn parse_new_quantity_params(&self, quantity_type: &str) -> Option<QuantityParams> {
        // Parse Scale parameters - handle all possible combinations of defaulted parameters
        let scale = if quantity_type.contains("Scale<") {
            // Any Scale format with parameters - handle all combinations of defaulted values
            let (p2, p3, p5, pi) = self.parse_scale_general_format(quantity_type)?;
            ScaleExponents([p2, p3, p5, pi])
        } else if quantity_type.contains("Scale,") {
            // Truncated format: Scale, or Scale> or Scale, Dimension (all parameters default to 0)
            ScaleExponents::IDENTITY
        } else {
            // Unknown format
            return None;
        };

        // Parse Dimension parameters - handle both full format and truncated format
        let dimensions = if quantity_type.contains("Dimension<_M<") && quantity_type.contains("_A<")
        {
            // Full format: Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>
            let (mass, length, time, current, temp, amount, lum, angle) =
                self.parse_dimension_full_format(quantity_type)?;
            DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle])
        } else if quantity_type.contains("Dimension,") || quantity_type.contains("Dimension>") {
            // Fully defaulted Dimension (dimensionless): Dimension, T or Dimension> T
            DynDimensionExponents::ZERO
        } else {
            // Truncated format: parse only the non-zero parameters
            // Look for patterns like Dimension<_M<0>, _L<1>> (only non-zero parameters are shown)
            let (mass, length, time, current, temp, amount, lum, angle) =
                self.parse_dimension_truncated_format(quantity_type);
            DynDimensionExponents([mass, length, time, current, temp, amount, lum, angle])
        };

        // Don't apply base scale offset here - let the prettyprint functions handle it
        // The prettyprint functions already have the correct base scale offset logic
        let adjusted_scale = scale;

        // Extract the actual generic type parameter and brand from the type string
        let (generic_type, brand) = self.extract_generic_type_and_brand(quantity_type);

        Some(QuantityParams {
            dimensions,
            scale: adjusted_scale,
            generic_type,
            brand,
        })
    }

    /// Parse general Scale format: Scale<_2[<P2>], _3[<P3>], _5[<P5>], _Pi[<PI>]>
    /// Handles all possible combinations of defaulted parameters
    /// Parameters can be either _2<value> (explicit) or _2 (defaulted to 0)
    fn parse_scale_general_format(&self, quantity_type: &str) -> Option<(i16, i16, i16, i16)> {
        let scale_start = quantity_type.find("Scale<")?;
        let scale_content = &quantity_type[scale_start + 6..]; // Skip "Scale<"

        // Find the end of the Scale struct
        let scale_end = self.find_matching_bracket(scale_content, 0)?;
        let scale_params = &scale_content[..scale_end];

        // Parse each parameter, handling both explicit values and defaults
        let p2 = self.parse_scale_param_with_default(scale_params, "_2");
        let p3 = self.parse_scale_param_with_default(scale_params, "_3");
        let p5 = self.parse_scale_param_with_default(scale_params, "_5");
        let pi = self.parse_scale_param_with_default(scale_params, "_Pi");

        Some((p2, p3, p5, pi))
    }

    /// Parse a scale parameter that may be either explicit (_2<value>) or defaulted (_2)
    fn parse_scale_param_with_default(&self, content: &str, prefix: &str) -> i16 {
        // First try to find explicit value: _2<value>
        if let Some(value) = self.parse_scale_param(content, &format!("{}<", prefix)) {
            value
        } else {
            // Check if parameter exists in defaulted form: _2 (without <value>)
            if content.contains(&format!("{},", prefix)) || content.ends_with(prefix) {
                0 // Default value
            } else {
                0 // Parameter not found, default to 0
            }
        }
    }

    /// Parse full Dimension format: Dimension<_M<MASS>, _L<LENGTH>, _T<TIME>, _I<CURRENT>, _Θ<TEMP>, _N<AMOUNT>, _J<LUMINOSITY>, _A<ANGLE>>
    fn parse_dimension_full_format(
        &self,
        quantity_type: &str,
    ) -> Option<(i16, i16, i16, i16, i16, i16, i16, i16)> {
        let dimension_start = quantity_type.find("Dimension<_M<")?;
        let dimension_content = &quantity_type[dimension_start + 9..]; // Skip "Dimension<"

        // Parse individual parameters directly from the dimension content
        // We don't need to find the end of the Dimension struct - we can parse each parameter individually
        let mass = self.parse_dimension_param(dimension_content, "_M<")?;
        let length = self.parse_dimension_param(dimension_content, "_L<")?;
        let time = self.parse_dimension_param(dimension_content, "_T<")?;
        let current = self.parse_dimension_param(dimension_content, "_I<")?;
        let temp = self.parse_dimension_param(dimension_content, "_Θ<")?;
        let amount = self.parse_dimension_param(dimension_content, "_N<")?;
        let lum = self.parse_dimension_param(dimension_content, "_J<")?;
        let angle = self.parse_dimension_param(dimension_content, "_A<")?;

        Some((mass, length, time, current, temp, amount, lum, angle))
    }

    /// Parse truncated Dimension format: Dimension<_M<0>, _L<1>> (only non-zero parameters are shown)
    fn parse_dimension_truncated_format(
        &self,
        quantity_type: &str,
    ) -> (i16, i16, i16, i16, i16, i16, i16, i16) {
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

        (
            mass_exp,
            length_exp,
            time_exp,
            electric_current_exp,
            temperature_exp,
            amount_of_substance_exp,
            luminous_intensity_exp,
            angle_exp,
        )
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
                }
                _ => {}
            }
            i += 1;
        }
        None
    }

    /// Extract both the generic type parameter and brand from a Quantity type string
    /// Returns (generic_type, brand) where brand is Some(...) if present, None otherwise
    fn extract_generic_type_and_brand(&self, quantity_type: &str) -> (String, Option<String>) {
        // Parse as a Rust type using syn - it handles const generics reliably
        if let Some(quantity_start) = quantity_type.find("Quantity<") {
            let quantity_content = &quantity_type[quantity_start..];

            match parse_str::<TypePath>(quantity_content) {
                Ok(type_path) => {
                    // Extract generic arguments
                    if let Some(segment) = type_path.path.segments.last() {
                        if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                            let args_vec: Vec<_> = args.args.iter().collect();

                            // We expect: [Scale, Dimension<...>, T, Brand?]
                            // Scale is at index 0, Dimension at 1, type T at 2, Brand at 3 (if present)

                            if args_vec.len() >= 3 {
                                // Extract the type parameter (index 2)
                                let type_arg = &args_vec[2];
                                let generic_type = match type_arg {
                                    syn::GenericArgument::Type(ty) => {
                                        quote::quote!(#ty).to_string()
                                    }
                                    _ => quote::quote!(#type_arg).to_string(),
                                };

                                // Extract Brand if present (index 3)
                                let brand = if args_vec.len() >= 4 {
                                    let brand_arg = &args_vec[3];
                                    match brand_arg {
                                        syn::GenericArgument::Type(ty) => {
                                            Some(quote::quote!(#ty).to_string())
                                        }
                                        _ => Some(quote::quote!(#brand_arg).to_string()),
                                    }
                                } else {
                                    None
                                };

                                return (
                                    generic_type.trim().to_string(),
                                    brand.map(|s| s.trim().to_string()),
                                );
                            } else if args_vec.len() == 2 {
                                // Only Scale and Dimension, type defaults to f64
                                return ("f64".to_string(), None);
                            }
                        }
                    }
                }
                Err(_) => {
                    // syn parsing failed - this shouldn't happen for valid Rust types
                    // but default to f64 if it does
                }
            }
        }

        // Default fallback if parsing fails
        ("f64".to_string(), None)
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
    pub fn extract_raw_type_from_hover(&self, hover_text: &str) -> String {
        // Look for the pattern: let [mut] [var]: Quantity<...>
        if let Some(start) = hover_text.find(": Quantity<") {
            // Find the start of the variable declaration by looking backwards for 'let'
            let mut var_start = start;
            let mut found_let = false;

            // Look backwards to find the start of the declaration
            while var_start > 0 {
                let char_before = hover_text[var_start - 1..var_start].chars().next().unwrap();
                if char_before.is_whitespace() {
                    // Check if we've found "let" or "let mut"
                    let potential_start = var_start;
                    let before_whitespace = &hover_text[..potential_start];

                    // Look for "let" at the end of the string
                    if before_whitespace.ends_with("let") {
                        // Check if there's whitespace before "let" or if it's at the start
                        let let_start = before_whitespace.len() - 3;
                        if let_start == 0
                            || hover_text[let_start - 1..let_start]
                                .chars()
                                .next()
                                .unwrap()
                                .is_whitespace()
                        {
                            var_start = let_start;
                            found_let = true;
                            break;
                        }
                    }
                }
                var_start -= 1;
            }

            if !found_let {
                // Fallback: find the start of the variable name
                while var_start > 0
                    && !hover_text[var_start..var_start + 1]
                        .chars()
                        .next()
                        .unwrap()
                        .is_whitespace()
                {
                    var_start -= 1;
                }
                if var_start > 0
                    && hover_text[var_start..var_start + 1]
                        .chars()
                        .next()
                        .unwrap()
                        .is_whitespace()
                {
                    var_start += 1; // Skip the whitespace
                }
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
                let full_declaration = &hover_text[var_start..start + ": ".len() + end_pos];
                return full_declaration.to_string();
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
}

#[derive(Debug)]
struct QuantityParams {
    dimensions: DynDimensionExponents,
    scale: ScaleExponents,
    generic_type: String,
    brand: Option<String>,
}
