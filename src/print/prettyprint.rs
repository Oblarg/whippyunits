use crate::print::name_lookup::generate_systematic_unit_name;
use crate::print::name_lookup::lookup_dimension_name;
use crate::print::utils::{to_unicode_superscript, get_si_prefix};

/// Generate dimension symbols for unresolved types (M, L, T)
pub fn generate_dimension_symbols(mass_exponent: i8, length_exponent: i8, time_exponent: i8) -> String {
    let mut parts = Vec::new();
    
    // Add mass dimension
    if mass_exponent != 0 {
        parts.push(format!("M{}", to_unicode_superscript(mass_exponent, false)));
    }
    
    // Add length dimension
    if length_exponent != 0 {
        parts.push(format!("L{}", to_unicode_superscript(length_exponent, false)));
    }
    
    // Add time dimension
    if time_exponent != 0 {
        parts.push(format!("T{}", to_unicode_superscript(time_exponent, false)));
    }
    
    if parts.is_empty() {
        "?".to_string()
    } else {
        parts.join("·")
    }
}

/// Generate verbose dimension names for unresolved types (Length, Time, Mass)
pub fn generate_verbose_dimension_names(mass_exponent: i8, length_exponent: i8, time_exponent: i8) -> String {
    let mut parts = Vec::new();
    
    // Add mass dimension
    if mass_exponent != 0 {
        let name = if mass_exponent == 1 { "Mass" } else { "Mass" };
        if mass_exponent != 1 {
            parts.push(format!("{}{}", name, to_unicode_superscript(mass_exponent, false)));
        } else {
            parts.push(name.to_string());
        }
    }
    
    // Add length dimension
    if length_exponent != 0 {
        let name = if length_exponent == 1 { "Length" } else { "Length" };
        if length_exponent != 1 {
            parts.push(format!("{}{}", name, to_unicode_superscript(length_exponent, false)));
        } else {
            parts.push(name.to_string());
        }
    }
    
    // Add time dimension
    if time_exponent != 0 {
        let name = if time_exponent == 1 { "Time" } else { "Time" };
        if time_exponent != 1 {
            parts.push(format!("{}{}", name, to_unicode_superscript(time_exponent, false)));
        } else {
            parts.push(name.to_string());
        }
    }
    
    if parts.is_empty() {
        "?".to_string()
    } else {
        parts.join("·")
    }
}

/// Calculate total power of 10 across all dimensions
fn calculate_total_scale_p10(
    mass_exponent: i8, mass_scale_p10: i8,
    length_exponent: i8, length_scale_p10: i8,
    time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
) -> i8 {
    let mut total_scale_p10 = 0;
    
    // Add mass contribution: exponent × scale
    if mass_exponent != 0 {
        // Mass scales are relative to kilograms (SI base unit)
        total_scale_p10 += mass_exponent * mass_scale_p10;
    }
    
    // Add length contribution: exponent × scale
    if length_exponent != 0 {
        total_scale_p10 += length_exponent * length_scale_p10;
    }
    
    // Add time contribution: only if it's a power of 10 case
    if time_exponent != 0 && time_scale_p2 == time_scale_p5 && time_scale_p3 == 0 {
        total_scale_p10 += time_exponent * time_scale_p2;
    }
    
    total_scale_p10
}

/// Generate SI unit with 10^n notation when no standard prefix is available
fn generate_si_unit_with_scale(
    total_scale_p10: i8,
    base_si_unit: &str,
    _long_name: bool,
) -> String {
    if total_scale_p10 == 0 {
        base_si_unit.to_string()
    } else {
        let superscript = to_unicode_superscript(total_scale_p10, false);
        format!("10{} {}", superscript, base_si_unit)
    }
}



/// Generate correctly-prefixed SI unit name
fn generate_prefixed_si_unit(
    mass_exponent: i8, mass_scale_p10: i8,
    length_exponent: i8, length_scale_p10: i8,
    time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
    base_si_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(
        mass_exponent, mass_scale_p10,
        length_exponent, length_scale_p10,
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
    );
    
    if let Some(prefix) = get_si_prefix(total_scale_p10, long_name) {
        format!("{}{}", prefix, base_si_unit)
    } else {
        // Fall back to SI unit with 10^n notation when SI prefix lookup fails
        generate_si_unit_with_scale(total_scale_p10, base_si_unit, long_name)
    }
}

