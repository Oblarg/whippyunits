use crate::{
    Dimension, SiPrefix, dimension_exponents::DynDimensionExponents,
    scale_exponents::ScaleExponents,
};

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::format;
#[cfg(not(test))]
use alloc::string::String;
#[cfg(not(test))]
use alloc::string::ToString;
#[cfg(not(test))]
use alloc::vec::Vec;

/// Configuration for unit literal generation
#[derive(Debug, Clone, Copy)]
pub struct UnitLiteralConfig {
    pub verbose: bool,
    pub prefer_si_units: bool,
}

impl Default for UnitLiteralConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            prefer_si_units: true,
        }
    }
}

/// Generate the best unit literal for a given set of dimensions and scales
/// This is the exact logic from the main crate's unit_literal_generator
pub fn generate_unit_literal(
    exponents: DynDimensionExponents,
    scale_factors: ScaleExponents,
    config: UnitLiteralConfig,
) -> String {
    // Convert DynDimensionExponents to Vec<i16> for compatibility with existing functions
    let exponents_vec = exponents.0.to_vec();

    // Generate systematic unit literal (base unit without prefix)
    let base_systematic_literal = generate_systematic_unit_name_with_scale_factors(
        exponents_vec.clone(),
        scale_factors,
        config.verbose,
    );

    // Check if we found a unit literal match - if so, use it directly without conversion factor
    let pure_systematic = generate_systematic_unit_name(exponents_vec.clone(), config.verbose);
    let found_unit_literal = base_systematic_literal != pure_systematic;
    let systematic_literal = if found_unit_literal {
        // We found a unit literal match, use it directly
        base_systematic_literal
    } else {
        // No unit literal match, apply SI prefix to the systematic unit literal
        generate_prefixed_systematic_unit(
            exponents,
            scale_factors,
            &base_systematic_literal,
            config.verbose,
        )
    };

    // If we don't prefer SI units, return the systematic literal
    if !config.prefer_si_units {
        return systematic_literal;
    }

    // Check if we have a recognized dimension with a specific SI unit
    if let Some(info) = lookup_dimension_name(exponents_vec) {
        if let Some(si_shortname) = if config.verbose {
            info.unit_si_shortname
        } else {
            info.unit_si_shortname_symbol
        } {
            // Apply SI prefix to the specific SI unit name
            let prefixed_si_unit =
                generate_prefixed_si_unit(scale_factors, si_shortname, config.verbose);

            // Return the prefixed SI unit if it's different from the systematic literal
            // But only if we didn't find a unit literal match
            if prefixed_si_unit != systematic_literal && !found_unit_literal {
                prefixed_si_unit
            } else {
                systematic_literal
            }
        } else {
            // No specific SI unit defined, use the systematic literal
            systematic_literal
        }
    } else {
        // Unknown dimension, use the systematic literal
        systematic_literal
    }
}

/// Calculate the storage unit name from scale factors and dimension exponents
/// This is the canonical implementation used by both proc macros and LSP proxy
pub fn get_storage_unit_name(
    scale_factors: ScaleExponents,
    dimension_exponents: DynDimensionExponents,
    long_name: bool,
) -> String {
    // Use the exact same logic as the prettyprint
    let unit_literal = generate_unit_literal(
        dimension_exponents,
        scale_factors,
        UnitLiteralConfig {
            verbose: long_name,
            prefer_si_units: true,
        },
    );

    // If we got a unit literal, use it; otherwise fall back to systematic generation
    if !unit_literal.is_empty() {
        unit_literal
    } else {
        // Fallback to systematic generation
        let exponents_vec = dimension_exponents.0.to_vec();
        generate_systematic_unit_name(exponents_vec, long_name)
    }
}

// Supporting functions moved from the main crate to ensure identical logic

/// Generate systematic unit name with scale factors
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

/// Generate systematic unit name
pub fn generate_systematic_unit_name(exponents: Vec<i16>, long_name: bool) -> String {
    generate_systematic_unit_name_with_format(exponents, long_name, UnitFormat::Unicode)
}

/// Unit format enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitFormat {
    Unicode,
    Ucum,
}

