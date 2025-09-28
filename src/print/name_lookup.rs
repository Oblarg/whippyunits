use whippyunits_default_dimensions::{lookup_dimension_by_exponents, UNIT_LITERALS};

/// Configuration for a unit dimension
#[derive(Debug, Clone)]
pub struct UnitConfig {
    pub base_name: &'static str,
    pub base_symbol: &'static str,
    pub base_scale_offset: i16,
    pub offset_adjusted_name: Option<&'static str>,
    pub offset_adjusted_symbol: Option<&'static str>,
    pub unknown_text: Option<&'static str>,
}

impl UnitConfig {
    /// Get the appropriate name/symbol considering base scale offset
    fn get_display_name(&self, long_name: bool) -> &'static str {
        if self.base_scale_offset != 0 {
            // This unit has a base scale offset - use the offset-adjusted name/symbol
            if long_name {
                self.offset_adjusted_name.unwrap_or(self.base_name)
            } else {
                self.offset_adjusted_symbol.unwrap_or(self.base_symbol)
            }
        } else {
            // No base scale offset, use the standard name/symbol
            if long_name {
                self.base_name
            } else {
                self.base_symbol
            }
        }
    }
}

/// Get unit configuration for a specific dimension index
/// This maps the 8 basic SI dimensions to their unit configurations
fn get_unit_config(index: usize) -> UnitConfig {
    match index {
        0 => UnitConfig {
            // Mass
            base_name: "gram",
            base_symbol: "g",
            base_scale_offset: 3, // gram has -3 offset relative to kg
            offset_adjusted_name: Some("kilogram"),
            offset_adjusted_symbol: Some("kg"),
            unknown_text: None,
        },
        1 => UnitConfig {
            // Length
            base_name: "meter",
            base_symbol: "m",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        2 => UnitConfig {
            // Time
            base_name: "second",
            base_symbol: "s",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        3 => UnitConfig {
            // Current
            base_name: "ampere",
            base_symbol: "A",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        4 => UnitConfig {
            // Temperature
            base_name: "kelvin",
            base_symbol: "K",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        5 => UnitConfig {
            // Amount
            base_name: "mole",
            base_symbol: "mol",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        6 => UnitConfig {
            // Luminosity
            base_name: "candela",
            base_symbol: "cd",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        7 => UnitConfig {
            // Angle
            base_name: "radian",
            base_symbol: "rad",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        _ => panic!("Invalid dimension index: {}", index),
    }
}

/// Look up a unit literal by its dimension exponents and scale factors
/// Returns the unit name/symbol if found, otherwise None
fn lookup_unit_literal_by_scale_factors(
    exponents: &[i16],
    scale_factors: (i16, i16, i16, i16),
    long_name: bool,
) -> Option<&'static str> {
    // Convert Vec<i16> to tuple for comparison
    if exponents.len() != 8 {
        return None;
    }

    let exponents_tuple = (
        exponents[0],
        exponents[1],
        exponents[2],
        exponents[3],
        exponents[4],
        exponents[5],
        exponents[6],
        exponents[7],
    );

    UNIT_LITERALS
        .iter()
        .find(|unit_info| {
            unit_info.dimension_exponents == exponents_tuple
                && unit_info.scale_factors == scale_factors
                && unit_info.conversion_factor.is_none() // Only consider pure SI units, not imperial units
        })
        .map(|unit_info| {
            if long_name {
                unit_info.long_name
            } else {
                unit_info.symbol
            }
        })
}

/// Generate systematic unit name with scale factors
/// This version can look up unit literals by their scale factors
pub fn generate_systematic_unit_name_with_scale_factors(
    exponents: Vec<i16>,
    scale_factors: (i16, i16, i16, i16),
    long_name: bool,
) -> String {
    // Check if all exponents are unknown
    if exponents.iter().all(|&exp| exp == i16::MIN) {
        return "?".to_string();
    }

    // check if the unit is "pure" (e.g. if only one exponent is nonzero)
    let is_pure = exponents.iter().filter(|&exp| *exp != 0).count() == 1;

    // For pure units, first try to find a unit literal that matches the scale factors
    if is_pure {
        if let Some(unit_name) =
            lookup_unit_literal_by_scale_factors(&exponents, scale_factors, long_name)
        {
            return unit_name.to_string();
        }
    }

    // Fall back to the original logic
    generate_systematic_unit_name(exponents, long_name)
}

pub fn generate_systematic_unit_name(exponents: Vec<i16>, long_name: bool) -> String {
    generate_systematic_unit_name_with_format(
        exponents,
        long_name,
        crate::print::prettyprint::UnitFormat::Unicode,
    )
}

pub fn generate_systematic_unit_name_with_format(
    exponents: Vec<i16>,
    long_name: bool,
    format: crate::print::prettyprint::UnitFormat,
) -> String {
    // Helper function to get unicode exponent
    fn get_unicode_exponent(exp: i16) -> String {
        crate::print::utils::to_unicode_superscript(exp, false)
    }

    // Render a unit that is not mixed with any other units - in this case, we try to append the SI prefix to the base unit directly
    fn render_unit_part(
        unit_data: UnitConfig,
        exponent: i16,
        long_name: bool,
        is_pure_unit: bool,
        format: crate::print::prettyprint::UnitFormat,
    ) -> String {
        if exponent != 0 {
            let base_name = unit_data.get_display_name(long_name);
            let exponent_str = match format {
                crate::print::prettyprint::UnitFormat::Unicode => get_unicode_exponent(exponent),
                crate::print::prettyprint::UnitFormat::Ucum => {
                    if exponent == 1 {
                        String::new()
                    } else {
                        exponent.to_string()
                    }
                }
            };
            format!("{}{}", base_name, exponent_str)
        } else {
            "".to_string()
        }
    }

    // Check if all exponents are unknown
    if exponents.iter().all(|&exp| exp == i16::MIN) {
        return "?".to_string();
    }

    // check if the unit is "pure" (e.g. if only one exponent is nonzero)
    let is_pure = exponents.iter().filter(|&exp| *exp != 0).count() == 1;

    // For pure units, first try to find a unit literal that matches the scale factors
    if is_pure {
        // Find the non-zero exponent and its index
        if let Some((dimension_index, &exponent)) =
            exponents.iter().enumerate().find(|(_, &exp)| exp != 0)
        {
            // For time units, we need to check if there's a unit literal that matches
            // We'll need to get the scale factors from somewhere - for now, let's check if it's a time unit
            if dimension_index == 2 && exponent == 1 { // Time dimension with exponent 1
                 // This is a pure time unit - check if we can find a matching unit literal
                 // We need to get the scale factors from the context, but for now let's use a different approach
                 // Let's check if this is one of our known time units by looking at the scale factors
                 // For now, we'll fall back to the original logic but this needs to be enhanced
            }
        }
    }

    match format {
        crate::print::prettyprint::UnitFormat::Unicode => {
            // Original Unicode logic
            let unit_parts: Vec<String> = exponents
                .iter()
                .enumerate()
                .map(|(index, &exp)| {
                    let config = get_unit_config(index);
                    let part = render_unit_part(config, exp, long_name, is_pure, format);
                    part
                })
                .filter(|part| !part.is_empty())
                .collect();

            let result = unit_parts.join("·");

            if is_pure {
                result
            } else {
                format!("({})", result)
            }
        }
        crate::print::prettyprint::UnitFormat::Ucum => {
            // UCUM format: separate positive and negative exponents
            let mut numerator_parts = Vec::new();
            let mut denominator_parts = Vec::new();

            for (index, &exp) in exponents.iter().enumerate() {
                if exp != 0 && exp != i16::MIN {
                    let config = get_unit_config(index);
                    let base_name = config.get_display_name(long_name);

                    if exp > 0 {
                        // Positive exponents go in numerator
                        let exponent_str = if exp == 1 {
                            String::new()
                        } else {
                            exp.to_string()
                        };
                        numerator_parts.push(format!("{}{}", base_name, exponent_str));
                    } else {
                        // Negative exponents go in denominator
                        let exponent_str = if exp == -1 {
                            String::new()
                        } else {
                            (-exp).to_string()
                        };
                        denominator_parts.push(format!("{}{}", base_name, exponent_str));
                    }
                }
            }

            // Construct the final unit string
            let mut unit_string = String::new();

            if numerator_parts.is_empty() && denominator_parts.is_empty() {
                return "1".to_string();
            }

            if numerator_parts.is_empty() {
                unit_string.push('1');
            } else {
                unit_string.push_str(&numerator_parts.join("."));
            }

            if !denominator_parts.is_empty() {
                if denominator_parts.len() == 1 {
                    unit_string.push_str(&format!("/{}", denominator_parts[0]));
                } else {
                    unit_string.push_str(&format!("/{}", denominator_parts.join(".")));
                }
            }

            unit_string
        }
    }
}

pub struct DimensionNames {
    pub dimension_name: &'static str,
    // only for dimensions with semi-systematic simplified names (e.g. Energy, Joule/J)
    pub unit_si_shortname_symbol: Option<&'static str>,
    pub unit_si_shortname: Option<&'static str>,
}

pub fn lookup_dimension_name(exponents: Vec<i16>) -> Option<DimensionNames> {
    // Convert Vec<i16> to tuple for lookup
    if exponents.len() != 8 {
        return None;
    }

    let exponents_tuple = (
        exponents[0],
        exponents[1],
        exponents[2],
        exponents[3],
        exponents[4],
        exponents[5],
        exponents[6],
        exponents[7],
    );

    // Use the shared dimension data as the source of truth
    lookup_dimension_by_exponents(exponents_tuple).map(|dim_info| DimensionNames {
        dimension_name: dim_info.name,
        unit_si_shortname_symbol: dim_info.si_symbol,
        unit_si_shortname: dim_info.si_long_name,
    })
}
