use crate::print::name_lookup::lookup_dimension_name;
use crate::print::name_lookup::generate_systematic_unit_name;
use crate::print::utils::{get_si_prefix, to_unicode_superscript};
use crate::print::unit_literal_generator::{generate_unit_literal, UnitLiteralConfig};
use whippyunits_core::api_helpers::get_atomic_dimension_symbols;

/// Check if a dimension is primitive (has exactly one non-zero exponent equal to 1)
/// Primitive dimensions are the 8 SI base quantities: Mass, Length, Time, Current, Temperature, Amount, Luminosity, Angle
fn is_primitive_dimension(exponents: Vec<i16>) -> bool {
    if exponents.len() != 8 {
        return false;
    }
    
    // Count non-zero exponents
    let non_zero_count = exponents.iter().filter(|&&exp| exp != 0).count();
    
    // A primitive dimension has exactly one non-zero exponent, and it must be 1
    non_zero_count == 1 && exponents.iter().any(|&exp| exp == 1)
}

/// Format configuration for unit symbol generation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitFormat {
    /// Unicode format with superscripts and special characters (default)
    Unicode,
    /// UCUM format with plain text exponents and dots
    Ucum,
}

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

/// Generate scale brackets with only non-zero exponents
fn generate_scale_brackets(scale_p2: i16, scale_p3: i16, scale_p5: i16, scale_pi: i16) -> String {
    let mut terms = Vec::new();
    
    if scale_p2 != 0 {
        terms.push(format!("2{}", format_scale_exponent(scale_p2)));
    }
    if scale_p3 != 0 {
        terms.push(format!("3{}", format_scale_exponent(scale_p3)));
    }
    if scale_p5 != 0 {
        terms.push(format!("5{}", format_scale_exponent(scale_p5)));
    }
    if scale_pi != 0 {
        terms.push(format!("π{}", format_scale_exponent(scale_pi)));
    }
    
    if terms.is_empty() {
        String::new()
    } else {
        format!(" [{}]", terms.join(", "))
    }
}

/// Generate dimension brackets with only non-zero exponents
fn generate_dimension_brackets(
    mass_exponent: i16,
    length_exponent: i16,
    time_exponent: i16,
    electric_current_exponent: i16,
    temperature_exponent: i16,
    amount_of_substance_exponent: i16,
    luminous_intensity_exponent: i16,
    angle_exponent: i16,
) -> String {
    let mut terms = Vec::new();
    
    if mass_exponent != 0 {
        terms.push(format!("mass{}", to_unicode_superscript(mass_exponent, true)));
    }
    if length_exponent != 0 {
        terms.push(format!("length{}", to_unicode_superscript(length_exponent, true)));
    }
    if time_exponent != 0 {
        terms.push(format!("time{}", to_unicode_superscript(time_exponent, true)));
    }
    if electric_current_exponent != 0 {
        terms.push(format!("current{}", to_unicode_superscript(electric_current_exponent, true)));
    }
    if temperature_exponent != 0 {
        terms.push(format!("temperature{}", to_unicode_superscript(temperature_exponent, true)));
    }
    if amount_of_substance_exponent != 0 {
        terms.push(format!("amount{}", to_unicode_superscript(amount_of_substance_exponent, true)));
    }
    if luminous_intensity_exponent != 0 {
        terms.push(format!("luminosity{}", to_unicode_superscript(luminous_intensity_exponent, true)));
    }
    if angle_exponent != 0 {
        terms.push(format!("angle{}", to_unicode_superscript(angle_exponent, true)));
    }
    
    if terms.is_empty() {
        String::new()
    } else {
        format!(" [{}]", terms.join(", "))
    }
}

/// Generate dimension symbols from atomic dimension symbols and exponents
///
/// This function uses the atomic dimension symbols from the default-dimensions
/// source of truth to generate composite dimension symbols.
/// Solved dimensions (non-zero exponents) are shown first, followed by all other dimensions (with ˀ).
pub fn generate_dimension_symbols(exponents: Vec<i16>) -> String {
    generate_dimension_symbols_with_format(exponents, UnitFormat::Unicode)
}

