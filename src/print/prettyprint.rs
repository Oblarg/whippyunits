use crate::print::name_lookup::generate_systematic_unit_name;
use crate::print::name_lookup::lookup_dimension_name;
use crate::print::utils::{to_unicode_superscript, get_si_prefix};

/// Helper function to format scale exponent values, using "ˀ" for i8::MIN values
fn format_scale_exponent(scale: i8) -> String {
    if scale == i8::MIN {
        "ˀ".to_string()
    } else {
        to_unicode_superscript(scale, true)
    }
}

#[macro_export]
macro_rules! define_generate_dimension_symbols {
    (($($dimension_symbols:tt)*), ($($dimension_exponents:tt)*)) => {
        pub fn generate_dimension_symbols(
            $($dimension_exponents)*
        ) -> String {
            let parts: Vec<String> = [
                $($dimension_symbols)*
            ].iter()
            .filter(|(exp, _)| *exp != 0)
            .map(|(exp, symbol)| format!("{}{}", symbol, to_unicode_superscript(*exp, false)))
            .collect();
            
            if parts.is_empty() { "?".to_string() } else { parts.join("·") }
        }
    };
}

define_generate_dimension_symbols!(
    (
        (mass_exponent, "M"),
        (length_exponent, "L"), 
        (time_exponent, "T"),
        (electric_current_exponent, "I"),
        (temperature_exponent, "θ"),
        (amount_of_substance_exponent, "N"),
        (luminous_intensity_exponent, "J"),
        (angle_exponent, "A")
    ),
    (mass_exponent: i8, length_exponent: i8, time_exponent: i8, electric_current_exponent: i8, temperature_exponent: i8, amount_of_substance_exponent: i8, luminous_intensity_exponent: i8, angle_exponent: i8)
);

#[macro_export]
macro_rules! define_generate_verbose_dimension_names {
    (($($dimension_names:tt)*), ($($dimension_exponents:tt)*)) => {
        /// Generate verbose dimension names for unresolved types (Length, Time, Mass)
        pub fn generate_verbose_dimension_names(
            $($dimension_exponents)*
        ) -> String {
            let parts: Vec<String> = [
                $($dimension_names)*
            ].iter()
            .filter(|(exp, _)| *exp != 0)
            .map(|(exp, name)| if *exp == 1 { 
                name.to_string() 
            } else { 
                format!("{}{}", name, to_unicode_superscript(*exp, false)) 
            })
            .collect();
            
            if parts.is_empty() { "?".to_string() } else { parts.join("·") }
        }
    };
}

define_generate_verbose_dimension_names!(
    (
        (mass_exponent, "Mass"),
        (length_exponent, "Length"),
        (time_exponent, "Time"),
        (electric_current_exponent, "Electric Current"),
        (temperature_exponent, "Temperature"),
        (amount_of_substance_exponent, "Amount of Substance"),
        (luminous_intensity_exponent, "Luminous Intensity"),
        (angle_exponent, "Angle")
    ),
    (mass_exponent: i8, length_exponent: i8, time_exponent: i8, electric_current_exponent: i8, temperature_exponent: i8, amount_of_substance_exponent: i8, luminous_intensity_exponent: i8, angle_exponent: i8)
);

#[macro_export]
macro_rules! define_calculate_total_scale_p10 {
    (($($dimension_params:tt)*), ($($total_scale_calculation:tt)*)) => {
        /// Calculate total power of 10 across all dimensions
        fn calculate_total_scale_p10(
            $($dimension_params)*
        ) -> i8 {
            // total_scale_p10
            $($total_scale_calculation)*
        }
    };
}

define_calculate_total_scale_p10!(
    (
        mass_exponent: i8, mass_scale_p10: i8,
        length_exponent: i8, length_scale_p10: i8,
        time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
        electric_current_exponent: i8, electric_current_scale_p10: i8,
        temperature_exponent: i8, temperature_scale_p10: i8,
        amount_of_substance_exponent: i8, amount_of_substance_scale_p10: i8,
        luminous_intensity_exponent: i8, luminous_intensity_scale_p10: i8,
        angle_exponent: i8, angle_scale_p2: i8, angle_scale_p3: i8, angle_scale_p5: i8, angle_scale_pi: i8
    ),
    (
        let mut total_scale_p10: i8 = 0;
        total_scale_p10 += mass_exponent * mass_scale_p10;
        total_scale_p10 += length_exponent * length_scale_p10;
        // only check pure powers of 10 on composite units
        if time_scale_p2 == time_scale_p5 && time_scale_p3 == 0 {
            total_scale_p10 += time_exponent * time_scale_p2;
        }
        total_scale_p10 += electric_current_exponent * electric_current_scale_p10;
        total_scale_p10 += temperature_exponent * temperature_scale_p10;
        total_scale_p10 += amount_of_substance_exponent * amount_of_substance_scale_p10;
        total_scale_p10 += luminous_intensity_exponent * luminous_intensity_scale_p10;
        if angle_scale_p2 == angle_scale_p5 && angle_scale_p3 == 0 && angle_scale_pi == 0 {
            total_scale_p10 += angle_exponent * angle_scale_p2;
        }
        total_scale_p10
    )
);

