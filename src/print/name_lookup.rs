use crate::print::utils::get_si_prefix;

pub fn generate_systematic_unit_name(
    mass_exponent: i8, mass_scale_p10: i8,
    length_exponent: i8, length_scale_p10: i8,
    time_exponent: i8, time_scale_p2: i8, time_scale_p3: i8, time_scale_p5: i8,
    long_name: bool,
) -> String {
    // Helper function to get unicode exponent
    fn get_unicode_exponent(exp: i8) -> String {
        crate::print::utils::to_unicode_superscript(exp, false)
    }
    
    // Helper function to render a unit with optional SI prefix or scale notation
    fn render_unit_with_scale(
        unit_string: &mut String,
        scale_p10: i8,
        base_unit: &str,
        exponent_str: &str,
        long_name: bool,
    ) {
        // Check if scale is unknown (i8::MIN)
        if scale_p10 == i8::MIN {
            // Unknown scale, show as "?" to indicate unresolved
            unit_string.push_str("?");
            return;
        }
        
        // For mass units, adjust scale by +3 since base unit is "gram" but scales are relative to "kilogram"
        let adjusted_scale = if base_unit == "gram" || base_unit == "g" {
            scale_p10 + 3
        } else {
            scale_p10
        };
        
        let prefix = get_si_prefix(adjusted_scale, long_name);
        
        if let Some(prefix) = prefix {
            unit_string.push_str(prefix);
            unit_string.push_str(base_unit);
        } else {
            // No SI prefix available, render as base unit times 10^scale
            if adjusted_scale != 0 {
                unit_string.push_str("(10");
                unit_string.push_str(&get_unicode_exponent(adjusted_scale));
                unit_string.push_str(" ");
                unit_string.push_str(base_unit);
                unit_string.push_str(")");
            } else {
                unit_string.push_str(base_unit);
            }
        }
        unit_string.push_str(exponent_str);
    }

    // Check if all exponents are unknown
    if mass_exponent == i8::MIN && length_exponent == i8::MIN && time_exponent == i8::MIN {
        return "?".to_string();
    }
    
    // Build the complete unit string by concatenating literals as we go
    let mut unit_string = String::new();
    
    // Add mass literal if mass exponent is nonzero
    if mass_exponent != 0 {
        let mass_base = if long_name { "gram" } else { "g" };
        let mass_exp = get_unicode_exponent(mass_exponent);
        
        render_unit_with_scale(&mut unit_string, mass_scale_p10, mass_base, &mass_exp, long_name);
    }
    
    // Add length literal if length exponent is nonzero
    if length_exponent != 0 {
        // Add multiplication dot separator if we already have mass
        if !unit_string.is_empty() {
            unit_string.push('·');
        }
        
        let length_base = if long_name { "meter" } else { "m" };
        let length_exp = get_unicode_exponent(length_exponent);
        
        render_unit_with_scale(&mut unit_string, length_scale_p10, length_base, &length_exp, long_name);
    }
    
    // Add time literal if time exponent is nonzero
    if time_exponent != 0 {
        // Add multiplication dot separator if we already have mass or length
        if !unit_string.is_empty() {
            unit_string.push('·');
        }
        
        // Check if this is a power of 10 case (p2 = p5, p3 = 0)
        if time_scale_p2 == time_scale_p5 && time_scale_p3 == 0 {
            // This is a power of 10 case, treat as SI-prefixed seconds
            let time_base = if long_name { "second" } else { "s" };
            let time_exp = get_unicode_exponent(time_exponent);
            
            render_unit_with_scale(&mut unit_string, time_scale_p2, time_base, &time_exp, long_name);
        } else {
            // Not a power of 10 case, return unknown time unit
            if long_name {
                unit_string.push_str("unknown time unit");
            } else {
                unit_string.push_str("t?");
            }
        }
    }
    
    unit_string
}

pub struct DimensionNames {
    pub dimension_name: &'static str,
    // only for dimensions with semi-systematic simplified names (e.g. Energy, Joule/J)
    pub unit_si_shortname_symbol: Option<&'static str>,
    pub unit_si_shortname: Option<&'static str>,
}

pub fn lookup_dimension_name(
    mass_exponent: i8,
    length_exponent: i8,
    time_exponent: i8,
) -> Option<DimensionNames> {
    match (mass_exponent, length_exponent, time_exponent) {
        (1, 0, 0) => {
            Some(DimensionNames {
                dimension_name: "Mass",
                unit_si_shortname_symbol: None,
                unit_si_shortname: None,
            })
        },
        (0, 1, 0) => {
            Some(DimensionNames {
                dimension_name: "Length",
                unit_si_shortname_symbol: None,
                unit_si_shortname: None,
            })
        },
        (0, 0, 1) => {
            Some(DimensionNames {
                dimension_name: "Time",
                unit_si_shortname_symbol: None,
                unit_si_shortname: None,
            })
        },
        (0, 0, -1) => {
            Some(DimensionNames {
                dimension_name: "Frequency",
                unit_si_shortname_symbol: Some("Hz"),
                unit_si_shortname: Some("Hertz"),
            })
        },
        (0, 2, 0) => {
            Some(DimensionNames {
                dimension_name: "Area",
                unit_si_shortname_symbol: None,
                unit_si_shortname: None,
            })
        },
        (0, -1, 0) => {
            Some(DimensionNames {
                dimension_name: "Inverse Length",
                unit_si_shortname_symbol: None,
                unit_si_shortname: None,
            })
        },
        (1, 2, -2) => {
            Some(DimensionNames {
                dimension_name: "Energy",
                unit_si_shortname_symbol: Some("J"),
                unit_si_shortname: Some("Joule"),
            })
        },
        _ => None,
    }
}