/// Generate dimension symbols with specified format
pub fn generate_dimension_symbols_with_format(exponents: Vec<i16>, format: UnitFormat) -> String {
    // Dimension symbols only support Unicode format
    // UCUM format should use unit name generation instead
    match format {
        UnitFormat::Unicode => generate_dimension_symbols_unicode(exponents),
        UnitFormat::Ucum => {
            // For UCUM format, redirect to unit name generation
            crate::print::name_lookup::generate_systematic_unit_name_with_format(
                exponents, false, format,
            )
        }
    }
}

/// Generate dimension symbols in Unicode format (original behavior)
fn generate_dimension_symbols_unicode(exponents: Vec<i16>) -> String {
    // Get atomic dimension symbols from the source of truth
    let atomic_symbols: Vec<&str> = get_atomic_dimension_symbols();

    let mut parts: Vec<String> = Vec::new();

    // First, add solved dimensions (non-zero, non--32768 exponents)
    for (idx, &exp) in exponents.iter().enumerate() {
        if exp != 0 && exp != -32768 {
            let symbol = atomic_symbols.get(idx).unwrap_or(&"?");
            let superscript = to_unicode_superscript(exp, false);
            parts.push(format!("{}{}", symbol, superscript));
        }
    }

    // Then, add all unsolved dimensions (exponents == -32768) with ˀ
    let mut unsolved_parts = Vec::new();
    for (idx, &exp) in exponents.iter().enumerate() {
        if exp == -32768 {
            // Only add unsolved dimensions
            let symbol = atomic_symbols.get(idx).unwrap_or(&"?");
            unsolved_parts.push(format!("{}{}", symbol, "ˀ"));
        }
    }

    // If we have unsolved dimensions, wrap them in parentheses
    if !unsolved_parts.is_empty() {
        parts.push(format!("({})", unsolved_parts.join("·")));
    }

    if parts.is_empty() {
        "?".to_string()
    } else {
        parts.join("·")
    }
}

#[macro_export]
#[doc(hidden)]
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

define_generate_verbose_dimension_names!((
    (0, "Mass"),
    (1, "Length"),
    (2, "Time"),
    (3, "Current"),
    (4, "Temperature"),
    (5, "Amount"),
    (6, "Luminosity"),
    (7, "Angle")
));

#[macro_export]
#[doc(hidden)]
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
        scale_pi: i16
    ),
    (
        let mut total_scale_p10: i16 = 0;
        // In the new system, powers of 10 are represented as equal powers of 2 and 5
        // AND all other scale factors must be zero for it to be a pure power of 10
        if scale_p2 == scale_p5 && scale_p3 == 0 && scale_pi == 0 {
            total_scale_p10 = scale_p2;
        }
        // Note: scale_p3 and scale_pi don't contribute to powers of 10
        total_scale_p10
    )
);

/// Generate SI unit with 10^n notation when no standard prefix is available
fn generate_si_unit_with_scale(
    total_scale_p10: i16,
    base_si_unit: &str,
    _long_name: bool,
) -> String {
    if total_scale_p10 == 0 {
        base_si_unit.to_string()
    } else {
        format!(
            "10{} {}",
            to_unicode_superscript(total_scale_p10, false),
            base_si_unit
        )
    }
}

/// Format scale factors by calculating the actual numeric value
fn format_scale_factors(scale_p2: i16, scale_p3: i16, scale_p5: i16, scale_pi: i16) -> String {
    // Calculate the actual numeric value: 2^p2 * 3^p3 * 5^p5 * π^pi
    let mut value = 1.0;

    if scale_p2 != 0 {
        value *= 2.0_f64.powi(scale_p2 as i32);
    }
    if scale_p3 != 0 {
        value *= 3.0_f64.powi(scale_p3 as i32);
    }
    if scale_p5 != 0 {
        value *= 5.0_f64.powi(scale_p5 as i32);
    }
    if scale_pi != 0 {
        value *= std::f64::consts::PI.powi(scale_pi as i32);
    }

    // If the value is 1.0, no scaling needed
    if value == 1.0 {
        String::new()
    } else {
        // Format with 5 significant figures for reasonable precision
        format!("({})", format_float_with_sig_figs(value, 5))
    }
}

