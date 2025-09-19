use crate::print::name_lookup::generate_systematic_unit_name;
use crate::print::name_lookup::lookup_dimension_name;
use crate::print::utils::{to_unicode_superscript, get_si_prefix};

// Helper function to get unicode exponent
fn get_unicode_exponent(exp: i16) -> String {
    to_unicode_superscript(exp, false)
}

/// Helper function to format scale exponent values, using "ˀ" for i16::MIN values
fn format_scale_exponent(scale: i16) -> String {
    if scale == i16::MIN {
        "ˀ".to_string()
    } else {
        to_unicode_superscript(scale, true)
    }
}

#[macro_export]
macro_rules! define_generate_dimension_symbols {
    (($($dimension_symbols:tt)*)) => {
        pub fn generate_dimension_symbols(
            exponents: Vec<i16>
        ) -> String {
            let parts: Vec<String> = [
                $($dimension_symbols)*
            ].iter()
            .filter(|(idx, _)| exponents[*idx] != 0)
            .map(|(idx, symbol)| {
                let superscript = to_unicode_superscript(exponents[*idx], false);
                format!("{}{}", symbol, superscript)
            })
            .collect();
            
            if parts.is_empty() { "?".to_string() } else { parts.join("·") }
        }
    };
}

define_generate_dimension_symbols!(
    (
        (0, "M"),
        (1, "L"), 
        (2, "T"),
        (3, "I"),
        (4, "θ"),
        (5, "N"),
        (6, "Cd"),
        (7, "A")
    )
);

#[macro_export]
macro_rules! define_generate_verbose_dimension_names {
    (($($dimension_names:tt)*)) => {
        /// Generate verbose dimension names for unresolved types (Length, Time, Mass)
        pub fn generate_verbose_dimension_names(
            exponents: Vec<i16>
        ) -> String {
            let parts: Vec<String> = [
                $($dimension_names)*
            ].iter()
            .filter(|(idx, _)| exponents[*idx] != 0)
            .map(|(idx, name)| if exponents[*idx] == 1 { 
                name.to_string() 
            } else { 
                let superscript = to_unicode_superscript(exponents[*idx], false);
                format!("{}{}", name, superscript)
            })
            .collect();
            
            if parts.is_empty() { "?".to_string() } else { parts.join("·") }
        }
    };
}

define_generate_verbose_dimension_names!(
    (
        (0, "Mass"),
        (1, "Length"),
        (2, "Time"),
        (3, "Current"),
        (4, "Temperature"),
        (5, "Amount"),
        (6, "Luminosity"),
        (7, "Angle")
    )
);

#[macro_export]
macro_rules! define_calculate_total_scale_p10 {
    (($($dimension_params:tt)*), ($($total_scale_calculation:tt)*)) => {
        /// Calculate total power of 10 across all dimensions
        fn calculate_total_scale_p10(
            $($dimension_params)*
        ) -> i16 {
            // total_scale_p10
            $($total_scale_calculation)*
        }
    };
}

define_calculate_total_scale_p10!(
    (
        scale_p2: i16,
        scale_p3: i16,
        scale_p5: i16,
        scale_p10: i16,
        scale_pi: i16
    ),
    (
        let mut total_scale_p10: i16 = scale_p10;
        // only check pure powers of 10 on composite units
        if scale_p2 == scale_p5 && scale_p3 == 0 {
            total_scale_p10 += scale_p2;
        }
        total_scale_p10
    )
);

/// Generate SI unit with 10^n notation when no standard prefix is available
fn generate_si_unit_with_scale(total_scale_p10: i16, base_si_unit: &str, _long_name: bool) -> String {
    if total_scale_p10 == 0 {
        base_si_unit.to_string()
    } else {
        format!("10{} {}", to_unicode_superscript(total_scale_p10, false), base_si_unit)
    }
}

fn generate_prefixed_si_unit(
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_p10: i16,
    scale_pi: i16,
    base_si_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(
        scale_p2, scale_p3, scale_p5, scale_p10, scale_pi
    );
    
    if let Some(prefix) = get_si_prefix(total_scale_p10, long_name) {
        format!("{}{}", prefix, base_si_unit)
    } else {
        // Fall back to SI unit with 10^n notation when SI prefix lookup fails
        generate_si_unit_with_scale(total_scale_p10, base_si_unit, long_name)
    }
}