/// Generate SI unit with 10^n notation when no standard prefix is available
fn generate_si_unit_with_scale(total_scale_p10: i8, base_si_unit: &str, _long_name: bool) -> String {
    if total_scale_p10 == 0 {
        base_si_unit.to_string()
    } else {
        format!("10{} {}", to_unicode_superscript(total_scale_p10, false), base_si_unit)
    }
}

#[macro_export]
macro_rules! define_generate_prefixed_si_unit {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*)) => {
        /// Generate correctly-prefixed SI unit name
        fn generate_prefixed_si_unit(
            $($dimension_signature_params)*,
            base_si_unit: &str,
            long_name: bool,
        ) -> String {
            let total_scale_p10 = calculate_total_scale_p10(
                $($dimension_args)*
            );
            
            if let Some(prefix) = get_si_prefix(total_scale_p10, long_name) {
                format!("{}{}", prefix, base_si_unit)
            } else {
                // Fall back to SI unit with 10^n notation when SI prefix lookup fails
                generate_si_unit_with_scale(total_scale_p10, base_si_unit, long_name)
            }
        }        
    };
}

define_generate_prefixed_si_unit!(
    (
        mass_exponent: i8, mass_scale_p10: i8,
        length_exponent: i8, length_scale_p10: i8,
        time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
        electric_current_exponent: i8, electric_current_scale_p10: i8,
        temperature_exponent: i8, temperature_scale_p10: i8,
        amount_of_substance_exponent: i8, amount_of_substance_scale_p10: i8,
        luminous_intensity_exponent: i8, luminous_intensity_scale_p10: i8,
        angle_exponent: i8, angle_scale_p2: i8, angle_scale_p3: i8, angle_scale_p5: i8, angle_scale_pi: i8
    ),
    (
        mass_exponent, mass_scale_p10, 
        length_exponent, length_scale_p10, 
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5, 
        electric_current_exponent, electric_current_scale_p10, 
        temperature_exponent, temperature_scale_p10, 
        amount_of_substance_exponent, amount_of_substance_scale_p10, 
        luminous_intensity_exponent, luminous_intensity_scale_p10, 
        angle_exponent, angle_scale_p2, angle_scale_p3, angle_scale_p5, angle_scale_pi,
    )
);

/// Helper function to format scale values, handling sentinel values
fn format_scale_value(scale: i8) -> String {
    if scale == i8::MAX { "unused".to_string() } else { scale.to_string() }
}

