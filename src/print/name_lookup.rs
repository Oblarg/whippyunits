use whippyunits_core::{
    Dimension, dimension_exponents::DynDimensionExponents, scale_exponents::ScaleExponents,
};

/// Look up a unit literal by its dimension exponents and scale factors
/// Returns the unit name/symbol if found, otherwise None
fn lookup_unit_literal_by_scale_factors(
    exponents: &[i16],
    scale_factors: ScaleExponents,
    long_name: bool,
) -> Option<&'static str> {
    // Convert Vec<i16> to DynDimensionExponents for comparison
    if exponents.len() != 8 {
        return None;
    }

    let dyn_exponents = DynDimensionExponents([
        exponents[0],
        exponents[1],
        exponents[2],
        exponents[3],
        exponents[4],
        exponents[5],
        exponents[6],
        exponents[7],
    ]);

    if let Some(dimension) = Dimension::find_dimension_by_exponents(dyn_exponents) {
        dimension
            .units
            .iter()
            .find(|unit| {
                unit.scale == scale_factors && unit.conversion_factor == 1.0 // Only consider pure SI units, not imperial units
            })
            .map(|unit| {
                if long_name {
                    unit.name
                } else {
                    unit.symbols[0]
                }
            })
    } else {
        None
    }
}

/// Generate systematic unit name with scale factors
/// This version can look up unit literals by their scale factors
pub fn generate_systematic_unit_name_with_scale_factors(
    exponents: Vec<i16>,
    scale_factors: ScaleExponents,
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
        unit_name: &'static str,
        unit_symbol: &'static str,
        base_scale_offset: i16,
        exponent: i16,
        long_name: bool,
        format: crate::print::prettyprint::UnitFormat,
    ) -> String {
        if exponent != 0 {
            // Use the base unit name/symbol directly - let the prefix system handle scaling
            let base_name = if long_name { unit_name } else { unit_symbol };

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

    match format {
        crate::print::prettyprint::UnitFormat::Unicode => {
            // Original Unicode logic
            let unit_parts: Vec<String> = exponents
                .iter()
                .enumerate()
                .map(|(index, &exp)| {
                    // Get unit configuration directly from Dimension::BASIS
                    let (unit_name, unit_symbol, base_scale_offset) =
                        if let Some(dimension) = Dimension::BASIS.get(index) {
                            if let Some(unit) = dimension.units.first() {
                                // Get the base scale offset from the unit's scale (systematic approach)
                                // For gram, this will be -3 (10^-3 of kilogram)
                                let base_scale_offset = unit.scale.log10().unwrap_or(0);
                                (unit.name, unit.symbols[0], base_scale_offset)
                            } else {
                                ("?", "?", 0)
                            }
                        } else {
                            ("?", "?", 0)
                        };
                    let part = render_unit_part(
                        unit_name,
                        unit_symbol,
                        base_scale_offset,
                        exp,
                        long_name,
                        format,
                    );
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
                    // Get unit configuration directly from Dimension::BASIS
                    let (unit_name, unit_symbol, base_scale_offset) =
                        if let Some(dimension) = Dimension::BASIS.get(index) {
                            if let Some(unit) = dimension.units.first() {
                                // Get the base scale offset from the unit's scale (systematic approach)
                                // For gram, this will be -3 (10^-3 of kilogram)
                                let base_scale_offset = unit.scale.log10().unwrap_or(0);
                                (unit.name, unit.symbols[0], base_scale_offset)
                            } else {
                                ("?", "?", 0)
                            }
                        } else {
                            ("?", "?", 0)
                        };

                    let base_name = if base_scale_offset != 0 {
                        if long_name {
                            match unit_name {
                                "gram" => "kilogram",
                                _ => unit_name,
                            }
                        } else {
                            match unit_symbol {
                                "g" => "kg",
                                _ => unit_symbol,
                            }
                        }
                    } else {
                        if long_name { unit_name } else { unit_symbol }
                    };

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
    // Convert Vec<i16> to DynDimensionExponents for lookup
    if exponents.len() != 8 {
        return None;
    }

    let dyn_exponents = DynDimensionExponents([
        exponents[0],
        exponents[1],
        exponents[2],
        exponents[3],
        exponents[4],
        exponents[5],
        exponents[6],
        exponents[7],
    ]);

    // Use Dimension::find_dimension_by_exponents directly
    Dimension::find_dimension_by_exponents(dyn_exponents).and_then(|dim_info| {
        // For pure atomic dimensions (like area = length²), prefer systematic generation
        // over predefined units when we have exact matches of atomic unit exponents
        let is_pure_atomic = exponents.iter().filter(|&exp| *exp != 0).count() == 1;
        
        if is_pure_atomic {
            // For pure atomic dimensions, check if we have an exact match with identity scale factors
            let has_exact_match = dim_info
                .units
                .iter()
                .any(|unit| {
                    unit.scale == whippyunits_core::scale_exponents::ScaleExponents::IDENTITY
                    && unit.conversion_factor == 1.0
                });
            
            if !has_exact_match {
                // No exact match found, return None to force systematic generation
                return None;
            }
        }
        
        // Prioritize exact matches of atomic unit exponents (scale factors of [0, 0, 0, 0])
        // over the first unit in the lexical list
        let preferred_unit = dim_info
            .units
            .iter()
            .find(|unit| {
                unit.scale == whippyunits_core::scale_exponents::ScaleExponents::IDENTITY
                && unit.conversion_factor == 1.0
            })
            .or_else(|| dim_info.units.first()); // Fall back to first unit if no exact match

        let unit_symbol = preferred_unit
            .and_then(|unit| unit.symbols.first().copied());
        let unit_long_name = preferred_unit.map(|unit| unit.name);

        Some(DimensionNames {
            dimension_name: dim_info.name,
            unit_si_shortname_symbol: unit_symbol, // Use actual unit symbol (e.g., "J") instead of dimension symbol (e.g., "ML²T⁻²")
            unit_si_shortname: unit_long_name, // Use unit long name (e.g., "joule") instead of dimension name (e.g., "Energy")
        })
    })
}