pub fn generate_prefixed_si_unit(
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_pi: i16,
    base_si_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(scale_p2, scale_p3, scale_p5, scale_pi);

    if let Some(prefix) = get_si_prefix(total_scale_p10, long_name) {
        format!("{}{}", prefix, base_si_unit)
    } else {
        // Check if this is a pure power of 10 (p2 == p5 and p3 == 0 and pi == 0)
        let is_pure_power_of_10 = scale_p2 == scale_p5 && scale_p3 == 0 && scale_pi == 0;

        if is_pure_power_of_10 {
            // Fall back to SI unit with 10^n notation when SI prefix lookup fails
            generate_si_unit_with_scale(total_scale_p10, base_si_unit, long_name)
        } else {
            // Not a pure power of 10, show the scale factors explicitly
            let scale_factors = format_scale_factors(scale_p2, scale_p3, scale_p5, scale_pi);
            if scale_factors.is_empty() {
                base_si_unit.to_string()
            } else {
                format!("{}{}", scale_factors, base_si_unit)
            }
        }
    }
}

pub fn generate_prefixed_systematic_unit(
    exponents: Vec<i16>,
    scale_p2: i16,
    scale_p3: i16,
    scale_p5: i16,
    scale_pi: i16,
    base_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(scale_p2, scale_p3, scale_p5, scale_pi);

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
            if let Some((dimension_index, &exponent)) =
                exponents.iter().enumerate().find(|&(_, &exp)| exp != 0)
            {
                // Check if the scale is a multiple of the exponent
                if effective_scale_p10 % exponent == 0 {
                    // Factor the prefix: divide scale by exponent
                    let factored_scale = effective_scale_p10 / exponent;

                    // Get the prefix for the factored scale
                    if let Some(factored_prefix) = get_si_prefix(factored_scale, long_name) {
                        // Get the base unit name without any scale or exponent
                        let base_unit_name = generate_systematic_unit_name(
                            exponents
                                .iter()
                                .enumerate()
                                .map(|(i, _)| if i == dimension_index { 1 } else { 0 })
                                .collect(),
                            long_name,
                        );

                        // Apply the factored prefix to the base unit name, then add the exponent
                        format!(
                            "{}{}{}",
                            factored_prefix,
                            base_unit_name,
                            get_unicode_exponent(exponent)
                        )
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
                &base_unit[1..base_unit.len() - 1]
            } else {
                base_unit
            };
            format!("{}({})", prefix, unit_without_parens)
        }
    } else {
        // No SI prefix available, check if we need to add numerical scale factor
        let scale_factors = format_scale_factors(scale_p2, scale_p3, scale_p5, scale_pi);
        if scale_factors.is_empty() {
            // No scaling needed, return base unit as-is
            base_unit.to_string()
        } else {
            // Add numerical scale factor prefix
            format!("{}{}", scale_factors, base_unit)
        }
    }
}

/// Helper function to format floating point numbers with a reasonable number of significant figures
fn format_float_with_sig_figs(value: f64, sig_figs: usize) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    
    let abs_value = value.abs();
    let magnitude = abs_value.log10().floor() as i32;
    let scale_factor = 10_f64.powi(sig_figs as i32 - 1 - magnitude as i32);
    
    let rounded = (value * scale_factor).round() / scale_factor;
    
    // Format with appropriate precision
    let formatted = if magnitude >= 0 {
        // For values >= 1, show up to sig_figs digits total
        let precision = (sig_figs as i32 - magnitude - 1).max(0) as usize;
        format!("{:.precision$}", rounded, precision = precision)
    } else {
        // For values < 1, show sig_figs significant digits after decimal
        format!("{:.precision$}", rounded, precision = (sig_figs as i32 + magnitude.abs()) as usize)
    };
    
    // Note: The ~ symbol should only be added when the storage scale is truncated,
    // not when the stored value is truncated during formatting. This function
    // is only responsible for formatting the stored value, so we don't add ~ here.
    formatted
}