/// Generate systematic unit name with format
pub fn generate_systematic_unit_name_with_format(
    exponents: Vec<i16>,
    long_name: bool,
    format: UnitFormat,
) -> String {
    // Convert Vec<i16> to DynDimensionExponents for the core function
    if exponents.len() != 8 {
        return "?".to_string();
    }

    let dimension_exponents = DynDimensionExponents([
        exponents[0],
        exponents[1],
        exponents[2],
        exponents[3],
        exponents[4],
        exponents[5],
        exponents[6],
        exponents[7],
    ]);

    // Use the centralized logic from whippyunits-core
    let base_result = generate_systematic_composite_unit_name(dimension_exponents, long_name);

    // Apply format-specific transformations
    match format {
        UnitFormat::Unicode => base_result,
        UnitFormat::Ucum => {
            // Convert Unicode format to UCUM format
            convert_unicode_to_ucum_format(&base_result)
        }
    }
}

/// Convert Unicode format unit string to UCUM format
fn convert_unicode_to_ucum_format(unicode_unit: &str) -> String {
    // This is a simplified conversion - in practice, you might need more sophisticated logic
    // For now, just return the unicode format as-is since the core logic already handles
    // the basic formatting correctly
    unicode_unit.to_string()
}

/// Look up a unit literal by its dimension exponents and scale factors
pub fn lookup_unit_literal_by_scale_factors(
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
        // First, try to find a unit that matches the exact scale factors
        // This is the preferred approach - use exact matches when possible
        if let Some(exact_unit) = dimension.units.iter().find(|unit| {
            unit.scale == scale_factors && unit.conversion_factor == 1.0 // Only consider pure SI units, not imperial units
        }) {
            return Some(if long_name {
                exact_unit.name
            } else {
                exact_unit.symbols[0]
            });
        }

        // If no exact match found, fall back to base unit (identity scale)
        dimension
            .units
            .iter()
            .find(|unit| unit.scale == ScaleExponents::IDENTITY && unit.conversion_factor == 1.0)
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

/// Dimension names struct
pub struct DimensionNames {
    pub dimension_name: &'static str,
    pub unit_si_shortname_symbol: Option<&'static str>,
    pub unit_si_shortname: Option<&'static str>,
}

/// Look up dimension name
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
            let has_exact_match = dim_info.units.iter().any(|unit| {
                unit.scale == ScaleExponents::IDENTITY && unit.conversion_factor == 1.0
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
            .find(|unit| unit.scale == ScaleExponents::IDENTITY && unit.conversion_factor == 1.0)
            .or_else(|| dim_info.units.first()); // Fall back to first unit if no exact match

        let unit_symbol = preferred_unit.and_then(|unit| unit.symbols.first().copied());
        let unit_long_name = preferred_unit.map(|unit| unit.name);

        Some(DimensionNames {
            dimension_name: dim_info.name,
            unit_si_shortname_symbol: unit_symbol, // Use actual unit symbol (e.g., "J") instead of dimension symbol (e.g., "ML²T⁻²")
            unit_si_shortname: unit_long_name, // Use unit long name (e.g., "joule") instead of dimension name (e.g., "Energy")
        })
    })
}

/// Generate prefixed systematic unit
pub fn generate_prefixed_systematic_unit(
    _exponents: DynDimensionExponents,
    scale_factors: ScaleExponents,
    base_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(
        scale_factors.0[0],
        scale_factors.0[1],
        scale_factors.0[2],
        scale_factors.0[3],
    );

    // Check if this is a pure unit (not compound)
    let is_pure_unit = !base_unit.contains("·");

    // For pure units, check if we need to apply base scale offset
    let effective_scale_p10 = if is_pure_unit {
        // Find the base scale offset by looking up the unit's scale from whippyunits-core
        // The base_unit comes from systematic unit name generation, so it should always be valid
        let base_scale_offset = Dimension::find_unit_by_symbol(base_unit)
            .or_else(|| Dimension::find_unit_by_name(base_unit))
            .map(|(_unit, _dimension)| _unit.scale.log10().unwrap_or(0))
            .unwrap_or(0);

        // Apply the base scale offset to the scale calculation
        // The base scale offset represents the offset of the base unit (e.g., gram = -3)
        // We need to subtract it from the total scale to get the effective scale
        total_scale_p10 - base_scale_offset
    } else {
        // For compound units, don't apply base scale offset to the aggregate prefix
        // The individual parts already have their base scale offsets applied
        total_scale_p10
    };

    if let Some(prefix) = get_si_prefix(effective_scale_p10, long_name) {
        // For pure powers of base units, add disambiguating parentheses
        if !base_unit.contains("·")
            && (base_unit.contains("^") || base_unit.contains("²") || base_unit.contains("³"))
        {
            format!("{}({})", prefix, base_unit)
        } else {
            format!("{}{}", prefix, base_unit)
        }
    } else {
        // Check if this is a pure power of 10 using whippyunits-core
        let is_pure_power_of_10 = scale_factors.log10().is_some();

        if is_pure_power_of_10 {
            // Fall back to SI unit with 10^n notation when SI prefix lookup fails
            generate_si_unit_with_scale(effective_scale_p10, base_unit, long_name)
        } else {
            // Not a pure power of 10, show the scale factors explicitly
            let scale_factors_str = format_scale_factors(
                scale_factors.0[0],
                scale_factors.0[1],
                scale_factors.0[2],
                scale_factors.0[3],
            );
            if scale_factors_str.is_empty() {
                base_unit.to_string()
            } else {
                format!("{}{}", scale_factors_str, base_unit)
            }
        }
    }
}

