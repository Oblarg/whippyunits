use crate::print::name_lookup::lookup_dimension_name;
use crate::print::name_lookup::{
    generate_systematic_unit_name, generate_systematic_unit_name_with_scale_factors,
};
use crate::print::utils::{get_si_prefix, to_unicode_superscript};
use whippyunits_default_dimensions::DIMENSION_LOOKUP;

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
    let atomic_symbols: Vec<&str> = DIMENSION_LOOKUP
        .iter()
        .take(8) // First 8 are the atomic dimensions
        .map(|info| info.symbol.unwrap_or("?"))
        .collect();

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
        // Format as integer if it's a whole number, otherwise show with reasonable precision
        if value.fract() == 0.0 {
            format!("({})", value as i64)
        } else {
            format!("({})", value)
        }
    }
}

fn generate_prefixed_si_unit(
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

fn generate_prefixed_systematic_unit(
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
                exponents.iter().enumerate().find(|(_, &exp)| exp != 0)
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
                                .map(|(i, &exp)| if i == dimension_index { 1 } else { 0 })
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

/// Helper function to format scale values, handling sentinel values
fn format_scale_value(scale: i16) -> String {
    if scale == i16::MAX {
        "unused".to_string()
    } else {
        scale.to_string()
    }
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
            let base_systematic_literal = generate_systematic_unit_name_with_scale_factors(
                [$($dimension_args)*].to_vec(),
                ($($scale_args)*),
                verbose, // Use full names in verbose mode, symbols in non-verbose mode
            );

            // Check if we found a unit literal match - if so, use it directly without conversion factor
            let systematic_literal = if base_systematic_literal != generate_systematic_unit_name([$($dimension_args)*].to_vec(), verbose) {
                // We found a unit literal match, use it directly
                base_systematic_literal
            } else {
                // No unit literal match, apply SI prefix to the systematic unit literal
                generate_prefixed_systematic_unit(
                    [$($dimension_args)*].to_vec(),
                    $($scale_args)*,
                    &base_systematic_literal,
                    verbose,
                )
            };

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
        scale_pi: i16
    ),
    (
        mass_exponent, length_exponent, time_exponent, electric_current_exponent, temperature_exponent, amount_of_substance_exponent, luminous_intensity_exponent, angle_exponent
    ),
    (
        scale_p2, scale_p3, scale_p5, scale_pi
    ),
    format!(
        "; [mass{}, length{}, time{}, current{}, temperature{}, amount{}, luminosity{}, angle{}] [2{}, 3{}, 5{}, π{}]",
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

        /// Ultra-terse pretty print for inlay hints - shows only the unit literal with SI prefixes
        pub fn pretty_print_quantity_inlay_hint(
            $($dimension_signature_params)*
        ) -> String {
            let systematic_literal = generate_systematic_unit_name_with_scale_factors(
                [$($dimension_args)*].to_vec(),
                ($($scale_args)*),
                false
            );

            // Check if we found a unit literal match - if so, use it directly without conversion factor
            let prefixed_systematic_literal = if systematic_literal != generate_systematic_unit_name([$($dimension_args)*].to_vec(), false) {
                // We found a unit literal match, use it directly
                systematic_literal
            } else {
                // No unit literal match, apply SI prefix to the systematic unit literal
                generate_prefixed_systematic_unit(
                    [$($dimension_args)*].to_vec(),
                    $($scale_args)*,
                    &systematic_literal,
                    false, // Use short names for inlay hints
                )
            };

            // Check if we have a recognized dimension with a specific SI unit
            if let Some(info) = lookup_dimension_name([$($dimension_args)*].to_vec()) {
                if let Some(si_shortname) = info.unit_si_shortname_symbol {
                    // Apply SI prefix to the specific SI unit name
                    let prefixed_si_unit = generate_prefixed_si_unit(
                        $($scale_args)*,
                        si_shortname,
                        false, // Use short names for inlay hints
                    );

                    // Return the prefixed SI unit if it's different from the systematic literal
                    if prefixed_si_unit != prefixed_systematic_literal {
                        prefixed_si_unit
                    } else {
                        prefixed_systematic_literal
                    }
                } else {
                    // No specific SI unit defined, use the prefixed systematic literal
                    prefixed_systematic_literal
                }
            } else {
                // Unknown dimension, use the prefixed systematic literal
                prefixed_systematic_literal
            }
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
