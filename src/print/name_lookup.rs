use whippyunits_default_dimensions::lookup_dimension_by_exponents;

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
        0 => UnitConfig { // Mass
            base_name: "gram",
            base_symbol: "g", 
            base_scale_offset: 3, // gram has -3 offset relative to kg
            offset_adjusted_name: Some("kilogram"),
            offset_adjusted_symbol: Some("kg"),
            unknown_text: None,
        },
        1 => UnitConfig { // Length
            base_name: "meter",
            base_symbol: "m",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        2 => UnitConfig { // Time
            base_name: "second",
            base_symbol: "s",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        3 => UnitConfig { // Current
            base_name: "ampere",
            base_symbol: "A",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        4 => UnitConfig { // Temperature
            base_name: "kelvin",
            base_symbol: "K",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        5 => UnitConfig { // Amount
            base_name: "mole",
            base_symbol: "mol",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        6 => UnitConfig { // Luminosity
            base_name: "candela",
            base_symbol: "cd",
            base_scale_offset: 0,
            offset_adjusted_name: None,
            offset_adjusted_symbol: None,
            unknown_text: None,
        },
        7 => UnitConfig { // Angle
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

pub fn generate_systematic_unit_name(
    exponents: Vec<i16>,
    long_name: bool
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
    ) -> String { 
        if exponent != 0 {
            let base_name = unit_data.get_display_name(long_name);
            format!("{}{}", base_name, get_unicode_exponent(exponent))
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
    
    // Map all exponents to corresponding unit configs, render, gather, then join
    let unit_parts: Vec<String> = exponents
        .iter()
        .enumerate()
        .map(|(index, &exp)| {
            let config = get_unit_config(index);
            
            // TODO: hook into method that handles 2s and 5s
            let part = render_unit_part(config, exp, long_name, is_pure);
               
            part
        })
        .filter(|part| !part.is_empty())  // Filter out empty parts
        .collect();

    let result = unit_parts.join("·");

    if is_pure {
        result
    } else {
        format!("({})", result)
    }
}

pub struct DimensionNames {
    pub dimension_name: &'static str,
    // only for dimensions with semi-systematic simplified names (e.g. Energy, Joule/J)
    pub unit_si_shortname_symbol: Option<&'static str>,
    pub unit_si_shortname: Option<&'static str>,
}

pub fn lookup_dimension_name(
    exponents: Vec<i16>,
) -> Option<DimensionNames> {
    // Convert Vec<i16> to tuple for lookup
    if exponents.len() != 8 {
        return None;
    }
    
    let exponents_tuple = (exponents[0], exponents[1], exponents[2], exponents[3], 
                          exponents[4], exponents[5], exponents[6], exponents[7]);
    
    // Use the shared dimension data as the source of truth
    lookup_dimension_by_exponents(exponents_tuple).map(|dim_info| DimensionNames {
        dimension_name: dim_info.name,
        unit_si_shortname_symbol: dim_info.si_symbol,
        unit_si_shortname: dim_info.si_long_name,
    })
}