#[macro_export]
macro_rules! define_pretty_print_quantity {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($dimension_exponents:tt)*), $unit_vector_format:expr) => {
        /// Formatted string in the format: `(value) Quantity<systematic_literal, unit_shortname, dimension_name, [exponents and scales]>`
        pub fn pretty_print_quantity(
            value: Option<f64>,
            $($dimension_signature_params)*
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
            
            // Generate systematic unit literal
            let systematic_literal = generate_systematic_unit_name(
                $($dimension_args)*
                verbose, // Use full names in verbose mode, symbols in non-verbose mode
            );
            
            // Look up dimension name
            let dimension_info = lookup_dimension_name($($dimension_exponents)*);
            
            // Generate SI shortname - use dimension-specific SI unit if available, otherwise don't show a shortname
            let unit_shortname = if let Some(ref info) = dimension_info {
                if let Some(base_si_unit) = if verbose {
                    info.unit_si_shortname
                } else {
                    info.unit_si_shortname_symbol
                } {
                    // Use the specific SI unit name with correct prefix (e.g., "μJ" for microjoule)
                    generate_prefixed_si_unit(
                        $($dimension_args)*
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
                    generate_verbose_dimension_names($($dimension_exponents)*)
                } else {
                    // For unrecognized dimensions, compute dimension symbol dynamically from exponents
                    generate_dimension_symbols($($dimension_exponents)*)
                }
            };
            
            let primary = if systematic_literal.is_empty() { &dimension_name } else { &systematic_literal };
            let secondary = if !unit_shortname.is_empty() && unit_shortname != systematic_literal {
                format!("; {}; {}", unit_shortname, dimension_name)
            } else if !systematic_literal.is_empty() {
                format!("; {}", dimension_name)
            } else {
                String::new()
            };
            let verbose_info = if verbose {
                if show_type_in_brackets {
                    // Insert the type name as the last item inside the square brackets
                    let unit_vector_with_type = $unit_vector_format.replace("]", &format!(", {}]", type_name));
                    unit_vector_with_type
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
        mass_exponent: i8, mass_scale_p10: i8,
        length_exponent: i8, length_scale_p10: i8,
        time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
        electric_current_exponent: i8, electric_current_scale_p10: i8,
        temperature_exponent: i8, temperature_scale_p10: i8,
        amount_of_substance_exponent: i8, amount_of_substance_scale_p10: i8,
        luminous_intensity_exponent: i8, luminous_intensity_scale_p10: i8,
        angle_exponent: i8, angle_scale_p2: i8, angle_scale_p3: i8, angle_scale_p5: i8, angle_scale_pi: i8,
    ),
    (
        mass_exponent, mass_scale_p10, 
        length_exponent, length_scale_p10, 
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5, 
        electric_current_exponent, electric_current_scale_p10, 
        temperature_exponent, temperature_scale_p10, 
        amount_of_substance_exponent, amount_of_substance_scale_p10, 
        luminous_intensity_exponent, luminous_intensity_scale_p10, 
        angle_exponent, angle_scale_p2, angle_scale_p3, angle_scale_p5, angle_scale_pi,
    ),
    (mass_exponent, length_exponent, time_exponent, electric_current_exponent, temperature_exponent, amount_of_substance_exponent, luminous_intensity_exponent, angle_exponent),
    format!(
        "; [mass{}(10{}), length{}(10{}), time{}(2{}, 3{}, 5{}), electric_current{}(10{}), temperature{}(10{}), amount_of_substance{}(10{}), luminous_intensity{}(10{}), angle{}(2{}, 3{}, 5{}, pi{})]",
        to_unicode_superscript(mass_exponent, true),
        format_scale_exponent(mass_scale_p10),
        to_unicode_superscript(length_exponent, true),
        format_scale_exponent(length_scale_p10),
        to_unicode_superscript(time_exponent, true),
        format_scale_exponent(time_scale_p2),
        format_scale_exponent(time_scale_p3),
        format_scale_exponent(time_scale_p5),
        to_unicode_superscript(electric_current_exponent, true),
        format_scale_exponent(electric_current_scale_p10),
        to_unicode_superscript(temperature_exponent, true),
        format_scale_exponent(temperature_scale_p10),
        to_unicode_superscript(amount_of_substance_exponent, true),
        format_scale_exponent(amount_of_substance_scale_p10),
        to_unicode_superscript(luminous_intensity_exponent, true),
        format_scale_exponent(luminous_intensity_scale_p10),
        to_unicode_superscript(angle_exponent, true),
        format_scale_exponent(angle_scale_p2),
        format_scale_exponent(angle_scale_p3),
        format_scale_exponent(angle_scale_p5),
        format_scale_exponent(angle_scale_pi)
    )
);

#[macro_export]
macro_rules! define_pretty_print_quantity_helpers {
    (($($dimension_signature_params:tt)*), ($($dimension_args:tt)*), ($($dimension_exponents:tt)*)) => {
        /// Pretty print a quantity type (without value)
        pub fn pretty_print_quantity_type(
            $($dimension_signature_params)*
            type_name: &str,
            verbose: bool,
            show_type_in_brackets: bool,
        ) -> String {
            pretty_print_quantity(
                None,
                $($dimension_args)*
                type_name,
                verbose,
                show_type_in_brackets,
            )
        }

        /// Pretty print a quantity value (with value)
        pub fn pretty_print_quantity_value(
            value: f64,
            $($dimension_signature_params)*
            type_name: &str,
            verbose: bool,
            show_type_in_brackets: bool,
        ) -> String {
            pretty_print_quantity(
                Some(value),
                $($dimension_args)*
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
                $($dimension_args)*
                false
            );
            
            lookup_dimension_name($($dimension_exponents)*)
                .and_then(|info| info.unit_si_shortname_symbol)
                .filter(|si_shortname| si_shortname != &systematic_literal)
                .map(|si_shortname| si_shortname.to_string())
                .unwrap_or(systematic_literal)
        }

    };
}

define_pretty_print_quantity_helpers!(
    (
        mass_exponent: i8, mass_scale_p10: i8,
        length_exponent: i8, length_scale_p10: i8,
        time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
        electric_current_exponent: i8, electric_current_scale_p10: i8,
        temperature_exponent: i8, temperature_scale_p10: i8,
        amount_of_substance_exponent: i8, amount_of_substance_scale_p10: i8,
        luminous_intensity_exponent: i8, luminous_intensity_scale_p10: i8,
        angle_exponent: i8, angle_scale_p2: i8, angle_scale_p3: i8, angle_scale_p5: i8, angle_scale_pi: i8,
    ),
    (
        mass_exponent, mass_scale_p10, 
        length_exponent, length_scale_p10, 
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5, 
        electric_current_exponent, electric_current_scale_p10, 
        temperature_exponent, temperature_scale_p10, 
        amount_of_substance_exponent, amount_of_substance_scale_p10, 
        luminous_intensity_exponent, luminous_intensity_scale_p10, 
        angle_exponent, angle_scale_p2, angle_scale_p3, angle_scale_p5, angle_scale_pi,
    ),
    (mass_exponent, length_exponent, time_exponent, electric_current_exponent, temperature_exponent, amount_of_substance_exponent, luminous_intensity_exponent, angle_exponent)
);