fn generate_prefixed_systematic_unit(
    exponents: Vec<i16>,
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_p10: i16,
    scale_pi: i16,
    base_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(
        scale_p2, scale_p3, scale_p5, scale_p10, scale_pi
    );
    
    // Check if this is a pure unit (not compound)
    let is_pure_unit = !base_unit.contains("·");
    
    // For pure units, check if we need to apply base scale offset
    let effective_scale_p10 = if is_pure_unit {
        // Check if this is a pure unit with base scale offset (like mass with "gram")
        if base_unit == "gram" {
            // Mass has base_scale_offset: 3, so apply it to the scale calculation
            total_scale_p10 + 3
        } else {
            total_scale_p10
        }
    } else {
        // For compound units, don't apply base scale offset to the aggregate prefix
        // The individual parts already have their base scale offsets applied
        total_scale_p10
    };
    
    
    if let Some(prefix) = get_si_prefix(effective_scale_p10, long_name) {
        if is_pure_unit {
            // Check if this is a pure unit with an exponent
            // Find the non-zero exponent (there should be exactly one for a pure unit)
            if let Some((dimension_index, &exponent)) = exponents.iter().enumerate().find(|(_, &exp)| exp != 0) {
                // Check if the scale is a multiple of the exponent
                if effective_scale_p10 % exponent == 0 {
                    // Factor the prefix: divide scale by exponent
                    let factored_scale = effective_scale_p10 / exponent;
                    
                    // Get the prefix for the factored scale
                    if let Some(factored_prefix) = get_si_prefix(factored_scale, long_name) {
                        // Get the base unit name without any scale or exponent
                        let base_unit_name = generate_systematic_unit_name(
                            exponents.iter().enumerate().map(|(i, &exp)| if i == dimension_index { 1 } else { 0 }).collect(),
                            long_name
                        );
                        
                        // Apply the factored prefix to the base unit name, then add the exponent
                        format!("{}{}{}", factored_prefix, base_unit_name, get_unicode_exponent(exponent))
                    } else {
                        // Fallback to original behavior
                        format!("{}{}", prefix, base_unit)
                    }
                } else {
                    // Scale is not a multiple of exponent, apply prefix to the entire unit with exponent
                    // For example: milli(meters²) instead of millimeter²
                    format!("{}({})", prefix, base_unit)
                }
            } else {
                // No non-zero exponent found, use original behavior
                format!("{}{}", prefix, base_unit)
            }
        } else {
            // For compound units: "milli(meter·second)"
            // The base_unit already has parentheses from generate_systematic_unit_name, so we need to remove them
            let unit_without_parens = if base_unit.starts_with("(") && base_unit.ends_with(")") {
                &base_unit[1..base_unit.len()-1]
            } else {
                base_unit
            };
            format!("{}({})", prefix, unit_without_parens)
        }
    } else {
        // No SI prefix available, return base unit as-is
        base_unit.to_string()
    }
}       

/// Helper function to format scale values, handling sentinel values
fn format_scale_value(scale: i16) -> String {
    if scale == i16::MAX { "unused".to_string() } else { scale.to_string() }
}