/// Generate prefixed SI unit
pub fn generate_prefixed_si_unit(
    scale_factors: ScaleExponents,
    base_si_unit: &str,
    long_name: bool,
) -> String {
    let total_scale_p10 = calculate_total_scale_p10(
        scale_factors.0[0],
        scale_factors.0[1],
        scale_factors.0[2],
        scale_factors.0[3],
    );

    // Apply base scale offset for mass units (same logic as generate_prefixed_systematic_unit)
    let effective_scale_p10 =
        if let Some((_unit, _dimension)) = Dimension::find_unit_by_symbol(base_si_unit) {
            // Get the base scale offset from the unit's scale (systematic approach)
            let base_scale_offset = _unit.scale.log10().unwrap_or(0);
            total_scale_p10 - base_scale_offset
        } else {
            // Fallback: try to find by name if symbol lookup fails
            if let Some((_unit, _dimension)) = Dimension::find_unit_by_name(base_si_unit) {
                let base_scale_offset = _unit.scale.log10().unwrap_or(0);
                total_scale_p10 - base_scale_offset
            } else {
                // No base scale offset found, use total scale as-is
                total_scale_p10
            }
        };

    if let Some(prefix) = get_si_prefix(effective_scale_p10, long_name) {
        // For pure powers of base units, add disambiguating parentheses
        if !base_si_unit.contains("·")
            && (base_si_unit.contains("^")
                || base_si_unit.contains("²")
                || base_si_unit.contains("³"))
        {
            format!("{}({})", prefix, base_si_unit)
        } else {
            format!("{}{}", prefix, base_si_unit)
        }
    } else {
        // Check if this is a pure power of 10 using whippyunits-core
        let is_pure_power_of_10 = scale_factors.log10().is_some();

        if is_pure_power_of_10 {
            // Fall back to SI unit with 10^n notation when SI prefix lookup fails
            generate_si_unit_with_scale(effective_scale_p10, base_si_unit, long_name)
        } else {
            // Not a pure power of 10, show the scale factors explicitly
            let scale_factors_str = format_scale_factors(
                scale_factors.0[0],
                scale_factors.0[1],
                scale_factors.0[2],
                scale_factors.0[3],
            );
            if scale_factors_str.is_empty() {
                base_si_unit.to_string()
            } else {
                format!("{}{}", scale_factors_str, base_si_unit)
            }
        }
    }
}

/// Calculate total power of 10 using whippyunits-core ScaleExponents
fn calculate_total_scale_p10(scale_p2: i16, scale_p3: i16, scale_p5: i16, scale_pi: i16) -> i16 {
    let scale_exponents = ScaleExponents([scale_p2, scale_p3, scale_p5, scale_pi]);
    scale_exponents.log10().unwrap_or(0)
}

/// Generate SI unit with 10^n notation when no standard prefix is available
fn generate_si_unit_with_scale(
    total_scale_p10: i16,
    base_si_unit: &str,
    _long_name: bool,
) -> String {
    if total_scale_p10 == 0 {
        base_si_unit.to_string()
    } else {
        format!("10^{} {}", total_scale_p10, base_si_unit)
    }
}

/// Format scale factors by calculating the actual numeric value using whippyunits-core
fn format_scale_factors(scale_p2: i16, scale_p3: i16, scale_p5: i16, scale_pi: i16) -> String {
    let scale_exponents = ScaleExponents([scale_p2, scale_p3, scale_p5, scale_pi]);
    let total_scale_p10 = scale_exponents.log10().unwrap_or(0);

    if total_scale_p10 == 0 {
        String::new()
    } else {
        format!("10^{}", total_scale_p10)
    }
}