#[macro_export]
#[doc(hidden)]
macro_rules! define_pretty_print_quantity {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($scale_args:tt)*), $unit_vector_format:expr) => {
        /// Formatted string in the format: `(value) Quantity<systematic_literal, unit_shortname, dimension_name, [exponents and scales], type>`
        pub fn pretty_print_quantity(
            value: Option<f64>,
            $($dimension_signature_params)*,
            type_name: &str,
            verbose: bool,
            _show_type_in_brackets: bool,
        ) -> String {
            let value_prefix = if let Some(val) = value {
                let formatted_val = format_float_with_sig_figs(val, 5);
                format!("({}) ", formatted_val)
            } else {
                String::new()
            };

            // Generate the best unit literal using centralized logic
            let unit_literal = generate_unit_literal(
                [$($dimension_args)*].to_vec(),
                $($scale_args)*,
                UnitLiteralConfig {
                    verbose,
                    prefer_si_units: true,
                },
            );

            // Look up dimension name for secondary display
            let dimension_info = lookup_dimension_name([$($dimension_args)*].to_vec());

            let dimension_name = if let Some(ref info) = dimension_info {
                // For recognized composite dimensions, always use the dimension name (e.g., "Force", "Energy")
                // regardless of verbose/non-verbose mode, since these are established names
                info.dimension_name.to_string()
            } else {
                if verbose {
                    // For unrecognized dimensions in verbose mode, generate verbose dimension names
                    generate_verbose_dimension_names([$($dimension_args)*].to_vec())
                } else {
                    // For unrecognized dimensions in non-verbose mode, use dimension symbols
                    generate_dimension_symbols([$($dimension_args)*].to_vec())
                }
            };

            let primary = if !unit_literal.is_empty() { 
                &unit_literal 
            } else { 
                &dimension_name 
            };
            let secondary = if verbose {
                // In verbose mode (debug), show the dimension name in parentheses
                // but only for composite dimensions, not primitive ones
                if dimension_info.is_some() && !is_primitive_dimension([$($dimension_args)*].to_vec()) {
                    format!(" ({})", dimension_name)
                } else {
                    String::new()
                }
            } else {
                // In non-verbose mode (display), don't show dimension names or semicolons
                String::new()
            };
            let verbose_info = if verbose {
                $unit_vector_format
            } else {
                String::new()
            };

            // Always add the type parameter at the end
            let type_suffix = if verbose {
                format!(", {}", type_name)
            } else {
                format!(", {}", type_name)
            };

            format!("{}Quantity<{}{}{}{}>", value_prefix, primary, secondary, verbose_info, type_suffix)
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
        scale_pi: i16
    ),
    (
        mass_exponent, length_exponent, time_exponent, electric_current_exponent, temperature_exponent, amount_of_substance_exponent, luminous_intensity_exponent, angle_exponent
    ),
    (
        scale_p2, scale_p3, scale_p5, scale_pi
    ),
    format!(
        "{}{}",
        generate_scale_brackets(scale_p2, scale_p3, scale_p5, scale_pi),
        generate_dimension_brackets(
            mass_exponent,
            length_exponent,
            time_exponent,
            electric_current_exponent,
            temperature_exponent,
            amount_of_substance_exponent,
            luminous_intensity_exponent,
            angle_exponent
        )
    )
);

#[macro_export]
#[doc(hidden)]
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
        scale_p2, scale_p3, scale_p5, scale_pi
    )
);