#[macro_export]
macro_rules! define_pretty_print_quantity {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*), $unit_vector_format:expr) => {
        /// Formatted string in the format: `(value) Quantity<systematic_literal, unit_shortname, dimension_name, [exponents and scales]>`
        pub fn pretty_print_quantity(
            value: Option<f64>,
            $($dimension_signature_params)*,
            type_name: &str,
            verbose: bool,
            show_type_in_brackets: bool,
        ) -> String {
            let value_prefix = if let Some(val) = value {
                if verbose && !show_type_in_brackets {
                    format!("({}_{}) ", val, type_name)
                } else {
                    format!("({}) ", val)
                }
            } else {
                String::new()
            };
            
            // Generate systematic unit literal (base unit without prefix)
            let base_systematic_literal = generate_systematic_unit_name(
                [$($dimension_args)*].to_vec(),
                verbose, // Use full names in verbose mode, symbols in non-verbose mode
            );
            
            // Apply SI prefix to the systematic unit literal
            let systematic_literal = generate_prefixed_systematic_unit(
                [$($dimension_args)*].to_vec(),
                $($scale_args)*,
                &base_systematic_literal,
                verbose,
            );
            
            // Look up dimension name
            let dimension_info = lookup_dimension_name([$($dimension_args)*].to_vec());
            
            // Generate SI shortname - use dimension-specific SI unit if available, otherwise don't show a shortname
            let unit_shortname = if let Some(ref info) = dimension_info {
                if let Some(base_si_unit) = if verbose {
                    info.unit_si_shortname
                } else {
                    info.unit_si_shortname_symbol
                } {
                    // Use the specific SI unit name with correct prefix (e.g., "μJ" for microjoule)
                    generate_prefixed_si_unit(
                        $($scale_args)*,
                        base_si_unit,
                        verbose,
                    )
                } else {
                    // No specific SI unit defined for this recognized dimension, don't show a shortname
                    String::new()
                }
            } else {
                // Unknown dimension, don't show a unit shortname to avoid stuttering
                String::new()
            };
            
            let dimension_name = if let Some(ref info) = dimension_info {
                // For recognized dimensions, show the dimension name in both modes
                info.dimension_name.to_string()
            } else {
                if verbose {
                    // For unrecognized dimensions in verbose mode, generate verbose dimension names
                    generate_verbose_dimension_names([$($dimension_args)*].to_vec())
                } else {
                    // For unrecognized dimensions, compute dimension symbol dynamically from exponents
                    generate_dimension_symbols([$($dimension_args)*].to_vec())
                }
            };
            
            let primary = if systematic_literal.is_empty() { &dimension_name } else { &systematic_literal };
            let secondary = if !unit_shortname.is_empty() && unit_shortname != systematic_literal {
                format!("; {}; {}", unit_shortname, dimension_name)
            } else if !systematic_literal.is_empty() {
                // For unknown dimensions, don't show the redundant dimension name
                if dimension_info.is_some() {
                    format!("; {}", dimension_name)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            let verbose_info = if verbose {
                if show_type_in_brackets {
                    // Add the type name after the square brackets without semicolon separator
                    format!("{} {}", $unit_vector_format, type_name)
                } else {
                    $unit_vector_format
                }
            } else {
                String::new()
            };
            
            format!("{}Quantity<{}{}{}>", value_prefix, primary, secondary, verbose_info)
        }
    };
}

define_pretty_print_quantity!(
    (
        mass_exponent: i16,
        length_exponent: i16,
        time_exponent: i16,
        electric_current_exponent: i16,
        temperature_exponent: i16,
        amount_of_substance_exponent: i16,
        luminous_intensity_exponent: i16,
        angle_exponent: i16,
        scale_p2: i16,
        scale_p3: i16,
        scale_p5: i16,
        scale_p10: i16,
        scale_pi: i16
    ),
    (
        mass_exponent, length_exponent, time_exponent, electric_current_exponent, temperature_exponent, amount_of_substance_exponent, luminous_intensity_exponent, angle_exponent
    ),
    (
        scale_p2, scale_p3, scale_p5, scale_p10, scale_pi
    ),
    format!(
        "; [mass{}, length{}, time{}, current{}, temperature{}, amount{}, luminosity{}, angle{}] [2{}, 3{}, 5{}, 10{}, π{}]",
        to_unicode_superscript(mass_exponent, true),
        to_unicode_superscript(length_exponent, true),
        to_unicode_superscript(time_exponent, true),
        to_unicode_superscript(electric_current_exponent, true),
        to_unicode_superscript(temperature_exponent, true),
        to_unicode_superscript(amount_of_substance_exponent, true),
        to_unicode_superscript(luminous_intensity_exponent, true),
        to_unicode_superscript(angle_exponent, true),
        format_scale_exponent(scale_p2),
        format_scale_exponent(scale_p3),
        format_scale_exponent(scale_p5),
        format_scale_exponent(scale_p10),
        format_scale_exponent(scale_pi)
    )
);

#[macro_export]
macro_rules! define_pretty_print_quantity_helpers {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*)) => {
        /// Pretty print a quantity type (without value)
        pub fn pretty_print_quantity_type(
            $($dimension_signature_params)*,
            type_name: &str,
            verbose: bool,
            show_type_in_brackets: bool,
        ) -> String {
            pretty_print_quantity(
                None,
                $($dimension_args)*,
                $($scale_args)*,
                type_name,
                verbose,
                show_type_in_brackets,
            )
        }

        /// Pretty print a quantity value (with value)
        pub fn pretty_print_quantity_value(
            value: f64,
            $($dimension_signature_params)*,
            type_name: &str,
            verbose: bool,
            show_type_in_brackets: bool,
        ) -> String {
            pretty_print_quantity(
                Some(value),
                $($dimension_args)*,
                $($scale_args)*,
                type_name,
                verbose,
                show_type_in_brackets,
            )
        }

        /// Ultra-terse pretty print for inlay hints - shows only the unit literal
        pub fn pretty_print_quantity_inlay_hint(
            $($dimension_signature_params)*
        ) -> String {
            let systematic_literal = generate_systematic_unit_name(
                [$($dimension_args)*].to_vec(),
                false
            );
            
            lookup_dimension_name([$($dimension_args)*].to_vec())
                .and_then(|info| info.unit_si_shortname_symbol)
                .filter(|si_shortname| si_shortname != &systematic_literal)
                .map(|si_shortname| si_shortname.to_string())
                .unwrap_or(systematic_literal)
        }
    };
}

define_pretty_print_quantity_helpers!(
    (
        mass_exponent: i16,
        length_exponent: i16,
        time_exponent: i16,
        electric_current_exponent: i16,
        temperature_exponent: i16,
        amount_of_substance_exponent: i16,
        luminous_intensity_exponent: i16,
        angle_exponent: i16,
        scale_p2: i16,
        scale_p3: i16,
        scale_p5: i16,
        scale_p10: i16,
        scale_pi: i16
    ),
    (
        mass_exponent, 
        length_exponent, 
        time_exponent, 
        electric_current_exponent, 
        temperature_exponent, 
        amount_of_substance_exponent, 
        luminous_intensity_exponent, 
        angle_exponent
    ),
    (
        scale_p2, scale_p3, scale_p5, scale_p10, scale_pi
    )
);