/// Get SI prefix for a given power of 10
fn get_si_prefix(power_of_10: i16, long_name: bool) -> Option<&'static str> {
    SiPrefix::ALL
        .iter()
        .find(|prefix| prefix.factor_log10() == power_of_10)
        .map(|prefix| {
            if long_name {
                prefix.name()
            } else {
                prefix.symbol()
            }
        })
}

/// Generate systematic unit name for composite dimensions
/// This is a public function that can be used by the main crate's prettyprint module
pub fn generate_systematic_composite_unit_name(
    dimension_exponents: DynDimensionExponents,
    long_name: bool,
) -> String {
    let exponents = dimension_exponents.0;

    // Check if all exponents are unknown
    if exponents.iter().all(|&exp| exp == i16::MIN) {
        return "?".to_string();
    }

    // Check if this is a pure dimension (only one non-zero exponent)
    let is_pure = exponents.iter().filter(|&exp| *exp != 0).count() == 1;

    // Generate unit parts for each dimension using simple string concatenation
    let mut result = String::new();
    let mut first = true;

    for (index, &exp) in exponents.iter().enumerate() {
        if exp == 0 {
            continue;
        }

        // Get unit configuration from Dimension::BASIS
        let (unit_name, unit_symbol, base_scale_offset) =
            if let Some(dimension) = Dimension::BASIS.get(index) {
                if let Some(unit) = dimension.units.first() {
                    let base_scale_offset = unit.scale.log10().unwrap_or(0);
                    (unit.name, unit.symbols[0], base_scale_offset)
                } else {
                    ("?", "?", 0)
                }
            } else {
                ("?", "?", 0)
            };

        // For compound units, convert g to kg for mass terms
        let is_compound_unit = exponents.iter().filter(|&&exp| exp != 0).count() > 1;
        let (adjusted_unit_name, adjusted_unit_symbol) =
            if is_compound_unit && base_scale_offset != 0 && index == 0 {
                // This is a compound unit with mass dimension (index 0) that has a scale offset
                if long_name {
                    match unit_name {
                        "gram" => ("kilogram", unit_symbol),
                        _ => (unit_name, unit_symbol),
                    }
                } else {
                    match unit_symbol {
                        "g" => (unit_name, "kg"),
                        _ => (unit_name, unit_symbol),
                    }
                }
            } else {
                (unit_name, unit_symbol)
            };

        // Generate the unit part with exponent
        let base_name = if long_name {
            adjusted_unit_name
        } else {
            adjusted_unit_symbol
        };
        let unit_part = if exp == 1 {
            base_name.to_string()
        } else {
            format!("{}^{}", base_name, exp)
        };

        if !first {
            result.push_str("·");
        }
        result.push_str(&unit_part);
        first = false;
    }

    if is_pure {
        result
    } else {
        format!("({})", result)
    }
}

/// Calculate the storage unit name from scale factors and dimension name
/// This is a convenience function for proc macros that have dimension name as string
pub fn get_storage_unit_name_by_dimension_name(
    scale_factors: ScaleExponents,
    dimension_name: &str,
    long_name: bool,
) -> String {
    // Map dimension name to exponents
    let dimension_exponents = match dimension_name {
        "Mass" => DynDimensionExponents([1, 0, 0, 0, 0, 0, 0, 0]),
        "Length" => DynDimensionExponents([0, 1, 0, 0, 0, 0, 0, 0]),
        "Time" => DynDimensionExponents([0, 0, 1, 0, 0, 0, 0, 0]),
        "Current" => DynDimensionExponents([0, 0, 0, 1, 0, 0, 0, 0]),
        "Temperature" => DynDimensionExponents([0, 0, 0, 0, 1, 0, 0, 0]),
        "Amount" => DynDimensionExponents([0, 0, 0, 0, 0, 1, 0, 0]),
        "Luminous Intensity" => DynDimensionExponents([0, 0, 0, 0, 0, 0, 1, 0]),
        "Angle" => DynDimensionExponents([0, 0, 0, 0, 0, 0, 0, 1]),
        _ => return "unknown".to_string(),
    };

    get_storage_unit_name(scale_factors, dimension_exponents, long_name)
}