/// Helper function to format scale values, handling sentinel values
fn format_scale_value(scale: i8) -> String {
    if scale == i8::MAX {
        "unused".to_string()
    } else {
        scale.to_string()
    }
}



/// Pretty print a quantity with full information
/// 
/// # Arguments
/// * `value` - Optional numeric value (None for type-only display)
/// * `mass_exponent` - Mass dimension exponent
/// * `mass_scale_p10` - Mass scale (power of 10)
/// * `length_exponent` - Length dimension exponent  
/// * `length_scale_p10` - Length scale (power of 10)
/// * `time_exponent` - Time dimension exponent
/// * `time_scale_p2` - Time scale (power of 2)
/// * `time_scale_p3` - Time scale (power of 3)
/// * `time_scale_p5` - Time scale (power of 5)
/// * `verbose` - Whether to show verbose output with all details
/// 
/// # Returns
/// Formatted string in the format: `(value) Quantity<systematic_literal, unit_shortname, dimension_name, [exponents and scales]>`
pub fn pretty_print_quantity(
    value: Option<f64>,
    mass_exponent: i8,
    mass_scale_p10: i8,
    length_exponent: i8,
    length_scale_p10: i8,
    time_exponent: i8,
    time_scale_p2: i8,
    time_scale_p3: i8,
    time_scale_p5: i8,
    verbose: bool,
) -> String {
    let mut result = String::new();
    
    // Add value if provided
    if let Some(val) = value {
        result.push_str(&format!("({}) ", val));
    }
    
    // Generate systematic unit literal
    let systematic_literal = generate_systematic_unit_name(
        mass_exponent, mass_scale_p10,
        length_exponent, length_scale_p10,
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
        verbose, // Use full names in verbose mode, symbols in non-verbose mode
    );
    
    // Look up dimension name
    let dimension_info = lookup_dimension_name(mass_exponent, length_exponent, time_exponent);
    
    // Generate SI shortname - use dimension-specific SI unit if available, otherwise don't show a shortname
    let unit_shortname = if let Some(ref info) = dimension_info {
        if let Some(base_si_unit) = if verbose {
            info.unit_si_shortname
        } else {
            info.unit_si_shortname_symbol
        } {
            // Use the specific SI unit name with correct prefix (e.g., "μJ" for microjoule)
            generate_prefixed_si_unit(
                mass_exponent, mass_scale_p10,
                length_exponent, length_scale_p10,
                time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
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
            generate_verbose_dimension_names(mass_exponent, length_exponent, time_exponent)
        } else {
            // For unrecognized dimensions, compute dimension symbol dynamically from exponents
            generate_dimension_symbols(mass_exponent, length_exponent, time_exponent)
        }
    };
    
    // Build the main format
    result.push_str("Quantity<");
    
    // If systematic literal is empty, use dimension name as fallback
    if systematic_literal.is_empty() {
        result.push_str(&dimension_name);
    } else {
        result.push_str(&systematic_literal);
    }
    
    // Only show SI shortname if it's different from the systematic literal
    if !unit_shortname.is_empty() && unit_shortname != systematic_literal {
        result.push_str("; ");
        result.push_str(&unit_shortname);
        result.push_str("; ");
        result.push_str(&dimension_name);
    } else {
        // If unit_shortname equals systematic literal, don't show redundant information
        // Just show the systematic literal and dimension name
        if !systematic_literal.is_empty() {
            result.push_str("; ");
            result.push_str(&dimension_name);
        }
    }
    
    // Add exponents and scales if verbose
    if verbose {
        result.push_str("; [");
        result.push_str(&format!("mass{}", to_unicode_superscript(mass_exponent, true)));
        if mass_scale_p10 == i8::MAX {
            result.push_str("(unused)");
        } else if mass_scale_p10 == i8::MIN {
            result.push_str("(10ˀ)");
        } else {
            result.push_str(&format!("(10{})", to_unicode_superscript(mass_scale_p10, false)));
        }
        result.push_str(&format!(", length{}", to_unicode_superscript(length_exponent, true)));
        if length_scale_p10 == i8::MAX {
            result.push_str("(unused)");
        } else if length_scale_p10 == i8::MIN {
            result.push_str("(10ˀ)");
        } else {
            result.push_str(&format!("(10{})", to_unicode_superscript(length_scale_p10, false)));
        }
        result.push_str(&format!(", time{}", to_unicode_superscript(time_exponent, true)));
        // Check if all time scales are unused
        if time_scale_p2 == i8::MAX && time_scale_p3 == i8::MAX && time_scale_p5 == i8::MAX {
            result.push_str("(unused)");
        } else {
            result.push_str("(2");
            if time_scale_p2 == i8::MAX {
                result.push_str("unused");
            } else if time_scale_p2 == i8::MIN {
                result.push_str("ˀ");
            } else {
                result.push_str(&to_unicode_superscript(time_scale_p2, false));
            }
            result.push_str(", 3");
            if time_scale_p3 == i8::MAX {
                result.push_str("unused");
            } else if time_scale_p3 == i8::MIN {
                result.push_str("ˀ");
            } else {
                result.push_str(&to_unicode_superscript(time_scale_p3, false));
            }
            result.push_str(", 5");
            if time_scale_p5 == i8::MAX {
                result.push_str("unused");
            } else if time_scale_p5 == i8::MIN {
                result.push_str("ˀ");
            } else {
                result.push_str(&to_unicode_superscript(time_scale_p5, false));
            }
            result.push_str(")");
        }
        result.push_str(")]");
    }
    
    result.push_str(">");
    
    result
}

/// Pretty print a quantity type (without value)
pub fn pretty_print_quantity_type(
    mass_exponent: i8,
    mass_scale_p10: i8,
    length_exponent: i8,
    length_scale_p10: i8,
    time_exponent: i8,
    time_scale_p2: i8,
    time_scale_p3: i8,
    time_scale_p5: i8,
    verbose: bool,
) -> String {
    pretty_print_quantity(
        None,
        mass_exponent, mass_scale_p10,
        length_exponent, length_scale_p10,
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
        verbose,
    )
}

/// Pretty print a quantity value (with value)
pub fn pretty_print_quantity_value(
    value: f64,
    mass_exponent: i8,
    mass_scale_p10: i8,
    length_exponent: i8,
    length_scale_p10: i8,
    time_exponent: i8,
    time_scale_p2: i8,
    time_scale_p3: i8,
    time_scale_p5: i8,
    verbose: bool,
) -> String {
    pretty_print_quantity(
        Some(value),
        mass_exponent, mass_scale_p10,
        length_exponent, length_scale_p10,
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
        verbose,
    )
}

/// Ultra-terse pretty print for inlay hints - shows only the unit literal
pub fn pretty_print_quantity_inlay_hint(
    mass_exponent: i8,
    mass_scale_p10: i8,
    length_exponent: i8,
    length_scale_p10: i8,
    time_exponent: i8,
    time_scale_p2: i8,
    time_scale_p3: i8,
    time_scale_p5: i8,
) -> String {
    // Generate systematic unit literal (this is the unit name like "mm", "kg", etc.)
    let systematic_literal = generate_systematic_unit_name(
        mass_exponent, mass_scale_p10,
        length_exponent, length_scale_p10,
        time_exponent, time_scale_p2, time_scale_p3, time_scale_p5,
        false, // Use short names for inlay hints
    );
    
    // Look up dimension name to check if we have a specific SI unit
    let dimension_info = lookup_dimension_name(mass_exponent, length_exponent, time_exponent);
    
    // If we have a specific SI unit that's different from the systematic literal, use it
    if let Some(ref info) = dimension_info {
        if let Some(si_shortname) = info.unit_si_shortname_symbol {
            // Use the SI shortname if it's different from the systematic literal
            if si_shortname != systematic_literal {
                return si_shortname.to_string();
            }
        }
    }
    
    // Otherwise, return the systematic literal
    systematic_literal
}
